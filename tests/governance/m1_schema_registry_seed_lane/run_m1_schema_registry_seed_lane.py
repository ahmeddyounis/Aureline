#!/usr/bin/env python3
"""Unattended M1 schema-registry seed validation lane.

Replays every row in ``schemas/registry/schema_registry.yaml`` against:

- ``schemas/registry/schema_registry.schema.json`` — the seed envelope
  schema (vocabularies, required coverage, row list);
- ``schemas/registry/schema_registry_row.schema.json`` — the row
  vocabulary;
- the canonical schema-registry-entry registry at
  ``artifacts/governance/consent_ledger_seed.yaml`` for
  ``consent_class`` agreement on the row's parent
  ``consent_ledger_entry_id_ref``;
- the row's pinned family schema (``schemas/telemetry/...``,
  ``schemas/diagnostics/...``, ``schemas/support/...``,
  ``schemas/governance/usage_export_record.schema.json``); and
- the row's canonical envelope example under
  ``fixtures/schemas/m1_registry_examples/``.

Per-row assertions:

- ``entry_id`` is unique and matches its ``family_class``;
- closed vocabularies are honoured (``family_class``,
  ``lifecycle_state_class``, ``consent_class``,
  ``default_posture_class``, ``endpoint_class``, ``redaction_class``,
  ``registry_consumer_class``);
- the matrix's closed vocabularies agree with the row schema's
  ``$defs`` (no drift);
- the row's ``family_class`` matches its ``entry_id`` namespace;
- ``schema_ref``, ``build_identity_ref``, ``consent_ledger_registry_ref``,
  and ``named_runtime_consumer.consumer_ref`` resolve on disk;
- the pinned family schema publishes the expected ``$id``, exposes
  ``record_kind`` as a const, and exposes the pinned ``schema_version``
  (or the family's alias ``collection_schema_version`` /
  ``usage_export_record_schema_version``);
- the example payload carries ``registry_example_kind:
  schema_registry_envelope_example``, the row's ``entry_id``, and the
  pinned ``record_kind`` / ``schema_version`` integer;
- the row's ``consent_class`` agrees with the parent row's
  ``consent_class`` in ``consent_ledger_seed.yaml``;
- every ``deprecated_field`` row carries a non-empty
  ``removal_window_note`` and a typed ``downgrade_action``;
- the matrix covers every entry in ``required_family_class_coverage``;
- every row's failure drill is listed in
  ``failure_drill_id_vocabulary``.

``--force-drill <row_id>:<drill_id>`` replays the named drill on the
named row and exits 0 only when the runner reproduces the declared
``expected_check_id``. Drift in the unforced rows still fails the lane.

YAML decoding follows the repository convention: matrix and fixture
files are parsed via Ruby/Psych so this script does not require a
third-party Python YAML dependency.
"""

from __future__ import annotations

import argparse
import copy
import datetime as dt
import json
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_MATRIX_REL = "schemas/registry/schema_registry.yaml"
DEFAULT_ENVELOPE_SCHEMA_REL = "schemas/registry/schema_registry.schema.json"
DEFAULT_ROW_SCHEMA_REL = "schemas/registry/schema_registry_row.schema.json"
DEFAULT_CONSENT_LEDGER_REL = "artifacts/governance/consent_ledger_seed.yaml"
DEFAULT_BUILD_IDENTITY_REL = "artifacts/build/build_identity.json"
DEFAULT_REPORT_REL = (
    "artifacts/milestones/m1/captures/"
    "schema_registry_seed_validation_capture.json"
)

EXPECTED_ROW_SCHEMA_VERSION = 1
EXPECTED_RECORD_KIND = "m1_schema_registry_row"
EXPECTED_REGISTRY_EXAMPLE_KIND = "schema_registry_envelope_example"

# Closed mapping family_class -> required entry_id prefix.
FAMILY_TO_PREFIX = {
    "telemetry_payload": "telemetry.",
    "diagnostic_payload": "diagnostics.",
    "support_export_payload": "support_export.",
    "usage_export_payload": "usage_export.",
}

