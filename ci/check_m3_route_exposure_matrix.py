#!/usr/bin/env python3
"""Validate the beta route/exposure matrix and handoff support packet."""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_MATRIX_REL = "artifacts/routes/m3/route_exposure_matrix.json"
DEFAULT_SCHEMA_REL = "schemas/routes/exposure_matrix.schema.json"
DEFAULT_RELEASE_DOC_REL = "artifacts/release/m3/route_exposure_matrix.md"
DEFAULT_SUPPORT_AUDIT_REL = "artifacts/support/m3/provider_and_browser_handoff_audit.md"
DEFAULT_UX_DOC_REL = "docs/ux/m3/route_exposure_and_handoff_beta.md"
DEFAULT_CLAIMED_SURFACE_REGISTER_REL = "artifacts/milestones/m3/claimed_surface_register.json"
DEFAULT_PROVIDER_EXPORT_REL = "artifacts/security/m3/route_resolution_panels/baseline_support_export.json"
DEFAULT_CAPTURE_REL = "artifacts/release/m3/captures/route_exposure_matrix_validation_capture.json"

REQUIRED_ORIGIN_CLASSES = {
    "local_desktop",
    "remote_helper",
    "managed_workspace",
    "browser_companion",
    "provider_linked_context",
    "embedded_docs_help_webview",
    "headless_cli",
}
REQUIRED_CONSUMER_SURFACES = {
    "help_about",
    "service_health",
    "diagnostics",
    "support_export",
    "docs_help",
}
REQUIRED_REAPPROVAL_TRIGGER_COVERAGE = {
    "target_identity_changed",
    "trust_posture_changed",
    "policy_epoch_changed",
    "host_or_domain_changed",
    "privacy_consequence_changed",
}
PATH_LIKE_SUFFIXES = (".json", ".md", ".yaml", ".yml", ".schema.json")
ID_PREFIXES = (
    "account-scope-beta:",
    "approval-ticket:",
    "auth-callback:",
    "auth-handoff-interstitial:",
    "browser-handoff-packet:",
    "command-route:",
    "copy-export-evidence:",
    "desktop-",
    "embedded-boundary",
    "handoff:",
    "handoff-packet:",
    "managed-",
    "notebook-",
    "preview-",
    "provider-",
    "publish-",
    "review-",
    "route-",
    "snapshot-",
    "support.item.",
    "system-browser",
    "tunnel-",
    "voice-",
    "workspace:",
)

REQUIRED_RELEASE_DOC_PHRASES = [
    "Route/exposure matrix beta packet",
    "artifacts/routes/m3/route_exposure_matrix.json",
    "schemas/routes/exposure_matrix.schema.json",
    "Promotion guard",
    "ci/check_m3_route_exposure_matrix.py",
]
REQUIRED_SUPPORT_DOC_PHRASES = [
    "Provider and browser-handoff audit packet",
    "Support-export parity contract",
    "Reapproval audit",
    "metadata-only",
]
REQUIRED_UX_DOC_PHRASES = [
    "Route Exposure And Handoff Beta",
    "Help/About",
    "service health",
    "support exports",
    "python3 ci/check_m3_route_exposure_matrix.py --repo-root . --check",
]


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
    parser.add_argument("--matrix", default=DEFAULT_MATRIX_REL)
    parser.add_argument("--schema", default=DEFAULT_SCHEMA_REL)
    parser.add_argument("--release-doc", default=DEFAULT_RELEASE_DOC_REL)
    parser.add_argument("--support-audit", default=DEFAULT_SUPPORT_AUDIT_REL)
    parser.add_argument("--ux-doc", default=DEFAULT_UX_DOC_REL)
    parser.add_argument("--claimed-surface-register", default=DEFAULT_CLAIMED_SURFACE_REGISTER_REL)
    parser.add_argument("--provider-export", default=DEFAULT_PROVIDER_EXPORT_REL)
    parser.add_argument("--capture", default=DEFAULT_CAPTURE_REL)
    parser.add_argument(
        "--check",
        action="store_true",
        help="Fail when the checked-in validation capture would change.",
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


def looks_like_path(ref: str) -> bool:
    clean = ref.split("#", 1)[0].strip()
    if not clean or clean.startswith(ID_PREFIXES):
        return False
    return "/" in clean or clean.endswith(PATH_LIKE_SUFFIXES)


def ref_exists(repo_root: Path, ref: str) -> bool:
    clean = ref.split("#", 1)[0].strip()
    return bool(clean) and (repo_root / clean).exists()


def add_path_ref_findings(
    repo_root: Path, refs: list[str], findings: list[Finding], subject: str
) -> None:
    for ref in refs:
        if looks_like_path(ref) and not ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="path_ref.missing",
                    message=f"referenced path does not exist: {ref}",
                    remediation="Seed the referenced artifact or correct the matrix ref.",
                    ref=subject,
                    details={"path_ref": ref},
                )
            )


