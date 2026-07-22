#!/usr/bin/env python3
"""Validate and render backup/restore/failover rehearsal artifacts."""

from __future__ import annotations

import argparse
import json
import os
import secrets
import stat
import subprocess
import sys
import unicodedata
from dataclasses import asdict, dataclass, field
from pathlib import Path, PurePosixPath
from typing import Any


DEFAULT_TAXONOMY_REL = "artifacts/ops/outage_taxonomy_alpha.yaml"
DEFAULT_PLAN_REL = "docs/ops/backup_restore_failover_rehearsal_plan.md"
DEFAULT_EXAMPLES_REL = "artifacts/ops/control_plane_vs_data_plane_examples.md"
DEFAULT_FIXTURE_MANIFEST_REL = "fixtures/ops/backup_restore_failover_rehearsal_cases/manifest.yaml"

MAX_INPUT_BYTES = 4 * 1024 * 1024
MAX_COLLECTION_ENTRIES = 4_096
MAX_PARSED_NODES = 65_536
MAX_NESTING_DEPTH = 128
MAX_SCALAR_BYTES = 256 * 1024
MAX_PATH_REFERENCE_BYTES = 4_096
YAML_PARSE_TIMEOUT_SECONDS = 15

REQUIRED_CLASS_PLANES = {
    "local_core_continuity": "local_core",
    "control_plane_impairment": "control_plane",
    "data_plane_impairment": "data_plane",
    "full_target_loss": "target_authority",
}

REQUIRED_ACCEPTANCE_STATES = {
    "taxonomy_covers_four_required_outage_classes",
    "each_outage_class_has_expected_product_posture",
    "each_outage_class_has_recovery_action_list",
    "rehearsal_plan_names_owners_cadence_and_proof_artifacts",
    "protected_fixtures_cover_all_outage_classes",
    "support_projection_is_metadata_only",
}

REQUIRED_ROLE_MARKERS = {
    "Support-room owner",
    "Release captain",
    "Supportability engineer",
    "Security/privacy reviewer",
    "Docs/comms owner",
}

REQUIRED_PLAN_MARKERS = {
    "Monthly",
    "Before each release candidate",
    "Required Proof Artifacts",
    "Restore Destination Review",
    "python3 ci/check_backup_restore_failover_alpha.py --repo-root .",
}

REQUIRED_DOC_PATH_MARKERS = {
    "artifacts/ops/outage_taxonomy_alpha.yaml",
    "artifacts/ops/backup_restore_failover_rehearsal_proof.md",
    "fixtures/ops/backup_restore_failover_rehearsal_cases/manifest.yaml",
    "artifacts/ops/control_plane_vs_data_plane_examples.md",
    "ci/check_backup_restore_failover_alpha.py",
}

REQUIRED_EXPORT_FALSE_FIELDS = {
    "raw_payload_exported",
    "raw_tenant_names_exported",
    "raw_urls_or_hostnames_exported",
    "raw_secret_material_exported",
}

REQUIRED_POSTURE_FIELDS = {
    "posture_class",
    "local_core_available",
    "user_visible_status",
    "managed_action_posture",
    "restore_posture",
    "support_export_posture",
    "boundary_review_required",
    "forbidden_postures",
}

REQUIRED_PLANE_STATE_FIELDS = {
    "local_core",
    "control_plane",
    "data_plane",
    "target_authority",
}

