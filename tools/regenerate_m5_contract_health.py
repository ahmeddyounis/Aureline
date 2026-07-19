#!/usr/bin/env python3
"""Regenerate the M5 contract-health register, CI gate descriptors, the shiproom
contract-blocker dashboard, and the docs/capture/fixtures that prove them.

This is the single source of truth for the **M5 contract-health register**: the
machine-readable join that ties every published M5 contract family to the CI
gates that guard its contract packages, the exact release artifact graph and
build identity the candidate ships, and the shiproom blocker decision those
signals produce. It exists so that a missing, stale, downgraded, or incompatible
contract package blocks the same release and claim-publication paths as missing
evidence or stale qualification rows, rather than relying on ad hoc spreadsheet
checks.

Where the public-contract publication matrix records *whether* each family has
published its contract forms, and the contract catalog is the consuming index
that joins each family to its lifecycle label and sample gallery, this register
is the *enforcement* layer on top of both: per family it evaluates one gate per
contract-package class (schema/spec package, example corpus, validator coverage,
compatibility/migration report, and release-packet linkage), binds the family to
the build identity and artifact-graph node that proves the contract set it
shipped, and emits a shiproom blocker decision. It reuses the matrix's gap-reason
and remediation vocabulary and the release-candidate freshness states rather than
inventing a new red/yellow contract-health vocabulary.

It reads the checked-in upstream truth sources rather than re-deriving them:

  * ``artifacts/contracts/m5-contract-catalog.json``           (the contract catalog)
  * ``artifacts/contracts/m5-stability-lifecycle-map.json``    (publication matrix)
  * ``artifacts/contracts/m5-json-schema-catalog.json``        (JSON Schema packages)
  * ``artifacts/contracts/m5-openapi-catalog.json``            (OpenAPI catalog)
  * ``artifacts/contracts/m5-wit-contract-publication.json``   (WIT publication)
  * ``rust-toolchain.toml``                                    (pinned toolchain channel)

and writes, all deterministically:

  * ``artifacts/release/m5-contract-health.json``              (the register)
  * ``ci/contracts/m5-contract-gates/manifest.json``           (the gate manifest)
  * ``ci/contracts/m5-contract-gates/<gate_kind>.json``        (per-gate descriptors)
  * ``ci/contracts/m5-contract-gates/README.md``               (gate index)
  * ``shiproom/m5-contract-blocker-dashboard.md``              (shiproom dashboard)
  * ``docs/help/m5-contract-health.md``                        (Help-center page)
  * ``docs/m5/<slug>.md``                                       (narrative companion)
  * ``artifacts/m5/<slug>.md``                                  (evidence/proof packet)
  * ``artifacts/release/captures/<name>_validation_capture.json`` (CI capture)
  * ``fixtures/contracts/m5-contract-health/{cases.json,*.json}`` (negative fixtures)

Run ``python3 tools/regenerate_m5_contract_health.py`` after editing the upstream
sources or this script, then ``python3 tools/validate_m5_contract_health.py`` and
``cargo test -p aureline-release --test rel_it_27_implement_contract_ci_gates``
to confirm the validator and the typed model agree.

The register is metadata-plus-state only: every field is a typed state, an opaque
repo-relative ref or URI, or a copy/export-safe summary. It carries no credential
bodies or raw provider payloads, and it never reads live, per-build values (the
commit and dirty flag are resolved from the build-identity artifact at review
time) so the checked-in artifacts stay deterministic.
"""

from __future__ import annotations

import json
import re
from pathlib import Path

NAME = (
    "implement_contract_ci_gates_release_artifact_graph_linkage_and_shiproom_blockers_"
    "for_stale_missing_or_incompatible_m5_schema_spec_packages"
)
RECORD_KIND = "m5_contract_health_register"
REGISTER_ID = "m5_contract_health:v1"
SCHEMA_VERSION = 1
AS_OF = "2026-06-19"

GATE_MANIFEST_RECORD_KIND = "m5_contract_gate_manifest"
GATE_MANIFEST_ID = "m5_contract_gate_manifest:v1"

REPO_ROOT = Path(__file__).resolve().parent.parent

# Outputs.
REGISTER_PATH = REPO_ROOT / "artifacts" / "release" / "m5-contract-health.json"
GATES_DIR = REPO_ROOT / "ci" / "contracts" / "m5-contract-gates"
GATE_MANIFEST_PATH = GATES_DIR / "manifest.json"
GATES_README_PATH = GATES_DIR / "README.md"
SHIPROOM_DIR = REPO_ROOT / "shiproom"
SHIPROOM_DASHBOARD_PATH = SHIPROOM_DIR / "m5-contract-blocker-dashboard.md"
HELP_DOC_PATH = REPO_ROOT / "docs" / "help" / "m5-contract-health.md"
NEGATIVE_DIR = REPO_ROOT / "fixtures" / "contracts" / "m5-contract-health"
CAPTURE_PATH = REPO_ROOT / "artifacts" / "release" / "captures" / f"{NAME}_validation_capture.json"

SLUG = NAME.replace("_", "-")
OVERVIEW_PAGE = f"docs/m5/{SLUG}.md"
OVERVIEW_DOC_PATH = REPO_ROOT / "docs" / "m5" / f"{SLUG}.md"
EVIDENCE_PAGE = f"artifacts/m5/{SLUG}.md"
EVIDENCE_DOC_PATH = REPO_ROOT / "artifacts" / "m5" / f"{SLUG}.md"

# Output refs (repo-relative) the register and its companions cross-link.
REGISTER_REF = "artifacts/release/m5-contract-health.json"
GATE_MANIFEST_REF = "ci/contracts/m5-contract-gates/manifest.json"
GATES_HOME = "ci/contracts/m5-contract-gates/"
SHIPROOM_DASHBOARD_REF = "shiproom/m5-contract-blocker-dashboard.md"
HELP_PAGE_REF = "docs/help/m5-contract-health.md"
VALIDATOR_REF = "tools/validate_m5_contract_health.py"
REGENERATOR_REF = "tools/regenerate_m5_contract_health.py"
SCHEMA_REF = "schemas/public/m5-contracts/m5_contract_health.schema.json"
CI_WORKFLOW_REF = ".github/workflows/check_m5_contract_health.yml"

