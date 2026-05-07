#!/usr/bin/env python3
from __future__ import annotations

import json
import pathlib
import sys
from typing import Any

import yaml
from jsonschema import Draft202012Validator


def load_schema(repo_root: pathlib.Path) -> Draft202012Validator:
    schema_path = repo_root / "schemas/release/release_provenance_crosswalk.schema.json"
    schema = json.loads(schema_path.read_text(encoding="utf-8"))
    return Draft202012Validator(schema)


def load_yaml(path: pathlib.Path) -> Any:
    return yaml.safe_load(path.read_text(encoding="utf-8"))


def iter_fixture_paths(repo_root: pathlib.Path) -> list[pathlib.Path]:
    fixture_dir = repo_root / "fixtures/release/release_center_linkage_cases"
    return sorted(fixture_dir.glob("*.yaml"))


def validate_path(validator: Draft202012Validator, path: pathlib.Path) -> list[str]:
    payload = load_yaml(path)
    errors = sorted(validator.iter_errors(payload), key=lambda e: list(e.path))
    messages: list[str] = []
    for err in errors:
        loc = "/".join(map(str, err.path))
        prefix = f"{path}:{loc}" if loc else str(path)
        messages.append(f"{prefix}: {err.message}")
    return messages


def validate_fixture_refs(repo_root: pathlib.Path, crosswalk: dict[str, Any], fixture_paths: list[pathlib.Path]) -> list[str]:
    errors: list[str] = []
    expected = {str(p.relative_to(repo_root)) for p in fixture_paths}
    referenced = crosswalk.get("case_fixture_refs") or []
    if not isinstance(referenced, list):
        return ["artifacts/release/release_support_crosswalk.yaml: case_fixture_refs must be an array"]

    missing = sorted(set(referenced) - expected)
    extra = sorted(expected - set(referenced))
    if missing:
        errors.append(
            "artifacts/release/release_support_crosswalk.yaml: case_fixture_refs references missing fixtures: "
            + ", ".join(missing)
        )
    if extra:
        errors.append(
            "artifacts/release/release_support_crosswalk.yaml: case_fixture_refs does not list fixtures: "
            + ", ".join(extra)
        )
    return errors


def validate_scenario_coverage(fixture_payloads: list[dict[str, Any]]) -> list[str]:
    scenarios = {payload.get("scenario_class") for payload in fixture_payloads}
    required = {
        "normal_release",
        "mirrored_release",
        "hotfix_symbol_update",
        "revoked_artifact",
        "rollback_candidate",
        "support_bundle_docs_stale_or_unavailable",
    }
    missing = sorted(required - scenarios)
    if missing:
        return [f"fixtures/release/release_center_linkage_cases: missing scenario coverage: {', '.join(missing)}"]
    return []


def main() -> int:
    repo_root = pathlib.Path(__file__).resolve().parents[1]
    validator = load_schema(repo_root)

    crosswalk_path = repo_root / "artifacts/release/release_support_crosswalk.yaml"
    fixture_paths = iter_fixture_paths(repo_root)
    targets = [crosswalk_path, *fixture_paths]

    all_errors: list[str] = []
    for target in targets:
        all_errors.extend(validate_path(validator, target))

    crosswalk_payload = load_yaml(crosswalk_path)
    if isinstance(crosswalk_payload, dict):
        all_errors.extend(validate_fixture_refs(repo_root, crosswalk_payload, fixture_paths))

    fixture_payloads: list[dict[str, Any]] = []
    for path in fixture_paths:
        payload = load_yaml(path)
        if isinstance(payload, dict):
            fixture_payloads.append(payload)
    all_errors.extend(validate_scenario_coverage(fixture_payloads))

    if all_errors:
        for message in all_errors:
            print(f"[validate-release-provenance-crosswalk] error: {message}", file=sys.stderr)
        return 1

    print("[validate-release-provenance-crosswalk] OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

