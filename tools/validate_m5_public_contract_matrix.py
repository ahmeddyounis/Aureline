#!/usr/bin/env python3
"""Validate the M5 public-contract publication matrix and its projections.

Validates:
- artifacts/contracts/m5-stability-lifecycle-map.json against
  schemas/public/m5-contracts/m5_public_contract_matrix.schema.json
- examples/contracts/m5/contract_row_example.json against the schema row $def
- the matrix's semantic invariants (derived gap reasons, narrowing, promotion,
  and summary recomputed from the publication requirements), mirroring the typed
  Rust consumer
- that the checked-in CSV and Markdown projections match the canonical JSON
  (no hand-edit drift)
- that every referenced repo-relative path exists and every contract_family_ref
  anchor resolves to a contract-family registry row
- that the checked-in negative fixtures under fixtures/contracts/m5/ are rejected
  by the semantic invariants

The validator imports the projection and compute helpers from the regenerator so
the two cannot drift.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any

TOOLS_DIR = Path(__file__).resolve().parent
if str(TOOLS_DIR) not in sys.path:
    sys.path.insert(0, str(TOOLS_DIR))

import regenerate_m5_public_contract_matrix as gen  # noqa: E402

REPO_ROOT = gen.REPO_ROOT
SCHEMA_PATH = REPO_ROOT / "schemas/public/m5-contracts/m5_public_contract_matrix.schema.json"
ROW_EXAMPLE_PATH = REPO_ROOT / "examples/contracts/m5/contract_row_example.json"
CONTRACT_FAMILIES_PATH = REPO_ROOT / "artifacts/contracts/contract_families.yaml"

# release_packet_ref / compatibility anchors that are ids, not file paths.
NON_PATH_PREFIXES = ("manifest_entry:", "compat_row:", "claim:")


def load_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def is_path_ref(value: str) -> bool:
    if not value:
        return False
    if "://" in value:
        return False
    if value.startswith(NON_PATH_PREFIXES):
        return False
    return "/" in value or value.endswith(
        (".json", ".yaml", ".yml", ".md", ".py", ".sh", ".toml", ".wit")
    )


def candidate_path(ref: str) -> str:
    return ref.split("#", 1)[0] if "#" in ref else ref


def collect_refs(matrix: dict) -> list[str]:
    refs: list[str] = []
    for key in (
        "overview_page",
        "claim_manifest_ref",
        "contract_family_registry_ref",
        "compatibility_surface_inventory_ref",
        "qualification_matrix_ref",
        "evidence_index_ref",
    ):
        value = matrix.get(key)
        if isinstance(value, str):
            refs.append(value)
    for row in matrix.get("rows", []):
        for key in ("contract_family_ref", "compatibility_surface_ref"):
            value = row.get(key)
            if isinstance(value, str):
                refs.append(value)
        for key in ("example_corpus_refs", "validator_suite_refs"):
            refs.extend(v for v in row.get(key, []) if isinstance(v, str))
        for cell in row.get("publication_requirements", []):
            refs.extend(v for v in cell.get("refs", []) if isinstance(v, str))
    return refs


def load_registry_family_ids() -> set[str]:
    try:
        import yaml
    except Exception:
        return set()
    payload = yaml.safe_load(CONTRACT_FAMILIES_PATH.read_text(encoding="utf-8"))
    ids: set[str] = set()
    for row in payload.get("rows", []):
        if isinstance(row, dict):
            if isinstance(row.get("family_id"), str):
                ids.add(row["family_id"])
            for alias in row.get("alias_ids", []) or []:
                if isinstance(alias, str):
                    ids.add(alias)
    return ids


def semantic_violations(matrix: dict) -> list[str]:
    """Recompute the matrix's derived state and report disagreements.

    Mirrors the typed Rust consumer's `validate()`. The canonical matrix must
    return no violations; each negative fixture must return at least one.
    """
    violations: list[str] = []

    # Closed-vocabulary equality with the canonical lexicons.
    for field, expected in [
        ("contract_forms", gen.CONTRACT_FORMS),
        ("contract_categories", gen.CONTRACT_CATEGORIES),
        ("maturity_lanes", gen.MATURITY_LANES),
        ("lifecycle_labels", gen.LIFECYCLE_LABELS),
        ("reader_writer_postures", gen.READER_WRITER_POSTURES),
        ("packaging_needs", gen.PACKAGING_NEEDS),
        ("publication_artifact_kinds", gen.PUBLICATION_ARTIFACT_KINDS),
        ("publication_states", gen.PUBLICATION_STATES),
        ("gap_reasons", gen.GAP_REASONS),
        ("remediation_actions", gen.REMEDIATION_ACTIONS),
    ]:
        if matrix.get(field) != expected:
            violations.append(f"closed vocabulary mismatch: {field}")

    if matrix.get("record_kind") != gen.RECORD_KIND:
        violations.append("record_kind mismatch")
    if matrix.get("schema_version") != gen.SCHEMA_VERSION:
        violations.append("schema_version mismatch")

    if matrix.get("stop_rules") != gen.build_stop_rules():
        violations.append("stop_rules disagree with the canonical stop-rule set")

    rows = matrix.get("rows", [])
    seen: set[str] = set()
    for row in rows:
        fid = row.get("family_id", "<unknown>")
        if fid in seen:
            violations.append(f"duplicate family_id: {fid}")
        seen.add(fid)

        requirements = row.get("publication_requirements", [])
        for cell in requirements:
            if cell.get("required") and cell.get("state") == "not_applicable":
                violations.append(
                    f"{fid}: required {cell.get('artifact_kind')} marked not_applicable"
                )

        gaps = gen.compute_gaps(requirements, row.get("release_packet_ref", ""))
        if gaps != row.get("active_gap_reasons"):
            violations.append(
                f"{fid}: active_gap_reasons {row.get('active_gap_reasons')} "
                f"disagree with derived {gaps}"
            )

        if gaps:
            expected_state = "narrowed"
            expected_label = gen.narrow_floor(row.get("claim_label", "withdrawn"))
        else:
            expected_state = "published"
            expected_label = row.get("claim_label")
        if row.get("row_state") != expected_state:
            violations.append(
                f"{fid}: row_state {row.get('row_state')} disagrees with derived {expected_state}"
            )
        if row.get("published_label") != expected_label:
            violations.append(
                f"{fid}: published_label {row.get('published_label')} disagrees with derived {expected_label}"
            )

        claim = row.get("claim_label", "withdrawn")
        published = row.get("published_label", "withdrawn")
        if gen.RANK.get(published, 0) > gen.RANK.get(claim, 0):
            violations.append(
                f"{fid}: published_label {published} is wider than claim_label {claim}"
            )

        if not str(row.get("release_packet_ref", "")).strip():
            violations.append(f"{fid}: empty release_packet_ref")

    declared = set(matrix.get("release_blocking_family_refs", []))
    covered = {r.get("family_id") for r in rows if r.get("release_blocking")}
    if declared != covered:
        violations.append("release_blocking_family_refs disagree with release-blocking rows")

    if matrix.get("promotion") != gen.compute_promotion(rows, matrix.get("stop_rules", [])):
        violations.append("promotion verdict disagrees with the firing stop rules")
    if matrix.get("summary") != gen.compute_summary(rows, matrix.get("stop_rules", [])):
        violations.append("summary counts disagree with the rows")

    return violations


def main() -> int:
    try:
        from jsonschema import Draft202012Validator
    except Exception as exc:  # pragma: no cover
        print(f"[m5-public-contract] error: python jsonschema is required: {exc}", file=sys.stderr)
        return 2

    failures: list[str] = []

    if not SCHEMA_PATH.exists():
        print(f"[m5-public-contract] error: missing schema {SCHEMA_PATH}", file=sys.stderr)
        return 2
    if not gen.MATRIX_PATH.exists():
        print(f"[m5-public-contract] error: missing matrix {gen.MATRIX_PATH}", file=sys.stderr)
        return 2

    schema = load_json(SCHEMA_PATH)
    validator = Draft202012Validator(schema)
    matrix = load_json(gen.MATRIX_PATH)

    # 1) Schema validation of the canonical matrix.
    schema_errors = sorted(validator.iter_errors(matrix), key=lambda e: list(e.path))
    for err in schema_errors:
        loc = "/".join(str(p) for p in err.path) or "<root>"
        failures.append(f"schema: {loc}: {err.message}")

    # 2) Schema validation of the standalone row example against the row $def.
    if ROW_EXAMPLE_PATH.exists():
        row_schema = {
            "$schema": schema["$schema"],
            "$defs": schema["$defs"],
            "$ref": "#/$defs/row",
        }
        row_validator = Draft202012Validator(row_schema)
        example = load_json(ROW_EXAMPLE_PATH)
        for err in sorted(row_validator.iter_errors(example), key=lambda e: list(e.path)):
            loc = "/".join(str(p) for p in err.path) or "<root>"
            failures.append(f"row-example: {loc}: {err.message}")
    else:
        failures.append(f"missing row example: {ROW_EXAMPLE_PATH.relative_to(REPO_ROOT)}")

    # 3) Semantic invariants on the canonical matrix.
    for msg in semantic_violations(matrix):
        failures.append(f"semantic: {msg}")

    # 4) Projection drift: CSV + Markdown must match the canonical JSON.
    expected_csv = gen.build_csv(matrix)
    actual_csv = gen.CSV_PATH.read_text(encoding="utf-8") if gen.CSV_PATH.exists() else ""
    if expected_csv != actual_csv:
        failures.append(
            f"drift: {gen.CSV_PATH.relative_to(REPO_ROOT)} is stale; "
            "run tools/regenerate_m5_public_contract_matrix.py"
        )
    expected_md = gen.build_markdown(matrix)
    if not expected_md.endswith("\n"):
        expected_md += "\n"
    actual_md = gen.MD_PATH.read_text(encoding="utf-8") if gen.MD_PATH.exists() else ""
    if expected_md != actual_md:
        failures.append(
            f"drift: {gen.MD_PATH.relative_to(REPO_ROOT)} is stale; "
            "run tools/regenerate_m5_public_contract_matrix.py"
        )

    # 5) Path existence.
    for ref in sorted(set(collect_refs(matrix))):
        if not is_path_ref(ref):
            continue
        if not (REPO_ROOT / candidate_path(ref)).exists():
            failures.append(f"missing referenced path: {ref}")

    # 6) contract_family_ref anchors resolve to registry rows.
    registry_ids = load_registry_family_ids()
    if registry_ids:
        for row in matrix.get("rows", []):
            ref = row.get("contract_family_ref", "")
            if "#" in ref:
                anchor = ref.split("#", 1)[1]
                if anchor not in registry_ids:
                    failures.append(
                        f"{row.get('family_id')}: contract_family_ref anchor "
                        f"'{anchor}' not found in contract-family registry"
                    )

    # 7) Negative fixtures must be rejected by the semantic invariants.
    cases_path = gen.FIXTURES_DIR / "cases.json"
    if cases_path.exists():
        cases = load_json(cases_path).get("cases", [])
        if not cases:
            failures.append("fixtures: cases.json lists no cases")
        for case in cases:
            file = case.get("file")
            fixture_path = gen.FIXTURES_DIR / file
            if not fixture_path.exists():
                failures.append(f"fixtures: missing {file}")
                continue
            fixture = load_json(fixture_path)
            if not semantic_violations(fixture):
                failures.append(f"fixtures: {file} was not rejected by the semantic invariants")
    else:
        failures.append("fixtures: missing cases.json")

    if failures:
        print("[m5-public-contract] FAIL", file=sys.stderr)
        for msg in failures[:200]:
            print(f"  - {msg}", file=sys.stderr)
        if len(failures) > 200:
            print(f"  ... ({len(failures) - 200} more)", file=sys.stderr)
        return 1

    print(
        "[m5-public-contract] OK: matrix, row example, semantic invariants, "
        "projections, paths, registry anchors, and negative fixtures validate"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