REQUIRED_CASE_PLANE_FIELDS = {
    "local_core_state",
    "control_plane_state",
    "data_plane_state",
    "target_authority_state",
    "target_reachable",
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


class InputContractError(RuntimeError):
    """Redaction-safe protected-input rejection."""


def _safe_relative_parts(reference: str, label: str, *, suffix: str | None = None) -> tuple[str, ...]:
    try:
        reference_bytes = reference.encode("utf-8")
    except UnicodeEncodeError as exc:
        raise InputContractError(f"{label} is not a valid UTF-8 path") from exc
    if not reference or len(reference_bytes) > MAX_PATH_REFERENCE_BYTES:
        raise InputContractError(f"{label} violates the path-size contract")
    if any(
        ord(char) < 32 or ord(char) == 127 or unicodedata.category(char) == "Cf"
        for char in reference
    ):
        raise InputContractError(f"{label} contains unsafe path characters")
    if "\\" in reference or (len(reference) > 1 and reference[1] == ":"):
        raise InputContractError(f"{label} is not a portable relative path")

    raw_parts = reference.split("/")
    parsed = PurePosixPath(reference)
    if parsed.is_absolute() or any(part in {"", ".", ".."} for part in raw_parts):
        raise InputContractError(f"{label} is outside its path authority")
    if suffix is not None and parsed.suffix != suffix:
        raise InputContractError(f"{label} has an unsupported file type")
    return parsed.parts


def _require_under(path: Path, authority: Path, label: str) -> None:
    try:
        path.relative_to(authority)
    except ValueError as exc:
        raise InputContractError(f"{label} is outside its path authority") from exc


def _resolve_existing_reference(
    authority: Path,
    base: Path,
    reference: str,
    label: str,
    *,
    suffix: str | None = None,
    require_file: bool = False,
) -> Path:
    parts = _safe_relative_parts(reference, label, suffix=suffix)
    authority = authority.resolve(strict=True)
    base = base.resolve(strict=True)
    _require_under(base, authority, label)
    candidate = base.joinpath(*parts)
    _require_under(candidate, base, label)

    current = authority
    for part in candidate.relative_to(authority).parts:
        current /= part
        try:
            metadata = os.lstat(current)
        except OSError as exc:
            raise InputContractError(f"{label} is unavailable") from exc
        if stat.S_ISLNK(metadata.st_mode):
            raise InputContractError(f"{label} uses a path redirect")

    try:
        resolved = candidate.resolve(strict=True)
    except OSError as exc:
        raise InputContractError(f"{label} is unavailable") from exc
    _require_under(resolved, base, label)
    if require_file and not resolved.is_file():
        raise InputContractError(f"{label} is not a regular file")
    return resolved


def _stable_file_identity(left: os.stat_result, right: os.stat_result) -> bool:
    return (
        stat.S_ISREG(left.st_mode)
        and stat.S_ISREG(right.st_mode)
        and left.st_dev == right.st_dev
        and left.st_ino == right.st_ino
        and left.st_size == right.st_size
        and left.st_mtime_ns == right.st_mtime_ns
        and left.st_ctime_ns == right.st_ctime_ns
    )


def _read_bounded_text(path: Path, authority: Path, label: str) -> str:
    _require_under(path, authority, label)
    try:
        before = os.lstat(path)
    except OSError as exc:
        raise InputContractError(f"{label} is unavailable") from exc
    if stat.S_ISLNK(before.st_mode) or not stat.S_ISREG(before.st_mode):
        raise InputContractError(f"{label} is not a stable regular file")
    if before.st_size > MAX_INPUT_BYTES:
        raise InputContractError(f"{label} exceeds the input-byte limit")

    try:
        with path.open("rb") as source:
            opened = os.fstat(source.fileno())
            if not _stable_file_identity(before, opened):
                raise InputContractError(f"{label} changed before reading")
            body = source.read(MAX_INPUT_BYTES + 1)
            descriptor_after = os.fstat(source.fileno())
    except InputContractError:
        raise
    except OSError as exc:
        raise InputContractError(f"{label} could not be read") from exc

    if len(body) > MAX_INPUT_BYTES:
        raise InputContractError(f"{label} exceeds the input-byte limit")
    try:
        path_after = os.lstat(path)
        resolved_after = path.resolve(strict=True)
    except OSError as exc:
        raise InputContractError(f"{label} changed while reading") from exc
    _require_under(resolved_after, authority, label)
    if resolved_after != path or not _stable_file_identity(opened, descriptor_after) or not _stable_file_identity(descriptor_after, path_after):
        raise InputContractError(f"{label} changed while reading")
    try:
        return body.decode("utf-8")
    except UnicodeDecodeError as exc:
        raise InputContractError(f"{label} is not valid UTF-8") from exc


def _validate_parsed_shape(value: Any, label: str) -> None:
    node_count = 0
    pending: list[tuple[Any, int]] = [(value, 0)]
    while pending:
        current, depth = pending.pop()
        if depth > MAX_NESTING_DEPTH:
            raise InputContractError(f"{label} exceeds the nesting-depth limit")
        node_count += 1
        if node_count > MAX_PARSED_NODES:
            raise InputContractError(f"{label} exceeds the parsed-node limit")
        if isinstance(current, str):
            try:
                scalar_bytes = current.encode("utf-8")
            except UnicodeEncodeError as exc:
                raise InputContractError(f"{label} contains invalid scalar text") from exc
            if len(scalar_bytes) > MAX_SCALAR_BYTES:
                raise InputContractError(f"{label} exceeds the scalar-byte limit")
        if isinstance(current, dict):
            if len(current) > MAX_COLLECTION_ENTRIES:
                raise InputContractError(f"{label} exceeds the mapping-entry limit")
            pending.extend((item, depth + 1) for pair in current.items() for item in pair)
        elif isinstance(current, list):
            if len(current) > MAX_COLLECTION_ENTRIES:
                raise InputContractError(f"{label} exceeds the sequence-entry limit")
            pending.extend((item, depth + 1) for item in current)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--taxonomy", default=DEFAULT_TAXONOMY_REL)
    parser.add_argument("--plan", default=DEFAULT_PLAN_REL)
    parser.add_argument("--examples", default=DEFAULT_EXAMPLES_REL)
    parser.add_argument("--fixture-manifest", default=DEFAULT_FIXTURE_MANIFEST_REL)
    parser.add_argument("--report", default=None)
    parser.add_argument(
        "--render-support-projection",
        action="store_true",
        help="Print the metadata-only support/release projection after validation.",
    )
    return parser.parse_args()


def render_yaml_as_json(path: Path, authority: Path, label: str) -> Any:
    body = _read_bounded_text(path, authority, label)
    try:
        ruby = subprocess.run(
            [
                "ruby",
                "-rjson",
                "-ryaml",
                "-e",
                (
                    "payload = YAML.safe_load(STDIN.read, permitted_classes: [], aliases: false); "
                    "STDOUT.write(JSON.generate(payload))"
                ),
            ],
            input=body,
            capture_output=True,
            text=True,
            timeout=YAML_PARSE_TIMEOUT_SECONDS,
            check=False,
        )
    except subprocess.TimeoutExpired as exc:
        raise InputContractError(f"{label} exceeded the parse-time limit") from exc
    except OSError as exc:
        raise InputContractError(f"{label} parser is unavailable") from exc
    if ruby.returncode != 0:
        raise InputContractError(f"{label} is not valid bounded YAML")
    try:
        payload = json.loads(ruby.stdout)
    except (json.JSONDecodeError, ValueError) as exc:
        raise InputContractError(f"{label} parser returned an invalid document") from exc
    _validate_parsed_shape(payload, label)
    return payload


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


def ensure_bool(value: Any, label: str) -> bool:
    if not isinstance(value, bool):
        raise SystemExit(f"{label} must be a boolean")
    return value


def strip_fragment(ref: str) -> str:
    return ref.split("#", 1)[0].strip()


def validate_path_ref(repo_root: Path, ref: str, label: str, findings: list[Finding]) -> None:
    clean = strip_fragment(ref)
    try:
        _resolve_existing_reference(
            repo_root,
            repo_root,
            clean,
            label,
        )
    except InputContractError:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.unsafe_or_missing_ref",
                message=f"{label} does not resolve within the repository path authority.",
                remediation="Use a stable repository-relative path to a regular, non-redirected artifact.",
            )
        )


