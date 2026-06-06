#!/usr/bin/env python3
"""Validate stable AI-pack promotion and mirror/offline publication artifacts."""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Any


DEFAULT_PACKET = "artifacts/ai/m4/provider-model-prompt-tool-rollout/rollout_packet.json"
DEFAULT_LOCAL_MANIFEST = "artifacts/ai/m4/local-model-pack-publication/manifest.json"
DEFAULT_SUPPORT_EXPORT = "artifacts/ai/m4/provider-model-prompt-tool-rollout/support_export.json"
REQUIRED_OBJECT_KINDS = {
    "provider_model_enablement",
    "prompt_pack",
    "tool_schema_pack",
    "feature_ai_rollout",
}
REQUIRED_RINGS = {"canary", "pilot", "broad", "lts"}


def load_json(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def non_empty(value: Any) -> bool:
    return isinstance(value, str) and bool(value.strip())


def validate_packet(packet: dict[str, Any]) -> list[str]:
    findings: list[str] = []
    if packet.get("record_kind") != "ai_rollout_publication_packet":
        findings.append("packet: unsupported record_kind")
    if packet.get("schema_version") != 1:
        findings.append("packet: unsupported schema_version")

    objects = {
        row.get("rollout_object_id"): row
        for row in packet.get("rollout_objects", [])
        if non_empty(row.get("rollout_object_id"))
    }
    routes = {
        row.get("route_id"): row
        for row in packet.get("stable_routes", [])
        if non_empty(row.get("route_id"))
    }

    if not routes:
        findings.append("packet: no stable routes")

    for object_id, row in objects.items():
        for field in (
            "owner_ref",
            "promotion_artifact_ref",
            "current_version_ref",
            "compatibility_range_ref",
            "graduation_packet_ref",
            "evidence_ref",
            "rollback_or_deny_lever_ref",
        ):
            if not non_empty(row.get(field)):
                findings.append(f"{object_id}: missing {field}")
        if row.get("rollout_state") == "stable":
            rings = set(row.get("rings_completed", []))
            if not REQUIRED_RINGS.issubset(rings):
                findings.append(f"{object_id}: missing stable ring evidence")
        fallback = row.get("fallback_contract", {})
        if not fallback.get("keeps_core_available", False):
            findings.append(f"{object_id}: fallback does not preserve local core")

    for route_id, row in routes.items():
        for field in (
            "provider_entry_ref",
            "model_entry_ref",
            "prompt_pack_version_ref",
            "tool_schema_pack_range_ref",
            "routing_policy_version_ref",
            "graduation_packet_ref",
            "mirror_publication_ref",
            "support_export_ref",
        ):
            if not non_empty(row.get(field)):
                findings.append(f"{route_id}: missing {field}")
        if row.get("route_origin_class") == "local_model" and not non_empty(
            row.get("local_model_pack_provenance_ref")
        ):
            findings.append(f"{route_id}: local model route missing pack provenance")
        if not row.get("independent_rollback_refs"):
            findings.append(f"{route_id}: missing independent rollback refs")

        object_kinds = set()
        for object_ref in row.get("rollout_object_refs", []):
            object_row = objects.get(object_ref)
            if object_row is None:
                findings.append(f"{route_id}: unknown rollout object {object_ref}")
                continue
            object_kinds.add(object_row.get("object_kind"))
        missing_kinds = REQUIRED_OBJECT_KINDS - object_kinds
        if row.get("route_origin_class") == "local_model":
            missing_kinds -= {"local_model_pack"}
            if "local_model_pack" not in object_kinds:
                findings.append(f"{route_id}: missing local_model_pack rollout object")
        if missing_kinds:
            findings.append(f"{route_id}: missing rollout object kinds {sorted(missing_kinds)}")
        if not row.get("fallback_contract", {}).get("keeps_core_available", False):
            findings.append(f"{route_id}: fallback does not preserve local core")

    receipts = packet.get("downgrade_receipts", [])
    receipt_objects = {row.get("withdrawn_object_ref") for row in receipts}
    for object_id, row in objects.items():
        if row.get("rollout_state") in {"withdrawn", "disabled"} and object_id not in receipt_objects:
            findings.append(f"{object_id}: withdrawn or disabled object lacks downgrade receipt")

    for receipt in receipts:
        receipt_id = receipt.get("receipt_id", "<missing>")
        if receipt.get("withdrawn_object_ref") not in objects:
            findings.append(f"{receipt_id}: references unknown withdrawn object")
        for route_ref in receipt.get("affected_route_refs", []):
            if route_ref not in routes:
                findings.append(f"{receipt_id}: references unknown affected route {route_ref}")
        if receipt.get("general_product_outage") is not False:
            findings.append(f"{receipt_id}: AI withdrawal is marked as product outage")

    mirror = packet.get("mirror_publication", {})
    for field in (
        "approved_prompt_pack_refs",
        "approved_tool_schema_pack_refs",
        "approved_local_model_pack_refs",
        "provenance_manifest_ref",
        "compatibility_manifest_ref",
        "revocation_manifest_ref",
        "downgrade_manifest_ref",
        "offline_drill_refs",
        "air_gapped_profile_refs",
    ):
        value = mirror.get(field)
        if not value:
            findings.append(f"mirror: missing {field}")
    if mirror.get("vendor_network_required") is not False:
        findings.append("mirror: vendor network required")

    return findings


def validate_local_manifest(
    manifest: dict[str, Any], packet: dict[str, Any], packet_ref: str
) -> list[str]:
    findings: list[str] = []
    if manifest.get("record_kind") != "local_model_pack_publication_manifest":
        findings.append("local manifest: unsupported record_kind")
    if manifest.get("schema_version") != 1:
        findings.append("local manifest: unsupported schema_version")
    if manifest.get("source_rollout_packet_ref") != packet_ref:
        findings.append("local manifest: source rollout packet mismatch")

    approved = set(packet.get("mirror_publication", {}).get("approved_local_model_pack_refs", []))
    for row in manifest.get("model_packs", []):
        row_ref = row.get("rollout_object_ref")
        if row_ref not in approved:
            findings.append(f"{row_ref}: local model pack is not approved in mirror publication")
        if row.get("vendor_network_required") is not False:
            findings.append(f"{row_ref}: local model publication requires vendor network")
        for field in (
            "artifact_digest_ref",
            "model_hash_ref",
            "runtime_abi_ref",
            "license_provenance_ref",
            "rollback_or_withdraw_ref",
            "downgrade_contract_ref",
        ):
            if not non_empty(row.get(field)):
                findings.append(f"{row_ref}: missing {field}")

    for drill in manifest.get("offline_drills", []):
        drill_ref = drill.get("drill_ref", "<missing>")
        if drill.get("validated_without_vendor_network") is not True:
            findings.append(f"{drill_ref}: drill depends on vendor network")
        preserved = set(drill.get("preserved_fields", []))
        if {"provenance", "compatibility", "revocation", "downgrade"} - preserved:
            findings.append(f"{drill_ref}: drill does not preserve all required fields")

    return findings


def validate_support_export(support_export: dict[str, Any], packet: dict[str, Any]) -> list[str]:
    findings: list[str] = []
    packet_routes = {row["route_id"] for row in packet.get("stable_routes", [])}
    support_routes = {row.get("route_id") for row in support_export.get("stable_routes", [])}
    if packet_routes != support_routes:
        findings.append("support export: route set does not match packet")
    if support_export.get("mirror_publication", {}).get("vendor_network_required") is not False:
        findings.append("support export: mirror publication requires vendor network")
    return findings


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--packet", default=DEFAULT_PACKET)
    parser.add_argument("--local-manifest", default=DEFAULT_LOCAL_MANIFEST)
    parser.add_argument("--support-export", default=DEFAULT_SUPPORT_EXPORT)
    args = parser.parse_args()

    repo_root = Path(args.repo_root)
    packet = load_json(repo_root / args.packet)
    local_manifest = load_json(repo_root / args.local_manifest)
    support_export = load_json(repo_root / args.support_export)

    findings = []
    findings.extend(validate_packet(packet))
    findings.extend(validate_local_manifest(local_manifest, packet, args.packet))
    findings.extend(validate_support_export(support_export, packet))

    if findings:
        for finding in findings:
            print(f"ERROR: {finding}")
        return 1

    print("AI pack promotion artifacts validate")
    return 0


if __name__ == "__main__":
    sys.exit(main())
