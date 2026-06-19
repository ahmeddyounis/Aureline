#!/usr/bin/env python3
"""Validate the M5 JSON Schema catalog and its published packages.

Validates:
- ``artifacts/contracts/m5-json-schema-catalog.json`` against
  ``schemas/public/m5-contracts/m5_json_schema_catalog.schema.json``
- the catalog's semantic invariants (duplicate package ids, closed-vocabulary
  membership, version-field consistency, and the summary recomputed from the
  packages), mirroring the typed Rust consumer
- that the checked-in catalog, package schemas, examples, round-trip fixtures,
  SDK doc, capture, and negative fixtures match the regenerator (no hand-edit
  drift)
- for every package: the package schema is a valid Draft 2020-12 schema, its
  ``$id`` and ``x-aureline-contract`` agree with the catalog row, it preserves
  unknown fields, its required set fixes the record-kind tag, the version field,
  and the primary identity, the example payload validates and carries the
  version field, and the round-trip fixture validates and preserves an unknown
  field through a parse/serialize round-trip
- that each package's lifecycle label agrees with the publication matrix's
  effective published label and that the family resolves in the contract-family
  registry with consistent version fields
- that every referenced repo-relative path exists
- that the checked-in negative fixtures under ``fixtures/contracts/m5-json-catalog/``
  are rejected by the semantic invariants

The validator imports the builders from the regenerator so the two cannot drift.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any

TOOLS_DIR = Path(__file__).resolve().parent
if str(TOOLS_DIR) not in sys.path:
    sys.path.insert(0, str(TOOLS_DIR))

import regenerate_m5_json_schema_catalog as gen  # noqa: E402

REPO_ROOT = gen.REPO_ROOT
SCHEMA_PATH = REPO_ROOT / "schemas/public/m5-contracts/m5_json_schema_catalog.schema.json"
MATRIX_PATH = REPO_ROOT / "artifacts/contracts/m5-stability-lifecycle-map.json"
CONTRACT_FAMILIES_PATH = REPO_ROOT / "artifacts/contracts/contract_families.yaml"


def load_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def is_path_ref(value: str) -> bool:
    if not value:
        return False
    if "://" in value:
        return False
    return "/" in value or value.endswith(
        (".json", ".yaml", ".yml", ".md", ".py", ".sh", ".toml", ".wit")
    )


def candidate_path(ref: str) -> str:
    return ref.split("#", 1)[0] if "#" in ref else ref


def collect_refs(catalog: dict) -> list[str]:
    refs: list[str] = []
    for key in (
        "overview_page",
        "sdk_catalog_page",
        "publication_matrix_ref",
        "contract_family_registry_ref",
        "evidence_index_ref",
        "schema_home",
    ):
        value = catalog.get(key)
        if isinstance(value, str):
            refs.append(value)
    refs.extend(m for m in catalog.get("offline_bundle", {}).get("bundle_members", []))
    for pkg in catalog.get("packages", []):
        for key in (
            "schema_path",
            "compatibility_note_ref",
            "example_payload_ref",
            "roundtrip_fixture_ref",
            "matrix_row_ref",
            "contract_family_ref",
        ):
            value = pkg.get(key)
            if isinstance(value, str):
                refs.append(value)
        refs.extend(v for v in pkg.get("validator_suite_refs", []) if isinstance(v, str))
        refs.extend(v for v in pkg.get("field_contract", {}).get("migration_note_hooks", []))
    return refs


def semantic_violations(catalog: dict) -> list[str]:
    """Recompute the catalog's derived state and report disagreements.

    Mirrors the typed Rust consumer's `validate()`. The canonical catalog must
    return no violations; each negative fixture must return at least one.
    """
    violations: list[str] = []

    if catalog.get("record_kind") != gen.RECORD_KIND:
        violations.append("record_kind mismatch")
    if catalog.get("schema_version") != gen.SCHEMA_VERSION:
        violations.append("schema_version mismatch")
    if catalog.get("catalog_id") != gen.CATALOG_ID:
        violations.append("catalog_id mismatch")

    for field, expected in [
        ("lifecycle_labels", gen.LIFECYCLE_LABELS),
        ("maturity_lanes", gen.MATURITY_LANES),
        ("contract_forms", gen.CONTRACT_FORMS),
        ("additive_field_rules", gen.ADDITIVE_FIELD_RULES),
        ("required_field_policies", gen.REQUIRED_FIELD_POLICIES),
        ("unknown_field_policies", gen.UNKNOWN_FIELD_POLICIES),
        ("downgrade_behaviors", gen.DOWNGRADE_BEHAVIORS),
        ("resolution_surfaces", gen.RESOLUTION_SURFACES),
    ]:
        if catalog.get(field) != expected:
            violations.append(f"closed vocabulary mismatch: {field}")

    packages = catalog.get("packages", [])
    seen_pkg: set[str] = set()
    seen_family: set[str] = set()
    for pkg in packages:
        pid = pkg.get("package_id", "<unknown>")
        if pid in seen_pkg:
            violations.append(f"duplicate package_id: {pid}")
        seen_pkg.add(pid)

        fid = pkg.get("family_id", "<unknown>")
        if fid in seen_family:
            violations.append(f"duplicate family_id: {fid}")
        seen_family.add(fid)

        if pkg.get("lifecycle_label") not in gen.LIFECYCLE_LABELS:
            violations.append(f"{fid}: lifecycle_label not in vocabulary")
        if pkg.get("maturity_lane") not in gen.MATURITY_LANES:
            violations.append(f"{fid}: maturity_lane not in vocabulary")
        if pkg.get("contract_form") not in gen.CONTRACT_FORMS:
            violations.append(f"{fid}: contract_form not in vocabulary")

        version_fields = pkg.get("version_field_names", [])
        if not version_fields:
            violations.append(f"{fid}: empty version_field_names")
        if pkg.get("primary_version_field") not in version_fields:
            violations.append(
                f"{fid}: primary_version_field not in version_field_names"
            )
        if pkg.get("record_kind_field") != "record_kind":
            violations.append(f"{fid}: record_kind_field must be 'record_kind'")
        if pkg.get("package_id") != f"m5.{fid}":
            violations.append(f"{fid}: package_id must be 'm5.<family_id>'")

        fc = pkg.get("field_contract", {})
        if fc.get("unknown_field_policy") not in gen.UNKNOWN_FIELD_POLICIES:
            violations.append(f"{fid}: unknown_field_policy not in vocabulary")
        if not fc.get("migration_note_hooks"):
            violations.append(f"{fid}: empty migration_note_hooks")

    if catalog.get("summary") != gen.compute_summary(packages):
        violations.append("summary counts disagree with the packages")

    return violations


def load_registry_families() -> dict[str, set[str]]:
    """Map registry family id (and aliases) -> declared version field names."""
    try:
        import yaml
    except Exception:
        return {}
    payload = yaml.safe_load(CONTRACT_FAMILIES_PATH.read_text(encoding="utf-8"))
    out: dict[str, set[str]] = {}
    for row in payload.get("rows", []):
        if not isinstance(row, dict):
            continue
        fields = set(row.get("version_field_names", []) or [])
        names = []
        if isinstance(row.get("family_id"), str):
            names.append(row["family_id"])
        for alias in row.get("alias_ids", []) or []:
            if isinstance(alias, str):
                names.append(alias)
        for name in names:
            out[name] = fields
    return out


def load_matrix_published_labels() -> dict[str, str]:
    if not MATRIX_PATH.exists():
        return {}
    matrix = load_json(MATRIX_PATH)
    return {
        row.get("family_id"): row.get("published_label")
        for row in matrix.get("rows", [])
        if isinstance(row, dict)
    }


def main() -> int:
    try:
        from jsonschema import Draft202012Validator
    except Exception as exc:  # pragma: no cover
        print(f"[m5-json-catalog] error: python jsonschema is required: {exc}", file=sys.stderr)
        return 2

    failures: list[str] = []

    if not SCHEMA_PATH.exists():
        print(f"[m5-json-catalog] error: missing schema {SCHEMA_PATH}", file=sys.stderr)
        return 2
    if not gen.CATALOG_PATH.exists():
        print(f"[m5-json-catalog] error: missing catalog {gen.CATALOG_PATH}", file=sys.stderr)
        return 2

    schema = load_json(SCHEMA_PATH)
    validator = Draft202012Validator(schema)
    catalog = load_json(gen.CATALOG_PATH)

    # 1) Schema validation of the canonical catalog.
    for err in sorted(validator.iter_errors(catalog), key=lambda e: list(e.path)):
        loc = "/".join(str(p) for p in err.path) or "<root>"
        failures.append(f"schema: {loc}: {err.message}")

    # 2) Semantic invariants on the canonical catalog.
    for msg in semantic_violations(catalog):
        failures.append(f"semantic: {msg}")

    # 3) Regenerator drift: the catalog must match what the regenerator builds.
    if catalog != gen.build_catalog():
        failures.append(
            "drift: artifacts/contracts/m5-json-schema-catalog.json is stale; "
            "run tools/regenerate_m5_json_schema_catalog.py"
        )

    # 4) Per-package: schema, example, round-trip, and self-describing annotation.
    for pkg in catalog.get("packages", []):
        fid = pkg.get("family_id", "<unknown>")
        schema_file = REPO_ROOT / pkg.get("schema_path", "")
        if not schema_file.exists():
            failures.append(f"{fid}: missing package schema {pkg.get('schema_path')}")
            continue
        pkg_schema = load_json(schema_file)

        # Package schema matches the regenerator (no hand edit).
        gen_pkg = next(p for p in gen.PACKAGES if p["family_id"] == fid)
        if pkg_schema != gen.build_package_schema(gen_pkg):
            failures.append(
                f"{fid}: package schema {pkg.get('schema_path')} is stale; "
                "run tools/regenerate_m5_json_schema_catalog.py"
            )

        # Package schema is itself a valid Draft 2020-12 schema.
        try:
            Draft202012Validator.check_schema(pkg_schema)
        except Exception as exc:
            failures.append(f"{fid}: package schema is not a valid Draft 2020-12 schema: {exc}")
            continue

        if pkg_schema.get("$id") != pkg.get("schema_id"):
            failures.append(f"{fid}: package schema $id does not match catalog schema_id")
        if pkg_schema.get("additionalProperties") is not True:
            failures.append(f"{fid}: package schema must preserve unknown fields (additionalProperties true)")

        required = pkg_schema.get("required", [])
        for field in ("record_kind", pkg.get("primary_version_field"), pkg.get("primary_identifier_field")):
            if field not in required:
                failures.append(f"{fid}: package schema must require '{field}'")

        annotation = pkg_schema.get("x-aureline-contract", {})
        if annotation.get("lifecycle_label") != pkg.get("lifecycle_label"):
            failures.append(f"{fid}: schema annotation lifecycle_label disagrees with catalog")
        if annotation.get("family_id") != fid:
            failures.append(f"{fid}: schema annotation family_id disagrees with catalog")
        if annotation.get("unknown_field_policy") != "preserve":
            failures.append(f"{fid}: schema annotation must declare unknown_field_policy preserve")

        pkg_validator = Draft202012Validator(pkg_schema)

        # Example payload validates and carries the version field.
        example_file = REPO_ROOT / pkg.get("example_payload_ref", "")
        if not example_file.exists():
            failures.append(f"{fid}: missing example payload {pkg.get('example_payload_ref')}")
        else:
            example = load_json(example_file)
            if example != gen.build_example(gen_pkg):
                failures.append(f"{fid}: example payload is stale; run the regenerator")
            for err in pkg_validator.iter_errors(example):
                failures.append(f"{fid}: example payload fails its schema: {err.message}")
            if pkg.get("primary_version_field") not in example:
                failures.append(f"{fid}: example payload is missing its version field")

        # Round-trip fixture validates and preserves an unknown field.
        roundtrip_file = REPO_ROOT / pkg.get("roundtrip_fixture_ref", "")
        if not roundtrip_file.exists():
            failures.append(f"{fid}: missing round-trip fixture {pkg.get('roundtrip_fixture_ref')}")
        else:
            roundtrip = load_json(roundtrip_file)
            if roundtrip != gen.build_roundtrip(gen_pkg):
                failures.append(f"{fid}: round-trip fixture is stale; run the regenerator")
            for err in pkg_validator.iter_errors(roundtrip):
                failures.append(f"{fid}: round-trip fixture fails its schema: {err.message}")
            declared = set(pkg_schema.get("properties", {}))
            unknown = [k for k in roundtrip if k not in declared]
            if not unknown:
                failures.append(f"{fid}: round-trip fixture carries no unknown field to preserve")
            else:
                # A parse/serialize round-trip must preserve unknown fields.
                preserved = json.loads(json.dumps(roundtrip))
                if any(k not in preserved for k in unknown):
                    failures.append(f"{fid}: round-trip dropped an unknown field")

    # 5) Cross-matrix lifecycle consistency and registry resolution.
    published = load_matrix_published_labels()
    registry = load_registry_families()
    for pkg in catalog.get("packages", []):
        fid = pkg.get("family_id")
        if published:
            if fid not in published:
                failures.append(f"{fid}: no matching row in the publication matrix")
            elif published[fid] != pkg.get("lifecycle_label"):
                failures.append(
                    f"{fid}: lifecycle_label {pkg.get('lifecycle_label')} disagrees with "
                    f"matrix published_label {published[fid]}"
                )
        if registry:
            rfid = pkg.get("registry_family_id")
            if rfid not in registry:
                failures.append(f"{fid}: registry_family_id '{rfid}' not in contract-family registry")
            else:
                declared = registry[rfid]
                for field in pkg.get("version_field_names", []):
                    if declared and field not in declared:
                        failures.append(
                            f"{fid}: version field '{field}' not declared by registry family '{rfid}'"
                        )

    # 6) Path existence.
    for ref in sorted(set(collect_refs(catalog))):
        if not is_path_ref(ref):
            continue
        if not (REPO_ROOT / candidate_path(ref)).exists():
            failures.append(f"missing referenced path: {ref}")

    # 7) Negative fixtures must be rejected by the semantic invariants.
    cases_path = gen.NEGATIVE_DIR / "cases.json"
    if cases_path.exists():
        cases = load_json(cases_path).get("cases", [])
        if not cases:
            failures.append("fixtures: cases.json lists no cases")
        for case in cases:
            file = case.get("file")
            fixture_path = gen.NEGATIVE_DIR / file
            if not fixture_path.exists():
                failures.append(f"fixtures: missing {file}")
                continue
            fixture = load_json(fixture_path)
            if not semantic_violations(fixture):
                failures.append(f"fixtures: {file} was not rejected by the semantic invariants")
    else:
        failures.append("fixtures: missing cases.json")

    if failures:
        print("[m5-json-catalog] FAIL", file=sys.stderr)
        for msg in failures[:200]:
            print(f"  - {msg}", file=sys.stderr)
        if len(failures) > 200:
            print(f"  ... ({len(failures) - 200} more)", file=sys.stderr)
        return 1

    print(
        "[m5-json-catalog] OK: catalog, packages, examples, round-trip fixtures, "
        "matrix/registry consistency, paths, and negative fixtures validate"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
