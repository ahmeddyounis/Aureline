#!/usr/bin/env python3
# SPDX-FileCopyrightText: 2026 Aureline contributors
# SPDX-License-Identifier: Apache-2.0

"""Validate migration schemas against their protected positive and privacy cases."""

from __future__ import annotations

import copy
import json
from pathlib import Path
from typing import Any

from jsonschema import Draft202012Validator, FormatChecker


REPO_ROOT = Path(__file__).resolve().parents[2]
MAPPING_SCHEMA = Path("artifacts/migration/m3/mapping_report.schema.json")
MAPPING_FIXTURE = Path("fixtures/migration/m3/migration_wizard/mapping_report.json")
MIGRATION_CENTER_SCHEMA = Path("schemas/ux/migration_center_beta.schema.json")
MIGRATION_CENTER_FIXTURES = (
    Path("fixtures/ux/m3/migration_center/page.json"),
    Path("fixtures/ux/m3/migration_center/support_export.json"),
)
STABLE_SCHEMA = Path(
    "schemas/ux/finish-the-migration-center-diff-rollback-and-unsupported.schema.json"
)
STABLE_FIXTURE_ROOT = Path(
    "fixtures/ux/m4/finish-the-migration-center-diff-rollback-and-unsupported"
)
STABLE_FIXTURES = tuple(
    STABLE_FIXTURE_ROOT / name
    for name in (
        "vs_code_code_oss.json",
        "jetbrains_family.json",
        "vim_neovim.json",
        "emacs.json",
    )
)


def load_json(relative: Path) -> Any:
    return json.loads((REPO_ROOT / relative).read_text(encoding="utf-8"))


def validator_for(relative: Path) -> Draft202012Validator:
    schema = load_json(relative)
    Draft202012Validator.check_schema(schema)
    return Draft202012Validator(schema, format_checker=FormatChecker())


def first_error(validator: Draft202012Validator, payload: Any) -> str | None:
    errors = sorted(validator.iter_errors(payload), key=lambda error: list(error.path))
    if not errors:
        return None
    error = errors[0]
    return f"{error.json_path}: {error.message}"


def expect_valid(
    failures: list[str],
    validator: Draft202012Validator,
    fixture: Path,
) -> Any:
    payload = load_json(fixture)
    error = first_error(validator, payload)
    if error is not None:
        failures.append(f"{fixture}: {error}")
    return payload


def expect_invalid(
    failures: list[str],
    validator: Draft202012Validator,
    case_id: str,
    payload: Any,
) -> None:
    if first_error(validator, payload) is None:
        failures.append(f"{case_id}: privacy/shape mutation unexpectedly validated")


def main() -> int:
    failures: list[str] = []
    mapping_validator = validator_for(MAPPING_SCHEMA)
    center_validator = validator_for(MIGRATION_CENTER_SCHEMA)
    stable_validator = validator_for(STABLE_SCHEMA)

    mapping = expect_valid(failures, mapping_validator, MAPPING_FIXTURE)
    center_payloads = [
        expect_valid(failures, center_validator, fixture)
        for fixture in MIGRATION_CENTER_FIXTURES
    ]
    stable_payloads = [
        expect_valid(failures, stable_validator, fixture) for fixture in STABLE_FIXTURES
    ]

    if isinstance(mapping, dict) and mapping.get("rows"):
        private_target = copy.deepcopy(mapping)
        private_target["descriptors"]["target_descriptor"] = (
            "/Users/alice/Private Workspace"
        )
        expect_invalid(
            failures,
            mapping_validator,
            "mapping_report_rejects_private_target_path",
            private_target,
        )

        file_uri_target = copy.deepcopy(mapping)
        file_uri_target["descriptors"]["target_descriptor"] = (
            "FILE:/Users/alice/Private"
        )
        expect_invalid(
            failures,
            mapping_validator,
            "mapping_report_rejects_file_uri_target",
            file_uri_target,
        )

        empty_support_pivot = copy.deepcopy(mapping)
        empty_support_pivot["rows"][0]["support_export_refs"] = []
        expect_invalid(
            failures,
            mapping_validator,
            "mapping_report_requires_every_row_support_pivot",
            empty_support_pivot,
        )

        legacy_row_id = copy.deepcopy(mapping)
        legacy_row_id["rows"][0]["row_id"] = "import-diff-row:legacy:settings:item"
        expect_invalid(
            failures,
            mapping_validator,
            "mapping_report_rejects_unclassified_row_id",
            legacy_row_id,
        )

    if center_payloads and isinstance(center_payloads[0], dict):
        private_entry = copy.deepcopy(center_payloads[0])
        private_entry["entries"][0]["title_label"] = (
            "https://alice@example.invalid/private?token=secret"
        )
        expect_invalid(
            failures,
            center_validator,
            "migration_center_rejects_private_support_label",
            private_entry,
        )

        delimited_private_entry = copy.deepcopy(center_payloads[0])
        delimited_private_entry["entries"][0]["title_label"] = (
            "target=/Users/alice/private/settings.json"
        )
        expect_invalid(
            failures,
            center_validator,
            "migration_center_rejects_delimited_private_path",
            delimited_private_entry,
        )

    if stable_payloads and isinstance(stable_payloads[0], dict):
        private_title = copy.deepcopy(stable_payloads[0])
        private_title["title"] = "../customer/private/settings.json"
        expect_invalid(
            failures,
            stable_validator,
            "stable_migration_disclosure_rejects_private_title",
            private_title,
        )

        file_uri_title = copy.deepcopy(stable_payloads[0])
        file_uri_title["title"] = "File:/Users/alice/private/settings.json"
        expect_invalid(
            failures,
            stable_validator,
            "stable_migration_disclosure_rejects_file_uri_title",
            file_uri_title,
        )

    if failures:
        for failure in failures:
            print(f"[migration-contract-fixtures] error: {failure}")
        return 1

    positive_count = 1 + len(MIGRATION_CENTER_FIXTURES) + len(STABLE_FIXTURES)
    print(
        "[migration-contract-fixtures] "
        f"validated {positive_count} positive fixtures and 8 negative boundary cases"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
