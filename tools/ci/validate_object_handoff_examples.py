#!/usr/bin/env python3
"""Validate object-handoff example packets without external deps."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path
from urllib.parse import urldefrag

REPO_ROOT = Path(__file__).resolve().parents[2]
SCHEMA_PATH = REPO_ROOT / "schemas/support/object_handoff_packet.schema.json"
EXAMPLE_DIR = REPO_ROOT / "fixtures/support/object_handoff_examples"
EXAMPLE_PATHS = sorted(EXAMPLE_DIR.glob("*.json"))


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
        return

    if schema_type is None:
        return

    raise ValidationError(f"{where}: unsupported schema type {schema_type!r}")


def strip_fixture_metadata(instance: object) -> object:
    if isinstance(instance, dict):
        return {key: value for key, value in instance.items() if key not in {"$schema", "__fixture__"}}
    return instance


def main() -> int:
    schema = load_schema(SCHEMA_PATH)
    if not EXAMPLE_PATHS:
        print("no object-handoff examples found", file=sys.stderr)
        return 1

    for path in EXAMPLE_PATHS:
        with path.open("r", encoding="utf-8") as fh:
            instance = strip_fixture_metadata(json.load(fh))
        validate(instance, schema, SCHEMA_PATH)
        print(path.relative_to(REPO_ROOT))
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except ValidationError as exc:
        print(exc, file=sys.stderr)
        raise SystemExit(1)
