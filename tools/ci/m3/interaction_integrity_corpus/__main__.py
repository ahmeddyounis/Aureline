#!/usr/bin/env python3
"""Validate and generate the interaction-integrity conformance packet."""

from __future__ import annotations

import argparse
import dataclasses
import json
import sys
from pathlib import Path
from typing import Any

from tools.ci.m3._common import render_yaml_as_json


DEFAULT_MANIFEST_REL = "fixtures/qe/m3/interaction_integrity_corpus/manifest.yaml"
DEFAULT_PACKET_REL = "artifacts/qe/m3/interaction_integrity_packets/conformance_packet.json"
DEFAULT_RELEASE_SNAPSHOT_REL = (
    "artifacts/qe/m3/interaction_integrity_packets/release_snapshot.json"
)
DEFAULT_SUPPORT_PROJECTION_REL = (
    "artifacts/qe/m3/interaction_integrity_packets/support_export_projection.json"
)
DEFAULT_CAPTURE_REL = (
    "artifacts/qe/m3/interaction_integrity_packets/captures/"
    "interaction_integrity_validation_capture.json"
)
DEFAULT_CLAIM_MANIFEST_REL = "artifacts/release/m3/claim_manifest.json"
DEFAULT_CLAIMED_SURFACE_REGISTER_REL = "artifacts/milestones/m3/claimed_surface_register.json"

EXPECTED_SCHEMA_VERSION = 1
EXPECTED_MANIFEST_RECORD_KIND = "interaction_integrity_conformance_manifest"
EXPECTED_CASE_RECORD_KIND = "interaction_integrity_conformance_case"
EXPECTED_PACKET_RECORD_KIND = "interaction_integrity_conformance_packet"
EXPECTED_RELEASE_SNAPSHOT_RECORD_KIND = "interaction_integrity_release_snapshot"
EXPECTED_SUPPORT_PROJECTION_RECORD_KIND = "interaction_integrity_support_export_projection"
EXPECTED_CAPTURE_RECORD_KIND = "interaction_integrity_validation_capture"

REQUIRED_CONTROL_CLASSES = {
    "focus_batch_scope",
    "preview_drift_invalidation",
    "permission_expiry",
    "safe_preview_copy_export_parity",
    "host_boundary_cues",
    "responsive_fallback",
}

BLOCKING_STATES = {"missing_packet", "stale_packet", "red_packet", "expired_waiver"}
PATH_LIKE_SUFFIXES = (".json", ".yaml", ".yml", ".md", ".rs", ".py", ".schema.json")
OPAQUE_PREFIXES = (
    "approval:",
    "approval_ticket:",
    "batch_scope:",
    "beta_surface:",
    "claim_row:",
    "conformance:",
    "control:",
    "evidence:",
    "host_boundary:",
    "interaction_integrity.",
    "packet:",
    "policy:",
    "preview:",
    "scenario:",
    "shell:",
    "support_export:",
    "surface:",
)


@dataclasses.dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    remediation: str
    ref: str | None = None
    details: dict[str, Any] = dataclasses.field(default_factory=dict)

    def as_report(self) -> dict[str, Any]:
        payload = dataclasses.asdict(self)
        if payload["ref"] is None:
            payload.pop("ref")
        if not payload["details"]:
            payload.pop("details")
        return payload


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--manifest", default=DEFAULT_MANIFEST_REL)
    parser.add_argument("--packet", default=DEFAULT_PACKET_REL)
    parser.add_argument("--release-snapshot", default=DEFAULT_RELEASE_SNAPSHOT_REL)
    parser.add_argument("--support-projection", default=DEFAULT_SUPPORT_PROJECTION_REL)
    parser.add_argument("--capture", default=DEFAULT_CAPTURE_REL)
    parser.add_argument("--claim-manifest", default=DEFAULT_CLAIM_MANIFEST_REL)
    parser.add_argument(
        "--claimed-surface-register", default=DEFAULT_CLAIMED_SURFACE_REGISTER_REL
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="Fail if generated packets or capture would change.",
    )
    return parser.parse_args()


