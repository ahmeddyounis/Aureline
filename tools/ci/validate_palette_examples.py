#!/usr/bin/env python3
"""Validate palette-mapping schema + fixtures without external deps."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path
from urllib.parse import urldefrag

REPO_ROOT = Path(__file__).resolve().parents[2]
SCHEMA_PATH = REPO_ROOT / "schemas/design/palette_mapping_row.schema.json"
EXAMPLE_DIR = REPO_ROOT / "fixtures/design/palette_examples"
EXAMPLE_PATHS = sorted(path for path in EXAMPLE_DIR.glob("*.json"))


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
        if "minProperties" in schema:
            expect(
                len(instance) >= schema["minProperties"],
                f"{where}: expected at least {schema['minProperties']} properties, got {len(instance)}",
            )
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
        if "maxLength" in schema:
            expect(
                len(instance) <= schema["maxLength"],
                f"{where}: expected maxLength {schema['maxLength']}, got {len(instance)}",
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


def load_json(path: Path) -> object:
    with path.open("r", encoding="utf-8") as fh:
        return json.load(fh)


def main() -> int:
    if not SCHEMA_PATH.exists():
        print(f"[palette-examples] missing schema at {SCHEMA_PATH}", file=sys.stderr)
        return 2
    if not EXAMPLE_DIR.exists():
        print(f"[palette-examples] missing fixtures dir at {EXAMPLE_DIR}", file=sys.stderr)
        return 2

    schema = load_schema(SCHEMA_PATH)

    failures: list[str] = []
    for example_path in EXAMPLE_PATHS:
        try:
            instance = load_json(example_path)
        except json.JSONDecodeError as exc:
            failures.append(f"{example_path}: invalid JSON: {exc}")
            continue

        try:
            validate(instance, schema, SCHEMA_PATH, where="root")
        except ValidationError as exc:
            failures.append(f"{example_path}: {exc}")

    if failures:
        print("[palette-examples] validation failed:", file=sys.stderr)
        for line in failures:
            print(f"  - {line}", file=sys.stderr)
        return 1

    print(f"[palette-examples] OK ({len(EXAMPLE_PATHS)} examples validated)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

