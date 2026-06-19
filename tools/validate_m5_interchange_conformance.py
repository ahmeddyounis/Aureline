#!/usr/bin/env python3
"""Validate the M5 interchange-conformance register, its validator descriptors, the
emitted-artifact corpus, the conformance report, and the negative fixtures.

Validates:
- ``artifacts/contracts/m5-interchange-conformance.json`` against
  ``schemas/public/m5-contracts/m5_interchange_conformance.schema.json``
- the register's semantic invariants (duplicate family ids, closed-vocabulary membership,
  one dimension per dimension kind, the conformance state recomputed from the dimensions,
  the decision recomputed from the conformance state, the top-level blocker decision
  recomputed from the rows, the summary recomputed from the rows, the consumer-agreement
  block consistent with the consumer surfaces, and active reason codes drawn from the
  closed reason-code vocabulary), mirroring the typed Rust consumer
- that the register, the validator manifest, the per-family validator descriptors, the
  validators README, the conformance report, the Help/overview/evidence docs, the capture,
  the emitted artifacts, and the negative fixtures match the regenerator (no hand-edit drift)
- that each catalog-linked family's lifecycle label equals the contract catalog's published
  label for that family
- that every referenced repo-relative structural path exists
- that each real emitted artifact carries its contract version, lifecycle label, and
  required provenance, names no credential-ish keys, and that its per-surface renderings
  agree with the row's consumer-agreement block
- that the checked-in negative fixtures under
  ``fixtures/contracts/m5-interchange/negative/`` are rejected by the semantic invariants

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

import regenerate_m5_interchange_conformance as gen  # noqa: E402

REPO_ROOT = gen.REPO_ROOT
SCHEMA_PATH = REPO_ROOT / gen.SCHEMA_REF
CATALOG_PATH = gen.CATALOG_PATH

# Keys an emitted artifact must never carry (no credential bodies or raw provider payloads).
_FORBIDDEN_KEY_SUBSTRINGS = (
    "token",
    "secret",
    "password",
    "passwd",
    "api_key",
    "apikey",
    "authorization",
    "auth_header",
    "cookie",
    "private_key",
    "credential",
    "bearer",
)


def load_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def is_path_ref(value: str) -> bool:
    if not value:
        return False
    if "://" in value:
        return False
    if ":" in value:
        return False
    return "/" in value or value.endswith(
        (".json", ".yaml", ".yml", ".md", ".py", ".sh", ".toml", ".wit")
    )


def candidate_path(ref: str) -> str:
    return ref.split("#", 1)[0] if "#" in ref else ref


def collect_structural_refs(register: dict) -> list[str]:
    """Collect the repo-relative structural paths this register authors."""
    refs: list[str] = []
    for key in (
        "overview_page",
        "evidence_page",
        "help_page",
        "conformance_report_ref",
        "validator_manifest_ref",
        "validators_home",
        "contract_catalog_ref",
        "publication_matrix_ref",
        "reader_writer_compat_ref",
        "evidence_index_ref",
        "build_identity_ref",
    ):
        value = register.get(key)
        if isinstance(value, str):
            refs.append(value)
    for row in register.get("rows", []):
        refs.append(row.get("validator", {}).get("descriptor_ref", ""))
        refs.append(row.get("runner", {}).get("artifact_ref", ""))
        if row.get("catalog_family_id"):
            refs.append(row.get("catalog_entry_ref", ""))
            refs.append(row.get("matrix_row_ref", ""))
    return [r for r in refs if isinstance(r, str) and r]


def recompute_conformance_state(row: dict) -> str:
    dims = row.get("dimensions", [])
    any_required_fail = any(d.get("outcome") == "fail" and d.get("required") for d in dims)
    any_required_downgrade = any(
        d.get("outcome") == "downgrade" and d.get("required") for d in dims
    )
    if row.get("release_blocking") and any_required_fail:
        return "failed"
    if row.get("narrowed") or any_required_downgrade or any_required_fail:
        return "narrowed"
    return "conformant"


def semantic_violations(register: dict) -> list[str]:
    """Recompute the register's derived state and report disagreements.

    Mirrors the typed Rust consumer's `validate()`. The canonical register must return no
    violations; each negative fixture must return at least one.
    """
    violations: list[str] = []

    if register.get("record_kind") != gen.RECORD_KIND:
        violations.append("record_kind mismatch")
    if register.get("schema_version") != gen.SCHEMA_VERSION:
        violations.append("schema_version mismatch")
    if register.get("register_id") != gen.REGISTER_ID:
        violations.append("register_id mismatch")

    for field, expected in [
        ("lifecycle_labels", gen.LIFECYCLE_LABELS),
        ("interchange_directions", gen.INTERCHANGE_DIRECTIONS),
        ("conformance_classes", gen.CONFORMANCE_CLASSES),
        ("consumer_surfaces", gen.CONSUMER_SURFACES),
        ("dimension_kinds", gen.DIMENSION_KINDS),
        ("dimension_outcomes", gen.DIMENSION_OUTCOMES),
        ("conformance_states", gen.CONFORMANCE_STATES),
        ("decision_states", gen.DECISION_STATES),
        ("degraded_states", gen.DEGRADED_STATES),
        ("reason_codes", gen.REASON_CODES),
    ]:
        if register.get(field) != expected:
            violations.append(f"closed vocabulary mismatch: {field}")

    rows = register.get("rows", [])
    seen: set[str] = set()
    for row in rows:
        fid = row.get("family_id", "<unknown>")
        if fid in seen:
            violations.append(f"rows.duplicate_family_id: {fid}")
        seen.add(fid)

        if row.get("conformance_state") not in gen.CONFORMANCE_STATES:
            violations.append(f"rows.unknown_conformance_state: {fid}")

        # One dimension per dimension kind, in the canonical order.
        dim_kinds = [d.get("dimension_kind") for d in row.get("dimensions", [])]
        if dim_kinds != gen.DIMENSION_KINDS:
            violations.append(
                f"rows.dimension_coverage: {fid} dimensions must be exactly the dimension-kind set"
            )

        expected_state = recompute_conformance_state(row)
        if row.get("conformance_state") != expected_state:
            violations.append(
                f"rows.conformance_state: {fid} conformance_state "
                f"{row.get('conformance_state')} disagrees with the dimensions ({expected_state})"
            )

        expected_decision = "hold" if expected_state == "failed" else "clear"
        if row.get("decision") != expected_decision:
            violations.append(
                f"rows.decision: {fid} decision {row.get('decision')} disagrees with the "
                f"conformance state ({expected_decision})"
            )

        # Active reason codes must be drawn from the closed vocabulary.
        for code in row.get("active_reason_codes", []):
            if code not in gen.REASON_CODES:
                violations.append(f"rows.reason_code_vocabulary: {fid} unknown reason code {code}")
        for code in row.get("validator", {}).get("reason_codes_emitted", []):
            if code not in gen.REASON_CODES:
                violations.append(
                    f"rows.reason_code_vocabulary: {fid} validator emits unknown reason code {code}"
                )

        # Consumer-agreement block must cover every consumer surface and the agreed values
        # must match the row.
        agreement = row.get("consumer_agreement", {})
        if agreement.get("surfaces") != gen.CONSUMER_SURFACES:
            violations.append(f"rows.consumer_agreement: {fid} surfaces are not the canonical set")
        if agreement.get("agreed_contract_version") != row.get("contract_version"):
            violations.append(
                f"rows.consumer_agreement: {fid} agreed contract version disagrees with the row"
            )
        if agreement.get("agreed_lifecycle_label") != row.get("lifecycle_label"):
            violations.append(
                f"rows.consumer_agreement: {fid} agreed lifecycle label disagrees with the row"
            )

    # Top-level blocker decision recomputed from the rows.
    blocked_ids = [r.get("family_id") for r in rows if recompute_conformance_state(r) == "failed"]
    blockers = register.get("blockers", {})
    if blockers.get("blocking_family_ids") != blocked_ids:
        violations.append("blockers.block: blocking_family_ids disagree with the failed rows")
    expected_top = "hold" if blocked_ids else "clear"
    if blockers.get("decision") != expected_top:
        violations.append("blockers.decision: top-level decision disagrees with the failed rows")

    if register.get("summary") != gen.compute_summary(rows):
        violations.append("summary.count_mismatch: summary counts disagree with the rows")

    return violations


def load_catalog_labels() -> dict[str, str]:
    if not CATALOG_PATH.exists():
        return {}
    catalog = load_json(CATALOG_PATH)
    return {
        f.get("family_id"): f.get("lifecycle_label")
        for f in catalog.get("families", [])
        if isinstance(f, dict)
    }


def forbidden_keys(node: Any, path: str = "") -> list[str]:
    """Recursively collect any credential-ish key names in an emitted artifact."""
    found: list[str] = []
    if isinstance(node, dict):
        for key, value in node.items():
            lowered = str(key).lower()
            if any(sub in lowered for sub in _FORBIDDEN_KEY_SUBSTRINGS):
                found.append(f"{path}/{key}" if path else key)
            found.extend(forbidden_keys(value, f"{path}/{key}" if path else str(key)))
    elif isinstance(node, list):
        for i, item in enumerate(node):
            found.extend(forbidden_keys(item, f"{path}[{i}]"))
    return found


def emitted_artifact_violations(register: dict) -> list[str]:
    """Check that every emitted artifact is a real, well-formed, copy-safe interchange object
    whose per-surface renderings agree with the row's consumer-agreement block."""
    out: list[str] = []
    for row in register.get("rows", []):
        fid = row["family_id"]
        artifact_ref = row["runner"]["artifact_ref"]
        path = REPO_ROOT / artifact_ref
        if not path.exists():
            out.append(f"{fid}: emitted artifact missing at {artifact_ref}")
            continue
        art = load_json(path)

        if art.get("record_kind") != gen.EMITTED_ARTIFACT_RECORD_KIND:
            out.append(f"{fid}: emitted artifact record_kind is not the interchange envelope kind")
        if art.get("family_id") != fid:
            out.append(f"{fid}: emitted artifact family_id mismatch")
        if art.get("emitted_record_kind") != row["runner"]["emitted_record_kind"]:
            out.append(f"{fid}: emitted artifact emitted_record_kind disagrees with the row")

        version_field = row["contract_version_field"]
        if art.get(version_field) != row["contract_version"]:
            out.append(
                f"{fid}: emitted artifact contract version field `{version_field}` "
                "disagrees with the row"
            )
        if art.get("lifecycle_label") != row["lifecycle_label"]:
            out.append(f"{fid}: emitted artifact lifecycle_label disagrees with the row")
        if art.get("conformance_class") != row["conformance_class"]:
            out.append(f"{fid}: emitted artifact conformance_class disagrees with the row")

        # Required provenance must be present and not stripped.
        provenance = art.get("provenance", {})
        for required in ("exported_by_surface", "build_identity_ref", "source_record_class", "redaction_class"):
            if not provenance.get(required):
                out.append(f"{fid}: emitted artifact missing required provenance field {required}")

        # Degraded state must be in the vocabulary (or the literal "none").
        degraded = art.get("degraded_state")
        if degraded not in (["none"] + gen.DEGRADED_STATES):
            out.append(f"{fid}: emitted artifact degraded_state `{degraded}` is off-vocabulary")

        # No credential-ish keys anywhere in the artifact.
        for bad in forbidden_keys(art):
            out.append(f"{fid}: emitted artifact carries a forbidden credential-ish key `{bad}`")

        # Per-surface renderings must cover every consumer surface and agree on version and
        # lifecycle label (the cross-surface agreement the runner proves).
        renderings = art.get("surface_renderings", {})
        agreement = row["consumer_agreement"]
        for surface in gen.CONSUMER_SURFACES:
            r = renderings.get(surface)
            if r is None:
                out.append(f"{fid}: emitted artifact missing surface rendering for {surface}")
                continue
            if r.get("contract_version") != agreement["agreed_contract_version"]:
                out.append(
                    f"{fid}: surface {surface} disagrees on contract version with consumer agreement"
                )
            if r.get("lifecycle_label") != agreement["agreed_lifecycle_label"]:
                out.append(
                    f"{fid}: surface {surface} disagrees on lifecycle label with consumer agreement"
                )
            if r.get("degraded_states") != agreement["agreed_degraded_states"]:
                out.append(
                    f"{fid}: surface {surface} disagrees on degraded-state vocabulary with consumer agreement"
                )
    return out


