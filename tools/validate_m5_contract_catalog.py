#!/usr/bin/env python3
"""Validate the M5 contract catalog, its sample payload galleries, and docs.

Validates:
- ``artifacts/contracts/m5-contract-catalog.json`` against
  ``schemas/public/m5-contracts/m5_contract_catalog.schema.json``
- the catalog's semantic invariants (duplicate family ids, closed-vocabulary
  membership, identity-kind consistency with the contract form, per-family sample
  classes including the required partial/not-provided sample, and the summary
  recomputed from the families), mirroring the typed Rust consumer
- that the checked-in catalog, galleries, Help/SDK/overview/evidence docs,
  capture, and negative fixtures match the regenerator (no hand-edit drift)
- for every family: the gallery exists and matches the regenerator, every sample
  carries field-by-field notes, the gallery includes a partial/not-provided
  sample, and (for a JSON-Schema-backed family) every sample payload validates
  against the published package schema named by ``json_schema_validation_ref``
- that each family's lifecycle label agrees with the publication matrix's
  effective published label and that the family appears in the matrix
- that every referenced repo-relative path exists
- that the checked-in negative fixtures under
  ``fixtures/contracts/m5-contract-catalog/`` are rejected by the semantic
  invariants

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

import regenerate_m5_contract_catalog as gen  # noqa: E402

REPO_ROOT = gen.REPO_ROOT
SCHEMA_PATH = REPO_ROOT / "schemas/public/m5-contracts/m5_contract_catalog.schema.json"
MATRIX_PATH = gen.MATRIX_PATH


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
        "evidence_page",
        "help_catalog_page",
        "sdk_samples_page",
        "publication_matrix_ref",
        "json_schema_catalog_ref",
        "openapi_catalog_ref",
        "wit_publication_ref",
        "contract_family_registry_ref",
        "evidence_index_ref",
        "gallery_home",
    ):
        value = catalog.get(key)
        if isinstance(value, str):
            refs.append(value)
    refs.extend(m for m in catalog.get("offline_bundle", {}).get("bundle_members", []))
    for fam in catalog.get("families", []):
        for key in (
            "compatibility_note_ref",
            "example_gallery_ref",
            "matrix_row_ref",
            "contract_family_ref",
        ):
            value = fam.get(key)
            if isinstance(value, str):
                refs.append(value)
        jref = fam.get("json_schema_validation_ref")
        if isinstance(jref, str):
            refs.append(jref)
        identity = fam.get("contract_identity", {})
        refs.append(identity.get("schema_or_spec_ref", ""))
        refs.append(identity.get("form_catalog_ref", ""))
    return [r for r in refs if isinstance(r, str) and r]


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
        ("identity_kinds", gen.IDENTITY_KINDS),
        ("sample_classes", gen.SAMPLE_CLASSES),
        ("packaging_needs", gen.PACKAGING_NEEDS),
        ("catalog_surfaces", gen.CATALOG_SURFACES),
    ]:
        if catalog.get(field) != expected:
            violations.append(f"closed vocabulary mismatch: {field}")

    families = catalog.get("families", [])
    seen: set[str] = set()
    for fam in families:
        fid = fam.get("family_id", "<unknown>")
        if fid in seen:
            violations.append(f"duplicate family_id: {fid}")
        seen.add(fid)

        if fam.get("lifecycle_label") not in gen.LIFECYCLE_LABELS:
            violations.append(f"{fid}: lifecycle_label not in vocabulary")
        if fam.get("maturity_lane") not in gen.MATURITY_LANES:
            violations.append(f"{fid}: maturity_lane not in vocabulary")
        form = fam.get("contract_form")
        if form not in gen.CONTRACT_FORMS:
            violations.append(f"{fid}: contract_form not in vocabulary")

        identity = fam.get("contract_identity", {})
        ikind = identity.get("identity_kind")
        if ikind not in gen.IDENTITY_KINDS:
            violations.append(f"{fid}: identity_kind not in vocabulary")
        elif form in gen.FORM_TO_IDENTITY and gen.FORM_TO_IDENTITY[form] != ikind:
            violations.append(
                f"{fid}: identity_kind {ikind} disagrees with contract_form {form}"
            )

        # Every family must publish both sample classes, including the
        # partial/not-provided sample, and count them.
        if fam.get("sample_classes") != gen.SAMPLE_CLASSES:
            violations.append(f"{fid}: sample_classes must be the full closed set")
        if fam.get("sample_count") != len(gen.SAMPLE_CLASSES):
            violations.append(f"{fid}: sample_count must equal the sample-class count")

        op = fam.get("offline_parity", {})
        if op.get("requires_runtime_service") is not False:
            violations.append(f"{fid}: offline_parity.requires_runtime_service must be false")
        if op.get("packaging_need") not in gen.PACKAGING_NEEDS:
            violations.append(f"{fid}: packaging_need not in vocabulary")

        gref = fam.get("example_gallery_ref", "")
        if gref != f"{gen.GALLERY_HOME}{fid}.json":
            violations.append(f"{fid}: example_gallery_ref must point at the family gallery")

    if catalog.get("summary") != gen.compute_summary(families):
        violations.append("summary counts disagree with the families")

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


def main() -> int:
    try:
        from jsonschema import Draft202012Validator
    except Exception as exc:  # pragma: no cover
        print(f"[m5-contract-catalog] error: python jsonschema is required: {exc}", file=sys.stderr)
        return 2

    failures: list[str] = []

    if not SCHEMA_PATH.exists():
        print(f"[m5-contract-catalog] error: missing schema {SCHEMA_PATH}", file=sys.stderr)
        return 2
    if not gen.CATALOG_PATH.exists():
        print(f"[m5-contract-catalog] error: missing catalog {gen.CATALOG_PATH}", file=sys.stderr)
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

    # 3) Regenerator drift: the catalog and every generated companion must match
    #    what the regenerator builds from the upstream contract truth.
    built_catalog, built_galleries = gen.build_catalog()
    if catalog != built_catalog:
        failures.append(
            "drift: artifacts/contracts/m5-contract-catalog.json is stale; "
            "run tools/regenerate_m5_contract_catalog.py"
        )
    for name, builder in [
        (gen.HELP_DOC_PATH, gen.build_help_doc),
        (gen.SDK_DOC_PATH, gen.build_sdk_doc),
        (gen.OVERVIEW_DOC_PATH, gen.build_overview_doc),
        (gen.EVIDENCE_DOC_PATH, gen.build_evidence_doc),
    ]:
        want = builder(built_catalog)
        if not want.endswith("\n"):
            want += "\n"
        if not name.exists() or name.read_text(encoding="utf-8") != want:
            failures.append(f"drift: {name.relative_to(REPO_ROOT)} is stale; run the regenerator")
    readme = gen.GALLERY_DIR / "README.md"
    want_readme = gen.build_gallery_readme(built_catalog)
    if not want_readme.endswith("\n"):
        want_readme += "\n"
    if not readme.exists() or readme.read_text(encoding="utf-8") != want_readme:
        failures.append("drift: examples/contracts/m5-gallery/README.md is stale; run the regenerator")

    capture = gen.CAPTURE_PATH
    if not capture.exists() or load_json(capture) != gen.build_capture(built_catalog):
        failures.append(f"drift: {capture.relative_to(REPO_ROOT)} is stale; run the regenerator")

    # 4) Per-family: the gallery exists, matches the regenerator, carries
    #    field-by-field notes and a partial sample, and (for JSON-Schema-backed
    #    families) its sample payloads validate against the package schema.
    for fam in catalog.get("families", []):
        fid = fam.get("family_id", "<unknown>")
        gallery_file = REPO_ROOT / fam.get("example_gallery_ref", "")
        if not gallery_file.exists():
            failures.append(f"{fid}: missing gallery {fam.get('example_gallery_ref')}")
            continue
        gallery = load_json(gallery_file)
        if gallery != built_galleries.get(fid):
            failures.append(f"{fid}: gallery is stale; run the regenerator")

        samples = gallery.get("samples", [])
        classes = [s.get("sample_class") for s in samples]
        if "partial_or_not_provided" not in classes:
            failures.append(f"{fid}: gallery omits a partial/not-provided sample")
        for sample in samples:
            if not sample.get("field_notes"):
                failures.append(f"{fid}: sample {sample.get('sample_id')} has no field notes")
            payload_fields = set(sample.get("payload", {}))
            note_fields = {n.get("field") for n in sample.get("field_notes", [])}
            if payload_fields != note_fields:
                failures.append(
                    f"{fid}: sample {sample.get('sample_id')} field notes do not cover every payload field"
                )

        # The gallery must point back to the catalog entry, not be standalone.
        if gallery.get("catalog_entry_ref", "").split("#", 1)[-1] != fid:
            failures.append(f"{fid}: gallery catalog_entry_ref does not point at the catalog entry")

        jref = fam.get("json_schema_validation_ref")
        if jref:
            schema_file = REPO_ROOT / jref
            if not schema_file.exists():
                failures.append(f"{fid}: missing package schema {jref}")
            else:
                pkg_schema = load_json(schema_file)
                try:
                    Draft202012Validator.check_schema(pkg_schema)
                    pkg_validator = Draft202012Validator(pkg_schema)
                except Exception as exc:
                    failures.append(f"{fid}: package schema is not valid: {exc}")
                    pkg_validator = None
                if pkg_validator is not None:
                    for sample in samples:
                        for err in pkg_validator.iter_errors(sample.get("payload", {})):
                            failures.append(
                                f"{fid}: sample {sample.get('sample_id')} fails its schema: {err.message}"
                            )

    # 5) Cross-matrix lifecycle consistency.
    published = load_matrix_published_labels()
    if published:
        for fam in catalog.get("families", []):
            fid = fam.get("family_id")
            if fid not in published:
                failures.append(f"{fid}: no matching row in the publication matrix")
            elif published[fid] != fam.get("lifecycle_label"):
                failures.append(
                    f"{fid}: lifecycle_label {fam.get('lifecycle_label')} disagrees with "
                    f"matrix published_label {published[fid]}"
                )
        for fid in published:
            if not any(f.get("family_id") == fid for f in catalog.get("families", [])):
                failures.append(f"catalog omits published matrix family: {fid}")

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
        print("[m5-contract-catalog] FAIL", file=sys.stderr)
        for msg in failures[:200]:
            print(f"  - {msg}", file=sys.stderr)
        if len(failures) > 200:
            print(f"  ... ({len(failures) - 200} more)", file=sys.stderr)
        return 1

    print(
        "[m5-contract-catalog] OK: catalog, galleries, field notes, sample-schema "
        "validation, matrix consistency, docs, paths, and negative fixtures validate"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
