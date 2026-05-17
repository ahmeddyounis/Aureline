#!/usr/bin/env python3
"""Validate M3 preview-row packets and their support-export projection."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_PACKET_REL = "artifacts/compat/m3/preview_row_packets/preview_row_packets.json"
DEFAULT_SUPPORT_REL = (
    "artifacts/compat/m3/preview_row_packets/support_export_projection.json"
)
DEFAULT_CLAIM_MANIFEST_REL = "artifacts/release/m3/claim_manifest.json"
DEFAULT_CLAIMED_SURFACE_REGISTER_REL = (
    "artifacts/milestones/m3/claimed_surface_register.json"
)
DEFAULT_CAPTURE_REL = (
    "artifacts/compat/m3/captures/preview_row_packet_validation_capture.json"
)

EXPECTED_PREVIEW_CANONICAL_FAMILIES = {"benchmark_publication", "launch_wedge"}
EXPECTED_ROW_KINDS = {"beta_surface_binding", "beta_archetype_binding"}
REQUIRED_SUPPORT_SURFACES = {"support_export"}
INSPECTABLE_SURFACES = {"cli_headless", "docs_help", "release_packet"}
REQUIRED_LANES = (
    "notebook_trust",
    "repair_posture",
    "install_review",
    "compatibility_label",
    "activation_budget",
)
PATH_LIKE_SUFFIXES = (
    ".json",
    ".yaml",
    ".yml",
    ".md",
    ".schema.json",
)
ID_PREFIXES = (
    "activation-budget:",
    "beta_archetype:",
    "beta_scope.",
    "beta_surface:",
    "claim_manifest:",
    "compat_row:",
    "packet:",
    "preview-row:",
    "projection:",
    "support-export:",
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
    parser.add_argument("--packet", default=DEFAULT_PACKET_REL)
    parser.add_argument("--support-projection", default=DEFAULT_SUPPORT_REL)
    parser.add_argument("--claim-manifest", default=DEFAULT_CLAIM_MANIFEST_REL)
    parser.add_argument(
        "--claimed-surface-register", default=DEFAULT_CLAIMED_SURFACE_REGISTER_REL
    )
    parser.add_argument("--capture", default=DEFAULT_CAPTURE_REL)
    parser.add_argument(
        "--check",
        action="store_true",
        help="Fail if the checked-in validation capture would change.",
    )
    return parser.parse_args()


def load_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing JSON file: {path}")
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON at {path}: {exc}") from exc


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be an object")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be an array")
    return value


def ensure_str(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value.strip():
        raise SystemExit(f"{label} must be a non-empty string")
    return value.strip()


def parse_date(value: str, label: str) -> dt.date:
    try:
        return dt.date.fromisoformat(value)
    except ValueError as exc:
        raise SystemExit(f"{label} must be YYYY-MM-DD, got {value!r}") from exc


def looks_like_path(ref: str) -> bool:
    clean = ref.split("#", 1)[0].strip()
    if not clean or clean.startswith(ID_PREFIXES):
        return False
    return "/" in clean or clean.endswith(PATH_LIKE_SUFFIXES)


def is_reference_key(key: str) -> bool:
    return key.endswith("_ref") or key.endswith("_refs") or key in {
        "docs_ref",
        "fixture_dir_ref",
        "packet_ref",
        "projection_ref",
        "source_ref",
        "source_packet_ref",
        "support_export_projection_ref",
    }


def collect_path_refs(value: Any, *, parent_key: str | None = None) -> list[str]:
    refs: list[str] = []
    if isinstance(value, dict):
        for key, nested in value.items():
            refs.extend(collect_path_refs(nested, parent_key=key))
    elif isinstance(value, list):
        for nested in value:
            refs.extend(collect_path_refs(nested, parent_key=parent_key))
    elif isinstance(value, str) and parent_key and is_reference_key(parent_key) and looks_like_path(value):
        refs.append(value)
    return refs


def validate_path_refs(repo_root: Path, packet: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    seen: set[str] = set()
    for ref in collect_path_refs(packet):
        clean = ref.split("#", 1)[0].strip()
        if clean in seen:
            continue
        seen.add(clean)
        if clean.startswith("not_yet_seeded"):
            continue
        if not (repo_root / clean).exists():
            findings.append(
                Finding(
                    severity="error",
                    check_id="path_ref.missing",
                    message=f"referenced path does not exist: {clean}",
                    remediation="Seed the referenced artifact or correct the packet ref.",
                    ref=ref,
                )
            )
    return findings


def expected_manifest_rows(manifest: dict[str, Any]) -> dict[str, dict[str, Any]]:
    indexed: dict[str, dict[str, Any]] = {}
    for raw_row in ensure_list(manifest.get("rows"), "claim_manifest.rows"):
        row = ensure_dict(raw_row, "claim_manifest.rows[]")
        row_id = ensure_str(row.get("row_id"), "claim_manifest.rows[].row_id")
        row_kind = ensure_str(row.get("row_kind"), f"{row_id}.row_kind")
        claim_family = ensure_str(row.get("claim_family"), f"{row_id}.claim_family")
        lifecycle = ensure_dict(row.get("lifecycle"), f"{row_id}.lifecycle")
        lifecycle_label = ensure_str(
            lifecycle.get("display_lifecycle_label"),
            f"{row_id}.lifecycle.display_lifecycle_label",
        )
        if row_kind in EXPECTED_ROW_KINDS or (
            lifecycle_label == "preview"
            and claim_family in EXPECTED_PREVIEW_CANONICAL_FAMILIES
        ):
            indexed[row_id] = row
    return indexed


def validate_row_against_manifest(
    row: dict[str, Any],
    manifest_row: dict[str, Any] | None,
    findings: list[Finding],
) -> None:
    row_id = ensure_str(row.get("row_id"), "packet.rows[].row_id")
    if manifest_row is None:
        return

    expected = {
        "row_kind": manifest_row.get("row_kind"),
        "lifecycle_label": ensure_dict(manifest_row.get("lifecycle"), f"{row_id}.lifecycle").get(
            "display_lifecycle_label"
        ),
        "claim_posture_effective": ensure_dict(
            manifest_row.get("claim_posture"), f"{row_id}.claim_posture"
        ).get("effective"),
        "support_effective": ensure_dict(manifest_row.get("support"), f"{row_id}.support").get(
            "effective"
        ),
    }
    for field_name, expected_value in expected.items():
        if row.get(field_name) != expected_value:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"row.{field_name}.drift",
                    message=(
                        f"{row_id} {field_name}={row.get(field_name)!r} does not match "
                        f"claim manifest value {expected_value!r}"
                    ),
                    remediation="Refresh the preview-row packet from the claim manifest.",
                    ref=row_id,
                )
            )


def validate_packet_rows(
    packet: dict[str, Any],
    manifest_rows: dict[str, dict[str, Any]],
    claimed_surface_register: dict[str, Any],
) -> list[Finding]:
    findings: list[Finding] = []
    rows = ensure_list(packet.get("rows"), "packet.rows")
    seen: set[str] = set()

    expected_ids = set(manifest_rows)
    actual_ids: set[str] = set()
    for raw_row in rows:
        row = ensure_dict(raw_row, "packet.rows[]")
        row_id = ensure_str(row.get("row_id"), "packet.rows[].row_id")
        actual_ids.add(row_id)
        if row_id in seen:
            findings.append(
                Finding(
                    severity="error",
                    check_id="row.duplicate",
                    message=f"duplicate preview-row packet row: {row_id}",
                    remediation="Keep one packet row per claimed row.",
                    ref=row_id,
                )
            )
        seen.add(row_id)
        validate_row_against_manifest(row, manifest_rows.get(row_id), findings)

        packet_state = ensure_str(row.get("packet_state"), f"{row_id}.packet_state")
        if packet_state not in {"current_packet", "downgraded_out_of_claimed_set"}:
            findings.append(
                Finding(
                    severity="error",
                    check_id="row.packet_state.unknown",
                    message=f"{row_id} has unknown packet_state {packet_state!r}",
                    remediation="Use current_packet or downgraded_out_of_claimed_set.",
                    ref=row_id,
                )
            )

        for lane_name in REQUIRED_LANES:
            lane = ensure_dict(row.get(lane_name), f"{row_id}.{lane_name}")
            state = ensure_str(lane.get("state"), f"{row_id}.{lane_name}.state")
            if state in {"missing", "unknown"}:
                findings.append(
                    Finding(
                        severity="error",
                        check_id=f"{lane_name}.missing",
                        message=f"{row_id} has no explicit {lane_name} state",
                        remediation="Populate a current, not_applicable, or downgraded lane state.",
                        ref=row_id,
                    )
                )

        if packet_state == "current_packet":
            label_state = ensure_str(
                ensure_dict(row.get("compatibility_label"), f"{row_id}.compatibility_label").get(
                    "state"
                ),
                f"{row_id}.compatibility_label.state",
            )
            budget_state = ensure_str(
                ensure_dict(row.get("activation_budget"), f"{row_id}.activation_budget").get(
                    "state"
                ),
                f"{row_id}.activation_budget.state",
            )
            if label_state != "current" and budget_state != "current":
                findings.append(
                    Finding(
                        severity="error",
                        check_id="row.compat_or_budget.current_missing",
                        message=(
                            f"{row_id} must carry current compatibility-label or "
                            "activation-budget truth"
                        ),
                        remediation="Refresh the label or budget packet, or downgrade the row.",
                        ref=row_id,
                    )
                )

            projections = ensure_list(row.get("consumer_projections"), f"{row_id}.consumer_projections")
            surfaces = {
                ensure_str(proj.get("surface"), f"{row_id}.consumer_projections[].surface")
                for proj in (ensure_dict(proj, f"{row_id}.consumer_projections[]") for proj in projections)
            }
            if not REQUIRED_SUPPORT_SURFACES.issubset(surfaces):
                findings.append(
                    Finding(
                        severity="error",
                        check_id="row.support_export_projection.missing",
                        message=f"{row_id} is not projected to support export",
                        remediation="Add a support_export consumer projection.",
                        ref=row_id,
                    )
                )
            if not surfaces.intersection(INSPECTABLE_SURFACES):
                findings.append(
                    Finding(
                        severity="error",
                        check_id="row.inspectable_projection.missing",
                        message=f"{row_id} lacks a CLI, docs/help, or release-packet projection",
                        remediation="Add at least one inspectable non-prose projection.",
                        ref=row_id,
                    )
                )
            if row.get("raw_private_material_excluded") is not True:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="row.raw_private_material",
                        message=f"{row_id} does not explicitly exclude raw private material",
                        remediation="Set raw_private_material_excluded to true.",
                        ref=row_id,
                    )
                )
            if row.get("ambient_authority_excluded") is not True:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="row.ambient_authority",
                        message=f"{row_id} does not explicitly exclude ambient authority",
                        remediation="Set ambient_authority_excluded to true.",
                        ref=row_id,
                    )
                )

    missing = sorted(expected_ids - actual_ids)
    if missing:
        findings.append(
            Finding(
                severity="error",
                check_id="packet.expected_rows_missing",
                message="preview-row packet omits claimed runtime/ecosystem rows",
                remediation="Add current packets or downgrade rows out of the claimed set.",
                details={"missing_row_ids": missing},
            )
        )

    held = {
        ensure_str(row.get("row_id"), "held_or_out_of_scope_surfaces[].row_id")
        for row in (
            ensure_dict(row, "held_or_out_of_scope_surfaces[]")
            for row in ensure_list(
                claimed_surface_register.get("held_or_out_of_scope_surfaces"),
                "claimed_surface_register.held_or_out_of_scope_surfaces",
            )
        )
    }
    for row_id in actual_ids - expected_ids:
        if row_id not in held:
            findings.append(
                Finding(
                    severity="error",
                    check_id="packet.unexpected_row",
                    message=f"{row_id} is neither in the claim manifest nor held/out of scope",
                    remediation="Remove the row or add it to the governing register.",
                    ref=row_id,
                )
            )
    return findings


def validate_support_projection(
    packet: dict[str, Any],
    support: dict[str, Any],
) -> list[Finding]:
    findings: list[Finding] = []
    if support.get("record_kind") != "m3_preview_row_support_export_projection":
        findings.append(
            Finding(
                severity="error",
                check_id="support.record_kind",
                message="support projection has the wrong record_kind",
                remediation="Use m3_preview_row_support_export_projection.",
            )
        )
    if support.get("source_packet_ref") != packet.get("packet_ref"):
        findings.append(
            Finding(
                severity="error",
                check_id="support.source_packet_ref",
                message="support projection does not point at the packet ref",
                remediation="Set source_packet_ref to the preview-row packet path.",
            )
        )

    packet_rows = {row["row_id"]: row for row in ensure_list(packet.get("rows"), "packet.rows")}
    support_rows = {
        ensure_str(row.get("row_id"), "support.rows[].row_id"): ensure_dict(
            row, "support.rows[]"
        )
        for row in ensure_list(support.get("rows"), "support.rows")
    }
    if set(packet_rows) != set(support_rows):
        findings.append(
            Finding(
                severity="error",
                check_id="support.row_set",
                message="support projection row set differs from packet rows",
                remediation="Regenerate the support projection from the packet rows.",
                details={
                    "missing_in_support": sorted(set(packet_rows) - set(support_rows)),
                    "extra_in_support": sorted(set(support_rows) - set(packet_rows)),
                },
            )
        )
    for row_id, packet_row in packet_rows.items():
        support_row = support_rows.get(row_id)
        if not support_row:
            continue
        comparisons = {
            "packet_state": packet_row.get("packet_state"),
            "lifecycle_label": packet_row.get("lifecycle_label"),
            "support_effective": packet_row.get("support_effective"),
            "notebook_trust_state": packet_row.get("notebook_trust", {}).get("state"),
            "repair_state": packet_row.get("repair_posture", {}).get("state"),
            "install_review_state": packet_row.get("install_review", {}).get("state"),
            "compatibility_label_state": packet_row.get("compatibility_label", {}).get("state"),
            "activation_budget_state": packet_row.get("activation_budget", {}).get("state"),
            "promotion_gate_state": packet_row.get("promotion_gate", {}).get("state"),
        }
        for field_name, expected_value in comparisons.items():
            if support_row.get(field_name) != expected_value:
                findings.append(
                    Finding(
                        severity="error",
                        check_id=f"support.{field_name}.drift",
                        message=f"{row_id} support {field_name} drifted from packet",
                        remediation="Regenerate the support projection from the packet rows.",
                        ref=row_id,
                    )
                )
        if support_row.get("raw_private_material_excluded") is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id="support.raw_private_material",
                    message=f"{row_id} support projection does not exclude raw private material",
                    remediation="Set raw_private_material_excluded to true.",
                    ref=row_id,
                )
            )
    return findings


def build_capture(
    packet: dict[str, Any],
    support: dict[str, Any],
    manifest: dict[str, Any],
    findings: list[Finding],
) -> dict[str, Any]:
    expected_rows = sorted(expected_manifest_rows(manifest))
    packet_rows = ensure_list(packet.get("rows"), "packet.rows")
    status = "pass" if not any(f.severity == "error" for f in findings) else "fail"
    return {
        "record_kind": "m3_preview_row_packet_validation_capture",
        "schema_version": 1,
        "captured_at": packet.get("generated_at"),
        "packet_ref": packet.get("packet_ref"),
        "support_projection_ref": support.get("projection_ref"),
        "source_claim_manifest_ref": packet.get("source_claim_manifest_ref"),
        "expected_claim_row_ids": expected_rows,
        "packet_row_count": len(packet_rows),
        "support_projection_row_count": len(ensure_list(support.get("rows"), "support.rows")),
        "current_packet_row_count": sum(
            1 for row in packet_rows if row.get("packet_state") == "current_packet"
        ),
        "downgraded_out_row_count": sum(
            1
            for row in packet_rows
            if row.get("packet_state") == "downgraded_out_of_claimed_set"
        ),
        "status": status,
        "findings": [finding.as_report() for finding in findings],
    }


def write_or_check_capture(path: Path, capture: dict[str, Any], check: bool) -> None:
    rendered = json.dumps(capture, indent=2, sort_keys=True) + "\n"
    if check:
        if not path.exists():
            raise SystemExit(f"validation capture is missing: {path}")
        existing = path.read_text(encoding="utf-8")
        if existing != rendered:
            raise SystemExit(
                f"validation capture drifted: {path}\n"
                "Run ci/check_m3_preview_row_packets.py --repo-root ."
            )
    else:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(rendered, encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    packet = ensure_dict(load_json(repo_root / args.packet), "packet")
    support = ensure_dict(load_json(repo_root / args.support_projection), "support")
    manifest = ensure_dict(load_json(repo_root / args.claim_manifest), "claim_manifest")
    claimed_surface_register = ensure_dict(
        load_json(repo_root / args.claimed_surface_register), "claimed_surface_register"
    )

    if packet.get("record_kind") != "m3_preview_row_packet_register":
        raise SystemExit("packet.record_kind must be m3_preview_row_packet_register")
    if packet.get("schema_version") != 1:
        raise SystemExit("packet.schema_version must be 1")
    parse_date(ensure_str(packet.get("as_of"), "packet.as_of"), "packet.as_of")

    findings: list[Finding] = []
    findings.extend(validate_path_refs(repo_root, packet))
    findings.extend(validate_path_refs(repo_root, support))
    findings.extend(
        validate_packet_rows(packet, expected_manifest_rows(manifest), claimed_surface_register)
    )
    findings.extend(validate_support_projection(packet, support))

    capture = build_capture(packet, support, manifest, findings)
    write_or_check_capture(repo_root / args.capture, capture, args.check)

    if any(finding.severity == "error" for finding in findings):
        print(json.dumps(capture, indent=2, sort_keys=True), file=sys.stderr)
        return 1
    print(
        f"validated {capture['packet_row_count']} preview-row packet rows "
        f"({capture['current_packet_row_count']} current, "
        f"{capture['downgraded_out_row_count']} downgraded out)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