def validate_export_safety(
    export_safety: dict[str, Any],
    label: str,
    findings: list[Finding],
    ref: str,
    *,
    metadata_only_required: bool = False,
) -> None:
    for field_name in REQUIRED_EXPORT_FALSE_FIELDS:
        if field_name in export_safety and ensure_bool(export_safety[field_name], f"{label}.{field_name}"):
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"{label}.{field_name}.must_be_false",
                    message=f"{label}.{field_name} must remain false.",
                    remediation="Keep this rehearsal lane metadata-only and redaction-safe.",
                    ref=ref,
                )
            )
    if metadata_only_required and export_safety.get("metadata_only") is not True:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.metadata_only.required",
                message=f"{label}.metadata_only must be true.",
                remediation="Mark the fixture export as metadata-only.",
                ref=ref,
            )
        )
    if export_safety.get("exact_build_identity_required") is False:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.exact_build_identity_required.false",
                message=f"{label}.exact_build_identity_required cannot be false.",
                remediation="Continuity proof must preserve exact-build identity.",
                ref=ref,
            )
        )


def validate_taxonomy(repo_root: Path, taxonomy: dict[str, Any], findings: list[Finding]) -> dict[str, dict[str, Any]]:
    if ensure_int(taxonomy.get("schema_version"), "taxonomy.schema_version") != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="taxonomy.schema_version.unsupported",
                message="taxonomy.schema_version must be 1.",
                remediation="Update the validator in the same change as a schema change.",
                ref=DEFAULT_TAXONOMY_REL,
            )
        )
    if ensure_str(taxonomy.get("record_kind"), "taxonomy.record_kind") != "outage_taxonomy_alpha":
        findings.append(
            Finding(
                severity="error",
                check_id="taxonomy.record_kind.invalid",
                message="taxonomy.record_kind must be outage_taxonomy_alpha.",
                remediation="Restore the canonical record kind.",
                ref=DEFAULT_TAXONOMY_REL,
            )
        )

    for idx, ref in enumerate(ensure_list(taxonomy.get("source_refs"), "taxonomy.source_refs")):
        validate_path_ref(repo_root, ensure_str(ref, f"taxonomy.source_refs[{idx}]"), "taxonomy.source_refs", findings)

    consumer = ensure_dict(taxonomy.get("first_consumer"), "taxonomy.first_consumer")
    validate_path_ref(
        repo_root,
        ensure_str(consumer.get("validator_ref"), "taxonomy.first_consumer.validator_ref"),
        "taxonomy.first_consumer.validator_ref",
        findings,
    )
    consumed_artifacts = ensure_list(
        consumer.get("consumed_artifacts"),
        "taxonomy.first_consumer.consumed_artifacts",
    )
    for idx, ref in enumerate(consumed_artifacts):
        validate_path_ref(
            repo_root,
            ensure_str(ref, f"taxonomy.first_consumer.consumed_artifacts[{idx}]"),
            "taxonomy.first_consumer.consumed_artifacts",
            findings,
        )

    export_safety = ensure_dict(taxonomy.get("export_safety"), "taxonomy.export_safety")
    validate_export_safety(export_safety, "taxonomy.export_safety", findings, DEFAULT_TAXONOMY_REL)
    if export_safety.get("exact_build_identity_required") is not True:
        findings.append(
            Finding(
                severity="error",
                check_id="taxonomy.export_safety.exact_build_identity_required",
                message="taxonomy.export_safety.exact_build_identity_required must be true.",
                remediation="Require exact-build identity on the proof path.",
                ref=DEFAULT_TAXONOMY_REL,
            )
        )

    controls = ensure_dict(taxonomy.get("rehearsal_controls"), "taxonomy.rehearsal_controls")
    for idx, ref in enumerate(ensure_list(controls.get("required_proof_artifacts"), "taxonomy.rehearsal_controls.required_proof_artifacts")):
        validate_path_ref(
            repo_root,
            ensure_str(ref, f"taxonomy.rehearsal_controls.required_proof_artifacts[{idx}]"),
            "taxonomy.rehearsal_controls.required_proof_artifacts",
            findings,
        )
    acceptance_states = set(ensure_list(controls.get("acceptance_states"), "taxonomy.rehearsal_controls.acceptance_states"))
    missing_acceptance = REQUIRED_ACCEPTANCE_STATES - acceptance_states
    if missing_acceptance:
        findings.append(
            Finding(
                severity="error",
                check_id="taxonomy.acceptance_states.missing",
                message=f"taxonomy is missing acceptance states: {sorted(missing_acceptance)}",
                remediation="Add the required acceptance state tokens to rehearsal_controls.acceptance_states.",
                ref=DEFAULT_TAXONOMY_REL,
            )
        )

    outage_classes: dict[str, dict[str, Any]] = {}
    for idx, raw_class in enumerate(ensure_list(taxonomy.get("outage_classes"), "taxonomy.outage_classes")):
        outage_class = ensure_dict(raw_class, f"taxonomy.outage_classes[{idx}]")
        class_id = ensure_str(outage_class.get("class_id"), f"taxonomy.outage_classes[{idx}].class_id")
        if class_id in outage_classes:
            findings.append(
                Finding(
                    severity="error",
                    check_id="taxonomy.outage_classes.duplicate",
                    message=f"duplicate outage class id: {class_id}",
                    remediation="Keep one canonical class row per outage class.",
                    ref=DEFAULT_TAXONOMY_REL,
                )
            )
        outage_classes[class_id] = outage_class

    missing_classes = set(REQUIRED_CLASS_PLANES) - set(outage_classes)
    extra_classes = set(outage_classes) - set(REQUIRED_CLASS_PLANES)
    if missing_classes or extra_classes:
        findings.append(
            Finding(
                severity="error",
                check_id="taxonomy.outage_classes.coverage",
                message=f"required={sorted(REQUIRED_CLASS_PLANES)}, found={sorted(outage_classes)}",
                remediation="Keep exactly the four required outage classes for this alpha taxonomy.",
                ref=DEFAULT_TAXONOMY_REL,
            )
        )

    for class_id, outage_class in outage_classes.items():
        expected_plane = REQUIRED_CLASS_PLANES.get(class_id)
        actual_plane = ensure_str(outage_class.get("primary_plane_class"), f"taxonomy.{class_id}.primary_plane_class")
        if expected_plane is not None and actual_plane != expected_plane:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"taxonomy.{class_id}.primary_plane_class",
                    message=f"{class_id} primary plane must be {expected_plane}, got {actual_plane}.",
                    remediation="Align the class with the fixed outage taxonomy.",
                    ref=DEFAULT_TAXONOMY_REL,
                )
            )

        distinguished = set(ensure_list(outage_class.get("distinguished_from"), f"taxonomy.{class_id}.distinguished_from"))
        expected_distinguished = set(REQUIRED_CLASS_PLANES) - {class_id}
        if not expected_distinguished.issubset(distinguished):
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"taxonomy.{class_id}.distinguished_from",
                    message=f"{class_id} must distinguish itself from {sorted(expected_distinguished)}.",
                    remediation="Add every other required outage class to distinguished_from.",
                    ref=DEFAULT_TAXONOMY_REL,
                )
            )

        states = ensure_dict(outage_class.get("affected_plane_states"), f"taxonomy.{class_id}.affected_plane_states")
        missing_plane_states = REQUIRED_PLANE_STATE_FIELDS - set(states)
        if missing_plane_states:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"taxonomy.{class_id}.affected_plane_states.missing",
                    message=f"{class_id} missing plane states: {sorted(missing_plane_states)}",
                    remediation="Declare local_core, control_plane, data_plane, and target_authority states.",
                    ref=DEFAULT_TAXONOMY_REL,
                )
            )

        posture = ensure_dict(outage_class.get("expected_product_posture"), f"taxonomy.{class_id}.expected_product_posture")
        missing_posture = REQUIRED_POSTURE_FIELDS - set(posture)
        if missing_posture:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"taxonomy.{class_id}.expected_product_posture.missing",
                    message=f"{class_id} missing posture fields: {sorted(missing_posture)}",
                    remediation="Add the expected product posture fields required by the rehearsal contract.",
                    ref=DEFAULT_TAXONOMY_REL,
                )
            )
        ensure_bool(posture.get("local_core_available"), f"taxonomy.{class_id}.expected_product_posture.local_core_available")
        ensure_bool(posture.get("boundary_review_required"), f"taxonomy.{class_id}.expected_product_posture.boundary_review_required")
        forbidden = ensure_list(posture.get("forbidden_postures"), f"taxonomy.{class_id}.expected_product_posture.forbidden_postures")
        if not forbidden:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"taxonomy.{class_id}.forbidden_postures.empty",
                    message=f"{class_id} must name forbidden postures.",
                    remediation="Add at least one forbidden posture that prevents overclaiming.",
                    ref=DEFAULT_TAXONOMY_REL,
                )
            )

        if class_id in {"control_plane_impairment", "full_target_loss"} and posture.get("boundary_review_required") is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"taxonomy.{class_id}.boundary_review_required",
                    message=f"{class_id} must require boundary or target review.",
                    remediation="Set boundary_review_required to true for authority-changing classes.",
                    ref=DEFAULT_TAXONOMY_REL,
                )
            )

        recovery_actions = ensure_list(outage_class.get("recovery_actions"), f"taxonomy.{class_id}.recovery_actions")
        if not recovery_actions:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"taxonomy.{class_id}.recovery_actions.empty",
                    message=f"{class_id} has no recovery actions.",
                    remediation="Add the class recovery action list.",
                    ref=DEFAULT_TAXONOMY_REL,
                )
            )
        seen_actions: set[str] = set()
        last_sequence = 0
        for action_idx, raw_action in enumerate(recovery_actions):
            action = ensure_dict(raw_action, f"taxonomy.{class_id}.recovery_actions[{action_idx}]")
            action_id = ensure_str(action.get("action_id"), f"taxonomy.{class_id}.recovery_actions[{action_idx}].action_id")
            if action_id in seen_actions:
                findings.append(
                    Finding(
                        severity="error",
                        check_id=f"taxonomy.{class_id}.recovery_actions.duplicate",
                        message=f"{class_id} duplicates recovery action {action_id}.",
                        remediation="Use each recovery action id once per class.",
                        ref=DEFAULT_TAXONOMY_REL,
                    )
                )
            seen_actions.add(action_id)
            sequence = ensure_int(action.get("sequence"), f"taxonomy.{class_id}.recovery_actions[{action_idx}].sequence")
            if sequence <= last_sequence:
                findings.append(
                    Finding(
                        severity="error",
                        check_id=f"taxonomy.{class_id}.recovery_actions.sequence",
                        message=f"{class_id} recovery action sequences must be increasing.",
                        remediation="Order recovery actions by increasing sequence.",
                        ref=DEFAULT_TAXONOMY_REL,
                    )
                )
            last_sequence = sequence
            ensure_bool(action.get("required"), f"taxonomy.{class_id}.recovery_actions[{action_idx}].required")

        for proof_idx, raw_proof in enumerate(ensure_list(outage_class.get("proof_requirements"), f"taxonomy.{class_id}.proof_requirements")):
            proof = ensure_dict(raw_proof, f"taxonomy.{class_id}.proof_requirements[{proof_idx}]")
            for ref_idx, ref in enumerate(ensure_list(proof.get("artifact_refs"), f"taxonomy.{class_id}.proof_requirements[{proof_idx}].artifact_refs")):
                validate_path_ref(
                    repo_root,
                    ensure_str(ref, f"taxonomy.{class_id}.proof_requirements[{proof_idx}].artifact_refs[{ref_idx}]"),
                    f"taxonomy.{class_id}.proof_requirements.artifact_refs",
                    findings,
                )

    return outage_classes


