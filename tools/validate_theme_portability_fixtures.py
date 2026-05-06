#!/usr/bin/env python3
from __future__ import annotations

import json
import pathlib
import sys

import yaml
from jsonschema import Draft202012Validator


def main() -> int:
    repo_root = pathlib.Path(__file__).resolve().parents[1]
    schema_path = repo_root / "schemas/design/theme_portability_record.schema.json"
    fixture_dir = repo_root / "fixtures/design/theme_portability_cases"

    schema = json.loads(schema_path.read_text(encoding="utf-8"))
    validator = Draft202012Validator(schema)

    fixture_paths = sorted(fixture_dir.glob("*.yaml"))
    if not fixture_paths:
        print(f"No fixtures found under {fixture_dir}", file=sys.stderr)
        return 2

    error_count = 0
    for fixture_path in fixture_paths:
        payload = yaml.safe_load(fixture_path.read_text(encoding="utf-8"))
        errors = sorted(validator.iter_errors(payload), key=lambda e: list(e.path))
        if not errors:
            continue

        error_count += len(errors)
        print(f"{fixture_path.relative_to(repo_root)}:")
        for error in errors:
            location = ".".join(str(part) for part in error.path) or "<root>"
            print(f"  - {location}: {error.message}")

    if error_count:
        print(f"\nFAILED: {error_count} validation error(s)", file=sys.stderr)
        return 1

    print(
        "OK: validated "
        f"{len(fixture_paths)} theme portability fixture(s) against {schema_path.relative_to(repo_root)}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

