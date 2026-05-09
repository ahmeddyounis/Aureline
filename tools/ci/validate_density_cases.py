#!/usr/bin/env python3
"""Validate density-case fixtures without external deps."""

from __future__ import annotations

import json
import re
import subprocess
import sys
from pathlib import Path
from urllib.parse import urldefrag

REPO_ROOT = Path(__file__).resolve().parents[2]
SCHEMA_PATH = REPO_ROOT / "schemas/design/density_case.schema.json"
LEDGER_PATH = REPO_ROOT / "artifacts/design/geometry_token_ledger.yaml"
CASE_DIR = REPO_ROOT / "fixtures/design/density_cases"
CASE_PATHS = sorted(path for path in CASE_DIR.glob("*.json"))


class ValidationError(Exception):
    """Raised when an instance fails schema validation."""


SCHEMA_CACHE: dict[Path, object] = {}


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


def load_geometry_ledger_px() -> dict[str, int]:
    if not LEDGER_PATH.exists():
        raise SystemExit(f"missing geometry token ledger: {LEDGER_PATH}")
    ledger = render_yaml_as_json(LEDGER_PATH)
    expect(isinstance(ledger, dict), "ledger: expected object")

    px: dict[str, int] = {}

    def index_section(section_key: str) -> None:
        rows = ledger.get(section_key)
        expect(isinstance(rows, list), f"ledger.{section_key}: expected array")
        for row in rows:
            expect(isinstance(row, dict), f"ledger.{section_key}: expected object rows")
            name = row.get("token_name")
            value = row.get("px")
            if isinstance(name, str) and isinstance(value, int):
                px[name] = value

    index_section("spacing_tokens")
    index_section("sizing_tokens")
    return px


def main() -> int:
    schema = load_schema(SCHEMA_PATH)
    if not CASE_PATHS:
        print(f"no density cases found in {CASE_DIR}", file=sys.stderr)
        return 1

    ledger_px = load_geometry_ledger_px()

    for path in CASE_PATHS:
        with path.open("r", encoding="utf-8") as fh:
            instance = json.load(fh)
        validate(instance, schema, SCHEMA_PATH, path.name)

        expect(isinstance(instance, dict), f"{path.name}: expected object")
        expectations = instance.get("token_expectations", [])
        expect(isinstance(expectations, list), f"{path.name}.token_expectations: expected array")
        for index, row in enumerate(expectations):
            expect(isinstance(row, dict), f"{path.name}.token_expectations[{index}]: expected object")
            token_ref = row.get("token_ref")
            expected_px = row.get("expected_px")
            expect(isinstance(token_ref, str), f"{path.name}.token_expectations[{index}].token_ref: expected string")
            expect(isinstance(expected_px, int), f"{path.name}.token_expectations[{index}].expected_px: expected int")
            ledger_value = ledger_px.get(token_ref)
            expect(ledger_value is not None, f"{path.name}: token_ref {token_ref!r} missing from geometry token ledger")
            expect(
                expected_px == ledger_value,
                f"{path.name}: expected_px drift for {token_ref} (expected {expected_px}, ledger {ledger_value})",
            )

        print(path.relative_to(REPO_ROOT))

    return 0


if __name__ == "__main__":
    raise SystemExit(main())