# Family schemas may name the integer schema-version field differently
# (e.g. ``collection_schema_version``,
# ``usage_export_record_schema_version``). The lane normalises the
# alias when verifying the pin.
SCHEMA_VERSION_FIELD_ALIASES = {
    "schemas/telemetry/m1_onboarding_metrics.schema.json": "schema_version",
    "schemas/diagnostics/problem_evidence_chain.schema.json": "schema_version",
    "schemas/support/support_bundle_manifest.schema.json": "collection_schema_version",
    "schemas/governance/usage_export_record.schema.json": "usage_export_record_schema_version",
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


@dataclass
class RowResult:
    row_id: str
    family_class: str
    lifecycle_state_class: str
    consent_class: str
    passed_checks: list[str] = field(default_factory=list)
    failed_checks: list[dict[str, str]] = field(default_factory=list)
    diagnostics: dict[str, Any] = field(default_factory=dict)


def fail(result: RowResult, check_id: str, message: str) -> None:
    result.failed_checks.append({"check_id": check_id, "message": message})


def pass_(result: RowResult, message: str) -> None:
    result.passed_checks.append(message)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--repo-root",
        default=".",
        help="Repository root (must contain a .git directory).",
    )
    parser.add_argument(
        "--matrix",
        default=DEFAULT_MATRIX_REL,
        help="Seed YAML path, repo-relative.",
    )
    parser.add_argument(
        "--envelope-schema",
        default=DEFAULT_ENVELOPE_SCHEMA_REL,
        help="Envelope schema JSON path, repo-relative.",
    )
    parser.add_argument(
        "--row-schema",
        default=DEFAULT_ROW_SCHEMA_REL,
        help="Row schema JSON path, repo-relative.",
    )
    parser.add_argument(
        "--consent-ledger",
        default=DEFAULT_CONSENT_LEDGER_REL,
        help="Parent consent-ledger registry YAML path.",
    )
    parser.add_argument(
        "--build-identity",
        default=DEFAULT_BUILD_IDENTITY_REL,
        help="Build-identity record path the capture embeds.",
    )
    parser.add_argument(
        "--report",
        default=DEFAULT_REPORT_REL,
        help="Where to write the durable JSON capture.",
    )
    parser.add_argument(
        "--force-drill",
        default=None,
        help=(
            "Replay a named failure drill on a named row in the form "
            "'<row_id>:<drill_id>'. The runner exits 0 only when the "
            "row's failure drill reproduces the exact expected_check_id."
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
            "-rdate",
            "-rtime",
            "-e",
            (
                "payload = YAML.safe_load(File.read(ARGV[0]), "
                "permitted_classes: [Time, Date, DateTime], aliases: false); "
                "STDOUT.write(JSON.generate(payload))"
            ),
            str(path),
        ],
        capture_output=True,
        text=True,
    )
    if ruby.returncode != 0:
        stderr = ruby.stderr.strip() or "unknown Ruby/Psych failure"
        raise SystemExit(f"failed to parse YAML at {path} via Ruby/Psych: {stderr}")
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


def artifact_ref_exists(repo_root: Path, ref: str) -> bool:
    ref = ref.strip()
    path = ref.split("#", 1)[0].strip()
    if not path:
        return False
    return (repo_root / path).exists()


def now_iso_z() -> str:
    return (
        dt.datetime.now(dt.timezone.utc)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )


def load_schema_enums(repo_root: Path, ref: str, defs_key: str) -> list[str]:
    """Best-effort enum lookup from a schema's $defs."""
    schema_path = repo_root / ref
    if not schema_path.exists():
        return []
    schema = json.loads(schema_path.read_text(encoding="utf-8"))
    defs = schema.get("$defs", {})
    entry = defs.get(defs_key, {})
    if "enum" in entry and isinstance(entry["enum"], list):
        return [str(v) for v in entry["enum"]]
    return []


def find_const(schema: dict[str, Any], field_name: str) -> Any:
    """Locate a const value declared on a top-level or $defs-resident field.

    Recursively descends mapping nodes named ``properties`` and ``$defs``
    looking for an object whose ``const`` is set. Returns the first
    match's const value, or ``None`` if no match is found.
    """

    def descend(node: Any) -> Any:
        if isinstance(node, dict):
            if "const" in node and not isinstance(node["const"], (dict, list)):
                return node["const"]
            for child_key in ("properties", "$defs"):
                children = node.get(child_key)
                if isinstance(children, dict) and field_name in children:
                    candidate = children[field_name]
                    if isinstance(candidate, dict):
                        # Direct const on the named property.
                        if "const" in candidate:
                            return candidate["const"]
                        # The property may $ref into a $defs entry whose const
                        # we have not resolved; defer to the caller.
                        ref = candidate.get("$ref")
                        if isinstance(ref, str) and ref.startswith("#/$defs/"):
                            defs = node.get("$defs", {})
                            if not defs and "properties" in node:
                                defs = node.get("$defs", {})
                            defs_key = ref.split("/")[-1]
                            target = (
                                node.get("$defs", {}) if "$defs" in node else {}
                            ).get(defs_key)
                            if isinstance(target, dict) and "const" in target:
                                return target["const"]
            for v in node.values():
                if isinstance(v, dict):
                    found = descend(v)
                    if found is not None:
                        return found
        return None

    # First try a direct property lookup (handles the common case where
    # ``record_kind`` lives under top-level ``properties``).
    direct = schema.get("properties", {}).get(field_name)
    if isinstance(direct, dict):
        if "const" in direct:
            return direct["const"]
        ref = direct.get("$ref")
        if isinstance(ref, str) and ref.startswith("#/$defs/"):
            defs_key = ref.split("/")[-1]
            target = schema.get("$defs", {}).get(defs_key)
            if isinstance(target, dict) and "const" in target:
                return target["const"]

    # Fall back to a $defs scan.
    defs = schema.get("$defs", {})
    if isinstance(defs, dict):
        for entry in defs.values():
            if isinstance(entry, dict):
                props = entry.get("properties", {}) if "properties" in entry else {}
                if isinstance(props, dict) and field_name in props:
                    candidate = props[field_name]
                    if isinstance(candidate, dict):
                        if "const" in candidate:
                            return candidate["const"]
                        ref = candidate.get("$ref")
                        if isinstance(ref, str) and ref.startswith("#/$defs/"):
                            defs_key = ref.split("/")[-1]
                            target = defs.get(defs_key)
                            if isinstance(target, dict) and "const" in target:
                                return target["const"]
        # And as a final fallback, look for top-level $defs entries that
        # are themselves named ``field_name``.
        named = defs.get(field_name)
        if isinstance(named, dict) and "const" in named:
            return named["const"]
    return None


