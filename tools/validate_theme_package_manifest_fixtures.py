#!/usr/bin/env python3
from __future__ import annotations

import json
import pathlib
import sys

import yaml
from jsonschema import Draft202012Validator


MODE_THEME_CLASS_MAP = {
    "dark": "dark_reference",
    "light": "light_parity",
    "hc-dark": "high_contrast_dark",
    "hc-light": "high_contrast_light",
}


def main() -> int:
    repo_root = pathlib.Path(__file__).resolve().parents[1]
    schema_path = repo_root / "schemas/design/theme_package_manifest.schema.json"
    fixture_dir = repo_root / "fixtures/design/theme_package_cases"
    matrix_path = repo_root / "artifacts/design/supported_mode_matrix.yaml"

    schema = json.loads(schema_path.read_text(encoding="utf-8"))
    validator = Draft202012Validator(schema)

    fixture_paths = sorted(fixture_dir.glob("*.yaml"))
    if not fixture_paths:
        print(f"No fixtures found under {fixture_dir}", file=sys.stderr)
        return 2

    error_count = 0

    if not matrix_path.exists():
        print(f"Missing supported-mode matrix: {matrix_path.relative_to(repo_root)}", file=sys.stderr)
        return 2

    try:
        matrix_payload = yaml.safe_load(matrix_path.read_text(encoding="utf-8"))
    except Exception as exc:
        print(f"Failed to parse {matrix_path.relative_to(repo_root)}: {exc}", file=sys.stderr)
        return 1

    rows = matrix_payload.get("rows")
    if not isinstance(rows, list) or not rows:
        print(f"{matrix_path.relative_to(repo_root)}: rows must be a non-empty list", file=sys.stderr)
        return 1

    claimant_families = {row.get("claimant_family_class") for row in rows if isinstance(row, dict)}
    required_families = {"first_party_surface", "imported_theme", "extension_surface", "embedded_surface"}
    missing_families = sorted(required_families - claimant_families)
    if missing_families:
        print(
            f"{matrix_path.relative_to(repo_root)}: missing claimant families: {', '.join(missing_families)}",
            file=sys.stderr,
        )
        error_count += 1

    for index, row in enumerate(rows):
        if not isinstance(row, dict):
            print(f"{matrix_path.relative_to(repo_root)}: rows[{index}] must be a mapping", file=sys.stderr)
            error_count += 1
            continue

        mode_class = row.get("mode_class")
        theme_class = row.get("theme_class")
        expected_theme_class = MODE_THEME_CLASS_MAP.get(mode_class)
        if expected_theme_class and theme_class != expected_theme_class:
            print(
                f"{matrix_path.relative_to(repo_root)}: rows[{index}] mode_class={mode_class} requires theme_class={expected_theme_class}",
                file=sys.stderr,
            )
            error_count += 1

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
        f"{len(fixture_paths)} theme-package fixture(s) against {schema_path.relative_to(repo_root)} "
        f"+ supported-mode matrix sanity checks ({matrix_path.relative_to(repo_root)})"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
