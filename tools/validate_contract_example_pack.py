#!/usr/bin/env python3
"""Validate the reference contract example pack index and gated examples.

Validates:
- artifacts/contracts/example_pack_index.yaml against schemas/contracts/example_pack_index.schema.json
- referenced payload + schema paths exist (unless sentinel)
- ci_required examples validate against their schema_ref after stripping
  fixture-only metadata keys ($schema, __fixture__).

Some example files are wrappers with `records: [...]`; in that case each record
is validated (the wrapper itself is not).
"""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any, Iterable

import yaml
from jsonschema import Draft202012Validator
from referencing import Registry, Resource
from referencing.exceptions import NoSuchResource
from referencing.jsonschema import DRAFT202012


REPO_ROOT = Path(__file__).resolve().parents[1]
INDEX_PATH = REPO_ROOT / "artifacts/contracts/example_pack_index.yaml"
INDEX_SCHEMA_PATH = REPO_ROOT / "schemas/contracts/example_pack_index.schema.json"

AURELINE_SCHEMA_PREFIX = "https://aureline.dev/schemas/"
SENTINEL_REFS = {"not_yet_seeded", "outline_only", "contract_not_yet_seeded"}


def retrieve_aureline_schema(uri: str) -> Resource:
    if not uri.startswith(AURELINE_SCHEMA_PREFIX):
        raise NoSuchResource(ref=uri)
    rel = uri.removeprefix(AURELINE_SCHEMA_PREFIX)
    candidate = REPO_ROOT / "schemas" / rel
    if not candidate.exists():
        raise NoSuchResource(ref=uri)
    contents = json.loads(candidate.read_text(encoding="utf-8"))
    return Resource.from_contents(contents, default_specification=DRAFT202012)


def is_path_ref(value: str) -> bool:
    if not value:
        return False
    if value in SENTINEL_REFS:
        return False
    if "://" in value:
        return False
    return "/" in value or value.endswith((".json", ".yaml", ".yml", ".md", ".py", ".sh", ".toml"))


def strip_fixture_metadata(value: Any) -> Any:
    if isinstance(value, dict):
        stripped: dict[str, Any] = {}
        for key, item in value.items():
            if key in {"$schema", "__fixture__"}:
                continue
            stripped[key] = strip_fixture_metadata(item)
        return stripped
    if isinstance(value, list):
        return [strip_fixture_metadata(item) for item in value]
    return value


def iter_instances(payload: Any) -> Iterable[tuple[Any, str]]:
    if isinstance(payload, dict):
        records = payload.get("records")
        if isinstance(records, list):
            for idx, record in enumerate(records):
                yield record, f"records[{idx}]"
            return
    if isinstance(payload, list):
        for idx, item in enumerate(payload):
            yield item, f"[{idx}]"
        return
    yield payload, "<root>"


def load_example_payload(path: Path) -> Any:
    if path.suffix == ".json":
        return json.loads(path.read_text(encoding="utf-8"))
    if path.suffix in {".yaml", ".yml"}:
        return yaml.safe_load(path.read_text(encoding="utf-8"))
    raise ValueError(f"unsupported payload format: {path}")


def load_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def validate_index(index_payload: Any) -> list[str]:
    schema = load_json(INDEX_SCHEMA_PATH)
    validator = Draft202012Validator(schema)
    errors = sorted(validator.iter_errors(index_payload), key=lambda e: list(e.path))
    messages: list[str] = []
    for err in errors:
        loc = "/".join(map(str, err.path)) or "<root>"
        messages.append(f"{INDEX_PATH.relative_to(REPO_ROOT)}:{loc}: {err.message}")
    return messages


def validate_paths_exist(index_payload: dict[str, Any]) -> list[str]:
    messages: list[str] = []
    for key in ("overview_page", "redaction_rules_page", "schema_ref"):
        ref = index_payload.get(key)
        if isinstance(ref, str) and is_path_ref(ref):
            candidate = REPO_ROOT / ref
            if not candidate.exists():
                messages.append(f"{INDEX_PATH.relative_to(REPO_ROOT)}: missing {key}: {ref}")

    families = index_payload.get("families") or []
    examples = index_payload.get("examples") or []
    example_ids = {
        row.get("example_id")
        for row in examples
        if isinstance(row, dict) and isinstance(row.get("example_id"), str) and row.get("example_id")
    }

    for family in families:
        if not isinstance(family, dict):
            continue
        for ref in family.get("schema_refs") or []:
            if isinstance(ref, str) and is_path_ref(ref) and not (REPO_ROOT / ref).exists():
                messages.append(f"{INDEX_PATH.relative_to(REPO_ROOT)}: missing schema_ref: {ref}")
        for ref in family.get("primary_doc_refs") or []:
            if isinstance(ref, str) and is_path_ref(ref) and not (REPO_ROOT / ref).exists():
                messages.append(f"{INDEX_PATH.relative_to(REPO_ROOT)}: missing primary_doc_ref: {ref}")
        for example_id in family.get("example_ids") or []:
            if isinstance(example_id, str) and example_id and example_id not in example_ids:
                messages.append(f"{INDEX_PATH.relative_to(REPO_ROOT)}: family lists unknown example_id: {example_id}")

    for example in examples:
        if not isinstance(example, dict):
            continue
        payload_ref = example.get("payload_ref")
        schema_ref = example.get("schema_ref")
        for label, ref in (("payload_ref", payload_ref), ("schema_ref", schema_ref)):
            if isinstance(ref, str) and is_path_ref(ref):
                candidate = REPO_ROOT / ref
                if not candidate.exists():
                    messages.append(
                        f"{INDEX_PATH.relative_to(REPO_ROOT)}: example {example.get('example_id','<unknown>')}: missing {label}: {ref}"
                    )
    return messages