def load_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing JSON file: {path}")
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON at {path}: {exc}") from exc


def load_payload(path: Path) -> Any:
    suffix = path.suffix.lower()
    if suffix == ".json":
        return load_json(path)
    if suffix in {".yaml", ".yml"}:
        return render_yaml_as_json(path)
    raise SystemExit(f"unsupported payload extension for {path}")


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be an object")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a list")
    return value


def ensure_str(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value.strip():
        raise SystemExit(f"{label} must be a non-empty string")
    return value.strip()


def optional_str(value: Any) -> str | None:
    if value is None:
        return None
    return ensure_str(value, "optional string")


def strip_fragment(ref: str) -> str:
    return ref.split("#", 1)[0].strip()


def looks_like_repo_ref(ref: Any) -> bool:
    if not isinstance(ref, str) or not ref.strip():
        return False
    clean = strip_fragment(ref)
    if clean.startswith(OPAQUE_PREFIXES):
        return False
    return "/" in clean or clean.endswith(PATH_LIKE_SUFFIXES)


def collect_reference_strings(value: Any, *, parent_key: str | None = None) -> list[str]:
    refs: list[str] = []
    if isinstance(value, dict):
        for key, nested in value.items():
            refs.extend(collect_reference_strings(nested, parent_key=key))
    elif isinstance(value, list):
        for nested in value:
            refs.extend(collect_reference_strings(nested, parent_key=parent_key))
    elif isinstance(value, str) and parent_key:
        if parent_key.endswith("_ref") or parent_key.endswith("_refs") or parent_key in {
            "case_ref",
            "script_ref",
        }:
            refs.append(value)
    return refs


def validate_repo_refs(
    repo_root: Path,
    owner: dict[str, Any],
    findings: list[Finding],
    generated_refs: set[str],
    owner_ref: str,
) -> None:
    seen: set[str] = set()
    for ref in collect_reference_strings(owner):
        if not looks_like_repo_ref(ref):
            continue
        clean = strip_fragment(ref)
        if clean in seen or clean in generated_refs:
            continue
        seen.add(clean)
        if not (repo_root / clean).exists():
            findings.append(
                Finding(
                    "error",
                    "path_ref.missing",
                    f"referenced repo path does not exist: {clean}",
                    "Seed the referenced artifact or correct the conformance reference.",
                    owner_ref,
                    {"ref": ref},
                )
            )


def render_json(payload: dict[str, Any]) -> str:
    return json.dumps(payload, indent=2, sort_keys=True) + "\n"


def write_or_check(path: Path, content: str, check: bool) -> bool:
    if check:
        if not path.exists() or path.read_text(encoding="utf-8") != content:
            print(f"would update {path}", file=sys.stderr)
            return False
        return True
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")
    return True


def load_cases(repo_root: Path, manifest: dict[str, Any]) -> list[dict[str, Any]]:
    cases: list[dict[str, Any]] = []
    for ref in ensure_list(manifest.get("case_refs"), "manifest.case_refs"):
        case_ref = ensure_str(ref, "manifest.case_refs[]")
        case_path = repo_root / strip_fragment(case_ref)
        cases.append(ensure_dict(load_payload(case_path), case_ref))
    return cases


def index_claim_rows(claim_manifest: dict[str, Any]) -> dict[str, dict[str, Any]]:
    indexed: dict[str, dict[str, Any]] = {}
    for row in ensure_list(claim_manifest.get("rows"), "claim_manifest.rows"):
        if not isinstance(row, dict):
            continue
        beta_surface_ref = row.get("beta_surface_ref")
        if isinstance(beta_surface_ref, str) and beta_surface_ref:
            indexed[beta_surface_ref] = row
    return indexed


def index_claimed_surfaces(register: dict[str, Any]) -> dict[str, dict[str, Any]]:
    indexed: dict[str, dict[str, Any]] = {}
    for row in ensure_list(register.get("claimed_surfaces"), "claimed_surface_register.claimed_surfaces"):
        if not isinstance(row, dict):
            continue
        surface_id = ensure_str(row.get("surface_id"), "claimed_surfaces[].surface_id")
        indexed[surface_id] = row
    return indexed


def validate_manifest(
    repo_root: Path,
    manifest: dict[str, Any],
    cases: list[dict[str, Any]],
    generated_refs: set[str],
    findings: list[Finding],
) -> None:
    if manifest.get("record_kind") != EXPECTED_MANIFEST_RECORD_KIND:
        findings.append(
            Finding(
                "error",
                "manifest.record_kind",
                "manifest record_kind is not the interaction-integrity conformance manifest",
                "Set record_kind to interaction_integrity_conformance_manifest.",
                DEFAULT_MANIFEST_REL,
            )
        )
    if manifest.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(
            Finding(
                "error",
                "manifest.schema_version",
                "manifest schema_version must be 1",
                "Update the manifest or validator together.",
                DEFAULT_MANIFEST_REL,
            )
        )

    declared_controls = {
        ensure_str(row.get("control_class"), "manifest.required_controls[].control_class")
        for row in ensure_list(manifest.get("required_controls"), "manifest.required_controls")
        if isinstance(row, dict)
    }
    missing_controls = sorted(REQUIRED_CONTROL_CLASSES - declared_controls)
    extra_controls = sorted(declared_controls - REQUIRED_CONTROL_CLASSES)
    if missing_controls or extra_controls:
        findings.append(
            Finding(
                "error",
                "manifest.control_vocabulary",
                "manifest controls do not match the required closed set",
                "Declare exactly the required interaction-integrity control classes.",
                DEFAULT_MANIFEST_REL,
                {"missing": missing_controls, "extra": extra_controls},
            )
        )

    expected_case_ids = set(ensure_list(manifest.get("expected_case_ids"), "manifest.expected_case_ids"))
    actual_case_ids = {case.get("case_id") for case in cases}
    if expected_case_ids != actual_case_ids:
        findings.append(
            Finding(
                "error",
                "manifest.case_id_set",
                "case ids do not match manifest expected_case_ids",
                "Update expected_case_ids and case_refs together.",
                DEFAULT_MANIFEST_REL,
                {
                    "missing": sorted(expected_case_ids - actual_case_ids),
                    "extra": sorted(actual_case_ids - expected_case_ids),
                },
            )
        )

    validate_repo_refs(repo_root, manifest, findings, generated_refs, DEFAULT_MANIFEST_REL)


def validate_case(
    repo_root: Path,
    case: dict[str, Any],
    generated_refs: set[str],
    findings: list[Finding],
) -> None:
    case_id = ensure_str(case.get("case_id"), "case.case_id")
    if case.get("record_kind") != EXPECTED_CASE_RECORD_KIND:
        findings.append(
            Finding(
                "error",
                "case.record_kind",
                f"{case_id} has wrong record_kind",
                "Set record_kind to interaction_integrity_conformance_case.",
                case_id,
            )
        )
    if case.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(
            Finding(
                "error",
                "case.schema_version",
                f"{case_id} schema_version must be 1",
                "Update the case or validator together.",
                case_id,
            )
        )

    control_class = ensure_str(case.get("control_class"), f"{case_id}.control_class")
    if control_class not in REQUIRED_CONTROL_CLASSES:
        findings.append(
            Finding(
                "error",
                "case.control_class",
                f"{case_id} uses unknown control class {control_class}",
                "Use the closed control vocabulary from the manifest.",
                case_id,
            )
        )

    drill = ensure_dict(case.get("scripted_drill"), f"{case_id}.scripted_drill")
    assertion_mode = ensure_str(drill.get("assertion_mode"), f"{case_id}.assertion_mode")
    if assertion_mode != "object_packet_assertions":
        findings.append(
            Finding(
                "error",
                "case.assertion_mode",
                f"{case_id} does not use object-packet assertions",
                "Use object_packet_assertions; screenshot-only proof is not accepted.",
                case_id,
            )
        )
    if drill.get("screenshot_diff_policy") == "required_primary":
        findings.append(
            Finding(
                "error",
                "case.screenshot_primary",
                f"{case_id} makes screenshot diff the primary proof",
                "Use screenshot diff only as supplemental evidence.",
                case_id,
            )
        )

    steps = ensure_list(drill.get("steps"), f"{case_id}.scripted_drill.steps")
    if not steps:
        findings.append(
            Finding(
                "error",
                "case.steps_missing",
                f"{case_id} has no scripted drill steps",
                "Add at least one object-based drill step.",
                case_id,
            )
        )
    for step in steps:
        step_obj = ensure_dict(step, f"{case_id}.steps[]")
        assertions = ensure_list(step_obj.get("assertions"), f"{case_id}.steps[].assertions")
        if not assertions:
            findings.append(
                Finding(
                    "error",
                    "case.step_assertions_missing",
                    f"{case_id} step has no assertions",
                    "Add explicit assertions for the drill step.",
                    case_id,
                )
            )

    coverage = ensure_dict(case.get("coverage"), f"{case_id}.coverage")
    covered_claims = ensure_list(
        coverage.get("covered_claim_surface_refs"), f"{case_id}.coverage.covered_claim_surface_refs"
    )
    if not covered_claims:
        findings.append(
            Finding(
                "error",
                "case.claim_surface_coverage_missing",
                f"{case_id} does not name any claim surfaces",
                "Bind the case to the marketed rows it gates.",
                case_id,
            )
        )

    conformance = ensure_dict(case.get("conformance"), f"{case_id}.conformance")
    if conformance.get("result") != "pass":
        findings.append(
            Finding(
                "error",
                "case.red_packet",
                f"{case_id} conformance result is not pass",
                "Fix the underlying control or mark the marketed row blocked.",
                case_id,
            )
        )
    if conformance.get("packet_freshness_state") != "current":
        findings.append(
            Finding(
                "error",
                "case.stale_packet",
                f"{case_id} packet freshness is not current",
                "Refresh the corpus packet before widening beta rows.",
                case_id,
            )
        )
    waiver = ensure_dict(conformance.get("waiver"), f"{case_id}.conformance.waiver")
    waiver_posture = ensure_str(waiver.get("posture"), f"{case_id}.waiver.posture")
    if waiver_posture in {"expired", "withdrawn"}:
        findings.append(
            Finding(
                "error",
                "case.expired_waiver",
                f"{case_id} cites an expired or withdrawn waiver",
                "Renew the waiver through release review or fix the conformance gap.",
                case_id,
            )
        )

    release_gate = ensure_dict(case.get("release_gate"), f"{case_id}.release_gate")
    if release_gate.get("required_for_beta_widening") is not True:
        findings.append(
            Finding(
                "error",
                "case.beta_gate_missing",
                f"{case_id} is not marked required for beta widening",
                "Set required_for_beta_widening to true.",
                case_id,
            )
        )
    if release_gate.get("required_for_rc_promotion") is not True:
        findings.append(
            Finding(
                "error",
                "case.rc_gate_missing",
                f"{case_id} is not marked required for release-candidate promotion",
                "Set required_for_rc_promotion to true.",
                case_id,
            )
        )
    block_conditions = set(ensure_list(release_gate.get("block_conditions"), f"{case_id}.block_conditions"))
    if not BLOCKING_STATES.issubset(block_conditions):
        findings.append(
            Finding(
                "error",
                "case.block_conditions_incomplete",
                f"{case_id} does not block every required bad packet state",
                "Include missing_packet, stale_packet, red_packet, and expired_waiver.",
                case_id,
                {"missing": sorted(BLOCKING_STATES - block_conditions)},
            )
        )

    validate_repo_refs(repo_root, case, findings, generated_refs, case_id)


def validate_marketed_rows(
    manifest: dict[str, Any],
    cases: list[dict[str, Any]],
    claim_rows: dict[str, dict[str, Any]],
    claimed_surfaces: dict[str, dict[str, Any]],
    findings: list[Finding],
) -> list[dict[str, Any]]:
    case_by_control_and_surface: dict[tuple[str, str], list[str]] = {}
    for case in cases:
        case_id = ensure_str(case.get("case_id"), "case.case_id")
        control_class = ensure_str(case.get("control_class"), f"{case_id}.control_class")
        coverage = ensure_dict(case.get("coverage"), f"{case_id}.coverage")
        for surface_ref in ensure_list(
            coverage.get("covered_claim_surface_refs"),
            f"{case_id}.coverage.covered_claim_surface_refs",
        ):
            case_by_control_and_surface.setdefault(
                (control_class, ensure_str(surface_ref, f"{case_id}.surface_ref")), []
            ).append(case_id)

    rows: list[dict[str, Any]] = []
    for raw_row in ensure_list(manifest.get("marketed_row_gates"), "manifest.marketed_row_gates"):
        row = ensure_dict(raw_row, "manifest.marketed_row_gates[]")
        surface_ref = ensure_str(row.get("surface_ref"), "marketed_row_gates[].surface_ref")
        required_controls = [
            ensure_str(control, f"{surface_ref}.required_control_classes[]")
            for control in ensure_list(
                row.get("required_control_classes"),
                f"{surface_ref}.required_control_classes",
            )
        ]
        if surface_ref not in claimed_surfaces:
            findings.append(
                Finding(
                    "error",
                    "marketed_row.unknown_surface",
                    f"{surface_ref} is not in the claimed-surface register",
                    "Use a beta claimed-surface id from the register.",
                    surface_ref,
                )
            )
        if surface_ref not in claim_rows:
            findings.append(
                Finding(
                    "error",
                    "marketed_row.no_claim_manifest_row",
                    f"{surface_ref} does not resolve to a claim-manifest row",
                    "Refresh the claim manifest or correct the surface ref.",
                    surface_ref,
                )
            )
        missing = [
            control
            for control in required_controls
            if not case_by_control_and_surface.get((control, surface_ref))
        ]
        if missing:
            findings.append(
                Finding(
                    "error",
                    "marketed_row.control_gap",
                    f"{surface_ref} lacks required interaction-integrity coverage",
                    "Add a corpus case for every required control on this marketed row.",
                    surface_ref,
                    {"missing_control_classes": missing},
                )
            )
        resolved_case_ids = sorted(
            {
                case_id
                for control in required_controls
                for case_id in case_by_control_and_surface.get((control, surface_ref), [])
            }
        )
        claim_row = claim_rows.get(surface_ref, {})
        surface_row = claimed_surfaces.get(surface_ref, {})
        rows.append(
            {
                "surface_ref": surface_ref,
                "title": surface_row.get("title") or row.get("title"),
                "claim_manifest_row_ref": claim_row.get("row_id"),
                "claim_state": surface_row.get("claim_state"),
                "support_class": surface_row.get("support_class"),
                "required_control_classes": required_controls,
                "conformance_case_refs": resolved_case_ids,
                "packet_state": "green" if not missing else "red",
                "widening_gate": "pass" if not missing else "blocked",
                "rc_promotion_gate": "pass" if not missing else "blocked",
                "active_waiver_refs": [],
            }
        )
    return rows


def build_packet(
    manifest: dict[str, Any],
    cases: list[dict[str, Any]],
    marketed_rows: list[dict[str, Any]],
    findings: list[Finding],
    packet_rel: str,
    release_snapshot_rel: str,
    support_projection_rel: str,
) -> dict[str, Any]:
    case_rows = []
    active_waivers = []
    for case in cases:
        case_id = ensure_str(case.get("case_id"), "case.case_id")
        conformance = ensure_dict(case.get("conformance"), f"{case_id}.conformance")
        waiver = ensure_dict(conformance.get("waiver"), f"{case_id}.waiver")
        waiver_ref = optional_str(waiver.get("waiver_ref"))
        if waiver.get("posture") not in {None, "none"} and waiver_ref:
            active_waivers.append(waiver_ref)
        case_rows.append(
            {
                "case_id": case_id,
                "title": case.get("title"),
                "control_class": case.get("control_class"),
                "result": conformance.get("result"),
                "packet_freshness_state": conformance.get("packet_freshness_state"),
                "packet_color": conformance.get("packet_color"),
                "waiver": waiver,
                "covered_claim_surface_refs": ensure_dict(
                    case.get("coverage"), f"{case_id}.coverage"
                ).get("covered_claim_surface_refs", []),
                "fixture_refs": case.get("fixture_refs", []),
                "degraded_state_lineage": conformance.get("degraded_state_lineage", []),
            }
        )

    status = "green"
    if findings:
        status = "red"
    if any(row["packet_state"] != "green" for row in marketed_rows):
        status = "red"

    generated_at = ensure_str(manifest.get("generated_at"), "manifest.generated_at")
    return {
        "record_kind": EXPECTED_PACKET_RECORD_KIND,
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "packet_id": "interaction_integrity.conformance_packet.beta",
        "generated_at": generated_at,
        "as_of": manifest.get("as_of"),
        "owner": manifest.get("owner"),
        "overall_status": status,
        "blocking_posture": "widening_and_release_candidate_blocking",
        "manifest_ref": DEFAULT_MANIFEST_REL,
        "release_snapshot_ref": release_snapshot_rel,
        "support_export_projection_ref": support_projection_rel,
        "required_control_classes": sorted(REQUIRED_CONTROL_CLASSES),
        "summary": {
            "case_count": len(case_rows),
            "passing_case_count": sum(1 for row in case_rows if row["result"] == "pass"),
            "current_packet_count": sum(
                1 for row in case_rows if row["packet_freshness_state"] == "current"
            ),
            "marketed_row_count": len(marketed_rows),
            "blocked_marketed_row_count": sum(
                1 for row in marketed_rows if row["widening_gate"] != "pass"
            ),
            "active_waiver_count": len(active_waivers),
            "finding_count": len(findings),
        },
        "release_policy": {
            "missing_stale_or_red_packets_block_beta_widening": True,
            "missing_stale_or_red_packets_block_release_candidate_promotion": True,
            "active_waivers_must_be_time_boxed_and_named": True,
            "block_conditions": sorted(BLOCKING_STATES),
        },
        "case_results": case_rows,
        "marketed_row_gates": marketed_rows,
        "active_waiver_refs": sorted(active_waivers),
        "finding_refs": [finding.as_report() for finding in findings],
        "generated_outputs": {
            "conformance_packet": packet_rel,
            "release_snapshot": release_snapshot_rel,
            "support_projection": support_projection_rel,
        },
    }


def build_release_snapshot(packet: dict[str, Any]) -> dict[str, Any]:
    return {
        "record_kind": EXPECTED_RELEASE_SNAPSHOT_RECORD_KIND,
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "snapshot_id": "interaction_integrity.release_snapshot.beta",
        "generated_at": packet["generated_at"],
        "source_packet_ref": DEFAULT_PACKET_REL,
        "overall_status": packet["overall_status"],
        "blocking_posture": packet["blocking_posture"],
        "release_policy": packet["release_policy"],
        "marketed_row_gates": packet["marketed_row_gates"],
        "case_results": [
            {
                "case_id": row["case_id"],
                "control_class": row["control_class"],
                "result": row["result"],
                "packet_freshness_state": row["packet_freshness_state"],
                "packet_color": row["packet_color"],
                "waiver_posture": row["waiver"].get("posture"),
            }
            for row in packet["case_results"]
        ],
    }


def build_support_projection(packet: dict[str, Any]) -> dict[str, Any]:
    return {
        "record_kind": EXPECTED_SUPPORT_PROJECTION_RECORD_KIND,
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "support_projection_id": "interaction_integrity.support_projection.beta",
        "generated_at": packet["generated_at"],
        "source_packet_ref": DEFAULT_PACKET_REL,
        "overall_status": packet["overall_status"],
        "raw_private_material_excluded": True,
        "raw_object_payloads_excluded": True,
        "raw_clipboard_or_preview_bodies_excluded": True,
        "case_summaries": [
            {
                "case_id": row["case_id"],
                "control_class": row["control_class"],
                "result": row["result"],
                "covered_claim_surface_refs": row["covered_claim_surface_refs"],
                "degraded_state_lineage": row["degraded_state_lineage"],
                "waiver": row["waiver"],
            }
            for row in packet["case_results"]
        ],
        "marketed_row_gates": packet["marketed_row_gates"],
    }


def build_capture(packet: dict[str, Any], findings: list[Finding]) -> dict[str, Any]:
    return {
        "record_kind": EXPECTED_CAPTURE_RECORD_KIND,
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "capture_id": "interaction_integrity.validation_capture.beta",
        "generated_at": packet["generated_at"],
        "status": "pass" if not findings else "fail",
        "summary": packet["summary"],
        "findings": [finding.as_report() for finding in findings],
        "generated_outputs": packet["generated_outputs"],
    }


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    manifest_rel = args.manifest
    manifest = ensure_dict(load_payload(repo_root / manifest_rel), manifest_rel)
    generated_refs = {
        strip_fragment(args.packet),
        strip_fragment(args.release_snapshot),
        strip_fragment(args.support_projection),
        strip_fragment(args.capture),
    }
    cases = load_cases(repo_root, manifest)
    claim_manifest = ensure_dict(load_json(repo_root / args.claim_manifest), args.claim_manifest)
    claimed_surface_register = ensure_dict(
        load_json(repo_root / args.claimed_surface_register),
        args.claimed_surface_register,
    )
    claim_rows = index_claim_rows(claim_manifest)
    claimed_surfaces = index_claimed_surfaces(claimed_surface_register)

    findings: list[Finding] = []
    validate_manifest(repo_root, manifest, cases, generated_refs, findings)
    for case in cases:
        validate_case(repo_root, case, generated_refs, findings)
    marketed_rows = validate_marketed_rows(
        manifest, cases, claim_rows, claimed_surfaces, findings
    )

    packet = build_packet(
        manifest,
        cases,
        marketed_rows,
        findings,
        args.packet,
        args.release_snapshot,
        args.support_projection,
    )
    release_snapshot = build_release_snapshot(packet)
    support_projection = build_support_projection(packet)
    capture = build_capture(packet, findings)

    writes_ok = True
    writes_ok &= write_or_check(repo_root / args.packet, render_json(packet), args.check)
    writes_ok &= write_or_check(
        repo_root / args.release_snapshot, render_json(release_snapshot), args.check
    )
    writes_ok &= write_or_check(
        repo_root / args.support_projection, render_json(support_projection), args.check
    )
    writes_ok &= write_or_check(repo_root / args.capture, render_json(capture), args.check)

    if findings:
        for finding in findings:
            print(f"{finding.severity}: {finding.check_id}: {finding.message}", file=sys.stderr)
        return 1
    if not writes_ok:
        return 1
    print("ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
