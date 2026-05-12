#!/usr/bin/env python3
"""Validate the external alpha benchmark fixture register."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_REGISTER_REL = "artifacts/benchmarks/m2_fixture_register.yaml"
DEFAULT_DOC_REL = "docs/benchmarks/privacy_cleared_corpus_workflow.md"
DEFAULT_PUBLICATION_TEMPLATE_REL = "docs/benchmarks/benchmark_publication_pack_template.md"
DEFAULT_CORPUS_REL = "fixtures/benchmarks/corpus_manifest.yaml"
DEFAULT_WORKSPACE_DIR_REL = "fixtures/workspaces/reference"
DEFAULT_SCOREBOARD_REL = "artifacts/milestones/m2/exit_gate_scoreboard.yaml"
DEFAULT_MATRIX_REL = "artifacts/milestones/m2/alpha_wedge_matrix.yaml"

REQUIRED_REFERENCE_WORKSPACES = {
    "refws.ts_web_app_archetype_seed": "alpha_wedge:typescript_javascript",
    "refws.python_data_app_archetype_seed": "alpha_wedge:python",
}

REQUIRED_ADDITION_FIELDS = {
    "register_row_id",
    "reference_workspace_id",
    "owner_dri",
    "corpus_refs",
    "intended_proof_lanes",
    "provenance",
    "privacy_review",
    "repeatability",
    "benchmark_packet_binding",
}

REQUIRED_REVIEW_STEPS = {
    "source_lineage_recorded",
    "privacy_clearance_complete",
    "retention_and_access_review_complete",
    "owner_and_backup_coverage_recorded",
    "corpus_or_reference_workspace_row_prepared",
    "benchmark_rehearsal_linked",
    "validator_passed",
}

REQUIRED_CITATION_FIELDS = {
    "register_row_id",
    "corpus_refs",
    "privacy_decision",
    "repeatability_notes",
}

REQUIRED_ACCEPTANCE_STATES = {
    "all_reference_workspaces_registered",
    "provenance_privacy_repeatability_required",
    "packets_cite_register_not_paths",
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
    parser.add_argument("--register", default=DEFAULT_REGISTER_REL)
    parser.add_argument("--doc", default=DEFAULT_DOC_REL)
    parser.add_argument("--publication-template", default=DEFAULT_PUBLICATION_TEMPLATE_REL)
    parser.add_argument("--corpus", default=DEFAULT_CORPUS_REL)
    parser.add_argument("--workspace-dir", default=DEFAULT_WORKSPACE_DIR_REL)
    parser.add_argument("--scoreboard", default=DEFAULT_SCOREBOARD_REL)
    parser.add_argument("--matrix", default=DEFAULT_MATRIX_REL)
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


def parse_iso_date(value: str, label: str) -> None:
    try:
        dt.date.fromisoformat(value)
    except ValueError as exc:
        raise SystemExit(f"{label} must be a YYYY-MM-DD date, got {value!r}") from exc


def strip_fragment(ref: str) -> str:
    return ref.split("#", 1)[0].strip()


def artifact_ref_exists(repo_root: Path, ref: str) -> bool:
    clean = strip_fragment(ref)
    return bool(clean) and (repo_root / clean).exists()


def validate_path_ref(repo_root: Path, ref: str, label: str, findings: list[Finding]) -> None:
    if not artifact_ref_exists(repo_root, ref):
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.missing_ref",
                message=f"{label} does not resolve: {ref}",
                remediation="Fix the path or seed the referenced artifact.",
                ref=ref,
            )
        )


def collect_corpus_ids(corpus: dict[str, Any]) -> set[str]:
    ids: set[str] = set()
    for idx, raw_fixture in enumerate(ensure_list(corpus.get("fixtures"), "corpus.fixtures")):
        fixture = ensure_dict(raw_fixture, f"corpus.fixtures[{idx}]")
        ids.add(ensure_str(fixture.get("id"), f"corpus.fixtures[{idx}].id"))
    return ids


def collect_scoreboard_ids(scoreboard: dict[str, Any]) -> set[str]:
    ids: set[str] = set()
    for idx, raw_row in enumerate(ensure_list(scoreboard.get("scoreboard_rows"), "scoreboard.scoreboard_rows")):
        row = ensure_dict(raw_row, f"scoreboard.scoreboard_rows[{idx}]")
        ids.add(ensure_str(row.get("row_id"), f"scoreboard.scoreboard_rows[{idx}].row_id"))
    return ids


def collect_workflow_ids(matrix: dict[str, Any]) -> set[str]:
    ids: set[str] = set()
    for row_idx, raw_row in enumerate(ensure_list(matrix.get("wedge_rows"), "matrix.wedge_rows")):
        row = ensure_dict(raw_row, f"matrix.wedge_rows[{row_idx}]")
        for workflow_idx, raw_workflow in enumerate(
            ensure_list(row.get("protected_workflows"), f"matrix.wedge_rows[{row_idx}].protected_workflows")
        ):
            workflow = ensure_dict(
                raw_workflow,
                f"matrix.wedge_rows[{row_idx}].protected_workflows[{workflow_idx}]",
            )
            ids.add(ensure_str(workflow.get("workflow_id"), "matrix.protected_workflows[].workflow_id"))
    return ids


def load_workspace_descriptors(repo_root: Path, workspace_dir: Path) -> dict[str, Path]:
    descriptors: dict[str, Path] = {}
    full_dir = repo_root / workspace_dir
    if not full_dir.exists():
        raise SystemExit(f"missing workspace descriptor directory: {full_dir}")
    for path in sorted(full_dir.glob("*.json")):
        payload = json.loads(path.read_text(encoding="utf-8"))
        workspace_id = payload.get("reference_workspace_id")
        if isinstance(workspace_id, str) and workspace_id:
            descriptors[workspace_id] = path
    return descriptors


def validate_header(register: dict[str, Any], findings: list[Finding]) -> None:
    schema_version = ensure_int(register.get("schema_version"), "register.schema_version")
    if schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="register.schema_version",
                message=f"register.schema_version must be 1, got {schema_version}",
                remediation="Update the validator in the same change as a schema bump.",
            )
        )
    ensure_str(register.get("register_id"), "register.register_id")
    ensure_int(register.get("register_revision"), "register.register_revision")
    parse_iso_date(ensure_str(register.get("as_of"), "register.as_of"), "register.as_of")
    ensure_str(register.get("owner"), "register.owner")


def validate_source_refs(repo_root: Path, register: dict[str, Any], findings: list[Finding]) -> None:
    validate_path_ref(repo_root, ensure_str(register.get("human_entrypoint_ref"), "register.human_entrypoint_ref"), "register.human_entrypoint_ref", findings)
    validator = ensure_dict(register.get("validator"), "register.validator")
    validate_path_ref(repo_root, ensure_str(validator.get("script_ref"), "register.validator.script_ref"), "register.validator.script_ref", findings)
    ensure_str(validator.get("command"), "register.validator.command")
    for key, raw_ref in ensure_dict(register.get("source_contract_refs"), "register.source_contract_refs").items():
        ref = ensure_str(raw_ref, f"register.source_contract_refs.{key}")
        validate_path_ref(repo_root, ref, f"register.source_contract_refs.{key}", findings)
    validate_path_ref(repo_root, ensure_str(register.get("fixture_packet_directory"), "register.fixture_packet_directory"), "register.fixture_packet_directory", findings)


def validate_addition_gate(register: dict[str, Any], findings: list[Finding]) -> None:
    gate = ensure_dict(register.get("addition_gate"), "register.addition_gate")
    required_fields = set(ensure_list(gate.get("new_corpus_addition_required_fields"), "register.addition_gate.new_corpus_addition_required_fields"))
    missing_fields = REQUIRED_ADDITION_FIELDS - required_fields
    if missing_fields:
        findings.append(
            Finding(
                severity="error",
                check_id="register.addition_gate.required_fields",
                message="addition gate is missing required corpus-admission fields",
                remediation="Require provenance, privacy review, repeatability, owners, corpus refs, proof lanes, and packet binding.",
                details={"missing": sorted(missing_fields)},
            )
        )
    review_steps = set(ensure_list(gate.get("review_steps"), "register.addition_gate.review_steps"))
    missing_steps = REQUIRED_REVIEW_STEPS - review_steps
    if missing_steps:
        findings.append(
            Finding(
                severity="error",
                check_id="register.addition_gate.review_steps",
                message="addition gate is missing required review steps",
                remediation="Add source lineage, privacy, retention, owner, rehearsal, and validator steps.",
                details={"missing": sorted(missing_steps)},
            )
        )
    if ensure_str(gate.get("default_for_missing_privacy_review"), "register.addition_gate.default_for_missing_privacy_review") != "exclude_until_replaced":
        findings.append(
            Finding(
                severity="error",
                check_id="register.addition_gate.privacy_default",
                message="missing privacy review must default to exclude_until_replaced",
                remediation="Set default_for_missing_privacy_review to exclude_until_replaced.",
            )
        )
    ensure_str(gate.get("packet_citation_rule"), "register.addition_gate.packet_citation_rule")


def validate_packet(
    repo_root: Path,
    packet_ref: str,
    row_id: str,
    reference_workspace_id: str,
    scoreboard_ids: set[str],
    workflow_ids: set[str],
    row_workflow_refs: set[str],
    findings: list[Finding],
) -> None:
    validate_path_ref(repo_root, packet_ref, "register.reference_workspaces.fixture_packet_ref", findings)
    if not artifact_ref_exists(repo_root, packet_ref):
        return
    packet = ensure_dict(render_yaml_as_json(repo_root / strip_fragment(packet_ref)), f"fixture_packet[{packet_ref}]")
    if ensure_str(packet.get("register_row_ref"), "fixture_packet.register_row_ref") != row_id:
        findings.append(
            Finding(
                severity="error",
                check_id="fixture_packet.register_row_ref",
                message=f"{packet_ref} does not point back to {row_id}",
                remediation="Set fixture packet register_row_ref to the owning register row.",
                ref=packet_ref,
            )
        )
    if ensure_str(packet.get("reference_workspace_id"), "fixture_packet.reference_workspace_id") != reference_workspace_id:
        findings.append(
            Finding(
                severity="error",
                check_id="fixture_packet.reference_workspace_id",
                message=f"{packet_ref} does not match register reference_workspace_id {reference_workspace_id}",
                remediation="Use the same reference_workspace_id in the register row and packet.",
                ref=packet_ref,
            )
        )
    for section in ("provenance", "privacy_review", "repeatability"):
        ensure_dict(packet.get(section), f"fixture_packet.{section}")
    packet_scoreboard_refs = set(ensure_list(packet.get("scoreboard_row_refs"), "fixture_packet.scoreboard_row_refs"))
    unknown_scoreboard_refs = packet_scoreboard_refs - scoreboard_ids
    if unknown_scoreboard_refs:
        findings.append(
            Finding(
                severity="error",
                check_id="fixture_packet.scoreboard_row_refs.unknown",
                message=f"{packet_ref} cites unknown scoreboard rows",
                remediation="Use scoreboard row ids from the alpha exit-gate scoreboard.",
                ref=packet_ref,
                details={"unknown": sorted(unknown_scoreboard_refs)},
            )
        )
    packet_workflow_refs: set[str] = set()
    for idx, raw_workflow in enumerate(ensure_list(packet.get("protected_workflows"), "fixture_packet.protected_workflows")):
        workflow = ensure_dict(raw_workflow, f"fixture_packet.protected_workflows[{idx}]")
        workflow_id = ensure_str(workflow.get("workflow_id"), f"fixture_packet.protected_workflows[{idx}].workflow_id")
        packet_workflow_refs.add(workflow_id)
        if workflow_id not in workflow_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="fixture_packet.protected_workflows.unknown",
                    message=f"{packet_ref} cites unknown matrix workflow {workflow_id}",
                    remediation="Use workflow ids from the alpha wedge matrix.",
                    ref=packet_ref,
                )
            )
        row_ref = ensure_str(workflow.get("scoreboard_row_ref"), f"fixture_packet.protected_workflows[{idx}].scoreboard_row_ref")
        if row_ref not in scoreboard_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="fixture_packet.protected_workflows.unknown_scoreboard_row",
                    message=f"{packet_ref} workflow cites unknown scoreboard row {row_ref}",
                    remediation="Use scoreboard row ids from the alpha exit-gate scoreboard.",
                    ref=packet_ref,
                )
            )
    missing_workflows = row_workflow_refs - packet_workflow_refs
    if missing_workflows:
        findings.append(
            Finding(
                severity="error",
                check_id="fixture_packet.protected_workflows.missing",
                message=f"{packet_ref} does not cover all register workflow refs",
                remediation="Add packet workflow blocks for every protected_workflow_ref in the register row.",
                ref=packet_ref,
                details={"missing": sorted(missing_workflows)},
            )
        )


def validate_reference_workspaces(
    repo_root: Path,
    register: dict[str, Any],
    workspace_ids: dict[str, Path],
    corpus_ids: set[str],
    scoreboard_ids: set[str],
    workflow_ids: set[str],
    findings: list[Finding],
) -> None:
    privacy_classes = set(ensure_list(register.get("privacy_class_vocabulary"), "register.privacy_class_vocabulary"))
    privacy_decisions = set(ensure_list(register.get("privacy_decision_vocabulary"), "register.privacy_decision_vocabulary"))
    source_classes = set(ensure_list(register.get("source_class_vocabulary"), "register.source_class_vocabulary"))
    admission_states = set(ensure_list(register.get("admission_state_vocabulary"), "register.admission_state_vocabulary"))

    rows = ensure_list(register.get("reference_workspaces"), "register.reference_workspaces")
    seen_rows: set[str] = set()
    seen_workspaces: dict[str, str] = {}
    for idx, raw_row in enumerate(rows):
        row = ensure_dict(raw_row, f"register.reference_workspaces[{idx}]")
        row_id = ensure_str(row.get("register_row_id"), f"register.reference_workspaces[{idx}].register_row_id")
        if row_id in seen_rows:
            findings.append(
                Finding(
                    severity="error",
                    check_id="register.reference_workspaces.duplicate_row",
                    message=f"duplicate register row id: {row_id}",
                    remediation="Use one register row per reference workspace.",
                    ref=row_id,
                )
            )
        seen_rows.add(row_id)

        reference_workspace_id = ensure_str(row.get("reference_workspace_id"), f"register.reference_workspaces[{idx}].reference_workspace_id")
        seen_workspaces[reference_workspace_id] = row_id
        expected_wedge = REQUIRED_REFERENCE_WORKSPACES.get(reference_workspace_id)
        if expected_wedge and ensure_str(row.get("wedge_ref"), f"register.reference_workspaces[{idx}].wedge_ref") != expected_wedge:
            findings.append(
                Finding(
                    severity="error",
                    check_id="register.reference_workspaces.wedge_ref",
                    message=f"{reference_workspace_id} is bound to the wrong alpha wedge",
                    remediation="Use the wedge ref declared by the alpha scope matrix.",
                    ref=row_id,
                    details={"expected": expected_wedge},
                )
            )
        if reference_workspace_id not in workspace_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="register.reference_workspaces.unknown_workspace",
                    message=f"reference_workspace_id is not present in fixture descriptors: {reference_workspace_id}",
                    remediation="Add the workspace descriptor or correct the register row.",
                    ref=row_id,
                )
            )

        descriptor_ref = ensure_str(row.get("workspace_descriptor_ref"), f"register.reference_workspaces[{idx}].workspace_descriptor_ref")
        validate_path_ref(repo_root, descriptor_ref, "register.reference_workspaces.workspace_descriptor_ref", findings)
        if artifact_ref_exists(repo_root, descriptor_ref):
            descriptor = json.loads((repo_root / strip_fragment(descriptor_ref)).read_text(encoding="utf-8"))
            if descriptor.get("reference_workspace_id") != reference_workspace_id:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="register.reference_workspaces.descriptor_id_mismatch",
                        message=f"{descriptor_ref} does not carry {reference_workspace_id}",
                        remediation="Fix the descriptor ref or register reference_workspace_id.",
                        ref=row_id,
                    )
                )

        corpus_refs = set(ensure_list(row.get("corpus_refs"), f"register.reference_workspaces[{idx}].corpus_refs"))
        unknown_corpus_refs = corpus_refs - corpus_ids
        if unknown_corpus_refs:
            findings.append(
                Finding(
                    severity="error",
                    check_id="register.reference_workspaces.unknown_corpus_refs",
                    message=f"{row_id} cites corpus ids absent from the protected manifest",
                    remediation="Use corpus ids from fixtures/benchmarks/corpus_manifest.yaml or add the corpus row in the same change.",
                    ref=row_id,
                    details={"unknown": sorted(unknown_corpus_refs)},
                )
            )

        intended_lanes = set(ensure_list(row.get("intended_proof_lanes"), f"register.reference_workspaces[{idx}].intended_proof_lanes"))
        unknown_lanes = intended_lanes - scoreboard_ids
        if unknown_lanes:
            findings.append(
                Finding(
                    severity="error",
                    check_id="register.reference_workspaces.unknown_proof_lanes",
                    message=f"{row_id} cites unknown scoreboard rows",
                    remediation="Use row ids from artifacts/milestones/m2/exit_gate_scoreboard.yaml.",
                    ref=row_id,
                    details={"unknown": sorted(unknown_lanes)},
                )
            )
        if "scoreboard_row:alpha_scope.benchmark_fixtures" not in intended_lanes:
            findings.append(
                Finding(
                    severity="error",
                    check_id="register.reference_workspaces.missing_benchmark_lane",
                    message=f"{row_id} must cite the benchmark fixture scoreboard row",
                    remediation="Add scoreboard_row:alpha_scope.benchmark_fixtures to intended_proof_lanes.",
                    ref=row_id,
                )
            )

        workflow_refs = set(ensure_list(row.get("protected_workflow_refs"), f"register.reference_workspaces[{idx}].protected_workflow_refs"))
        unknown_workflows = workflow_refs - workflow_ids
        if unknown_workflows:
            findings.append(
                Finding(
                    severity="error",
                    check_id="register.reference_workspaces.unknown_workflows",
                    message=f"{row_id} cites unknown alpha matrix workflows",
                    remediation="Use workflow ids from artifacts/milestones/m2/alpha_wedge_matrix.yaml.",
                    ref=row_id,
                    details={"unknown": sorted(unknown_workflows)},
                )
            )

        owner = ensure_dict(row.get("owner"), f"register.reference_workspaces[{idx}].owner")
        for field_name in ("owner_dri", "selection_owner_ref", "evidence_owner_ref", "publication_owner_ref", "privacy_reviewer_ref"):
            ensure_str(owner.get(field_name), f"register.reference_workspaces[{idx}].owner.{field_name}")

        provenance = ensure_dict(row.get("provenance"), f"register.reference_workspaces[{idx}].provenance")
        source_class = ensure_str(provenance.get("source_class"), f"register.reference_workspaces[{idx}].provenance.source_class")
        if source_class not in source_classes:
            findings.append(
                Finding(
                    severity="error",
                    check_id="register.reference_workspaces.invalid_source_class",
                    message=f"{row_id} uses unknown source_class {source_class}",
                    remediation="Use a source class listed in source_class_vocabulary.",
                    ref=row_id,
                )
            )
        for field_name in ("origin", "source_revision_ref", "license_status", "lineage_summary_ref"):
            ensure_str(provenance.get(field_name), f"register.reference_workspaces[{idx}].provenance.{field_name}")

        privacy = ensure_dict(row.get("privacy_review"), f"register.reference_workspaces[{idx}].privacy_review")
        privacy_class = ensure_str(privacy.get("privacy_class"), f"register.reference_workspaces[{idx}].privacy_review.privacy_class")
        if privacy_class not in privacy_classes:
            findings.append(
                Finding(
                    severity="error",
                    check_id="register.reference_workspaces.invalid_privacy_class",
                    message=f"{row_id} uses unknown privacy_class {privacy_class}",
                    remediation="Use a privacy class listed in privacy_class_vocabulary.",
                    ref=row_id,
                )
            )
        privacy_decision = ensure_str(privacy.get("privacy_decision"), f"register.reference_workspaces[{idx}].privacy_review.privacy_decision")
        if privacy_decision not in privacy_decisions:
            findings.append(
                Finding(
                    severity="error",
                    check_id="register.reference_workspaces.invalid_privacy_decision",
                    message=f"{row_id} uses unknown privacy_decision {privacy_decision}",
                    remediation="Use a privacy decision listed in privacy_decision_vocabulary.",
                    ref=row_id,
                )
            )
        if privacy_class.startswith("public_") and privacy_decision not in {"admit_public", "redact_then_admit"}:
            findings.append(
                Finding(
                    severity="error",
                    check_id="register.reference_workspaces.public_privacy_decision",
                    message=f"{row_id} has public privacy class but non-public clearance decision",
                    remediation="Public fixture rows must be admit_public or redact_then_admit.",
                    ref=row_id,
                )
            )
        for field_name in ("reviewed_on", "reviewer_ref", "raw_private_bytes_omission_note"):
            ensure_str(privacy.get(field_name), f"register.reference_workspaces[{idx}].privacy_review.{field_name}")
        parse_iso_date(ensure_str(privacy.get("reviewed_on"), f"register.reference_workspaces[{idx}].privacy_review.reviewed_on"), "privacy_review.reviewed_on")
        if not ensure_list(privacy.get("review_evidence_refs"), f"register.reference_workspaces[{idx}].privacy_review.review_evidence_refs"):
            findings.append(
                Finding(
                    severity="error",
                    check_id="register.reference_workspaces.privacy_evidence.empty",
                    message=f"{row_id} must carry privacy review evidence refs",
                    remediation="Add review_evidence_refs to the privacy_review block.",
                    ref=row_id,
                )
            )

        repeatability = ensure_dict(row.get("repeatability"), f"register.reference_workspaces[{idx}].repeatability")
        for field_name in ("resolution_mode", "host_platform_class", "toolchain_assumption"):
            ensure_str(repeatability.get(field_name), f"register.reference_workspaces[{idx}].repeatability.{field_name}")
        deterministic_inputs = ensure_list(repeatability.get("deterministic_inputs"), f"register.reference_workspaces[{idx}].repeatability.deterministic_inputs")
        for input_ref in deterministic_inputs:
            validate_path_ref(repo_root, ensure_str(input_ref, "repeatability.deterministic_inputs[]"), "repeatability.deterministic_inputs", findings)
        if not ensure_list(repeatability.get("repeatability_notes"), f"register.reference_workspaces[{idx}].repeatability.repeatability_notes"):
            findings.append(
                Finding(
                    severity="error",
                    check_id="register.reference_workspaces.repeatability_notes.empty",
                    message=f"{row_id} must carry repeatability notes",
                    remediation="Add repeatability_notes explaining how future runs reproduce the fixture.",
                    ref=row_id,
                )
            )

        packet_binding = ensure_dict(row.get("benchmark_packet_binding"), f"register.reference_workspaces[{idx}].benchmark_packet_binding")
        citation_fields = set(ensure_list(packet_binding.get("citation_fields"), f"register.reference_workspaces[{idx}].benchmark_packet_binding.citation_fields"))
        missing_citation_fields = REQUIRED_CITATION_FIELDS - citation_fields
        if missing_citation_fields:
            findings.append(
                Finding(
                    severity="error",
                    check_id="register.reference_workspaces.packet_binding.citation_fields",
                    message=f"{row_id} packet binding does not expose all required citation fields",
                    remediation="Include register_row_id, corpus_refs, privacy_decision, and repeatability_notes.",
                    ref=row_id,
                    details={"missing": sorted(missing_citation_fields)},
                )
            )
        validate_path_ref(
            repo_root,
            ensure_str(packet_binding.get("packet_template_ref"), f"register.reference_workspaces[{idx}].benchmark_packet_binding.packet_template_ref"),
            "benchmark_packet_binding.packet_template_ref",
            findings,
        )

        admission = ensure_dict(row.get("admission"), f"register.reference_workspaces[{idx}].admission")
        state = ensure_str(admission.get("state"), f"register.reference_workspaces[{idx}].admission.state")
        if state not in admission_states:
            findings.append(
                Finding(
                    severity="error",
                    check_id="register.reference_workspaces.invalid_admission_state",
                    message=f"{row_id} uses unknown admission state {state}",
                    remediation="Use a state listed in admission_state_vocabulary.",
                    ref=row_id,
                )
            )

        validate_packet(
            repo_root,
            ensure_str(row.get("fixture_packet_ref"), f"register.reference_workspaces[{idx}].fixture_packet_ref"),
            row_id,
            reference_workspace_id,
            scoreboard_ids,
            workflow_ids,
            workflow_refs,
            findings,
        )

    missing_workspaces = set(REQUIRED_REFERENCE_WORKSPACES) - set(seen_workspaces)
    if missing_workspaces:
        findings.append(
            Finding(
                severity="error",
                check_id="register.reference_workspaces.missing_required",
                message="fixture register is missing required alpha reference workspaces",
                remediation="Register the TS/JS and Python alpha reference workspaces.",
                details={"missing": sorted(missing_workspaces)},
            )
        )


def validate_supporting_cases(repo_root: Path, register: dict[str, Any], findings: list[Finding]) -> None:
    cases = ensure_list(register.get("supporting_privacy_clearance_cases"), "register.supporting_privacy_clearance_cases")
    if len(cases) < 3:
        findings.append(
            Finding(
                severity="error",
                check_id="register.supporting_privacy_clearance_cases.too_few",
                message="supporting privacy clearance cases must cover admitted, internal-only, and excluded paths",
                remediation="Add privacy clearance case refs for admitted, internal-only, and excluded outcomes.",
            )
        )
    for idx, raw_case in enumerate(cases):
        case = ensure_dict(raw_case, f"register.supporting_privacy_clearance_cases[{idx}]")
        validate_path_ref(repo_root, ensure_str(case.get("case_ref"), "supporting_privacy_clearance_cases.case_ref"), "supporting_privacy_clearance_cases.case_ref", findings)
        ensure_str(case.get("exercises"), "supporting_privacy_clearance_cases.exercises")


def validate_acceptance(register: dict[str, Any], findings: list[Finding]) -> None:
    coverage = ensure_list(register.get("acceptance_state_coverage"), "register.acceptance_state_coverage")
    states: set[str] = set()
    for idx, raw_case in enumerate(coverage):
        case = ensure_dict(raw_case, f"register.acceptance_state_coverage[{idx}]")
        states.add(ensure_str(case.get("exercises_state"), f"register.acceptance_state_coverage[{idx}].exercises_state"))
        ensure_str(case.get("fixture_ref"), f"register.acceptance_state_coverage[{idx}].fixture_ref")
        ensure_str(case.get("expected_validator_result"), f"register.acceptance_state_coverage[{idx}].expected_validator_result")
    missing_states = REQUIRED_ACCEPTANCE_STATES - states
    if missing_states:
        findings.append(
            Finding(
                severity="error",
                check_id="register.acceptance_state_coverage.missing",
                message="acceptance coverage is missing required states",
                remediation="Cover reference workspace registration, admission gate review fields, and packet citation.",
                details={"missing": sorted(missing_states)},
            )
        )


def validate_docs(repo_root: Path, doc_rel: str, template_rel: str, register_rel: str, findings: list[Finding]) -> None:
    for label, rel in (("workflow_doc", doc_rel), ("publication_template", template_rel)):
        path = repo_root / rel
        if not path.exists():
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"{label}.missing",
                    message=f"{rel} is missing",
                    remediation="Create the document or fix the validator argument.",
                    ref=rel,
                )
            )
            continue
        text = path.read_text(encoding="utf-8")
        if register_rel not in text:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"{label}.missing_register_ref",
                    message=f"{rel} does not cite {register_rel}",
                    remediation="Cite the fixture register so packets resolve through the canonical source.",
                    ref=rel,
                )
            )
        if label == "workflow_doc":
            for phrase in ("provenance", "privacy review", "repeatability", "Fixture register"):
                if phrase not in text:
                    findings.append(
                        Finding(
                            severity="error",
                            check_id=f"{label}.missing_required_phrase",
                            message=f"{rel} does not mention {phrase!r}",
                            remediation="Document provenance, privacy review, repeatability, and the fixture register.",
                            ref=rel,
                        )
                    )
        else:
            for phrase in ("Fixture register revision", "Fixture register rows cited"):
                if phrase not in text:
                    findings.append(
                        Finding(
                            severity="error",
                            check_id=f"{label}.missing_packet_field",
                            message=f"{rel} does not include packet field {phrase!r}",
                            remediation="Add fixture-register citation fields to the benchmark publication template.",
                            ref=rel,
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

    register = ensure_dict(render_yaml_as_json(repo_root / args.register), "register")
    corpus = ensure_dict(render_yaml_as_json(repo_root / args.corpus), "corpus")
    scoreboard = ensure_dict(render_yaml_as_json(repo_root / args.scoreboard), "scoreboard")
    matrix = ensure_dict(render_yaml_as_json(repo_root / args.matrix), "matrix")
    workspace_ids = load_workspace_descriptors(repo_root, Path(args.workspace_dir))
    corpus_ids = collect_corpus_ids(corpus)
    scoreboard_ids = collect_scoreboard_ids(scoreboard)
    workflow_ids = collect_workflow_ids(matrix)

    findings: list[Finding] = []
    validate_header(register, findings)
    validate_source_refs(repo_root, register, findings)
    validate_addition_gate(register, findings)
    validate_reference_workspaces(repo_root, register, workspace_ids, corpus_ids, scoreboard_ids, workflow_ids, findings)
    validate_supporting_cases(repo_root, register, findings)
    validate_acceptance(register, findings)
    validate_docs(repo_root, args.doc, args.publication_template, args.register, findings)

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

    print("benchmark fixture register validated")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

