#!/usr/bin/env python3
"""Validate component conformance matrix + packet fixtures without external deps."""

from __future__ import annotations

import json
import re
import subprocess
import sys
from pathlib import Path
from urllib.parse import urldefrag

REPO_ROOT = Path(__file__).resolve().parents[2]

MATRIX_SCHEMA_PATH = REPO_ROOT / "schemas/design/component_conformance_matrix.schema.json"
PACKET_SCHEMA_PATH = REPO_ROOT / "schemas/design/component_conformance_packet.schema.json"

MATRIX_PATH = REPO_ROOT / "artifacts/design/component_conformance_matrix.yaml"
PACKET_DIR = REPO_ROOT / "fixtures/design/component_packet_examples"
PACKET_PATHS = sorted(path for path in PACKET_DIR.glob("*.yaml") if path.name != "README.md")

METRICS_LEDGER_PATH = REPO_ROOT / "artifacts/design/component_metrics_ledger.yaml"
METRICS_SCHEMA_PATH = REPO_ROOT / "schemas/design/component_metrics.schema.json"

REQUIRED_FAMILIES = {
    "button",
    "text_input",
    "choice_control",
    "tabs",
    "tree",
    "list_row",
    "table",
    "dialog_or_sheet",
    "banner",
    "toast",
    "status_item",
    "settings_row",
    "permission_sheet",
    "badge_or_pill",
}


class ValidationError(Exception):
    """Raised when an instance fails schema validation."""


SCHEMA_CACHE: dict[Path, object] = {}


def render_yaml_as_json(path: Path) -> object:
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
        raise SystemExit(f"failed to parse YAML at {path} via Ruby/Psych: {stderr}")
    try:
        return json.loads(ruby.stdout)
    except json.JSONDecodeError as exc:
        raise SystemExit(f"Ruby/Psych emitted invalid JSON for {path}: {exc}") from exc


def load_schema(path: Path) -> object:
    path = path.resolve()
    if path not in SCHEMA_CACHE:
        with path.open("r", encoding="utf-8") as fh:
            SCHEMA_CACHE[path] = json.load(fh)
    return SCHEMA_CACHE[path]


def resolve_ref(ref: str, base_path: Path) -> tuple[object, Path]:
    ref_path, fragment = urldefrag(ref)
    if ref_path.startswith(("http://", "https://")):
        raise ValidationError(f"remote refs are unsupported in this validator: {ref}")

    if ref_path:
        target_path = (base_path.parent / ref_path).resolve()
        target = load_schema(target_path)
        target_base = target_path
    else:
        target = load_schema(base_path)
        target_base = base_path

    if not fragment:
        return target, target_base

    node = target
    for part in fragment.lstrip("/").split("/"):
        part = part.replace("~1", "/").replace("~0", "~")
        node = node[part]
    return node, target_base


def expect(condition: bool, message: str) -> None:
    if not condition:
        raise ValidationError(message)


def is_number(value: object) -> bool:
    return isinstance(value, (int, float)) and not isinstance(value, bool)


def validate(instance: object, schema: object, base_path: Path, where: str = "root") -> None:
    schema = dict(schema)

    if "$ref" in schema:
        target, target_base = resolve_ref(schema["$ref"], base_path)
        validate(instance, target, target_base, where)
        return

    if "oneOf" in schema:
        matches = 0
        errors: list[str] = []
        for option in schema["oneOf"]:
            try:
                validate(instance, option, base_path, where)
                matches += 1
            except ValidationError as exc:
                errors.append(str(exc))
        if matches != 1:
            raise ValidationError(
                f"{where}: expected exactly one oneOf branch, got {matches} matches; errors={errors}"
            )
        return

    if "const" in schema:
        expect(instance == schema["const"], f"{where}: expected const {schema['const']!r}, got {instance!r}")

    if "enum" in schema:
        expect(instance in schema["enum"], f"{where}: expected one of {schema['enum']!r}, got {instance!r}")

    schema_type = schema.get("type")
    if schema_type == "null":
        expect(instance is None, f"{where}: expected null, got {type(instance).__name__}")
        return

    if schema_type == "object":
        expect(isinstance(instance, dict), f"{where}: expected object, got {type(instance).__name__}")
        required = schema.get("required", [])
        for key in required:
            expect(key in instance, f"{where}: missing required key {key!r}")
        properties = schema.get("properties", {})
        if schema.get("additionalProperties") is False:
            extra = set(instance) - set(properties)
            expect(not extra, f"{where}: unexpected keys {sorted(extra)!r}")
        for key, subschema in properties.items():
            if key in instance:
                validate(instance[key], subschema, base_path, f"{where}.{key}")
        return

    if schema_type == "array":
        expect(isinstance(instance, list), f"{where}: expected array, got {type(instance).__name__}")
        if "minItems" in schema:
            expect(
                len(instance) >= schema["minItems"],
                f"{where}: expected at least {schema['minItems']} items, got {len(instance)}",
            )
        if schema.get("uniqueItems"):
            marker = [json.dumps(item, sort_keys=True) for item in instance]
            expect(len(marker) == len(set(marker)), f"{where}: expected unique items")
        if "items" in schema:
            for index, item in enumerate(instance):
                validate(item, schema["items"], base_path, f"{where}[{index}]")
        return

    if schema_type == "string":
        expect(isinstance(instance, str), f"{where}: expected string, got {type(instance).__name__}")
        if "minLength" in schema:
            expect(
                len(instance) >= schema["minLength"],
                f"{where}: expected minLength {schema['minLength']}, got {len(instance)}",
            )
        if "pattern" in schema:
            expect(
                re.match(schema["pattern"], instance) is not None,
                f"{where}: value {instance!r} does not match pattern {schema['pattern']!r}",
            )
        return

    if schema_type == "integer":
        expect(
            isinstance(instance, int) and not isinstance(instance, bool),
            f"{where}: expected integer, got {type(instance).__name__}",
        )
        if "minimum" in schema:
            expect(instance >= schema["minimum"], f"{where}: expected minimum {schema['minimum']}, got {instance}")
        if "maximum" in schema:
            expect(instance <= schema["maximum"], f"{where}: expected maximum {schema['maximum']}, got {instance}")
        return

    if schema_type == "number":
        expect(is_number(instance), f"{where}: expected number, got {type(instance).__name__}")
        if "minimum" in schema:
            expect(instance >= schema["minimum"], f"{where}: expected minimum {schema['minimum']}, got {instance}")
        if "maximum" in schema:
            expect(instance <= schema["maximum"], f"{where}: expected maximum {schema['maximum']}, got {instance}")
        return

    if schema_type == "boolean":
        expect(isinstance(instance, bool), f"{where}: expected boolean, got {type(instance).__name__}")
        return

    if schema_type is None:
        return

    raise ValidationError(f"{where}: unsupported schema type {schema_type!r}")


