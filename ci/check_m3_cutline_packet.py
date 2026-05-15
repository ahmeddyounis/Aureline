#!/usr/bin/env python3
"""Validate the M3 cutline packet, unlock map, and stable-planning handoff
checklist.

This is the first consumer for the three checked-in artifacts:
  - docs/milestones/m3/cutline_packet.md
  - artifacts/milestones/m3/unlock_map.yaml
  - artifacts/milestones/m3/stable_planning_handoff_checklist.md

The cutline validator checks the unlock map and handoff checklist against
the canonical beta admission artifacts (claimed-surface register and
cohort guardrails) so the cutline cannot drift from the beta scope freeze.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_UNLOCK_MAP_REL = "artifacts/milestones/m3/unlock_map.yaml"
DEFAULT_HANDOFF_REL = "artifacts/milestones/m3/stable_planning_handoff_checklist.md"
DEFAULT_CUTLINE_DOC_REL = "docs/milestones/m3/cutline_packet.md"
DEFAULT_REGISTER_REL = "artifacts/milestones/m3/claimed_surface_register.json"
DEFAULT_COHORTS_REL = "artifacts/milestones/m3/cohort_guardrails.yaml"

REQUIRED_CUTLINE_IDS = {
    "m3_cutline:rollback_proven_in_partner_orgs",
    "m3_cutline:policy_explainability_baseline",
    "m3_cutline:extension_isolation_envelope",
    "m3_cutline:migration_honesty_register",
    "m3_cutline:supportability_baseline",
    "m3_cutline:debug_test_task_truth",
    "m3_cutline:certified_archetype_publication",
}

REQUIRED_M4_UNLOCKS = {
    "m4_unlock:stable_api_freeze",
    "m4_unlock:release_center_rehearsal",
    "m4_unlock:certified_archetype_publication",
}

REQUIRED_HANDOFF_PHRASES = [
    "docs/milestones/m3/cutline_packet.md",
    "artifacts/milestones/m3/unlock_map.yaml",
    "artifacts/milestones/m3/claimed_surface_register.json",
    "artifacts/milestones/m3/cohort_guardrails.yaml",
    "ci/check_m3_cutline_packet.py",
    "Non-descopable M3 truths",
    "Downstream M4 unlocks",
    "How to refresh",
]

REQUIRED_CUTLINE_DOC_PHRASES = [
    "artifacts/milestones/m3/unlock_map.yaml",
    "artifacts/milestones/m3/stable_planning_handoff_checklist.md",
    "ci/check_m3_cutline_packet.py",
    "Non-descopable M3 truths",
    "Definition of green",
    "Update rules",
]

PATH_LIKE_SUFFIXES = (".yaml", ".yml", ".json", ".md", ".toml", ".rs", ".py", ".mmd")
ID_PREFIXES = (
    "archetype_row:",
    "archetype_certification_seed:",
    "beta_archetype:",
    "beta_surface:",
    "beta_only:",
    "beta_scope.",
    "cohort:",
    "compat_row:",
    "fixture_register:",
    "m3_cutline:",
    "m4_unlock:",
    "scoreboard_row:",
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
    parser.add_argument("--unlock-map", default=DEFAULT_UNLOCK_MAP_REL)
    parser.add_argument("--handoff", default=DEFAULT_HANDOFF_REL)
    parser.add_argument("--cutline-doc", default=DEFAULT_CUTLINE_DOC_REL)
    parser.add_argument("--register", default=DEFAULT_REGISTER_REL)
    parser.add_argument("--cohorts", default=DEFAULT_COHORTS_REL)
    parser.add_argument("--report", default=None)
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


def ensure_int(value: Any, label: str) -> int:
    if not isinstance(value, int):
        raise SystemExit(f"{label} must be an integer")
    return value


def parse_iso_date(value: str, label: str) -> None:
    try:
        dt.date.fromisoformat(value)
    except ValueError as exc:
        raise SystemExit(f"{label} must be a YYYY-MM-DD date, got {value!r}") from exc


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


def validate_path_refs(
    repo_root: Path,
    refs: list[Any],
    label: str,
    findings: list[Finding],
) -> None:
    for idx, raw_ref in enumerate(refs):
        if not isinstance(raw_ref, str) or not raw_ref.strip():
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"{label}.invalid_ref",
                    message=f"{label}[{idx}] must be a non-empty string",
                    remediation="Replace the empty or non-string ref with a repo-relative artifact path or stable row id.",
                )
            )
            continue
        ref = raw_ref.strip()
        if looks_like_path(ref) and not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"{label}.missing_ref",
                    message=f"{label}[{idx}] does not resolve: {ref}",
                    remediation="Fix the path or seed the referenced artifact so the cutline lane stays inspectable.",
                    ref=ref,
                )
            )


def collect_register_surface_ids(register: dict[str, Any]) -> set[str]:
    rows = ensure_list(register.get("claimed_surfaces"), "register.claimed_surfaces")
    return {
        ensure_str(row.get("surface_id"), f"register.claimed_surfaces[{idx}].surface_id")
        for idx, row in enumerate(rows)
    }


def collect_register_held_row_ids(register: dict[str, Any]) -> set[str]:
    rows = ensure_list(register.get("held_or_out_of_scope_surfaces"), "register.held_or_out_of_scope_surfaces")
    return {
        ensure_str(row.get("row_id"), f"register.held_or_out_of_scope_surfaces[{idx}].row_id")
        for idx, row in enumerate(rows)
    }


def collect_cohort_ids(cohorts: dict[str, Any]) -> set[str]:
    rows = ensure_list(cohorts.get("cohorts"), "cohorts.cohorts")
    return {
        ensure_str(row.get("cohort_id"), f"cohorts.cohorts[{idx}].cohort_id")
        for idx, row in enumerate(rows)
    }


def validate_unlock_map(
    repo_root: Path,
    unlock_map: dict[str, Any],
    register_surface_ids: set[str],
    register_held_row_ids: set[str],
    cohort_ids: set[str],
    findings: list[Finding],
) -> None:
    schema_version = ensure_int(unlock_map.get("schema_version"), "unlock_map.schema_version")
    if schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="unlock_map.schema_version.unsupported",
                message=f"unlock_map.schema_version must be 1, got {schema_version}",
                remediation="Update the validator in the same change that bumps the artifact schema.",
            )
        )
    parse_iso_date(ensure_str(unlock_map.get("as_of"), "unlock_map.as_of"), "unlock_map.as_of")
    ensure_str(unlock_map.get("owner"), "unlock_map.owner")
    ensure_str(unlock_map.get("unlock_map_id"), "unlock_map.unlock_map_id")
    if ensure_str(unlock_map.get("scope_state"), "unlock_map.scope_state") != "frozen":
        findings.append(
            Finding(
                severity="error",
                check_id="unlock_map.scope_state.not_frozen",
                message="unlock_map.scope_state must be frozen",
                remediation="Set scope_state to frozen once the cutline lane is locked.",
            )
        )
    validate_path_refs(
        repo_root,
        [
            unlock_map.get("human_entrypoint_ref"),
            unlock_map.get("cutline_packet_ref"),
            unlock_map.get("handoff_checklist_ref"),
            unlock_map.get("claimed_surface_register_ref"),
            unlock_map.get("cohort_guardrails_ref"),
            unlock_map.get("dependency_graph_ref"),
        ],
        "unlock_map.primary_refs",
        findings,
    )
    validate_path_refs(
        repo_root,
        ensure_list(unlock_map.get("source_contract_refs"), "unlock_map.source_contract_refs"),
        "unlock_map.source_contract_refs",
        findings,
    )

    cutline_rows = ensure_list(unlock_map.get("cutline_rows"), "unlock_map.cutline_rows")
    cutline_ids: set[str] = set()
    cutline_to_unlocks: dict[str, set[str]] = {}
    descopable_flags: dict[str, bool] = {}
    for idx, raw_row in enumerate(cutline_rows):
        row = ensure_dict(raw_row, f"unlock_map.cutline_rows[{idx}]")
        cutline_id = ensure_str(row.get("cutline_id"), f"unlock_map.cutline_rows[{idx}].cutline_id")
        if cutline_id in cutline_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="unlock_map.cutline_rows.duplicate",
                    message=f"duplicate cutline_id: {cutline_id}",
                    remediation="Use one row per cutline id.",
                    ref=cutline_id,
                )
            )
        cutline_ids.add(cutline_id)
        ensure_str(row.get("title"), f"unlock_map.cutline_rows[{idx}].title")
        primary_surfaces = ensure_list(
            row.get("primary_surface_refs"),
            f"unlock_map.cutline_rows[{idx}].primary_surface_refs",
        )
        if not primary_surfaces:
            findings.append(
                Finding(
                    severity="error",
                    check_id="unlock_map.cutline_rows.primary_surface_refs.empty",
                    message=f"{cutline_id} must name at least one primary_surface_ref",
                    remediation="Bind the cutline row to at least one beta_surface: id from the register.",
                    ref=cutline_id,
                )
            )
        for surface_ref in primary_surfaces:
            if surface_ref not in register_surface_ids:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="unlock_map.cutline_rows.primary_surface_refs.unknown",
                        message=f"{cutline_id} cites unknown surface_id: {surface_ref}",
                        remediation="Use a beta_surface: id from the claimed_surface_register, or add the surface there first.",
                        ref=cutline_id,
                    )
                )
        primary_cohorts = ensure_list(
            row.get("primary_cohort_refs"),
            f"unlock_map.cutline_rows[{idx}].primary_cohort_refs",
        )
        if not primary_cohorts:
            findings.append(
                Finding(
                    severity="error",
                    check_id="unlock_map.cutline_rows.primary_cohort_refs.empty",
                    message=f"{cutline_id} must name at least one primary_cohort_ref",
                    remediation="Bind the cutline row to at least one cohort_id from cohort_guardrails.yaml.",
                    ref=cutline_id,
                )
            )
        for cohort_ref in primary_cohorts:
            if cohort_ref not in cohort_ids:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="unlock_map.cutline_rows.primary_cohort_refs.unknown",
                        message=f"{cutline_id} cites unknown cohort_id: {cohort_ref}",
                        remediation="Add the cohort to cohort_guardrails.yaml or correct the cutline binding.",
                        ref=cutline_id,
                    )
                )
        evidence_classes = ensure_list(
            row.get("evidence_classes"),
            f"unlock_map.cutline_rows[{idx}].evidence_classes",
        )
        if not evidence_classes:
            findings.append(
                Finding(
                    severity="error",
                    check_id="unlock_map.cutline_rows.evidence_classes.empty",
                    message=f"{cutline_id} must name at least one evidence class",
                    remediation="List the evidence classes the cutline row inherits from the beta surface.",
                    ref=cutline_id,
                )
            )
        validate_path_refs(
            repo_root,
            ensure_list(
                row.get("propagation_refs"),
                f"unlock_map.cutline_rows[{idx}].propagation_refs",
            ),
            "unlock_map.cutline_rows.propagation_refs",
            findings,
        )
        unlocks = ensure_list(
            row.get("unlocks"),
            f"unlock_map.cutline_rows[{idx}].unlocks",
        )
        if not unlocks:
            findings.append(
                Finding(
                    severity="error",
                    check_id="unlock_map.cutline_rows.unlocks.empty",
                    message=f"{cutline_id} must name at least one downstream unlock",
                    remediation="Tie the cutline row to at least one m4_unlock id.",
                    ref=cutline_id,
                )
            )
        cutline_to_unlocks[cutline_id] = set(unlocks)
        descopable = row.get("descopable")
        if not isinstance(descopable, bool):
            findings.append(
                Finding(
                    severity="error",
                    check_id="unlock_map.cutline_rows.descopable.not_bool",
                    message=f"{cutline_id} must declare descopable as a boolean",
                    remediation="Set descopable: false for non-descopable M3 truths.",
                    ref=cutline_id,
                )
            )
        else:
            descopable_flags[cutline_id] = descopable

    missing_required_cutlines = REQUIRED_CUTLINE_IDS - cutline_ids
    if missing_required_cutlines:
        findings.append(
            Finding(
                severity="error",
                check_id="unlock_map.cutline_rows.missing_required",
                message="unlock_map is missing required M3 cutline rows",
                remediation="Add rows for rollback, policy explainability, extension isolation, migration honesty, supportability, debug/test/task truth, and certified-archetype publication.",
                details={"missing": sorted(missing_required_cutlines)},
            )
        )
    descopable_required = [
        cutline_id
        for cutline_id in REQUIRED_CUTLINE_IDS
        if descopable_flags.get(cutline_id, False)
    ]
    if descopable_required:
        findings.append(
            Finding(
                severity="error",
                check_id="unlock_map.cutline_rows.required_must_be_non_descopable",
                message="required cutline rows must declare descopable: false",
                remediation="Flip descopable to false for the required M3 truths.",
                details={"violations": sorted(descopable_required)},
            )
        )

    unlock_rows = ensure_list(
        unlock_map.get("required_downstream_unlocks"),
        "unlock_map.required_downstream_unlocks",
    )
    unlock_ids: set[str] = set()
    for idx, raw_row in enumerate(unlock_rows):
        row = ensure_dict(raw_row, f"unlock_map.required_downstream_unlocks[{idx}]")
        unlock_id = ensure_str(
            row.get("unlock_id"),
            f"unlock_map.required_downstream_unlocks[{idx}].unlock_id",
        )
        if unlock_id in unlock_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="unlock_map.required_downstream_unlocks.duplicate",
                    message=f"duplicate unlock_id: {unlock_id}",
                    remediation="Use one row per m4_unlock id.",
                    ref=unlock_id,
                )
            )
        unlock_ids.add(unlock_id)
        ensure_str(row.get("title"), f"unlock_map.required_downstream_unlocks[{idx}].title")
        ensure_str(row.get("description"), f"unlock_map.required_downstream_unlocks[{idx}].description")
        ensure_str(row.get("current_state"), f"unlock_map.required_downstream_unlocks[{idx}].current_state")
        validate_path_refs(
            repo_root,
            ensure_list(
                row.get("downstream_consumer_refs"),
                f"unlock_map.required_downstream_unlocks[{idx}].downstream_consumer_refs",
            ),
            "unlock_map.required_downstream_unlocks.downstream_consumer_refs",
            findings,
        )
        required_cutlines = ensure_list(
            row.get("required_cutline_refs"),
            f"unlock_map.required_downstream_unlocks[{idx}].required_cutline_refs",
        )
        if not required_cutlines:
            findings.append(
                Finding(
                    severity="error",
                    check_id="unlock_map.required_downstream_unlocks.required_cutline_refs.empty",
                    message=f"{unlock_id} must cite at least one cutline row",
                    remediation="Tie the downstream unlock to one or more m3_cutline ids.",
                    ref=unlock_id,
                )
            )
        for cutline_ref in required_cutlines:
            if cutline_ref not in cutline_ids:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="unlock_map.required_downstream_unlocks.required_cutline_refs.unknown",
                        message=f"{unlock_id} cites unknown cutline row: {cutline_ref}",
                        remediation="Add the cutline row first, or correct the reference.",
                        ref=unlock_id,
                    )
                )
        blocking_surfaces = ensure_list(
            row.get("blocking_surfaces"),
            f"unlock_map.required_downstream_unlocks[{idx}].blocking_surfaces",
        )
        for surface_ref in blocking_surfaces:
            if surface_ref not in register_surface_ids:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="unlock_map.required_downstream_unlocks.blocking_surfaces.unknown",
                        message=f"{unlock_id} cites unknown blocking surface: {surface_ref}",
                        remediation="Use a beta_surface: id from claimed_surface_register, or add it first.",
                        ref=unlock_id,
                    )
                )

    missing_required_unlocks = REQUIRED_M4_UNLOCKS - unlock_ids
    if missing_required_unlocks:
        findings.append(
            Finding(
                severity="error",
                check_id="unlock_map.required_downstream_unlocks.missing_required",
                message="unlock_map is missing required M4 unlocks",
                remediation="Add stable_api_freeze, release_center_rehearsal, and certified_archetype_publication unlocks.",
                details={"missing": sorted(missing_required_unlocks)},
            )
        )

    unreferenced_cutline_unlocks = {
        unlock_ref
        for refs in cutline_to_unlocks.values()
        for unlock_ref in refs
    } - unlock_ids
    if unreferenced_cutline_unlocks:
        findings.append(
            Finding(
                severity="error",
                check_id="unlock_map.cutline_rows.unknown_unlock",
                message="cutline rows cite unknown unlock ids",
                remediation="Add the missing unlock ids to required_downstream_unlocks or correct the cutline binding.",
                details={"unknown": sorted(unreferenced_cutline_unlocks)},
            )
        )
    unmapped_unlocks = unlock_ids - {
        unlock_ref
        for refs in cutline_to_unlocks.values()
        for unlock_ref in refs
    }
    if unmapped_unlocks:
        findings.append(
            Finding(
                severity="error",
                check_id="unlock_map.required_downstream_unlocks.not_blocked",
                message="downstream unlocks are not blocked by any cutline row",
                remediation="Either remove the unlock or wire at least one cutline row's unlocks list to it.",
                details={"unmapped": sorted(unmapped_unlocks)},
            )
        )

    beta_only_rows = ensure_list(unlock_map.get("beta_only_surfaces"), "unlock_map.beta_only_surfaces")
    if not beta_only_rows:
        findings.append(
            Finding(
                severity="error",
                check_id="unlock_map.beta_only_surfaces.empty",
                message="unlock_map must declare beta-only surfaces",
                remediation="Mirror the held / out-of-scope rows from the claimed_surface_register so stable planning sees them.",
            )
        )
    required_widening = {
        "release_council_review",
        "claimed_surface_register_row",
        "cohort_guardrails_row",
        "known_limit_note",
    }
    for idx, raw_row in enumerate(beta_only_rows):
        row = ensure_dict(raw_row, f"unlock_map.beta_only_surfaces[{idx}]")
        ensure_str(row.get("row_id"), f"unlock_map.beta_only_surfaces[{idx}].row_id")
        mirror_id = ensure_str(
            row.get("mirrors_register_row"),
            f"unlock_map.beta_only_surfaces[{idx}].mirrors_register_row",
        )
        if mirror_id not in register_held_row_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="unlock_map.beta_only_surfaces.mirror_unknown",
                    message=f"beta-only row {row.get('row_id')} mirrors unknown register row: {mirror_id}",
                    remediation="Use a row_id from claimed_surface_register.held_or_out_of_scope_surfaces.",
                    ref=str(row.get("row_id")),
                )
            )
        widening = set(ensure_list(
            row.get("widening_requires"),
            f"unlock_map.beta_only_surfaces[{idx}].widening_requires",
        ))
        missing = required_widening - widening
        if missing:
            findings.append(
                Finding(
                    severity="error",
                    check_id="unlock_map.beta_only_surfaces.widening_requires",
                    message=f"beta-only row missing widening requirements: {row.get('row_id')}",
                    remediation="Require release_council_review, claimed_surface_register_row, cohort_guardrails_row, and known_limit_note.",
                    ref=str(row.get("row_id")),
                    details={"missing": sorted(missing)},
                )
            )

    change_control = ensure_dict(unlock_map.get("change_control"), "unlock_map.change_control")
    addition_refs = set(ensure_list(
        change_control.get("addition_requires_refs"),
        "unlock_map.change_control.addition_requires_refs",
    ))
    required_addition_refs = {
        "docs/milestones/m3/cutline_packet.md",
        "artifacts/milestones/m3/unlock_map.yaml",
        "artifacts/milestones/m3/stable_planning_handoff_checklist.md",
        "artifacts/milestones/m3/claimed_surface_register.json",
        "artifacts/milestones/m3/cohort_guardrails.yaml",
    }
    missing_addition_refs = required_addition_refs - addition_refs
    if missing_addition_refs:
        findings.append(
            Finding(
                severity="error",
                check_id="unlock_map.change_control.missing_addition_refs",
                message="unlock_map.change_control.addition_requires_refs is missing required artifacts",
                remediation="Add the cutline packet, unlock map, handoff checklist, claimed-surface register, and cohort guardrails refs.",
                details={"missing": sorted(missing_addition_refs)},
            )
        )


def validate_cutline_doc(doc_path: Path, findings: list[Finding]) -> None:
    if not doc_path.exists():
        findings.append(
            Finding(
                severity="error",
                check_id="cutline_doc.missing",
                message=f"cutline packet doc is missing: {doc_path}",
                remediation="Author docs/milestones/m3/cutline_packet.md before running the validator.",
            )
        )
        return
    text = doc_path.read_text(encoding="utf-8")
    missing_phrases = [phrase for phrase in REQUIRED_CUTLINE_DOC_PHRASES if phrase not in text]
    if missing_phrases:
        findings.append(
            Finding(
                severity="error",
                check_id="cutline_doc.missing_phrases",
                message="cutline packet doc is missing required cross-references or sections",
                remediation="Link the unlock map, handoff checklist, validator command, and definition-of-green / update-rules sections.",
                details={"missing": missing_phrases},
            )
        )
    for cutline_id in REQUIRED_CUTLINE_IDS:
        if cutline_id not in text:
            findings.append(
                Finding(
                    severity="error",
                    check_id="cutline_doc.missing_cutline_id",
                    message=f"cutline packet doc is missing required cutline id: {cutline_id}",
                    remediation="Name the cutline id in the non-descopable truths table so reviewers can cite it.",
                    ref=cutline_id,
                )
            )


def validate_handoff(handoff_path: Path, findings: list[Finding]) -> None:
    if not handoff_path.exists():
        findings.append(
            Finding(
                severity="error",
                check_id="handoff.missing",
                message=f"handoff checklist is missing: {handoff_path}",
                remediation="Author artifacts/milestones/m3/stable_planning_handoff_checklist.md before running the validator.",
            )
        )
        return
    text = handoff_path.read_text(encoding="utf-8")
    missing_phrases = [phrase for phrase in REQUIRED_HANDOFF_PHRASES if phrase not in text]
    if missing_phrases:
        findings.append(
            Finding(
                severity="error",
                check_id="handoff.missing_phrases",
                message="handoff checklist is missing required cross-references or sections",
                remediation="Cite the cutline packet, unlock map, register, guardrails, validator, and section headers.",
                details={"missing": missing_phrases},
            )
        )
    for cutline_id in REQUIRED_CUTLINE_IDS:
        if cutline_id not in text:
            findings.append(
                Finding(
                    severity="error",
                    check_id="handoff.missing_cutline_id",
                    message=f"handoff checklist is missing required cutline id: {cutline_id}",
                    remediation="Add a checklist line that cites the cutline id by name.",
                    ref=cutline_id,
                )
            )
    for unlock_id in REQUIRED_M4_UNLOCKS:
        if unlock_id not in text:
            findings.append(
                Finding(
                    severity="error",
                    check_id="handoff.missing_unlock_id",
                    message=f"handoff checklist is missing required unlock id: {unlock_id}",
                    remediation="Add a checklist line that cites the unlock id by name.",
                    ref=unlock_id,
                )
            )


def write_report(path: Path, findings: list[Finding]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": 1,
        "status": "pass" if not any(item.severity == "error" for item in findings) else "fail",
        "generated_at": dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
        "summary": {
            "errors": sum(1 for item in findings if item.severity == "error"),
            "warnings": sum(1 for item in findings if item.severity == "warning"),
        },
        "findings": [item.as_report() for item in findings],
    }
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    register = ensure_dict(load_json(repo_root / args.register), "register")
    cohorts = ensure_dict(render_yaml_as_json(repo_root / args.cohorts), "cohorts")
    unlock_map = ensure_dict(render_yaml_as_json(repo_root / args.unlock_map), "unlock_map")

    register_surface_ids = collect_register_surface_ids(register)
    register_held_row_ids = collect_register_held_row_ids(register)
    cohort_ids = collect_cohort_ids(cohorts)

    findings: list[Finding] = []
    validate_unlock_map(
        repo_root,
        unlock_map,
        register_surface_ids,
        register_held_row_ids,
        cohort_ids,
        findings,
    )
    validate_cutline_doc(repo_root / args.cutline_doc, findings)
    validate_handoff(repo_root / args.handoff, findings)

    if args.report:
        write_report(repo_root / args.report, findings)

    errors = [item for item in findings if item.severity == "error"]
    if errors:
        for item in errors:
            ref = f" ({item.ref})" if item.ref else ""
            print(f"ERROR [{item.check_id}]{ref}: {item.message}", file=sys.stderr)
            print(f"  remediation: {item.remediation}", file=sys.stderr)
        return 1

    warnings = [item for item in findings if item.severity == "warning"]
    for item in warnings:
        ref = f" ({item.ref})" if item.ref else ""
        print(f"WARNING [{item.check_id}]{ref}: {item.message}", file=sys.stderr)

    print("m3 cutline packet validated")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