def main() -> int:
    try:
        from jsonschema import Draft202012Validator
    except Exception as exc:  # pragma: no cover
        print(f"[m5-interchange] error: python jsonschema is required: {exc}", file=sys.stderr)
        return 2

    failures: list[str] = []

    if not SCHEMA_PATH.exists():
        print(f"[m5-interchange] error: missing schema {SCHEMA_PATH}", file=sys.stderr)
        return 2
    if not gen.REGISTER_PATH.exists():
        print(f"[m5-interchange] error: missing register {gen.REGISTER_PATH}", file=sys.stderr)
        return 2

    schema = load_json(SCHEMA_PATH)
    validator = Draft202012Validator(schema)
    register = load_json(gen.REGISTER_PATH)

    # 1) Schema validation of the canonical register.
    for err in sorted(validator.iter_errors(register), key=lambda e: list(e.path)):
        loc = "/".join(str(p) for p in err.path) or "<root>"
        failures.append(f"schema: {loc}: {err.message}")

    # 2) Semantic invariants on the canonical register.
    for msg in semantic_violations(register):
        failures.append(f"semantic: {msg}")

    # 3) Regenerator drift: the register and every generated companion must match what the
    #    regenerator builds from the upstream contract truth.
    built = gen.build_register()
    if register != built:
        failures.append(
            "drift: artifacts/contracts/m5-interchange-conformance.json is stale; "
            "run tools/regenerate_m5_interchange_conformance.py"
        )

    if not gen.VALIDATOR_MANIFEST_PATH.exists() or load_json(gen.VALIDATOR_MANIFEST_PATH) != gen.build_validator_manifest(built):
        failures.append("drift: validators/m5-interchange/manifest.json is stale; run the regenerator")
    for family, row in zip(gen.FAMILIES, built["rows"]):
        descriptor_path = gen.VALIDATORS_DIR / f"{family['family_id']}.json"
        if not descriptor_path.exists() or load_json(descriptor_path) != gen.build_validator_descriptor(family, row):
            failures.append(f"drift: {descriptor_path.relative_to(REPO_ROOT)} is stale; run the regenerator")
        emitted_path = gen.EMITTED_DIR / f"{family['family_id']}.json"
        if not emitted_path.exists() or load_json(emitted_path) != gen.build_emitted_artifact(family, row):
            failures.append(f"drift: {emitted_path.relative_to(REPO_ROOT)} is stale; run the regenerator")

    for path, builder in [
        (gen.REPORT_PATH, gen.build_report),
        (gen.VALIDATORS_README_PATH, gen.build_validators_readme),
        (gen.HELP_DOC_PATH, gen.build_help_doc),
        (gen.OVERVIEW_DOC_PATH, gen.build_overview_doc),
        (gen.EVIDENCE_DOC_PATH, gen.build_evidence_doc),
    ]:
        want = builder(built)
        if not want.endswith("\n"):
            want += "\n"
        if not path.exists() or path.read_text(encoding="utf-8") != want:
            failures.append(f"drift: {path.relative_to(REPO_ROOT)} is stale; run the regenerator")

    capture = gen.CAPTURE_PATH
    if not capture.exists() or load_json(capture) != gen.build_capture(built):
        failures.append(f"drift: {capture.relative_to(REPO_ROOT)} is stale; run the regenerator")

    # 4) Cross-source lifecycle consistency: a catalog-linked family's lifecycle label equals
    #    the contract catalog's published label for that family.
    catalog_labels = load_catalog_labels()
    for row in register.get("rows", []):
        cat = row.get("catalog_family_id")
        if cat:
            if cat not in catalog_labels:
                failures.append(f"{row['family_id']}: catalog family {cat} is not in the contract catalog")
            elif catalog_labels[cat] != row.get("lifecycle_label"):
                failures.append(
                    f"{row['family_id']}: lifecycle_label {row.get('lifecycle_label')} disagrees with "
                    f"catalog label {catalog_labels[cat]} for {cat}"
                )

    # 5) Path existence for structural refs.
    for ref in sorted(set(collect_structural_refs(register))):
        if not is_path_ref(ref):
            continue
        if not (REPO_ROOT / candidate_path(ref)).exists():
            failures.append(f"missing referenced path: {ref}")

    # 6) Emitted-artifact corpus checks.
    for msg in emitted_artifact_violations(register):
        failures.append(f"emitted: {msg}")

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
        print("[m5-interchange] FAIL", file=sys.stderr)
        for msg in failures[:200]:
            print(f"  - {msg}", file=sys.stderr)
        if len(failures) > 200:
            print(f"  ... ({len(failures) - 200} more)", file=sys.stderr)
        return 1

    print(
        "[m5-interchange] OK: register, validators, emitted-artifact corpus, conformance "
        "report, docs, catalog consistency, paths, and negative fixtures validate"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
