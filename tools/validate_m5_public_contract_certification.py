#!/usr/bin/env python3
"""Validate the M5 public-contract certification register, its companion docs, the shiproom
dashboard, the CI capture, and the negative fixtures.

Validates:
- ``artifacts/certification/m5-public-contract-certification.json`` against
  ``schemas/public/m5-contracts/m5_public_contract_certification.schema.json``
- the register's semantic invariants (duplicate family ids, closed-vocabulary membership,
  one pillar per pillar kind, the certification state recomputed from the pillars and the
  claim/published labels, the certified label never greener than the public claim, the
  promotion decision recomputed from the withheld release-blocking rows, and the summary
  recomputed from the rows), mirroring the typed Rust consumer
- that the register, the report, the shiproom dashboard, the Help/overview/evidence docs,
  the capture, and the negative fixtures match the regenerator (no hand-edit drift)
- that every claimed family in the upstream contract-health register appears with the same
  certified label, public claim, and published label the matrix and health register carry
- that every referenced repo-relative structural path exists
- that the checked-in negative fixtures under
  ``fixtures/contracts/m5-public-contract-certification/`` are rejected by the semantic
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

import regenerate_m5_public_contract_certification as gen  # noqa: E402

REPO_ROOT = gen.REPO_ROOT
SCHEMA_PATH = REPO_ROOT / gen.SCHEMA_REF
CONTRACT_HEALTH_PATH = gen.CONTRACT_HEALTH_PATH
MATRIX_PATH = gen.MATRIX_PATH

# Keys the register must never carry (no credential bodies or raw provider payloads).
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


def label_rank(label: str) -> int:
    return gen.LIFECYCLE_LABELS.index(label)


def collect_structural_refs(register: dict) -> list[str]:
    refs: list[str] = []
    for key in (
        "overview_page",
        "evidence_page",
        "help_page",
        "report_page",
        "shiproom_dashboard_page",
        "contract_health_ref",
        "publication_matrix_ref",
        "contract_catalog_ref",
        "json_schema_catalog_ref",
        "openapi_catalog_ref",
        "wit_publication_ref",
        "reader_writer_compat_ref",
        "interchange_conformance_ref",
        "evidence_index_ref",
        "build_identity_ref",
    ):
        value = register.get(key)
        if isinstance(value, str):
            refs.append(value)
    for row in register.get("rows", []):
        proof = row.get("proof", {})
        for key in (
            "health_row_ref",
            "matrix_row_ref",
            "catalog_entry_ref",
            "contract_form_catalog_ref",
            "compatibility_report_ref",
        ):
            if isinstance(proof.get(key), str):
                refs.append(proof[key])
        for pillar in row.get("pillars", []):
            if isinstance(pillar.get("certifying_artifact_ref"), str):
                refs.append(pillar["certifying_artifact_ref"])
    return [r for r in refs if isinstance(r, str) and r]


def recompute_certification_state(row: dict) -> str:
    """Recompute the certification state from the row's own pillars and labels.

    Mirrors the regenerator's derivation from the upstream contract-health state: the health
    state is reconstructed from the required pillars (a missing required pillar on a
    release-blocking family is `blocked`; any stale or non-blocking-missing required pillar is
    `narrowed`; otherwise `healthy`), then mapped onto the certification state with the
    claim/published narrowing and retest flags.
    """
    pillars = row.get("pillars", [])
    any_missing = any(p.get("evidence_state") == "missing" and p.get("required") for p in pillars)
    any_stale = any(p.get("evidence_state") == "stale" and p.get("required") for p in pillars)
    release_blocking = bool(row.get("release_blocking"))
    downgraded = label_rank(row.get("source_published_label")) > label_rank(row.get("claim_label"))
    retest = bool(row.get("blocker", {}).get("retest_needed"))

    if release_blocking and any_missing:
        health = "blocked"
    elif any_stale or any_missing:
        health = "narrowed"
    else:
        health = "healthy"

    if health == "blocked":
        return "withheld"
    if health == "narrowed":
        if downgraded:
            return "narrowed_row_downgraded"
        if retest:
            return "narrowed_retest_pending"
        return "narrowed_stale"
    return "narrowed_row_downgraded" if downgraded else "certified"


def semantic_violations(register: dict) -> list[str]:
    """Recompute the register's derived state and report disagreements.

    The canonical register must return no violations; each negative fixture must return at
    least one.
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
        ("contract_forms", gen.CONTRACT_FORMS),
        ("pillar_kinds", gen.PILLAR_KINDS),
        ("evidence_states", gen.EVIDENCE_STATES),
        ("certification_states", gen.CERTIFICATION_STATES),
        ("certification_reasons", gen.CERTIFICATION_REASONS),
        ("stop_actions", gen.STOP_ACTIONS),
        ("mirror_parity_states", gen.MIRROR_PARITY_STATES),
        ("consumer_surfaces", gen.CONSUMER_SURFACES),
        ("decision_states", gen.DECISION_STATES),
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

        if row.get("certification_state") not in gen.CERTIFICATION_STATES:
            violations.append(f"rows.unknown_certification_state: {fid}")
            continue

        # One pillar per pillar kind, in canonical order.
        pillar_kinds = [p.get("pillar_kind") for p in row.get("pillars", [])]
        if pillar_kinds != gen.PILLAR_KINDS:
            violations.append(
                f"rows.pillar_coverage: {fid} pillars must be exactly the pillar-kind set"
            )

        expected_state = recompute_certification_state(row)
        if row.get("certification_state") != expected_state:
            violations.append(
                f"rows.certification_state: {fid} certification_state "
                f"{row.get('certification_state')} disagrees with the pillars ({expected_state})"
            )

        # The certified label may never be greener than the public claim.
        if label_rank(row.get("certified_label")) < label_rank(row.get("claim_label")):
            violations.append(
                f"rows.claim_parity: {fid} certified label {row.get('certified_label')} is "
                f"greener than the public claim {row.get('claim_label')}"
            )

        # Active reasons and stop actions must be drawn from the closed vocabularies.
        for reason in row.get("active_certification_reasons", []):
            if reason not in gen.CERTIFICATION_REASONS:
                violations.append(f"rows.reason_vocabulary: {fid} unknown reason {reason}")
        for action in row.get("stop_actions", []):
            if action not in gen.STOP_ACTIONS:
                violations.append(f"rows.action_vocabulary: {fid} unknown stop action {action}")

        # The per-row blocker decision must follow the state.
        expected_blocker = "hold" if expected_state == "withheld" else "clear"
        if row.get("blocker", {}).get("decision") != expected_blocker:
            violations.append(
                f"rows.blocker_decision: {fid} blocker decision disagrees with the state "
                f"({expected_blocker})"
            )

    # Promotion decision recomputed from the withheld, release-blocking rows.
    blocking = [
        r.get("family_id")
        for r in rows
        if recompute_certification_state(r) == "withheld" and r.get("release_blocking")
    ]
    promotion = register.get("promotion", {})
    if promotion.get("blocking_family_ids") != blocking:
        violations.append("promotion.block: blocking_family_ids disagree with the withheld rows")
    expected_decision = "hold" if blocking else "proceed"
    if promotion.get("decision") != expected_decision:
        violations.append("promotion.decision: top-level decision disagrees with the withheld rows")

    if register.get("summary") != gen.compute_summary(rows):
        violations.append("summary.count_mismatch: summary counts disagree with the rows")

    return violations