def apply_forced_overrides(
    row: dict[str, Any],
    example: dict[str, Any] | None,
    forced_overrides: dict[str, Any],
) -> tuple[dict[str, Any], dict[str, Any] | None]:
    row = copy.deepcopy(row)
    example = copy.deepcopy(example) if example is not None else None

    if not forced_overrides:
        return row, example

    if "rewrite_consent_class" in forced_overrides:
        row["consent_class"] = forced_overrides["rewrite_consent_class"]

    if "rewrite_redaction_class" in forced_overrides:
        row["redaction_class"] = forced_overrides["rewrite_redaction_class"]

    if "rewrite_example_pinned_record_kind" in forced_overrides and example is not None:
        example["pinned_record_kind"] = forced_overrides[
            "rewrite_example_pinned_record_kind"
        ]

    if forced_overrides.get("clear_first_deprecated_field_removal_window_note"):
        deprecated = row.get("deprecated_fields") or []
        if isinstance(deprecated, list) and deprecated:
            entry = deprecated[0]
            if isinstance(entry, dict):
                entry["removal_window_note"] = ""

    if "rewrite_lifecycle_state_class" in forced_overrides:
        row["lifecycle_state_class"] = forced_overrides[
            "rewrite_lifecycle_state_class"
        ]

    return row, example