def validate_markdown_docs(repo_root: Path, plan_ref: str, examples_ref: str, findings: list[Finding]) -> None:
    plan_path = _resolve_existing_reference(
        repo_root,
        repo_root,
        plan_ref,
        "rehearsal plan",
        suffix=".md",
        require_file=True,
    )
    examples_path = _resolve_existing_reference(
        repo_root,
        repo_root,
        examples_ref,
        "plane examples",
        suffix=".md",
        require_file=True,
    )
    plan_text = _read_bounded_text(plan_path, repo_root, "rehearsal plan")
    examples_text = _read_bounded_text(examples_path, repo_root, "plane examples")

    for class_id in REQUIRED_CLASS_PLANES:
        if class_id not in plan_text:
            findings.append(
                Finding(
                    severity="error",
                    check_id="plan.outage_class.missing",
                    message=f"rehearsal plan does not mention {class_id}.",
                    remediation="Document every required outage class in the rehearsal plan.",
                    ref=plan_ref,
                )
            )
        if class_id not in examples_text:
            findings.append(
                Finding(
                    severity="error",
                    check_id="examples.outage_class.missing",
                    message=f"examples doc does not mention {class_id}.",
                    remediation="Document every required outage class in the examples companion.",
                    ref=examples_ref,
                )
            )

    for marker in REQUIRED_ROLE_MARKERS | REQUIRED_PLAN_MARKERS | REQUIRED_DOC_PATH_MARKERS:
        if marker not in plan_text:
            findings.append(
                Finding(
                    severity="error",
                    check_id="plan.required_marker.missing",
                    message=f"rehearsal plan missing required marker: {marker}",
                    remediation="Add owner, cadence, proof artifact, and validation command disclosures.",
                    ref=plan_ref,
                )
            )

    for marker in {"Control-plane", "Data-plane", "Target authority", "Local core"}:
        if marker not in examples_text:
            findings.append(
                Finding(
                    severity="error",
                    check_id="examples.plane_marker.missing",
                    message=f"examples doc missing plane marker: {marker}",
                    remediation="Keep the examples focused on the plane split.",
                    ref=examples_ref,
                )
            )