def schema_enum(schema: dict[str, Any], def_name: str) -> set[str]:
    return set(
        ensure_list(
            ensure_dict(schema.get("$defs"), "schema.$defs")
            .get(def_name, {})
            .get("enum", []),
            f"schema.$defs.{def_name}.enum",
        )
    )


def validate_vocabularies(
    matrix: dict[str, Any], schema: dict[str, Any], findings: list[Finding]
) -> dict[str, set[str]]:
    vocab = ensure_dict(matrix.get("vocabularies"), "matrix.vocabularies")
    vocab_sets: dict[str, set[str]] = {}
    schema_names = [
        "origin_class",
        "action_origin_class",
        "target_class",
        "action_route_class",
        "action_exposure_class",
        "route_change_reason_code",
        "approval_reuse_class",
        "reapproval_trigger_class",
        "privacy_consequence_class",
        "browser_handoff_class",
        "consumer_surface_class",
    ]
    for name in schema_names:
        tokens = set(ensure_list(vocab.get(name), f"matrix.vocabularies.{name}"))
        vocab_sets[name] = tokens
        allowed = schema_enum(schema, name)
        unknown = sorted(tokens - allowed)
        if unknown:
            findings.append(
                Finding(
                    severity="error",
                    check_id="vocabulary.unknown_token",
                    message=f"{name} contains tokens outside the schema enum.",
                    remediation="Update the schema and validator with the new token, or use an existing token.",
                    ref=f"vocabularies.{name}",
                    details={"unknown_tokens": unknown},
                )
            )
    missing_origins = sorted(REQUIRED_ORIGIN_CLASSES - vocab_sets["origin_class"])
    if missing_origins:
        findings.append(
            Finding(
                severity="error",
                check_id="vocabulary.required_origin_missing",
                message="The matrix vocabulary is missing required origin classes.",
                remediation="Add the missing origin classes to the matrix vocabulary.",
                ref="vocabularies.origin_class",
                details={"missing": missing_origins},
            )
        )
    return vocab_sets


def collect_claimed_handoff_routes(register: dict[str, Any]) -> dict[str, str]:
    routes: dict[str, str] = {}
    for raw_row in ensure_list(
        register.get("preview_surface_qualifications"),
        "claimed_surface_register.preview_surface_qualifications",
    ):
        row = ensure_dict(raw_row, "preview_surface_qualifications[]")
        handoff = row.get("handoff")
        if not isinstance(handoff, dict):
            continue
        route_ref = ensure_str(handoff.get("route_ref"), "handoff.route_ref")
        surface_id = ensure_str(row.get("surface_id"), "surface_id")
        routes[route_ref] = surface_id
    return routes


def collect_provider_rows(provider_export: dict[str, Any]) -> dict[str, dict[str, Any]]:
    page = ensure_dict(provider_export.get("page"), "provider_export.page")
    rows: dict[str, dict[str, Any]] = {}
    for raw_row in ensure_list(page.get("rows"), "provider_export.page.rows"):
        row = ensure_dict(raw_row, "provider_export.page.rows[]")
        rows[ensure_str(row.get("row_id"), "provider row_id")] = row
    return rows