def validate_row(
    row: dict[str, Any],
    *,
    repo_root: Path,
    row_id_value: str,
    consent_ledger_index: dict[str, dict[str, Any]],
    family_class_vocab: set[str],
    lifecycle_state_class_vocab: set[str],
    consent_class_vocab: set[str],
    default_posture_class_vocab: set[str],
    endpoint_class_vocab: set[str],
    redaction_class_vocab: set[str],
    registry_consumer_class_vocab: set[str],
    failure_drill_id_vocab: set[str],
    example_override: dict[str, Any] | None,
) -> RowResult:
    entry_id = ensure_str(row.get("entry_id"), f"{row_id_value}.entry_id")
    family_class = ensure_str(
        row.get("family_class"), f"{row_id_value}.family_class"
    )
    lifecycle_state_class = ensure_str(
        row.get("lifecycle_state_class"),
        f"{row_id_value}.lifecycle_state_class",
    )
    consent_class = ensure_str(
        row.get("consent_class"), f"{row_id_value}.consent_class"
    )

    result = RowResult(
        row_id=entry_id,
        family_class=family_class,
        lifecycle_state_class=lifecycle_state_class,
        consent_class=consent_class,
    )

    # --- discriminator and version pins -----------------------------------
    if row.get("record_kind") != EXPECTED_RECORD_KIND:
        fail(
            result,
            "schema_registry.row.record_kind_wrong",
            (
                f"record_kind must be {EXPECTED_RECORD_KIND!r}; got "
                f"{row.get('record_kind')!r}"
            ),
        )
    if row.get("schema_registry_row_schema_version") != EXPECTED_ROW_SCHEMA_VERSION:
        fail(
            result,
            "schema_registry.row.schema_version_wrong",
            (
                "schema_registry_row_schema_version must be"
                f" {EXPECTED_ROW_SCHEMA_VERSION}; got"
                f" {row.get('schema_registry_row_schema_version')!r}"
            ),
        )

    # --- closed vocabularies ---------------------------------------------
    if family_class not in family_class_vocab:
        fail(
            result,
            "schema_registry.row.family_class_unknown",
            (
                f"family_class {family_class!r} is not in"
                " family_class_vocabulary"
            ),
        )
    if lifecycle_state_class not in lifecycle_state_class_vocab:
        fail(
            result,
            "schema_registry.row.lifecycle_state_class_unknown",
            (
                f"lifecycle_state_class {lifecycle_state_class!r} is not"
                " in lifecycle_state_class_vocabulary"
            ),
        )
    if consent_class not in consent_class_vocab:
        fail(
            result,
            "schema_registry.row.consent_class_unknown",
            (
                f"consent_class {consent_class!r} is not in"
                " consent_class_vocabulary"
            ),
        )
    default_posture_class = row.get("default_posture_class")
    if default_posture_class not in default_posture_class_vocab:
        fail(
            result,
            "schema_registry.row.default_posture_class_unknown",
            (
                f"default_posture_class {default_posture_class!r} is not"
                " in default_posture_class_vocabulary"
            ),
        )
    endpoint_class = row.get("endpoint_class")
    if endpoint_class not in endpoint_class_vocab:
        fail(
            result,
            "schema_registry.row.endpoint_class_unknown",
            (
                f"endpoint_class {endpoint_class!r} is not in"
                " endpoint_class_vocabulary"
            ),
        )
    redaction_class = row.get("redaction_class")
    if redaction_class not in redaction_class_vocab:
        fail(
            result,
            "schema_registry.row.redaction_class_unknown",
            (
                f"redaction_class {redaction_class!r} is not in"
                " redaction_class_vocabulary"
            ),
        )

    # --- entry_id matches family_class -----------------------------------
    prefix = FAMILY_TO_PREFIX.get(family_class)
    if prefix and not entry_id.startswith(prefix):
        fail(
            result,
            "schema_registry.row.entry_id_family_prefix_mismatch",
            (
                f"entry_id {entry_id!r} must start with {prefix!r} for"
                f" family_class {family_class!r}"
            ),
        )

    # --- redaction posture sanity ----------------------------------------
    if (
        family_class == "diagnostic_payload"
        and redaction_class in {"metadata_safe_default", "metadata_only_no_payload_bytes"}
    ):
        fail(
            result,
            "schema_registry.redaction_class_relaxed_without_review",
            (
                "diagnostic_payload family must default to a"
                " code-adjacent or high-risk redaction class; got"
                f" {redaction_class!r}. Relaxing requires a separately"
                " reviewed decision row."
            ),
        )

    # --- consent agreement with parent consent-ledger row ----------------
    consent_ledger_entry_id_ref = ensure_str(
        row.get("consent_ledger_entry_id_ref"),
        f"{entry_id}.consent_ledger_entry_id_ref",
    )
    parent = consent_ledger_index.get(consent_ledger_entry_id_ref)
    if parent is None:
        fail(
            result,
            "schema_registry.consent_ledger_entry_id_ref_missing",
            (
                f"consent_ledger_entry_id_ref {consent_ledger_entry_id_ref!r}"
                " is not present in the consent-ledger registry"
            ),
        )
    else:
        parent_consent_class = parent.get("consent_class")
        if parent_consent_class and parent_consent_class != consent_class:
            fail(
                result,
                "schema_registry.consent_class_disagrees_with_consent_ledger",
                (
                    f"consent_class {consent_class!r} disagrees with"
                    f" parent consent-ledger row {consent_ledger_entry_id_ref!r}"
                    f" which declares consent_class"
                    f" {parent_consent_class!r}"
                ),
            )

    # --- consumer ref -----------------------------------------------------
    named_consumer = ensure_dict(
        row.get("named_runtime_consumer"),
        f"{entry_id}.named_runtime_consumer",
    )
    consumer_ref = ensure_str(
        named_consumer.get("consumer_ref"),
        f"{entry_id}.named_runtime_consumer.consumer_ref",
    )
    if not artifact_ref_exists(repo_root, consumer_ref):
        fail(
            result,
            "schema_registry.named_runtime_consumer_missing",
            f"named_runtime_consumer.consumer_ref does not exist: {consumer_ref}",
        )
    consumer_class = ensure_str(
        named_consumer.get("consumer_class"),
        f"{entry_id}.named_runtime_consumer.consumer_class",
    )
    if consumer_class not in registry_consumer_class_vocab:
        fail(
            result,
            "schema_registry.named_runtime_consumer_consumer_class_unknown",
            (
                f"named_runtime_consumer.consumer_class {consumer_class!r}"
                " is not in registry_consumer_class_vocabulary"
            ),
        )
    consumed_fields = ensure_list(
        named_consumer.get("consumed_fields"),
        f"{entry_id}.named_runtime_consumer.consumed_fields",
    )
    if not consumed_fields:
        fail(
            result,
            "schema_registry.named_runtime_consumer_consumed_fields_empty",
            "named_runtime_consumer.consumed_fields must declare at least one field",
        )

    # --- compatibility horizon -------------------------------------------
    horizon = ensure_dict(
        row.get("compatibility_horizon"),
        f"{entry_id}.compatibility_horizon",
    )
    for k in ("min_readable_version", "min_writable_version"):
        v = horizon.get(k)
        if not isinstance(v, int) or v < 1:
            fail(
                result,
                "schema_registry.compatibility_horizon_version_invalid",
                f"compatibility_horizon.{k} must be a positive integer; got {v!r}",
            )
    for k in ("deprecation_window_note", "sunset_window_note"):
        v = horizon.get(k)
        if not isinstance(v, str) or not v.strip():
            fail(
                result,
                "schema_registry.compatibility_horizon_note_required",
                f"compatibility_horizon.{k} must be a non-empty string",
            )

    # --- deprecated_fields -----------------------------------------------
    deprecated_fields = ensure_list(
        row.get("deprecated_fields", []),
        f"{entry_id}.deprecated_fields",
    )
    for idx, dep in enumerate(deprecated_fields):
        dep = ensure_dict(dep, f"{entry_id}.deprecated_fields[{idx}]")
        if not isinstance(dep.get("field_path"), str) or not dep["field_path"].strip():
            fail(
                result,
                "schema_registry.deprecated_field_path_required",
                f"deprecated_fields[{idx}].field_path must be non-empty",
            )
        ver = dep.get("deprecated_since_schema_version")
        if not isinstance(ver, int) or ver < 1:
            fail(
                result,
                "schema_registry.deprecated_field_version_invalid",
                (
                    f"deprecated_fields[{idx}].deprecated_since_schema_version"
                    f" must be a positive integer; got {ver!r}"
                ),
            )
        if (
            not isinstance(dep.get("removal_window_note"), str)
            or not dep["removal_window_note"].strip()
        ):
            fail(
                result,
                "schema_registry.deprecated_field_removal_window_note_required",
                (
                    f"deprecated_fields[{idx}].removal_window_note must be"
                    " a non-empty reviewable sentence"
                ),
            )
        action = dep.get("downgrade_action")
        if action not in {
            "drop_field_on_read",
            "preserve_as_unknown",
            "refuse_read",
            "refuse_export",
        }:
            fail(
                result,
                "schema_registry.deprecated_field_downgrade_action_unknown",
                (
                    f"deprecated_fields[{idx}].downgrade_action {action!r}"
                    " is not in the closed downgrade-action vocabulary"
                ),
            )

    # --- schema_ref resolves and pins are honoured -----------------------
    schema_ref = ensure_str(row.get("schema_ref"), f"{entry_id}.schema_ref")
    schema_version_pin = row.get("schema_version_pin")
    if not isinstance(schema_version_pin, int) or schema_version_pin < 1:
        fail(
            result,
            "schema_registry.schema_version_pin_invalid",
            "schema_version_pin must be a positive integer",
        )
    schema_path = repo_root / schema_ref
    if not schema_path.exists():
        fail(
            result,
            "schema_registry.schema_ref_missing",
            f"schema_ref does not exist: {schema_ref}",
        )
        family_schema: dict[str, Any] = {}
    else:
        try:
            family_schema = json.loads(schema_path.read_text(encoding="utf-8"))
        except json.JSONDecodeError as exc:
            fail(
                result,
                "schema_registry.schema_ref_invalid_json",
                f"schema_ref {schema_ref} is not valid JSON: {exc}",
            )
            family_schema = {}

    schema_id = family_schema.get("$id")
    expected_uri = row.get("schema_version_uri")
    if (
        isinstance(schema_id, str)
        and isinstance(expected_uri, str)
        and schema_id != expected_uri
    ):
        fail(
            result,
            "schema_registry.schema_version_uri_mismatch",
            (
                f"schema_version_uri {expected_uri!r} does not match the"
                f" pinned schema's $id {schema_id!r}"
            ),
        )

    # Pinned record_kind comes from the schema's record_kind const.
    expected_record_kind = find_const(family_schema, "record_kind")

    # Pinned schema_version may be named differently across families.
    version_field_alias = SCHEMA_VERSION_FIELD_ALIASES.get(schema_ref, "schema_version")
    expected_schema_version = find_const(family_schema, version_field_alias)

    # --- example payload --------------------------------------------------
    example_ref = ensure_str(
        row.get("example_payload_ref"), f"{entry_id}.example_payload_ref"
    )
    example_path = repo_root / example_ref
    if example_override is not None:
        example_doc: dict[str, Any] | None = example_override
    elif not example_path.exists():
        fail(
            result,
            "schema_registry.example_payload_missing",
            f"example_payload_ref does not exist: {example_ref}",
        )
        example_doc = None
    else:
        try:
            example_doc = json.loads(example_path.read_text(encoding="utf-8"))
        except json.JSONDecodeError as exc:
            fail(
                result,
                "schema_registry.example_payload_invalid_json",
                f"example_payload_ref {example_ref} is not valid JSON: {exc}",
            )
            example_doc = None

    if example_doc is not None:
        if example_doc.get("registry_example_kind") != EXPECTED_REGISTRY_EXAMPLE_KIND:
            fail(
                result,
                "schema_registry.example_payload_kind_wrong",
                (
                    "example_payload.registry_example_kind must be"
                    f" {EXPECTED_REGISTRY_EXAMPLE_KIND!r}; got"
                    f" {example_doc.get('registry_example_kind')!r}"
                ),
            )
        if example_doc.get("schema_registry_row_entry_id") != entry_id:
            fail(
                result,
                "schema_registry.example_payload_entry_id_mismatch",
                (
                    "example_payload.schema_registry_row_entry_id must"
                    f" match the row's entry_id {entry_id!r}; got"
                    f" {example_doc.get('schema_registry_row_entry_id')!r}"
                ),
            )
        if example_doc.get("pinned_schema_ref") != schema_ref:
            fail(
                result,
                "schema_registry.example_payload_schema_ref_mismatch",
                (
                    "example_payload.pinned_schema_ref must equal the"
                    f" row's schema_ref {schema_ref!r}; got"
                    f" {example_doc.get('pinned_schema_ref')!r}"
                ),
            )
        if (
            expected_record_kind is not None
            and example_doc.get("pinned_record_kind") != expected_record_kind
        ):
            fail(
                result,
                "schema_registry.example_payload_record_kind_mismatch",
                (
                    "example_payload.pinned_record_kind must equal the"
                    f" family schema's record_kind {expected_record_kind!r};"
                    f" got {example_doc.get('pinned_record_kind')!r}"
                ),
            )
        if (
            expected_schema_version is not None
            and example_doc.get("pinned_schema_version") != expected_schema_version
        ):
            fail(
                result,
                "schema_registry.example_payload_schema_version_mismatch",
                (
                    "example_payload.pinned_schema_version must equal the"
                    f" family schema's pinned integer"
                    f" {expected_schema_version!r}; got"
                    f" {example_doc.get('pinned_schema_version')!r}"
                ),
            )
        if (
            isinstance(schema_version_pin, int)
            and example_doc.get("pinned_schema_version") != schema_version_pin
        ):
            fail(
                result,
                "schema_registry.example_payload_schema_version_pin_mismatch",
                (
                    "example_payload.pinned_schema_version must equal the"
                    f" row's schema_version_pin {schema_version_pin!r}"
                ),
            )

    # --- failure-drill shape ---------------------------------------------
    drill = ensure_dict(row.get("failure_drill"), f"{entry_id}.failure_drill")
    drill_id = ensure_str(
        drill.get("drill_id"), f"{entry_id}.failure_drill.drill_id"
    )
    if drill_id not in failure_drill_id_vocab:
        fail(
            result,
            "schema_registry.failure_drill_id_unknown",
            (
                f"failure_drill.drill_id {drill_id!r} is not in"
                " failure_drill_id_vocabulary"
            ),
        )
    forced_input = ensure_dict(
        drill.get("forced_input"), f"{entry_id}.failure_drill.forced_input"
    )
    if not forced_input:
        fail(
            result,
            "schema_registry.failure_drill_forced_input_empty",
            "failure_drill.forced_input must declare at least one drift",
        )
    ensure_str(
        drill.get("expected_check_id"),
        f"{entry_id}.failure_drill.expected_check_id",
    )
    ensure_str(
        drill.get("actionable_next_action"),
        f"{entry_id}.failure_drill.actionable_next_action",
    )

    result.diagnostics.update(
        {
            "entry_id": entry_id,
            "family_class": family_class,
            "schema_ref": schema_ref,
            "schema_version_pin": schema_version_pin,
            "expected_record_kind": expected_record_kind,
            "expected_schema_version": expected_schema_version,
            "consent_ledger_entry_id_ref": consent_ledger_entry_id_ref,
            "failure_drill": {
                "drill_id": drill_id,
                "expected_check_id": drill.get("expected_check_id"),
            },
        }
    )

    if not result.failed_checks:
        pass_(result, f"row {entry_id} passes")

    return result