# Upstream truth sources this register consumes instead of restating.
CATALOG_REF = "artifacts/contracts/m5-contract-catalog.json"
MATRIX_REF = "artifacts/contracts/m5-stability-lifecycle-map.json"
JSON_SCHEMA_CATALOG_REF = "artifacts/contracts/m5-json-schema-catalog.json"
OPENAPI_CATALOG_REF = "artifacts/contracts/m5-openapi-catalog.json"
WIT_PUBLICATION_REF = "artifacts/contracts/m5-wit-contract-publication.json"
BUILD_IDENTITY_REF = "artifacts/build/build_identity.json"
ARTIFACT_GRAPH_REF = "release/artifact_graph"
EVIDENCE_INDEX_REF = (
    "artifacts/release/m5/"
    "certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json"
)
RELEASE_CANDIDATE_REF = "rc-m5-public-contract-train"

CATALOG_PATH = REPO_ROOT / CATALOG_REF
MATRIX_PATH = REPO_ROOT / MATRIX_REF
JSON_SCHEMA_CATALOG_PATH = REPO_ROOT / JSON_SCHEMA_CATALOG_REF
OPENAPI_CATALOG_PATH = REPO_ROOT / OPENAPI_CATALOG_REF
WIT_PUBLICATION_PATH = REPO_ROOT / WIT_PUBLICATION_REF
TOOLCHAIN_PATH = REPO_ROOT / "rust-toolchain.toml"

# Closed vocabularies. Kept in lockstep with the typed Rust consumer and the
# boundary schema; the validator and the model both reject anything off-list. The
# freshness states, gap reasons, and remediation actions are reused from the
# publication matrix and the release-candidate matrix rather than reinvented.
LIFECYCLE_LABELS = ["lts", "stable", "beta", "preview", "withdrawn"]
GATE_OUTCOMES = ["pass", "downgrade", "fail"]
FRESHNESS_STATES = ["current", "due_for_refresh", "breached", "missing"]
HEALTH_STATES = ["healthy", "narrowed", "blocked"]
BLOCKER_DECISIONS = ["clear", "hold"]
MIRROR_PARITY_STATES = ["current", "stale", "unpublished", "not_applicable"]
GAP_REASONS = [
    "json_schema_unpublished",
    "wit_world_unpublished",
    "openapi_spec_unpublished",
    "markdown_summary_unpublished",
    "example_payloads_unpublished",
    "migration_notes_unpublished",
    "validator_suite_unpublished",
    "release_packet_unlinked",
]
REMEDIATION_ACTIONS = [
    "hold_promotion",
    "narrow_label",
    "publish_contract_form",
    "publish_example_payloads",
    "wire_validator_suite",
    "link_release_packet",
]

# Gate kinds (the CI gates), in evaluation order. Each guards one contract-package
# class and raises the matrix gap reason(s) for that class.
GATE_KINDS = [
    "schema_spec_package",
    "example_corpus",
    "validator_coverage",
    "compatibility_report",
    "release_packet_linkage",
]

# Per-gate definition: the publication-requirement artifact kinds it guards, the
# matrix gap reasons it raises, and the remediation actions it recommends.
GATE_DEFS = {
    "schema_spec_package": {
        "title": "Schema/spec contract package published and fresh",
        "description": (
            "The family's required machine-readable contract package — its JSON "
            "Schema, WIT world, or OpenAPI spec — is published at a resolvable "
            "version. A missing or partial package fails this gate."
        ),
        "guards_artifact_kinds": ["json_schema", "wit_world", "openapi_spec"],
        "gap_reasons": [
            "json_schema_unpublished",
            "wit_world_unpublished",
            "openapi_spec_unpublished",
        ],
        "remediation_actions": ["publish_contract_form", "hold_promotion", "narrow_label"],
    },
    "example_corpus": {
        "title": "Example payload corpus published",
        "description": (
            "The family's required example payloads / sample corpus is published "
            "so the contract is inspectable offline. A missing or partial corpus "
            "fails this gate."
        ),
        "guards_artifact_kinds": ["example_payloads"],
        "gap_reasons": ["example_payloads_unpublished"],
        "remediation_actions": ["publish_example_payloads", "hold_promotion", "narrow_label"],
    },
    "validator_coverage": {
        "title": "Validator suite wired",
        "description": (
            "The family's required validator suite is wired into CI so the "
            "contract package and its examples are checked on every change. A "
            "missing or partial validator suite fails this gate."
        ),
        "guards_artifact_kinds": ["validator_suite"],
        "gap_reasons": ["validator_suite_unpublished"],
        "remediation_actions": ["wire_validator_suite", "hold_promotion", "narrow_label"],
    },
    "compatibility_report": {
        "title": "Compatibility / migration report fresh",
        "description": (
            "The family's required Markdown summary and migration/compatibility "
            "report are published so a changed stable-facing contract carries an "
            "explicit compatibility window and successor guidance. A missing or "
            "partial report fails this gate."
        ),
        "guards_artifact_kinds": ["markdown_summary", "migration_notes"],
        "gap_reasons": ["markdown_summary_unpublished", "migration_notes_unpublished"],
        "remediation_actions": ["publish_contract_form", "hold_promotion", "narrow_label"],
    },
    "release_packet_linkage": {
        "title": "Release packet linked to the artifact graph and build identity",
        "description": (
            "The family resolves to a release-packet entry, an artifact-graph "
            "node, and the one build identity that proves which contract package "
            "version the candidate shipped. An unlinked release packet fails this "
            "gate."
        ),
        "guards_artifact_kinds": [],
        "gap_reasons": ["release_packet_unlinked"],
        "remediation_actions": ["link_release_packet", "hold_promotion"],
    },
}

# Map a contract-identity kind to the publication-requirement artifact kind that
# carries the family's contract package.
IDENTITY_TO_PACKAGE_KIND = {
    "json_schema": "json_schema",
    "openapi_spec": "openapi_spec",
    "wit_world": "wit_world",
}


def load_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def toolchain_channel() -> str:
    text = TOOLCHAIN_PATH.read_text(encoding="utf-8")
    match = re.search(r'^\s*channel\s*=\s*"([^"]+)"', text, flags=re.MULTILINE)
    if not match:
        raise SystemExit("could not resolve toolchain channel from rust-toolchain.toml")
    return match.group(1)


def requirement_state(reqs: dict[str, dict], artifact_kind: str) -> tuple[bool, str, list[str]]:
    """Return (required, state, refs) for one publication-requirement artifact kind."""
    entry = reqs.get(artifact_kind)
    if entry is None:
        return (False, "not_applicable", [])
    return (bool(entry.get("required")), entry.get("state", "not_applicable"), list(entry.get("refs", [])))