def forbidden_keys(node: Any, path: str = "") -> list[str]:
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


def cross_source_violations(register: dict) -> list[str]:
    """Every claimed family certifies the same labels the upstream health register and the
    publication matrix carry, so the certification join never restates a different truth."""
    out: list[str] = []
    if not CONTRACT_HEALTH_PATH.exists():
        out.append(f"upstream contract-health register missing at {gen.CONTRACT_HEALTH_REF}")
        return out
    if not MATRIX_PATH.exists():
        out.append(f"upstream publication matrix missing at {gen.MATRIX_REF}")
        return out

    health = {r["family_id"]: r for r in load_json(CONTRACT_HEALTH_PATH).get("rows", [])}
    matrix = {r["family_id"]: r for r in load_json(MATRIX_PATH).get("rows", [])}

    cert_ids = {r["family_id"] for r in register.get("rows", [])}
    missing = sorted(set(health) - cert_ids)
    for fid in missing:
        out.append(f"family {fid} is in the contract-health register but not certified")

    for row in register.get("rows", []):
        fid = row["family_id"]
        h = health.get(fid)
        m = matrix.get(fid)
        if h is None:
            out.append(f"{fid}: certified family is not in the contract-health register")
            continue
        if m is None:
            out.append(f"{fid}: certified family is not in the publication matrix")
            continue
        if row.get("certified_label") != h.get("lifecycle_label"):
            out.append(
                f"{fid}: certified_label {row.get('certified_label')} disagrees with the "
                f"contract-health lifecycle label {h.get('lifecycle_label')}"
            )
        if row.get("source_published_label") != h.get("published_label"):
            out.append(
                f"{fid}: source_published_label disagrees with the contract-health published label"
            )
        if row.get("claim_label") != m.get("claim_label"):
            out.append(
                f"{fid}: claim_label {row.get('claim_label')} disagrees with the matrix claim "
                f"label {m.get('claim_label')}"
            )
        if row.get("release_blocking") != h.get("release_blocking"):
            out.append(f"{fid}: release_blocking disagrees with the contract-health register")
    return out