def load_fixture_case(repo_root: Path, path: Path, case_ref: str) -> dict[str, Any]:
    payload = render_yaml_as_json(path, repo_root, "fixture case")
    return ensure_dict(payload, case_ref)


def validate_fixture_manifest(
    repo_root: Path,
    manifest: dict[str, Any],
    outage_classes: dict[str, dict[str, Any]],
    findings: list[Finding],
    manifest_ref: str,
    manifest_path: Path,
) -> list[dict[str, Any]]:
    if ensure_int(manifest.get("schema_version"), "fixture_manifest.schema_version") != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="fixture_manifest.schema_version.unsupported",
                message="fixture manifest schema_version must be 1.",
                remediation="Update the validator with any schema change.",
                ref=manifest_ref,
            )
        )
    if ensure_str(manifest.get("status"), "fixture_manifest.status") != "protected":
        findings.append(
            Finding(
                severity="error",
                check_id="fixture_manifest.status.not_protected",
                message="fixture manifest status must be protected.",
                remediation="Mark the rehearsal fixture lane as protected.",
                ref=manifest_ref,
            )
        )

    for label in ("taxonomy_ref", "rehearsal_plan_ref", "examples_ref", "proof_packet_ref", "validator_ref"):
        validate_path_ref(repo_root, ensure_str(manifest.get(label), f"fixture_manifest.{label}"), f"fixture_manifest.{label}", findings)

    required_ids = set(ensure_list(manifest.get("required_outage_class_ids"), "fixture_manifest.required_outage_class_ids"))
    if required_ids != set(REQUIRED_CLASS_PLANES):
        findings.append(
            Finding(
                severity="error",
                check_id="fixture_manifest.required_outage_class_ids",
                message=f"fixture manifest required classes must be {sorted(REQUIRED_CLASS_PLANES)}, got {sorted(required_ids)}.",
                remediation="Keep protected fixture coverage aligned to the taxonomy.",
                ref=manifest_ref,
            )
        )

    acceptance_states = set(ensure_list(manifest.get("acceptance_states"), "fixture_manifest.acceptance_states"))
    missing_acceptance = REQUIRED_ACCEPTANCE_STATES - acceptance_states
    if missing_acceptance:
        findings.append(
            Finding(
                severity="error",
                check_id="fixture_manifest.acceptance_states.missing",
                message=f"fixture manifest missing acceptance states: {sorted(missing_acceptance)}",
                remediation="Add the required acceptance state tokens.",
                ref=manifest_ref,
            )
        )

    validate_export_safety(
        ensure_dict(manifest.get("export_safety"), "fixture_manifest.export_safety"),
        "fixture_manifest.export_safety",
        findings,
        manifest_ref,
    )

    fixture_dir = manifest_path.parent
    cases: list[dict[str, Any]] = []
    seen_case_classes: set[str] = set()
    for idx, raw_case_ref in enumerate(ensure_list(manifest.get("case_files"), "fixture_manifest.case_files")):
        case_ref = ensure_dict(raw_case_ref, f"fixture_manifest.case_files[{idx}]")
        file_name = ensure_str(case_ref.get("file"), f"fixture_manifest.case_files[{idx}].file")
        class_id = ensure_str(case_ref.get("outage_class_id"), f"fixture_manifest.case_files[{idx}].outage_class_id")
        expected_plane = ensure_str(
            case_ref.get("expected_primary_plane_class"),
            f"fixture_manifest.case_files[{idx}].expected_primary_plane_class",
        )
        seen_case_classes.add(class_id)
        try:
            case_path = _resolve_existing_reference(
                repo_root,
                fixture_dir,
                file_name,
                "fixture case reference",
                suffix=".yaml",
                require_file=True,
            )
            case_path_ref = case_path.relative_to(repo_root).as_posix()
            case = load_fixture_case(repo_root, case_path, case_path_ref)
        except InputContractError:
            findings.append(
                Finding(
                    severity="error",
                    check_id="fixture_manifest.case_file.rejected",
                    message="fixture case failed the protected fixture ingestion contract.",
                    remediation="Use bounded UTF-8 YAML below the protected fixture directory without path redirects.",
                    ref=manifest_ref,
                )
            )
            continue

        cases.append(case)
        validate_fixture_case(
            repo_root,
            case,
            class_id,
            expected_plane,
            outage_classes,
            findings,
            case_path_ref,
        )

    if seen_case_classes != set(REQUIRED_CLASS_PLANES):
        findings.append(
            Finding(
                severity="error",
                check_id="fixture_manifest.case_class_coverage",
                message=f"fixture cases must cover {sorted(REQUIRED_CLASS_PLANES)}, got {sorted(seen_case_classes)}.",
                remediation="Add protected cases for every required outage class.",
                ref=manifest_ref,
            )
        )

    return cases


