#!/usr/bin/env python3
from __future__ import annotations

import json
import pathlib
import sys
from typing import Any

import yaml
from jsonschema import Draft202012Validator
from referencing import Registry
from referencing.exceptions import NoSuchResource
from referencing.jsonschema import DRAFT202012


def repo_root() -> pathlib.Path:
    return pathlib.Path(__file__).resolve().parents[1]


def load_yaml(path: pathlib.Path) -> Any:
    return yaml.safe_load(path.read_text(encoding="utf-8"))


def load_json(path: pathlib.Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def build_schema_registry(root: pathlib.Path) -> Registry[Any]:
    base = "https://aureline.dev/"

    def retrieve(uri: str):  # type: ignore[override]
        if uri.startswith(base):
            rel = uri[len(base) :]
            path = root / rel
            if path.exists():
                return DRAFT202012.create_resource(load_json(path))
        raise NoSuchResource(ref=uri)

    return Registry(retrieve=retrieve)


def validate_fixture(
    *,
    root: pathlib.Path,
    registry: Registry[Any],
    fixture_path: pathlib.Path,
    schema_rel: str,
) -> list[str]:
    schema_path = root / schema_rel
    schema = load_json(schema_path)
    validator = Draft202012Validator(schema, registry=registry)
    payload = load_yaml(fixture_path)

    errors = sorted(validator.iter_errors(payload), key=lambda e: list(e.path))
    messages: list[str] = []
    for err in errors:
        loc = "/".join(map(str, err.path))
        prefix = f"{fixture_path}:{loc}" if loc else str(fixture_path)
        messages.append(f"{prefix}: {err.message}")
    return messages


def main() -> int:
    root = repo_root()
    registry = build_schema_registry(root)

    manifest_path = root / "fixtures/security/trust_root_cases/manifest.yaml"
    manifest = load_yaml(manifest_path)
    if not isinstance(manifest, dict):
        print(f"[validate-trust-root-cases] {manifest_path}: expected mapping", file=sys.stderr)
        return 2

    cases = manifest.get("cases") or []
    if not isinstance(cases, list) or not cases:
        print(f"[validate-trust-root-cases] {manifest_path}: missing cases[]", file=sys.stderr)
        return 2

    failures: list[str] = []
    for case in cases:
        if not isinstance(case, dict):
            failures.append(f"{manifest_path}: case entries must be mappings")
            continue
        fixture_name = case.get("fixture")
        schema_ref = case.get("schema_ref")
        if not isinstance(fixture_name, str) or not fixture_name:
            failures.append(f"{manifest_path}: case missing fixture")
            continue
        if not isinstance(schema_ref, str) or not schema_ref:
            failures.append(f"{manifest_path}: {fixture_name}: case missing schema_ref")
            continue

        fixture_path = manifest_path.parent / fixture_name
        if not fixture_path.exists():
            failures.append(f"{manifest_path}: missing fixture file {fixture_path}")
            continue

        failures.extend(
            validate_fixture(
                root=root,
                registry=registry,
                fixture_path=fixture_path,
                schema_rel=schema_ref,
            )
        )

    if failures:
        print("[validate-trust-root-cases] FAIL", file=sys.stderr)
        for line in failures[:200]:
            print(f"  - {line}", file=sys.stderr)
        if len(failures) > 200:
            print(f"  ... {len(failures) - 200} more", file=sys.stderr)
        return 1

    print("[validate-trust-root-cases] OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

