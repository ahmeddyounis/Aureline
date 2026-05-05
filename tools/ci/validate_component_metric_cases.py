#!/usr/bin/env python3
"""Validate component metrics ledger + component-metric case fixtures without external deps."""

from __future__ import annotations

import json
import re
import subprocess
import sys
from pathlib import Path
from urllib.parse import urldefrag

REPO_ROOT = Path(__file__).resolve().parents[2]
SCHEMA_PATH = REPO_ROOT / "schemas/design/component_metrics.schema.json"
LEDGER_PATH = REPO_ROOT / "artifacts/design/component_metrics_ledger.yaml"
CASE_DIR = REPO_ROOT / "fixtures/design/component_metric_cases"
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


def main() -> int:
    schema = load_schema(SCHEMA_PATH)

    if not LEDGER_PATH.exists():
        print(f"missing component metrics ledger: {LEDGER_PATH}", file=sys.stderr)
        return 1

    if not CASE_PATHS:
        print(f"no component-metric cases found in {CASE_DIR}", file=sys.stderr)
        return 1

    ledger = render_yaml_as_json(LEDGER_PATH)
    validate(ledger, schema, SCHEMA_PATH, "ledger")
    print(LEDGER_PATH.relative_to(REPO_ROOT))

    expect(isinstance(ledger, dict), "ledger: expected object")
    rows = ledger.get("metric_rows")
    expect(isinstance(rows, list), "ledger.metric_rows: expected array")

    metric_index: dict[str, dict[str, object]] = {}
    for idx, row in enumerate(rows):
        expect(isinstance(row, dict), f"ledger.metric_rows[{idx}]: expected object rows")
        metric_id = row.get("metric_id")
        expect(isinstance(metric_id, str) and metric_id, f"ledger.metric_rows[{idx}].metric_id missing")
        expect(metric_id not in metric_index, f"ledger.metric_rows[{idx}]: duplicate metric_id {metric_id!r}")
        metric_index[metric_id] = row

    for path in CASE_PATHS:
        instance = render_yaml_as_json(path)
        validate(instance, schema, SCHEMA_PATH, str(path.relative_to(REPO_ROOT)))
        expect(isinstance(instance, dict), f"{path}: expected object")

        expectations = instance.get("metric_expectations")
        expect(isinstance(expectations, list), f"{path}: metric_expectations must be an array")
        for idx, expectation in enumerate(expectations):
            expect(isinstance(expectation, dict), f"{path}: metric_expectations[{idx}] must be an object")
            metric_id = expectation.get("metric_id")
            expect(isinstance(metric_id, str) and metric_id, f"{path}: metric_expectations[{idx}].metric_id missing")
            row = metric_index.get(metric_id)
            expect(row is not None, f"{path}: metric_id {metric_id!r} not found in component metrics ledger")

            if "expected_px" in expectation:
                expected_px = expectation.get("expected_px")
                expect(
                    isinstance(expected_px, int) and not isinstance(expected_px, bool),
                    f"{path}: metric_expectations[{idx}].expected_px must be an integer",
                )
                actual_px = row.get("px")
                expect(
                    isinstance(actual_px, int) and not isinstance(actual_px, bool),
                    f"{path}: ledger row for {metric_id!r} must expose px for fixed expectations",
                )
                expect(
                    actual_px == expected_px,
                    f"{path}: metric_id {metric_id!r} expected_px={expected_px} does not match ledger px={actual_px}",
                )
            else:
                expected_min = expectation.get("expected_minimum_px")
                expect(
                    isinstance(expected_min, int) and not isinstance(expected_min, bool),
                    f"{path}: metric_expectations[{idx}].expected_minimum_px must be an integer",
                )
                actual_min = row.get("minimum_px")
                expect(
                    isinstance(actual_min, int) and not isinstance(actual_min, bool),
                    f"{path}: ledger row for {metric_id!r} must expose minimum_px for range expectations",
                )
                expect(
                    actual_min == expected_min,
                    f"{path}: metric_id {metric_id!r} expected_minimum_px={expected_min} does not match ledger minimum_px={actual_min}",
                )

                optional_checks = [
                    ("expected_recommended_default_min_px", "recommended_default_min_px"),
                    ("expected_recommended_default_max_px", "recommended_default_max_px"),
                    ("expected_recommended_maximum_px", "recommended_maximum_px"),
                ]
                for expected_key, actual_key in optional_checks:
                    if expected_key in expectation:
                        expected_val = expectation.get(expected_key)
                        expect(
                            isinstance(expected_val, int) and not isinstance(expected_val, bool),
                            f"{path}: metric_expectations[{idx}].{expected_key} must be an integer",
                        )
                        actual_val = row.get(actual_key)
                        expect(
                            isinstance(actual_val, int) and not isinstance(actual_val, bool),
                            f"{path}: ledger row for {metric_id!r} must expose {actual_key} when a case expects it",
                        )
                        expect(
                            actual_val == expected_val,
                            (
                                f"{path}: metric_id {metric_id!r} {expected_key}={expected_val} "
                                f"does not match ledger {actual_key}={actual_val}"
                            ),
                        )

        print(path.relative_to(REPO_ROOT))

    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except ValidationError as exc:
        print(exc, file=sys.stderr)
        raise SystemExit(1)