def validate_fixture_case(
    repo_root: Path,
    case: dict[str, Any],
    expected_class_id: str,
    expected_plane: str,
    outage_classes: dict[str, dict[str, Any]],
    findings: list[Finding],
    case_ref: str,
) -> None:
    if ensure_int(case.get("schema_version"), f"{case_ref}.schema_version") != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="fixture_case.schema_version.unsupported",
                message=f"{case_ref} schema_version must be 1.",
                remediation="Update the validator with any fixture schema change.",
                ref=case_ref,
            )
        )
    if ensure_str(case.get("record_kind"), f"{case_ref}.record_kind") != "backup_restore_failover_rehearsal_case":
        findings.append(
            Finding(
                severity="error",
                check_id="fixture_case.record_kind.invalid",
                message=f"{case_ref} record_kind must be backup_restore_failover_rehearsal_case.",
                remediation="Restore the protected fixture record kind.",
                ref=case_ref,
            )
        )
    class_id = ensure_str(case.get("outage_class_id"), f"{case_ref}.outage_class_id")
    if class_id != expected_class_id:
        findings.append(
            Finding(
                severity="error",
                check_id="fixture_case.outage_class_id.mismatch",
                message=f"{case_ref} class id {class_id} does not match manifest {expected_class_id}.",
                remediation="Align the fixture case with the manifest row.",
                ref=case_ref,
            )
        )
    primary_plane = ensure_str(case.get("primary_plane_class"), f"{case_ref}.primary_plane_class")
    if primary_plane != expected_plane:
        findings.append(
            Finding(
                severity="error",
                check_id="fixture_case.primary_plane_class.mismatch",
                message=f"{case_ref} primary plane {primary_plane} does not match manifest {expected_plane}.",
                remediation="Align the fixture case plane with the manifest row.",
                ref=case_ref,
            )
        )
    taxonomy_class = outage_classes.get(class_id)
    if taxonomy_class and primary_plane != taxonomy_class.get("primary_plane_class"):
        findings.append(
            Finding(
                severity="error",
                check_id="fixture_case.primary_plane_class.taxonomy_mismatch",
                message=f"{case_ref} primary plane differs from taxonomy row for {class_id}.",
                remediation="Keep fixture primary plane aligned with the taxonomy.",
                ref=case_ref,
            )
        )

    for idx, ref in enumerate(ensure_list(case.get("source_refs"), f"{case_ref}.source_refs")):
        validate_path_ref(repo_root, ensure_str(ref, f"{case_ref}.source_refs[{idx}]"), f"{case_ref}.source_refs", findings)

    plane_observation = ensure_dict(case.get("plane_observation"), f"{case_ref}.plane_observation")
    missing_plane_fields = REQUIRED_CASE_PLANE_FIELDS - set(plane_observation)
    if missing_plane_fields:
        findings.append(
            Finding(
                severity="error",
                check_id="fixture_case.plane_observation.missing",
                message=f"{case_ref} missing plane observation fields: {sorted(missing_plane_fields)}",
                remediation="Declare all required plane observation fields.",
                ref=case_ref,
            )
        )
    else:
        ensure_bool(plane_observation.get("target_reachable"), f"{case_ref}.plane_observation.target_reachable")

    posture = ensure_dict(case.get("expected_product_posture"), f"{case_ref}.expected_product_posture")
    for field_name in (
        "posture_class",
        "local_core_available",
        "must_surface_local_safe_baseline",
        "must_label_cached_or_optional_state",
        "boundary_review_required",
        "restore_claim_class",
    ):
        if field_name not in posture:
            findings.append(
                Finding(
                    severity="error",
                    check_id="fixture_case.expected_product_posture.missing",
                    message=f"{case_ref} missing expected_product_posture.{field_name}.",
                    remediation="Add the required fixture posture field.",
                    ref=case_ref,
                )
            )
    for bool_field in (
        "local_core_available",
        "must_surface_local_safe_baseline",
        "must_label_cached_or_optional_state",
        "boundary_review_required",
    ):
        if bool_field in posture:
            ensure_bool(posture.get(bool_field), f"{case_ref}.expected_product_posture.{bool_field}")

    if taxonomy_class:
        taxonomy_posture = ensure_dict(
            taxonomy_class.get("expected_product_posture"),
            f"taxonomy.{class_id}.expected_product_posture",
        )
        if posture.get("posture_class") != taxonomy_posture.get("posture_class"):
            findings.append(
                Finding(
                    severity="error",
                    check_id="fixture_case.posture_class.taxonomy_mismatch",
                    message=f"{case_ref} posture class differs from taxonomy row {class_id}.",
                    remediation="Align fixture posture with the taxonomy.",
                    ref=case_ref,
                )
            )

        taxonomy_action_ids = {
            ensure_str(action.get("action_id"), f"taxonomy.{class_id}.recovery_actions.action_id")
            for action in ensure_list(taxonomy_class.get("recovery_actions"), f"taxonomy.{class_id}.recovery_actions")
            if isinstance(action, dict)
        }
        case_action_ids = {
            ensure_str(action, f"{case_ref}.expected_recovery_actions[]")
            for action in ensure_list(case.get("expected_recovery_actions"), f"{case_ref}.expected_recovery_actions")
        }
        missing_actions = case_action_ids - taxonomy_action_ids
        if missing_actions:
            findings.append(
                Finding(
                    severity="error",
                    check_id="fixture_case.expected_recovery_actions.unknown",
                    message=f"{case_ref} references recovery actions not in taxonomy: {sorted(missing_actions)}",
                    remediation="Use only action ids declared on the class recovery action list.",
                    ref=case_ref,
                )
            )
        if not case_action_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="fixture_case.expected_recovery_actions.empty",
                    message=f"{case_ref} has no expected recovery actions.",
                    remediation="Name the recovery actions this case exercises.",
                    ref=case_ref,
                )
            )

    for idx, ref in enumerate(ensure_list(case.get("proof_artifacts"), f"{case_ref}.proof_artifacts")):
        validate_path_ref(repo_root, ensure_str(ref, f"{case_ref}.proof_artifacts[{idx}]"), f"{case_ref}.proof_artifacts", findings)

    assertions = ensure_dict(case.get("acceptance_assertions"), f"{case_ref}.acceptance_assertions")
    if assertions.get("exact_build_identity_required") is not True:
        findings.append(
            Finding(
                severity="error",
                check_id="fixture_case.exact_build_identity_required",
                message=f"{case_ref} must require exact-build identity.",
                remediation="Set acceptance_assertions.exact_build_identity_required to true.",
                ref=case_ref,
            )
        )

    validate_export_safety(
        ensure_dict(case.get("export_safety"), f"{case_ref}.export_safety"),
        f"{case_ref}.export_safety",
        findings,
        case_ref,
        metadata_only_required=True,
    )

    if class_id == "full_target_loss":
        if plane_observation.get("target_reachable") is not False:
            findings.append(
                Finding(
                    severity="error",
                    check_id="fixture_case.full_target_loss.target_reachable",
                    message="full_target_loss fixture must set target_reachable to false.",
                    remediation="Keep full target loss distinct from data-plane or control-plane impairment.",
                    ref=case_ref,
                )
            )
        restore_claim = ensure_str(posture.get("restore_claim_class"), f"{case_ref}.expected_product_posture.restore_claim_class")
        if restore_claim == "exact_without_evidence" or "requires" not in restore_claim:
            findings.append(
                Finding(
                    severity="error",
                    check_id="fixture_case.full_target_loss.restore_claim",
                    message="full_target_loss must not imply exact restore without matching evidence.",
                    remediation="Use a restore claim that requires target/source evidence review.",
                    ref=case_ref,
                )
            )


