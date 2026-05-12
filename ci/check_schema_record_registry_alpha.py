#!/usr/bin/env python3
"""Validate and render the alpha schema and record registries."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_SCHEMA_REGISTRY_REL = "artifacts/governance/schema_registry_alpha.yaml"
DEFAULT_RECORD_REGISTRY_REL = "artifacts/governance/record_class_registry_alpha.yaml"
DEFAULT_BASE_RECORD_REGISTRY_REL = "artifacts/governance/record_class_registry.yaml"
DEFAULT_FIXTURE_MANIFEST_REL = "fixtures/governance/schema_record_registry_alpha_cases/manifest.yaml"
DEFAULT_ALPHA_SCOREBOARD_REL = "artifacts/milestones/m2/exit_gate_scoreboard.yaml"

REQUIRED_CLASS_SCOPES = {
    "durable_state",
    "support_bundle",
    "portable_package",
    "managed_copy",
    "export_packet",
    "receipt",
}

REQUIRED_SCHEMA_ROLES = {
    "durable_state_schema",
    "support_export_packet_schema",
    "portable_package_schema",
    "managed_copy_schema",
    "export_packet_schema",
    "receipt_schema",
}

REQUIRED_PROJECTION_FIELDS = {
    "record_class_id",
    "class_scope",
    "owner_dri",
    "schema_rows",
    "local_truth_authority",
    "managed_copy_posture",
    "retention_label",
    "export_semantics",
    "delete_semantics",
    "hold_semantics",
}

REQUIRED_FIXTURE_STATES = {
    "durable_state_schema_and_record_coverage",
    "support_bundle_export_packet_coverage",
    "portable_package_export_coverage",
    "managed_copy_schema_placeholder_and_record_coverage",
    "delete_export_hold_semantics",
    "first_consumer_projection",
}

REQUIRED_ALPHA_SCOREBOARD_ROW = "scoreboard_row:alpha_scope.schema_record_registry"

PATH_LIKE_SUFFIXES = (".yaml", ".yml", ".json", ".md", ".toml", ".rs", ".py")
ID_PREFIXES = (
    "alpha_wedge:",
    "policy:",
    "proof_packet:",
    "record_alpha:",
    "schema_alpha:",
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
    parser.add_argument("--schema-registry", default=DEFAULT_SCHEMA_REGISTRY_REL)
    parser.add_argument("--record-registry", default=DEFAULT_RECORD_REGISTRY_REL)
    parser.add_argument("--base-record-registry", default=DEFAULT_BASE_RECORD_REGISTRY_REL)
    parser.add_argument("--fixture-manifest", default=DEFAULT_FIXTURE_MANIFEST_REL)
    parser.add_argument("--alpha-scoreboard", default=DEFAULT_ALPHA_SCOREBOARD_REL)
    parser.add_argument("--report", default=None)
    parser.add_argument(
        "--render-support-export-projection",
        action="store_true",
        help="Print the support/export-safe projection consumed by docs, CLI, and support surfaces.",
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


def ensure_bool(value: Any, label: str) -> bool:
    if not isinstance(value, bool):
        raise SystemExit(f"{label} must be a boolean")
    return value


def parse_iso_date(value: str, label: str, findings: list[Finding], ref: str | None = None) -> None:
    try:
        dt.date.fromisoformat(value)
    except ValueError:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.invalid_date",
                message=f"{label} must be a YYYY-MM-DD date, got {value!r}",
                remediation="Use an ISO-8601 date without a time component.",
                ref=ref,
            )
        )


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


def validate_path_ref(repo_root: Path, ref: str, label: str, findings: list[Finding]) -> None:
    if looks_like_path(ref) and not artifact_ref_exists(repo_root, ref):
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.missing_ref",
                message=f"{label} does not resolve: {ref}",
                remediation="Fix the path or seed the referenced artifact.",
                ref=ref,
            )
        )


def validate_header(payload: dict[str, Any], label: str, findings: list[Finding]) -> None:
    schema_version = ensure_int(payload.get("schema_version"), f"{label}.schema_version")
    if schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.schema_version.unsupported",
                message=f"{label}.schema_version must be 1, got {schema_version}",
                remediation="Update this validator in the same change that bumps the artifact schema.",
            )
        )
    parse_iso_date(ensure_str(payload.get("as_of"), f"{label}.as_of"), f"{label}.as_of", findings)
    ensure_str(payload.get("owner"), f"{label}.owner")
    ensure_str(payload.get("registry_id"), f"{label}.registry_id")


def collect_scoreboard_ids(scoreboard: dict[str, Any]) -> set[str]:
    ids: set[str] = set()
    for idx, raw_row in enumerate(ensure_list(scoreboard.get("scoreboard_rows"), "scoreboard.scoreboard_rows")):
        row = ensure_dict(raw_row, f"scoreboard.scoreboard_rows[{idx}]")
        ids.add(ensure_str(row.get("row_id"), f"scoreboard.scoreboard_rows[{idx}].row_id"))
    return ids


def collect_base_record_class_ids(registry: dict[str, Any]) -> set[str]:
    ids: set[str] = set()
    for idx, raw_row in enumerate(ensure_list(registry.get("rows"), "base_record_registry.rows")):
        row = ensure_dict(raw_row, f"base_record_registry.rows[{idx}]")
        ids.add(ensure_str(row.get("record_class_id"), f"base_record_registry.rows[{idx}].record_class_id"))
    return ids


def validate_first_consumers(
    repo_root: Path,
    rows: list[Any],
    label: str,
    findings: list[Finding],
) -> None:
    if not rows:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.first_consumers.missing",
                message=f"{label} must declare at least one first consumer",
                remediation="Add the CLI/support/export projection consumer row.",
            )
        )
        return
    for idx, raw_consumer in enumerate(rows):
        consumer = ensure_dict(raw_consumer, f"{label}.first_consumers[{idx}]")
        ensure_str(consumer.get("consumer_id"), f"{label}.first_consumers[{idx}].consumer_id")
        consumer_ref = ensure_str(consumer.get("consumer_ref"), f"{label}.first_consumers[{idx}].consumer_ref")
        validate_path_ref(repo_root, consumer_ref, f"{label}.first_consumers.consumer_ref", findings)
        ensure_str(consumer.get("command"), f"{label}.first_consumers[{idx}].command")
        rendered_fields = {
            ensure_str(item, f"{label}.first_consumers[{idx}].required_rendered_fields[]")
            for item in ensure_list(consumer.get("required_rendered_fields"), f"{label}.first_consumers[{idx}].required_rendered_fields")
        }
        missing = REQUIRED_PROJECTION_FIELDS - rendered_fields
        if missing:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"{label}.first_consumers.rendered_fields_missing",
                    message="first consumer does not render all required support/export fields",
                    remediation="Add local truth, managed-copy, retention, export, delete, and hold fields.",
                    ref=consumer_ref,
                    details={"missing": sorted(missing)},
                )
            )


def validate_schema_registry(
    repo_root: Path,
    registry: dict[str, Any],
    scoreboard_ids: set[str],
    findings: list[Finding],
) -> dict[str, dict[str, Any]]:
    validate_header(registry, "schema_registry", findings)
    for field_name in (
        "human_entrypoint_ref",
        "record_registry_ref",
        "alpha_wedge_matrix_ref",
        "alpha_exit_scoreboard_ref",
    ):
        validate_path_ref(repo_root, ensure_str(registry.get(field_name), f"schema_registry.{field_name}"), f"schema_registry.{field_name}", findings)
    for ref in ensure_list(registry.get("source_registry_refs"), "schema_registry.source_registry_refs"):
        validate_path_ref(repo_root, ensure_str(ref, "schema_registry.source_registry_refs[]"), "schema_registry.source_registry_refs", findings)

    status_vocabulary = set(ensure_list(registry.get("schema_status_vocabulary"), "schema_registry.schema_status_vocabulary"))
    role_vocabulary = set(ensure_list(registry.get("schema_role_vocabulary"), "schema_registry.schema_role_vocabulary"))
    surface_vocabulary = set(ensure_list(registry.get("consumer_surface_vocabulary"), "schema_registry.consumer_surface_vocabulary"))
    validate_first_consumers(repo_root, ensure_list(registry.get("first_consumers"), "schema_registry.first_consumers"), "schema_registry", findings)

    validator = ensure_dict(registry.get("validator"), "schema_registry.validator")
    validate_path_ref(repo_root, ensure_str(validator.get("script_ref"), "schema_registry.validator.script_ref"), "schema_registry.validator.script_ref", findings)
    ensure_str(validator.get("command"), "schema_registry.validator.command")
    ensure_str(validator.get("consumer_projection_command"), "schema_registry.validator.consumer_projection_command")

    if REQUIRED_ALPHA_SCOREBOARD_ROW not in scoreboard_ids:
        findings.append(
            Finding(
                severity="error",
                check_id="alpha_scoreboard.required_row_missing",
                message=f"alpha scoreboard is missing {REQUIRED_ALPHA_SCOREBOARD_ROW}",
                remediation="Restore the schema/record-registry scoreboard row before claiming registry readiness.",
                ref=REQUIRED_ALPHA_SCOREBOARD_ROW,
            )
        )

    rows: dict[str, dict[str, Any]] = {}
    for idx, raw_row in enumerate(ensure_list(registry.get("schema_rows"), "schema_registry.schema_rows")):
        row = ensure_dict(raw_row, f"schema_registry.schema_rows[{idx}]")
        row_id = ensure_str(row.get("row_id"), f"schema_registry.schema_rows[{idx}].row_id")
        if row_id in rows:
            findings.append(
                Finding(
                    severity="error",
                    check_id="schema.rows.duplicate",
                    message=f"duplicate schema row id: {row_id}",
                    remediation="Use one row per schema or placeholder id.",
                    ref=row_id,
                )
            )
        rows[row_id] = row

        missing_fields = sorted(
            {
                "schema_row_kind",
                "schema_row_version",
                "row_id",
                "title",
                "schema_role",
                "schema_ref",
                "schema_status",
                "owner_dri",
                "schema_version_pin",
                "record_class_refs",
                "truth_surfaces",
                "consumer_refs",
            }
            - set(row)
        )
        if missing_fields:
            findings.append(
                Finding(
                    severity="error",
                    check_id="schema.rows.missing_fields",
                    message=f"schema row is missing required fields: {row_id}",
                    remediation="Add owner, role, status, schema ref, record class refs, truth surfaces, and consumers.",
                    ref=row_id,
                    details={"missing": missing_fields},
                )
            )
            continue

        if ensure_int(row.get("schema_row_version"), f"schema_registry.schema_rows[{idx}].schema_row_version") != 1:
            findings.append(
                Finding(
                    severity="error",
                    check_id="schema.rows.schema_row_version.unsupported",
                    message=f"{row_id} must use schema_row_version 1",
                    remediation="Update this validator with any row-shape version bump.",
                    ref=row_id,
                )
            )
        schema_role = ensure_str(row.get("schema_role"), f"schema_registry.schema_rows[{idx}].schema_role")
        if schema_role not in role_vocabulary:
            findings.append(
                Finding(
                    severity="error",
                    check_id="schema.rows.role_invalid",
                    message=f"{row_id} uses unknown schema_role: {schema_role}",
                    remediation="Use a role from schema_role_vocabulary.",
                    ref=row_id,
                )
            )
        schema_status = ensure_str(row.get("schema_status"), f"schema_registry.schema_rows[{idx}].schema_status")
        if schema_status not in status_vocabulary:
            findings.append(
                Finding(
                    severity="error",
                    check_id="schema.rows.status_invalid",
                    message=f"{row_id} uses unknown schema_status: {schema_status}",
                    remediation="Use a status from schema_status_vocabulary.",
                    ref=row_id,
                )
            )
        schema_ref = ensure_str(row.get("schema_ref"), f"schema_registry.schema_rows[{idx}].schema_ref")
        if schema_status == "placeholder_schema":
            for field_name in ("placeholder_owner_dri", "placeholder_until", "placeholder_exit_criteria"):
                if not row.get(field_name):
                    findings.append(
                        Finding(
                            severity="error",
                            check_id=f"schema.rows.{field_name}_missing",
                            message=f"{row_id} is a placeholder but lacks {field_name}",
                            remediation="Add placeholder owner, expiry, and exit criteria.",
                            ref=row_id,
                        )
                    )
            if row.get("placeholder_until"):
                parse_iso_date(ensure_str(row.get("placeholder_until"), f"schema_registry.schema_rows[{idx}].placeholder_until"), "schema.rows.placeholder_until", findings, row_id)
        else:
            validate_path_ref(repo_root, schema_ref, "schema.rows.schema_ref", findings)
        ensure_str(row.get("owner_dri"), f"schema_registry.schema_rows[{idx}].owner_dri")
        ensure_int(row.get("schema_version_pin"), f"schema_registry.schema_rows[{idx}].schema_version_pin")

        record_refs = ensure_list(row.get("record_class_refs"), f"schema_registry.schema_rows[{idx}].record_class_refs")
        if not record_refs:
            findings.append(
                Finding(
                    severity="error",
                    check_id="schema.rows.record_class_refs.empty",
                    message=f"{row_id} must bind to at least one alpha record row",
                    remediation="Add record_class_refs pointing into the alpha record registry.",
                    ref=row_id,
                )
            )
        for surface in ensure_list(row.get("truth_surfaces"), f"schema_registry.schema_rows[{idx}].truth_surfaces"):
            surface = ensure_str(surface, f"schema_registry.schema_rows[{idx}].truth_surfaces[]")
            if surface not in surface_vocabulary:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="schema.rows.truth_surface_invalid",
                        message=f"{row_id} uses unknown truth surface: {surface}",
                        remediation="Use a surface from consumer_surface_vocabulary.",
                        ref=row_id,
                    )
                )
        for ref in ensure_list(row.get("consumer_refs"), f"schema_registry.schema_rows[{idx}].consumer_refs"):
            validate_path_ref(repo_root, ensure_str(ref, f"schema_registry.schema_rows[{idx}].consumer_refs[]"), "schema.rows.consumer_refs", findings)

    observed_roles = {ensure_str(row.get("schema_role"), "schema_row.schema_role") for row in rows.values() if row.get("schema_role")}
    missing_roles = REQUIRED_SCHEMA_ROLES - observed_roles
    if missing_roles:
        findings.append(
            Finding(
                severity="error",
                check_id="schema.coverage.required_role_missing",
                message="schema registry is missing required schema roles",
                remediation="Add durable state, support export, portable package, managed copy, export packet, and receipt schema rows.",
                details={"missing": sorted(missing_roles)},
            )
        )
    return rows


def validate_record_registry(
    repo_root: Path,
    registry: dict[str, Any],
    schema_rows: dict[str, dict[str, Any]],
    base_record_ids: set[str],
    findings: list[Finding],
) -> dict[str, dict[str, Any]]:
    validate_header(registry, "record_registry", findings)
    for field_name in (
        "human_entrypoint_ref",
        "schema_registry_ref",
        "base_record_class_registry_ref",
        "alpha_wedge_matrix_ref",
        "alpha_exit_scoreboard_ref",
    ):
        validate_path_ref(repo_root, ensure_str(registry.get(field_name), f"record_registry.{field_name}"), f"record_registry.{field_name}", findings)
    for ref in ensure_list(registry.get("source_registry_refs"), "record_registry.source_registry_refs"):
        validate_path_ref(repo_root, ensure_str(ref, "record_registry.source_registry_refs[]"), "record_registry.source_registry_refs", findings)

    class_scopes = set(ensure_list(registry.get("class_scope_vocabulary"), "record_registry.class_scope_vocabulary"))
    authority_classes = set(ensure_list(registry.get("authority_class_vocabulary"), "record_registry.authority_class_vocabulary"))
    managed_copy_postures = set(ensure_list(registry.get("managed_copy_posture_vocabulary"), "record_registry.managed_copy_posture_vocabulary"))
    retention_labels = set(ensure_list(registry.get("retention_label_vocabulary"), "record_registry.retention_label_vocabulary"))
    delete_semantics = set(ensure_list(registry.get("delete_semantic_vocabulary"), "record_registry.delete_semantic_vocabulary"))
    export_semantics = set(ensure_list(registry.get("export_semantic_vocabulary"), "record_registry.export_semantic_vocabulary"))
    validate_first_consumers(repo_root, ensure_list(registry.get("first_consumers"), "record_registry.first_consumers"), "record_registry", findings)

    validator = ensure_dict(registry.get("validator"), "record_registry.validator")
    validate_path_ref(repo_root, ensure_str(validator.get("script_ref"), "record_registry.validator.script_ref"), "record_registry.validator.script_ref", findings)
    ensure_str(validator.get("command"), "record_registry.validator.command")
    ensure_str(validator.get("consumer_projection_command"), "record_registry.validator.consumer_projection_command")

    rows: dict[str, dict[str, Any]] = {}
    required_fields = {
        "row_kind",
        "row_version",
        "row_id",
        "record_class_id",
        "title",
        "class_scope",
        "owner_dri",
        "base_record_class_refs",
        "placeholder_record_class",
        "schema_row_refs",
        "local_truth",
        "retention",
        "hold_semantics",
        "delete_semantics",
        "export_semantics",
        "support_surface_refs",
        "product_surface_refs",
        "docs_refs",
    }
    for idx, raw_row in enumerate(ensure_list(registry.get("record_classes"), "record_registry.record_classes")):
        row = ensure_dict(raw_row, f"record_registry.record_classes[{idx}]")
        row_id = ensure_str(row.get("row_id"), f"record_registry.record_classes[{idx}].row_id")
        if row_id in rows:
            findings.append(
                Finding(
                    severity="error",
                    check_id="record.rows.duplicate",
                    message=f"duplicate record row id: {row_id}",
                    remediation="Use one alpha row per record class.",
                    ref=row_id,
                )
            )
        rows[row_id] = row
        missing_fields = sorted(required_fields - set(row))
        if missing_fields:
            findings.append(
                Finding(
                    severity="error",
                    check_id="record.rows.missing_fields",
                    message=f"record row is missing required fields: {row_id}",
                    remediation="Add schema refs and local truth, retention, hold, delete, and export semantics.",
                    ref=row_id,
                    details={"missing": missing_fields},
                )
            )
            continue

        if ensure_int(row.get("row_version"), f"record_registry.record_classes[{idx}].row_version") != 1:
            findings.append(
                Finding(
                    severity="error",
                    check_id="record.rows.row_version.unsupported",
                    message=f"{row_id} must use row_version 1",
                    remediation="Update this validator with any row-shape version bump.",
                    ref=row_id,
                )
            )
        class_scope = ensure_str(row.get("class_scope"), f"record_registry.record_classes[{idx}].class_scope")
        if class_scope not in class_scopes:
            findings.append(
                Finding(
                    severity="error",
                    check_id="record.rows.class_scope.invalid",
                    message=f"{row_id} uses unknown class_scope: {class_scope}",
                    remediation="Use a class scope from class_scope_vocabulary.",
                    ref=row_id,
                )
            )
        ensure_str(row.get("owner_dri"), f"record_registry.record_classes[{idx}].owner_dri")

        placeholder = ensure_bool(row.get("placeholder_record_class"), f"record_registry.record_classes[{idx}].placeholder_record_class")
        base_refs = [
            ensure_str(ref, f"record_registry.record_classes[{idx}].base_record_class_refs[]")
            for ref in ensure_list(row.get("base_record_class_refs"), f"record_registry.record_classes[{idx}].base_record_class_refs")
        ]
        if placeholder:
            ensure_str(row.get("placeholder_exit_criteria"), f"record_registry.record_classes[{idx}].placeholder_exit_criteria")
        elif not base_refs:
            findings.append(
                Finding(
                    severity="error",
                    check_id="record.rows.base_record_class_refs.empty",
                    message=f"{row_id} is not a placeholder but has no base record-class ref",
                    remediation="Reference the existing base record-class row or mark the alpha row as a placeholder.",
                    ref=row_id,
                )
            )
        for base_ref in base_refs:
            if base_ref not in base_record_ids:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="record.rows.base_record_class_refs.unknown",
                        message=f"{row_id} cites unknown base record class: {base_ref}",
                        remediation="Use a record_class_id from artifacts/governance/record_class_registry.yaml.",
                        ref=row_id,
                    )
                )

        schema_refs = ensure_list(row.get("schema_row_refs"), f"record_registry.record_classes[{idx}].schema_row_refs")
        if not schema_refs:
            findings.append(
                Finding(
                    severity="error",
                    check_id="record.rows.schema_row_refs.empty",
                    message=f"{row_id} has no schema refs",
                    remediation="Bind every record row to one or more alpha schema rows.",
                    ref=row_id,
                )
            )
        for ref in schema_refs:
            ref = ensure_str(ref, f"record_registry.record_classes[{idx}].schema_row_refs[]")
            if ref not in schema_rows:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="record.rows.schema_ref_unknown",
                        message=f"{row_id} cites unknown schema row: {ref}",
                        remediation="Add the schema row or correct the record row reference.",
                        ref=row_id,
                    )
                )

        local_truth = ensure_dict(row.get("local_truth"), f"record_registry.record_classes[{idx}].local_truth")
        authority = ensure_str(local_truth.get("authority_class"), f"record_registry.record_classes[{idx}].local_truth.authority_class")
        if authority not in authority_classes:
            findings.append(
                Finding(
                    severity="error",
                    check_id="record.rows.authority_class.invalid",
                    message=f"{row_id} uses unknown authority class: {authority}",
                    remediation="Use an authority class from authority_class_vocabulary.",
                    ref=row_id,
                )
            )
        managed_posture = ensure_str(local_truth.get("managed_copy_posture"), f"record_registry.record_classes[{idx}].local_truth.managed_copy_posture")
        if managed_posture not in managed_copy_postures:
            findings.append(
                Finding(
                    severity="error",
                    check_id="record.rows.managed_copy_posture_invalid",
                    message=f"{row_id} uses unknown managed copy posture: {managed_posture}",
                    remediation="Use a posture from managed_copy_posture_vocabulary.",
                    ref=row_id,
                )
            )
        ensure_str(local_truth.get("managed_copy_label"), f"record_registry.record_classes[{idx}].local_truth.managed_copy_label")

        retention = ensure_dict(row.get("retention"), f"record_registry.record_classes[{idx}].retention")
        retention_label = ensure_str(retention.get("retention_label"), f"record_registry.record_classes[{idx}].retention.retention_label")
        if retention_label not in retention_labels:
            findings.append(
                Finding(
                    severity="error",
                    check_id="record.rows.retention_label.invalid",
                    message=f"{row_id} uses unknown retention label: {retention_label}",
                    remediation="Use a label from retention_label_vocabulary.",
                    ref=row_id,
                )
            )
        for field_name in ("local_owner_ref", "managed_owner_ref", "trigger_class"):
            ensure_str(retention.get(field_name), f"record_registry.record_classes[{idx}].retention.{field_name}")
        for ref in ensure_list(retention.get("retention_artifact_refs"), f"record_registry.record_classes[{idx}].retention.retention_artifact_refs"):
            validate_path_ref(repo_root, ensure_str(ref, f"record_registry.record_classes[{idx}].retention.retention_artifact_refs[]"), "record.rows.retention_artifact_refs", findings)

        hold = ensure_dict(row.get("hold_semantics"), f"record_registry.record_classes[{idx}].hold_semantics")
        eligible = ensure_bool(hold.get("eligible"), f"record_registry.record_classes[{idx}].hold_semantics.eligible")
        hold_classes = ensure_list(hold.get("hold_classes"), f"record_registry.record_classes[{idx}].hold_semantics.hold_classes")
        if eligible and not hold_classes:
            findings.append(
                Finding(
                    severity="error",
                    check_id="record.rows.hold_semantics_missing",
                    message=f"{row_id} is hold eligible but has no hold classes",
                    remediation="Name the hold classes that can block completion.",
                    ref=row_id,
                )
            )
        if not eligible and hold_classes:
            findings.append(
                Finding(
                    severity="error",
                    check_id="record.rows.hold_semantics_inconsistent",
                    message=f"{row_id} is not hold eligible but lists hold classes",
                    remediation="Clear hold_classes or set eligible true.",
                    ref=row_id,
                )
            )

        delete = ensure_dict(row.get("delete_semantics"), f"record_registry.record_classes[{idx}].delete_semantics")
        for field_name in ("request_supported", "local_and_managed_actions_are_distinct", "hold_blocks_completion"):
            ensure_bool(delete.get(field_name), f"record_registry.record_classes[{idx}].delete_semantics.{field_name}")
        ensure_str(delete.get("completion_evidence"), f"record_registry.record_classes[{idx}].delete_semantics.completion_evidence")
        delete_classes = {
            ensure_str(item, f"record_registry.record_classes[{idx}].delete_semantics.semantic_classes[]")
            for item in ensure_list(delete.get("semantic_classes"), f"record_registry.record_classes[{idx}].delete_semantics.semantic_classes")
        }
        if not delete_classes:
            findings.append(
                Finding(
                    severity="error",
                    check_id="record.rows.delete_semantics_missing",
                    message=f"{row_id} has no delete semantic classes",
                    remediation="Declare local delete, managed delete, hold blocking, receipt, or invalidation semantics.",
                    ref=row_id,
                )
            )
        unknown_delete = sorted(delete_classes - delete_semantics)
        if unknown_delete:
            findings.append(
                Finding(
                    severity="error",
                    check_id="record.rows.delete_semantics.invalid",
                    message=f"{row_id} uses unknown delete semantic classes",
                    remediation="Use classes from delete_semantic_vocabulary.",
                    ref=row_id,
                    details={"unknown": unknown_delete},
                )
            )

        export = ensure_dict(row.get("export_semantics"), f"record_registry.record_classes[{idx}].export_semantics")
        ensure_str(export.get("availability"), f"record_registry.record_classes[{idx}].export_semantics.availability")
        ensure_bool(export.get("manifest_required"), f"record_registry.record_classes[{idx}].export_semantics.manifest_required")
        ensure_bool(export.get("local_export_copy_disclosed"), f"record_registry.record_classes[{idx}].export_semantics.local_export_copy_disclosed")
        export_classes = {
            ensure_str(item, f"record_registry.record_classes[{idx}].export_semantics.semantic_classes[]")
            for item in ensure_list(export.get("semantic_classes"), f"record_registry.record_classes[{idx}].export_semantics.semantic_classes")
        }
        if not export_classes:
            findings.append(
                Finding(
                    severity="error",
                    check_id="record.rows.export_semantics_missing",
                    message=f"{row_id} has no export semantic classes",
                    remediation="Declare exportability, manifest, packet, receipt, inventory, or non-exportable semantics.",
                    ref=row_id,
                )
            )
        unknown_export = sorted(export_classes - export_semantics)
        if unknown_export:
            findings.append(
                Finding(
                    severity="error",
                    check_id="record.rows.export_semantics.invalid",
                    message=f"{row_id} uses unknown export semantic classes",
                    remediation="Use classes from export_semantic_vocabulary.",
                    ref=row_id,
                    details={"unknown": unknown_export},
                )
            )

        for list_name in ("support_surface_refs", "product_surface_refs", "docs_refs"):
            for ref in ensure_list(row.get(list_name), f"record_registry.record_classes[{idx}].{list_name}"):
                validate_path_ref(repo_root, ensure_str(ref, f"record_registry.record_classes[{idx}].{list_name}[]"), f"record.rows.{list_name}", findings)

    observed_scopes = {ensure_str(row.get("class_scope"), "record.class_scope") for row in rows.values() if row.get("class_scope")}
    missing_scopes = REQUIRED_CLASS_SCOPES - observed_scopes
    if missing_scopes:
        findings.append(
            Finding(
                severity="error",
                check_id="registry.coverage.required_class_scope_missing",
                message="record registry is missing required class scopes",
                remediation="Add durable state, support bundle, portable package, managed copy, export packet, and receipt rows.",
                details={"missing": sorted(missing_scopes)},
            )
        )
    return rows


def validate_cross_links(
    schema_rows: dict[str, dict[str, Any]],
    record_rows: dict[str, dict[str, Any]],
    findings: list[Finding],
) -> None:
    for schema_id, schema_row in schema_rows.items():
        for raw_ref in ensure_list(schema_row.get("record_class_refs"), f"schema_rows[{schema_id}].record_class_refs"):
            ref = ensure_str(raw_ref, f"schema_rows[{schema_id}].record_class_refs[]")
            if ref not in record_rows:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="schema.rows.record_class_ref_unknown",
                        message=f"{schema_id} cites unknown alpha record row: {ref}",
                        remediation="Add the record row or correct record_class_refs.",
                        ref=schema_id,
                    )
                )


def validate_fixture_manifest(repo_root: Path, manifest: dict[str, Any], findings: list[Finding]) -> None:
    schema_version = ensure_int(manifest.get("schema_version"), "fixture_manifest.schema_version")
    if schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="fixture_manifest.schema_version.unsupported",
                message=f"fixture manifest schema_version must be 1, got {schema_version}",
                remediation="Update the validator if the fixture manifest shape changes.",
            )
        )
    parse_iso_date(ensure_str(manifest.get("as_of"), "fixture_manifest.as_of"), "fixture_manifest.as_of", findings)
    for field_name in ("schema_registry_ref", "record_registry_ref", "validator_ref"):
        validate_path_ref(repo_root, ensure_str(manifest.get(field_name), f"fixture_manifest.{field_name}"), f"fixture_manifest.{field_name}", findings)

    states: set[str] = set()
    for idx, raw_case in enumerate(ensure_list(manifest.get("acceptance_cases"), "fixture_manifest.acceptance_cases")):
        case = ensure_dict(raw_case, f"fixture_manifest.acceptance_cases[{idx}]")
        ensure_str(case.get("case_id"), f"fixture_manifest.acceptance_cases[{idx}].case_id")
        states.add(ensure_str(case.get("exercises_state"), f"fixture_manifest.acceptance_cases[{idx}].exercises_state"))
        validate_path_ref(
            repo_root,
            ensure_str(case.get("fixture_ref"), f"fixture_manifest.acceptance_cases[{idx}].fixture_ref"),
            "fixture_manifest.acceptance_cases.fixture_ref",
            findings,
        )
        ensure_str(case.get("passing_condition"), f"fixture_manifest.acceptance_cases[{idx}].passing_condition")
    missing = REQUIRED_FIXTURE_STATES - states
    if missing:
        findings.append(
            Finding(
                severity="error",
                check_id="fixture_manifest.acceptance_states.missing",
                message="fixture manifest does not exercise all required acceptance states",
                remediation="Add cases for durable state, support bundle, portable package, managed copy, receipt/delete semantics, and first consumer projection.",
                details={"missing": sorted(missing)},
            )
        )


def render_projection(schema_rows: dict[str, dict[str, Any]], record_rows: dict[str, dict[str, Any]]) -> list[dict[str, Any]]:
    projection: list[dict[str, Any]] = []
    for row_id in sorted(record_rows):
        row = record_rows[row_id]
        schema_projection = []
        for schema_ref in row.get("schema_row_refs", []):
            schema_row = schema_rows.get(schema_ref)
            if not schema_row:
                continue
            schema_projection.append(
                {
                    "row_id": schema_ref,
                    "schema_role": schema_row.get("schema_role"),
                    "schema_ref": schema_row.get("schema_ref"),
                    "schema_status": schema_row.get("schema_status"),
                    "owner_dri": schema_row.get("owner_dri"),
                }
            )
        projection.append(
            {
                "record_class_id": row.get("record_class_id"),
                "row_id": row_id,
                "class_scope": row.get("class_scope"),
                "owner_dri": row.get("owner_dri"),
                "schema_rows": schema_projection,
                "local_truth_authority": row.get("local_truth", {}).get("authority_class"),
                "managed_copy_posture": row.get("local_truth", {}).get("managed_copy_posture"),
                "retention_label": row.get("retention", {}).get("retention_label"),
                "export_semantics": row.get("export_semantics"),
                "delete_semantics": row.get("delete_semantics"),
                "hold_semantics": row.get("hold_semantics"),
            }
        )
    return projection


def validate_projection_fields(projection: list[dict[str, Any]], findings: list[Finding]) -> None:
    for item in projection:
        missing = sorted(field for field in REQUIRED_PROJECTION_FIELDS if field not in item or item[field] in (None, "", []))
        if missing:
            findings.append(
                Finding(
                    severity="error",
                    check_id="projection.required_field_missing",
                    message=f"support/export projection is missing fields for {item.get('row_id')}",
                    remediation="Ensure render_projection emits all required consumer fields.",
                    ref=str(item.get("row_id")),
                    details={"missing": missing},
                )
            )


def write_report(
    path: Path,
    schema_registry_rel: str,
    record_registry_rel: str,
    projection: list[dict[str, Any]],
    findings: list[Finding],
) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": 1,
        "status": "pass" if not any(item.severity == "error" for item in findings) else "fail",
        "generated_at": dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
        "schema_registry_ref": schema_registry_rel,
        "record_registry_ref": record_registry_rel,
        "summary": {
            "errors": sum(1 for item in findings if item.severity == "error"),
            "warnings": sum(1 for item in findings if item.severity == "warning"),
            "checked_record_classes": [item["row_id"] for item in projection],
            "checked_schema_rows": sorted({schema["row_id"] for item in projection for schema in item["schema_rows"]}),
        },
        "consumer_projection": projection,
        "findings": [item.as_report() for item in findings],
    }
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(f"--repo-root does not look like a repository root: {repo_root}")

    schema_registry_rel = str(args.schema_registry)
    record_registry_rel = str(args.record_registry)
    schema_registry = ensure_dict(render_yaml_as_json(repo_root / schema_registry_rel), "schema_registry")
    record_registry = ensure_dict(render_yaml_as_json(repo_root / record_registry_rel), "record_registry")
    base_record_registry = ensure_dict(render_yaml_as_json(repo_root / args.base_record_registry), "base_record_registry")
    fixture_manifest = ensure_dict(render_yaml_as_json(repo_root / args.fixture_manifest), "fixture_manifest")
    alpha_scoreboard = ensure_dict(render_yaml_as_json(repo_root / args.alpha_scoreboard), "alpha_scoreboard")

    findings: list[Finding] = []
    scoreboard_ids = collect_scoreboard_ids(alpha_scoreboard)
    schema_rows = validate_schema_registry(repo_root, schema_registry, scoreboard_ids, findings)
    record_rows = validate_record_registry(
        repo_root,
        record_registry,
        schema_rows,
        collect_base_record_class_ids(base_record_registry),
        findings,
    )
    validate_cross_links(schema_rows, record_rows, findings)
    validate_fixture_manifest(repo_root, fixture_manifest, findings)
    projection = render_projection(schema_rows, record_rows)
    validate_projection_fields(projection, findings)

    if args.render_support_export_projection:
        print(json.dumps({"consumer_projection": projection}, indent=2, sort_keys=True))

    if args.report:
        write_report(repo_root / str(args.report), schema_registry_rel, record_registry_rel, projection, findings)

    errors = [finding for finding in findings if finding.severity == "error"]
    warnings = [finding for finding in findings if finding.severity == "warning"]
    status = "PASS" if not errors else "FAIL"
    print(f"[schema-record-registry-alpha] {status} ({len(errors)} errors, {len(warnings)} warnings)", file=sys.stderr if args.render_support_export_projection else sys.stdout)
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        line = f"[schema-record-registry-alpha] {prefix} {finding.check_id}: {finding.message}{ref_suffix}"
        remediation = f"[schema-record-registry-alpha]   remediation: {finding.remediation}"
        print(line, file=sys.stderr if args.render_support_export_projection else sys.stdout)
        print(remediation, file=sys.stderr if args.render_support_export_projection else sys.stdout)
    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[schema-record-registry-alpha] interrupted", file=sys.stderr)
        sys.exit(130)