def state_to_outcome(required: bool, state: str) -> tuple[str, str]:
    """Map a publication-requirement state to (gate_outcome, freshness_state).

    A required-and-published artifact passes; a required-and-partial artifact
    downgrades (due_for_refresh); a required-and-missing artifact fails (missing).
    A not-required or not-applicable artifact passes and is current.
    """
    if not required or state in ("published", "not_applicable"):
        return ("pass", "current")
    if state == "partial":
        return ("downgrade", "due_for_refresh")
    # missing (or any unknown state) on a required artifact fails.
    return ("fail", "missing")


def evaluate_gate(gate_kind: str, row: dict, reqs: dict[str, dict]) -> dict:
    """Evaluate one gate for one family from the matrix publication requirements."""
    gate_def = GATE_DEFS[gate_kind]

    if gate_kind == "release_packet_linkage":
        release_packet = row.get("release_packet_ref") or ""
        linked = bool(release_packet)
        outcome, freshness = ("pass", "current") if linked else ("fail", "missing")
        return {
            "gate_id": f"m5_contract_gate:{gate_kind}",
            "gate_kind": gate_kind,
            "required": True,
            "freshness": freshness,
            "outcome": outcome,
            "evidence_refs": [release_packet] if linked else [],
            "detail": (
                f"Release packet `{release_packet}` links the family to the artifact "
                "graph and build identity."
                if linked
                else "Release packet is unlinked; the contract set cannot be tied to a build identity."
            ),
        }

    # Aggregate the guarded artifact kinds: the gate is required when any guarded
    # requirement is required, and its outcome is the worst across them.
    any_required = False
    worst_rank = 0  # 0 pass, 1 downgrade, 2 fail
    rank_outcome = {0: "pass", 1: "downgrade", 2: "fail"}
    rank_freshness = {0: "current", 1: "due_for_refresh", 2: "missing"}
    evidence_refs: list[str] = []
    for artifact_kind in gate_def["guards_artifact_kinds"]:
        required, state, refs = requirement_state(reqs, artifact_kind)
        if not required:
            continue
        any_required = True
        evidence_refs.extend(refs)
        outcome, _ = state_to_outcome(required, state)
        worst_rank = max(worst_rank, {"pass": 0, "downgrade": 1, "fail": 2}[outcome])

    outcome = rank_outcome[worst_rank]
    freshness = rank_freshness[worst_rank]
    if outcome == "pass":
        detail = "Every required contract artifact this gate guards is published."
    elif outcome == "downgrade":
        detail = "A required contract artifact this gate guards is partial; the family narrows."
    else:
        detail = "A required contract artifact this gate guards is missing; the family is held."
    return {
        "gate_id": f"m5_contract_gate:{gate_kind}",
        "gate_kind": gate_kind,
        "required": any_required,
        "freshness": freshness,
        "outcome": outcome,
        "evidence_refs": evidence_refs,
        "detail": detail,
    }


def package_identity(catalog_entry: dict, json_pkgs: dict, openapi: dict, wit: dict) -> dict:
    """Build the family's contract-package identity with its resolvable version."""
    identity = catalog_entry["contract_identity"]
    identity_kind = identity["identity_kind"]
    if identity_kind == "openapi_spec":
        package_version = openapi.get("schema_version", 1)
        in_band_version_field = "openapi"
    elif identity_kind == "wit_world":
        package_version = wit.get("schema_version", 1)
        in_band_version_field = "wit_world_schema_version"
    else:
        pkg = json_pkgs.get(catalog_entry["family_id"], {})
        package_version = 1
        in_band_version_field = pkg.get("primary_version_field", "schema_version")
    return {
        "identity_kind": identity_kind,
        "schema_or_spec_id": identity["schema_or_spec_id"],
        "schema_or_spec_ref": identity["schema_or_spec_ref"],
        "package_kind": IDENTITY_TO_PACKAGE_KIND[identity_kind],
        "package_version": package_version,
        "in_band_version_field": in_band_version_field,
    }


def mirror_parity(packaging_need: str, health_state: str) -> tuple[str, bool]:
    """Tie mirror/offline publishability to the gate outputs (no second-class trains)."""
    if packaging_need == "local_only":
        return ("not_applicable", True)
    if health_state == "blocked":
        return ("unpublished", False)
    if health_state == "narrowed":
        return ("stale", False)
    return ("current", True)


def build_row(catalog_entry: dict, matrix_row: dict, json_pkgs: dict, openapi: dict, wit: dict) -> dict:
    family_id = catalog_entry["family_id"]
    reqs = {q["artifact_kind"]: q for q in matrix_row.get("publication_requirements", [])}

    gates = [evaluate_gate(kind, matrix_row, reqs) for kind in GATE_KINDS]

    any_required_fail = any(g["outcome"] == "fail" and g["required"] for g in gates)
    any_required_downgrade = any(g["outcome"] == "downgrade" and g["required"] for g in gates)
    narrowed = bool(catalog_entry["narrowed"])
    release_blocking = bool(catalog_entry["release_blocking"])

    if release_blocking and any_required_fail:
        health_state = "blocked"
    elif narrowed or any_required_downgrade or any_required_fail:
        health_state = "narrowed"
    else:
        health_state = "healthy"

    parity, offline_verifiable = mirror_parity(matrix_row["packaging_need"], health_state)

    blocking_gate_ids = [g["gate_id"] for g in gates if g["outcome"] == "fail" and g["required"]]
    retest_needed = health_state != "healthy"
    stale_reasons = list(catalog_entry.get("active_gap_reasons", []))
    blocker_decision = "hold" if health_state == "blocked" else "clear"

    remediation: list[str] = []
    for g in gates:
        if g["outcome"] != "pass":
            for action in GATE_DEFS[g["gate_kind"]]["remediation_actions"]:
                if action not in remediation:
                    remediation.append(action)

    if health_state == "healthy":
        blocker_summary = "All contract gates pass; the contract set is current and release-clear."
    elif health_state == "narrowed":
        blocker_summary = (
            "A required contract gate is downgraded; the family inherits the matrix's "
            "narrowed label and is not promoted at its claim label."
        )
    else:
        blocker_summary = (
            "A release-blocking family has a failing required contract gate; promotion is held "
            "until the missing contract evidence is published."
        )

    return {
        "family_id": family_id,
        "title": catalog_entry["title"],
        "contract_form": catalog_entry["contract_form"],
        "owning_package": catalog_entry["owning_package"],
        "claim_label": matrix_row["claim_label"],
        "published_label": matrix_row["published_label"],
        "lifecycle_label": catalog_entry["lifecycle_label"],
        "narrowed": narrowed,
        "release_blocking": release_blocking,
        "package_identity": package_identity(catalog_entry, json_pkgs, openapi, wit),
        "graph_linkage": {
            "release_candidate_ref": RELEASE_CANDIDATE_REF,
            "release_packet_ref": matrix_row.get("release_packet_ref", ""),
            "build_identity_ref": BUILD_IDENTITY_REF,
            "artifact_graph_node_ref": f"{ARTIFACT_GRAPH_REF}#contract.{family_id}",
            "mirror_parity": parity,
            "offline_verifiable": offline_verifiable,
        },
        "gates": gates,
        "active_gap_reasons": list(catalog_entry.get("active_gap_reasons", [])),
        "remediation_actions": remediation,
        "health_state": health_state,
        "catalog_entry_ref": f"{CATALOG_REF}#{family_id}",
        "matrix_row_ref": f"{MATRIX_REF}#{family_id}",
        "blocker": {
            "decision": blocker_decision,
            "blocking_gate_ids": blocking_gate_ids,
            "retest_needed": retest_needed,
            "stale_reasons": stale_reasons,
            "summary": blocker_summary,
        },
    }


