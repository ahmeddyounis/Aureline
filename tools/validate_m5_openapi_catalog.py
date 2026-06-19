#!/usr/bin/env python3
"""Validate the M5 OpenAPI publication catalog and its example packs.

Validates:
- ``artifacts/contracts/m5-openapi-catalog.json`` against
  ``schemas/public/m5-contracts/m5_openapi_catalog.schema.json``
- the catalog's semantic invariants (duplicate endpoint ids, closed-vocabulary
  membership, per-endpoint lifecycle equals the family label, read-only/preview
  coupling, and the summary recomputed from the endpoints), mirroring the typed
  Rust consumer
- that the checked-in catalog, example packs, SDK doc, overview doc, capture, and
  negative fixtures match the regenerator (no hand-edit drift)
- that ``family_lifecycle_label`` equals the publication matrix's effective
  published label for the ``service_optional_api`` family
- for every endpoint (when PyYAML is available): its operation id, method, path,
  and ``x-aureline-api-surface-id`` are present in the OpenAPI document; its
  auth-source, entitlement, policy-override, offline, deprecation, and sunset
  postures match the matching surface row in
  ``artifacts/service/api_surface_rows.yaml``; and its example request/response
  pack validates against the OpenAPI document's component schemas
- that no example pack carries a literal live URL or credential token
- that every referenced repo-relative path exists
- that the checked-in negative fixtures under ``fixtures/contracts/m5-openapi/``
  are rejected by the semantic invariants

The validator imports the builders from the regenerator so the two cannot drift.
The YAML-backed cross-checks are skipped (not failed) when PyYAML is unavailable,
matching the other contract validators; the regenerator drift check and the
typed Rust consumer still bind the catalog to the source set.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any

TOOLS_DIR = Path(__file__).resolve().parent
if str(TOOLS_DIR) not in sys.path:
    sys.path.insert(0, str(TOOLS_DIR))

import regenerate_m5_openapi_catalog as gen  # noqa: E402

REPO_ROOT = gen.REPO_ROOT
SCHEMA_PATH = REPO_ROOT / "schemas/public/m5-contracts/m5_openapi_catalog.schema.json"
MATRIX_PATH = REPO_ROOT / "artifacts/contracts/m5-stability-lifecycle-map.json"
OPENAPI_PATH = REPO_ROOT / "openapi/service_api_seed.yaml"
SURFACE_ROWS_PATH = REPO_ROOT / "artifacts/service/api_surface_rows.yaml"

# Substrings that must never appear in an example pack: literal live URLs or
# credential tokens. The "urn:" scheme used by SCIM schema identifiers is allowed.
FORBIDDEN_EXAMPLE_SUBSTRINGS = ["://", "bearer ", "password", "secret", "api_key", "apikey"]


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
        "sdk_doc_page",
        "openapi_readme_ref",
        "json_schema_catalog_ref",
        "publication_matrix_ref",
        "api_surface_rows_ref",
        "slo_rows_ref",
        "evidence_index_ref",
        "primary_openapi_document_ref",
        "example_pack_home",
    ):
        value = catalog.get(key)
        if isinstance(value, str):
            refs.append(value)
    refs.extend(catalog.get("offline_bundle", {}).get("bundle_members", []))
    for ep in catalog.get("endpoints", []):
        for key in (
            "openapi_document_ref",
            "example_pack_ref",
            "compatibility_note_ref",
            "matrix_row_ref",
            "surface_row_ref",
        ):
            value = ep.get(key)
            if isinstance(value, str):
                refs.append(value)
        refs.extend(v for v in ep.get("validator_suite_refs", []) if isinstance(v, str))
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
    if catalog.get("family_id") != gen.FAMILY_ID:
        violations.append("family_id mismatch")

    for field, expected in [
        ("http_methods", gen.HTTP_METHODS),
        ("auth_source_classes", gen.AUTH_SOURCE_CLASSES),
        ("entitlement_classes", gen.ENTITLEMENT_CLASSES),
        ("policy_override_postures", gen.POLICY_OVERRIDE_POSTURES),
        ("mutability_postures", gen.MUTABILITY_POSTURES),
        ("preview_support_classes", gen.PREVIEW_SUPPORT_CLASSES),
        ("offline_behavior_classes", gen.OFFLINE_BEHAVIOR_CLASSES),
        ("deprecation_lane_classes", gen.DEPRECATION_LANE_CLASSES),
        ("sunset_postures", gen.SUNSET_POSTURES),
        ("lifecycle_labels", gen.LIFECYCLE_LABELS),
        ("maturity_lanes", gen.MATURITY_LANES),
        ("downgrade_behaviors", gen.DOWNGRADE_BEHAVIORS),
    ]:
        if catalog.get(field) != expected:
            violations.append(f"closed vocabulary mismatch: {field}")

    family_label = catalog.get("family_lifecycle_label")
    if family_label not in gen.LIFECYCLE_LABELS:
        violations.append("family_lifecycle_label not in vocabulary")

    endpoints = catalog.get("endpoints", [])
    seen: set[str] = set()
    for ep in endpoints:
        eid = ep.get("endpoint_id", "<unknown>")
        if eid in seen:
            violations.append(f"duplicate endpoint_id: {eid}")
        seen.add(eid)

        checks = [
            ("http_method", gen.HTTP_METHODS),
            ("auth_source_class", gen.AUTH_SOURCE_CLASSES),
            ("entitlement_class", gen.ENTITLEMENT_CLASSES),
            ("policy_override_posture", gen.POLICY_OVERRIDE_POSTURES),
            ("mutability_posture", gen.MUTABILITY_POSTURES),
            ("preview_support_class", gen.PREVIEW_SUPPORT_CLASSES),
            ("offline_behavior_class", gen.OFFLINE_BEHAVIOR_CLASSES),
            ("deprecation_lane_class", gen.DEPRECATION_LANE_CLASSES),
            ("sunset_posture", gen.SUNSET_POSTURES),
            ("maturity_lane", gen.MATURITY_LANES),
        ]
        for field, vocab in checks:
            if ep.get(field) not in vocab:
                violations.append(f"{eid}: {field} '{ep.get(field)}' not in vocabulary")

        if ep.get("lifecycle_label") != family_label:
            violations.append(
                f"{eid}: lifecycle_label {ep.get('lifecycle_label')} disagrees with the "
                f"family label {family_label}"
            )

        mut = ep.get("mutability_posture")
        if mut == "read_only":
            if ep.get("request_schema_ref") is not None:
                violations.append(f"{eid}: read_only endpoint carries a request body")
            if ep.get("preview_support_class") != "read_only_no_mutation":
                violations.append(
                    f"{eid}: read_only endpoint must use the read_only_no_mutation preview class"
                )
        else:
            if ep.get("preview_support_class") == "read_only_no_mutation":
                violations.append(
                    f"{eid}: mutating endpoint must not use the read_only_no_mutation preview class"
                )

        if not ep.get("response_schema_ref"):
            violations.append(f"{eid}: missing response_schema_ref")
        if not ep.get("example_pack_ref"):
            violations.append(f"{eid}: missing example_pack_ref")
        if not ep.get("validator_suite_refs"):
            violations.append(f"{eid}: missing validator_suite_refs")

    if catalog.get("summary") != gen.compute_summary(endpoints):
        violations.append("summary counts disagree with the endpoints")

    return violations


def load_matrix_published_label() -> str | None:
    if not MATRIX_PATH.exists():
        return None
    matrix = load_json(MATRIX_PATH)
    for row in matrix.get("rows", []):
        if isinstance(row, dict) and row.get("family_id") == gen.FAMILY_ID:
            return row.get("published_label")
    return None


def try_load_yaml(path: Path) -> Any | None:
    try:
        import yaml  # type: ignore
    except Exception:
        return None
    return yaml.safe_load(path.read_text(encoding="utf-8"))


def component_schema_validator(seed: dict, name: str):
    from jsonschema import Draft202012Validator

    target = seed.get("components", {}).get("schemas", {}).get(name)
    if target is None:
        return None
    # Carry the full components block alongside the target schema so nested
    # "#/components/schemas/..." refs resolve against this same document.
    schema_doc = dict(target)
    schema_doc["components"] = seed["components"]
    return Draft202012Validator(schema_doc)


def main() -> int:
    try:
        from jsonschema import Draft202012Validator
    except Exception as exc:  # pragma: no cover
        print(f"[m5-openapi] error: python jsonschema is required: {exc}", file=sys.stderr)
        return 2

    failures: list[str] = []

    if not SCHEMA_PATH.exists():
        print(f"[m5-openapi] error: missing schema {SCHEMA_PATH}", file=sys.stderr)
        return 2
    if not gen.CATALOG_PATH.exists():
        print(f"[m5-openapi] error: missing catalog {gen.CATALOG_PATH}", file=sys.stderr)
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

    # 3) Regenerator drift: catalog, example packs, docs, capture, and fixtures.
    if catalog != gen.build_catalog():
        failures.append(
            "drift: artifacts/contracts/m5-openapi-catalog.json is stale; "
            "run tools/regenerate_m5_openapi_catalog.py"
        )
    for ep in gen.ENDPOINTS:
        pack_path = gen.EXAMPLE_DIR / f"{ep['operation_id']}.json"
        if not pack_path.exists():
            failures.append(f"drift: missing example pack {pack_path.relative_to(REPO_ROOT)}")
            continue
        if load_json(pack_path) != gen.build_example_pack(ep):
            failures.append(
                f"drift: example pack {pack_path.relative_to(REPO_ROOT)} is stale; "
                "run tools/regenerate_m5_openapi_catalog.py"
            )
    for path, builder in [
        (gen.SDK_DOC_PATH, gen.build_sdk_doc),
        (gen.OVERVIEW_DOC_PATH, gen.build_overview_doc),
    ]:
        expected = builder(catalog)
        if not expected.endswith("\n"):
            expected += "\n"
        actual = path.read_text(encoding="utf-8") if path.exists() else ""
        if expected != actual:
            failures.append(
                f"drift: {path.relative_to(REPO_ROOT)} is stale; "
                "run tools/regenerate_m5_openapi_catalog.py"
            )
    if gen.CAPTURE_PATH.exists():
        if load_json(gen.CAPTURE_PATH) != gen.build_capture(catalog):
            failures.append("drift: validation capture is stale; run the regenerator")
    else:
        failures.append("drift: missing validation capture")

    # 4) family_lifecycle_label equals the matrix published label.
    published = load_matrix_published_label()
    if published is None:
        failures.append(f"family {gen.FAMILY_ID} has no row in the publication matrix")
    elif published != catalog.get("family_lifecycle_label"):
        failures.append(
            f"family_lifecycle_label {catalog.get('family_lifecycle_label')} disagrees with "
            f"matrix published_label {published}"
        )

    # 5) Example-pack content guard: no literal live URL or credential token.
    for ep in catalog.get("endpoints", []):
        pack_path = gen.EXAMPLE_DIR / f"{ep['operation_id']}.json"
        if not pack_path.exists():
            continue
        body = json.dumps(load_json(pack_path).get("request")) + json.dumps(
            load_json(pack_path).get("response")
        )
        lowered = body.lower()
        for needle in FORBIDDEN_EXAMPLE_SUBSTRINGS:
            if needle in lowered:
                failures.append(
                    f"{ep['operation_id']}: example pack carries a forbidden substring '{needle}'"
                )

    # 6) OpenAPI-document and surface-row cross-checks (require PyYAML).
    seed = try_load_yaml(OPENAPI_PATH)
    rows_doc = try_load_yaml(SURFACE_ROWS_PATH)
    if seed is None or rows_doc is None:
        print(
            "[m5-openapi] note: PyYAML unavailable; skipping OpenAPI-document and "
            "surface-row cross-checks (regenerator drift still binds the catalog).",
            file=sys.stderr,
        )
    else:
        # Index the OpenAPI document operations.
        seed_ops: dict[str, dict] = {}
        for path, item in (seed.get("paths") or {}).items():
            for method, op in item.items():
                if not isinstance(op, dict) or "operationId" not in op:
                    continue
                seed_ops[op["operationId"]] = {
                    "method": method.lower(),
                    "path": path,
                    "surface_id": op.get("x-aureline-api-surface-id"),
                }
        rows_by_id = {
            r.get("api_surface_id"): r
            for r in rows_doc.get("rows", [])
            if isinstance(r, dict)
        }
        for ep in catalog.get("endpoints", []):
            eid = ep["operation_id"]
            seed_op = seed_ops.get(eid)
            if seed_op is None:
                failures.append(f"{eid}: operation not present in the OpenAPI document")
            else:
                if seed_op["method"] != ep["http_method"]:
                    failures.append(f"{eid}: http_method disagrees with the OpenAPI document")
                if seed_op["path"] != ep["path"]:
                    failures.append(f"{eid}: path disagrees with the OpenAPI document")
                if seed_op["surface_id"] != ep["api_surface_id"]:
                    failures.append(
                        f"{eid}: api_surface_id disagrees with the OpenAPI document "
                        f"x-aureline-api-surface-id"
                    )

            row = rows_by_id.get(ep["api_surface_id"])
            if row is None:
                failures.append(f"{eid}: api_surface_id has no row in api_surface_rows.yaml")
            else:
                auth = row.get("auth", {})
                offline = row.get("offline_and_cache_posture", {})
                deprecation = row.get("deprecation_lane", {})
                expected = {
                    "auth_source_class": auth.get("auth_mode"),
                    "entitlement_class": auth.get("entitlement_class"),
                    "policy_override_posture": auth.get("policy_override_posture"),
                    "offline_behavior_class": offline.get("offline_behavior_class"),
                    "deprecation_lane_class": deprecation.get("lane_class"),
                    "sunset_posture": deprecation.get("sunset_posture"),
                    "api_family_class": row.get("api_family_class"),
                    "maturity_lane": row.get("maturity_lane"),
                }
                for field, want in expected.items():
                    if ep.get(field) != want:
                        failures.append(
                            f"{eid}: {field} {ep.get(field)!r} disagrees with surface row {want!r}"
                        )

        # Each example request/response validates against the document schema.
        for ep in catalog.get("endpoints", []):
            pack_path = gen.EXAMPLE_DIR / f"{ep['operation_id']}.json"
            if not pack_path.exists():
                continue
            pack = load_json(pack_path)
            for kind, schema_field in (("request", "request_schema_ref"), ("response", "response_schema_ref")):
                ref = ep.get(schema_field)
                payload = pack.get(kind)
                if ref is None:
                    if payload is not None:
                        failures.append(f"{ep['operation_id']}: {kind} present without a schema ref")
                    continue
                name = ref.split("/")[-1]
                comp_validator = component_schema_validator(seed, name)
                if comp_validator is None:
                    failures.append(f"{ep['operation_id']}: component schema {name} not in the OpenAPI document")
                    continue
                for err in comp_validator.iter_errors(payload):
                    failures.append(
                        f"{ep['operation_id']}: {kind} example fails {name}: {err.message}"
                    )

    # 7) Path existence.
    for ref in sorted(set(collect_refs(catalog))):
        if not is_path_ref(ref):
            continue
        if not (REPO_ROOT / candidate_path(ref)).exists():
            failures.append(f"missing referenced path: {ref}")

    # 8) Negative fixtures must be rejected by the semantic invariants.
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
        print("[m5-openapi] FAIL", file=sys.stderr)
        for msg in failures[:200]:
            print(f"  - {msg}", file=sys.stderr)
        if len(failures) > 200:
            print(f"  ... ({len(failures) - 200} more)", file=sys.stderr)
        return 1

    print(
        "[m5-openapi] OK: catalog, endpoints, OpenAPI-document and surface-row parity, "
        "example-pack schema conformance, matrix lifecycle, paths, and negative fixtures "
        "validate"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
