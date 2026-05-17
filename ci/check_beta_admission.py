#!/usr/bin/env python3
"""Validate the M3 beta admission matrix, claimed-surface register,
cohort guardrails, and dependency graph.

This is the first consumer for the four checked-in artifacts:
  - docs/milestones/m3/beta_admission_matrix.md
  - artifacts/milestones/m3/claimed_surface_register.json
  - artifacts/milestones/m3/cohort_guardrails.yaml
  - artifacts/milestones/m3/dependency_graph.mmd
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


DEFAULT_REGISTER_REL = "artifacts/milestones/m3/claimed_surface_register.json"
DEFAULT_COHORTS_REL = "artifacts/milestones/m3/cohort_guardrails.yaml"
DEFAULT_GRAPH_REL = "artifacts/milestones/m3/dependency_graph.mmd"
DEFAULT_DOC_REL = "docs/milestones/m3/beta_admission_matrix.md"

REQUIRED_SURFACES = {
    "beta_surface:extension_runtime",
    "beta_surface:debug_test_task_model",
    "beta_surface:packaging_update_rollback",
    "beta_surface:policy_proxy_transport",
    "beta_surface:support_export_diagnostics",
    "beta_surface:importer_and_migration",
    "beta_surface:compatibility_publication",
}

REQUIRED_M3_COHORTS = {
    "cohort:extension_author",
    "cohort:design_partner_managed_pilot",
    "cohort:certified_archetype",
}

REQUIRED_BETA_ARCHETYPE_ROWS = {
    "archetype_row:ts_web_app_or_service",
    "archetype_row:python_service_or_data_app",
    "archetype_row:java_or_kotlin_service",
    "archetype_row:rust_workspace",
    "archetype_row:go_service_or_monorepo_slice",
    "archetype_row:c_or_cpp_native_project",
}

REQUIRED_GRAPH_NODES = {
    "admission_doc",
    "surface_register",
    "cohort_guardrails",
    "beta_admission_validator",
    "validation_capture",
    "m2_matrix",
    "reference_workspace_rows",
    "deployment_locality",
    "known_limits_contract",
    "certified_archetype_template",
}

PATH_LIKE_SUFFIXES = (".yaml", ".yml", ".json", ".md", ".toml", ".rs", ".py", ".mmd")
ID_PREFIXES = (
    "archetype_row:",
    "archetype_certification_seed:",
    "beta_archetype:",
    "beta_surface:",
    "cohort:",
    "compat_row:",
    "fixture_register:",
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
    parser.add_argument("--register", default=DEFAULT_REGISTER_REL)
    parser.add_argument("--cohorts", default=DEFAULT_COHORTS_REL)
    parser.add_argument("--graph", default=DEFAULT_GRAPH_REL)
    parser.add_argument("--doc", default=DEFAULT_DOC_REL)
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
                    remediation="Fix the path or seed the referenced artifact so the beta admission lane stays inspectable.",
                    ref=ref,
                )
            )


def validate_content_integrity_beta_gate(
    repo_root: Path,
    gate: dict[str, Any],
    claimed_surfaces: list[Any],
    findings: list[Finding],
) -> None:
    start_finding_count = len(findings)
    packet_ref = ensure_str(gate.get("packet_ref"), "register.content_integrity_beta.packet_ref")
    validate_path_refs(
        repo_root,
        [
            packet_ref,
            gate.get("doc_ref"),
            gate.get("fixture_dir_ref"),
        ],
        "register.content_integrity_beta.refs",
        findings,
    )

    required_status = ensure_str(
        gate.get("required_status"),
        "register.content_integrity_beta.required_status",
    )
    if required_status != "green":
        findings.append(
            Finding(
                severity="error",
                check_id="register.content_integrity_beta.required_status",
                message="content_integrity_beta.required_status must be green",
                remediation="Keep the gate fail-closed by requiring green content-integrity validation.",
                ref=packet_ref,
            )
        )

    packet_path = repo_root / strip_fragment(packet_ref)
    try:
        packet = ensure_dict(load_json(packet_path), "content_integrity_beta.packet")
    except SystemExit as exc:
        findings.append(
            Finding(
                severity="error",
                check_id="register.content_integrity_beta.packet_missing",
                message=str(exc),
                remediation="Regenerate the content-integrity beta packet and commit it with the register update.",
                ref=packet_ref,
            )
        )
        return

    if ensure_str(packet.get("record_kind"), "content_integrity_beta.packet.record_kind") != "content_integrity_beta_packet":
        findings.append(
            Finding(
                severity="error",
                check_id="register.content_integrity_beta.packet_record_kind",
                message="content-integrity beta packet has the wrong record kind",
                remediation="Regenerate the packet with the content_integrity_beta CLI.",
                ref=packet_ref,
            )
        )
    if ensure_int(packet.get("schema_version"), "content_integrity_beta.packet.schema_version") != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="register.content_integrity_beta.packet_schema_version",
                message="content-integrity beta packet schema_version must be 1",
                remediation="Update the validator in the same change that bumps the packet schema.",
                ref=packet_ref,
            )
        )
    if packet.get("normalization_applied") is not False:
        findings.append(
            Finding(
                severity="error",
                check_id="register.content_integrity_beta.normalization_applied",
                message="content-integrity beta packet normalized or stripped source bytes",
                remediation="Regenerate the packet from the shared detector without normalizing raw content.",
                ref=packet_ref,
            )
        )

    required_surface_tokens = {
        ensure_str(token, "register.content_integrity_beta.required_surface_tokens[]")
        for token in ensure_list(
            gate.get("required_surface_tokens"),
            "register.content_integrity_beta.required_surface_tokens",
        )
    }
    required_warning_tokens = {
        ensure_str(token, "register.content_integrity_beta.required_warning_class_tokens[]")
        for token in ensure_list(
            gate.get("required_warning_class_tokens"),
            "register.content_integrity_beta.required_warning_class_tokens",
        )
    }
    required_representation_tokens = {
        ensure_str(token, "register.content_integrity_beta.required_representation_tokens[]")
        for token in ensure_list(
            gate.get("required_representation_tokens"),
            "register.content_integrity_beta.required_representation_tokens",
        )
    }
    claim_states_requiring_gate = {
        ensure_str(token, "register.content_integrity_beta.required_for_claim_states[]")
        for token in ensure_list(
            gate.get("required_for_claim_states"),
            "register.content_integrity_beta.required_for_claim_states",
        )
    }

    packet_warning_tokens = set(
        ensure_str(token, "content_integrity_beta.packet.finding_class_tokens[]")
        for token in ensure_list(
            packet.get("finding_class_tokens"),
            "content_integrity_beta.packet.finding_class_tokens",
        )
    )
    if not required_warning_tokens <= packet_warning_tokens:
        findings.append(
            Finding(
                severity="error",
                check_id="register.content_integrity_beta.warning_classes_missing",
                message="content-integrity beta packet is missing required suspicious-content classes",
                remediation="Refresh the packet from the shared detector fixture so all required warning classes are present.",
                ref=packet_ref,
                details={"missing": sorted(required_warning_tokens - packet_warning_tokens)},
            )
        )

    rows = ensure_list(packet.get("surfaces"), "content_integrity_beta.packet.surfaces")
    surface_tokens: set[str] = set()
    representation_tokens: set[str] = set()
    claimed_surface_ids = {
        ensure_str(
            ensure_dict(row, "register.claimed_surfaces[]").get("surface_id"),
            "register.claimed_surfaces[].surface_id",
        )
        for row in claimed_surfaces
    }
    for idx, raw_row in enumerate(rows):
        row = ensure_dict(raw_row, f"content_integrity_beta.packet.surfaces[{idx}]")
        surface_token = ensure_str(row.get("surface_token"), f"content_integrity_beta.packet.surfaces[{idx}].surface_token")
        surface_tokens.add(surface_token)
        declared_ref = ensure_str(
            row.get("declared_beta_surface_ref"),
            f"content_integrity_beta.packet.surfaces[{idx}].declared_beta_surface_ref",
        )
        if declared_ref not in claimed_surface_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="register.content_integrity_beta.declared_surface_unknown",
                    message=f"content-integrity packet cites unknown claimed surface {declared_ref}",
                    remediation="Bind packet rows only to ids in claimed_surfaces.",
                    ref=declared_ref,
                )
            )
        row_warning_tokens = set(
            ensure_str(token, f"content_integrity_beta.packet.surfaces[{idx}].warning_class_tokens[]")
            for token in ensure_list(
                row.get("warning_class_tokens"),
                f"content_integrity_beta.packet.surfaces[{idx}].warning_class_tokens",
            )
        )
        if row_warning_tokens != packet_warning_tokens:
            findings.append(
                Finding(
                    severity="error",
                    check_id="register.content_integrity_beta.surface_warning_class_drift",
                    message=f"surface {surface_token} warning classes drift from packet classes",
                    remediation="Regenerate the packet so every surface reads the same detector result.",
                    ref=packet_ref,
                    details={"surface": surface_token},
                )
            )
        warnings = ensure_list(
            row.get("content_integrity_warnings"),
            f"content_integrity_beta.packet.surfaces[{idx}].content_integrity_warnings",
        )
        if not warnings:
            findings.append(
                Finding(
                    severity="error",
                    check_id="register.content_integrity_beta.surface_warnings_missing",
                    message=f"surface {surface_token} carries no content-integrity warnings",
                    remediation="Regenerate the packet so each surface carries shared warning records.",
                    ref=packet_ref,
                )
            )
        for warning in warnings:
            warning_row = ensure_dict(warning, f"content_integrity_beta.packet.surfaces[{idx}].content_integrity_warnings[]")
            if ensure_str(warning_row.get("record_kind"), "content_integrity_warning.record_kind") != "content_integrity_warning_record":
                findings.append(
                    Finding(
                        severity="error",
                        check_id="register.content_integrity_beta.warning_record_kind",
                        message=f"surface {surface_token} carries a non-shared warning record",
                        remediation="Use content_integrity_warning_record rows emitted by aureline-content-safety.",
                        ref=packet_ref,
                    )
                )
                break
        controls = ensure_dict(
            row.get("operator_truth"),
            f"content_integrity_beta.packet.surfaces[{idx}].operator_truth",
        )
        for key in (
            "trust_class_badge_visible",
            "raw_rendered_state_visible",
            "copy_export_representation_labels_visible",
            "review_flow_preserves_warning_refs",
            "support_export_preserves_warning_refs",
        ):
            if controls.get(key) is not True:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="register.content_integrity_beta.operator_truth_not_green",
                        message=f"surface {surface_token} has operator-truth control {key} disabled",
                        remediation="Block the beta claim or restore the missing content-integrity truth control.",
                        ref=packet_ref,
                    )
                )
        choices = ensure_list(
            row.get("representation_choices"),
            f"content_integrity_beta.packet.surfaces[{idx}].representation_choices",
        )
        if sum(1 for choice in choices if ensure_dict(choice, "representation_choice").get("default_for_surface") is True) != 1:
            findings.append(
                Finding(
                    severity="error",
                    check_id="register.content_integrity_beta.default_representation_invalid",
                    message=f"surface {surface_token} must declare exactly one default representation action",
                    remediation="Refresh representation choices so the active copy/export default is unambiguous.",
                    ref=packet_ref,
                )
            )
        for choice in choices:
            choice_row = ensure_dict(choice, f"content_integrity_beta.packet.surfaces[{idx}].representation_choices[]")
            representation = ensure_str(
                choice_row.get("representation_class"),
                f"content_integrity_beta.packet.surfaces[{idx}].representation_choices[].representation_class",
            )
            representation_tokens.add(representation)
            if not ensure_str(
                choice_row.get("visible_label"),
                f"content_integrity_beta.packet.surfaces[{idx}].representation_choices[].visible_label",
            ):
                findings.append(
                    Finding(
                        severity="error",
                        check_id="register.content_integrity_beta.representation_label_missing",
                        message=f"surface {surface_token} has a representation choice without a label",
                        remediation="Name every copy/export representation explicitly.",
                        ref=packet_ref,
                    )
                )
            if choice_row.get("review_flow_visible") is not True:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="register.content_integrity_beta.representation_not_review_visible",
                        message=f"surface {surface_token} representation {representation} is not visible in review flow",
                        remediation="Keep raw/rendered/sanitized/redacted choices visible through review handoff.",
                        ref=packet_ref,
                    )
                )
            if representation == "redacted" and choice_row.get("redaction_applied") is not True:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="register.content_integrity_beta.redacted_without_redaction",
                        message=f"surface {surface_token} declares redacted output without applying redaction",
                        remediation="Apply redaction or remove the redacted representation claim.",
                        ref=packet_ref,
                    )
                )
            if representation in {"sanitized", "redacted"} and choice_row.get("support_export_safe") is not True:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="register.content_integrity_beta.support_export_not_safe",
                        message=f"surface {surface_token} representation {representation} is not support-export safe",
                        remediation="Mark only support-safe sanitized/redacted representations as exportable.",
                        ref=packet_ref,
                    )
                )

    if not required_surface_tokens <= surface_tokens:
        findings.append(
            Finding(
                severity="error",
                check_id="register.content_integrity_beta.surface_tokens_missing",
                message="content-integrity beta packet is missing required surfaces",
                remediation="Regenerate the packet with all declared beta content-integrity surfaces.",
                ref=packet_ref,
                details={"missing": sorted(required_surface_tokens - surface_tokens)},
            )
        )
    if not required_representation_tokens <= representation_tokens:
        findings.append(
            Finding(
                severity="error",
                check_id="register.content_integrity_beta.representation_tokens_missing",
                message="content-integrity beta packet is missing required representation labels",
                remediation="Expose raw, rendered, sanitized, and redacted labels before claiming beta.",
                ref=packet_ref,
                details={"missing": sorted(required_representation_tokens - representation_tokens)},
            )
        )

    has_required_claim = any(
        ensure_str(
            ensure_dict(row, "register.claimed_surfaces[]").get("claim_state"),
            "register.claimed_surfaces[].claim_state",
        )
        in claim_states_requiring_gate
        for row in claimed_surfaces
    )
    if has_required_claim and len(findings) > start_finding_count:
        findings.append(
            Finding(
                severity="error",
                check_id="register.content_integrity_beta.blocks_beta_claim",
                message="beta claimed surfaces are blocked because the content-integrity packet is not green",
                remediation="Fix the packet or narrow the affected beta claims before admission.",
                ref=packet_ref,
            )
        )


def validate_header(
    payload: dict[str, Any],
    label: str,
    findings: list[Finding],
) -> None:
    schema_version = ensure_int(payload.get("schema_version"), f"{label}.schema_version")
    if schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.schema_version.unsupported",
                message=f"{label}.schema_version must be 1, got {schema_version}",
                remediation="Update the validator in the same change that bumps the artifact schema.",
            )
        )
    parse_iso_date(ensure_str(payload.get("as_of"), f"{label}.as_of"), f"{label}.as_of")
    ensure_str(payload.get("owner"), f"{label}.owner")


def validate_register(
    repo_root: Path,
    register: dict[str, Any],
    cohort_ids: set[str],
    findings: list[Finding],
) -> None:
    validate_header(register, "register", findings)
    ensure_str(register.get("register_id"), "register.register_id")
    if ensure_str(register.get("scope_state"), "register.scope_state") != "frozen":
        findings.append(
            Finding(
                severity="error",
                check_id="register.scope_state.not_frozen",
                message="register.scope_state must be frozen",
                remediation="Set scope_state to frozen once the beta claim surface is locked.",
            )
        )

    upstream_refs = ensure_dict(
        register.get("upstream_canonical_refs"),
        "register.upstream_canonical_refs",
    )
    required_upstream = {
        "alpha_wedge_matrix",
        "alpha_exit_gate_scoreboard",
        "alpha_dependency_graph",
        "alpha_archetype_seed_rows",
        "reference_workspace_rows",
        "archetype_rubric",
        "deployment_locality_matrix",
        "known_limits_contract",
    }
    missing_upstream = required_upstream - set(upstream_refs)
    if missing_upstream:
        findings.append(
            Finding(
                severity="error",
                check_id="register.upstream_canonical_refs.missing",
                message="register.upstream_canonical_refs is missing required upstream contracts",
                remediation="Add the missing upstream references so the beta scope inherits one canonical source.",
                details={"missing": sorted(missing_upstream)},
            )
        )
    validate_path_refs(
        repo_root,
        list(upstream_refs.values()),
        "register.upstream_canonical_refs",
        findings,
    )
    validate_path_refs(
        repo_root,
        [
            register.get("human_entrypoint_ref"),
            register.get("cohort_guardrails_ref"),
            register.get("dependency_graph_ref"),
        ],
        "register.primary_refs",
        findings,
    )
    content_integrity_beta = ensure_dict(
        register.get("content_integrity_beta"),
        "register.content_integrity_beta",
    )

    change_control = ensure_dict(register.get("change_control"), "register.change_control")
    addition_refs = set(ensure_list(change_control.get("addition_requires_refs"), "register.change_control.addition_requires_refs"))
    required_addition_refs = {
        "artifacts/milestones/m3/claimed_surface_register.json",
        "artifacts/milestones/m3/cohort_guardrails.yaml",
        "artifacts/milestones/m3/dependency_graph.mmd",
        "docs/milestones/m3/beta_admission_matrix.md",
    }
    missing_addition_refs = required_addition_refs - addition_refs
    if missing_addition_refs:
        findings.append(
            Finding(
                severity="error",
                check_id="register.change_control.missing_addition_refs",
                message="change_control.addition_requires_refs is missing required beta admission artifacts",
                remediation="Add the register, guardrails, dependency graph, and admission doc refs.",
                details={"missing": sorted(missing_addition_refs)},
            )
        )

    claimed_surfaces = ensure_list(register.get("claimed_surfaces"), "register.claimed_surfaces")
    surface_ids: set[str] = set()
    for idx, raw_row in enumerate(claimed_surfaces):
        row = ensure_dict(raw_row, f"register.claimed_surfaces[{idx}]")
        surface_id = ensure_str(row.get("surface_id"), f"register.claimed_surfaces[{idx}].surface_id")
        if surface_id in surface_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="register.claimed_surfaces.duplicate",
                    message=f"duplicate surface_id: {surface_id}",
                    remediation="Use one row per claimed beta surface id.",
                    ref=surface_id,
                )
            )
        surface_ids.add(surface_id)
        ensure_str(row.get("title"), f"register.claimed_surfaces[{idx}].title")
        ensure_str(row.get("lifecycle_label"), f"register.claimed_surfaces[{idx}].lifecycle_label")
        ensure_str(row.get("claim_state"), f"register.claimed_surfaces[{idx}].claim_state")
        ensure_str(row.get("support_class"), f"register.claimed_surfaces[{idx}].support_class")

        primary_cohort_refs = ensure_list(row.get("primary_cohort_refs"), f"register.claimed_surfaces[{idx}].primary_cohort_refs")
        if not primary_cohort_refs:
            findings.append(
                Finding(
                    severity="error",
                    check_id="register.claimed_surfaces.primary_cohort_refs.empty",
                    message=f"{surface_id} must name at least one primary_cohort_ref",
                    remediation="Bind the claimed surface to at least one cohort_id from cohort_guardrails.yaml.",
                    ref=surface_id,
                )
            )
        for cohort_ref in primary_cohort_refs:
            if cohort_ref not in cohort_ids:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="register.claimed_surfaces.primary_cohort_refs.unknown",
                        message=f"{surface_id} cites unknown cohort_id: {cohort_ref}",
                        remediation="Add the cohort to cohort_guardrails.yaml or correct the surface binding.",
                        ref=surface_id,
                    )
                )

        downgrade_rules = ensure_list(row.get("downgrade_rules"), f"register.claimed_surfaces[{idx}].downgrade_rules")
        if not downgrade_rules:
            findings.append(
                Finding(
                    severity="error",
                    check_id="register.claimed_surfaces.downgrade_rules.empty",
                    message=f"{surface_id} must define at least one downgrade rule",
                    remediation="Add a downgrade_rules entry so beta claims cannot drift silently.",
                    ref=surface_id,
                )
            )
        for rule_idx, raw_rule in enumerate(downgrade_rules):
            rule = ensure_dict(raw_rule, f"register.claimed_surfaces[{idx}].downgrade_rules[{rule_idx}]")
            ensure_str(rule.get("trigger"), f"register.claimed_surfaces[{idx}].downgrade_rules[{rule_idx}].trigger")
            ensure_str(rule.get("downgrade_to"), f"register.claimed_surfaces[{idx}].downgrade_rules[{rule_idx}].downgrade_to")
            validate_path_refs(
                repo_root,
                ensure_list(rule.get("propagation_refs"), f"register.claimed_surfaces[{idx}].downgrade_rules[{rule_idx}].propagation_refs"),
                "register.claimed_surfaces.downgrade_rules.propagation_refs",
                findings,
            )

    missing_required_surfaces = REQUIRED_SURFACES - surface_ids
    if missing_required_surfaces:
        findings.append(
            Finding(
                severity="error",
                check_id="register.claimed_surfaces.missing_required",
                message="register is missing required M3 beta surface rows",
                remediation="Add rows covering extension runtime, debug/test/task, packaging, policy, support export, importer, and compatibility publication.",
                details={"missing": sorted(missing_required_surfaces)},
            )
        )
    validate_content_integrity_beta_gate(
        repo_root,
        content_integrity_beta,
        claimed_surfaces,
        findings,
    )

    archetype_rows = ensure_list(register.get("claimed_archetype_rows"), "register.claimed_archetype_rows")
    archetype_refs_used: set[str] = set()
    for idx, raw_row in enumerate(archetype_rows):
        row = ensure_dict(raw_row, f"register.claimed_archetype_rows[{idx}]")
        ensure_str(row.get("archetype_surface_id"), f"register.claimed_archetype_rows[{idx}].archetype_surface_id")
        ref = ensure_str(row.get("archetype_row_ref"), f"register.claimed_archetype_rows[{idx}].archetype_row_ref")
        archetype_refs_used.add(ref)
        if not ref.startswith("archetype_row:"):
            findings.append(
                Finding(
                    severity="error",
                    check_id="register.claimed_archetype_rows.invalid_ref",
                    message=f"archetype_row_ref must start with archetype_row:, got {ref}",
                    remediation="Use the canonical archetype_row id from artifacts/compat/reference_workspace_rows.yaml.",
                    ref=ref,
                )
            )
        ensure_list(row.get("minimum_platform_matrix"), f"register.claimed_archetype_rows[{idx}].minimum_platform_matrix")
        ensure_list(row.get("minimum_mode_matrix"), f"register.claimed_archetype_rows[{idx}].minimum_mode_matrix")
        ensure_list(row.get("required_cohort_refs"), f"register.claimed_archetype_rows[{idx}].required_cohort_refs")
        for cohort_ref in row.get("required_cohort_refs", []):
            if cohort_ref not in cohort_ids:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="register.claimed_archetype_rows.unknown_cohort",
                        message=f"archetype row cites unknown cohort_id: {cohort_ref}",
                        remediation="Add the cohort to cohort_guardrails.yaml or correct the archetype binding.",
                        ref=str(row.get("archetype_surface_id")),
                    )
                )

    missing_archetypes = REQUIRED_BETA_ARCHETYPE_ROWS - archetype_refs_used
    if missing_archetypes:
        findings.append(
            Finding(
                severity="error",
                check_id="register.claimed_archetype_rows.missing_required",
                message="register is missing required M3 beta archetype rows",
                remediation="Cover the M2 carryover (TS/JS, Python) plus the four M3 additions (Java/Kotlin, Rust, Go, C/C++).",
                details={"missing": sorted(missing_archetypes)},
            )
        )

    held_rows = ensure_list(register.get("held_or_out_of_scope_surfaces"), "register.held_or_out_of_scope_surfaces")
    if not held_rows:
        findings.append(
            Finding(
                severity="error",
                check_id="register.held_or_out_of_scope.empty",
                message="register must declare held or explicitly out-of-scope surfaces",
                remediation="Add held_or_out_of_scope_surfaces so beta scope cannot widen by omission.",
            )
        )
    required_widening = {
        "release_council_review",
        "claimed_surface_register_row",
        "cohort_guardrails_row",
        "known_limit_note",
    }
    for idx, raw_row in enumerate(held_rows):
        row = ensure_dict(raw_row, f"register.held_or_out_of_scope_surfaces[{idx}]")
        widening = set(ensure_list(row.get("widening_requires"), f"register.held_or_out_of_scope_surfaces[{idx}].widening_requires"))
        missing = required_widening - widening
        if missing:
            findings.append(
                Finding(
                    severity="error",
                    check_id="register.held_or_out_of_scope.widening_requires",
                    message=f"held/out-of-scope row missing widening requirements: {row.get('row_id')}",
                    remediation="Require release_council_review, claimed_surface_register_row, cohort_guardrails_row, and known_limit_note.",
                    ref=str(row.get("row_id")),
                    details={"missing": sorted(missing)},
                )
            )


def validate_cohorts(
    repo_root: Path,
    cohorts: dict[str, Any],
    findings: list[Finding],
) -> set[str]:
    validate_header(cohorts, "cohorts", findings)
    ensure_str(cohorts.get("guardrails_id"), "cohorts.guardrails_id")
    if ensure_str(cohorts.get("scope_state"), "cohorts.scope_state") != "frozen":
        findings.append(
            Finding(
                severity="error",
                check_id="cohorts.scope_state.not_frozen",
                message="cohorts.scope_state must be frozen",
                remediation="Set scope_state to frozen once the beta cohort vocabulary is locked.",
            )
        )
    validate_path_refs(
        repo_root,
        [
            cohorts.get("human_entrypoint_ref"),
            cohorts.get("claimed_surface_register_ref"),
            cohorts.get("dependency_graph_ref"),
        ],
        "cohorts.primary_refs",
        findings,
    )
    validate_path_refs(
        repo_root,
        ensure_list(cohorts.get("source_contract_refs"), "cohorts.source_contract_refs"),
        "cohorts.source_contract_refs",
        findings,
    )

    cohort_rows = ensure_list(cohorts.get("cohorts"), "cohorts.cohorts")
    cohort_ids: set[str] = set()
    for idx, raw_row in enumerate(cohort_rows):
        row = ensure_dict(raw_row, f"cohorts.cohorts[{idx}]")
        cohort_id = ensure_str(row.get("cohort_id"), f"cohorts.cohorts[{idx}].cohort_id")
        if cohort_id in cohort_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="cohorts.cohorts.duplicate",
                    message=f"duplicate cohort_id: {cohort_id}",
                    remediation="Use one row per beta cohort id.",
                    ref=cohort_id,
                )
            )
        cohort_ids.add(cohort_id)
        ensure_str(row.get("title"), f"cohorts.cohorts[{idx}].title")
        ensure_str(row.get("purpose"), f"cohorts.cohorts[{idx}].purpose")
        primary_surfaces = ensure_list(row.get("primary_surface_refs"), f"cohorts.cohorts[{idx}].primary_surface_refs")
        if not primary_surfaces:
            findings.append(
                Finding(
                    severity="error",
                    check_id="cohorts.cohorts.primary_surface_refs.empty",
                    message=f"{cohort_id} must name at least one primary surface",
                    remediation="Bind the cohort to at least one beta_surface: id from the register.",
                    ref=cohort_id,
                )
            )
        for field_name in ("intake_requirements", "minimum_evidence_classes", "graduation_criteria"):
            if not ensure_list(row.get(field_name), f"cohorts.cohorts[{idx}].{field_name}"):
                findings.append(
                    Finding(
                        severity="error",
                        check_id=f"cohorts.cohorts.{field_name}.empty",
                        message=f"{cohort_id} must declare non-empty {field_name}",
                        remediation=f"Populate {field_name} so cohort admission and graduation are inspectable.",
                        ref=cohort_id,
                    )
                )
        downgrade_rules = ensure_list(row.get("downgrade_rules"), f"cohorts.cohorts[{idx}].downgrade_rules")
        if not downgrade_rules:
            findings.append(
                Finding(
                    severity="error",
                    check_id="cohorts.cohorts.downgrade_rules.empty",
                    message=f"{cohort_id} must define at least one downgrade rule",
                    remediation="Add a downgrade_rules entry so cohort claims cannot drift silently.",
                    ref=cohort_id,
                )
            )
        for rule_idx, raw_rule in enumerate(downgrade_rules):
            rule = ensure_dict(raw_rule, f"cohorts.cohorts[{idx}].downgrade_rules[{rule_idx}]")
            ensure_str(rule.get("trigger"), f"cohorts.cohorts[{idx}].downgrade_rules[{rule_idx}].trigger")
            ensure_str(rule.get("downgrade_to"), f"cohorts.cohorts[{idx}].downgrade_rules[{rule_idx}].downgrade_to")
            validate_path_refs(
                repo_root,
                ensure_list(rule.get("propagation_refs"), f"cohorts.cohorts[{idx}].downgrade_rules[{rule_idx}].propagation_refs"),
                "cohorts.cohorts.downgrade_rules.propagation_refs",
                findings,
            )

    missing_required_cohorts = REQUIRED_M3_COHORTS - cohort_ids
    if missing_required_cohorts:
        findings.append(
            Finding(
                severity="error",
                check_id="cohorts.cohorts.missing_required",
                message="cohort guardrails are missing required M3 cohorts",
                remediation="Add extension-author, design-partner / managed pilot, and certified-archetype cohorts.",
                details={"missing": sorted(missing_required_cohorts)},
            )
        )

    return cohort_ids


def parse_mermaid_nodes(text: str) -> set[str]:
    pattern = re.compile(r"^\s*([A-Za-z_][A-Za-z0-9_]*)\[", re.MULTILINE)
    return set(pattern.findall(text))


def validate_graph(repo_root: Path, graph_path: Path, findings: list[Finding]) -> None:
    if not graph_path.exists():
        findings.append(
            Finding(
                severity="error",
                check_id="graph.missing",
                message=f"dependency graph file is missing: {graph_path}",
                remediation="Author artifacts/milestones/m3/dependency_graph.mmd before running the validator.",
            )
        )
        return
    text = graph_path.read_text(encoding="utf-8")
    if not text.lstrip().startswith(("%%", "graph", "flowchart")):
        findings.append(
            Finding(
                severity="error",
                check_id="graph.not_mermaid",
                message="dependency graph must be a Mermaid file (header `graph` or `flowchart` or `%%` comment)",
                remediation="Use Mermaid syntax so the dependency graph can be rendered alongside docs.",
            )
        )
    node_ids = parse_mermaid_nodes(text)
    missing_nodes = REQUIRED_GRAPH_NODES - node_ids
    if missing_nodes:
        findings.append(
            Finding(
                severity="error",
                check_id="graph.missing_required_nodes",
                message="dependency graph is missing required nodes",
                remediation="Add Mermaid nodes for the admission doc, register, guardrails, validator, capture, M2 matrix, reference workspace rows, deployment locality, known limits contract, and certified archetype template.",
                details={"missing": sorted(missing_nodes)},
            )
        )
    referenced_paths = {
        DEFAULT_REGISTER_REL,
        DEFAULT_COHORTS_REL,
        DEFAULT_DOC_REL,
        "ci/check_beta_admission.py",
        "artifacts/milestones/m2/alpha_wedge_matrix.yaml",
        "artifacts/compat/reference_workspace_rows.yaml",
        "artifacts/deployment/locality_matrix.yaml",
        "docs/product/known_limits_contract.md",
        "docs/release/certified_archetype_report_template.md",
    }
    missing_path_refs = {p for p in referenced_paths if p not in text}
    if missing_path_refs:
        findings.append(
            Finding(
                severity="error",
                check_id="graph.missing_referenced_paths",
                message="dependency graph must cite the canonical artifact paths in node labels",
                remediation="Add the missing repo-relative paths so reviewers can trace each node to its artifact.",
                details={"missing": sorted(missing_path_refs)},
            )
        )


def validate_admission_doc(repo_root: Path, doc_path: Path, findings: list[Finding]) -> None:
    if not doc_path.exists():
        findings.append(
            Finding(
                severity="error",
                check_id="doc.missing",
                message=f"admission doc is missing: {doc_path}",
                remediation="Author docs/milestones/m3/beta_admission_matrix.md before running the validator.",
            )
        )
        return
    text = doc_path.read_text(encoding="utf-8")
    required_phrases = [
        "artifacts/milestones/m3/claimed_surface_register.json",
        "artifacts/milestones/m3/cohort_guardrails.yaml",
        "artifacts/milestones/m3/dependency_graph.mmd",
        "ci/check_beta_admission.py",
        "Definition of green",
        "Update rules",
    ]
    missing_phrases = [phrase for phrase in required_phrases if phrase not in text]
    if missing_phrases:
        findings.append(
            Finding(
                severity="error",
                check_id="doc.missing_phrases",
                message="admission doc is missing required cross-references or sections",
                remediation="Link the canonical artifacts, validator command, and definition-of-green / update-rules sections.",
                details={"missing": missing_phrases},
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

    findings: list[Finding] = []
    cohort_ids = validate_cohorts(repo_root, cohorts, findings)
    validate_register(repo_root, register, cohort_ids, findings)
    validate_graph(repo_root, repo_root / args.graph, findings)
    validate_admission_doc(repo_root, repo_root / args.doc, findings)

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

    print("beta admission artifacts validated")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