def build_support_projection(taxonomy: dict[str, Any]) -> dict[str, Any]:
    rows: list[dict[str, Any]] = []
    for outage_class in ensure_list(taxonomy.get("outage_classes"), "taxonomy.outage_classes"):
        row = ensure_dict(outage_class, "taxonomy.outage_classes[]")
        posture = ensure_dict(row.get("expected_product_posture"), "outage_class.expected_product_posture")
        proof_refs: list[str] = []
        for proof in ensure_list(row.get("proof_requirements"), "outage_class.proof_requirements"):
            proof_row = ensure_dict(proof, "outage_class.proof_requirements[]")
            proof_refs.extend(
                ensure_str(ref, "proof.artifact_refs[]")
                for ref in ensure_list(proof_row.get("artifact_refs"), "proof.artifact_refs")
            )
        rows.append(
            {
                "class_id": row.get("class_id"),
                "primary_plane_class": row.get("primary_plane_class"),
                "posture_class": posture.get("posture_class"),
                "local_core_available": posture.get("local_core_available"),
                "boundary_review_required": posture.get("boundary_review_required"),
                "recovery_action_count": len(ensure_list(row.get("recovery_actions"), "outage_class.recovery_actions")),
                "proof_artifact_refs": proof_refs,
                "redaction_class": taxonomy.get("export_safety", {}).get("redaction_default_class"),
                "raw_payload_exported": False,
            }
        )
    return {
        "record_kind": "backup_restore_failover_support_projection",
        "schema_version": 1,
        "taxonomy_id": taxonomy.get("taxonomy_id"),
        "projection_class": "metadata_only_support_release_review",
        "exact_build_identity_required": taxonomy.get("export_safety", {}).get("exact_build_identity_required"),
        "rows": rows,
    }


def render_human_summary(findings: list[Finding], projection: dict[str, Any]) -> str:
    errors = [finding for finding in findings if finding.severity == "error"]
    warnings = [finding for finding in findings if finding.severity == "warning"]
    lines = [
        "[backup-restore-failover] validation summary",
        f"  rows: {len(projection['rows'])}",
        f"  errors: {len(errors)}",
        f"  warnings: {len(warnings)}",
    ]
    for finding in findings:
        lines.append(
            f"  {finding.severity}: {finding.check_id}: {finding.message}"
            + (f" ({finding.ref})" if finding.ref else "")
        )
    if not findings:
        lines.append("  status: ok")
    return "\n".join(lines) + "\n"


def _same_directory_identity(left: os.stat_result, right: os.stat_result) -> bool:
    return (
        stat.S_ISDIR(left.st_mode)
        and stat.S_ISDIR(right.st_mode)
        and left.st_dev == right.st_dev
        and left.st_ino == right.st_ino
    )


def _prepare_report_parent(repo_root: Path, reference: str) -> tuple[Path, str]:
    parts = _safe_relative_parts(reference, "validation report", suffix=".json")
    current = repo_root
    for part in parts[:-1]:
        candidate = current / part
        try:
            candidate.mkdir(mode=0o755)
        except FileExistsError:
            pass
        except OSError as exc:
            raise InputContractError("validation report directory is unavailable") from exc
        try:
            metadata = os.lstat(candidate)
        except OSError as exc:
            raise InputContractError("validation report directory is unavailable") from exc
        if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
            raise InputContractError("validation report directory uses an unsafe path component")
        try:
            current = candidate.resolve(strict=True)
        except OSError as exc:
            raise InputContractError("validation report directory is unavailable") from exc
        _require_under(current, repo_root, "validation report")
    return current, parts[-1]