def main() -> int:
    try:
        from jsonschema import Draft202012Validator
    except Exception as exc:  # pragma: no cover
        print(f"[m5-public-contract-cert] error: python jsonschema is required: {exc}", file=sys.stderr)
        return 2

    failures: list[str] = []

    if not SCHEMA_PATH.exists():
        print(f"[m5-public-contract-cert] error: missing schema {SCHEMA_PATH}", file=sys.stderr)
        return 2
    if not gen.REGISTER_PATH.exists():
        print(f"[m5-public-contract-cert] error: missing register {gen.REGISTER_PATH}", file=sys.stderr)
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

    # 3) No credential-ish keys anywhere in the register.
    for bad in forbidden_keys(register):
        failures.append(f"forbidden: register carries a credential-ish key `{bad}`")

    # 4) Regenerator drift: the register and every generated companion must match what the
    #    regenerator builds from the upstream contract truth.
    built = gen.build_register()
    if register != built:
        failures.append(
            "drift: artifacts/certification/m5-public-contract-certification.json is stale; "
            "run tools/regenerate_m5_public_contract_certification.py"
        )

    for path, builder in [
        (gen.REPORT_PATH, gen.build_report),
        (gen.SHIPROOM_PATH, gen.build_shiproom_dashboard),
        (gen.HELP_DOC_PATH, gen.build_help_doc),
        (gen.OVERVIEW_DOC_PATH, gen.build_overview_doc),
        (gen.EVIDENCE_DOC_PATH, gen.build_evidence_doc),
    ]:
        want = builder(built)
        if not want.endswith("\n"):
            want += "\n"
        if not path.exists() or path.read_text(encoding="utf-8") != want:
            failures.append(f"drift: {path.relative_to(REPO_ROOT)} is stale; run the regenerator")

    if not gen.CAPTURE_PATH.exists() or load_json(gen.CAPTURE_PATH) != gen.build_capture(built):
        failures.append(f"drift: {gen.CAPTURE_PATH.relative_to(REPO_ROOT)} is stale; run the regenerator")

    # 5) Cross-source consistency with the upstream health register and matrix.
    for msg in cross_source_violations(register):
        failures.append(f"cross-source: {msg}")

    # 6) Path existence for structural refs.
    for ref in sorted(set(collect_structural_refs(register))):
        if not is_path_ref(ref):
            continue
        if not (REPO_ROOT / candidate_path(ref)).exists():
            failures.append(f"missing referenced path: {ref}")

    # 7) Negative fixtures must be rejected by the semantic invariants.
    fixtures = gen.build_negative_fixtures(built)
    for filename, data in fixtures.items():
        fixture_path = gen.NEGATIVE_DIR / filename
        if not fixture_path.exists():
            failures.append(f"fixtures: missing {filename}")
            continue
        if load_json(fixture_path) != data:
            failures.append(f"drift: fixtures/contracts/m5-public-contract-certification/{filename} is stale; run the regenerator")
        if not semantic_violations(load_json(fixture_path)):
            failures.append(f"fixtures: {filename} was not rejected by the semantic invariants")

    cases_path = gen.NEGATIVE_DIR / "cases.json"
    if not cases_path.exists():
        failures.append("fixtures: missing cases.json")
    elif load_json(cases_path) != {"cases": gen.NEGATIVE_CASES}:
        failures.append("drift: fixtures/contracts/m5-public-contract-certification/cases.json is stale; run the regenerator")

    if failures:
        print("[m5-public-contract-cert] FAIL", file=sys.stderr)
        for msg in failures[:200]:
            print(f"  - {msg}", file=sys.stderr)
        if len(failures) > 200:
            print(f"  ... ({len(failures) - 200} more)", file=sys.stderr)
        return 1

    print(
        "[m5-public-contract-cert] OK: register, report, shiproom dashboard, docs, capture, "
        "cross-source consistency, paths, and negative fixtures validate"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
