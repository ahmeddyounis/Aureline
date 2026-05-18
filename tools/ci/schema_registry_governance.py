#!/usr/bin/env python3
"""Validate governed schema and record-class registries."""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any


DEFAULT_SCHEMA_REGISTRY = "schemas/registry/schema_registry.json"
DEFAULT_RECORD_CLASS_REGISTRY = "schemas/registry/record_class_registry.json"
EXPECTED_SCHEMA_VERSION = 1
REQUIRED_VERSION_LABELS = {"supported", "limited", "deprecated", "unsupported"}


@dataclass
class Finding:
    check_id: str
    message: str
    ref: str


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".", help="Repository root.")
    parser.add_argument(
        "--schema-registry",
        default=DEFAULT_SCHEMA_REGISTRY,
        help="Schema registry JSON path, repo-relative.",
    )
    parser.add_argument(
        "--record-class-registry",
        default=DEFAULT_RECORD_CLASS_REGISTRY,
        help="Record-class registry JSON path, repo-relative.",
    )
    return parser.parse_args()


def load_json(root: Path, rel: str) -> Any:
    path = root / rel
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def require_file(root: Path, rel: str, findings: list[Finding], check_id: str) -> None:
    if not (root / rel).exists():
        findings.append(Finding(check_id, f"missing referenced file {rel}", rel))


def require_non_empty(
    value: Any,
    row_id: str,
    field_name: str,
    findings: list[Finding],
    check_id: str,
) -> None:
    if value is None or value == "" or value == []:
        findings.append(
            Finding(check_id, f"{row_id} has empty required field {field_name}", row_id)
        )


def validate_registry_versions(
    schema_registry: dict[str, Any],
    record_registry: dict[str, Any],
    findings: list[Finding],
) -> None:
    for rel, payload in [
        (DEFAULT_SCHEMA_REGISTRY, schema_registry),
        (DEFAULT_RECORD_CLASS_REGISTRY, record_registry),
    ]:
        actual = payload.get("schema_version")
        if actual != EXPECTED_SCHEMA_VERSION:
            findings.append(
                Finding(
                    "schema_registry.unsupported_registry_version",
                    f"{rel} has schema_version {actual!r}; expected {EXPECTED_SCHEMA_VERSION}",
                    rel,
                )
            )


def validate_record_classes(
    root: Path,
    record_registry: dict[str, Any],
    findings: list[Finding],
) -> set[str]:
    ids: set[str] = set()
    for row in record_registry.get("rows", []):
        row_id = row.get("record_class_id", "")
        if row_id in ids:
            findings.append(
                Finding(
                    "record_class_registry.duplicate_record_class",
                    f"duplicate record_class_id {row_id}",
                    row_id,
                )
            )
        ids.add(row_id)

        for field in [
            "record_class_id",
            "owner_ref",
            "local_vs_managed_truth",
            "export_semantics",
            "delete_semantics",
            "hold_semantics",
            "redaction_posture",
            "retention_posture",
            "offboarding_posture",
        ]:
            require_non_empty(
                row.get(field),
                row_id,
                field,
                findings,
                "record_class_registry.required_field_missing",
            )

        for ref in row.get("governance_refs", []):
            require_file(
                root,
                ref.split("#", 1)[0],
                findings,
                "record_class_registry.governance_ref_missing",
            )

    return ids


