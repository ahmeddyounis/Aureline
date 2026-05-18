#!/usr/bin/env python3
"""Generate and validate qualified preview-surface scope rows."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import re
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_REGISTER_REL = "artifacts/milestones/m3/claimed_surface_register.json"
DEFAULT_OUTPUT_REL = "artifacts/compat/m3/qualified_preview_rows.json"
DEFAULT_CAPTURE_REL = (
    "artifacts/compat/m3/captures/qualified_preview_rows_validation_capture.json"
)
DEFAULT_FIXTURE_DIR_REL = "fixtures/compat/m3/preview_scope_and_handoff"

REQUIRED_SURFACE_FAMILIES = {
    "notebook",
    "voice",
    "browser_companion",
    "preview_canvas",
}
REQUIRED_CONSUMER_PROJECTIONS = {
    "start_center",
    "docs_help",
    "help_about",
    "service_health",
    "compatibility_report",
    "marketplace_help_metadata",
    "support_export",
}
SUPPORT_STRICTNESS = {
    "limited": 1,
    "experimental": 2,
    "retest_pending": 3,
    "unsupported": 4,
}
PATH_LIKE_SUFFIXES = (".json", ".yaml", ".yml", ".md", ".schema.json")
ID_PREFIXES = (
    "handoff:",
    "preview_surface:",
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
    parser.add_argument("--register", default=DEFAULT_REGISTER_REL)
    parser.add_argument("--output", default=DEFAULT_OUTPUT_REL)
    parser.add_argument("--capture", default=DEFAULT_CAPTURE_REL)
    parser.add_argument("--fixture-dir", default=DEFAULT_FIXTURE_DIR_REL)
    parser.add_argument(
        "--check",
        action="store_true",
        help="Fail if generated packet or capture would drift.",
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


def ensure_bool(value: Any, label: str) -> bool:
    if not isinstance(value, bool):
        raise SystemExit(f"{label} must be a boolean")
    return value


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


def validate_path_refs(repo_root: Path, refs: list[str], findings: list[Finding]) -> None:
    seen: set[str] = set()
    for ref in refs:
        if not looks_like_path(ref):
            continue
        clean = ref.split("#", 1)[0].strip()
        if clean in seen:
            continue
        seen.add(clean)
        if not (repo_root / clean).exists():
            findings.append(
                Finding(
                    severity="error",
                    check_id="path_ref.missing",
                    message=f"referenced path does not exist: {clean}",
                    remediation="Seed the referenced artifact or correct the qualification row ref.",
                    ref=ref,
                )
            )


def narrower_support(left: str, right: str) -> str:
    if SUPPORT_STRICTNESS[right] > SUPPORT_STRICTNESS[left]:
        return right
    return left


def derive_freshness(
    evidence: dict[str, Any],
    register_as_of: str,
    row_id: str,
) -> tuple[str, int | None]:
    declared = ensure_str(
        evidence.get("freshness_state", "missing"),
        f"{row_id}.evidence.freshness_state",
    )
    evidence_as_of = evidence.get("as_of")
    if evidence_as_of is None:
        return "missing", None
    evidence_date = parse_date(ensure_str(evidence_as_of, f"{row_id}.evidence.as_of"), f"{row_id}.evidence.as_of")
    register_date = parse_date(register_as_of, "register.as_of")
    review_window_days = int(evidence.get("review_window_days", 0))
    age_days = (register_date - evidence_date).days
    if age_days > review_window_days:
        return "stale", age_days
    if declared not in {"current", "partial"}:
        return declared, age_days
    return declared, age_days


def support_label(token: str) -> str:
    return {
        "limited": "Limited",
        "experimental": "Experimental",
        "retest_pending": "Retest pending",
        "unsupported": "Unsupported",
    }[token]


def lifecycle_label(token: str) -> str:
    return {
        "preview": "Preview",
        "beta": "Beta",
    }[token]


def compose_row(
    raw: dict[str, Any],
    register_as_of: str,
    repo_root: Path,
    findings: list[Finding],
) -> dict[str, Any]:
    row_id = ensure_str(raw.get("surface_id"), "preview_surface.surface_id")
    declared_lifecycle = ensure_str(
        raw.get("declared_lifecycle_label"),
        f"{row_id}.declared_lifecycle_label",
    )
    declared_support = ensure_str(
        raw.get("declared_support_class"),
        f"{row_id}.declared_support_class",
    )
    if declared_support not in SUPPORT_STRICTNESS:
        findings.append(
            Finding(
                severity="error",
                check_id="row.support_class.unknown",
                message=f"{row_id} declares unsupported support class {declared_support!r}",
                remediation="Use limited, experimental, retest_pending, or unsupported.",
                ref=row_id,
            )
        )
        declared_support = "unsupported"

    evidence = ensure_dict(raw.get("evidence"), f"{row_id}.evidence")
    freshness_state, evidence_age_days = derive_freshness(evidence, register_as_of, row_id)
    downgrade = ensure_dict(raw.get("downgrade"), f"{row_id}.downgrade")
    effective_support = declared_support
    downgrade_reason_tokens: list[str] = []
    downgrade_reasons: list[str] = []

    if freshness_state == "missing":
        effective_support = narrower_support(
            effective_support,
            ensure_str(
                downgrade.get("missing_evidence_downgrade_to"),
                f"{row_id}.downgrade.missing_evidence_downgrade_to",
            ),
        )
        downgrade_reason_tokens.append("missing_evidence")
    elif freshness_state == "stale":
        effective_support = narrower_support(
            effective_support,
            ensure_str(
                downgrade.get("stale_evidence_downgrade_to"),
                f"{row_id}.downgrade.stale_evidence_downgrade_to",
            ),
        )
        downgrade_reason_tokens.append("stale_evidence")

    for gate in ensure_list(raw.get("qualification_gates"), f"{row_id}.qualification_gates"):
        gate = ensure_dict(gate, f"{row_id}.qualification_gates[]")
        state = ensure_str(gate.get("state"), f"{row_id}.qualification_gates[].state")
        if ensure_bool(
            gate.get("required_for_beta"),
            f"{row_id}.qualification_gates[].required_for_beta",
        ) and state != "current":
            effective_support = narrower_support(
                effective_support,
                ensure_str(gate.get("downgrade_to"), f"{row_id}.qualification_gates[].downgrade_to"),
            )
            downgrade_reason_tokens.append(
                ensure_str(
                    gate.get("downgrade_reason_token"),
                    f"{row_id}.qualification_gates[].downgrade_reason_token",
                )
            )

    reason_token = ensure_str(downgrade.get("reason_token"), f"{row_id}.downgrade.reason_token")
    if reason_token not in downgrade_reason_tokens:
        downgrade_reason_tokens.append(reason_token)
    downgrade_reasons.append(ensure_str(downgrade.get("reason"), f"{row_id}.downgrade.reason"))

    effective_lifecycle = declared_lifecycle
    if freshness_state in {"missing", "stale"}:
        effective_lifecycle = "preview"
    if effective_support in {"experimental", "retest_pending", "unsupported"}:
        effective_lifecycle = "preview"

    handoff = ensure_dict(raw.get("handoff"), f"{row_id}.handoff")
    handoff_required = ensure_bool(handoff.get("required"), f"{row_id}.handoff.required")
    limitation_statement = ensure_str(
        handoff.get("limitation_statement"),
        f"{row_id}.handoff.limitation_statement",
    )
    surface_family = ensure_str(raw.get("surface_family"), f"{row_id}.surface_family")
    native_depth_claimed = ensure_bool(
        raw.get("native_depth_capability_claimed"),
        f"{row_id}.native_depth_capability_claimed",
    )
    if surface_family in {"browser_companion", "voice"} and native_depth_claimed:
        findings.append(
            Finding(
                severity="error",
                check_id="row.native_depth_overclaim",
                message=f"{row_id} claims native-depth capability on a scoped preview surface",
                remediation="Set native_depth_capability_claimed=false and provide a handoff target.",
                ref=row_id,
            )
        )
    if surface_family in {"browser_companion", "voice", "preview_canvas", "notebook"}:
        if not handoff_required or not limitation_statement:
            findings.append(
                Finding(
                    severity="error",
                    check_id="row.handoff_missing",
                    message=f"{row_id} lacks explicit handoff or limitation text",
                    remediation="Add a required handoff target and limitation statement.",
                    ref=row_id,
                )
            )

    projections = [
        ensure_dict(projection, f"{row_id}.consumer_projections[]")
        for projection in ensure_list(
            raw.get("consumer_projections"),
            f"{row_id}.consumer_projections",
        )
    ]
    projection_surfaces = {
        ensure_str(projection.get("surface"), f"{row_id}.consumer_projections[].surface")
        for projection in projections
    }
    missing_projections = REQUIRED_CONSUMER_PROJECTIONS - projection_surfaces
    if missing_projections:
        findings.append(
            Finding(
                severity="error",
                check_id="row.consumer_projection.missing",
                message=f"{row_id} is missing required consumer projections",
                remediation="Project the same qualification row to all required product, docs, compatibility, and support surfaces.",
                ref=row_id,
                details={"missing": sorted(missing_projections)},
            )
        )

    path_refs: list[str] = []
    path_refs.extend(
        ensure_str(ref, f"{row_id}.evidence.evidence_refs[]")
        for ref in ensure_list(evidence.get("evidence_refs"), f"{row_id}.evidence.evidence_refs")
    )
    path_refs.extend(
        ensure_str(projection.get("projection_ref"), f"{row_id}.consumer_projections[].projection_ref")
        for projection in projections
    )
    validate_path_refs(repo_root, path_refs, findings)

    client_scope = ensure_str(raw.get("client_scope"), f"{row_id}.client_scope")
    support_export_row = {
        "support_row_id": f"support-export:preview-scope:{row_id.split(':', 1)[1]}",
        "surface_ref": row_id,
        "surface_family": surface_family,
        "lifecycle_label": effective_lifecycle,
        "support_class": effective_support,
        "client_scope": client_scope,
        "freshness_state": freshness_state,
        "handoff_required": handoff_required,
        "handoff_target": ensure_str(handoff.get("target"), f"{row_id}.handoff.target"),
        "downgrade_reason_tokens": downgrade_reason_tokens,
        "raw_private_material_excluded": True,
        "ambient_authority_excluded": True,
    }

    return {
        "surface_id": row_id,
        "title": ensure_str(raw.get("title"), f"{row_id}.title"),
        "surface_family": surface_family,
        "public_label": ensure_str(raw.get("public_label"), f"{row_id}.public_label"),
        "declared_lifecycle_label": declared_lifecycle,
        "declared_lifecycle_label_display": lifecycle_label(declared_lifecycle),
        "effective_lifecycle_label": effective_lifecycle,
        "effective_lifecycle_label_display": lifecycle_label(effective_lifecycle),
        "declared_support_class": declared_support,
        "declared_support_label": support_label(declared_support),
        "effective_support_class": effective_support,
        "effective_support_label": support_label(effective_support),
        "client_scope": client_scope,
        "client_scope_label": ensure_str(raw.get("client_scope_label"), f"{row_id}.client_scope_label"),
        "native_depth_capability_claimed": native_depth_claimed,
        "evidence": {
            "as_of": evidence.get("as_of"),
            "review_window_days": int(evidence.get("review_window_days", 0)),
            "freshness_state": freshness_state,
            "evidence_age_days": evidence_age_days,
            "evidence_refs": path_refs[: len(evidence.get("evidence_refs", []))],
        },
        "qualification_gates": raw["qualification_gates"],
        "handoff": {
            "required": handoff_required,
            "target": ensure_str(handoff.get("target"), f"{row_id}.handoff.target"),
            "target_label": ensure_str(handoff.get("target_label"), f"{row_id}.handoff.target_label"),
            "route_ref": ensure_str(handoff.get("route_ref"), f"{row_id}.handoff.route_ref"),
            "limitation_statement": limitation_statement,
            "preserves_context": ensure_bool(
                handoff.get("preserves_context"),
                f"{row_id}.handoff.preserves_context",
            ),
        },
        "downgrade": {
            "downgrade_reason_tokens": downgrade_reason_tokens,
            "downgrade_reasons": downgrade_reasons,
            "stale_evidence_downgrade_to": ensure_str(
                downgrade.get("stale_evidence_downgrade_to"),
                f"{row_id}.downgrade.stale_evidence_downgrade_to",
            ),
            "missing_evidence_downgrade_to": ensure_str(
                downgrade.get("missing_evidence_downgrade_to"),
                f"{row_id}.downgrade.missing_evidence_downgrade_to",
            ),
        },
        "consumer_projections": projections,
        "support_export": support_export_row,
        "display_summary": (
            f"{ensure_str(raw.get('public_label'), f'{row_id}.public_label')} "
            f"- {lifecycle_label(effective_lifecycle)} / {support_label(effective_support)} "
            f"/ {ensure_str(raw.get('client_scope_label'), f'{row_id}.client_scope_label')}; "
            f"handoff: {ensure_str(handoff.get('target_label'), f'{row_id}.handoff.target_label')}"
        ),
    }


def compose_packet(
    register: dict[str, Any],
    repo_root: Path,
    output_rel: str,
    generated_at: str,
    findings: list[Finding],
) -> dict[str, Any]:
    register_as_of = ensure_str(register.get("as_of"), "register.as_of")
    rows = [
        compose_row(ensure_dict(row, "preview_surface_qualifications[]"), register_as_of, repo_root, findings)
        for row in ensure_list(
            register.get("preview_surface_qualifications"),
            "register.preview_surface_qualifications",
        )
    ]

    families = {row["surface_family"] for row in rows}
    missing_families = REQUIRED_SURFACE_FAMILIES - families
    if missing_families:
        findings.append(
            Finding(
                severity="error",
                check_id="register.surface_family.missing",
                message="preview-surface qualifications do not cover every required family",
                remediation="Add rows for notebook, voice, browser_companion, and preview_canvas.",
                details={"missing": sorted(missing_families)},
            )
        )

    support_export_rows = [row["support_export"] for row in rows]
    return {
        "record_kind": "m3_qualified_preview_row_register",
        "schema_version": 1,
        "packet_id": "qualified_preview_rows:m3.beta",
        "packet_ref": output_rel,
        "source_claimed_surface_register_ref": DEFAULT_REGISTER_REL,
        "docs_ref": "docs/release/m3/preview_surface_scope_labels.md",
        "fixture_dir_ref": DEFAULT_FIXTURE_DIR_REL,
        "as_of": register_as_of,
        "generated_at": generated_at,
        "summary": {
            "row_count": len(rows),
            "required_surface_families": sorted(REQUIRED_SURFACE_FAMILIES),
            "covered_surface_families": sorted(families),
            "handoff_required_row_count": sum(1 for row in rows if row["handoff"]["required"]),
            "preview_lifecycle_row_count": sum(
                1 for row in rows if row["effective_lifecycle_label"] == "preview"
            ),
            "beta_lifecycle_row_count": sum(
                1 for row in rows if row["effective_lifecycle_label"] == "beta"
            ),
            "support_export_row_count": len(support_export_rows),
        },
        "rows": rows,
        "support_export_rows": support_export_rows,
        "raw_private_material_excluded": True,
        "ambient_authority_excluded": True,
    }


def fixture_files(fixture_dir: Path) -> list[Path]:
    if not fixture_dir.exists():
        return []
    return sorted(path for path in fixture_dir.glob("*.json") if path.name != "manifest.json")


def validate_fixtures(
    fixture_dir: Path,
    rows_by_id: dict[str, dict[str, Any]],
    register_as_of: str,
    repo_root: Path,
    findings: list[Finding],
) -> None:
    manifest_path = fixture_dir / "manifest.json"
    if manifest_path.exists():
        manifest = ensure_dict(load_json(manifest_path), "fixture manifest")
        listed = {
            ensure_str(item, "fixture_manifest.fixtures[]")
            for item in ensure_list(manifest.get("fixtures"), "fixture_manifest.fixtures")
        }
        actual = {path.name for path in fixture_files(fixture_dir)}
        if listed != actual:
            findings.append(
                Finding(
                    severity="error",
                    check_id="fixtures.manifest_drift",
                    message="preview scope fixture manifest does not match fixture files",
                    remediation="Refresh fixtures/compat/m3/preview_scope_and_handoff/manifest.json.",
                    details={
                        "missing_from_manifest": sorted(actual - listed),
                        "missing_from_dir": sorted(listed - actual),
                    },
                )
            )

    for path in fixture_files(fixture_dir):
        fixture = ensure_dict(load_json(path), f"fixture {path.name}")
        if "synthetic_row" in fixture:
            row = compose_row(
                ensure_dict(fixture.get("synthetic_row"), f"{path.name}.synthetic_row"),
                register_as_of,
                repo_root,
                findings,
            )
            row_id = row["surface_id"]
        else:
            row_id = ensure_str(fixture.get("surface_id"), f"{path.name}.surface_id")
            row = rows_by_id.get(row_id)
            if row is None:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="fixture.surface_unknown",
                        message=f"{path.name} references unknown preview surface {row_id}",
                        remediation="Use a surface_id from the generated preview-scope packet.",
                        ref=str(path),
                    )
                )
                continue
        expected = ensure_dict(fixture.get("expected"), f"{path.name}.expected")
        comparisons = {
            "effective_lifecycle_label": row["effective_lifecycle_label"],
            "effective_support_class": row["effective_support_class"],
            "client_scope": row["client_scope"],
            "handoff_required": row["handoff"]["required"],
            "handoff_target": row["handoff"]["target"],
            "native_depth_capability_claimed": row["native_depth_capability_claimed"],
        }
        for field_name, actual_value in comparisons.items():
            if field_name not in expected:
                continue
            if expected[field_name] != actual_value:
                findings.append(
                    Finding(
                        severity="error",
                        check_id=f"fixture.{field_name}.drift",
                        message=(
                            f"{path.name} expected {field_name}={expected[field_name]!r} "
                            f"but generated row has {actual_value!r}"
                        ),
                        remediation="Update the source register row or the fixture expectation.",
                        ref=str(path),
                    )
                )
        required_tokens = set(
            ensure_str(token, f"{path.name}.expected.required_downgrade_reason_tokens[]")
            for token in ensure_list(
                expected.get("required_downgrade_reason_tokens", []),
                f"{path.name}.expected.required_downgrade_reason_tokens",
            )
        )
        actual_tokens = set(row["downgrade"]["downgrade_reason_tokens"])
        if not required_tokens.issubset(actual_tokens):
            findings.append(
                Finding(
                    severity="error",
                    check_id="fixture.downgrade_reason.missing",
                    message=f"{path.name} expected downgrade reason tokens that were not generated",
                    remediation="Keep fixture expectations aligned with generated downgrade reasons.",
                    ref=str(path),
                    details={"missing": sorted(required_tokens - actual_tokens)},
                )
            )


_GENERATED_AT_RE = re.compile(r'"generated_at":\s*"[^"]*"')


def normalize_generated_at(text: str) -> str:
    return _GENERATED_AT_RE.sub("__generated_at__", text)


def write_if_changed(path: Path, content: str, check_only: bool) -> bool:
    path.parent.mkdir(parents=True, exist_ok=True)
    existing = path.read_text(encoding="utf-8") if path.exists() else None
    changed = existing is None or normalize_generated_at(existing) != normalize_generated_at(content)
    if not check_only:
        path.write_text(content, encoding="utf-8")
    return changed


def build_capture(
    packet: dict[str, Any],
    findings: list[Finding],
    output_rel: str,
    fixture_dir_rel: str,
    generated_at: str,
) -> dict[str, Any]:
    return {
        "record_kind": "m3_qualified_preview_rows_validation_capture",
        "schema_version": 1,
        "status": "pass" if not any(f.severity == "error" for f in findings) else "fail",
        "generated_at": generated_at,
        "packet_ref": output_rel,
        "fixture_dir_ref": fixture_dir_rel,
        "summary": {
            "row_count": packet["summary"]["row_count"],
            "handoff_required_row_count": packet["summary"]["handoff_required_row_count"],
            "preview_lifecycle_row_count": packet["summary"]["preview_lifecycle_row_count"],
            "beta_lifecycle_row_count": packet["summary"]["beta_lifecycle_row_count"],
            "errors": sum(1 for finding in findings if finding.severity == "error"),
            "warnings": sum(1 for finding in findings if finding.severity == "warning"),
        },
        "findings": [finding.as_report() for finding in findings],
    }


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    register = ensure_dict(load_json(repo_root / args.register), "claimed_surface_register")
    generated_at = (
        dt.datetime.now(dt.UTC)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )
    findings: list[Finding] = []
    packet = compose_packet(register, repo_root, args.output, generated_at, findings)
    validate_fixtures(
        repo_root / args.fixture_dir,
        {row["surface_id"]: row for row in packet["rows"]},
        ensure_str(register.get("as_of"), "register.as_of"),
        repo_root,
        findings,
    )
    capture = build_capture(packet, findings, args.output, args.fixture_dir, generated_at)

    output_changed = write_if_changed(
        repo_root / args.output,
        json.dumps(packet, indent=2, sort_keys=True) + "\n",
        args.check,
    )
    capture_changed = write_if_changed(
        repo_root / args.capture,
        json.dumps(capture, indent=2, sort_keys=True) + "\n",
        args.check,
    )
    if args.check and (output_changed or capture_changed):
        findings.append(
            Finding(
                severity="error",
                check_id="packet.stale",
                message="checked-in qualified preview rows drifted from the source register",
                remediation="Run `python3 ci/check_m3_qualified_preview_rows.py --repo-root .`.",
                details={
                    "output_changed": output_changed,
                    "capture_changed": capture_changed,
                },
            )
        )

    errors = [finding for finding in findings if finding.severity == "error"]
    if errors:
        for item in errors:
            ref = f" ({item.ref})" if item.ref else ""
            print(f"ERROR [{item.check_id}]{ref}: {item.message}", file=sys.stderr)
            print(f"  remediation: {item.remediation}", file=sys.stderr)
        return 1

    print(
        f"validated {packet['summary']['row_count']} qualified preview rows "
        f"({packet['summary']['handoff_required_row_count']} with handoff)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
