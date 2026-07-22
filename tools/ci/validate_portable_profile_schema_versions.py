#!/usr/bin/env python3
# SPDX-FileCopyrightText: 2026 Aureline contributors
# SPDX-License-Identifier: Apache-2.0

"""Validate portable-profile version-specific vocabulary fixtures."""

from __future__ import annotations

import json
import re
from pathlib import Path

from jsonschema import Draft202012Validator


REPO_ROOT = Path(__file__).resolve().parents[2]
SCHEMA_PATH = REPO_ROOT / "schemas/profile/portable_profile.schema.json"
CASES_PATH = REPO_ROOT / "fixtures/profile/schema_version_vocabulary_cases.json"


def main() -> int:
    schema = json.loads(SCHEMA_PATH.read_text(encoding="utf-8"))
    case_set = json.loads(CASES_PATH.read_text(encoding="utf-8"))
    Draft202012Validator.check_schema(schema)
    validator = Draft202012Validator(schema)
    failures: list[str] = []
    cases = case_set.get("cases")

    if case_set.get("record_kind") != "portable_profile_schema_version_vocabulary_case_set":
        failures.append("case set record_kind is invalid")
    if case_set.get("schema_version") != 1:
        failures.append("case set schema_version is invalid")
    if not isinstance(cases, list) or not cases or len(cases) > 64:
        failures.append("cases must be a non-empty bounded array")
        cases = []

    seen_case_ids: set[str] = set()
    for case in cases:
        if not isinstance(case, dict):
            failures.append("case row must be an object")
            continue
        case_id = case.get("case_id", "<missing-case-id>")
        if not isinstance(case_id, str) or not re.fullmatch(
            r"[a-z0-9][a-z0-9_]{0,127}", case_id
        ):
            failures.append("case_id must use the bounded stable-token grammar")
            continue
        if case_id in seen_case_ids:
            failures.append(f"{case_id}: duplicate case_id")
            continue
        seen_case_ids.add(case_id)
        if not isinstance(case.get("expected_valid"), bool):
            failures.append(f"{case_id}: expected_valid must be boolean")
            continue
        if not isinstance(case.get("payload"), dict):
            failures.append(f"{case_id}: payload must be an object")
            continue
        errors = sorted(
            validator.iter_errors(case.get("payload")), key=lambda error: list(error.path)
        )
        observed_valid = not errors
        expected_valid = case.get("expected_valid")
        if observed_valid != expected_valid:
            detail = errors[0].message if errors else "payload unexpectedly validated"
            failures.append(f"{case_id}: {detail}")

    if failures:
        for failure in failures:
            print(f"[portable-profile-schema-versions] error: {failure}")
        return 1

    print(
        "[portable-profile-schema-versions] "
        f"validated {len(cases)} version-vocabulary cases"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
