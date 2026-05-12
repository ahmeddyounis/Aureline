#!/usr/bin/env python3
"""Validate the checked-in build identity artifact.

Validates:
- artifacts/build/build_identity.json exists
- payload validates against schemas/build/build_identity.schema.json
- checked-in string fields are not the placeholder literal 'unknown'

This is intentionally lightweight so proof indices and CI checks can depend on
it without running a full build.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

from jsonschema import Draft202012Validator


REPO_ROOT = Path(__file__).resolve().parents[1]
BUILD_IDENTITY_PATH = REPO_ROOT / "artifacts/build/build_identity.json"
SCHEMA_PATH = REPO_ROOT / "schemas/build/build_identity.schema.json"


def load_json(path: Path) -> object:
    return json.loads(path.read_text(encoding="utf-8"))


def reject_unknown_fields(payload: object) -> bool:
    if not isinstance(payload, dict):
        return False

    unknown_fields = [
        field
        for field, value in payload.items()
        if isinstance(value, str) and value == "unknown"
    ]
    if not unknown_fields:
        return False

    rel = BUILD_IDENTITY_PATH.relative_to(REPO_ROOT)
    for field in unknown_fields:
        print(f"{rel}:{field}: must not be the literal string 'unknown'", file=sys.stderr)
    return True


def main() -> int:
    if not BUILD_IDENTITY_PATH.exists():
        print(f"missing build identity artifact: {BUILD_IDENTITY_PATH.relative_to(REPO_ROOT)}", file=sys.stderr)
        return 1
    if not SCHEMA_PATH.exists():
        print(f"missing schema: {SCHEMA_PATH.relative_to(REPO_ROOT)}", file=sys.stderr)
        return 1

    schema = load_json(SCHEMA_PATH)
    payload = load_json(BUILD_IDENTITY_PATH)

    validator = Draft202012Validator(schema)
    errors = sorted(validator.iter_errors(payload), key=lambda e: list(e.path))
    if errors:
        for err in errors[:50]:
            loc = ".".join(map(str, err.path)) or "<root>"
            print(f"{BUILD_IDENTITY_PATH.relative_to(REPO_ROOT)}:{loc}: {err.message}", file=sys.stderr)
        if len(errors) > 50:
            print(f"... {len(errors) - 50} more errors", file=sys.stderr)
        return 1

    if reject_unknown_fields(payload):
        return 1

    print(f"ok: {BUILD_IDENTITY_PATH.relative_to(REPO_ROOT)} validates against {SCHEMA_PATH.relative_to(REPO_ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