def validate_row(
    repo_root: Path,
    row: dict[str, Any],
    vocab: dict[str, set[str]],
    findings: list[Finding],
) -> None:
    row_id = ensure_str(row.get("row_id"), "rows[].row_id")
    origin = ensure_dict(row.get("origin"), f"{row_id}.origin")
    target = ensure_dict(row.get("target"), f"{row_id}.target")
    route = ensure_dict(row.get("route"), f"{row_id}.route")
    exposure = ensure_dict(row.get("exposure"), f"{row_id}.exposure")
    approval = ensure_dict(row.get("approval"), f"{row_id}.approval")
    handoff = ensure_dict(row.get("handoff"), f"{row_id}.handoff")
    support_export = ensure_dict(row.get("support_export"), f"{row_id}.support_export")
    promotion_guard = ensure_dict(row.get("promotion_guard"), f"{row_id}.promotion_guard")

    field_tokens = {
        "origin_class": origin.get("origin_class"),
        "action_origin_class": origin.get("action_origin_class"),
        "target_class": target.get("target_class"),
        "action_route_class": route.get("action_route_class"),
        "action_exposure_class": exposure.get("action_exposure_class"),
        "route_change_reason_code": route.get("route_change_reason_code"),
        "approval_reuse_class": approval.get("approval_reuse_class"),
        "privacy_consequence_class": exposure.get("privacy_consequence_class"),
        "browser_handoff_class": handoff.get("browser_handoff_class"),
    }
    for field_name, token in field_tokens.items():
        if token not in vocab[field_name]:
            findings.append(
                Finding(
                    severity="error",
                    check_id="row.token_not_in_vocabulary",
                    message=f"{row_id} has {field_name}={token!r} outside the matrix vocabulary.",
                    remediation="Use a closed matrix vocabulary token or update the schema, matrix, and validator together.",
                    ref=row_id,
                    details={"field": field_name, "token": token},
                )
            )

    trigger_tokens = set(
        ensure_list(approval.get("reapproval_trigger_classes"), f"{row_id}.approval.reapproval_trigger_classes")
    )
    unknown_triggers = sorted(trigger_tokens - vocab["reapproval_trigger_class"])
    if unknown_triggers:
        findings.append(
            Finding(
                severity="error",
                check_id="row.reapproval_trigger_unknown",
                message=f"{row_id} has unknown reapproval trigger tokens.",
                remediation="Use closed trigger tokens from the matrix vocabulary.",
                ref=row_id,
                details={"unknown_triggers": unknown_triggers},
            )
        )
    if approval.get("approval_reuse_class") != "not_required_read_only" and not trigger_tokens:
        findings.append(
            Finding(
                severity="error",
                check_id="row.reapproval_triggers_missing",
                message=f"{row_id} declares approval reuse without reapproval triggers.",
                remediation="Declare the target, trust, policy, host, privacy, scope, freshness, or exposure changes that force reapproval.",
                ref=row_id,
            )
        )

    consumer_surfaces = set(
        ensure_list(support_export.get("consumer_surfaces"), f"{row_id}.support_export.consumer_surfaces")
    )
    missing_surfaces = sorted(REQUIRED_CONSUMER_SURFACES - consumer_surfaces)
    if missing_surfaces:
        findings.append(
            Finding(
                severity="error",
                check_id="row.consumer_surface_missing",
                message=f"{row_id} is missing required support/export consumer surfaces.",
                remediation="Add Help/About, service-health, diagnostics, docs/help, and support-export parity to the row.",
                ref=row_id,
                details={"missing": missing_surfaces},
            )
        )
    unknown_surfaces = sorted(consumer_surfaces - vocab["consumer_surface_class"])
    if unknown_surfaces:
        findings.append(
            Finding(
                severity="error",
                check_id="row.consumer_surface_unknown",
                message=f"{row_id} has unknown consumer-surface tokens.",
                remediation="Use closed consumer-surface tokens from the matrix vocabulary.",
                ref=row_id,
                details={"unknown": unknown_surfaces},
            )
        )

    if not support_export.get("parity_required"):
        findings.append(
            Finding(
                severity="error",
                check_id="row.support_parity_not_required",
                message=f"{row_id} does not require support-export parity.",
                remediation="Set support_export.parity_required=true for beta route/exposure rows.",
                ref=row_id,
            )
        )
    for field_name in [
        "raw_url_export_allowed",
        "raw_token_export_allowed",
        "raw_provider_payload_export_allowed",
    ]:
        if support_export.get(field_name) is not False:
            findings.append(
                Finding(
                    severity="error",
                    check_id="row.raw_export_not_blocked",
                    message=f"{row_id} allows raw export through {field_name}.",
                    remediation="Route/exposure support rows are metadata-only; set raw export booleans to false.",
                    ref=row_id,
                    details={"field": field_name},
                )
            )

    high_risk = promotion_guard.get("high_risk") is True
    if high_risk:
        if exposure.get("action_exposure_class") == "exposure_unknown_requires_review":
            findings.append(
                Finding(
                    severity="error",
                    check_id="row.high_risk_unknown_exposure",
                    message=f"{row_id} is high risk but has unknown exposure.",
                    remediation="Classify the exposure before beta promotion.",
                    ref=row_id,
                )
            )
        if promotion_guard.get("uncategorized_high_risk_gap") is not False:
            findings.append(
                Finding(
                    severity="error",
                    check_id="row.high_risk_uncategorized",
                    message=f"{row_id} is high risk and uncategorized.",
                    remediation="Declare origin, target, route, exposure, approval, handoff, and support parity before promotion.",
                    ref=row_id,
                )
            )
        if promotion_guard.get("beta_promotion_blocking_when_unknown") is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id="row.high_risk_not_promotion_blocking",
                    message=f"{row_id} high-risk unknown state is not promotion-blocking.",
                    remediation="Set beta_promotion_blocking_when_unknown=true.",
                    ref=row_id,
                )
            )

    handoff_class = handoff.get("browser_handoff_class")
    needs_packet = handoff_class in {
        "system_browser_required",
        "system_browser_fallback_available",
        "embedded_webview_boundary",
        "browser_companion_to_desktop",
        "provider_callback_return",
    }
    if needs_packet and not handoff.get("browser_handoff_packet_ref"):
        findings.append(
            Finding(
                severity="error",
                check_id="row.handoff_packet_missing",
                message=f"{row_id} crosses a browser/system boundary without a handoff packet ref.",
                remediation="Add an opaque browser_handoff_packet_ref or narrow the handoff class.",
                ref=row_id,
            )
        )

    path_refs: list[str] = []
    path_refs.extend(ensure_list(row.get("source_artifact_refs"), f"{row_id}.source_artifact_refs"))
    path_refs.extend(ensure_list(row.get("claimed_beta_surface_refs"), f"{row_id}.claimed_beta_surface_refs"))
    path_refs.append(ensure_str(support_export.get("projection_ref"), f"{row_id}.support_export.projection_ref"))
    add_path_ref_findings(repo_root, path_refs, findings, row_id)