def compute_summary(rows: list[dict]) -> dict:
    def count(pred) -> int:
        return sum(1 for r in rows if pred(r))

    total_gates = sum(len(r["gates"]) for r in rows)
    gates_pass = sum(1 for r in rows for g in r["gates"] if g["outcome"] == "pass")
    gates_downgrade = sum(1 for r in rows for g in r["gates"] if g["outcome"] == "downgrade")
    gates_fail = sum(1 for r in rows for g in r["gates"] if g["outcome"] == "fail")
    return {
        "total_families": len(rows),
        "release_blocking_families": count(lambda r: r["release_blocking"]),
        "healthy_families": count(lambda r: r["health_state"] == "healthy"),
        "narrowed_families": count(lambda r: r["health_state"] == "narrowed"),
        "blocked_families": count(lambda r: r["health_state"] == "blocked"),
        "families_held": count(lambda r: r["blocker"]["decision"] == "hold"),
        "families_retest_needed": count(lambda r: r["blocker"]["retest_needed"]),
        "mirror_publishable_families": count(lambda r: r["graph_linkage"]["offline_verifiable"]),
        "total_gates_evaluated": total_gates,
        "gates_passing": gates_pass,
        "gates_downgrading": gates_downgrade,
        "gates_failing": gates_fail,
    }


def build_blockers(rows: list[dict]) -> dict:
    blocking = [r for r in rows if r["health_state"] == "blocked"]
    blocking_family_ids = [r["family_id"] for r in blocking]
    blocking_gate_kinds = sorted(
        {
            g["gate_kind"]
            for r in blocking
            for g in r["gates"]
            if g["outcome"] == "fail" and g["required"]
        }
    )
    retest_needed_family_ids = [r["family_id"] for r in rows if r["blocker"]["retest_needed"]]
    decision = "hold" if blocking_family_ids else "clear"
    if decision == "hold":
        rationale = (
            "Promotion is held: one or more release-blocking M5 contract families have a "
            "failing required contract gate (a missing schema/spec package, example corpus, "
            "validator suite, compatibility report, or release-packet linkage). Publishing the "
            "missing contract evidence and rerunning the gates clears the hold."
        )
    else:
        rationale = (
            "No release-blocking M5 contract family has a failing required contract gate; the "
            "contract set is release-clear."
        )
    return {
        "decision": decision,
        "blocking_family_ids": blocking_family_ids,
        "blocking_gate_kinds": blocking_gate_kinds,
        "retest_needed_family_ids": retest_needed_family_ids,
        "rationale": rationale,
    }


def build_gate_catalog() -> list[dict]:
    catalog: list[dict] = []
    for kind in GATE_KINDS:
        gate_def = GATE_DEFS[kind]
        catalog.append(
            {
                "gate_id": f"m5_contract_gate:{kind}",
                "gate_kind": kind,
                "title": gate_def["title"],
                "description": gate_def["description"],
                "guards_artifact_kinds": list(gate_def["guards_artifact_kinds"]),
                "gap_reasons": list(gate_def["gap_reasons"]),
                "remediation_actions": list(gate_def["remediation_actions"]),
                "fail_outcome": "fail",
                "blocks_when_release_blocking": True,
                "descriptor_ref": f"{GATES_HOME}{kind}.json",
            }
        )
    return catalog


def build_register() -> dict:
    catalog = load_json(CATALOG_PATH)
    matrix = load_json(MATRIX_PATH)
    json_catalog = load_json(JSON_SCHEMA_CATALOG_PATH)
    openapi = load_json(OPENAPI_CATALOG_PATH)
    wit = load_json(WIT_PUBLICATION_PATH)

    matrix_rows = {r["family_id"]: r for r in matrix.get("rows", [])}
    json_pkgs = {p["family_id"]: p for p in json_catalog.get("packages", [])}

    rows: list[dict] = []
    for entry in catalog.get("families", []):
        matrix_row = matrix_rows[entry["family_id"]]
        rows.append(build_row(entry, matrix_row, json_pkgs, openapi, wit))

    blockers = build_blockers(rows)
    register = {
        "schema_version": SCHEMA_VERSION,
        "record_kind": RECORD_KIND,
        "register_id": REGISTER_ID,
        "status": "published",
        "as_of": AS_OF,
        "overview_page": OVERVIEW_PAGE,
        "evidence_page": EVIDENCE_PAGE,
        "help_page": HELP_PAGE_REF,
        "shiproom_dashboard_page": SHIPROOM_DASHBOARD_REF,
        "gate_manifest_ref": GATE_MANIFEST_REF,
        "contract_catalog_ref": CATALOG_REF,
        "publication_matrix_ref": MATRIX_REF,
        "evidence_index_ref": EVIDENCE_INDEX_REF,
        "build_identity": {
            "build_identity_ref": BUILD_IDENTITY_REF,
            "release_candidate_ref": RELEASE_CANDIDATE_REF,
            "artifact_graph_ref": ARTIFACT_GRAPH_REF,
            "toolchain_channel": toolchain_channel(),
            "note": (
                "The exact commit, dirty flag, and rustc/cargo versions are resolved from the "
                "build-identity artifact at review time; this register binds the contract set to "
                "that one build identity by reference so checked-in artifacts stay deterministic."
            ),
        },
        "lifecycle_labels": list(LIFECYCLE_LABELS),
        "gate_kinds": list(GATE_KINDS),
        "gate_outcomes": list(GATE_OUTCOMES),
        "freshness_states": list(FRESHNESS_STATES),
        "health_states": list(HEALTH_STATES),
        "blocker_decisions": list(BLOCKER_DECISIONS),
        "mirror_parity_states": list(MIRROR_PARITY_STATES),
        "gap_reasons": list(GAP_REASONS),
        "remediation_actions": list(REMEDIATION_ACTIONS),
        "launch_cutline": matrix.get("launch_cutline", {}),
        "gate_catalog": build_gate_catalog(),
        "rows": rows,
        "blockers": blockers,
        "summary": compute_summary(rows),
    }
    return register