def dig(obj: object, path: str) -> object:
    node = obj
    for part in path.split("."):
        if not isinstance(node, dict):
            raise ValidationError(f"{path}: expected object at {part!r}, got {type(node).__name__}")
        if part not in node:
            raise ValidationError(f"{path}: missing key {part!r}")
        node = node[part]
    return node


def ensure_nonempty(value: object, path: str) -> None:
    if value is None:
        raise ValidationError(f"{path}: expected non-empty value, got null")
    if isinstance(value, str):
        expect(value.strip() != "", f"{path}: expected non-empty string")
        return
    if isinstance(value, list):
        expect(len(value) > 0, f"{path}: expected non-empty array")
        return
    if isinstance(value, dict):
        expect(bool(value), f"{path}: expected non-empty object")
        return
    raise ValidationError(f"{path}: unsupported value type {type(value).__name__}")


def load_metric_ids() -> set[str]:
    if not METRICS_LEDGER_PATH.exists():
        raise ValidationError(f"missing component metrics ledger: {METRICS_LEDGER_PATH}")
    ledger = render_yaml_as_json(METRICS_LEDGER_PATH)
    schema = load_schema(METRICS_SCHEMA_PATH)
    validate(ledger, schema, METRICS_SCHEMA_PATH, "metrics_ledger")
    expect(isinstance(ledger, dict), "metrics_ledger: expected object")
    rows = ledger.get("metric_rows")
    expect(isinstance(rows, list), "metrics_ledger.metric_rows: expected array")
    metric_ids: set[str] = set()
    for idx, row in enumerate(rows):
        expect(isinstance(row, dict), f"metrics_ledger.metric_rows[{idx}]: expected object")
        metric_id = row.get("metric_id")
        expect(isinstance(metric_id, str), f"metrics_ledger.metric_rows[{idx}].metric_id: expected string")
        metric_ids.add(metric_id)
    return metric_ids