def validate_matrix(
    repo_root: Path,
    matrix: dict[str, Any],
    schema: dict[str, Any],
    claimed_surface_register: dict[str, Any],
    provider_export: dict[str, Any],
    docs: dict[str, str],
) -> list[Finding]:
    findings: list[Finding] = []
    if matrix.get("schema_version") != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.schema_version",
                message="Matrix schema_version must be 1.",
                remediation="Update the validator and schema in the same change that changes schema_version.",
            )
        )
    if matrix.get("record_kind") != "route_exposure_matrix":
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.record_kind",
                message="Matrix record_kind must be route_exposure_matrix.",
                remediation="Restore the canonical record_kind.",
            )
        )

    refs = [ensure_str(matrix.get("schema_ref"), "matrix.schema_ref")]
    refs.extend(str(ref) for ref in ensure_dict(matrix.get("source_refs"), "matrix.source_refs").values())
    contract = ensure_dict(matrix.get("validation_contract"), "matrix.validation_contract")
    refs.extend(
        ensure_str(contract.get(field), f"validation_contract.{field}")
        for field in [
            "claimed_handoff_register_ref",
            "provider_route_support_export_ref",
            "release_matrix_ref",
            "support_audit_ref",
            "ux_doc_ref",
        ]
    )
    add_path_ref_findings(repo_root, refs, findings, "matrix")

    vocab = validate_vocabularies(matrix, schema, findings)
    rows = [ensure_dict(row, "matrix.rows[]") for row in ensure_list(matrix.get("rows"), "matrix.rows")]
    seen_row_ids: set[str] = set()
    route_refs: set[str] = set()
    provider_row_refs: set[str] = set()
    origin_classes: set[str] = set()
    trigger_coverage: set[str] = set()

    for row in rows:
        row_id = ensure_str(row.get("row_id"), "rows[].row_id")
        if row_id in seen_row_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="row.duplicate_id",
                    message=f"Duplicate row id {row_id}.",
                    remediation="Keep route/exposure row ids unique.",
                    ref=row_id,
                )
            )
        seen_row_ids.add(row_id)
        route_refs.update(ensure_list(row.get("route_refs"), f"{row_id}.route_refs"))
        provider_row_refs.update(row.get("provider_route_resolution_row_refs", []))
        origin_classes.add(ensure_dict(row.get("origin"), f"{row_id}.origin").get("origin_class"))
        trigger_coverage.update(
            ensure_dict(row.get("approval"), f"{row_id}.approval").get(
                "reapproval_trigger_classes", []
            )
        )
        validate_row(repo_root, row, vocab, findings)

    missing_origins = sorted(REQUIRED_ORIGIN_CLASSES - origin_classes)
    if missing_origins:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.required_origin_uncovered",
                message="The rows do not cover every required origin class.",
                remediation="Add rows for the missing origin classes.",
                details={"missing": missing_origins},
            )
        )

    missing_trigger_coverage = sorted(REQUIRED_REAPPROVAL_TRIGGER_COVERAGE - trigger_coverage)
    if missing_trigger_coverage:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.reapproval_trigger_coverage_missing",
                message="The rows do not cover the required reapproval trigger families.",
                remediation="Add row trigger coverage for target, trust, policy, host, and privacy drift.",
                details={"missing": missing_trigger_coverage},
            )
        )

    claimed_routes = collect_claimed_handoff_routes(claimed_surface_register)
    missing_claimed_routes = sorted(set(claimed_routes) - route_refs)
    if missing_claimed_routes:
        findings.append(
            Finding(
                severity="error",
                check_id="claimed_handoff_route.missing_matrix_row",
                message="Claimed handoff routes are missing from the route/exposure matrix.",
                remediation="Add a route/exposure row for each claimed handoff route.",
                details={"missing": missing_claimed_routes},
            )
        )

    provider_rows = collect_provider_rows(provider_export)
    missing_provider_rows = sorted(set(provider_rows) - provider_row_refs)
    if missing_provider_rows:
        findings.append(
            Finding(
                severity="error",
                check_id="provider_route_row.missing_matrix_row",
                message="Provider route-resolution rows are missing from the route/exposure matrix.",
                remediation="Add provider_route_resolution_row_refs for every provider beta row.",
                details={"missing": missing_provider_rows},
            )
        )

    rows_by_provider = {
        provider_ref: row
        for row in rows
        for provider_ref in row.get("provider_route_resolution_row_refs", [])
    }
    for provider_row_id, provider_row in provider_rows.items():
        fallback = ensure_dict(provider_row.get("fallback"), f"{provider_row_id}.fallback")
        browser_packet = fallback.get("browser_handoff_packet_ref")
        if fallback.get("fallback_mode") == "open_in_provider":
            matrix_row = rows_by_provider.get(provider_row_id)
            if matrix_row is None:
                continue
            handoff = ensure_dict(matrix_row.get("handoff"), f"{matrix_row['row_id']}.handoff")
            if handoff.get("browser_handoff_packet_ref") != browser_packet:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="provider_handoff_packet.mismatch",
                        message=f"{matrix_row['row_id']} does not preserve the provider browser-handoff packet ref.",
                        remediation="Quote the same browser_handoff_packet_ref used by the provider route-resolution export.",
                        ref=matrix_row["row_id"],
                        details={
                            "provider_row": provider_row_id,
                            "expected_packet_ref": browser_packet,
                            "actual_packet_ref": handoff.get("browser_handoff_packet_ref"),
                        },
                    )
                )

    for doc_name, phrases in [
        ("release", REQUIRED_RELEASE_DOC_PHRASES),
        ("support", REQUIRED_SUPPORT_DOC_PHRASES),
        ("ux", REQUIRED_UX_DOC_PHRASES),
    ]:
        text = docs[doc_name]
        for phrase in phrases:
            if phrase not in text:
                findings.append(
                    Finding(
                        severity="error",
                        check_id=f"doc.{doc_name}.phrase_missing",
                        message=f"{doc_name} document is missing required phrase {phrase!r}.",
                        remediation="Refresh the document so it exposes the current route/exposure contract.",
                    )
                )
        for row_id in seen_row_ids:
            if row_id not in text:
                findings.append(
                    Finding(
                        severity="error",
                        check_id=f"doc.{doc_name}.row_missing",
                        message=f"{doc_name} document does not mention {row_id}.",
                        remediation="Add the route/exposure row to the release, support, and UX packet.",
                        ref=row_id,
                    )
                )

    return findings