def _write_validation_report(repo_root: Path, reference: str, report: dict[str, Any]) -> None:
    parent, file_name = _prepare_report_parent(repo_root, reference)
    try:
        parent_before = os.lstat(parent)
        directory_fd = os.open(
            parent,
            os.O_RDONLY | getattr(os, "O_DIRECTORY", 0) | getattr(os, "O_NOFOLLOW", 0),
        )
    except OSError as exc:
        raise InputContractError("validation report directory is unavailable") from exc

    temporary_name: str | None = None
    try:
        opened_parent = os.fstat(directory_fd)
        if not _same_directory_identity(parent_before, opened_parent):
            raise InputContractError("validation report directory changed before writing")
        try:
            existing = os.stat(file_name, dir_fd=directory_fd, follow_symlinks=False)
        except FileNotFoundError:
            existing = None
        except OSError as exc:
            raise InputContractError("validation report target is unavailable") from exc
        if existing is not None and (stat.S_ISLNK(existing.st_mode) or not stat.S_ISREG(existing.st_mode)):
            raise InputContractError("validation report target is not a regular file")

        body = (json.dumps(report, indent=2, sort_keys=True) + "\n").encode("utf-8")
        if len(body) > MAX_INPUT_BYTES:
            raise InputContractError("validation report exceeds the output-byte limit")

        flags = os.O_WRONLY | os.O_CREAT | os.O_EXCL | getattr(os, "O_NOFOLLOW", 0)
        for _ in range(8):
            temporary_name = f".{file_name}.{os.getpid()}.{secrets.token_hex(8)}.tmp"
            try:
                output_fd = os.open(temporary_name, flags, 0o644, dir_fd=directory_fd)
                break
            except FileExistsError:
                temporary_name = None
            except OSError as exc:
                raise InputContractError("validation report temporary target is unavailable") from exc
        else:
            raise InputContractError("validation report temporary target is unavailable")

        try:
            with os.fdopen(output_fd, "wb") as output:
                output.write(body)
                output.flush()
                os.fsync(output.fileno())
        except OSError as exc:
            raise InputContractError("validation report could not be written") from exc

        try:
            os.replace(
                temporary_name,
                file_name,
                src_dir_fd=directory_fd,
                dst_dir_fd=directory_fd,
            )
        except OSError as exc:
            raise InputContractError("validation report could not be installed") from exc
        temporary_name = None
    finally:
        if temporary_name is not None:
            try:
                os.unlink(temporary_name, dir_fd=directory_fd)
            except OSError:
                pass
        try:
            os.close(directory_fd)
        except OSError:
            pass


def main() -> int:
    args = parse_args()
    try:
        try:
            repo_root = Path(args.repo_root).resolve(strict=True)
        except OSError as exc:
            raise InputContractError("repository root is unavailable") from exc
        if not repo_root.is_dir():
            raise InputContractError("repository root is not a directory")

        taxonomy_ref = args.taxonomy
        plan_ref = args.plan
        examples_ref = args.examples
        fixture_manifest_ref = args.fixture_manifest

        taxonomy_path = _resolve_existing_reference(
            repo_root,
            repo_root,
            taxonomy_ref,
            "taxonomy input",
            suffix=".yaml",
            require_file=True,
        )
        manifest_path = _resolve_existing_reference(
            repo_root,
            repo_root,
            fixture_manifest_ref,
            "fixture manifest input",
            suffix=".yaml",
            require_file=True,
        )
        taxonomy = ensure_dict(
            render_yaml_as_json(taxonomy_path, repo_root, "taxonomy input"),
            "taxonomy",
        )
        manifest = ensure_dict(
            render_yaml_as_json(manifest_path, repo_root, "fixture manifest input"),
            "fixture_manifest",
        )

        findings: list[Finding] = []
        validate_path_ref(repo_root, taxonomy_ref, "args.taxonomy", findings)
        validate_path_ref(repo_root, plan_ref, "args.plan", findings)
        validate_path_ref(repo_root, examples_ref, "args.examples", findings)
        validate_path_ref(repo_root, fixture_manifest_ref, "args.fixture_manifest", findings)

        outage_classes = validate_taxonomy(repo_root, taxonomy, findings)
        validate_markdown_docs(repo_root, plan_ref, examples_ref, findings)
        cases = validate_fixture_manifest(
            repo_root,
            manifest,
            outage_classes,
            findings,
            fixture_manifest_ref,
            manifest_path,
        )
        projection = build_support_projection(taxonomy)

        report = {
            "record_kind": "backup_restore_failover_alpha_validation_report",
            "schema_version": 1,
            "status": "failed" if any(finding.severity == "error" for finding in findings) else "passed",
            "validated_artifacts": {
                "taxonomy": taxonomy_ref,
                "plan": plan_ref,
                "examples": examples_ref,
                "fixture_manifest": fixture_manifest_ref,
                "fixture_case_count": len(cases),
            },
            "required_outage_class_ids": sorted(REQUIRED_CLASS_PLANES),
            "support_projection": projection,
            "findings": [finding.as_report() for finding in findings],
        }

        sys.stdout.write(render_human_summary(findings, projection))
        if args.render_support_projection:
            sys.stdout.write(json.dumps(projection, indent=2, sort_keys=True) + "\n")
        if args.report:
            _write_validation_report(repo_root, args.report, report)

        return 1 if any(finding.severity == "error" for finding in findings) else 0
    except InputContractError as exc:
        sys.stderr.write(f"[backup-restore-failover] protected input rejected: {exc}\n")
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