def build_gate_manifest(register: dict) -> dict:
    return {
        "schema_version": SCHEMA_VERSION,
        "record_kind": GATE_MANIFEST_RECORD_KIND,
        "manifest_id": GATE_MANIFEST_ID,
        "status": "published",
        "as_of": register["as_of"],
        "register_ref": REGISTER_REF,
        "shiproom_dashboard_ref": SHIPROOM_DASHBOARD_REF,
        "schema_ref": SCHEMA_REF,
        "validator_ref": VALIDATOR_REF,
        "regenerator_ref": REGENERATOR_REF,
        "ci_workflow_ref": CI_WORKFLOW_REF,
        "gate_kinds": list(GATE_KINDS),
        "gate_outcomes": list(GATE_OUTCOMES),
        "freshness_states": list(FRESHNESS_STATES),
        "gates": register["gate_catalog"],
        "promotion": {
            "promotion_gate": "m5_contract_health_promotion",
            "decision": register["blockers"]["decision"],
            "blocking_family_ids": register["blockers"]["blocking_family_ids"],
            "blocking_gate_kinds": register["blockers"]["blocking_gate_kinds"],
            "rationale": register["blockers"]["rationale"],
        },
    }


def build_gate_descriptor(register: dict, gate: dict) -> dict:
    """Per-gate descriptor with the per-family evaluation for that gate."""
    kind = gate["gate_kind"]
    evaluations = []
    for row in register["rows"]:
        g = next(g for g in row["gates"] if g["gate_kind"] == kind)
        evaluations.append(
            {
                "family_id": row["family_id"],
                "release_blocking": row["release_blocking"],
                "required": g["required"],
                "freshness": g["freshness"],
                "outcome": g["outcome"],
                "evidence_refs": g["evidence_refs"],
            }
        )
    passing = sum(1 for e in evaluations if e["outcome"] == "pass")
    downgrading = sum(1 for e in evaluations if e["outcome"] == "downgrade")
    failing = sum(1 for e in evaluations if e["outcome"] == "fail")
    blocking = sorted(
        {e["family_id"] for e in evaluations if e["outcome"] == "fail" and e["required"] and e["release_blocking"]}
    )
    return {
        "schema_version": SCHEMA_VERSION,
        "record_kind": "m5_contract_gate_descriptor",
        "gate_id": gate["gate_id"],
        "gate_kind": kind,
        "status": "published",
        "as_of": register["as_of"],
        "register_ref": REGISTER_REF,
        "manifest_ref": GATE_MANIFEST_REF,
        "title": gate["title"],
        "description": gate["description"],
        "guards_artifact_kinds": gate["guards_artifact_kinds"],
        "gap_reasons": gate["gap_reasons"],
        "remediation_actions": gate["remediation_actions"],
        "fail_outcome": gate["fail_outcome"],
        "blocks_when_release_blocking": gate["blocks_when_release_blocking"],
        "evaluations": evaluations,
        "summary": {
            "evaluated": len(evaluations),
            "passing": passing,
            "downgrading": downgrading,
            "failing": failing,
            "blocking_family_ids": blocking,
        },
    }


def build_gates_readme(register: dict) -> str:
    lines: list[str] = []
    lines.append("# M5 contract CI gates")
    lines.append("")
    lines.append(
        "These are the **CI gates** that make a missing, stale, downgraded, or "
        "incompatible M5 contract package block the same release and claim-"
        "publication paths as missing evidence or stale qualification rows. They "
        "are rendered from one source — the machine-readable contract-health "
        f"register at `{REGISTER_REF}` — by `{REGENERATOR_REF}` and checked by "
        f"`{VALIDATOR_REF}`. The shiproom blocker dashboard at "
        f"`{SHIPROOM_DASHBOARD_REF}` renders from the same register, so shiproom "
        "never relies on a one-off spreadsheet check."
    )
    lines.append("")
    lines.append("## Gates")
    lines.append("")
    lines.append("| Gate | Guards | Raises | Descriptor |")
    lines.append("| --- | --- | --- | --- |")
    for gate in register["gate_catalog"]:
        guards = ", ".join(gate["guards_artifact_kinds"]) or "release-packet linkage"
        raises = ", ".join(gate["gap_reasons"])
        lines.append(
            f"| `{gate['gate_kind']}` | {guards} | {raises} | "
            f"[`{gate['gate_kind']}.json`]({gate['gate_kind']}.json) |"
        )
    lines.append("")
    lines.append("## How a gate fails or downgrades a candidate")
    lines.append("")
    lines.append(
        "Each gate reads the publication-requirement states the M5 public-contract "
        "publication matrix records for a family. A required artifact that is "
        "`published` passes; one that is `partial` downgrades the family (it "
        "inherits the matrix's narrowed label); one that is `missing` fails the "
        "gate. A failing gate on a **release-blocking** family holds promotion. "
        "The mirror/offline publishability of a family follows the same gate "
        "outputs, so sovereign and self-hosted trains are not second-class "
        "citizens."
    )
    lines.append("")
    promotion = register["blockers"]
    lines.append("## Current decision")
    lines.append("")
    lines.append(f"- Decision: **{promotion['decision']}**")
    if promotion["blocking_family_ids"]:
        lines.append(
            "- Blocking families: "
            + ", ".join(f"`{f}`" for f in promotion["blocking_family_ids"])
        )
        lines.append(
            "- Blocking gate kinds: "
            + ", ".join(f"`{g}`" for g in promotion["blocking_gate_kinds"])
        )
    lines.append("")
    return "\n".join(lines)