def build_capture(
    matrix_rel: str,
    schema_rel: str,
    release_doc_rel: str,
    support_audit_rel: str,
    ux_doc_rel: str,
    claimed_routes: dict[str, str],
    provider_rows: dict[str, dict[str, Any]],
    matrix: dict[str, Any],
    findings: list[Finding],
) -> dict[str, Any]:
    rows = ensure_list(matrix.get("rows"), "matrix.rows")
    high_risk_ids = [
        row["row_id"]
        for row in rows
        if ensure_dict(row.get("promotion_guard"), f"{row.get('row_id')}.promotion_guard").get("high_risk")
    ]
    rows_by_origin: dict[str, int] = {}
    rows_by_exposure: dict[str, int] = {}
    rows_by_handoff: dict[str, int] = {}
    for row in rows:
        origin = row["origin"]["origin_class"]
        exposure = row["exposure"]["action_exposure_class"]
        handoff = row["handoff"]["browser_handoff_class"]
        rows_by_origin[origin] = rows_by_origin.get(origin, 0) + 1
        rows_by_exposure[exposure] = rows_by_exposure.get(exposure, 0) + 1
        rows_by_handoff[handoff] = rows_by_handoff.get(handoff, 0) + 1
    error_count = sum(1 for finding in findings if finding.severity == "error")
    return {
        "record_kind": "route_exposure_matrix_validation_capture",
        "schema_version": 1,
        "generated_at": f"{matrix['as_of']}T00:00:00Z",
        "matrix_ref": matrix_rel,
        "schema_ref": schema_rel,
        "release_doc_ref": release_doc_rel,
        "support_audit_ref": support_audit_rel,
        "ux_doc_ref": ux_doc_rel,
        "status": "pass" if error_count == 0 else "fail",
        "row_count": len(rows),
        "high_risk_row_count": len(high_risk_ids),
        "high_risk_row_ids": high_risk_ids,
        "claimed_handoff_route_refs": sorted(claimed_routes),
        "provider_route_resolution_row_refs": sorted(provider_rows),
        "rows_by_origin_class": dict(sorted(rows_by_origin.items())),
        "rows_by_exposure_class": dict(sorted(rows_by_exposure.items())),
        "rows_by_handoff_class": dict(sorted(rows_by_handoff.items())),
        "required_consumer_surfaces": sorted(REQUIRED_CONSUMER_SURFACES),
        "finding_count": len(findings),
        "error_count": error_count,
        "findings": [finding.as_report() for finding in findings],
    }


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    matrix = ensure_dict(load_json(repo_root / args.matrix), "matrix")
    schema = ensure_dict(load_json(repo_root / args.schema), "schema")
    claimed_register = ensure_dict(
        load_json(repo_root / args.claimed_surface_register), "claimed_surface_register"
    )
    provider_export = ensure_dict(load_json(repo_root / args.provider_export), "provider_export")
    docs = {
        "release": (repo_root / args.release_doc).read_text(encoding="utf-8"),
        "support": (repo_root / args.support_audit).read_text(encoding="utf-8"),
        "ux": (repo_root / args.ux_doc).read_text(encoding="utf-8"),
    }

    findings = validate_matrix(
        repo_root, matrix, schema, claimed_register, provider_export, docs
    )
    capture = build_capture(
        args.matrix,
        args.schema,
        args.release_doc,
        args.support_audit,
        args.ux_doc,
        collect_claimed_handoff_routes(claimed_register),
        collect_provider_rows(provider_export),
        matrix,
        findings,
    )
    rendered = json.dumps(capture, indent=2, sort_keys=True) + "\n"
    capture_path = repo_root / args.capture
    if args.check:
        if not capture_path.exists():
            print(f"missing validation capture: {capture_path}", file=sys.stderr)
            return 1
        current = capture_path.read_text(encoding="utf-8")
        if current != rendered:
            print(
                f"validation capture is stale; regenerate with: python3 ci/check_m3_route_exposure_matrix.py --repo-root {args.repo_root}",
                file=sys.stderr,
            )
            return 1
    else:
        capture_path.parent.mkdir(parents=True, exist_ok=True)
        capture_path.write_text(rendered, encoding="utf-8")

    if capture["error_count"]:
        print(rendered)
        return 1
    print(
        f"route exposure matrix validated: {capture['row_count']} rows, {capture['high_risk_row_count']} high-risk rows"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
