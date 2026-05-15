#!/usr/bin/env python3
"""Generate and validate the M3 governed compatibility report.

The script reads the canonical compatibility seeds and the M3-scoped
skew-window matrix and writes:

  - artifacts/compat/m3/compatibility_report.json — machine-readable
    record conforming to schemas/governance/compatibility_report.schema.json;
  - artifacts/compat/m3/compatibility_report.md   — reviewer-facing
    rendering with the same row truth; and
  - artifacts/compat/m3/captures/compatibility_report_validation_capture.json
    — checked-in validation capture for downstream reviewers.

The report is generated from the matrices (qualification matrix, skew
windows, version-skew register, and the cohort/archetype scorecard
register) rather than authored as prose, so release evidence, partner
packets, docs, and Help/About surfaces all read one machine-derived
truth.
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


DEFAULT_MATRIX_REL = "artifacts/compat/m3/skew_window_matrix.yaml"
DEFAULT_REPORT_JSON_REL = "artifacts/compat/m3/compatibility_report.json"
DEFAULT_REPORT_MD_REL = "artifacts/compat/m3/compatibility_report.md"
DEFAULT_CAPTURE_REL = (
    "artifacts/compat/m3/captures/compatibility_report_validation_capture.json"
)
DEFAULT_SCHEMA_REL = "schemas/governance/compatibility_report.schema.json"

REQUIRED_ROW_SCOPES = {
    "desktop",
    "helper",
    "extension",
    "schema",
    "provider",
    "deployment_profile",
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
    parser.add_argument("--matrix", default=DEFAULT_MATRIX_REL)
    parser.add_argument("--report-json", default=DEFAULT_REPORT_JSON_REL)
    parser.add_argument("--report-md", default=DEFAULT_REPORT_MD_REL)
    parser.add_argument("--capture", default=DEFAULT_CAPTURE_REL)
    parser.add_argument("--schema", default=DEFAULT_SCHEMA_REL)
    parser.add_argument(
        "--check",
        action="store_true",
        help=(
            "Fail if the on-disk report or capture would change after "
            "regeneration. Use this in CI to keep the checked-in artifacts "
            "fresh against the matrices."
        ),
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
        raise SystemExit(f"failed to parse YAML at {path}: {stderr}")
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
        raise SystemExit(f"{label} must be an object/mapping")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a list/array")
    return value


def ensure_str(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value.strip():
        raise SystemExit(f"{label} must be a non-empty string")
    return value.strip()


PATH_LIKE_SUFFIXES = (".yaml", ".yml", ".json", ".md", ".toml", ".py", ".mmd")
ID_PREFIXES = (
    "archetype_row:",
    "beta_archetype:",
    "beta_surface:",
    "cohort:",
    "compat_row:",
    "compat_report:",
    "scoreboard_row:",
)


def looks_like_path(ref: str) -> bool:
    clean = ref.split("#", 1)[0].strip()
    if not clean or clean.startswith(ID_PREFIXES):
        return False
    return "/" in clean or clean.endswith(PATH_LIKE_SUFFIXES)


def validate_path_refs(
    repo_root: Path,
    refs: list[str],
    label: str,
    findings: list[Finding],
) -> None:
    for idx, ref in enumerate(refs):
        if not looks_like_path(ref):
            continue
        clean = ref.split("#", 1)[0].strip()
        if not (repo_root / clean).exists():
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"{label}.missing_ref",
                    message=f"{label}[{idx}] does not resolve: {ref}",
                    remediation=(
                        "Fix the path or seed the referenced artifact so the "
                        "compatibility report stays inspectable."
                    ),
                    ref=ref,
                )
            )


def index_qualification_rows(qm: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {
        row["row_id"]: row
        for row in ensure_list(
            qm.get("qualification_rows"), "qualification_matrix.qualification_rows"
        )
    }


def index_skew_windows(sw: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {
        row["skew_window_id"]: row
        for row in ensure_list(sw.get("declarations"), "skew_windows.declarations")
    }


def index_skew_register(reg: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {
        row["register_id"]: row
        for row in ensure_list(reg.get("register"), "version_skew_register.register")
    }


def primary_skew_case(register_entry: dict[str, Any]) -> str:
    """Return the canonical "supported" skew_case for a register entry."""

    supported = ensure_list(
        register_entry.get("supported"),
        f"{register_entry.get('register_id')}.supported",
    )
    if not supported:
        # Fall back to the first declared bucket so the report still
        # cites a stable skew_case_id rather than fabricating one.
        for bucket in ("best_effort", "untested", "unsupported"):
            entries = ensure_list(
                register_entry.get(bucket, []),
                f"{register_entry.get('register_id')}.{bucket}",
            )
            if entries:
                return ensure_str(
                    entries[0].get("skew_case_id"),
                    f"{register_entry.get('register_id')}.{bucket}[0].skew_case_id",
                )
        raise SystemExit(
            f"register entry {register_entry.get('register_id')} has no skew cases"
        )
    return ensure_str(
        supported[0].get("skew_case_id"),
        f"{register_entry.get('register_id')}.supported[0].skew_case_id",
    )


def index_archetype_rows(register: dict[str, Any]) -> dict[str, dict[str, Any]]:
    archetype_rows = ensure_list(
        register.get("claimed_archetype_rows"),
        "claimed_surface_register.claimed_archetype_rows",
    )
    return {row["archetype_row_ref"]: row for row in archetype_rows}


def index_scorecard_rows(register: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {
        row["scorecard_id"]: row
        for row in ensure_list(
            register.get("archetype_rows"), "scorecard_register.archetype_rows"
        )
        + ensure_list(
            register.get("cohort_rows"), "scorecard_register.cohort_rows"
        )
    }


def index_claimed_surfaces(register: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {
        row["surface_id"]: row
        for row in ensure_list(
            register.get("claimed_surfaces"),
            "claimed_surface_register.claimed_surfaces",
        )
    }


def index_cohorts_from_guardrails(
    repo_root: Path,
) -> set[str]:
    guardrails = ensure_dict(
        render_yaml_as_json(
            repo_root / "artifacts/milestones/m3/cohort_guardrails.yaml"
        ),
        "cohort_guardrails",
    )
    cohorts = ensure_list(guardrails.get("cohorts"), "cohort_guardrails.cohorts")
    return {ensure_str(c.get("cohort_id"), "cohort_guardrails.cohorts[].cohort_id") for c in cohorts}


SKEW_WINDOW_DEFAULTS_BY_QUALIFICATION_CLASS: dict[str, dict[str, str]] = {
    "exact_match": {
        "support_class": "downgrade_unsupported",
        "state_preservation_note": (
            "Exact-match windows do not support downgrade; readers refuse "
            "the claim path when identity does not resolve."
        ),
        "contract_rule": (
            "A reader that cannot resolve the exact identity refuses the "
            "claim-bearing path with an attributed reason."
        ),
    },
    "coordinated_artifact_set_only": {
        "support_class": "downgrade_requires_coordinated_artifact_set",
        "state_preservation_note": (
            "Rollback restores the prior coordinated artifact set; "
            "individual components do not downgrade independently."
        ),
        "contract_rule": (
            "Mixed startup outside the coordinated artifact set is refused, "
            "not warned."
        ),
    },
    "same_schema_epoch_additive_only": {
        "support_class": "downgrade_supported",
        "state_preservation_note": (
            "Additive-only downgrade preserves user-authored durable truth; "
            "meaning-changing downgrades refuse rather than rewrite state."
        ),
        "contract_rule": (
            "Consumers that cannot attribute a required field refuse the "
            "payload with an attributed error."
        ),
    },
    "declared_adjacent_window": {
        "support_class": "downgrade_best_effort",
        "state_preservation_note": (
            "Review-only or file-only attach remains available during "
            "downgrade; mutating attach may narrow."
        ),
        "contract_rule": (
            "Out-of-window attach degrades to file-or-review-only mode or "
            "refuses outright; never partially loads the mutating session."
        ),
    },
    "published_sdk_support_window": {
        "support_class": "downgrade_unsupported",
        "state_preservation_note": (
            "Host downgrade disables or quarantines extensions that expect "
            "a newer permission vocabulary rather than partially loading "
            "them."
        ),
        "contract_rule": (
            "Required WIT world, permission version, or SDK floor outside "
            "the published window is a refusal, not a warning."
        ),
    },
    "current_plus_previous_minor_or_lts": {
        "support_class": "downgrade_best_effort",
        "state_preservation_note": (
            "Cached read-only state remains usable; privileged write paths "
            "are refused rather than partially executed."
        ),
        "contract_rule": (
            "Privileged or approval-bearing writes targeting an API family "
            "outside the published window fail closed; cached reads remain "
            "available."
        ),
    },
    "certified_report_freshness_window": {
        "support_class": "downgrade_best_effort",
        "state_preservation_note": (
            "Certified wording narrows automatically when freshness or "
            "evidence drifts; the row remains inspectable in narrowed form."
        ),
        "contract_rule": (
            "Stale or missing reference reports narrow the claim or refuse "
            "promotion rather than render certified wording without proof."
        ),
    },
}


def derive_downgrade_behavior(
    skew_window_decl: dict[str, Any] | None,
    qualification_row: dict[str, Any],
    out_of_window_posture: str,
) -> dict[str, str]:
    if skew_window_decl is not None:
        downgrade = ensure_dict(
            skew_window_decl.get("downgrade_behavior"),
            f"{skew_window_decl.get('skew_window_id')}.downgrade_behavior",
        )
        unsupported = ensure_dict(
            skew_window_decl.get("unsupported_state_behavior"),
            f"{skew_window_decl.get('skew_window_id')}.unsupported_state_behavior",
        )
        return {
            "support_class": ensure_str(
                downgrade.get("support_class"),
                f"{skew_window_decl.get('skew_window_id')}.downgrade.support_class",
            ),
            "out_of_window_posture": ensure_str(
                unsupported.get("out_of_window_posture"),
                f"{skew_window_decl.get('skew_window_id')}.posture",
            ),
            "state_preservation_note": ensure_str(
                downgrade.get("state_preservation_note"),
                f"{skew_window_decl.get('skew_window_id')}.state_preservation_note",
            ),
            "contract_rule": ensure_str(
                unsupported.get("contract_rule"),
                f"{skew_window_decl.get('skew_window_id')}.contract_rule",
            ),
        }

    qualification_class = ensure_str(
        qualification_row.get("skew_window_class"),
        f"{qualification_row.get('row_id')}.skew_window_class",
    )
    defaults = SKEW_WINDOW_DEFAULTS_BY_QUALIFICATION_CLASS.get(qualification_class)
    if defaults is None:
        raise SystemExit(
            "no downgrade defaults known for skew_window_class "
            f"{qualification_class!r}; extend "
            "SKEW_WINDOW_DEFAULTS_BY_QUALIFICATION_CLASS."
        )
    return {
        "support_class": defaults["support_class"],
        "out_of_window_posture": out_of_window_posture,
        "state_preservation_note": defaults["state_preservation_note"],
        "contract_rule": defaults["contract_rule"],
    }


def compose_report(
    repo_root: Path,
    matrix: dict[str, Any],
    qualification_index: dict[str, dict[str, Any]],
    skew_window_index: dict[str, dict[str, Any]],
    skew_register_index: dict[str, dict[str, Any]],
    scorecard_index: dict[str, dict[str, Any]],
    archetype_index: dict[str, dict[str, Any]],
    claimed_surface_index: dict[str, dict[str, Any]],
    known_cohort_ids: set[str],
    findings: list[Finding],
    generated_at: str,
) -> dict[str, Any]:
    rows: list[dict[str, Any]] = []
    seen_scopes: set[str] = set()

    for idx, raw_row in enumerate(
        ensure_list(matrix.get("rows"), "matrix.rows")
    ):
        row = ensure_dict(raw_row, f"matrix.rows[{idx}]")
        report_row_id = ensure_str(
            row.get("report_row_id"), f"matrix.rows[{idx}].report_row_id"
        )
        compat_row_id = ensure_str(
            row.get("row_id"), f"{report_row_id}.row_id"
        )
        row_scope = ensure_str(
            row.get("row_scope"), f"{report_row_id}.row_scope"
        )
        if row_scope not in REQUIRED_ROW_SCOPES:
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.row_scope.invalid",
                    message=(
                        f"{report_row_id} declares unknown row_scope "
                        f"{row_scope!r}"
                    ),
                    remediation=(
                        "Use one of the spec-mandated row scopes: "
                        + ", ".join(sorted(REQUIRED_ROW_SCOPES))
                    ),
                    ref=report_row_id,
                )
            )
        seen_scopes.add(row_scope)

        qualification_row = qualification_index.get(compat_row_id)
        if qualification_row is None:
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.row_id.unknown",
                    message=(
                        f"{report_row_id} cites unknown compat_row_id: "
                        f"{compat_row_id}"
                    ),
                    remediation=(
                        "Use a row_id from artifacts/compat/qualification_matrix_seed.yaml."
                    ),
                    ref=report_row_id,
                )
            )
            continue

        version_register_ref = ensure_str(
            qualification_row.get("version_skew_register_ref"),
            f"{compat_row_id}.version_skew_register_ref",
        )
        register_entry = skew_register_index.get(version_register_ref)
        if register_entry is None:
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.skew_register.unknown",
                    message=(
                        f"{compat_row_id} version_skew_register_ref "
                        f"{version_register_ref} not present in version_skew_register"
                    ),
                    remediation=(
                        "Seed the matching entry in artifacts/compat/version_skew_register.yaml."
                    ),
                    ref=report_row_id,
                )
            )
            continue
        current_skew_case_ref = primary_skew_case(register_entry)

        skew_window_id = row.get("skew_window_id")
        skew_window_decl: dict[str, Any] | None = None
        if skew_window_id is not None:
            skew_window_id = ensure_str(
                skew_window_id, f"{report_row_id}.skew_window_id"
            )
            skew_window_decl = skew_window_index.get(skew_window_id)
            if skew_window_decl is None:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="matrix.skew_window.unknown",
                        message=(
                            f"{report_row_id} cites unknown skew_window_id: "
                            f"{skew_window_id}"
                        ),
                        remediation=(
                            "Use a skew_window_id from artifacts/compat/skew_windows.yaml or set "
                            "skew_window_id to null."
                        ),
                        ref=report_row_id,
                    )
                )
                continue
            decl_qualification_ref = ensure_str(
                skew_window_decl.get("qualification_row_ref"),
                f"{skew_window_id}.qualification_row_ref",
            )
            if decl_qualification_ref != compat_row_id:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="matrix.skew_window.row_mismatch",
                        message=(
                            f"{report_row_id} skew_window {skew_window_id} "
                            f"binds to {decl_qualification_ref}, not {compat_row_id}"
                        ),
                        remediation=(
                            "Pick a skew_window_id whose qualification_row_ref matches the row, "
                            "or set skew_window_id to null."
                        ),
                        ref=report_row_id,
                    )
                )
                continue

        primary_cohorts = [
            ensure_str(ref, f"{report_row_id}.primary_cohort_refs[{i}]")
            for i, ref in enumerate(
                ensure_list(
                    row.get("primary_cohort_refs"),
                    f"{report_row_id}.primary_cohort_refs",
                )
            )
        ]
        for cohort_ref in primary_cohorts:
            if cohort_ref not in known_cohort_ids:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="matrix.cohort_ref.unknown",
                        message=(
                            f"{report_row_id} cites unknown cohort_id: {cohort_ref}"
                        ),
                        remediation=(
                            "Use a cohort_id from artifacts/milestones/m3/cohort_guardrails.yaml."
                        ),
                        ref=report_row_id,
                    )
                )

        claimed_beta_surfaces = [
            ensure_str(ref, f"{report_row_id}.claimed_beta_surface_refs[{i}]")
            for i, ref in enumerate(
                ensure_list(
                    row.get("claimed_beta_surface_refs"),
                    f"{report_row_id}.claimed_beta_surface_refs",
                )
            )
        ]
        for surface_ref in claimed_beta_surfaces:
            if surface_ref not in claimed_surface_index:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="matrix.surface_ref.unknown",
                        message=(
                            f"{report_row_id} cites unknown beta surface: {surface_ref}"
                        ),
                        remediation=(
                            "Use a surface_id from artifacts/milestones/m3/claimed_surface_register.json."
                        ),
                        ref=report_row_id,
                    )
                )

        declared_support = ensure_str(
            qualification_row.get("support_class"),
            f"{compat_row_id}.support_class",
        )

        scorecard_ref = row.get("scorecard_ref")
        if scorecard_ref is not None:
            scorecard_ref = ensure_str(
                scorecard_ref, f"{report_row_id}.scorecard_ref"
            )
            scorecard_row = scorecard_index.get(scorecard_ref)
            if scorecard_row is None:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="matrix.scorecard_ref.unknown",
                        message=(
                            f"{report_row_id} cites unknown scorecard_id: {scorecard_ref}"
                        ),
                        remediation=(
                            "Use a scorecard_id from "
                            "artifacts/milestones/m3/captures/cohort_archetype_scorecard_register.json."
                        ),
                        ref=report_row_id,
                    )
                )
                effective_support = declared_support
                triggers = ["none"]
            else:
                effective_support = ensure_str(
                    scorecard_row.get("effective_support_class"),
                    f"{scorecard_ref}.effective_support_class",
                )
                triggers = [
                    ensure_str(
                        trigger,
                        f"{scorecard_ref}.downgrade_triggers_fired[{t}]",
                    )
                    for t, trigger in enumerate(
                        ensure_list(
                            scorecard_row.get("downgrade_triggers_fired"),
                            f"{scorecard_ref}.downgrade_triggers_fired",
                        )
                    )
                ] or ["none"]
        else:
            effective_support = declared_support
            triggers = ["none"]

        out_of_window_posture = ensure_str(
            qualification_row.get("out_of_window_posture"),
            f"{compat_row_id}.out_of_window_posture",
        )
        downgrade_behavior = derive_downgrade_behavior(
            skew_window_decl, qualification_row, out_of_window_posture
        )

        evidence_refs = [
            ensure_str(ref, f"{compat_row_id}.supporting_artifact_refs[{i}]")
            for i, ref in enumerate(
                ensure_list(
                    qualification_row.get("supporting_artifact_refs"),
                    f"{compat_row_id}.supporting_artifact_refs",
                )
            )
        ]

        skew_window_summary = ensure_str(
            qualification_row.get("supported_skew_window"),
            f"{compat_row_id}.supported_skew_window",
        )
        if skew_window_decl is not None:
            decl_summary = ensure_str(
                ensure_dict(
                    skew_window_decl.get("supported_window"),
                    f"{skew_window_id}.supported_window",
                ).get("summary"),
                f"{skew_window_id}.supported_window.summary",
            )
            skew_window_summary = decl_summary

        notes = ensure_str(row.get("notes"), f"{report_row_id}.notes")

        rows.append(
            {
                "report_row_id": report_row_id,
                "row_id": compat_row_id,
                "artifact_or_protocol_boundary_label": ensure_str(
                    qualification_row.get("artifact_or_protocol_boundary_label"),
                    f"{compat_row_id}.artifact_or_protocol_boundary_label",
                ),
                "row_scope": row_scope,
                "claimed_surface": ensure_str(
                    qualification_row.get("claimed_surface"),
                    f"{compat_row_id}.claimed_surface",
                ),
                "support_class": {
                    "declared": declared_support,
                    "effective": effective_support,
                    "downgrade_triggers_fired": triggers,
                    "scorecard_ref": scorecard_ref,
                },
                "client_scope": {
                    "deployment_profiles": [
                        ensure_str(
                            p,
                            f"{compat_row_id}.claimed_deployment_profiles[{i}]",
                        )
                        for i, p in enumerate(
                            ensure_list(
                                qualification_row.get("claimed_deployment_profiles"),
                                f"{compat_row_id}.claimed_deployment_profiles",
                            )
                        )
                    ],
                    "primary_cohort_refs": primary_cohorts,
                    "claimed_beta_surface_refs": claimed_beta_surfaces,
                },
                "skew_window": {
                    "skew_window_id": skew_window_id,
                    "window_class": ensure_str(
                        qualification_row.get("skew_window_class"),
                        f"{compat_row_id}.skew_window_class",
                    ),
                    "summary": skew_window_summary,
                    "version_skew_register_ref": version_register_ref,
                    "current_skew_case_ref": current_skew_case_ref,
                },
                "downgrade_behavior": downgrade_behavior,
                "evidence_refs": evidence_refs,
                "notes": notes,
            }
        )

    missing_scopes = REQUIRED_ROW_SCOPES - seen_scopes
    if missing_scopes:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.row_scope.missing",
                message=(
                    "M3 compatibility report does not cover required row scopes"
                ),
                remediation=(
                    "Add at least one matrix row for every spec-mandated scope."
                ),
                details={"missing": sorted(missing_scopes)},
            )
        )

    consuming_surfaces = [
        ensure_str(ref, f"matrix.consuming_surfaces[{i}]")
        for i, ref in enumerate(
            ensure_list(
                matrix.get("consuming_surfaces"), "matrix.consuming_surfaces"
            )
        )
    ]
    validate_path_refs(
        repo_root,
        consuming_surfaces,
        "matrix.consuming_surfaces",
        findings,
    )
    for row_payload in rows:
        validate_path_refs(
            repo_root,
            row_payload["evidence_refs"],
            f"{row_payload['report_row_id']}.evidence_refs",
            findings,
        )

    backup_owner = matrix.get("backup_owner")
    backup_waiver = matrix.get("backup_waiver")
    if backup_owner is not None:
        backup_owner = ensure_str(backup_owner, "matrix.backup_owner")
    if backup_waiver is not None:
        backup_waiver = ensure_str(backup_waiver, "matrix.backup_waiver")
    if backup_owner is None and not backup_waiver:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.backup_waiver.required",
                message=(
                    "matrix backup_owner is null but backup_waiver is missing"
                ),
                remediation=(
                    "Set backup_owner to a handle or attach a backup_waiver id."
                ),
            )
        )

    return {
        "schema_version": 1,
        "record_kind": "compatibility_report",
        "report_id": ensure_str(matrix.get("report_id"), "matrix.report_id"),
        "report_revision": int(
            matrix.get("matrix_revision", 1)
        ),
        "milestone_id": ensure_str(
            matrix.get("milestone_id"), "matrix.milestone_id"
        ),
        "report_state": ensure_str(
            matrix.get("report_state"), "matrix.report_state"
        ),
        "release_channel_scope": ensure_str(
            matrix.get("release_channel_scope"),
            "matrix.release_channel_scope",
        ),
        "as_of": ensure_str(matrix.get("as_of"), "matrix.as_of"),
        "generated_at": generated_at,
        "owner": ensure_str(matrix.get("owner"), "matrix.owner"),
        "backup_owner": backup_owner,
        "backup_waiver": backup_waiver,
        "source_refs": {
            "qualification_matrix": ensure_str(
                matrix.get("qualification_matrix_source"),
                "matrix.qualification_matrix_source",
            ),
            "skew_windows": ensure_str(
                matrix.get("skew_windows_source"),
                "matrix.skew_windows_source",
            ),
            "version_skew_register": ensure_str(
                matrix.get("version_skew_register_source"),
                "matrix.version_skew_register_source",
            ),
            "scorecard_register": ensure_str(
                matrix.get("scorecard_register_source"),
                "matrix.scorecard_register_source",
            ),
            "milestone_skew_window_matrix": ensure_str(
                matrix.get("matrix_id"), "matrix.matrix_id"
            )
            and "artifacts/compat/m3/skew_window_matrix.yaml",
            "claimed_surface_register": ensure_str(
                matrix.get("claimed_surface_register_source"),
                "matrix.claimed_surface_register_source",
            ),
        },
        "vocabularies": {
            "support_class": [
                "certified",
                "supported",
                "limited",
                "experimental",
                "community",
                "retest_pending",
                "evidence_stale",
                "unsupported",
            ],
            "deployment_profile": [
                "individual_local",
                "self_hosted",
                "enterprise_online",
                "air_gapped",
                "managed_cloud",
            ],
            "skew_window_class": [
                "coordinated_artifact_set_only",
                "exact_match",
                "same_schema_epoch_additive_only",
                "declared_adjacent_window",
                "published_sdk_support_window",
                "current_plus_previous_minor_or_lts",
                "certified_report_freshness_window",
            ],
            "out_of_window_posture": [
                "fail_closed",
                "read_only",
                "degraded",
                "explicitly_unsupported",
            ],
            "downgrade_support_class": [
                "downgrade_supported",
                "downgrade_best_effort",
                "downgrade_untested",
                "downgrade_unsupported",
                "downgrade_requires_coordinated_artifact_set",
            ],
            "row_scope": sorted(REQUIRED_ROW_SCOPES),
        },
        "consuming_surfaces": consuming_surfaces,
        "rows": rows,
        "notes": (
            "Generated by ci/check_m3_compatibility_report.py from the "
            "checked-in matrices. Do not edit by hand; refresh the matrix "
            "or seed instead and re-run the generator."
        ),
    }


def render_markdown(report: dict[str, Any]) -> str:
    lines: list[str] = []
    lines.append("# M3 governed compatibility report")
    lines.append("")
    lines.append(
        "This file is generated by "
        "`ci/check_m3_compatibility_report.py` from the checked-in "
        "compatibility matrices. Do not hand-edit; update the source "
        "matrix at `artifacts/compat/m3/skew_window_matrix.yaml`, the "
        "qualification matrix, the skew-window declarations, the "
        "version-skew register, or the cohort/archetype scorecard "
        "register and re-run the generator."
    )
    lines.append("")
    lines.append("## Report metadata")
    lines.append("")
    lines.append(f"- **Report id:** `{report['report_id']}`")
    lines.append(f"- **Report revision:** `{report['report_revision']}`")
    lines.append(f"- **Milestone:** `{report['milestone_id']}`")
    lines.append(f"- **Report state:** `{report['report_state']}`")
    lines.append(
        f"- **Release channel scope:** `{report['release_channel_scope']}`"
    )
    lines.append(f"- **As of:** `{report['as_of']}`")
    lines.append(f"- **Generated at:** `{report['generated_at']}`")
    lines.append(f"- **Owner:** {report['owner']}")
    backup_owner = report["backup_owner"] or "_(none, see backup waiver)_"
    lines.append(f"- **Backup owner:** {backup_owner}")
    if report["backup_waiver"]:
        lines.append(f"- **Backup waiver:** `{report['backup_waiver']}`")
    lines.append("")

    lines.append("## Source seeds")
    lines.append("")
    for label, ref in report["source_refs"].items():
        lines.append(f"- `{label}`: `{ref}`")
    lines.append("")

    lines.append("## Consuming surfaces")
    lines.append("")
    for ref in report["consuming_surfaces"]:
        lines.append(f"- `{ref}`")
    lines.append("")

    lines.append("## Row matrix")
    lines.append("")
    lines.append(
        "| Scope | Row | Support class (declared / effective) | Skew window | "
        "Downgrade behavior | Out-of-window posture | Client scope |"
    )
    lines.append(
        "|---|---|---|---|---|---|---|"
    )
    for row in report["rows"]:
        scope = row["row_scope"]
        row_id = row["row_id"]
        sc = row["support_class"]
        support = f"{sc['declared']} / {sc['effective']}"
        sw = row["skew_window"]
        skew = f"{sw['window_class']} (`{sw['skew_window_id'] or 'inline'}`)"
        db = row["downgrade_behavior"]
        downgrade = db["support_class"]
        posture = db["out_of_window_posture"]
        client = ", ".join(row["client_scope"]["deployment_profiles"])
        lines.append(
            f"| `{scope}` | `{row_id}` | {support} | {skew} | {downgrade} | "
            f"{posture} | {client} |"
        )
    lines.append("")

    lines.append("## Per-row detail")
    lines.append("")
    for row in report["rows"]:
        lines.append(f"### `{row['report_row_id']}`")
        lines.append("")
        lines.append(f"- **Compat row id:** `{row['row_id']}`")
        lines.append(
            f"- **Boundary label:** `{row['artifact_or_protocol_boundary_label']}`"
        )
        lines.append(f"- **Row scope:** `{row['row_scope']}`")
        lines.append(f"- **Claimed surface:** `{row['claimed_surface']}`")
        sc = row["support_class"]
        lines.append(
            f"- **Support class:** declared `{sc['declared']}`, "
            f"effective `{sc['effective']}`"
        )
        triggers = ", ".join(f"`{t}`" for t in sc["downgrade_triggers_fired"])
        lines.append(f"- **Downgrade triggers fired:** {triggers}")
        scorecard = sc["scorecard_ref"] or "_(no scorecard binding)_"
        lines.append(f"- **Scorecard ref:** {scorecard}")
        client = row["client_scope"]
        lines.append(
            "- **Client scope:** profiles="
            + ", ".join(f"`{p}`" for p in client["deployment_profiles"])
        )
        if client["primary_cohort_refs"]:
            lines.append(
                "  - primary cohorts: "
                + ", ".join(f"`{c}`" for c in client["primary_cohort_refs"])
            )
        if client["claimed_beta_surface_refs"]:
            lines.append(
                "  - claimed beta surfaces: "
                + ", ".join(
                    f"`{s}`" for s in client["claimed_beta_surface_refs"]
                )
            )
        sw = row["skew_window"]
        lines.append(
            f"- **Skew window:** class `{sw['window_class']}`, "
            f"id `{sw['skew_window_id'] or 'inline'}`, "
            f"register `{sw['version_skew_register_ref']}`, "
            f"current case `{sw['current_skew_case_ref']}`"
        )
        lines.append(f"  - summary: {sw['summary']}")
        db = row["downgrade_behavior"]
        lines.append(
            f"- **Downgrade behavior:** `{db['support_class']}` (out-of-window "
            f"posture `{db['out_of_window_posture']}`)"
        )
        lines.append(
            f"  - state preservation: {db['state_preservation_note']}"
        )
        lines.append(f"  - contract rule: {db['contract_rule']}")
        if row["evidence_refs"]:
            lines.append("- **Evidence refs:**")
            for ref in row["evidence_refs"]:
                lines.append(f"  - `{ref}`")
        lines.append(f"- **Notes:** {row['notes']}")
        lines.append("")

    lines.append("## How to refresh")
    lines.append("")
    lines.append(
        "Run the generator to re-derive the report and refresh the validation "
        "capture in the same change set:"
    )
    lines.append("")
    lines.append("```")
    lines.append("python3 ci/check_m3_compatibility_report.py --repo-root .")
    lines.append("```")
    lines.append("")
    lines.append(
        "Use `--check` in CI to fail when the on-disk report or capture would "
        "drift from the checked-in matrices."
    )
    lines.append("")
    return "\n".join(lines)


def write_capture(
    path: Path,
    findings: list[Finding],
    matrix_rel: str,
    report_json_rel: str,
    report_md_rel: str,
    generated_at: str,
) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": 1,
        "status": "pass"
        if not any(f.severity == "error" for f in findings)
        else "fail",
        "generated_at": generated_at,
        "matrix_ref": matrix_rel,
        "report_json_ref": report_json_rel,
        "report_md_ref": report_md_rel,
        "summary": {
            "errors": sum(1 for f in findings if f.severity == "error"),
            "warnings": sum(1 for f in findings if f.severity == "warning"),
        },
        "findings": [f.as_report() for f in findings],
    }
    path.write_text(
        json.dumps(payload, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )


_GENERATED_AT_RE = re.compile(
    r'"generated_at":\s*"[^"]*"|\*\*Generated at:\*\*\s*`[^`]*`'
)


def _normalize(text: str) -> str:
    return _GENERATED_AT_RE.sub("__generated_at__", text)


def write_if_changed(path: Path, content: str, check_only: bool) -> bool:
    """Write content to path. Return True if normalized content changed.

    The `generated_at` timestamp is excluded from the change check so the
    `--check` mode is idempotent across runs that produced the same row
    truth.
    """

    path.parent.mkdir(parents=True, exist_ok=True)
    existing: str | None = None
    if path.exists():
        existing = path.read_text(encoding="utf-8")
    changed = existing is None or _normalize(existing) != _normalize(content)
    if not check_only:
        path.write_text(content, encoding="utf-8")
    return changed


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    matrix_path = repo_root / args.matrix
    report_json_path = repo_root / args.report_json
    report_md_path = repo_root / args.report_md
    capture_path = repo_root / args.capture
    schema_path = repo_root / args.schema

    if not schema_path.exists():
        raise SystemExit(f"missing schema file: {schema_path}")

    matrix = ensure_dict(render_yaml_as_json(matrix_path), "matrix")

    qualification_matrix = ensure_dict(
        render_yaml_as_json(
            repo_root / matrix["qualification_matrix_source"]
        ),
        "qualification_matrix",
    )
    skew_windows = ensure_dict(
        render_yaml_as_json(repo_root / matrix["skew_windows_source"]),
        "skew_windows",
    )
    version_skew_register = ensure_dict(
        render_yaml_as_json(
            repo_root / matrix["version_skew_register_source"]
        ),
        "version_skew_register",
    )
    scorecard_register = ensure_dict(
        load_json(repo_root / matrix["scorecard_register_source"]),
        "scorecard_register",
    )
    claimed_surface_register = ensure_dict(
        load_json(repo_root / matrix["claimed_surface_register_source"]),
        "claimed_surface_register",
    )

    qualification_index = index_qualification_rows(qualification_matrix)
    skew_window_index = index_skew_windows(skew_windows)
    skew_register_index = index_skew_register(version_skew_register)
    scorecard_index = index_scorecard_rows(scorecard_register)
    archetype_index = index_archetype_rows(claimed_surface_register)
    claimed_surface_index = index_claimed_surfaces(claimed_surface_register)
    known_cohort_ids = index_cohorts_from_guardrails(repo_root)

    findings: list[Finding] = []
    generated_at = (
        dt.datetime.now(dt.UTC)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )

    report = compose_report(
        repo_root=repo_root,
        matrix=matrix,
        qualification_index=qualification_index,
        skew_window_index=skew_window_index,
        skew_register_index=skew_register_index,
        scorecard_index=scorecard_index,
        archetype_index=archetype_index,
        claimed_surface_index=claimed_surface_index,
        known_cohort_ids=known_cohort_ids,
        findings=findings,
        generated_at=generated_at,
    )

    report_json_text = (
        json.dumps(report, indent=2, sort_keys=True) + "\n"
    )
    report_md_text = render_markdown(report)

    json_changed = write_if_changed(
        report_json_path, report_json_text, args.check
    )
    md_changed = write_if_changed(
        report_md_path, report_md_text, args.check
    )

    if args.check and (json_changed or md_changed):
        findings.append(
            Finding(
                severity="error",
                check_id="report.stale",
                message=(
                    "checked-in compatibility report is stale relative to "
                    "the matrices"
                ),
                remediation=(
                    "Run `python3 ci/check_m3_compatibility_report.py --repo-root .` "
                    "and commit the regenerated artifacts."
                ),
                details={
                    "report_json_changed": json_changed,
                    "report_md_changed": md_changed,
                },
            )
        )

    write_capture(
        capture_path,
        findings,
        args.matrix,
        args.report_json,
        args.report_md,
        generated_at,
    )

    errors = [f for f in findings if f.severity == "error"]
    if errors:
        for item in errors:
            ref = f" ({item.ref})" if item.ref else ""
            print(
                f"ERROR [{item.check_id}]{ref}: {item.message}",
                file=sys.stderr,
            )
            print(f"  remediation: {item.remediation}", file=sys.stderr)
        return 1

    print("M3 compatibility report generated and validated")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