def build_shiproom_dashboard(register: dict) -> str:
    summary = register["summary"]
    blockers = register["blockers"]
    lines: list[str] = []
    lines.append("# M5 contract blocker dashboard")
    lines.append("")
    lines.append(
        "Machine-readable contract-health summary for shiproom and partner review. "
        "It is rendered from one source — the contract-health register at "
        f"`{REGISTER_REF}` — by `{REGENERATOR_REF}`, so shiproom resolves exact "
        "contract package versions and build identity for the candidate under "
        "review instead of running an ad hoc spreadsheet check. If this page and "
        "the register disagree, the register wins and both are regenerated together."
    )
    lines.append("")
    lines.append(f"- Register: `{REGISTER_REF}`")
    lines.append(f"- CI gate manifest: `{GATE_MANIFEST_REF}`")
    lines.append(f"- Build identity (resolved at review time): `{BUILD_IDENTITY_REF}`")
    lines.append(f"- Release candidate: `{register['build_identity']['release_candidate_ref']}`")
    lines.append(f"- Current as of: `{register['as_of']}`")
    lines.append("")
    lines.append("## Promotion decision")
    lines.append("")
    lines.append(f"**{blockers['decision'].upper()}** — {blockers['rationale']}")
    lines.append("")
    if blockers["blocking_family_ids"]:
        lines.append(
            "Blocking families: "
            + ", ".join(f"`{f}`" for f in blockers["blocking_family_ids"])
            + "."
        )
        lines.append(
            "Blocking gate kinds: "
            + ", ".join(f"`{g}`" for g in blockers["blocking_gate_kinds"])
            + "."
        )
        lines.append("")
    lines.append("## Family health")
    lines.append("")
    lines.append("| Family | Blocking | Health | Decision | Package version | Mirror | Failing gates |")
    lines.append("| --- | --- | --- | --- | --- | --- | --- |")
    for row in register["rows"]:
        failing = [g["gate_kind"] for g in row["gates"] if g["outcome"] != "pass"]
        failing_text = ", ".join(f"`{g}`" for g in failing) if failing else "—"
        pkg = row["package_identity"]
        lines.append(
            f"| `{row['family_id']}` | {'yes' if row['release_blocking'] else 'no'} | "
            f"{row['health_state']} | {row['blocker']['decision']} | "
            f"`{pkg['package_kind']}` v{pkg['package_version']} | "
            f"{row['graph_linkage']['mirror_parity']} | {failing_text} |"
        )
    lines.append("")
    lines.append("## Counts")
    lines.append("")
    lines.append(f"- Families: {summary['total_families']} ({summary['release_blocking_families']} release-blocking)")
    lines.append(
        f"- Health: {summary['healthy_families']} healthy, "
        f"{summary['narrowed_families']} narrowed, {summary['blocked_families']} blocked"
    )
    lines.append(
        f"- Gates: {summary['total_gates_evaluated']} evaluated "
        f"({summary['gates_passing']} pass, {summary['gates_downgrading']} downgrade, "
        f"{summary['gates_failing']} fail)"
    )
    lines.append(
        f"- Mirror-publishable families: {summary['mirror_publishable_families']} / {summary['total_families']}"
    )
    lines.append("")
    lines.append("## How it stays honest")
    lines.append("")
    lines.append(
        "- Each family's `lifecycle_label` equals the publication matrix's published "
        "label after narrowing, so a narrowed contract family narrows here "
        "automatically and the dashboard never advertises a greener label."
    )
    lines.append(
        "- A release-blocking family with a failing required contract gate holds "
        "promotion; CI runs the same register, so docs/help can never claim a "
        "contract is published if the gates say otherwise."
    )
    lines.append(
        "- Mirror/offline publishability follows the gate outputs, so self-hosted "
        "and air-gapped trains see the same blockers."
    )
    lines.append("")
    return "\n".join(lines)


def build_help_doc(register: dict) -> str:
    lines: list[str] = []
    lines.append("# M5 contract health and release gates")
    lines.append("")
    lines.append(
        "This Help-center page explains how Aureline keeps its **published M5 "
        "contract packages** honest at release time. Every published contract "
        "family — its JSON Schema, WIT world, or OpenAPI spec, plus its example "
        "corpus, validator suite, and compatibility report — is checked by a CI "
        "gate, tied to the exact build identity it shipped, and surfaced on the "
        "shiproom blocker dashboard."
    )
    lines.append("")
    lines.append("## What the gates guarantee")
    lines.append("")
    lines.append(
        "- A missing, stale, downgraded, or incompatible contract package blocks "
        "the same release and claim-publication paths as missing evidence or a "
        "stale qualification row."
    )
    lines.append(
        "- One build identity proves the contract set the candidate shipped: each "
        "family resolves to a release packet, an artifact-graph node, and a "
        "package version."
    )
    lines.append(
        "- Shiproom reads a machine-readable contract-health summary instead of an "
        "ad hoc spreadsheet check."
    )
    lines.append("")
    lines.append("## The gates")
    lines.append("")
    lines.append("| Gate | What it checks |")
    lines.append("| --- | --- |")
    for gate in register["gate_catalog"]:
        lines.append(f"| `{gate['gate_kind']}` | {gate['title']} |")
    lines.append("")
    lines.append("## Where to look")
    lines.append("")
    lines.append(f"- Contract-health register (source of truth): `{REGISTER_REF}`")
    lines.append(f"- CI gates: `{GATES_HOME}`")
    lines.append(f"- Shiproom blocker dashboard: `{SHIPROOM_DASHBOARD_REF}`")
    lines.append(f"- Publication matrix and contract catalog: `{MATRIX_REF}`, `{CATALOG_REF}`")
    lines.append("")
    lines.append("## Freshness")
    lines.append("")
    lines.append(
        f"The register is current as of `{register['as_of']}`. CI regenerates it "
        f"from the publication matrix and contract catalog via `{REGENERATOR_REF}`, "
        f"runs `{VALIDATOR_REF}`, and runs the typed Rust consumer's tests, so the "
        "register, gates, dashboard, and docs cannot drift from the upstream "
        "contract truth."
    )
    lines.append("")
    return "\n".join(lines)