def main() -> int:
    matrix_schema = load_schema(MATRIX_SCHEMA_PATH)
    packet_schema = load_schema(PACKET_SCHEMA_PATH)

    if not MATRIX_PATH.exists():
        print(f"missing component conformance matrix: {MATRIX_PATH}", file=sys.stderr)
        return 1
    matrix = render_yaml_as_json(MATRIX_PATH)
    try:
        validate(matrix, matrix_schema, MATRIX_SCHEMA_PATH, "matrix")
    except ValidationError as exc:
        print(f"{MATRIX_PATH.relative_to(REPO_ROOT)}: {exc}", file=sys.stderr)
        return 1
    print(MATRIX_PATH.relative_to(REPO_ROOT))

    expect(isinstance(matrix, dict), "matrix: expected object")
    family_rows = matrix.get("component_family_rows")
    expect(isinstance(family_rows, list), "matrix.component_family_rows: expected array")
    family_index = {row.get("component_family_class") for row in family_rows if isinstance(row, dict)}
    missing_families = sorted(REQUIRED_FAMILIES - set(family_index))
    if missing_families:
        print(
            f"{MATRIX_PATH.relative_to(REPO_ROOT)}: missing required component families: {missing_families}",
            file=sys.stderr,
        )
        return 1

    if not PACKET_DIR.exists():
        print(f"missing packet fixture dir: {PACKET_DIR}", file=sys.stderr)
        return 1
    if not PACKET_PATHS:
        print(f"no component conformance packet fixtures found in {PACKET_DIR}", file=sys.stderr)
        return 1

    metric_ids = load_metric_ids()

    packets: dict[str, dict[str, object]] = {}
    for path in PACKET_PATHS:
        packet = render_yaml_as_json(path)
        try:
            validate(packet, packet_schema, PACKET_SCHEMA_PATH, "packet")
        except ValidationError as exc:
            print(f"{path.relative_to(REPO_ROOT)}: {exc}", file=sys.stderr)
            return 1
        expect(isinstance(packet, dict), f"{path.relative_to(REPO_ROOT)}: expected object")
        packet_id = packet.get("packet_id")
        expect(isinstance(packet_id, str), f"{path.relative_to(REPO_ROOT)}: packet_id missing or not a string")
        if packet_id in packets:
            print(f"duplicate packet_id {packet_id} in {path}", file=sys.stderr)
            return 1
        packets[packet_id] = packet
        print(path.relative_to(REPO_ROOT))

        # Cross-check metric ids against the ledger for early drift detection.
        metric_block = packet.get("metric_requirements")
        if isinstance(metric_block, dict):
            for metric_id in metric_block.get("metric_ids", []):
                if isinstance(metric_id, str) and metric_id not in metric_ids:
                    print(f"{path.relative_to(REPO_ROOT)}: unknown metric id {metric_id!r}", file=sys.stderr)
                    return 1

    fail_gate = matrix.get("ci_fail_gate_contract")
    expect(isinstance(fail_gate, dict), "matrix.ci_fail_gate_contract: expected object")
    policies = fail_gate.get("launch_priority_policies")
    expect(isinstance(policies, list), "matrix.ci_fail_gate_contract.launch_priority_policies: expected array")
    policy_index: dict[str, dict[str, object]] = {}
    for idx, policy in enumerate(policies):
        expect(isinstance(policy, dict), f"policy[{idx}]: expected object")
        priority = policy.get("launch_priority_class")
        expect(isinstance(priority, str), f"policy[{idx}].launch_priority_class: expected string")
        policy_index[priority] = policy

    launch_critical_packet_refs = matrix.get("launch_critical_packet_refs", [])
    expect(isinstance(launch_critical_packet_refs, list), "matrix.launch_critical_packet_refs: expected array")
    for ref in launch_critical_packet_refs:
        expect(isinstance(ref, str), "matrix.launch_critical_packet_refs: expected string entries")
        if ref not in packets:
            print(f"{MATRIX_PATH.relative_to(REPO_ROOT)}: missing packet fixture for {ref}", file=sys.stderr)
            return 1

        packet = packets[ref]
        priority = packet.get("launch_priority_class")
        expect(isinstance(priority, str), f"{ref}: launch_priority_class must be a string")
        policy = policy_index.get(priority)
        if policy is None:
            print(f"{ref}: no fail-gate policy for launch_priority_class={priority!r}", file=sys.stderr)
            return 1

        posture = packet.get("conformance_posture_class")
        allowed_postures = policy.get("allowed_conformance_posture_classes", [])
        expect(isinstance(allowed_postures, list), f"{ref}: policy.allowed_conformance_posture_classes must be array")
        expect(posture in allowed_postures, f"{ref}: conformance_posture_class {posture!r} not allowed")

        required_paths = policy.get("required_nonempty_fields", [])
        expect(isinstance(required_paths, list), f"{ref}: policy.required_nonempty_fields must be array")
        for required_path in required_paths:
            expect(isinstance(required_path, str), f"{ref}: required_nonempty_fields entries must be strings")
            ensure_nonempty(dig(packet, required_path), f"{ref}.{required_path}")

        exceptions = packet.get("extension_embedded_exceptions", [])
        expect(isinstance(exceptions, list), f"{ref}: extension_embedded_exceptions must be array")
        if posture == "fully_conformant":
            expect(len(exceptions) == 0, f"{ref}: fully_conformant packets must not declare exceptions")
        if posture == "conformant_with_disclosed_gaps":
            expect(len(exceptions) > 0, f"{ref}: conformant_with_disclosed_gaps requires at least one exception")
            for idx, exception in enumerate(exceptions):
                expect(isinstance(exception, dict), f"{ref}.extension_embedded_exceptions[{idx}]: expected object")
                owner = exception.get("surface_owner_role_class")
                gap_class = exception.get("allowed_inheritance_gap_class")
                if isinstance(owner, str) and isinstance(gap_class, str):
                    if owner == "extension_contributed_surface":
                        expect(
                            gap_class.startswith("extension_"),
                            f"{ref}.extension_embedded_exceptions[{idx}]: extension owner requires extension_* gap class",
                        )
                    if owner == "embedded_surface_contributed":
                        expect(
                            gap_class.startswith("embedded_"),
                            f"{ref}.extension_embedded_exceptions[{idx}]: embedded owner requires embedded_* gap class",
                        )

    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except ValidationError as exc:
        print(f"component conformance validation error: {exc}", file=sys.stderr)
        raise