def validate_schema_rows(
    root: Path,
    schema_registry: dict[str, Any],
    record_class_ids: set[str],
    findings: list[Finding],
) -> None:
    seen_schema_ids: set[str] = set()
    observed_family_classes: set[str] = set()
    required_surfaces = set(schema_registry.get("required_surface_visibility", []))
    telemetry_default = schema_registry.get("open_source_telemetry_default")

    for row in schema_registry.get("rows", []):
        schema_id = row.get("schema_id", "")
        family_class = row.get("family_class", "")
        observed_family_classes.add(family_class)

        if schema_id in seen_schema_ids:
            findings.append(
                Finding(
                    "schema_registry.duplicate_schema_id",
                    f"duplicate schema_id {schema_id}",
                    schema_id,
                )
            )
        seen_schema_ids.add(schema_id)

        for field in [
            "schema_id",
            "owner_ref",
            "schema_ref",
            "schema_version_uri",
            "version_change_rationale",
            "consent_class",
            "endpoint_class",
            "retention_posture",
            "lifecycle_state",
            "open_source_default_posture",
        ]:
            require_non_empty(
                row.get(field),
                schema_id,
                field,
                findings,
                "schema_registry.required_field_missing",
            )

        if not isinstance(row.get("schema_version"), int) or row.get("schema_version", 0) < 1:
            findings.append(
                Finding(
                    "schema_registry.schema_version_required",
                    f"{schema_id} must declare schema_version >= 1",
                    schema_id,
                )
            )

        require_file(
            root,
            row.get("schema_ref", ""),
            findings,
            "schema_registry.schema_ref_missing",
        )

        for docs_ref in row.get("docs_refs", []):
            require_file(
                root,
                docs_ref,
                findings,
                "schema_registry.docs_ref_missing",
            )

        missing_labels = REQUIRED_VERSION_LABELS - set(row.get("version_support_labels", []))
        if missing_labels:
            findings.append(
                Finding(
                    "schema_registry.version_support_labels_incomplete",
                    f"{schema_id} is missing visible version labels {sorted(missing_labels)}",
                    schema_id,
                )
            )

        downgrade = row.get("downgrade_rule", {})
        for field in [
            "min_readable_version",
            "min_writable_version",
            "unknown_version_policy",
            "deprecated_version_policy",
        ]:
            require_non_empty(
                downgrade.get(field),
                schema_id,
                f"downgrade_rule.{field}",
                findings,
                "schema_registry.downgrade_rule_missing",
            )

        for record_class_id in row.get("record_class_id_refs", []):
            if record_class_id not in record_class_ids:
                findings.append(
                    Finding(
                        "schema_registry.record_class_missing",
                        f"{schema_id} references unknown record class {record_class_id}",
                        schema_id,
                    )
                )

        missing_surfaces = required_surfaces - set(row.get("surface_visibility", []))
        if missing_surfaces:
            findings.append(
                Finding(
                    "schema_registry.surface_visibility_incomplete",
                    f"{schema_id} is missing required surfaces {sorted(missing_surfaces)}",
                    schema_id,
                )
            )

        if (
            family_class == "telemetry_payload"
            and row.get("open_source_default_posture") != telemetry_default
        ):
            findings.append(
                Finding(
                    "schema_registry.telemetry_open_build_must_be_opt_in",
                    f"{schema_id} has open-source default {row.get('open_source_default_posture')!r}",
                    schema_id,
                )
            )

        separation = row.get("separation", {})
        if separation.get("canonical_family_class") != family_class:
            findings.append(
                Finding(
                    "schema_registry.separation_family_mismatch",
                    f"{schema_id} separation canonical family does not match row family",
                    schema_id,
                )
            )
        if family_class == "support_export_payload" and "telemetry_payload" not in separation.get(
            "must_not_conflate_with", []
        ):
            findings.append(
                Finding(
                    "schema_registry.support_must_be_separate_from_telemetry",
                    f"{schema_id} must explicitly stay separate from telemetry",
                    schema_id,
                )
            )

    for family_class in schema_registry.get("required_family_class_coverage", []):
        if family_class not in observed_family_classes:
            findings.append(
                Finding(
                    "schema_registry.family_coverage_missing",
                    f"missing required family class {family_class}",
                    family_class,
                )
            )


def main() -> int:
    args = parse_args()
    root = Path(args.repo_root).resolve()
    findings: list[Finding] = []

    schema_registry = load_json(root, args.schema_registry)
    record_registry = load_json(root, args.record_class_registry)

    validate_registry_versions(schema_registry, record_registry, findings)
    for rel in [
        args.schema_registry,
        args.record_class_registry,
        schema_registry.get("overview_page", ""),
        record_registry.get("overview_page", ""),
    ]:
        require_file(root, rel, findings, "schema_registry.required_artifact_missing")

    record_class_ids = validate_record_classes(root, record_registry, findings)
    validate_schema_rows(root, schema_registry, record_class_ids, findings)

    if findings:
        print("schema registry governance validation failed", file=sys.stderr)
        for finding in findings:
            print(
                f"- {finding.check_id}: {finding.message} ({finding.ref})",
                file=sys.stderr,
            )
        return 1

    print(
        "schema registry governance validation passed: "
        f"{len(schema_registry.get('rows', []))} schema rows, "
        f"{len(record_registry.get('rows', []))} record-class rows"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
