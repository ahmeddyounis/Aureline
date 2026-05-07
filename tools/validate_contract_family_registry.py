#!/usr/bin/env python3
"""Validate the contract-family registry and worked example fixtures.

The registry is YAML, but the boundary schema is JSON Schema draft 2020-12.
To avoid adding a Python YAML dependency, this validator parses YAML via
Ruby/Psych (bundled on macOS and standard CI images) and validates the resulting
JSON via `jsonschema`.
"""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parent.parent
SCHEMA_PATH = REPO_ROOT / "schemas/contracts/contract_family_registry.schema.json"
REGISTRY_PATH = REPO_ROOT / "artifacts/contracts/contract_families.yaml"
EXAMPLES_DIR = REPO_ROOT / "fixtures/contracts/contract_family_examples"

COMPAT_SURFACES_PATH = REPO_ROOT / "artifacts/governance/compatibility_surfaces.yaml"
QUAL_MATRIX_PATH = REPO_ROOT / "artifacts/compat/qualification_matrix_seed.yaml"


def load_yaml_via_ruby(path: Path) -> Any:
    payload = subprocess.run(
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
    if payload.returncode != 0:
        stderr = payload.stderr.strip() or "unknown Ruby/Psych failure"
        raise SystemExit(f"[validate-contract-families] failed to parse YAML at {path}: {stderr}")
    return json.loads(payload.stdout)


def iter_paths(value: Any) -> list[str]:
    if isinstance(value, str):
        return [value]
    if isinstance(value, list):
        paths: list[str] = []
        for item in value:
            paths.extend(iter_paths(item))
        return paths
    if isinstance(value, dict):
        paths = []
        for item in value.values():
            paths.extend(iter_paths(item))
        return paths
    return []


def load_surface_ids() -> set[str]:
    payload = load_yaml_via_ruby(COMPAT_SURFACES_PATH)
    surfaces: set[str] = set()
    for row in payload.get("rows", []):
        surface_id = row.get("surface_id")
        if isinstance(surface_id, str):
            surfaces.add(surface_id)
    return surfaces


def load_qualification_row_ids() -> set[str]:
    payload = load_yaml_via_ruby(QUAL_MATRIX_PATH)
    rows: set[str] = set()
    for row in payload.get("qualification_rows", []):
        row_id = row.get("row_id")
        if isinstance(row_id, str):
            rows.add(row_id)
    return rows


def validate_paths_exist(label: str, owner: Path, paths: list[str]) -> list[str]:
    missing: list[str] = []
    for ref in paths:
        if not isinstance(ref, str) or not ref:
            continue
        if "://" in ref:
            continue
        if "#" in ref:
            candidate = ref.split("#", 1)[0]
        else:
            candidate = ref
        if candidate in {"outline_only", "not_yet_seeded", "contract_not_yet_seeded"}:
            continue
        path = owner / candidate
        if not path.exists():
            missing.append(ref)
    return missing


def main() -> int:
    try:
        from jsonschema import Draft202012Validator
    except Exception as exc:  # pragma: no cover
        print(f"[validate-contract-families] error: python jsonschema is required: {exc}", file=sys.stderr)
        return 2

    if not SCHEMA_PATH.exists():
        print(f"[validate-contract-families] error: missing schema at {SCHEMA_PATH}", file=sys.stderr)
        return 2
    if not REGISTRY_PATH.exists():
        print(f"[validate-contract-families] error: missing registry at {REGISTRY_PATH}", file=sys.stderr)
        return 2

    schema = json.loads(SCHEMA_PATH.read_text(encoding="utf-8"))
    validator = Draft202012Validator(schema)

    failures = 0

    registry = load_yaml_via_ruby(REGISTRY_PATH)
    errors = sorted(validator.iter_errors(registry), key=lambda e: list(e.path))
    if errors:
        failures += 1
        print(f"[validate-contract-families] {REGISTRY_PATH.relative_to(REPO_ROOT)}: {len(errors)} error(s)")
        for err in errors[:20]:
            loc = "/".join(str(part) for part in err.path) or "(root)"
            print(f"  - {loc}: {err.message}")

    family_rows = registry.get("rows", [])
    family_ids: list[str] = []
    for row in family_rows:
        if isinstance(row, dict) and isinstance(row.get("family_id"), str):
            family_ids.append(row["family_id"])
    family_id_set = set(family_ids)
    if len(family_ids) != len(family_id_set):
        failures += 1
        seen: set[str] = set()
        dupes: set[str] = set()
        for family_id in family_ids:
            if family_id in seen:
                dupes.add(family_id)
            seen.add(family_id)
        print(f"[validate-contract-families] error: duplicate family_id(s): {', '.join(sorted(dupes))}", file=sys.stderr)

    surface_ids = load_surface_ids()
    qualification_ids = load_qualification_row_ids()

    for row in family_rows:
        if not isinstance(row, dict):
            continue
        family_id = row.get("family_id", "<unknown>")
        missing_refs: list[str] = []
        missing_refs.extend(validate_paths_exist("primary_doc_refs", REPO_ROOT, row.get("primary_doc_refs", [])))
        missing_refs.extend(validate_paths_exist("example_source_refs", REPO_ROOT, row.get("example_source_refs", [])))
        if missing_refs:
            failures += 1
            print(f"[validate-contract-families] {family_id}: missing referenced path(s):")
            for ref in missing_refs[:30]:
                print(f"  - {ref}")

        links = row.get("compatibility_links", {})
        if isinstance(links, dict):
            unknown_surface_ids = sorted(
                {
                    surface_id
                    for surface_id in links.get("compatibility_surface_ids", [])
                    if isinstance(surface_id, str) and surface_id not in surface_ids
                }
            )
            if unknown_surface_ids:
                failures += 1
                print(f"[validate-contract-families] {family_id}: unknown compatibility_surface_ids:")
                for sid in unknown_surface_ids:
                    print(f"  - {sid}")

            unknown_qual = sorted(
                {
                    row_id
                    for row_id in links.get("qualification_row_refs", [])
                    if isinstance(row_id, str) and row_id not in qualification_ids
                }
            )
            if unknown_qual:
                failures += 1
                print(f"[validate-contract-families] {family_id}: unknown qualification_row_refs:")
                for rid in unknown_qual:
                    print(f"  - {rid}")

    example_paths = sorted(EXAMPLES_DIR.glob("*.yaml"))
    if not example_paths:
        print(f"[validate-contract-families] error: no example fixtures under {EXAMPLES_DIR}", file=sys.stderr)
        return 2

    for path in example_paths:
        instance = load_yaml_via_ruby(path)
        errors = sorted(validator.iter_errors(instance), key=lambda e: list(e.path))
        if errors:
            failures += 1
            rel = path.relative_to(REPO_ROOT)
            print(f"[validate-contract-families] {rel}: {len(errors)} error(s)")
            for err in errors[:20]:
                loc = "/".join(str(part) for part in err.path) or "(root)"
                print(f"  - {loc}: {err.message}")
            continue

        family_id = instance.get("family_id")
        if isinstance(family_id, str) and family_id not in family_id_set:
            failures += 1
            rel = path.relative_to(REPO_ROOT)
            print(
                f"[validate-contract-families] {rel}: family_id '{family_id}' not found in registry rows",
                file=sys.stderr,
            )

    if failures:
        print(f"[validate-contract-families] FAIL ({failures} problem(s))", file=sys.stderr)
        return 1

    print("[validate-contract-families] OK (registry and contract-family examples validate)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
