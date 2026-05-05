#!/usr/bin/env python3
"""Validate motion token ledger + motion-case fixtures without external deps."""

from __future__ import annotations

import json
import re
import subprocess
import sys
from pathlib import Path
from urllib.parse import urldefrag

REPO_ROOT = Path(__file__).resolve().parents[2]
SCHEMA_PATH = REPO_ROOT / "schemas/design/motion_transition.schema.json"
LEDGER_PATH = REPO_ROOT / "artifacts/design/motion_tokens.yaml"
CASE_DIR = REPO_ROOT / "fixtures/design/motion_cases"
CASE_PATHS = sorted(path for path in CASE_DIR.glob("*.yaml") if path.name != "README.md")


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

    if "allOf" in schema:
        for idx, subschema in enumerate(schema["allOf"]):
            validate(instance, subschema, base_path, f"{where}.allOf[{idx}]")

    if "if" in schema:
        try:
            validate(instance, schema["if"], base_path, f"{where}.if")
            condition = True
        except ValidationError:
            condition = False

        if condition and "then" in schema:
            validate(instance, schema["then"], base_path, f"{where}.then")
        if not condition and "else" in schema:
            validate(instance, schema["else"], base_path, f"{where}.else")

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
        if "contains" in schema:
            matches = 0
            for index, item in enumerate(instance):
                try:
                    validate(item, schema["contains"], base_path, f"{where}.contains[{index}]")
                    matches += 1
                except ValidationError:
                    continue
            expect(matches >= 1, f"{where}: expected array to contain at least one matching item")
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


def main() -> int:
    schema = load_schema(SCHEMA_PATH)

    if not LEDGER_PATH.exists():
        print(f"missing motion token ledger: {LEDGER_PATH}", file=sys.stderr)
        return 1
    if not CASE_PATHS:
        print(f"no motion cases found in {CASE_DIR}", file=sys.stderr)
        return 1

    ledger = render_yaml_as_json(LEDGER_PATH)
    validate(ledger, schema, SCHEMA_PATH, "ledger")
    print(LEDGER_PATH.relative_to(REPO_ROOT))

    expect(isinstance(ledger, dict), "ledger: expected object")
    duration_rows = ledger.get("duration_tokens")
    easing_rows = ledger.get("easing_tokens")
    expect(isinstance(duration_rows, list), "ledger.duration_tokens: expected array")
    expect(isinstance(easing_rows, list), "ledger.easing_tokens: expected array")

    duration_tokens = {row.get("token_name") for row in duration_rows if isinstance(row, dict)}
    easing_tokens = {row.get("token_name") for row in easing_rows if isinstance(row, dict)}

    for path in CASE_PATHS:
        instance = render_yaml_as_json(path)
        validate(instance, schema, SCHEMA_PATH, str(path.relative_to(REPO_ROOT)))
        expect(isinstance(instance, dict), f"{path}: expected object")

        default_durations = instance.get("default_duration_tokens", [])
        default_easings = instance.get("default_easing_tokens", [])
        expect(isinstance(default_durations, list), f"{path}: default_duration_tokens must be an array")
        expect(isinstance(default_easings, list), f"{path}: default_easing_tokens must be an array")
        for token in default_durations:
            expect(token in duration_tokens, f"{path}: duration token {token!r} missing from motion token ledger")
        for token in default_easings:
            expect(token in easing_tokens, f"{path}: easing token {token!r} missing from motion token ledger")

        fallbacks = instance.get("reduced_motion_fallbacks", [])
        expect(isinstance(fallbacks, list), f"{path}: reduced_motion_fallbacks must be an array")
        for idx, fallback in enumerate(fallbacks):
            if not isinstance(fallback, dict):
                continue
            fallback_duration = fallback.get("fallback_duration_token")
            if fallback_duration is not None:
                expect(
                    fallback_duration in duration_tokens,
                    f"{path}: fallback duration token {fallback_duration!r} missing from motion token ledger",
                )
            fallback_easing = fallback.get("fallback_easing_token")
            if fallback_easing is not None:
                expect(
                    fallback_easing in easing_tokens,
                    f"{path}: fallback easing token {fallback_easing!r} missing from motion token ledger",
                )

        print(path.relative_to(REPO_ROOT))

    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except ValidationError as exc:
        print(exc, file=sys.stderr)
        raise SystemExit(1)

