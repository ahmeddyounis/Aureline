#!/usr/bin/env python3
"""Validate the continuity-claim matrix fixtures against their canonical schema.

The fixtures under ``fixtures/continuity/m5-continuity-profile-cases`` are
emitted by ``cargo run -p aureline-continuity --example
dump_m5_continuity_claim_matrix_fixtures``. They are matrix page records, a
matrix summary record, and a support-export record; every file must validate
against ``schemas/continuity/m5-continuity-claim-row.schema.json``.
"""

from __future__ import annotations

import json
import pathlib
import sys

from jsonschema import Draft202012Validator
from referencing import Registry, Resource
from referencing.exceptions import NoSuchResource
from referencing.jsonschema import DRAFT202012

AURELINE_SCHEMA_PREFIX = "https://aureline.dev/schemas/"


def retrieve_aureline_schema(repo_root: pathlib.Path, uri: str) -> Resource:
    if not uri.startswith(AURELINE_SCHEMA_PREFIX):
        raise NoSuchResource(ref=uri)

    rel = uri.removeprefix(AURELINE_SCHEMA_PREFIX)
    candidate = repo_root / "schemas" / rel
    if not candidate.exists():
        raise NoSuchResource(ref=uri)

    contents = json.loads(candidate.read_text(encoding="utf-8"))
    return Resource.from_contents(contents, default_specification=DRAFT202012)


def main() -> int:
    repo_root = pathlib.Path(__file__).resolve().parents[1]
    schema_path = repo_root / "schemas/continuity/m5-continuity-claim-row.schema.json"
    fixture_dir = repo_root / "fixtures/continuity/m5-continuity-profile-cases"

    if not schema_path.exists():
        print(f"Missing schema: {schema_path.relative_to(repo_root)}", file=sys.stderr)
        return 2

    if not fixture_dir.exists():
        print(f"Missing fixtures directory: {fixture_dir.relative_to(repo_root)}", file=sys.stderr)
        return 2

    schema = json.loads(schema_path.read_text(encoding="utf-8"))
    registry = Registry(retrieve=lambda uri: retrieve_aureline_schema(repo_root, uri))
    validator = Draft202012Validator(schema, registry=registry)

    fixture_paths = sorted(path for path in fixture_dir.glob("*.json"))
    if not fixture_paths:
        print(f"No fixtures found under {fixture_dir.relative_to(repo_root)}", file=sys.stderr)
        return 2

    error_count = 0
    for fixture_path in fixture_paths:
        try:
            payload = json.loads(fixture_path.read_text(encoding="utf-8"))
        except Exception as exc:
            error_count += 1
            print(f"{fixture_path.relative_to(repo_root)}:", file=sys.stderr)
            print(f"  - json: {exc}", file=sys.stderr)
            continue

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
        f"{len(fixture_paths)} continuity-claim matrix fixture(s) against "
        f"{schema_path.relative_to(repo_root)}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