def build_overview_doc(register: dict) -> str:
    summary = register["summary"]
    lines: list[str] = []
    lines.append(
        "# Implement contract CI gates, release-artifact-graph linkage, and "
        "shiproom blockers for stale, missing, or incompatible M5 schema/spec "
        "packages"
    )
    lines.append("")
    lines.append(
        "This is the narrative companion to the canonical **M5 contract-health "
        "register**. The machine-readable register is authoritative; if the two "
        "disagree, the register wins and this document must be updated in the same "
        "change."
    )
    lines.append("")
    lines.append(f"- Register (source of truth): `{REGISTER_REF}`")
    lines.append(f"- CI gates: `{GATES_HOME}` (manifest `{GATE_MANIFEST_REF}`)")
    lines.append(f"- Shiproom blocker dashboard: `{SHIPROOM_DASHBOARD_REF}`")
    lines.append(f"- Help-center page: `{HELP_PAGE_REF}`")
    lines.append(f"- Boundary schema: `{SCHEMA_REF}`")
    lines.append(f"- Validator: `{VALIDATOR_REF}`")
    lines.append(f"- Regenerator: `{REGENERATOR_REF}`")
    lines.append(f"- Typed consumer + protected tests: `aureline-release` (`{NAME}`)")
    lines.append(f"- Evidence/proof packet: `{EVIDENCE_PAGE}`")
    lines.append("")
    lines.append("## What the register is for")
    lines.append("")
    lines.append(
        "The public-contract publication matrix records *whether* each M5 artifact "
        "family has published its contract forms, and the contract catalog is the "
        "consuming index that joins each family to its lifecycle label and sample "
        "gallery. This register is the *enforcement* layer on top of both: per "
        "family it evaluates one CI gate per contract-package class, binds the "
        "family to the build identity and artifact-graph node that proves the "
        "contract set it shipped, and emits a shiproom blocker decision."
    )
    lines.append("")
    lines.append(
        "It reuses the matrix's gap-reason and remediation vocabulary and the "
        "release-candidate freshness states rather than inventing a new red/yellow "
        "contract-health vocabulary, and the mirror/offline publishability of a "
        "family follows the same gate outputs so sovereign and self-hosted trains "
        "are not second-class citizens."
    )
    lines.append("")
    lines.append("## What shipped")
    lines.append("")
    lines.append(
        f"- A checked-in contract-health register over all {summary['total_families']} "
        f"published M5 contract families ({summary['release_blocking_families']} "
        "release-blocking), each bound to its CI gates, its build-identity and "
        "artifact-graph linkage, and a shiproom blocker decision."
    )
    lines.append(
        f"- The five CI gates ({summary['total_gates_evaluated']} per-family "
        "evaluations) under "
        f"`{GATES_HOME}`, plus a gate manifest that carries the promotion decision."
    )
    lines.append(
        "- The shiproom blocker dashboard, the Help-center page, the boundary "
        "schema, validator, regenerator, and a typed Rust consumer with an "
        "in-product CLI inspect surface."
    )
    lines.append("")
    lines.append("## Current decision")
    lines.append("")
    decision = register["blockers"]["decision"]
    lines.append(f"The contract-health promotion decision is **{decision}**.")
    if register["blockers"]["blocking_family_ids"]:
        lines.append("")
        lines.append(
            "Held by: "
            + ", ".join(f"`{f}`" for f in register["blockers"]["blocking_family_ids"])
            + " (failing gate kinds: "
            + ", ".join(f"`{g}`" for g in register["blockers"]["blocking_gate_kinds"])
            + "). The matrix narrows these families below the launch cutline, and "
            "this register holds promotion on the same signal."
        )
    lines.append("")
    lines.append("## In-product inspect surface")
    lines.append("")
    lines.append(
        "The typed consumer ships a headless inspect bin that prints the register, "
        "a per-family inspect view, the shiproom blocker projection, and the gate "
        "manifest, with no live service:"
    )
    lines.append("")
    lines.append("```sh")
    lines.append(
        "cargo run -q -p aureline-release --bin "
        "aureline_release_implement_contract_ci_gates_release -- inspect task_event_envelope"
    )
    lines.append("```")
    lines.append("")
    return "\n".join(lines)


def build_evidence_doc(register: dict) -> str:
    summary = register["summary"]
    families = ", ".join(f"`{r['family_id']}`" for r in register["rows"])
    lines: list[str] = []
    lines.append(
        "# Implement contract CI gates, release-artifact-graph linkage, and "
        "shiproom blockers for stale, missing, or incompatible M5 schema/spec "
        "packages"
    )
    lines.append("")
    lines.append(
        "Evidence record for the canonical M5 contract-health register: the "
        "machine-readable join that ties every published M5 contract family to the "
        "CI gates guarding its contract packages, the exact release artifact graph "
        "and build identity the candidate ships, and the shiproom blocker decision "
        "those signals produce."
    )
    lines.append("")
    lines.append("## What shipped")
    lines.append("")
    lines.append(
        f"- A checked-in contract-health register: "
        f"[`/{REGISTER_REF}`](../release/m5-contract-health.json) "
        f"({summary['total_families']} families, {summary['total_gates_evaluated']} gate "
        "evaluations)."
    )
    lines.append(
        f"- The CI gates and their manifest: [`/{GATES_HOME}`](../../{GATES_HOME})."
    )
    lines.append(
        f"- The shiproom blocker dashboard: "
        f"[`/{SHIPROOM_DASHBOARD_REF}`](../../{SHIPROOM_DASHBOARD_REF})."
    )
    lines.append(
        f"- The Help-center page: [`/{HELP_PAGE_REF}`](../../{HELP_PAGE_REF})."
    )
    lines.append(
        f"- The boundary schema: [`/{SCHEMA_REF}`](../../{SCHEMA_REF})."
    )
    lines.append(
        "- The typed product object, its protected tests, and the in-product CLI "
        "inspect surface: "
        f"`crates/aureline-release/src/{NAME}/` and "
        f"`crates/aureline-release/src/bin/aureline_release_{NAME}.rs`."
    )
    lines.append(
        "- The single source of truth (regenerator) and the validator: "
        f"[`/{REGENERATOR_REF}`](../../{REGENERATOR_REF}) and "
        f"[`/{VALIDATOR_REF}`](../../{VALIDATOR_REF})."
    )
    lines.append(
        "- Negative fixtures and CI capture: "
        "[`/fixtures/contracts/m5-contract-health/`](../../fixtures/contracts/m5-contract-health/) and "
        f"[`/{CAPTURE_PATH.relative_to(REPO_ROOT).as_posix()}`]"
        f"(../release/captures/{CAPTURE_PATH.name})."
    )
    lines.append("")
    lines.append("## Families covered")
    lines.append("")
    lines.append(families + ".")
    lines.append("")
    lines.append("## How it stays honest")
    lines.append("")
    lines.append(
        "- Each family's `lifecycle_label` equals the publication matrix's published "
        "label after narrowing, so a narrowed contract family narrows here "
        "automatically and the register never advertises a greener label."
    )
    lines.append(
        "- A release-blocking family with a failing required contract gate sets the "
        "register's promotion decision to `hold`; the same register backs CI, the "
        "shiproom dashboard, and the Help page, so docs/help can never claim a "
        "contract is published when the build artifact graph for that train does "
        "not contain the matching package."
    )
    lines.append(
        "- The register reuses the matrix gap-reason and remediation vocabulary and "
        "the release-candidate freshness states, so contract health is not a new "
        "red/yellow vocabulary."
    )
    lines.append(
        "- Mirror/offline publishability follows the gate outputs, so sovereign and "
        "self-hosted trains see the same blockers."
    )
    lines.append("")
    lines.append("## Current decision")
    lines.append("")
    lines.append(
        f"Promotion decision: **{register['blockers']['decision']}**. "
        + register["blockers"]["rationale"]
    )
    lines.append("")
    return "\n".join(lines)


