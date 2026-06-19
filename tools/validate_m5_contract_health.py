#!/usr/bin/env python3
"""Validate the M5 contract-health register, its CI gates, and the shiproom dashboard.

Validates:
- ``artifacts/release/m5-contract-health.json`` against
  ``schemas/public/m5-contracts/m5_contract_health.schema.json``
- the register's semantic invariants (duplicate family ids, closed-vocabulary
  membership, one gate per gate kind, the health state recomputed from the gates,
  the blocker decision recomputed from the health state, the mirror/offline
  posture consistent with the health state, the top-level blocker decision
  recomputed from the rows, and the summary recomputed from the rows), mirroring
  the typed Rust consumer
- that the register, the gate manifest, the per-gate descriptors, the gate README,
  the shiproom dashboard, the Help/overview/evidence docs, the capture, and the
  negative fixtures match the regenerator (no hand-edit drift)
- that each family's lifecycle label agrees with the publication matrix's effective
  published label and that the family appears in the matrix and the contract catalog
- that every referenced repo-relative structural path exists
- that the checked-in negative fixtures under
  ``fixtures/contracts/m5-contract-health/`` are rejected by the semantic invariants

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

import regenerate_m5_contract_health as gen  # noqa: E402

REPO_ROOT = gen.REPO_ROOT
SCHEMA_PATH = REPO_ROOT / "schemas/public/m5-contracts/m5_contract_health.schema.json"
MATRIX_PATH = gen.MATRIX_PATH
CATALOG_PATH = gen.CATALOG_PATH

# The gate ranks the model and the validator share when recomputing health.
_OUTCOME_RANK = {"pass": 0, "downgrade": 1, "fail": 2}


def load_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def is_path_ref(value: str) -> bool:
    if not value:
        return False
    if "://" in value:
        return False
    if ":" in value:
        # Symbolic ref (e.g. manifest_entry:...) — not a repo path.
        return False
    return "/" in value or value.endswith(
        (".json", ".yaml", ".yml", ".md", ".py", ".sh", ".toml", ".wit")
    )


def candidate_path(ref: str) -> str:
    return ref.split("#", 1)[0] if "#" in ref else ref


def collect_structural_refs(register: dict) -> list[str]:
    """Collect only the repo-relative structural paths this row authors.

    Symbolic refs (artifact-graph nodes, release packets, release candidates) and
    matrix-sourced evidence refs are intentionally excluded; they are governed by
    the matrix and artifact-graph validators.
    """
    refs: list[str] = []
    for key in (
        "overview_page",
        "evidence_page",
        "help_page",
        "shiproom_dashboard_page",
        "gate_manifest_ref",
        "contract_catalog_ref",
        "publication_matrix_ref",
        "evidence_index_ref",
    ):
        value = register.get(key)
        if isinstance(value, str):
            refs.append(value)
    refs.append(register.get("build_identity", {}).get("build_identity_ref", ""))
    for gate in register.get("gate_catalog", []):
        refs.append(gate.get("descriptor_ref", ""))
    for row in register.get("rows", []):
        refs.append(row.get("catalog_entry_ref", ""))
        refs.append(row.get("matrix_row_ref", ""))
        refs.append(row.get("package_identity", {}).get("schema_or_spec_ref", ""))
    return [r for r in refs if isinstance(r, str) and r]


def recompute_health(row: dict) -> str:
    gates = row.get("gates", [])
    any_required_fail = any(g.get("outcome") == "fail" and g.get("required") for g in gates)
    any_required_downgrade = any(g.get("outcome") == "downgrade" and g.get("required") for g in gates)
    if row.get("release_blocking") and any_required_fail:
        return "blocked"
    if row.get("narrowed") or any_required_downgrade or any_required_fail:
        return "narrowed"
    return "healthy"


def semantic_violations(register: dict) -> list[str]:
    """Recompute the register's derived state and report disagreements.

    Mirrors the typed Rust consumer's `validate()`. The canonical register must
    return no violations; each negative fixture must return at least one.
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
        ("gate_kinds", gen.GATE_KINDS),
        ("gate_outcomes", gen.GATE_OUTCOMES),
        ("freshness_states", gen.FRESHNESS_STATES),
        ("health_states", gen.HEALTH_STATES),
        ("blocker_decisions", gen.BLOCKER_DECISIONS),
        ("mirror_parity_states", gen.MIRROR_PARITY_STATES),
        ("gap_reasons", gen.GAP_REASONS),
        ("remediation_actions", gen.REMEDIATION_ACTIONS),
    ]:
        if register.get(field) != expected:
            violations.append(f"closed vocabulary mismatch: {field}")

    gate_catalog_kinds = [g.get("gate_kind") for g in register.get("gate_catalog", [])]
    if gate_catalog_kinds != gen.GATE_KINDS:
        violations.append("gate_catalog kinds off the canonical list")

    rows = register.get("rows", [])
    seen: set[str] = set()
    for row in rows:
        fid = row.get("family_id", "<unknown>")
        if fid in seen:
            violations.append(f"rows.duplicate_family_id: {fid}")
        seen.add(fid)

        if row.get("lifecycle_label") not in gen.LIFECYCLE_LABELS:
            violations.append(f"{fid}: lifecycle_label not in vocabulary")
        if row.get("health_state") not in gen.HEALTH_STATES:
            violations.append(f"rows.unknown_health_state: {fid}")

        # One gate per gate kind, in the canonical order.
        gate_kinds = [g.get("gate_kind") for g in row.get("gates", [])]
        if gate_kinds != gen.GATE_KINDS:
            violations.append(f"rows.gate_coverage: {fid} gates must be exactly the gate-kind set")

        expected_health = recompute_health(row)
        if row.get("health_state") != expected_health:
            violations.append(
                f"rows.unknown_health_state: {fid} health_state "
                f"{row.get('health_state')} disagrees with the gates ({expected_health})"
            )

        expected_decision = "hold" if expected_health == "blocked" else "clear"
        blocker = row.get("blocker", {})
        if blocker.get("decision") != expected_decision:
            violations.append(
                f"rows.blocker_decision: {fid} blocker decision {blocker.get('decision')} "
                f"disagrees with the health state ({expected_decision})"
            )

        # Mirror/offline posture must follow the gate outputs.
        linkage = row.get("graph_linkage", {})
        parity = linkage.get("mirror_parity")
        offline = linkage.get("offline_verifiable")
        if offline != (parity in ("current", "not_applicable")):
            violations.append(f"rows.mirror_parity: {fid} offline_verifiable disagrees with mirror_parity")
        if expected_health == "blocked" and offline:
            violations.append(f"rows.mirror_parity: {fid} blocked family must not be offline_verifiable")

    # Top-level blocker decision recomputed from the rows.
    blocked_ids = [r.get("family_id") for r in rows if recompute_health(r) == "blocked"]
    blockers = register.get("blockers", {})
    if blockers.get("blocking_family_ids") != blocked_ids:
        violations.append("blockers.block: blocking_family_ids disagree with the blocked rows")
    expected_top = "hold" if blocked_ids else "clear"
    if blockers.get("decision") != expected_top:
        violations.append("blockers.decision: top-level decision disagrees with the blocked rows")

    if register.get("summary") != gen.compute_summary(rows):
        violations.append("summary.count_mismatch: summary counts disagree with the rows")

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
        print(f"[m5-contract-health] error: python jsonschema is required: {exc}", file=sys.stderr)
        return 2

    failures: list[str] = []

    if not SCHEMA_PATH.exists():
        print(f"[m5-contract-health] error: missing schema {SCHEMA_PATH}", file=sys.stderr)
        return 2
    if not gen.REGISTER_PATH.exists():
        print(f"[m5-contract-health] error: missing register {gen.REGISTER_PATH}", file=sys.stderr)
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

    # 3) Regenerator drift: the register and every generated companion must match
    #    what the regenerator builds from the upstream contract truth.
    built = gen.build_register()
    if register != built:
        failures.append(
            "drift: artifacts/release/m5-contract-health.json is stale; "
            "run tools/regenerate_m5_contract_health.py"
        )

    if not gen.GATE_MANIFEST_PATH.exists() or load_json(gen.GATE_MANIFEST_PATH) != gen.build_gate_manifest(built):
        failures.append("drift: ci/contracts/m5-contract-gates/manifest.json is stale; run the regenerator")
    for gate in built["gate_catalog"]:
        descriptor_path = gen.GATES_DIR / f"{gate['gate_kind']}.json"
        if not descriptor_path.exists() or load_json(descriptor_path) != gen.build_gate_descriptor(built, gate):
            failures.append(f"drift: {descriptor_path.relative_to(REPO_ROOT)} is stale; run the regenerator")

    for path, builder in [
        (gen.GATES_README_PATH, gen.build_gates_readme),
        (gen.SHIPROOM_DASHBOARD_PATH, gen.build_shiproom_dashboard),
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

    # 4) Cross-source lifecycle consistency: each family appears in the matrix and
    #    the contract catalog, and its lifecycle label equals the matrix's
    #    published label after narrowing.
    published = load_matrix_published_labels()
    catalog_ids: set[str] = set()
    if CATALOG_PATH.exists():
        catalog_ids = {f.get("family_id") for f in load_json(CATALOG_PATH).get("families", [])}
    for row in register.get("rows", []):
        fid = row.get("family_id")
        if published:
            if fid not in published:
                failures.append(f"{fid}: no matching row in the publication matrix")
            elif published[fid] != row.get("lifecycle_label"):
                failures.append(
                    f"{fid}: lifecycle_label {row.get('lifecycle_label')} disagrees with "
                    f"matrix published_label {published[fid]}"
                )
        if catalog_ids and fid not in catalog_ids:
            failures.append(f"{fid}: no matching entry in the contract catalog")
    for fid in published:
        if not any(r.get("family_id") == fid for r in register.get("rows", [])):
            failures.append(f"register omits published matrix family: {fid}")

    # 5) Path existence for structural refs.
    for ref in sorted(set(collect_structural_refs(register))):
        if not is_path_ref(ref):
            continue
        if not (REPO_ROOT / candidate_path(ref)).exists():
            failures.append(f"missing referenced path: {ref}")

    # 6) Negative fixtures must be rejected by the semantic invariants.
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
        print("[m5-contract-health] FAIL", file=sys.stderr)
        for msg in failures[:200]:
            print(f"  - {msg}", file=sys.stderr)
        if len(failures) > 200:
            print(f"  ... ({len(failures) - 200} more)", file=sys.stderr)
        return 1

    print(
        "[m5-contract-health] OK: register, CI gates, shiproom dashboard, docs, matrix "
        "consistency, paths, and negative fixtures validate"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
