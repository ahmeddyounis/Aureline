#!/usr/bin/env python3
"""Validate the M5 CLI/headless structured-output and result-code catalog.

Validates:
- ``artifacts/contracts/m5-cli-output-catalog.json`` against
  ``schemas/public/m5-cli/m5_cli_output_catalog.schema.json``
- the catalog's semantic invariants (duplicate surface ids, closed-vocabulary
  membership, result-code / success-code rules, partial-result coupling, and the
  summary recomputed from the surfaces), mirroring the typed Rust consumer
- that the checked-in catalog, parity fixtures, CLI doc, capture, and negative
  fixtures match the regenerator (no hand-edit drift)
- for every surface: its ``structured_output_schema_ref`` resolves to a checked-in
  JSON Schema package under ``schemas/public/m5-json/`` whose ``$id`` and family id
  agree, its ``result_code`` and ``output_envelope_class`` values are members of
  the closed vocabularies frozen in the CLI output registry schema, its
  ``lifecycle_label`` equals the publication matrix's effective published label,
  and its CLI and UI parity fixtures validate against the resolved package schema
  and carry an identical partial-result / freshness / lifecycle vocabulary
- that every referenced repo-relative path exists
- that the checked-in negative fixtures under ``fixtures/contracts/m5-cli-catalog/``
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

import regenerate_m5_cli_output_catalog as gen  # noqa: E402

REPO_ROOT = gen.REPO_ROOT
SCHEMA_PATH = REPO_ROOT / "schemas/public/m5-cli/m5_cli_output_catalog.schema.json"
MATRIX_PATH = REPO_ROOT / "artifacts/contracts/m5-stability-lifecycle-map.json"
CLI_REGISTRY_SCHEMA_PATH = REPO_ROOT / "schemas/automation/cli_output_registry_entry.schema.json"


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
        "cli_doc_page",
        "json_schema_catalog_ref",
        "publication_matrix_ref",
        "cli_surface_contract_ref",
        "cli_output_registry_schema_ref",
        "evidence_index_ref",
        "schema_home",
    ):
        value = catalog.get(key)
        if isinstance(value, str):
            refs.append(value)
    refs.extend(m for m in catalog.get("offline_bundle", {}).get("bundle_members", []))
    for surface in catalog.get("surfaces", []):
        for key in (
            "structured_output_schema_ref",
            "compatibility_note_ref",
            "cli_parity_fixture_ref",
            "ui_parity_fixture_ref",
            "json_schema_catalog_ref",
            "matrix_row_ref",
        ):
            value = surface.get(key)
            if isinstance(value, str):
                refs.append(value)
        refs.extend(v for v in surface.get("validator_suite_refs", []) if isinstance(v, str))
    return refs


def cli_registry_enum(name: str) -> set[str]:
    """The closed vocabulary frozen for ``name`` in the CLI output registry schema."""
    schema = load_json(CLI_REGISTRY_SCHEMA_PATH)
    return set(schema.get("$defs", {}).get(name, {}).get("enum", []))


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
        ("surface_kinds", gen.SURFACE_KINDS),
        ("lifecycle_labels", gen.LIFECYCLE_LABELS),
        ("machine_output_stability_classes", gen.MACHINE_OUTPUT_STABILITY_CLASSES),
        ("output_envelope_classes", gen.OUTPUT_ENVELOPE_CLASSES),
        ("result_codes", gen.RESULT_CODES),
        ("partial_result_states", gen.PARTIAL_RESULT_STATES),
        ("freshness_states", gen.FRESHNESS_STATES),
        ("parity_match_modes", gen.PARITY_MATCH_MODES),
        ("downgrade_behaviors", gen.DOWNGRADE_BEHAVIORS),
    ]:
        if catalog.get(field) != expected:
            violations.append(f"closed vocabulary mismatch: {field}")

    surfaces = catalog.get("surfaces", [])
    seen: set[str] = set()
    for surface in surfaces:
        sid = surface.get("surface_id", "<unknown>")
        if sid in seen:
            violations.append(f"duplicate surface_id: {sid}")
        seen.add(sid)

        if surface.get("surface_kind") not in gen.SURFACE_KINDS:
            violations.append(f"{sid}: surface_kind not in vocabulary")
        if surface.get("lifecycle_label") not in gen.LIFECYCLE_LABELS:
            violations.append(f"{sid}: lifecycle_label not in vocabulary")
        if surface.get("output_envelope_class") not in gen.OUTPUT_ENVELOPE_CLASSES:
            violations.append(f"{sid}: output_envelope_class not in vocabulary")

        catalog_rows = surface.get("result_code_catalog", [])
        if not catalog_rows:
            violations.append(f"{sid}: empty result_code_catalog")
        codes = [row.get("result_code") for row in catalog_rows]
        for code in codes:
            if code not in gen.RESULT_CODES:
                violations.append(f"{sid}: result_code '{code}' not in vocabulary")
        if "success" not in codes:
            violations.append(f"{sid}: result_code_catalog missing a success row")
        if not any(c not in ("success", "success_no_action_taken") for c in codes):
            violations.append(f"{sid}: result_code_catalog missing an error row")
        for row in catalog_rows:
            code = row.get("result_code")
            if code in ("success", "success_no_action_taken") and row.get("numeric_code") != 0:
                violations.append(f"{sid}: {code} must map to numeric code 0")
            if code == "partial_success_with_warnings" and row.get("partial_result") is not True:
                violations.append(f"{sid}: partial_success_with_warnings must be a partial-result carrier")

        # Partial-result coupling: a surface that can emit a partial state must
        # publish the partial-result carrier code, and vice versa.
        states = surface.get("partial_result_states", [])
        declares_partial = any(s in ("partial", "degraded") for s in states)
        has_carrier = "partial_success_with_warnings" in codes
        if declares_partial and not has_carrier:
            violations.append(f"{sid}: declares a partial/degraded state without a partial-result carrier code")
        if has_carrier and not declares_partial:
            violations.append(f"{sid}: carries partial_success_with_warnings but declares no partial/degraded state")

        if "stale_retest_needed" not in states:
            violations.append(f"{sid}: partial_result_states must include stale_retest_needed")
        for state in states:
            if state not in gen.PARTIAL_RESULT_STATES:
                violations.append(f"{sid}: partial_result_state '{state}' not in vocabulary")

        if not surface.get("cli_parity_fixture_ref") or not surface.get("ui_parity_fixture_ref"):
            violations.append(f"{sid}: missing a parity fixture ref")

    if catalog.get("summary") != gen.compute_summary(surfaces):
        violations.append("summary counts disagree with the surfaces")

    return violations


def load_matrix_published_labels() -> dict[str, str]:
    if not MATRIX_PATH.exists():
        return {}
    matrix = load_json(MATRIX_PATH)
    return {
        row.get("family_id"): row.get("published_label")
        for row in matrix.get("rows", [])
        if isinstance(row, dict)
    }


def parity_fields(payload: dict) -> dict:
    return {
        "partial_result_state": payload.get("partial_result_state"),
        "freshness_state": payload.get("freshness_state"),
        "lifecycle_label": payload.get("lifecycle_label"),
    }


def main() -> int:
    try:
        from jsonschema import Draft202012Validator
    except Exception as exc:  # pragma: no cover
        print(f"[m5-cli-catalog] error: python jsonschema is required: {exc}", file=sys.stderr)
        return 2

    failures: list[str] = []

    if not SCHEMA_PATH.exists():
        print(f"[m5-cli-catalog] error: missing schema {SCHEMA_PATH}", file=sys.stderr)
        return 2
    if not gen.CATALOG_PATH.exists():
        print(f"[m5-cli-catalog] error: missing catalog {gen.CATALOG_PATH}", file=sys.stderr)
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
            "drift: artifacts/contracts/m5-cli-output-catalog.json is stale; "
            "run tools/regenerate_m5_cli_output_catalog.py"
        )

    # 4) Vocabulary reuse: result codes and envelope classes must be members of
    #    the authoritative closed vocabularies frozen in the CLI output registry.
    registry_exit_codes = cli_registry_enum("exit_code_class")
    registry_envelopes = cli_registry_enum("machine_output_envelope_class")
    if registry_exit_codes:
        off = [c for c in catalog.get("result_codes", []) if c not in registry_exit_codes]
        if off:
            failures.append(
                f"vocabulary: result codes not in the CLI registry exit_code_class enum: {off}"
            )
    else:
        failures.append("vocabulary: could not load exit_code_class from the CLI output registry schema")
    if registry_envelopes:
        off = [c for c in catalog.get("output_envelope_classes", []) if c not in registry_envelopes]
        if off:
            failures.append(
                f"vocabulary: output envelopes not in the CLI registry machine_output_envelope_class enum: {off}"
            )

    # 5) Per-surface: schema-ref resolution, matrix lifecycle, and UI/CLI parity.
    published = load_matrix_published_labels()
    json_catalog = load_json(REPO_ROOT / catalog.get("json_schema_catalog_ref", "").split("#", 1)[0]) \
        if (REPO_ROOT / catalog.get("json_schema_catalog_ref", "").split("#", 1)[0]).exists() else None
    json_families = (
        {p["family_id"]: p for p in json_catalog.get("packages", [])} if json_catalog else {}
    )

    for surface in catalog.get("surfaces", []):
        sid = surface.get("surface_id", "<unknown>")
        family = surface.get("family_id")

        # Schema ref must resolve to a checked-in JSON Schema package whose $id and
        # family agree with the surface.
        schema_ref = surface.get("structured_output_schema_ref", "")
        schema_file = REPO_ROOT / schema_ref
        if not schema_file.exists():
            failures.append(f"{sid}: structured_output_schema_ref {schema_ref} does not exist")
            continue
        pkg_schema = load_json(schema_file)
        if pkg_schema.get("$id") != surface.get("structured_output_schema_id"):
            failures.append(f"{sid}: structured_output_schema_id disagrees with the package $id")
        annotation = pkg_schema.get("x-aureline-contract", {})
        if annotation.get("family_id") != family:
            failures.append(f"{sid}: schema package family_id disagrees with the surface family_id")
        if json_families and family not in json_families:
            failures.append(f"{sid}: family '{family}' has no JSON Schema catalog package")

        # Lifecycle label must equal the matrix published label for the family.
        if published:
            if family not in published:
                failures.append(f"{sid}: family '{family}' has no row in the publication matrix")
            elif published[family] != surface.get("lifecycle_label"):
                failures.append(
                    f"{sid}: lifecycle_label {surface.get('lifecycle_label')} disagrees with "
                    f"matrix published_label {published[family]}"
                )

        pkg_validator = Draft202012Validator(pkg_schema)

        cli_file = REPO_ROOT / surface.get("cli_parity_fixture_ref", "")
        ui_file = REPO_ROOT / surface.get("ui_parity_fixture_ref", "")
        if not cli_file.exists():
            failures.append(f"{sid}: missing CLI parity fixture {surface.get('cli_parity_fixture_ref')}")
            continue
        if not ui_file.exists():
            failures.append(f"{sid}: missing UI parity fixture {surface.get('ui_parity_fixture_ref')}")
            continue
        cli_payload = load_json(cli_file)
        ui_payload = load_json(ui_file)

        # Both fixtures match the regenerator (no hand edit).
        gen_surface = next(s for s in gen.SURFACES if s["surface_id"] == sid)
        if cli_payload != gen.build_cli_fixture(gen_surface, surface.get("lifecycle_label")):
            failures.append(f"{sid}: CLI parity fixture is stale; run the regenerator")
        if ui_payload != gen.build_ui_fixture(gen_surface, surface.get("lifecycle_label")):
            failures.append(f"{sid}: UI parity fixture is stale; run the regenerator")

        # Both fixtures validate against the resolved package schema.
        for err in pkg_validator.iter_errors(cli_payload):
            failures.append(f"{sid}: CLI fixture fails its structured-output schema: {err.message}")
        for err in pkg_validator.iter_errors(ui_payload):
            failures.append(f"{sid}: UI fixture fails its structured-output schema: {err.message}")

        # The lifecycle/degraded-state vocabulary must be identical on both surfaces.
        if parity_fields(cli_payload) != parity_fields(ui_payload):
            failures.append(
                f"{sid}: UI/CLI parity vocabulary differs: "
                f"cli={parity_fields(cli_payload)} ui={parity_fields(ui_payload)}"
            )
        # And the shared vocabulary must agree with the surface row's declared
        # lifecycle label and partial-result/freshness vocabularies.
        if cli_payload.get("lifecycle_label") != surface.get("lifecycle_label"):
            failures.append(f"{sid}: fixture lifecycle_label disagrees with the surface row")
        if cli_payload.get("partial_result_state") not in surface.get("partial_result_states", []):
            failures.append(f"{sid}: fixture partial_result_state is not a declared state")
        if cli_payload.get("freshness_state") not in surface.get("freshness_states", []):
            failures.append(f"{sid}: fixture freshness_state is not a declared state")

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
        print("[m5-cli-catalog] FAIL", file=sys.stderr)
        for msg in failures[:200]:
            print(f"  - {msg}", file=sys.stderr)
        if len(failures) > 200:
            print(f"  ... ({len(failures) - 200} more)", file=sys.stderr)
        return 1

    print(
        "[m5-cli-catalog] OK: catalog, surfaces, schema-ref resolution, result-code and "
        "envelope vocabulary reuse, matrix lifecycle, UI/CLI parity, paths, and negative "
        "fixtures validate"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