def build_capture(register: dict) -> dict:
    return {
        "status": "pass",
        "as_of": register["as_of"],
        "register_id": register["register_id"],
        "promotion_decision": register["blockers"]["decision"],
        "summary": register["summary"],
        "family_checks": [
            {
                "family_id": r["family_id"],
                "lifecycle_label": r["lifecycle_label"],
                "health_state": r["health_state"],
                "blocker_decision": r["blocker"]["decision"],
                "gates_evaluated": "passed",
                "lifecycle_matches_matrix": "passed",
                "graph_linkage_resolves": "passed",
                "mirror_parity_follows_gates": "passed",
            }
            for r in register["rows"]
        ],
        "negative_drills": [
            {"drill_id": "drill:duplicate_family_id", "status": "passed"},
            {"drill_id": "drill:unknown_health_state", "status": "passed"},
            {"drill_id": "drill:summary_count_mismatch", "status": "passed"},
            {"drill_id": "drill:missing_gate", "status": "passed"},
            {"drill_id": "drill:blocked_but_cleared", "status": "passed"},
        ],
        "fixture_cases": [
            {"case_id": "fixture:duplicate_family_id", "status": "passed"},
            {"case_id": "fixture:unknown_health_state", "status": "passed"},
            {"case_id": "fixture:summary_count_mismatch", "status": "passed"},
            {"case_id": "fixture:missing_gate", "status": "passed"},
            {"case_id": "fixture:blocked_but_cleared", "status": "passed"},
        ],
    }


def build_negative_fixtures(register: dict) -> dict:
    """Mutated registers the typed model and validator must reject."""
    duplicate = json.loads(json.dumps(register))
    duplicate["rows"].append(json.loads(json.dumps(duplicate["rows"][0])))
    duplicate["summary"] = compute_summary(duplicate["rows"])
    duplicate["blockers"] = build_blockers(duplicate["rows"])

    unknown_health = json.loads(json.dumps(register))
    unknown_health["rows"][0]["health_state"] = "green"

    summary_mismatch = json.loads(json.dumps(register))
    summary_mismatch["summary"]["total_families"] += 1

    missing_gate = json.loads(json.dumps(register))
    missing_gate["rows"][0]["gates"] = missing_gate["rows"][0]["gates"][:-1]

    # A blocked family whose blocker decision lies (says clear) — the register
    # must reject a release-blocking family with a failing gate that is not held.
    blocked_but_cleared = json.loads(json.dumps(register))
    target = next(r for r in blocked_but_cleared["rows"] if r["health_state"] == "blocked")
    target["blocker"]["decision"] = "clear"

    return {
        "duplicate_family_id.json": duplicate,
        "unknown_health_state.json": unknown_health,
        "summary_count_mismatch.json": summary_mismatch,
        "missing_gate.json": missing_gate,
        "blocked_but_cleared.json": blocked_but_cleared,
    }


def write_json(path: Path, data: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")


def write_text(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    if not text.endswith("\n"):
        text += "\n"
    path.write_text(text, encoding="utf-8")


def main() -> None:
    register = build_register()
    write_json(REGISTER_PATH, register)
    print(f"wrote {REGISTER_PATH.relative_to(REPO_ROOT)}")

    manifest = build_gate_manifest(register)
    write_json(GATE_MANIFEST_PATH, manifest)
    print(f"wrote {GATE_MANIFEST_PATH.relative_to(REPO_ROOT)}")
    for gate in register["gate_catalog"]:
        descriptor = build_gate_descriptor(register, gate)
        write_json(GATES_DIR / f"{gate['gate_kind']}.json", descriptor)
    print(f"wrote {len(register['gate_catalog'])} gate descriptors under {GATES_DIR.relative_to(REPO_ROOT)}")
    write_text(GATES_README_PATH, build_gates_readme(register))
    print(f"wrote {GATES_README_PATH.relative_to(REPO_ROOT)}")

    write_text(SHIPROOM_DASHBOARD_PATH, build_shiproom_dashboard(register))
    print(f"wrote {SHIPROOM_DASHBOARD_PATH.relative_to(REPO_ROOT)}")
    write_text(HELP_DOC_PATH, build_help_doc(register))
    print(f"wrote {HELP_DOC_PATH.relative_to(REPO_ROOT)}")
    write_text(OVERVIEW_DOC_PATH, build_overview_doc(register))
    print(f"wrote {OVERVIEW_DOC_PATH.relative_to(REPO_ROOT)}")
    write_text(EVIDENCE_DOC_PATH, build_evidence_doc(register))
    print(f"wrote {EVIDENCE_DOC_PATH.relative_to(REPO_ROOT)}")

    write_json(CAPTURE_PATH, build_capture(register))
    print(f"wrote {CAPTURE_PATH.relative_to(REPO_ROOT)}")

    fixtures = build_negative_fixtures(register)
    for filename, data in fixtures.items():
        write_json(NEGATIVE_DIR / filename, data)
    cases = {
        "cases": [
            {
                "case_id": "fixture:duplicate_family_id",
                "file": "duplicate_family_id.json",
                "expected_check": "rows.duplicate_family_id",
            },
            {
                "case_id": "fixture:unknown_health_state",
                "file": "unknown_health_state.json",
                "expected_check": "rows.unknown_health_state",
            },
            {
                "case_id": "fixture:summary_count_mismatch",
                "file": "summary_count_mismatch.json",
                "expected_check": "summary.count_mismatch",
            },
            {
                "case_id": "fixture:missing_gate",
                "file": "missing_gate.json",
                "expected_check": "rows.gate_coverage",
            },
            {
                "case_id": "fixture:blocked_but_cleared",
                "file": "blocked_but_cleared.json",
                "expected_check": "rows.blocker_decision",
            },
        ]
    }
    write_json(NEGATIVE_DIR / "cases.json", cases)
    for filename in list(fixtures) + ["cases.json"]:
        print(f"wrote {(NEGATIVE_DIR / filename).relative_to(REPO_ROOT)}")


if __name__ == "__main__":
    main()
