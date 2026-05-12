#!/usr/bin/env python3
"""Validate and render the alpha boundary/offboarding truth artifacts."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_BOUNDARY_REL = "artifacts/governance/boundary_manifest_alpha.yaml"
DEFAULT_ENTITLEMENT_REL = "artifacts/governance/entitlement_snapshot_alpha.yaml"
DEFAULT_ORG_SWITCH_REL = "artifacts/governance/org_switch_posture_alpha.yaml"
DEFAULT_USAGE_REL = "artifacts/governance/usage_export_baseline_alpha.yaml"
DEFAULT_FIXTURE_REL = "fixtures/governance/boundary_offboarding_alpha_cases/manifest.yaml"

REQUIRED_LOCAL_CORE_CAPABILITIES = {
    "editor_core",
    "command_plane",
    "local_git",
    "configuration_profiles",
    "local_ai_byok",
    "local_support_bundle",
}

REQUIRED_MANAGED_CAPABILITIES = {
    "identity_policy_service",
    "workspace_control_plane",
    "model_gateway",
    "extension_registry_mirror",
    "telemetry_support_pipeline",
    "managed_sync_profile",
    "hosted_marketplace_ui",
    "managed_ai_quota_billing",
    "fleet_admin_ui_scim",
}

REQUIRED_ENTITLEMENT_STATES = {
    "entitlement_state.active",
    "entitlement_state.signed_out",
    "entitlement_state.grace",
    "entitlement_state.restricted_managed_only",
    "entitlement_state.offboarded",
}

REQUIRED_FIXTURE_STATES = {
    "boundary_manifest_claimed_rows",
    "local_core_no_managed_prerequisite",
    "entitlement_signed_out",
    "entitlement_grace",
    "entitlement_restricted_managed_only",
    "entitlement_offboarded",
    "org_switch_reauth_and_revocation",
    "usage_export_meter_reconciliation",
    "offboarding_references_usage_export",
    "consumer_projection",
}

REQUIRED_CONSUMER_SURFACES = {"docs_help", "admin", "support_export", "cli"}

MANAGED_BOUNDARY_CLASSES = {
    "self_hostable_service",
    "mirrored_service",
    "managed_optional",
    "managed_premium",
    "managed_admin_only",
}

LOCAL_ALLOWED_NETWORK_BOUNDARIES = {
    "none",
    "optional_public_or_mirror",
    "mirror_or_file_import",
}

PATH_LIKE_SUFFIXES = (".yaml", ".yml", ".json", ".md", ".toml", ".rs", ".py")
ID_PREFIXES = (
    "boundary_manifest.",
    "entitlement_state.",
    "entitlement_snapshot.",
    "meter.",
    "offboarding_packet.",
    "org_context.",
    "org_switch.",
    "usage_export_packet.",
    "usage_export_row.",
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
    parser.add_argument("--boundary-manifest", default=DEFAULT_BOUNDARY_REL)
    parser.add_argument("--entitlement-snapshot", default=DEFAULT_ENTITLEMENT_REL)
    parser.add_argument("--org-switch-posture", default=DEFAULT_ORG_SWITCH_REL)
    parser.add_argument("--usage-export-baseline", default=DEFAULT_USAGE_REL)
    parser.add_argument("--fixture-manifest", default=DEFAULT_FIXTURE_REL)
    parser.add_argument("--report", default=None)
    parser.add_argument(
        "--render-consumer-projection",
        action="store_true",
        help="Print the docs/help, admin, CLI, and support-export projection as JSON.",
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


def ensure_bool(value: Any, label: str) -> bool:
    if not isinstance(value, bool):
        raise SystemExit(f"{label} must be a boolean")
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
                remediation="Use an ISO-8601 date without a time component.",
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


def validate_header(payload: dict[str, Any], label: str, findings: list[Finding]) -> None:
    version = ensure_int(payload.get("schema_version"), f"{label}.schema_version")
    if version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.schema_version.unsupported",
                message=f"{label}.schema_version must be 1, got {version}",
                remediation="Update the validator in the same change that bumps the artifact schema.",
            )
        )
    parse_iso_date(ensure_str(payload.get("as_of"), f"{label}.as_of"), f"{label}.as_of", findings)
    ensure_str(payload.get("owner_dri"), f"{label}.owner_dri")


def validate_refs_in_mapping(repo_root: Path, mapping: dict[str, Any], label: str, findings: list[Finding]) -> None:
    for key, value in mapping.items():
        if isinstance(value, str):
            validate_path_ref(repo_root, value, f"{label}.{key}", findings)
        elif isinstance(value, list):
            for item in value:
                if isinstance(item, str):
                    validate_path_ref(repo_root, item, f"{label}.{key}", findings)
        elif isinstance(value, dict):
            validate_refs_in_mapping(repo_root, value, f"{label}.{key}", findings)


def index_by(items: list[Any], key: str, label: str, findings: list[Finding]) -> dict[str, dict[str, Any]]:
    indexed: dict[str, dict[str, Any]] = {}
    for idx, raw_item in enumerate(items):
        item = ensure_dict(raw_item, f"{label}[{idx}]")
        item_id = ensure_str(item.get(key), f"{label}[{idx}].{key}")
        if item_id in indexed:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"{label}.duplicate",
                    message=f"duplicate {key}: {item_id}",
                    remediation="Use stable unique ids.",
                    ref=item_id,
                )
            )
        indexed[item_id] = item
    return indexed


def validate_boundary_manifest(
    repo_root: Path,
    boundary: dict[str, Any],
    findings: list[Finding],
) -> tuple[dict[str, dict[str, Any]], dict[str, dict[str, Any]]]:
    validate_header(boundary, "boundary", findings)
    validate_path_ref(repo_root, ensure_str(boundary.get("schema_ref"), "boundary.schema_ref"), "boundary.schema_ref", findings)
    validate_refs_in_mapping(repo_root, ensure_dict(boundary.get("source_contract_refs"), "boundary.source_contract_refs"), "boundary.source_contract_refs", findings)
    consumer_bindings = ensure_dict(boundary.get("consumer_bindings"), "boundary.consumer_bindings")
    first_consumer = ensure_dict(consumer_bindings.get("first_consumer"), "boundary.consumer_bindings.first_consumer")
    ensure_str(first_consumer.get("consumer_id"), "boundary.consumer_bindings.first_consumer.consumer_id")
    ensure_str(first_consumer.get("consumer_kind"), "boundary.consumer_bindings.first_consumer.consumer_kind")
    validate_path_ref(
        repo_root,
        ensure_str(first_consumer.get("consumer_ref"), "boundary.consumer_bindings.first_consumer.consumer_ref"),
        "boundary.consumer_bindings.first_consumer.consumer_ref",
        findings,
    )
    ensure_str(first_consumer.get("render_command"), "boundary.consumer_bindings.first_consumer.render_command")

    surfaces = set(ensure_list(consumer_bindings.get("consumer_surfaces"), "boundary.consumer_bindings.consumer_surfaces"))
    missing_surfaces = REQUIRED_CONSUMER_SURFACES - surfaces
    if missing_surfaces:
        findings.append(
            Finding(
                severity="error",
                check_id="boundary.consumer_surfaces.missing",
                message="boundary consumer surfaces must include docs_help, admin, support_export, and cli",
                remediation="Add the missing surfaces to consumer_bindings.consumer_surfaces.",
                details={"missing": sorted(missing_surfaces)},
            )
        )

    meters = index_by(ensure_list(boundary.get("meter_definitions"), "boundary.meter_definitions"), "meter_id", "boundary.meter_definitions", findings)
    capabilities = index_by(ensure_list(boundary.get("capability_rows"), "boundary.capability_rows"), "capability_id", "boundary.capability_rows", findings)

    missing_local = REQUIRED_LOCAL_CORE_CAPABILITIES - set(capabilities)
    if missing_local:
        findings.append(
            Finding(
                severity="error",
                check_id="boundary.local_core_capabilities.missing",
                message="boundary manifest is missing required local-core capabilities",
                remediation="Add rows for the local-core floor before claiming managed boundaries.",
                details={"missing": sorted(missing_local)},
            )
        )

    missing_managed = REQUIRED_MANAGED_CAPABILITIES - set(capabilities)
    if missing_managed:
        findings.append(
            Finding(
                severity="error",
                check_id="boundary.managed_capabilities.missing",
                message="boundary manifest is missing required managed-truth seed capabilities",
                remediation="Add rows for each claimed managed/support/export lane.",
                details={"missing": sorted(missing_managed)},
            )
        )

    for meter_id, meter in meters.items():
        for capability_id in ensure_list(meter.get("capability_refs"), f"meter {meter_id}.capability_refs"):
            capability_id = ensure_str(capability_id, f"meter {meter_id}.capability_refs[]")
            if capability_id not in capabilities:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="boundary.meter.capability_ref_unknown",
                        message=f"meter {meter_id} cites unknown capability {capability_id}",
                        remediation="Fix the meter capability_refs or add the capability row.",
                        ref=meter_id,
                    )
                )
        validate_path_ref(repo_root, ensure_str(meter.get("export_row_contract_ref"), f"meter {meter_id}.export_row_contract_ref"), "boundary.meter.export_row_contract_ref", findings)

    for capability_id, capability in capabilities.items():
        boundary_class = ensure_str(capability.get("boundary_class"), f"capability {capability_id}.boundary_class")
        claim_status = ensure_str(capability.get("claim_status"), f"capability {capability_id}.claim_status")
        identity_boundary = ensure_str(capability.get("required_identity_boundary"), f"capability {capability_id}.required_identity_boundary")
        network_boundary = ensure_str(capability.get("required_network_boundary"), f"capability {capability_id}.required_network_boundary")
        quota_refs = [ensure_str(item, f"capability {capability_id}.quota_meter_refs[]") for item in ensure_list(capability.get("quota_meter_refs"), f"capability {capability_id}.quota_meter_refs")]
        usage_refs = ensure_list(capability.get("usage_export_baseline_refs"), f"capability {capability_id}.usage_export_baseline_refs")
        lifecycle = ensure_dict(capability.get("lifecycle_metadata"), f"capability {capability_id}.lifecycle_metadata")
        parse_iso_date(ensure_str(lifecycle.get("introduced_at"), f"capability {capability_id}.lifecycle_metadata.introduced_at"), "boundary.lifecycle_metadata.introduced_at", findings, capability_id)
        for source_ref in ensure_list(lifecycle.get("source_refs"), f"capability {capability_id}.lifecycle_metadata.source_refs"):
            validate_path_ref(repo_root, ensure_str(source_ref, f"capability {capability_id}.source_refs[]"), "boundary.capability.source_refs", findings)
        for meter_ref in quota_refs:
            if meter_ref not in meters:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="boundary.capability.meter_ref_unknown",
                        message=f"capability {capability_id} cites unknown meter {meter_ref}",
                        remediation="Add the meter definition or correct the quota_meter_refs entry.",
                        ref=capability_id,
                    )
                )

        if boundary_class == "open_local_core":
            if claim_status != "alpha_core_claim":
                findings.append(
                    Finding(
                        severity="error",
                        check_id="boundary.local_core.claim_status",
                        message=f"local-core capability {capability_id} must use alpha_core_claim",
                        remediation="Keep local-core claim status distinct from managed truth seeds.",
                        ref=capability_id,
                    )
                )
            if identity_boundary != "no_identity_required":
                findings.append(
                    Finding(
                        severity="error",
                        check_id="boundary.local_core.identity_required",
                        message=f"local-core capability {capability_id} requires identity boundary {identity_boundary}",
                        remediation="Local-core rows must not require managed identity.",
                        ref=capability_id,
                    )
                )
            if network_boundary not in LOCAL_ALLOWED_NETWORK_BOUNDARIES:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="boundary.local_core.network_required",
                        message=f"local-core capability {capability_id} requires managed network boundary {network_boundary}",
                        remediation="Local-core rows may not require managed control-plane reachability.",
                        ref=capability_id,
                    )
                )
            if quota_refs or usage_refs:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="boundary.local_core.quota_or_export_gate",
                        message=f"local-core capability {capability_id} has quota or usage-export refs",
                        remediation="Remove quota and usage-export prerequisites from local-core rows.",
                        ref=capability_id,
                    )
                )
        elif boundary_class in MANAGED_BOUNDARY_CLASSES:
            if claim_status != "alpha_managed_truth_seed":
                findings.append(
                    Finding(
                        severity="error",
                        check_id="boundary.managed.claim_status",
                        message=f"managed capability {capability_id} must use alpha_managed_truth_seed",
                        remediation="Managed boundary rows should be truth seeds, not local-core claims.",
                        ref=capability_id,
                    )
                )
            open_alt = ensure_dict(capability.get("open_local_alternative"), f"capability {capability_id}.open_local_alternative")
            self_host_alt = ensure_dict(capability.get("self_host_alternative"), f"capability {capability_id}.self_host_alternative")
            mirror_alt = ensure_dict(capability.get("mirrored_or_airgap_alternative"), f"capability {capability_id}.mirrored_or_airgap_alternative")
            if not ensure_bool(open_alt.get("available"), f"capability {capability_id}.open_local_alternative.available"):
                findings.append(
                    Finding(
                        severity="error",
                        check_id="boundary.managed.missing_open_local_alternative",
                        message=f"managed capability {capability_id} lacks an open/local alternative",
                        remediation="Name the local-core behavior that remains available when managed service is absent.",
                        ref=capability_id,
                    )
                )
            if boundary_class == "self_hostable_service" and not ensure_bool(self_host_alt.get("available"), f"capability {capability_id}.self_host_alternative.available"):
                findings.append(
                    Finding(
                        severity="error",
                        check_id="boundary.self_hostable.missing_self_host_alternative",
                        message=f"self-hostable capability {capability_id} lacks a self-host alternative",
                        remediation="Declare the customer-operated or file-based path.",
                        ref=capability_id,
                    )
                )
            if boundary_class == "mirrored_service" and not ensure_bool(mirror_alt.get("available"), f"capability {capability_id}.mirrored_or_airgap_alternative.available"):
                findings.append(
                    Finding(
                        severity="error",
                        check_id="boundary.mirrored.missing_mirror_alternative",
                        message=f"mirrored capability {capability_id} lacks a mirror/offline alternative",
                        remediation="Declare the mirror, file import, or offline bundle posture.",
                        ref=capability_id,
                    )
                )
            if boundary_class.startswith("managed_") and not (
                ensure_bool(self_host_alt.get("available"), f"capability {capability_id}.self_host_alternative.available")
                or ensure_bool(mirror_alt.get("available"), f"capability {capability_id}.mirrored_or_airgap_alternative.available")
            ):
                findings.append(
                    Finding(
                        severity="error",
                        check_id="boundary.managed.missing_exit_alternative",
                        message=f"managed capability {capability_id} lacks self-host or mirror alternative",
                        remediation="Declare an anti-lock-in path for managed convenience rows.",
                        ref=capability_id,
                    )
                )
        elif boundary_class == "explicitly_out_of_scope":
            if quota_refs or usage_refs:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="boundary.out_of_scope.has_meter_or_export",
                        message=f"out-of-scope capability {capability_id} has meter or usage export refs",
                        remediation="Reserved out-of-scope rows must not imply active managed surfaces.",
                        ref=capability_id,
                    )
                )

    return capabilities, meters


def validate_entitlements(
    entitlement: dict[str, Any],
    capabilities: dict[str, dict[str, Any]],
    meters: dict[str, dict[str, Any]],
    findings: list[Finding],
) -> tuple[dict[str, dict[str, Any]], dict[str, dict[str, Any]]]:
    validate_header(entitlement, "entitlement", findings)
    state_rows = index_by(ensure_list(entitlement.get("snapshot_rows"), "entitlement.snapshot_rows"), "state_id", "entitlement.snapshot_rows", findings)
    capability_rows = index_by(ensure_list(entitlement.get("capability_entitlements"), "entitlement.capability_entitlements"), "capability_id", "entitlement.capability_entitlements", findings)

    missing_states = REQUIRED_ENTITLEMENT_STATES - set(state_rows)
    if missing_states:
        findings.append(
            Finding(
                severity="error",
                check_id="entitlement.states.missing",
                message="entitlement snapshot is missing required lifecycle states",
                remediation="Add active, signed_out, grace, restricted_managed_only, and offboarded rows.",
                details={"missing": sorted(missing_states)},
            )
        )

    for state_id, row in state_rows.items():
        if not ensure_bool(row.get("local_core_available"), f"entitlement {state_id}.local_core_available"):
            findings.append(
                Finding(
                    severity="error",
                    check_id="entitlement.local_core_unavailable",
                    message=f"entitlement state {state_id} marks local core unavailable",
                    remediation="Entitlement state may narrow managed actions only.",
                    ref=state_id,
                )
            )
        local_ids = set(ensure_list(row.get("local_core_capability_ids"), f"entitlement {state_id}.local_core_capability_ids"))
        missing_local = REQUIRED_LOCAL_CORE_CAPABILITIES - local_ids
        if missing_local:
            findings.append(
                Finding(
                    severity="error",
                    check_id="entitlement.local_core_ids_missing",
                    message=f"entitlement state {state_id} does not list the full local-core floor",
                    remediation="Carry the full local-core capability list into every entitlement state.",
                    ref=state_id,
                    details={"missing": sorted(missing_local)},
                )
            )

    signed_out = state_rows.get("entitlement_state.signed_out", {})
    if signed_out and not ensure_bool(signed_out.get("new_managed_actions_blocked"), "entitlement.signed_out.new_managed_actions_blocked"):
        findings.append(
            Finding(
                severity="error",
                check_id="entitlement.signed_out.must_block_managed_actions",
                message="signed-out state must block new org-scoped managed actions",
                remediation="Set new_managed_actions_blocked to true for signed-out state.",
                ref="entitlement_state.signed_out",
            )
        )

    grace = state_rows.get("entitlement_state.grace", {})
    if grace:
        if ensure_int(grace.get("grace_window_days"), "entitlement.grace.grace_window_days") <= 0:
            findings.append(
                Finding(
                    severity="error",
                    check_id="entitlement.grace.window_missing",
                    message="grace state must declare a positive export/recovery window",
                    remediation="Set grace_window_days to the declared grace period.",
                    ref="entitlement_state.grace",
                )
            )
        if not ensure_bool(grace.get("usage_export_available"), "entitlement.grace.usage_export_available"):
            findings.append(
                Finding(
                    severity="error",
                    check_id="entitlement.grace.usage_export_unavailable",
                    message="grace state must keep usage export available",
                    remediation="Make usage export available during grace.",
                    ref="entitlement_state.grace",
                )
            )

    for restricted_state in ("entitlement_state.restricted_managed_only", "entitlement_state.offboarded"):
        row = state_rows.get(restricted_state, {})
        if row:
            if not ensure_bool(row.get("new_managed_actions_blocked"), f"entitlement {restricted_state}.new_managed_actions_blocked"):
                findings.append(
                    Finding(
                        severity="error",
                        check_id="entitlement.restricted_or_offboarded.must_block_managed_actions",
                        message=f"{restricted_state} must block new managed actions",
                        remediation="Set new_managed_actions_blocked to true.",
                        ref=restricted_state,
                    )
                )
            if not ensure_bool(row.get("usage_export_available"), f"entitlement {restricted_state}.usage_export_available"):
                findings.append(
                    Finding(
                        severity="error",
                        check_id="entitlement.restricted_or_offboarded.usage_export_unavailable",
                        message=f"{restricted_state} must keep usage export available",
                        remediation="Keep export/recovery available after restriction or access end.",
                        ref=restricted_state,
                    )
                )

    missing_entitlements = REQUIRED_MANAGED_CAPABILITIES - set(capability_rows)
    if missing_entitlements:
        findings.append(
            Finding(
                severity="error",
                check_id="entitlement.capability_entitlements.missing",
                message="managed capabilities are missing entitlement rows",
                remediation="Add capability_entitlements rows for every managed boundary row.",
                details={"missing": sorted(missing_entitlements)},
            )
        )

    for capability_id, row in capability_rows.items():
        if capability_id not in capabilities:
            findings.append(
                Finding(
                    severity="error",
                    check_id="entitlement.capability_unknown",
                    message=f"entitlement row cites unknown capability {capability_id}",
                    remediation="Add the capability to the boundary manifest or correct the entitlement row.",
                    ref=capability_id,
                )
            )
        for state_ref in ensure_list(row.get("allowed_state_refs"), f"entitlement {capability_id}.allowed_state_refs") + ensure_list(row.get("blocked_state_refs"), f"entitlement {capability_id}.blocked_state_refs"):
            state_ref = ensure_str(state_ref, f"entitlement {capability_id}.state_refs[]")
            if state_ref not in state_rows:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="entitlement.state_ref_unknown",
                        message=f"entitlement row {capability_id} cites unknown state {state_ref}",
                        remediation="Correct the state ref or add the missing snapshot state.",
                        ref=capability_id,
                    )
                )
        for meter_ref in ensure_list(row.get("quota_meter_refs"), f"entitlement {capability_id}.quota_meter_refs"):
            meter_ref = ensure_str(meter_ref, f"entitlement {capability_id}.quota_meter_refs[]")
            if meter_ref not in meters:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="entitlement.meter_ref_unknown",
                        message=f"entitlement row {capability_id} cites unknown meter {meter_ref}",
                        remediation="Correct the quota meter refs.",
                        ref=capability_id,
                    )
                )

    return state_rows, capability_rows


def validate_org_switch(
    org_switch: dict[str, Any],
    capabilities: dict[str, dict[str, Any]],
    states: dict[str, dict[str, Any]],
    findings: list[Finding],
) -> dict[str, dict[str, Any]]:
    validate_header(org_switch, "org_switch", findings)
    rows = index_by(ensure_list(org_switch.get("posture_rows"), "org_switch.posture_rows"), "posture_id", "org_switch.posture_rows", findings)
    local_ids = set(ensure_list(org_switch.get("local_core_capability_ids"), "org_switch.local_core_capability_ids"))
    missing_local = REQUIRED_LOCAL_CORE_CAPABILITIES - local_ids
    if missing_local:
        findings.append(
            Finding(
                severity="error",
                check_id="org_switch.local_core_ids_missing",
                message="org switch posture does not list the full local-core floor",
                remediation="Carry all local-core capability ids in org_switch.local_core_capability_ids.",
                details={"missing": sorted(missing_local)},
            )
        )

    required_postures = {
        "org_switch.switching_org_reauth",
        "org_switch.restricted_after_revocation",
        "org_switch.offboarded_org",
    }
    missing_postures = required_postures - set(rows)
    if missing_postures:
        findings.append(
            Finding(
                severity="error",
                check_id="org_switch.required_postures_missing",
                message="org switch posture is missing reauth, revocation, or offboarded rows",
                remediation="Add the required org-switch posture rows.",
                details={"missing": sorted(missing_postures)},
            )
        )

    for posture_id, row in rows.items():
        state_ref = ensure_str(row.get("entitlement_state_ref"), f"org_switch {posture_id}.entitlement_state_ref")
        if state_ref not in states:
            findings.append(
                Finding(
                    severity="error",
                    check_id="org_switch.entitlement_state_ref_unknown",
                    message=f"org switch posture {posture_id} cites unknown entitlement state {state_ref}",
                    remediation="Correct entitlement_state_ref.",
                    ref=posture_id,
                )
            )
        if not ensure_bool(row.get("local_core_available"), f"org_switch {posture_id}.local_core_available"):
            findings.append(
                Finding(
                    severity="error",
                    check_id="org_switch.local_core_unavailable",
                    message=f"org switch posture {posture_id} blocks local core",
                    remediation="Org switching may invalidate managed credentials only.",
                    ref=posture_id,
                )
            )
        invalidation = ensure_str(row.get("managed_credential_invalidation"), f"org_switch {posture_id}.managed_credential_invalidation")
        if posture_id in required_postures and invalidation == "not_applicable_current_context":
            findings.append(
                Finding(
                    severity="error",
                    check_id="org_switch.missing_credential_invalidation",
                    message=f"org switch posture {posture_id} does not invalidate managed credentials",
                    remediation="Declare the managed credential invalidation behavior.",
                    ref=posture_id,
                )
            )
        export_posture = ensure_dict(row.get("export_posture"), f"org_switch {posture_id}.export_posture")
        if not ensure_bool(export_posture.get("support_export_available"), f"org_switch {posture_id}.support_export_available"):
            findings.append(
                Finding(
                    severity="error",
                    check_id="org_switch.support_export_unavailable",
                    message=f"org switch posture {posture_id} blocks support export",
                    remediation="Keep support export available for org-switch diagnosis.",
                    ref=posture_id,
                )
            )
        for capability_id in ensure_list(row.get("blocked_managed_capability_ids"), f"org_switch {posture_id}.blocked_managed_capability_ids"):
            capability_id = ensure_str(capability_id, f"org_switch {posture_id}.blocked_managed_capability_ids[]")
            if capability_id not in capabilities:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="org_switch.blocked_capability_unknown",
                        message=f"org switch posture {posture_id} cites unknown blocked capability {capability_id}",
                        remediation="Correct the blocked managed capability list.",
                        ref=posture_id,
                    )
                )
    return rows


def validate_usage_baseline(
    usage: dict[str, Any],
    capabilities: dict[str, dict[str, Any]],
    meters: dict[str, dict[str, Any]],
    states: dict[str, dict[str, Any]],
    entitlement_rows: dict[str, dict[str, Any]],
    findings: list[Finding],
) -> tuple[dict[str, dict[str, Any]], dict[str, dict[str, Any]]]:
    validate_header(usage, "usage", findings)
    packets = index_by(ensure_list(usage.get("usage_export_packets"), "usage.usage_export_packets"), "packet_id", "usage.usage_export_packets", findings)
    offboarding_packets = index_by(ensure_list(usage.get("offboarding_packets"), "usage.offboarding_packets"), "packet_id", "usage.offboarding_packets", findings)

    for packet_id, packet in packets.items():
        if ensure_bool(packet.get("support_ticket_required"), f"usage {packet_id}.support_ticket_required"):
            findings.append(
                Finding(
                    severity="error",
                    check_id="usage.support_ticket_required",
                    message=f"usage export packet {packet_id} requires a support ticket",
                    remediation="Usage export packets must be customer-visible and scriptable.",
                    ref=packet_id,
                )
            )
        for state_ref in ensure_list(packet.get("entitlement_state_refs"), f"usage {packet_id}.entitlement_state_refs"):
            state_ref = ensure_str(state_ref, f"usage {packet_id}.entitlement_state_refs[]")
            if state_ref not in states:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="usage.entitlement_state_ref_unknown",
                        message=f"usage packet {packet_id} cites unknown entitlement state {state_ref}",
                        remediation="Correct the state ref.",
                        ref=packet_id,
                    )
                )
        for capability_id in ensure_list(packet.get("capability_refs"), f"usage {packet_id}.capability_refs"):
            capability_id = ensure_str(capability_id, f"usage {packet_id}.capability_refs[]")
            if capability_id not in capabilities:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="usage.capability_ref_unknown",
                        message=f"usage packet {packet_id} cites unknown capability {capability_id}",
                        remediation="Correct capability_refs.",
                        ref=packet_id,
                    )
                )
        for quota_idx, raw_quota_row in enumerate(ensure_list(packet.get("quota_rows"), f"usage {packet_id}.quota_rows")):
            quota_row = ensure_dict(raw_quota_row, f"usage {packet_id}.quota_rows[{quota_idx}]")
            meter_id = ensure_str(quota_row.get("meter_id"), f"usage {packet_id}.quota_rows[{quota_idx}].meter_id")
            meter = meters.get(meter_id)
            if meter is None:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="usage.quota_row.meter_unknown",
                        message=f"usage packet {packet_id} quota row cites unknown meter {meter_id}",
                        remediation="Correct the meter id or add the boundary meter definition.",
                        ref=packet_id,
                    )
                )
                continue
            for field_name in ("service_family_class", "quota_family_class", "quota_unit_class"):
                expected = ensure_str(meter.get(field_name), f"meter {meter_id}.{field_name}")
                actual = ensure_str(quota_row.get(field_name), f"usage {packet_id}.{field_name}")
                if actual != expected:
                    findings.append(
                        Finding(
                            severity="error",
                            check_id="usage.quota_row.meter_mismatch",
                            message=f"usage packet {packet_id} {field_name} {actual} does not match meter {meter_id} value {expected}",
                            remediation="Use the boundary manifest meter definition as the source of truth.",
                            ref=packet_id,
                        )
                    )
            capability_id = ensure_str(quota_row.get("capability_id"), f"usage {packet_id}.quota_rows[{quota_idx}].capability_id")
            meter_capabilities = set(ensure_list(meter.get("capability_refs"), f"meter {meter_id}.capability_refs"))
            if capability_id not in meter_capabilities:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="usage.quota_row.capability_not_metered",
                        message=f"usage packet {packet_id} maps {meter_id} to non-metered capability {capability_id}",
                        remediation="Use a capability listed on the meter definition.",
                        ref=packet_id,
                    )
                )

    for offboarding_id, packet in offboarding_packets.items():
        if ensure_bool(packet.get("support_ticket_required"), f"offboarding {offboarding_id}.support_ticket_required"):
            findings.append(
                Finding(
                    severity="error",
                    check_id="offboarding.support_ticket_required",
                    message=f"offboarding packet {offboarding_id} requires a support ticket",
                    remediation="Exit packages must be available without support-ticket dependency.",
                    ref=offboarding_id,
                )
            )
        if not ensure_bool(packet.get("local_artifacts_accessible"), f"offboarding {offboarding_id}.local_artifacts_accessible"):
            findings.append(
                Finding(
                    severity="error",
                    check_id="offboarding.local_artifacts_not_accessible",
                    message=f"offboarding packet {offboarding_id} does not preserve local artifact access",
                    remediation="Offboarding must preserve local artifacts and prior exports.",
                    ref=offboarding_id,
                )
            )
        if not ensure_bool(packet.get("siblings_not_embedded"), f"offboarding {offboarding_id}.siblings_not_embedded"):
            findings.append(
                Finding(
                    severity="error",
                    check_id="offboarding.siblings_embedded",
                    message=f"offboarding packet {offboarding_id} embeds sibling payloads",
                    remediation="Reference sibling export families by opaque id.",
                    ref=offboarding_id,
                )
            )
        usage_refs = ensure_list(packet.get("usage_export_packet_refs"), f"offboarding {offboarding_id}.usage_export_packet_refs")
        if not usage_refs:
            findings.append(
                Finding(
                    severity="error",
                    check_id="offboarding.usage_export_refs_missing",
                    message=f"offboarding packet {offboarding_id} has no usage export refs",
                    remediation="Reference last contractual usage-export packets by id.",
                    ref=offboarding_id,
                )
            )
        for usage_ref in usage_refs:
            usage_ref = ensure_str(usage_ref, f"offboarding {offboarding_id}.usage_export_packet_refs[]")
            if usage_ref not in packets:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="offboarding.usage_export_ref_unknown",
                        message=f"offboarding packet {offboarding_id} cites unknown usage packet {usage_ref}",
                        remediation="Correct usage_export_packet_refs.",
                        ref=offboarding_id,
                    )
                )

    for capability_id, capability in capabilities.items():
        for usage_ref in ensure_list(capability.get("usage_export_baseline_refs"), f"capability {capability_id}.usage_export_baseline_refs"):
            usage_ref = ensure_str(usage_ref, f"capability {capability_id}.usage_export_baseline_refs[]")
            if usage_ref not in packets:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="boundary.usage_export_ref_unknown",
                        message=f"capability {capability_id} cites unknown usage export packet {usage_ref}",
                        remediation="Correct the boundary usage refs or add the baseline packet.",
                        ref=capability_id,
                    )
                )
        for offboarding_ref in ensure_list(capability.get("offboarding_baseline_refs"), f"capability {capability_id}.offboarding_baseline_refs"):
            offboarding_ref = ensure_str(offboarding_ref, f"capability {capability_id}.offboarding_baseline_refs[]")
            if offboarding_ref not in offboarding_packets:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="boundary.offboarding_ref_unknown",
                        message=f"capability {capability_id} cites unknown offboarding packet {offboarding_ref}",
                        remediation="Correct the boundary offboarding refs or add the baseline packet.",
                        ref=capability_id,
                    )
                )

    for capability_id, entitlement_row in entitlement_rows.items():
        for usage_ref in ensure_list(entitlement_row.get("usage_export_baseline_refs"), f"entitlement {capability_id}.usage_export_baseline_refs"):
            usage_ref = ensure_str(usage_ref, f"entitlement {capability_id}.usage_export_baseline_refs[]")
            if usage_ref not in packets:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="entitlement.usage_export_ref_unknown",
                        message=f"entitlement row {capability_id} cites unknown usage export packet {usage_ref}",
                        remediation="Correct entitlement usage refs.",
                        ref=capability_id,
                    )
                )

    coverage_states = {
        ensure_str(ensure_dict(item, "usage.state_sample_coverage[]").get("state_ref"), "usage.state_sample_coverage.state_ref")
        for item in ensure_list(usage.get("state_sample_coverage"), "usage.state_sample_coverage")
    }
    missing_coverage = {
        "entitlement_state.signed_out",
        "entitlement_state.grace",
        "entitlement_state.restricted_managed_only",
        "entitlement_state.offboarded",
    } - coverage_states
    if missing_coverage:
        findings.append(
            Finding(
                severity="error",
                check_id="usage.state_sample_coverage.missing",
                message="usage baseline does not cover every required entitlement acceptance state",
                remediation="Add state_sample_coverage rows for signed-out, grace, restricted, and offboarded states.",
                details={"missing": sorted(missing_coverage)},
            )
        )

    return packets, offboarding_packets


def validate_fixtures(
    fixture: dict[str, Any],
    capabilities: dict[str, dict[str, Any]],
    states: dict[str, dict[str, Any]],
    usage_packets: dict[str, dict[str, Any]],
    offboarding_packets: dict[str, dict[str, Any]],
    org_switch_rows: dict[str, dict[str, Any]],
    findings: list[Finding],
) -> None:
    version = ensure_int(fixture.get("schema_version"), "fixture.schema_version")
    if version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="fixture.schema_version.unsupported",
                message=f"fixture schema_version must be 1, got {version}",
                remediation="Update the fixture validator in the same change as the fixture schema.",
            )
        )
    required_states = set(ensure_list(fixture.get("required_acceptance_states"), "fixture.required_acceptance_states"))
    missing = REQUIRED_FIXTURE_STATES - required_states
    if missing:
        findings.append(
            Finding(
                severity="error",
                check_id="fixture.required_states.missing",
                message="fixture manifest is missing required acceptance states",
                remediation="Add the missing protected proof states.",
                details={"missing": sorted(missing)},
            )
        )
    exercised: set[str] = set()
    for idx, raw_case in enumerate(ensure_list(fixture.get("cases"), "fixture.cases")):
        case = ensure_dict(raw_case, f"fixture.cases[{idx}]")
        case_id = ensure_str(case.get("case_id"), f"fixture.cases[{idx}].case_id")
        exercised.add(ensure_str(case.get("exercises_state"), f"fixture.cases[{idx}].exercises_state"))
        ensure_str(case.get("expected_validator_result"), f"fixture.cases[{idx}].expected_validator_result")
        for key, collection in (
            ("capability_refs", capabilities),
            ("usage_export_packet_refs", usage_packets),
            ("posture_refs", org_switch_rows),
        ):
            for ref in case.get(key, []) or []:
                ref = ensure_str(ref, f"fixture case {case_id}.{key}[]")
                if ref not in collection:
                    findings.append(
                        Finding(
                            severity="error",
                            check_id=f"fixture.{key}.unknown",
                            message=f"fixture case {case_id} cites unknown ref {ref}",
                            remediation="Correct the fixture ref.",
                            ref=case_id,
                        )
                    )
        for key, collection in (
            ("entitlement_state_ref", states),
            ("usage_export_packet_ref", usage_packets),
            ("offboarding_packet_ref", offboarding_packets),
        ):
            if key in case and case[key] is not None:
                ref = ensure_str(case.get(key), f"fixture case {case_id}.{key}")
                if ref not in collection:
                    findings.append(
                        Finding(
                            severity="error",
                            check_id=f"fixture.{key}.unknown",
                            message=f"fixture case {case_id} cites unknown ref {ref}",
                            remediation="Correct the fixture ref.",
                            ref=case_id,
                        )
                    )
    unexercised = REQUIRED_FIXTURE_STATES - exercised
    if unexercised:
        findings.append(
            Finding(
                severity="error",
                check_id="fixture.cases.missing_exercises",
                message="fixture cases do not exercise every required state",
                remediation="Add fixture cases for the missing acceptance states.",
                details={"missing": sorted(unexercised)},
            )
        )


def build_projection(
    capabilities: dict[str, dict[str, Any]],
    entitlement_rows: dict[str, dict[str, Any]],
    state_rows: dict[str, dict[str, Any]],
    org_switch_rows: dict[str, dict[str, Any]],
    usage_packets: dict[str, dict[str, Any]],
    offboarding_packets: dict[str, dict[str, Any]],
    meters: dict[str, dict[str, Any]],
) -> dict[str, Any]:
    entitlement_by_capability = {capability_id: row for capability_id, row in entitlement_rows.items()}
    org_switch_summary = {
        posture_id: {
            "entitlement_state_ref": row.get("entitlement_state_ref"),
            "managed_action_posture": row.get("managed_action_posture"),
            "local_core_available": row.get("local_core_available"),
            "export_posture": row.get("export_posture"),
        }
        for posture_id, row in sorted(org_switch_rows.items())
    }
    rows: list[dict[str, Any]] = []
    for capability_id, capability in sorted(capabilities.items()):
        entitlement = entitlement_by_capability.get(capability_id, {})
        meter_refs = ensure_list(capability.get("quota_meter_refs"), f"projection {capability_id}.quota_meter_refs")
        usage_refs = ensure_list(capability.get("usage_export_baseline_refs"), f"projection {capability_id}.usage_export_baseline_refs")
        offboarding_refs = ensure_list(capability.get("offboarding_baseline_refs"), f"projection {capability_id}.offboarding_baseline_refs")
        rows.append(
            {
                "capability_id": capability_id,
                "title": capability.get("title"),
                "boundary_class": capability.get("boundary_class"),
                "claim_status": capability.get("claim_status"),
                "open_local_alternative": capability.get("open_local_alternative"),
                "self_host_alternative": capability.get("self_host_alternative"),
                "mirrored_or_airgap_alternative": capability.get("mirrored_or_airgap_alternative"),
                "required_identity_boundary": capability.get("required_identity_boundary"),
                "required_network_boundary": capability.get("required_network_boundary"),
                "local_core_guardrail": capability.get("local_core_guardrail"),
                "entitlement": {
                    "allowed_state_refs": entitlement.get("allowed_state_refs", []),
                    "blocked_state_refs": entitlement.get("blocked_state_refs", []),
                    "local_core_effect": entitlement.get("local_core_effect", "none"),
                },
                "quota_meters": [
                    {
                        "meter_id": meter_ref,
                        "service_family_class": meters[meter_ref].get("service_family_class"),
                        "quota_family_class": meters[meter_ref].get("quota_family_class"),
                        "quota_unit_class": meters[meter_ref].get("quota_unit_class"),
                    }
                    for meter_ref in meter_refs
                    if meter_ref in meters
                ],
                "usage_exports": [
                    {
                        "packet_id": usage_ref,
                        "availability_class": usage_packets[usage_ref].get("availability_class"),
                        "support_ticket_required": usage_packets[usage_ref].get("support_ticket_required"),
                    }
                    for usage_ref in usage_refs
                    if usage_ref in usage_packets
                ],
                "offboarding_packets": [
                    {
                        "packet_id": offboarding_ref,
                        "support_ticket_required": offboarding_packets[offboarding_ref].get("support_ticket_required"),
                        "local_artifacts_accessible": offboarding_packets[offboarding_ref].get("local_artifacts_accessible"),
                    }
                    for offboarding_ref in offboarding_refs
                    if offboarding_ref in offboarding_packets
                ],
                "consumer_surfaces": ["docs_help", "admin", "support_export", "cli", "release_evidence"],
            }
        )
    return {
        "schema_version": 1,
        "projection_id": "boundary_offboarding_alpha_projection",
        "generated_at": dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
        "state_summaries": {
            state_id: {
                "managed_action_posture": row.get("managed_action_posture"),
                "local_core_available": row.get("local_core_available"),
                "usage_export_available": row.get("usage_export_available"),
                "offboarding_packet_available": row.get("offboarding_packet_available"),
            }
            for state_id, row in sorted(state_rows.items())
        },
        "org_switch_summaries": org_switch_summary,
        "capability_rows": rows,
    }


def write_report(path: Path, refs: dict[str, str], findings: list[Finding]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": 1,
        "status": "pass" if not any(item.severity == "error" for item in findings) else "fail",
        "generated_at": dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
        "artifact_refs": refs,
        "summary": {
            "errors": sum(1 for item in findings if item.severity == "error"),
            "warnings": sum(1 for item in findings if item.severity == "warning"),
        },
        "findings": [item.as_report() for item in findings],
    }
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def print_findings(findings: list[Finding], *, stream: Any = sys.stdout) -> None:
    errors = [finding for finding in findings if finding.severity == "error"]
    warnings = [finding for finding in findings if finding.severity == "warning"]
    status = "PASS" if not errors else "FAIL"
    print(f"[boundary-offboarding-alpha] {status} ({len(errors)} errors, {len(warnings)} warnings)", file=stream)
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        print(f"[boundary-offboarding-alpha] {prefix} {finding.check_id}: {finding.message}{ref_suffix}", file=stream)
        print(f"[boundary-offboarding-alpha]   remediation: {finding.remediation}", file=stream)


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(f"--repo-root does not look like a repository root: {repo_root}")

    refs = {
        "boundary_manifest": str(args.boundary_manifest),
        "entitlement_snapshot": str(args.entitlement_snapshot),
        "org_switch_posture": str(args.org_switch_posture),
        "usage_export_baseline": str(args.usage_export_baseline),
        "fixture_manifest": str(args.fixture_manifest),
    }

    boundary = ensure_dict(render_yaml_as_json(repo_root / args.boundary_manifest), "boundary")
    entitlement = ensure_dict(render_yaml_as_json(repo_root / args.entitlement_snapshot), "entitlement")
    org_switch = ensure_dict(render_yaml_as_json(repo_root / args.org_switch_posture), "org_switch")
    usage = ensure_dict(render_yaml_as_json(repo_root / args.usage_export_baseline), "usage")
    fixture = ensure_dict(render_yaml_as_json(repo_root / args.fixture_manifest), "fixture")

    findings: list[Finding] = []
    for ref in refs.values():
        validate_path_ref(repo_root, ref, "input", findings)

    capabilities, meters = validate_boundary_manifest(repo_root, boundary, findings)
    states, entitlement_rows = validate_entitlements(entitlement, capabilities, meters, findings)
    org_switch_rows = validate_org_switch(org_switch, capabilities, states, findings)
    usage_packets, offboarding_packets = validate_usage_baseline(
        usage,
        capabilities,
        meters,
        states,
        entitlement_rows,
        findings,
    )
    validate_fixtures(
        fixture,
        capabilities,
        states,
        usage_packets,
        offboarding_packets,
        org_switch_rows,
        findings,
    )

    if args.report:
        write_report(repo_root / args.report, refs, findings)

    errors = [finding for finding in findings if finding.severity == "error"]
    if args.render_consumer_projection:
        if errors:
            print_findings(findings, stream=sys.stderr)
            return 1
        projection = build_projection(
            capabilities,
            entitlement_rows,
            states,
            org_switch_rows,
            usage_packets,
            offboarding_packets,
            meters,
        )
        print(json.dumps(projection, indent=2, sort_keys=True))
        return 0

    print_findings(findings)
    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[boundary-offboarding-alpha] interrupted", file=sys.stderr)
        sys.exit(130)