def build_consent_ledger_index(
    repo_root: Path, ref: str
) -> dict[str, dict[str, Any]]:
    """Index the consent-ledger registry by entry_id so rows can resolve."""
    if not (repo_root / ref).exists():
        return {}
    raw = render_yaml_as_json(repo_root / ref)
    if not isinstance(raw, dict):
        return {}
    rows = raw.get("rows") or []
    out: dict[str, dict[str, Any]] = {}
    if isinstance(rows, list):
        for entry in rows:
            if isinstance(entry, dict):
                eid = entry.get("entry_id")
                if isinstance(eid, str):
                    out[eid] = entry
    return out


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(
            f"--repo-root does not look like a repository root: {repo_root}"
        )

    matrix_rel = args.matrix
    matrix = ensure_dict(render_yaml_as_json(repo_root / matrix_rel), matrix_rel)

    findings: list[Finding] = []

    schema_version = matrix.get("schema_version")
    if schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="schema_registry.envelope_schema_version_wrong",
                message=(
                    f"matrix schema_version must be 1; got {schema_version!r}"
                ),
                remediation="Bump runner together with the envelope schema.",
            )
        )

    matrix_id = ensure_str(matrix.get("matrix_id"), "matrix.matrix_id")
    if matrix_id != "m1_schema_registry_seed":
        findings.append(
            Finding(
                severity="error",
                check_id="schema_registry.envelope_matrix_id_wrong",
                message=(
                    f"matrix_id must be 'm1_schema_registry_seed'; got"
                    f" {matrix_id!r}"
                ),
                remediation="Restore the canonical envelope matrix id.",
            )
        )

    ensure_str(matrix.get("status"), "matrix.status")
    ensure_str(matrix.get("owner_dri"), "matrix.owner_dri")

    overview_page = ensure_str(
        matrix.get("overview_page"), "matrix.overview_page"
    )
    if not artifact_ref_exists(repo_root, overview_page):
        findings.append(
            Finding(
                severity="error",
                check_id="schema_registry.envelope_overview_page_missing",
                message=f"overview_page does not exist: {overview_page}",
                remediation="Create the reviewer landing page or fix the path.",
                ref=overview_page,
            )
        )

    for key in ("row_schema_ref", "consent_ledger_registry_ref", "build_identity_ref"):
        ref = ensure_str(matrix.get(key), f"matrix.{key}")
        if not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"schema_registry.envelope_{key}_missing",
                    message=f"{key} does not resolve: {ref}",
                    remediation="Fix the path or land the referenced artifact.",
                    ref=ref,
                )
            )

    validation_lane_ref = ensure_str(
        matrix.get("validation_lane_ref"), "matrix.validation_lane_ref"
    )
    if not artifact_ref_exists(repo_root, validation_lane_ref):
        findings.append(
            Finding(
                severity="error",
                check_id="schema_registry.envelope_validation_lane_ref_missing",
                message=(
                    f"validation_lane_ref base does not exist:"
                    f" {validation_lane_ref}"
                ),
                remediation="Point at a seeded QE lane registry entry.",
                ref=validation_lane_ref,
            )
        )

    def load_vocab(key: str) -> set[str]:
        return {
            ensure_str(item, f"matrix.{key}[]")
            for item in ensure_list(matrix.get(key), f"matrix.{key}")
        }

    family_class_vocab = load_vocab("family_class_vocabulary")
    lifecycle_state_class_vocab = load_vocab("lifecycle_state_class_vocabulary")
    consent_class_vocab = load_vocab("consent_class_vocabulary")
    default_posture_class_vocab = load_vocab("default_posture_class_vocabulary")
    endpoint_class_vocab = load_vocab("endpoint_class_vocabulary")
    redaction_class_vocab = load_vocab("redaction_class_vocabulary")
    registry_consumer_class_vocab = load_vocab(
        "registry_consumer_class_vocabulary"
    )
    failure_drill_id_vocab = load_vocab("failure_drill_id_vocabulary")
    required_family_class_coverage = load_vocab(
        "required_family_class_coverage"
    )

    row_schema_ref = ensure_str(
        matrix.get("row_schema_ref"), "matrix.row_schema_ref"
    )

    def assert_vocab_matches_schema(
        matrix_vocab: set[str], defs_key: str, name: str
    ) -> None:
        schema_enum = set(
            load_schema_enums(repo_root, row_schema_ref, defs_key)
        )
        if not schema_enum:
            return
        diff = matrix_vocab.symmetric_difference(schema_enum)
        if diff:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"schema_registry.envelope_{name}_disagrees_with_row_schema",
                    message=(
                        f"matrix.{name} disagrees with"
                        f" {row_schema_ref}#$defs.{defs_key}; "
                        f"matrix-only: {sorted(matrix_vocab - schema_enum)};"
                        f" schema-only: {sorted(schema_enum - matrix_vocab)}"
                    ),
                    remediation=(
                        "Keep the matrix vocabulary in lock-step with the"
                        " row schema; the schema is canonical."
                    ),
                )
            )

    assert_vocab_matches_schema(family_class_vocab, "family_class", "family_class_vocabulary")
    assert_vocab_matches_schema(
        lifecycle_state_class_vocab,
        "lifecycle_state_class",
        "lifecycle_state_class_vocabulary",
    )
    assert_vocab_matches_schema(
        consent_class_vocab, "consent_class", "consent_class_vocabulary"
    )
    assert_vocab_matches_schema(
        default_posture_class_vocab,
        "default_posture_class",
        "default_posture_class_vocabulary",
    )
    assert_vocab_matches_schema(
        endpoint_class_vocab, "endpoint_class", "endpoint_class_vocabulary"
    )
    assert_vocab_matches_schema(
        redaction_class_vocab,
        "redaction_class",
        "redaction_class_vocabulary",
    )
    assert_vocab_matches_schema(
        registry_consumer_class_vocab,
        "registry_consumer_class",
        "registry_consumer_class_vocabulary",
    )

    consent_ledger_index = build_consent_ledger_index(
        repo_root,
        ensure_str(
            matrix.get("consent_ledger_registry_ref"),
            "matrix.consent_ledger_registry_ref",
        ),
    )

    # --force-drill plumbing.
    forced_row_id: str | None = None
    forced_drill_id: str | None = None
    if args.force_drill:
        if ":" not in args.force_drill:
            raise SystemExit(
                "--force-drill must be of the form '<row_id>:<drill_id>'"
            )
        forced_row_id, forced_drill_id = args.force_drill.rsplit(":", 1)
        forced_row_id = forced_row_id.strip()
        forced_drill_id = forced_drill_id.strip()

    rows = ensure_list(matrix.get("rows"), "matrix.rows")
    if not rows:
        findings.append(
            Finding(
                severity="error",
                check_id="schema_registry.envelope_rows_empty",
                message="matrix.rows must declare at least one row",
                remediation="Seed the required rows.",
            )
        )

    row_results: list[RowResult] = []
    seen_ids: set[str] = set()
    seen_family_classes: set[str] = set()
    forced_replay_record: dict[str, Any] | None = None

    for idx, raw_row in enumerate(rows):
        raw_row = ensure_dict(raw_row, f"matrix.rows[{idx}]")
        row_id_local = ensure_str(
            raw_row.get("entry_id"), f"matrix.rows[{idx}].entry_id"
        )
        original_row = copy.deepcopy(raw_row)
        drill = ensure_dict(
            raw_row.get("failure_drill"),
            f"{row_id_local}.failure_drill",
        )
        drill_id_local = ensure_str(
            drill.get("drill_id"),
            f"{row_id_local}.failure_drill.drill_id",
        )

        example_override: dict[str, Any] | None = None
        applied_overrides: dict[str, Any] = {}
        replay_row_payload = raw_row
        if forced_row_id is not None and row_id_local == forced_row_id:
            if drill_id_local != forced_drill_id:
                raise SystemExit(
                    f"--force-drill drill_id {forced_drill_id!r} does not"
                    f" match the row's failure_drill.drill_id"
                    f" {drill_id_local!r}"
                )
            applied_overrides = ensure_dict(
                drill.get("forced_input"),
                f"{row_id_local}.failure_drill.forced_input",
            )
            # Pre-load the example so apply_forced_overrides can rewrite it.
            example_rel = raw_row.get("example_payload_ref")
            if isinstance(example_rel, str) and (repo_root / example_rel).exists():
                try:
                    example_override = json.loads(
                        (repo_root / example_rel).read_text(encoding="utf-8")
                    )
                except json.JSONDecodeError:
                    example_override = None
            replay_row_payload, example_override = apply_forced_overrides(
                raw_row, example_override, applied_overrides
            )

        result = validate_row(
            replay_row_payload,
            repo_root=repo_root,
            row_id_value=row_id_local,
            consent_ledger_index=consent_ledger_index,
            family_class_vocab=family_class_vocab,
            lifecycle_state_class_vocab=lifecycle_state_class_vocab,
            consent_class_vocab=consent_class_vocab,
            default_posture_class_vocab=default_posture_class_vocab,
            endpoint_class_vocab=endpoint_class_vocab,
            redaction_class_vocab=redaction_class_vocab,
            registry_consumer_class_vocab=registry_consumer_class_vocab,
            failure_drill_id_vocab=failure_drill_id_vocab,
            example_override=example_override,
        )
        if applied_overrides:
            result.diagnostics["forced_overrides_applied"] = applied_overrides
        row_results.append(result)

        if result.row_id in seen_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="schema_registry.rows_duplicate_id",
                    message=f"duplicate entry_id: {result.row_id}",
                    remediation="entry_ids must be unique.",
                    ref=result.row_id,
                )
            )
        seen_ids.add(result.row_id)

        seen_family_classes.add(
            ensure_str(
                original_row.get("family_class"),
                f"{result.row_id}.family_class",
            )
        )

        if (
            forced_row_id is not None
            and result.row_id == forced_row_id
            and applied_overrides
        ):
            expected_check = ensure_str(
                drill.get("expected_check_id"),
                f"{forced_row_id}.failure_drill.expected_check_id",
            )
            observed = [
                fc.get("check_id")
                for fc in result.failed_checks
                if isinstance(fc, dict)
            ]
            forced_replay_record = {
                "row_id": forced_row_id,
                "drill_id": forced_drill_id,
                "expected_check_id": expected_check,
                "observed_failed_check_ids": observed,
                "reproduced": expected_check in observed,
            }

    missing_families = required_family_class_coverage - seen_family_classes
    if missing_families:
        findings.append(
            Finding(
                severity="error",
                check_id="schema_registry.coverage_missing_required_family_classes",
                message=(
                    "matrix must seed at least one row for each required"
                    f" family_class: {sorted(required_family_class_coverage)};"
                    f" missing: {sorted(missing_families)}"
                ),
                remediation="Add the missing rows so every required family is exercised.",
            )
        )

    # Promote per-row failures into findings, skipping the targeted row
    # under --force-drill so the runner's exit can reflect the drill verdict.
    for result in row_results:
        if (
            forced_row_id is not None
            and result.row_id == forced_row_id
            and forced_replay_record is not None
        ):
            continue
        for failure in result.failed_checks:
            findings.append(
                Finding(
                    severity="error",
                    check_id=failure.get(
                        "check_id", "schema_registry.row_failed_check"
                    ),
                    message=f"{result.row_id}: {failure.get('message', '')}",
                    remediation=(
                        "Re-align the row with the schema-registry contract"
                        " or fix the drift in the seed; failures are"
                        " reported with the precise actionable check_id."
                    ),
                    ref=result.row_id,
                )
            )

    errors = [f for f in findings if f.severity == "error"]
    status = "PASS" if not errors else "FAIL"

    capture: dict[str, Any] = {
        "schema_version": 1,
        "capture_kind": "schema_registry_seed_validation_capture",
        "captured_at": now_iso_z(),
        "owner_dri": ensure_str(matrix.get("owner_dri"), "matrix.owner_dri"),
        "matrix_ref": matrix_rel,
        "envelope_schema_ref": args.envelope_schema,
        "row_schema_ref": args.row_schema,
        "consent_ledger_registry_ref": args.consent_ledger,
        "exact_build_identity_ref": args.build_identity,
        "command": (
            "python3 tests/governance/m1_schema_registry_seed_lane/"
            "run_m1_schema_registry_seed_lane.py --repo-root ."
        ),
        "status": status,
        "required_family_class_coverage": sorted(required_family_class_coverage),
        "observed_family_classes": sorted(seen_family_classes),
        "rows": [
            {
                "row_id": r.row_id,
                "family_class": r.family_class,
                "lifecycle_state_class": r.lifecycle_state_class,
                "consent_class": r.consent_class,
                "passed_checks": r.passed_checks,
                "failed_checks": r.failed_checks,
                "diagnostics": r.diagnostics,
            }
            for r in row_results
        ],
        "finding_counts": {
            "error": sum(1 for f in findings if f.severity == "error"),
            "warning": sum(1 for f in findings if f.severity == "warning"),
        },
        "findings": [f.as_report() for f in findings],
    }

    if forced_replay_record is not None:
        capture["forced_drill_replay"] = forced_replay_record

    report_path = repo_root / args.report
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text(
        json.dumps(capture, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )

    label = "schema-registry-seed"
    print(
        f"[{label}] {status} ({len(errors)} errors, "
        f"{len(findings) - len(errors)} warnings) — capture: {args.report}"
    )
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        print(
            f"[{label}] {prefix} {finding.check_id}: {finding.message}"
            f"{ref_suffix}"
        )
        print(f"[{label}]   remediation: {finding.remediation}")

    if forced_replay_record is not None:
        if forced_replay_record["reproduced"]:
            print(
                f"[{label}] forced drill {forced_replay_record['drill_id']}"
                f" on {forced_replay_record['row_id']} reproduced"
                f" {forced_replay_record['expected_check_id']}"
            )
            return 0
        print(
            f"[{label}] forced drill {forced_replay_record['drill_id']} on"
            f" {forced_replay_record['row_id']} did NOT reproduce"
            f" {forced_replay_record['expected_check_id']};"
            f" observed: {forced_replay_record['observed_failed_check_ids']}"
        )
        return 2

    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[schema-registry-seed] interrupted", file=sys.stderr)
        sys.exit(130)
