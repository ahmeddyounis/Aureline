#!/usr/bin/env python3
"""Validate the M3 beta managed-boundary and offboarding manifest.

This is the first consumer for the three artifacts in the M3
beta-boundary review batch:

  - artifacts/milestones/m3/boundary_manifest_beta.yaml
  - docs/release/m3/managed_boundary_beta.md
  - schemas/governance/boundary_manifest_beta.schema.json

The validator parses the boundary manifest, cross-checks the rows
against the closed vocabularies declared in the manifest header,
the alpha boundary manifest (for capability continuity), the
claimed-surface register, and the release doc, then writes a
machine-readable validation capture so downstream surfaces can
consume one source of truth without re-parsing prose.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import re
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_MANIFEST_REL = "artifacts/milestones/m3/boundary_manifest_beta.yaml"
DEFAULT_RELEASE_DOC_REL = "docs/release/m3/managed_boundary_beta.md"
DEFAULT_SCHEMA_REL = "schemas/governance/boundary_manifest_beta.schema.json"
DEFAULT_CAPTURE_REL = (
    "artifacts/milestones/m3/captures/"
    "boundary_manifest_beta_validation_capture.json"
)

REQUIRED_BOUNDARY_CLASSES = {
    "local_only",
    "mirrored",
    "self_hosted",
    "managed",
    "paid_seat_bound",
}

NARROWS_REQUIRED_FOR = {"managed", "paid_seat_bound"}
BETA_EVIDENCE_CLASSES = {
    "beta_partner_packet",
    "beta_drill",
    "policy_decision",
}

REQUIRED_DOC_PHRASES = [
    "Managed-boundary and offboarding beta manifest",
    "schemas/governance/boundary_manifest_beta.schema.json",
    "artifacts/milestones/m3/boundary_manifest_beta.yaml",
    "ci/check_m3_boundary_manifest_beta.py",
    "Row vocabulary",
    "Org-switch behavior",
    "Grace windows",
    "Seat and quota state",
    "Offboarding and export semantics",
    "Failure drill",
]

DURATION_PATTERN = re.compile(r"^P([0-9]+D|T[0-9]+H)$")


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
    parser.add_argument("--manifest", default=DEFAULT_MANIFEST_REL)
    parser.add_argument("--release-doc", default=DEFAULT_RELEASE_DOC_REL)
    parser.add_argument("--schema", default=DEFAULT_SCHEMA_REL)
    parser.add_argument("--capture", default=DEFAULT_CAPTURE_REL)
    parser.add_argument(
        "--check",
        action="store_true",
        help=(
            "Fail when the on-disk validation capture would change after "
            "regeneration. Use this in CI to keep the capture committed."
        ),
    )
    return parser.parse_args()


def render_yaml_file_as_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing YAML file: {path}")
    ruby = subprocess.run(
        [
            "ruby",
            "-rjson",
            "-ryaml",
            "-e",
            (
                "require 'date';"
                " payload = YAML.safe_load(File.read(ARGV[0]),"
                " permitted_classes: [Date, Time], aliases: false);"
                " STDOUT.write(JSON.generate(payload))"
            ),
            str(path),
        ],
        capture_output=True,
        text=True,
    )
    if ruby.returncode != 0:
        stderr = ruby.stderr.strip() or "unknown Ruby/Psych failure"
        raise SystemExit(f"failed to parse YAML for {path}: {stderr}")
    try:
        return json.loads(ruby.stdout)
    except json.JSONDecodeError as exc:
        raise SystemExit(
            f"Ruby/Psych emitted invalid JSON for {path}: {exc}"
        ) from exc


def load_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing JSON file: {path}")
    return json.loads(path.read_text(encoding="utf-8"))


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


def ensure_int(value: Any, label: str) -> int:
    if not isinstance(value, int):
        raise SystemExit(f"{label} must be an integer")
    return value


def parse_iso_date(value: str, label: str) -> dt.date:
    try:
        return dt.date.fromisoformat(value)
    except ValueError as exc:
        raise SystemExit(
            f"{label} must be a YYYY-MM-DD date, got {value!r}"
        ) from exc


def ref_exists(repo_root: Path, ref: str) -> bool:
    target = ref.split("#", 1)[0].strip()
    return bool(target) and (repo_root / target).exists()


def validate_header(
    repo_root: Path,
    manifest: dict[str, Any],
    schema_rel: str,
    release_doc_rel: str,
    findings: list[Finding],
) -> None:
    schema_version = ensure_int(
        manifest.get("schema_version"), "manifest.schema_version"
    )
    if schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="manifest.schema_version.unsupported",
                message=(
                    f"schema_version must be 1, got {schema_version}"
                ),
                remediation=(
                    "Bump the validator in the same change set that "
                    "bumps schema_version."
                ),
            )
        )

    manifest_id = ensure_str(manifest.get("manifest_id"), "manifest.manifest_id")
    if not manifest_id.startswith("boundary_manifest."):
        findings.append(
            Finding(
                severity="error",
                check_id="manifest.manifest_id.pattern",
                message=(
                    f"manifest_id must start with 'boundary_manifest.', "
                    f"got {manifest_id!r}"
                ),
                remediation="Use a stable manifest id under the boundary_manifest namespace.",
            )
        )

    parse_iso_date(
        ensure_str(manifest.get("as_of"), "manifest.as_of"), "manifest.as_of"
    )
    ensure_str(manifest.get("owner_dri"), "manifest.owner_dri")
    schema_ref = ensure_str(manifest.get("schema_ref"), "manifest.schema_ref")
    if schema_ref != schema_rel:
        findings.append(
            Finding(
                severity="error",
                check_id="manifest.schema_ref.mismatch",
                message=(
                    f"schema_ref must point at {schema_rel}, got {schema_ref}"
                ),
                remediation=(
                    "Refresh the schema_ref to match the canonical schema path."
                ),
            )
        )
    if not ref_exists(repo_root, schema_ref):
        findings.append(
            Finding(
                severity="error",
                check_id="manifest.schema_ref.missing",
                message=f"schema_ref does not exist: {schema_ref}",
                remediation="Restore the schema or fix the path.",
                ref=schema_ref,
            )
        )

    status = ensure_str(manifest.get("status"), "manifest.status")
    if status not in {"seeded", "draft", "ratified"}:
        findings.append(
            Finding(
                severity="error",
                check_id="manifest.status.invalid",
                message=f"status must be seeded/draft/ratified, got {status!r}",
                remediation="Set the manifest status to a known lifecycle value.",
            )
        )

    if manifest.get("release_channel_scope") != "beta":
        findings.append(
            Finding(
                severity="error",
                check_id="manifest.release_channel_scope.invalid",
                message="release_channel_scope must be 'beta'",
                remediation="Set release_channel_scope: beta for this manifest.",
            )
        )
    if manifest.get("milestone_id") != "m3":
        findings.append(
            Finding(
                severity="error",
                check_id="manifest.milestone_id.invalid",
                message="milestone_id must be 'm3'",
                remediation="Set milestone_id: m3 for this manifest.",
            )
        )

    source_refs = ensure_dict(
        manifest.get("source_contract_refs"), "manifest.source_contract_refs"
    )
    for key in (
        "claimed_surface_register_ref",
        "cohort_guardrails_ref",
        "alpha_boundary_manifest_ref",
        "open_paid_boundary_rows_ref",
        "usage_offboarding_contract_ref",
        "boundary_strawman_ref",
    ):
        value = ensure_str(source_refs.get(key), f"manifest.source_contract_refs.{key}")
        if not ref_exists(repo_root, value):
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"manifest.source_contract_refs.{key}.missing",
                    message=f"{key} does not exist: {value}",
                    remediation="Fix the path or seed the referenced artifact.",
                    ref=value,
                )
            )
    release_doc_in_refs = source_refs.get("release_doc_ref")
    if release_doc_in_refs and release_doc_in_refs != release_doc_rel:
        findings.append(
            Finding(
                severity="error",
                check_id="manifest.source_contract_refs.release_doc_ref.mismatch",
                message=(
                    "release_doc_ref must point at the canonical release "
                    f"doc {release_doc_rel}, got {release_doc_in_refs}"
                ),
                remediation="Refresh the release_doc_ref to match.",
            )
        )


def validate_rows(
    repo_root: Path,
    manifest: dict[str, Any],
    findings: list[Finding],
) -> list[dict[str, Any]]:
    rows = ensure_list(manifest.get("rows"), "manifest.rows")
    seen_row_ids: set[str] = set()
    seen_boundary_classes: set[str] = set()
    derived: list[dict[str, Any]] = []

    boundary_vocab = set(
        ensure_list(
            manifest.get("boundary_class_vocabulary"),
            "manifest.boundary_class_vocabulary",
        )
    )
    profile_vocab = set(
        ensure_list(
            manifest.get("deployment_profile_vocabulary"),
            "manifest.deployment_profile_vocabulary",
        )
    )
    seat_state_vocab = set(
        ensure_list(
            manifest.get("seat_quota_state_vocabulary"),
            "manifest.seat_quota_state_vocabulary",
        )
    )
    org_switch_vocab = set(
        ensure_list(
            manifest.get("org_switch_behavior_vocabulary"),
            "manifest.org_switch_behavior_vocabulary",
        )
    )
    grace_vocab = set(
        ensure_list(
            manifest.get("grace_window_class_vocabulary"),
            "manifest.grace_window_class_vocabulary",
        )
    )
    phase_vocab = set(
        ensure_list(
            manifest.get("offboarding_phase_vocabulary"),
            "manifest.offboarding_phase_vocabulary",
        )
    )
    export_class_vocab = set(
        ensure_list(
            manifest.get("export_packet_class_vocabulary"),
            "manifest.export_packet_class_vocabulary",
        )
    )
    claim_state_vocab = set(
        ensure_list(
            manifest.get("claim_state_vocabulary"),
            "manifest.claim_state_vocabulary",
        )
    )

    for idx, raw_row in enumerate(rows):
        row = ensure_dict(raw_row, f"manifest.rows[{idx}]")
        row_id = ensure_str(row.get("row_id"), f"manifest.rows[{idx}].row_id")
        if not row_id.startswith("beta_boundary_row:"):
            findings.append(
                Finding(
                    severity="error",
                    check_id="manifest.rows.row_id.pattern",
                    message=(
                        f"row_id must start with 'beta_boundary_row:', "
                        f"got {row_id!r}"
                    ),
                    remediation="Namespace stable row ids under beta_boundary_row.",
                    ref=row_id,
                )
            )
        if row_id in seen_row_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="manifest.rows.duplicate_row_id",
                    message=f"duplicate row_id: {row_id}",
                    remediation="Row ids MUST be stable and unique.",
                    ref=row_id,
                )
            )
        seen_row_ids.add(row_id)

        ensure_str(row.get("title"), f"manifest.rows[{idx}].title")
        ensure_str(row.get("summary"), f"manifest.rows[{idx}].summary")

        boundary_class = ensure_str(
            row.get("boundary_class"), f"manifest.rows[{idx}].boundary_class"
        )
        if boundary_class not in boundary_vocab:
            findings.append(
                Finding(
                    severity="error",
                    check_id="manifest.rows.boundary_class.unknown",
                    message=(
                        f"row {row_id} cites unknown boundary_class "
                        f"{boundary_class!r}"
                    ),
                    remediation=(
                        "Use a class from boundary_class_vocabulary "
                        "or add the value first."
                    ),
                    ref=row_id,
                )
            )
        seen_boundary_classes.add(boundary_class)

        claim_state = ensure_str(
            row.get("claim_state"), f"manifest.rows[{idx}].claim_state"
        )
        if claim_state not in claim_state_vocab:
            findings.append(
                Finding(
                    severity="error",
                    check_id="manifest.rows.claim_state.unknown",
                    message=(
                        f"row {row_id} cites unknown claim_state "
                        f"{claim_state!r}"
                    ),
                    remediation="Use a value from claim_state_vocabulary.",
                    ref=row_id,
                )
            )

        profiles = ensure_list(
            row.get("deployment_profiles"),
            f"manifest.rows[{idx}].deployment_profiles",
        )
        for profile in profiles:
            if profile not in profile_vocab:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="manifest.rows.deployment_profile.unknown",
                        message=(
                            f"row {row_id} cites unknown deployment profile "
                            f"{profile!r}"
                        ),
                        remediation=(
                            "Use a profile id from "
                            "deployment_profile_vocabulary."
                        ),
                        ref=row_id,
                    )
                )

        ensure_str(
            row.get("local_core_continuity"),
            f"manifest.rows[{idx}].local_core_continuity",
        )

        org_switch = ensure_dict(
            row.get("org_switch_behavior"),
            f"manifest.rows[{idx}].org_switch_behavior",
        )
        behavior_class = ensure_str(
            org_switch.get("behavior_class"),
            f"manifest.rows[{idx}].org_switch_behavior.behavior_class",
        )
        if behavior_class not in org_switch_vocab:
            findings.append(
                Finding(
                    severity="error",
                    check_id="manifest.rows.org_switch.unknown",
                    message=(
                        f"row {row_id} cites unknown org_switch behavior "
                        f"{behavior_class!r}"
                    ),
                    remediation=(
                        "Use a class from org_switch_behavior_vocabulary."
                    ),
                    ref=row_id,
                )
            )
        ensure_str(
            org_switch.get("summary"),
            f"manifest.rows[{idx}].org_switch_behavior.summary",
        )

        grace = ensure_dict(
            row.get("grace_window"), f"manifest.rows[{idx}].grace_window"
        )
        window_class = ensure_str(
            grace.get("window_class"),
            f"manifest.rows[{idx}].grace_window.window_class",
        )
        if window_class not in grace_vocab:
            findings.append(
                Finding(
                    severity="error",
                    check_id="manifest.rows.grace_window.unknown",
                    message=(
                        f"row {row_id} cites unknown grace window class "
                        f"{window_class!r}"
                    ),
                    remediation=(
                        "Use a class from grace_window_class_vocabulary."
                    ),
                    ref=row_id,
                )
            )
        duration = grace.get("duration_iso8601")
        if duration is not None:
            if not isinstance(duration, str) or not DURATION_PATTERN.match(duration):
                findings.append(
                    Finding(
                        severity="error",
                        check_id="manifest.rows.grace_window.duration_invalid",
                        message=(
                            f"row {row_id} grace_window.duration_iso8601 must "
                            f"match P<N>D or PT<N>H, got {duration!r}"
                        ),
                        remediation="Use an ISO-8601 duration like P7D or PT12H.",
                        ref=row_id,
                    )
                )
        ensure_str(
            grace.get("summary"),
            f"manifest.rows[{idx}].grace_window.summary",
        )

        seat = ensure_dict(
            row.get("seat_quota"), f"manifest.rows[{idx}].seat_quota"
        )
        quota_state = ensure_str(
            seat.get("quota_state"),
            f"manifest.rows[{idx}].seat_quota.quota_state",
        )
        if quota_state not in seat_state_vocab:
            findings.append(
                Finding(
                    severity="error",
                    check_id="manifest.rows.seat_quota.state_unknown",
                    message=(
                        f"row {row_id} cites unknown quota_state "
                        f"{quota_state!r}"
                    ),
                    remediation=(
                        "Use a state from seat_quota_state_vocabulary."
                    ),
                    ref=row_id,
                )
            )
        states_observed = ensure_list(
            seat.get("states_observed"),
            f"manifest.rows[{idx}].seat_quota.states_observed",
        )
        for state in states_observed:
            if state not in seat_state_vocab:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="manifest.rows.seat_quota.observed_unknown",
                        message=(
                            f"row {row_id} cites unknown observed state "
                            f"{state!r}"
                        ),
                        remediation=(
                            "Use a state from seat_quota_state_vocabulary."
                        ),
                        ref=row_id,
                    )
                )
        ensure_str(
            seat.get("summary"),
            f"manifest.rows[{idx}].seat_quota.summary",
        )

        offboarding = ensure_dict(
            row.get("offboarding"), f"manifest.rows[{idx}].offboarding"
        )
        phases = ensure_list(
            offboarding.get("phases_observed"),
            f"manifest.rows[{idx}].offboarding.phases_observed",
        )
        for phase in phases:
            if phase not in phase_vocab:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="manifest.rows.offboarding.phase_unknown",
                        message=(
                            f"row {row_id} cites unknown offboarding phase "
                            f"{phase!r}"
                        ),
                        remediation=(
                            "Use a phase from offboarding_phase_vocabulary."
                        ),
                        ref=row_id,
                    )
                )
        export_class = ensure_str(
            offboarding.get("export_packet_class"),
            f"manifest.rows[{idx}].offboarding.export_packet_class",
        )
        if export_class not in export_class_vocab:
            findings.append(
                Finding(
                    severity="error",
                    check_id="manifest.rows.offboarding.export_class_unknown",
                    message=(
                        f"row {row_id} cites unknown export_packet_class "
                        f"{export_class!r}"
                    ),
                    remediation=(
                        "Use a class from export_packet_class_vocabulary."
                    ),
                    ref=row_id,
                )
            )
        ensure_str(
            offboarding.get("summary"),
            f"manifest.rows[{idx}].offboarding.summary",
        )

        narrows = row.get("absence_narrows_to")
        if boundary_class in NARROWS_REQUIRED_FOR:
            if not narrows or not str(narrows).strip():
                findings.append(
                    Finding(
                        severity="error",
                        check_id="manifest.rows.absence_narrows_to.required",
                        message=(
                            f"row {row_id} ({boundary_class}) MUST declare "
                            "absence_narrows_to so loss of the managed/paid "
                            "surface narrows rather than failing silently"
                        ),
                        remediation=(
                            "Add an absence_narrows_to clause naming the "
                            "narrowed local or self-hosted fallback."
                        ),
                        ref=row_id,
                    )
                )

        linked_evidence_class = ensure_str(
            row.get("linked_evidence_class"),
            f"manifest.rows[{idx}].linked_evidence_class",
        )
        if boundary_class in NARROWS_REQUIRED_FOR and linked_evidence_class not in BETA_EVIDENCE_CLASSES:
            findings.append(
                Finding(
                    severity="error",
                    check_id="manifest.rows.linked_evidence_class.alpha_only",
                    message=(
                        f"row {row_id} ({boundary_class}) cites "
                        f"linked_evidence_class {linked_evidence_class!r}; "
                        "managed and paid_seat_bound rows MUST link a "
                        "beta-era evidence class"
                    ),
                    remediation=(
                        "Link a beta_partner_packet, beta_drill, or "
                        "policy_decision evidence class, not alpha_seed."
                    ),
                    ref=row_id,
                )
            )

        evidence_refs = ensure_list(
            row.get("evidence_refs"), f"manifest.rows[{idx}].evidence_refs"
        )
        if not evidence_refs:
            findings.append(
                Finding(
                    severity="error",
                    check_id="manifest.rows.evidence_refs.empty",
                    message=f"row {row_id} must cite at least one evidence ref",
                    remediation=(
                        "Cite a checked-in evidence artifact so the row is "
                        "inspectable."
                    ),
                    ref=row_id,
                )
            )
        for evidence_idx, evidence_ref in enumerate(evidence_refs):
            evidence_ref = ensure_str(
                evidence_ref,
                f"manifest.rows[{idx}].evidence_refs[{evidence_idx}]",
            )
            if not ref_exists(repo_root, evidence_ref):
                findings.append(
                    Finding(
                        severity="error",
                        check_id="manifest.rows.evidence_refs.missing",
                        message=(
                            f"row {row_id} evidence ref does not exist: "
                            f"{evidence_ref}"
                        ),
                        remediation=(
                            "Seed the evidence artifact or fix the path."
                        ),
                        ref=evidence_ref,
                    )
                )

        derived.append(
            {
                "row_id": row_id,
                "boundary_class": boundary_class,
                "claim_state": claim_state,
                "deployment_profiles": list(profiles),
                "org_switch_behavior": behavior_class,
                "grace_window_class": window_class,
                "grace_window_duration": duration,
                "quota_state": quota_state,
                "states_observed": list(states_observed),
                "offboarding_phases": list(phases),
                "export_packet_class": export_class,
                "linked_evidence_class": linked_evidence_class,
            }
        )

    required_coverage = set(
        ensure_list(
            manifest.get("required_boundary_class_coverage", []),
            "manifest.required_boundary_class_coverage",
        )
    ) or REQUIRED_BOUNDARY_CLASSES
    missing_coverage = required_coverage - seen_boundary_classes
    if missing_coverage:
        findings.append(
            Finding(
                severity="error",
                check_id="manifest.required_boundary_class.missing",
                message=(
                    "required boundary classes have no row in the manifest: "
                    f"{sorted(missing_coverage)}"
                ),
                remediation=(
                    "Seed at least one row per required boundary class so "
                    "the beta vocabulary is fully exercised."
                ),
            )
        )

    return derived


def validate_release_doc(
    repo_root: Path,
    release_doc_rel: str,
    findings: list[Finding],
) -> None:
    doc_path = repo_root / release_doc_rel
    if not doc_path.exists():
        findings.append(
            Finding(
                severity="error",
                check_id="release_doc.missing",
                message=f"release doc does not exist: {release_doc_rel}",
                remediation="Seed the release doc.",
                ref=release_doc_rel,
            )
        )
        return
    body = doc_path.read_text(encoding="utf-8")
    for phrase in REQUIRED_DOC_PHRASES:
        if phrase not in body:
            findings.append(
                Finding(
                    severity="error",
                    check_id="release_doc.missing_phrase",
                    message=(
                        f"release doc is missing required phrase: {phrase!r}"
                    ),
                    remediation=(
                        "Update the release doc so it carries the standard "
                        "sections, schema/validator refs, and failure drill."
                    ),
                    ref=release_doc_rel,
                )
            )


def validate_alpha_continuity(
    manifest: dict[str, Any],
    alpha_manifest: dict[str, Any],
    findings: list[Finding],
) -> None:
    alpha_rows = alpha_manifest.get("capability_rows", [])
    alpha_capability_ids = {
        row.get("capability_id") for row in alpha_rows if isinstance(row, dict)
    }
    for idx, raw_row in enumerate(manifest.get("rows", [])):
        if not isinstance(raw_row, dict):
            continue
        row_id = raw_row.get("row_id", f"row[{idx}]")
        for cap_ref in raw_row.get("alpha_capability_refs", []) or []:
            if cap_ref not in alpha_capability_ids:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="manifest.rows.alpha_capability_refs.unknown",
                        message=(
                            f"row {row_id} cites alpha capability "
                            f"{cap_ref!r} that is not in the alpha boundary "
                            "manifest"
                        ),
                        remediation=(
                            "Use a capability id from the alpha boundary "
                            "manifest, or remove the ref."
                        ),
                        ref=row_id,
                    )
                )


def write_capture(
    repo_root: Path,
    capture_rel: str,
    manifest_rel: str,
    derived_rows: list[dict[str, Any]],
    findings: list[Finding],
    generated_at: str,
    check_only: bool,
) -> bool:
    capture_path = repo_root / capture_rel
    capture_path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": 1,
        "check_id": "m3_boundary_manifest_beta",
        "generated_at": generated_at,
        "manifest_ref": manifest_rel,
        "status": "pass"
        if not any(f.severity == "error" for f in findings)
        else "fail",
        "finding_counts": {
            "error": sum(1 for f in findings if f.severity == "error"),
            "warning": sum(1 for f in findings if f.severity == "warning"),
        },
        "findings": [f.as_report() for f in findings],
        "row_projection": derived_rows,
    }
    new_text = json.dumps(payload, indent=2, sort_keys=True) + "\n"
    old_text: str | None = None
    if capture_path.exists():
        old_text = capture_path.read_text(encoding="utf-8")
    changed = old_text is None or _normalize(old_text) != _normalize(new_text)
    if not check_only:
        capture_path.write_text(new_text, encoding="utf-8")
    return changed


_GENERATED_AT_RE = re.compile(r'"generated_at":\s*"[^"]*"')


def _normalize(text: str) -> str:
    return _GENERATED_AT_RE.sub('"generated_at": "__generated_at__"', text)


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(
            f"--repo-root does not look like a repository root: {repo_root}"
        )

    manifest_path = repo_root / args.manifest
    manifest = ensure_dict(
        render_yaml_file_as_json(manifest_path), args.manifest
    )

    findings: list[Finding] = []
    validate_header(
        repo_root, manifest, args.schema, args.release_doc, findings
    )
    derived_rows = validate_rows(repo_root, manifest, findings)
    validate_release_doc(repo_root, args.release_doc, findings)

    alpha_ref = (
        manifest.get("source_contract_refs", {}) or {}
    ).get("alpha_boundary_manifest_ref")
    if isinstance(alpha_ref, str) and (repo_root / alpha_ref).exists():
        alpha_manifest = ensure_dict(
            render_yaml_file_as_json(repo_root / alpha_ref),
            alpha_ref,
        )
        validate_alpha_continuity(manifest, alpha_manifest, findings)

    generated_at = (
        dt.datetime.now(dt.timezone.utc)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )

    capture_changed = write_capture(
        repo_root=repo_root,
        capture_rel=args.capture,
        manifest_rel=args.manifest,
        derived_rows=derived_rows,
        findings=findings,
        generated_at=generated_at,
        check_only=args.check,
    )

    if args.check and capture_changed:
        findings.append(
            Finding(
                severity="error",
                check_id="capture.stale",
                message=(
                    "checked-in boundary manifest beta capture is stale "
                    "relative to the manifest, release doc, or schema"
                ),
                remediation=(
                    "Run `python3 ci/check_m3_boundary_manifest_beta.py "
                    "--repo-root .` and commit the regenerated capture."
                ),
                details={"capture_changed": capture_changed},
            )
        )
        write_capture(
            repo_root=repo_root,
            capture_rel=args.capture,
            manifest_rel=args.manifest,
            derived_rows=derived_rows,
            findings=findings,
            generated_at=generated_at,
            check_only=False,
        )

    errors = [finding for finding in findings if finding.severity == "error"]
    warnings = [finding for finding in findings if finding.severity == "warning"]
    status = "PASS" if not errors else "FAIL"
    print(
        f"[m3-boundary-manifest-beta] {status} "
        f"({len(errors)} errors, {len(warnings)} warnings)"
    )
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        print(
            f"[m3-boundary-manifest-beta] {prefix} {finding.check_id}: "
            f"{finding.message}{ref_suffix}"
        )
        print(
            f"[m3-boundary-manifest-beta]   remediation: {finding.remediation}"
        )
    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[m3-boundary-manifest-beta] interrupted", file=sys.stderr)
        sys.exit(130)