def validate_family_links(index_payload: dict[str, Any]) -> list[str]:
    messages: list[str] = []
    families = index_payload.get("families") or []
    examples = index_payload.get("examples") or []

    family_ids: set[str] = set()
    for row in families:
        if isinstance(row, dict) and isinstance(row.get("family_id"), str):
            family_ids.add(row["family_id"])

    for row in examples:
        if not isinstance(row, dict):
            continue
        family_id = row.get("family_id")
        if isinstance(family_id, str) and family_id and family_id not in family_ids:
            messages.append(
                f"{INDEX_PATH.relative_to(REPO_ROOT)}: example {row.get('example_id','<unknown>')} references unknown family_id {family_id}"
            )
    return messages


def validate_examples(index_payload: dict[str, Any]) -> list[str]:
    messages: list[str] = []
    registry = Registry(retrieve=lambda uri: retrieve_aureline_schema(uri))
    validator_cache: dict[str, Draft202012Validator] = {}

    for row in index_payload.get("examples") or []:
        if not isinstance(row, dict):
            continue
        if row.get("validation_gate") != "ci_required":
            continue

        example_id = row.get("example_id", "<unknown>")
        payload_ref = row.get("payload_ref")
        schema_ref = row.get("schema_ref")
        if not isinstance(payload_ref, str) or not is_path_ref(payload_ref):
            continue
        if not isinstance(schema_ref, str) or not is_path_ref(schema_ref):
            continue

        payload_path = REPO_ROOT / payload_ref
        schema_path = REPO_ROOT / schema_ref
        try:
            payload = load_example_payload(payload_path)
        except Exception as exc:
            messages.append(f"{payload_ref}: failed to load payload: {exc}")
            continue

        try:
            schema = load_json(schema_path)
        except Exception as exc:
            messages.append(f"{schema_ref}: failed to load schema: {exc}")
            continue

        cache_key = schema_ref
        validator = validator_cache.get(cache_key)
        if validator is None:
            validator = Draft202012Validator(schema, registry=registry)
            validator_cache[cache_key] = validator

        for instance, where in iter_instances(payload):
            instance = strip_fixture_metadata(instance)
            errors = sorted(validator.iter_errors(instance), key=lambda e: list(e.path))
            for err in errors[:50]:
                loc = ".".join(map(str, err.path)) or "<root>"
                messages.append(f"{payload_ref}:{where}:{loc}: {example_id}: {err.message}")

    return messages


def main() -> int:
    if not INDEX_PATH.exists():
        print(f"[example-pack] error: missing index at {INDEX_PATH.relative_to(REPO_ROOT)}", file=sys.stderr)
        return 2
    if not INDEX_SCHEMA_PATH.exists():
        print(f"[example-pack] error: missing schema at {INDEX_SCHEMA_PATH.relative_to(REPO_ROOT)}", file=sys.stderr)
        return 2

    try:
        index_payload = yaml.safe_load(INDEX_PATH.read_text(encoding="utf-8"))
    except Exception as exc:
        print(f"[example-pack] error: failed to parse index YAML: {exc}", file=sys.stderr)
        return 2

    if not isinstance(index_payload, dict):
        print(f"[example-pack] error: index must parse as a mapping", file=sys.stderr)
        return 2

    errors: list[str] = []
    errors.extend(validate_index(index_payload))
    errors.extend(validate_paths_exist(index_payload))
    errors.extend(validate_family_links(index_payload))
    errors.extend(validate_examples(index_payload))

    if errors:
        print("[example-pack] FAIL", file=sys.stderr)
        for msg in errors[:200]:
            print(f"  - {msg}", file=sys.stderr)
        if len(errors) > 200:
            print(f"  ... ({len(errors) - 200} more)", file=sys.stderr)
        return 1

    print(
        "[example-pack] OK: validated "
        f"{INDEX_PATH.relative_to(REPO_ROOT)} + ci_required example payloads"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

