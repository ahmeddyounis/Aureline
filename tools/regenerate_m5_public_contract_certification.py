#!/usr/bin/env python3
"""Regenerate the M5 public-contract certification register, its human-readable report,
the shiproom certification dashboard, the Help/overview/evidence docs, the CI capture, and
the negative fixtures that prove it.

This is the closeout row for the whole M5 public-contract publication lane. Earlier rows
publish the individual contract forms (the JSON Schema catalog, the OpenAPI catalog, the
WIT publication, the reader/writer compatibility suite, the interchange-conformance
register), the publication matrix that records *whether* each family published its required
forms, and the contract-health register that *enforces* those forms with CI gates and a
release-graph linkage. This register is the certification join above all of them: for every
claimed M5 public artifact family it binds the published contract form, the lifecycle
metadata (version + label), the example corpus, the validator coverage, the compatibility
report, and the release-graph linkage into one certification packet, decides whether the
family may carry a certified contract claim or is narrowed/withheld, and emits one
exportable proof shiproom, support, docs, SDK, and partner review consume without restating
field semantics.

It never mints a new certification vocabulary: it reuses the contract-health register's
per-family gate evaluation and the publication matrix's lifecycle labels, narrowing a
marketed claim exactly when an upstream contract pillar is missing or stale. A family may
never certify a greener label than its public claim, and a release-blocking family with a
missing required contract pillar withholds certification and holds promotion.

It reads, as upstream truth:

  * ``artifacts/release/m5-contract-health.json``            (the contract-health register)
  * ``artifacts/contracts/m5-stability-lifecycle-map.json``  (the publication matrix)
  * ``artifacts/contracts/m5-contract-catalog.json``         (the contract catalog/galleries)
  * ``artifacts/contracts/m5-json-schema-catalog.json``      (JSON Schema packages)
  * ``artifacts/contracts/m5-openapi-catalog.json``          (OpenAPI catalog)
  * ``artifacts/contracts/m5-wit-contract-publication.json`` (WIT publication)
  * ``artifacts/contracts/m5-reader-writer-compat-suite.json`` (reader/writer compat suite)
  * ``artifacts/contracts/m5-interchange-conformance.json``  (interchange conformance)

It writes, all deterministically:

  * ``artifacts/certification/m5-public-contract-certification.json``  (the register)
  * ``artifacts/certification/m5-public-contract-certification.md``    (the report)
  * ``shiproom/m5-public-contract-certification-dashboard.md``         (shiproom dashboard)
  * ``docs/help/m5-public-contract-certification.md``                  (Help-center page)
  * ``docs/m5/<slug>.md``                                              (narrative companion)
  * ``artifacts/m5/<slug>.md``                                         (evidence/proof packet)
  * ``artifacts/release/captures/<name>_validation_capture.json``      (CI capture)
  * ``fixtures/contracts/m5-public-contract-certification/{cases.json,*.json}`` (negative fixtures)

Run ``python3 tools/regenerate_m5_public_contract_certification.py`` after editing this
script or any upstream contract artifact, then
``python3 tools/validate_m5_public_contract_certification.py`` and
``cargo test -p aureline-release --test rel_it_09_certify_schema_publication_wit``
to confirm the validator and the typed model agree.

The register is metadata-plus-state only: every field is a typed state, an opaque
repo-relative ref or URI, or a copy/export-safe summary. It carries no credential bodies or
raw provider payloads, and it never reads live, per-build values (the commit and dirty flag
are resolved from the build-identity artifact at review time) so the checked-in artifacts
stay deterministic.
"""

from __future__ import annotations

import json
from pathlib import Path

NAME = (
    "certify_schema_publication_wit_openapi_packaging_validator_coverage_and_"
    "compatibility_truth_on_every_claimed_m5_public_artifact_family"
)
RECORD_KIND = "m5_public_contract_certification"
REGISTER_ID = "m5_public_contract_certification:v1"
SCHEMA_VERSION = 1
AS_OF = "2026-06-19"

REPO_ROOT = Path(__file__).resolve().parent.parent

DOC_SLUG = (
    "certify-schema-publication-wit-openapi-packaging-validator-coverage-and-"
    "compatibility-truth-on-every-claimed-m5-public-artifact-family"
)

# Outputs.
REGISTER_PATH = REPO_ROOT / "artifacts" / "certification" / "m5-public-contract-certification.json"
REPORT_PATH = REPO_ROOT / "artifacts" / "certification" / "m5-public-contract-certification.md"
SHIPROOM_PATH = REPO_ROOT / "shiproom" / "m5-public-contract-certification-dashboard.md"
HELP_DOC_PATH = REPO_ROOT / "docs" / "help" / "m5-public-contract-certification.md"
OVERVIEW_DOC_PATH = REPO_ROOT / "docs" / "m5" / f"{DOC_SLUG}.md"
EVIDENCE_DOC_PATH = REPO_ROOT / "artifacts" / "m5" / f"{DOC_SLUG}.md"
CAPTURE_PATH = REPO_ROOT / "artifacts" / "release" / "captures" / f"{NAME}_validation_capture.json"
NEGATIVE_DIR = REPO_ROOT / "fixtures" / "contracts" / "m5-public-contract-certification"

# Output refs (repo-relative) the register and its companions cross-link.
REGISTER_REF = "artifacts/certification/m5-public-contract-certification.json"
REPORT_REF = "artifacts/certification/m5-public-contract-certification.md"
SHIPROOM_REF = "shiproom/m5-public-contract-certification-dashboard.md"
HELP_PAGE_REF = "docs/help/m5-public-contract-certification.md"
OVERVIEW_PAGE = f"docs/m5/{DOC_SLUG}.md"
EVIDENCE_PAGE = f"artifacts/m5/{DOC_SLUG}.md"
SCHEMA_REF = "schemas/public/m5-contracts/m5_public_contract_certification.schema.json"
VALIDATOR_REF = "tools/validate_m5_public_contract_certification.py"
REGENERATOR_REF = "tools/regenerate_m5_public_contract_certification.py"
CI_WORKFLOW_REF = ".github/workflows/check_m5_public_contract_certification.yml"

# Upstream truth sources this register joins instead of restating.
CONTRACT_HEALTH_REF = "artifacts/release/m5-contract-health.json"
MATRIX_REF = "artifacts/contracts/m5-stability-lifecycle-map.json"
CONTRACT_CATALOG_REF = "artifacts/contracts/m5-contract-catalog.json"
JSON_SCHEMA_CATALOG_REF = "artifacts/contracts/m5-json-schema-catalog.json"
OPENAPI_CATALOG_REF = "artifacts/contracts/m5-openapi-catalog.json"
WIT_PUBLICATION_REF = "artifacts/contracts/m5-wit-contract-publication.json"
READER_WRITER_COMPAT_REF = "artifacts/contracts/m5-reader-writer-compat-suite.json"
INTERCHANGE_CONFORMANCE_REF = "artifacts/contracts/m5-interchange-conformance.json"
EVIDENCE_INDEX_REF = (
    "artifacts/release/m5/"
    "certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json"
)
BUILD_IDENTITY_REF = "artifacts/build/build_identity.json"
CONTRACT_VALIDATION_REF = "ci/contract_validation.sh"

CONTRACT_HEALTH_PATH = REPO_ROOT / CONTRACT_HEALTH_REF
MATRIX_PATH = REPO_ROOT / MATRIX_REF

# Closed vocabularies. Lifecycle labels and contract forms mirror the publication matrix.
LIFECYCLE_LABELS = ["lts", "stable", "beta", "preview", "withdrawn"]
CONTRACT_FORMS = [
    "json_schema_backed_contract_doc",
    "json_schema_registry",
    "record_registry",
    "event_envelope_schema",
    "wit_world_package",
    "openapi_family",
    "field_set",
    "cli_structured_output",
    "textual_interchange_contract",
    "asset_package_manifest",
    "teaching_content_pack",
]
# The six public-contract pillars every claimed family must publish to certify a contract
# claim. The first five map one-to-one onto the contract-health register's gate kinds; the
# sixth, lifecycle_metadata, certifies that the family declares an explicit version field and
# a lifecycle label (the publication matrix is its certifying artifact).
PILLAR_KINDS = [
    "published_contract_form",
    "lifecycle_metadata",
    "example_corpus",
    "validator_coverage",
    "compatibility_report",
    "release_graph_linkage",
]
EVIDENCE_STATES = ["current", "stale", "missing"]
CERTIFICATION_STATES = [
    "certified",
    "narrowed_row_downgraded",
    "narrowed_stale",
    "narrowed_retest_pending",
    "withheld",
]
CERTIFICATION_REASONS = [
    "row_downgraded",
    "schema_spec_package_missing",
    "lifecycle_metadata_missing",
    "example_corpus_missing",
    "validator_coverage_missing",
    "compatibility_report_missing",
    "release_packet_unlinked",
    "evidence_stale",
    "evidence_missing",
    "retest_pending",
    "mirror_parity_incomplete",
]
STOP_ACTIONS = [
    "hold_certification",
    "hold_promotion",
    "narrow_claim",
    "publish_contract_form",
    "publish_lifecycle_metadata",
    "publish_example_corpus",
    "wire_validator_coverage",
    "publish_compatibility_report",
    "link_release_packet",
    "refresh_evidence",
    "schedule_retest",
    "republish_mirror_bundle",
]
MIRROR_PARITY_STATES = ["current", "stale", "unpublished", "not_applicable"]
# The downstream surfaces that consume this certification packet without manual restatement.
CONSUMER_SURFACES = [
    "claim_publication",
    "release_center",
    "support_center",
    "sdk_docs_publication",
]
DECISION_STATES = ["proceed", "hold"]

# The contract-health gate kind that certifies each pillar.
PILLAR_GATE_KIND = {
    "published_contract_form": "schema_spec_package",
    "example_corpus": "example_corpus",
    "validator_coverage": "validator_coverage",
    "compatibility_report": "compatibility_report",
    "release_graph_linkage": "release_packet_linkage",
}
# The missing-evidence certification reason each pillar raises.
PILLAR_MISSING_REASON = {
    "published_contract_form": "schema_spec_package_missing",
    "lifecycle_metadata": "lifecycle_metadata_missing",
    "example_corpus": "example_corpus_missing",
    "validator_coverage": "validator_coverage_missing",
    "compatibility_report": "compatibility_report_missing",
    "release_graph_linkage": "release_packet_unlinked",
}
# The publish stop-action each pillar raises when it is missing.
PILLAR_PUBLISH_ACTION = {
    "published_contract_form": "publish_contract_form",
    "lifecycle_metadata": "publish_lifecycle_metadata",
    "example_corpus": "publish_example_corpus",
    "validator_coverage": "wire_validator_coverage",
    "compatibility_report": "publish_compatibility_report",
    "release_graph_linkage": "link_release_packet",
}
# Human-readable pillar titles for the docs and report.
PILLAR_TITLES = {
    "published_contract_form": "Published contract form (JSON Schema / WIT / OpenAPI)",
    "lifecycle_metadata": "Lifecycle metadata (explicit version field + lifecycle label)",
    "example_corpus": "Example payload corpus",
    "validator_coverage": "Validator coverage wired into CI",
    "compatibility_report": "Compatibility / migration report",
    "release_graph_linkage": "Release-graph linkage (release packet + build identity)",
}

# Cutline: a family may carry a Stable (or LTS) certified contract claim only when every
# required contract pillar is current. A family missing any required pillar narrows below the
# cutline rather than inheriting an adjacent family's claim.
LAUNCH_CUTLINE = {
    "cutline_level": "stable",
    "above_cutline_levels": ["lts", "stable"],
    "below_cutline_levels": ["beta", "preview", "withdrawn"],
    "description": (
        "A claimed M5 public artifact family certifies a Stable (or LTS) contract claim only "
        "when its published contract form, lifecycle metadata, example corpus, validator "
        "coverage, compatibility report, and release-graph linkage are all current. A family "
        "missing or stale on any required pillar narrows below the cutline; a release-blocking "
        "family missing a required pillar withholds certification and holds promotion."
    ),
}

STOP_RULES = [
    {
        "rule_id": "cert_stop:missing_required_pillar",
        "title": "Missing required contract pillar withholds certification",
        "trigger_reason": "evidence_missing",
        "applies_to_labels": ["lts", "stable", "beta"],
        "default_action": "hold_certification",
        "blocks_promotion": True,
        "rationale": (
            "A claimed family whose published contract form, example corpus, validator "
            "coverage, compatibility report, or release-graph linkage is missing cannot certify "
            "its contract claim; a release-blocking family withholds certification and holds "
            "promotion until the missing contract evidence is published."
        ),
    },
    {
        "rule_id": "cert_stop:stale_pillar",
        "title": "Stale contract pillar narrows the certified claim",
        "trigger_reason": "evidence_stale",
        "applies_to_labels": ["lts", "stable", "beta"],
        "default_action": "narrow_claim",
        "blocks_promotion": False,
        "rationale": (
            "A claimed family whose required contract pillar is due for refresh, breached, or "
            "downgraded narrows its certified claim below the cutline until the evidence is "
            "refreshed; it never inherits an adjacent family's current claim."
        ),
    },
    {
        "rule_id": "cert_stop:claim_parity",
        "title": "Certified label may never run ahead of the public claim",
        "trigger_reason": "row_downgraded",
        "applies_to_labels": ["lts", "stable", "beta", "preview"],
        "default_action": "narrow_claim",
        "blocks_promotion": False,
        "rationale": (
            "A family whose public claim has already narrowed below its marketed label inherits "
            "that narrowing; the certified label equals the published label and never certifies "
            "a greener contract claim than the public claim carries."
        ),
    },
    {
        "rule_id": "cert_stop:mirror_parity",
        "title": "Incomplete mirror/offline contract assets narrow the claim",
        "trigger_reason": "mirror_parity_incomplete",
        "applies_to_labels": ["lts", "stable", "beta"],
        "default_action": "republish_mirror_bundle",
        "blocks_promotion": False,
        "rationale": (
            "A family whose mirror bundle or offline pack lacks the matching contract assets "
            "narrows its certified claim until mirror parity is republished; a stable/beta "
            "contract claim never stays green on a build whose mirror or offline pack is stale "
            "or unpublished."
        ),
    },
]


def load_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def health_register() -> dict:
    return load_json(CONTRACT_HEALTH_PATH)


def matrix_rows() -> dict[str, dict]:
    matrix = load_json(MATRIX_PATH)
    return {row["family_id"]: row for row in matrix.get("rows", [])}


def evidence_state_for_gate(gate: dict) -> str:
    """Map a contract-health gate (freshness + outcome) onto a certification evidence state."""
    outcome = gate.get("outcome")
    freshness = gate.get("freshness")
    if outcome == "fail" or freshness == "missing":
        return "missing"
    if outcome == "pass" and freshness == "current":
        return "current"
    return "stale"


def certifying_artifact_ref(pillar_kind: str, identity_kind: str) -> str:
    """The upstream contract artifact that certifies a pillar (so this register never restates
    field semantics: it points at the canonical form catalog)."""
    if pillar_kind == "published_contract_form":
        return {
            "json_schema": JSON_SCHEMA_CATALOG_REF,
            "openapi_spec": OPENAPI_CATALOG_REF,
            "wit_world": WIT_PUBLICATION_REF,
        }.get(identity_kind, JSON_SCHEMA_CATALOG_REF)
    return {
        "lifecycle_metadata": MATRIX_REF,
        "example_corpus": CONTRACT_CATALOG_REF,
        "validator_coverage": CONTRACT_VALIDATION_REF,
        "compatibility_report": READER_WRITER_COMPAT_REF,
        "release_graph_linkage": BUILD_IDENTITY_REF,
    }[pillar_kind]


def gate_by_kind(health_row: dict) -> dict[str, dict]:
    return {gate["gate_kind"]: gate for gate in health_row.get("gates", [])}


def build_pillars(health_row: dict) -> list[dict]:
    """One certification pillar per pillar kind, derived from the contract-health gates."""
    gates = gate_by_kind(health_row)
    identity_kind = health_row["package_identity"]["identity_kind"]
    pillars: list[dict] = []
    for kind in PILLAR_KINDS:
        if kind == "lifecycle_metadata":
            identity = health_row["package_identity"]
            has_version = isinstance(identity.get("package_version"), int) and identity["package_version"] >= 1
            has_label = health_row.get("lifecycle_label") in LIFECYCLE_LABELS
            state = "current" if (has_version and has_label) else "missing"
            source_refs = [
                f"{MATRIX_REF}#{health_row['family_id']}",
                identity["schema_or_spec_ref"],
            ]
            detail = (
                f"Declares contract version v{identity['package_version']} via "
                f"`{identity['in_band_version_field']}` and lifecycle label "
                f"`{health_row['lifecycle_label']}`."
            )
        else:
            gate = gates[PILLAR_GATE_KIND[kind]]
            state = evidence_state_for_gate(gate)
            source_refs = list(gate.get("evidence_refs", []))
            detail = gate.get("detail", "")
        pillars.append(
            {
                "pillar_kind": kind,
                "title": PILLAR_TITLES[kind],
                "required": True,
                "evidence_state": state,
                "certifying_artifact_ref": certifying_artifact_ref(kind, identity_kind),
                "source_refs": source_refs,
                "detail": detail,
            }
        )
    return pillars


def certification_state(health_row: dict, matrix_row: dict) -> str:
    """Derive the certification state from the upstream contract-health state.

    A blocked release-blocking family withholds; an upstream-narrowed claim inherits the
    narrowing; a retest-pending family narrows for retest; any other narrowing is a stale
    narrowing; an otherwise-healthy, published family certifies.
    """
    health_state = health_row.get("health_state")
    matrix_state = matrix_row.get("row_state")
    if health_state == "blocked":
        return "withheld"
    if health_state == "narrowed":
        if matrix_state == "narrowed":
            return "narrowed_row_downgraded"
        if health_row.get("blocker", {}).get("retest_needed"):
            return "narrowed_retest_pending"
        return "narrowed_stale"
    # healthy
    if matrix_state == "narrowed":
        return "narrowed_row_downgraded"
    return "certified"


def label_rank(label: str) -> int:
    return LIFECYCLE_LABELS.index(label)


def active_reasons(health_row: dict, matrix_row: dict, pillars: list[dict]) -> list[str]:
    reasons: set[str] = set()
    claim = matrix_row["claim_label"]
    published = health_row["lifecycle_label"]
    if label_rank(published) > label_rank(claim):
        reasons.add("row_downgraded")
    for pillar in pillars:
        if pillar["evidence_state"] == "missing":
            reasons.add("evidence_missing")
            reasons.add(PILLAR_MISSING_REASON[pillar["pillar_kind"]])
        elif pillar["evidence_state"] == "stale":
            reasons.add("evidence_stale")
    if health_row.get("blocker", {}).get("retest_needed"):
        reasons.add("retest_pending")
    if health_row["graph_linkage"]["mirror_parity"] in ("stale", "unpublished"):
        reasons.add("mirror_parity_incomplete")
    return [r for r in CERTIFICATION_REASONS if r in reasons]


def stop_actions_for(state: str, health_row: dict, pillars: list[dict], reasons: list[str]) -> list[str]:
    actions: set[str] = set()
    if state == "withheld":
        actions.add("hold_certification")
        if health_row.get("release_blocking"):
            actions.add("hold_promotion")
    if state != "certified":
        actions.add("narrow_claim")
    for pillar in pillars:
        if pillar["evidence_state"] == "missing":
            actions.add(PILLAR_PUBLISH_ACTION[pillar["pillar_kind"]])
        elif pillar["evidence_state"] == "stale":
            actions.add("refresh_evidence")
    if "retest_pending" in reasons:
        actions.add("schedule_retest")
    if "mirror_parity_incomplete" in reasons:
        actions.add("republish_mirror_bundle")
    return [a for a in STOP_ACTIONS if a in actions]


def cert_blocker(state: str, health_row: dict, pillars: list[dict]) -> dict:
    blocking = [
        p["pillar_kind"] for p in pillars if p["evidence_state"] == "missing"
    ]
    decision = "hold" if state == "withheld" else "clear"
    retest = bool(health_row.get("blocker", {}).get("retest_needed"))
    if decision == "hold":
        summary = (
            "A release-blocking family has a missing required contract pillar; certification is "
            "withheld and promotion is held until the missing contract evidence is published."
        )
    elif state.startswith("narrowed"):
        summary = (
            "The family certifies a narrowed contract claim below the cutline; it does not "
            "inherit an adjacent family's current claim."
        )
    else:
        summary = "All contract pillars are current; the family certifies its public contract claim."
    return {
        "decision": decision,
        "blocking_pillar_kinds": blocking,
        "retest_needed": retest,
        "summary": summary,
    }


def build_row(health_row: dict, matrix_rows_by_id: dict[str, dict]) -> dict:
    family_id = health_row["family_id"]
    matrix_row = matrix_rows_by_id[family_id]
    pillars = build_pillars(health_row)
    state = certification_state(health_row, matrix_row)
    reasons = active_reasons(health_row, matrix_row, pillars)
    actions = stop_actions_for(state, health_row, pillars, reasons)
    blocker = cert_blocker(state, health_row, pillars)
    certified_label = health_row["lifecycle_label"]
    claim_label = matrix_row["claim_label"]

    if state == "certified":
        rationale = (
            f"All six contract pillars are current and the published label matches the "
            f"`{claim_label}` public claim; the family certifies its public contract claim."
        )
    elif state == "withheld":
        rationale = (
            f"A required contract pillar is missing; the marketed `{claim_label}` claim "
            f"narrows to a certified `{certified_label}` claim and certification is withheld "
            f"until the missing contract evidence is published."
        )
    else:
        rationale = (
            f"The public claim already narrowed from `{claim_label}` to `{certified_label}`; "
            f"the certification inherits that narrowing and never certifies a greener claim."
        )

    return {
        "family_id": family_id,
        "title": health_row["title"],
        "summary": matrix_row["summary"],
        "owning_package": health_row["owning_package"],
        "owner_dri": matrix_row["owner_dri"],
        "category": matrix_row["category"],
        "contract_form": health_row["contract_form"],
        "release_blocking": health_row["release_blocking"],
        "claim_label": claim_label,
        "source_published_label": health_row["published_label"],
        "certified_label": certified_label,
        "contract_version": health_row["package_identity"]["package_version"],
        "package_identity": health_row["package_identity"],
        "graph_linkage": health_row["graph_linkage"],
        "mirror_parity": health_row["graph_linkage"]["mirror_parity"],
        "pillars": pillars,
        "certification_state": state,
        "active_certification_reasons": reasons,
        "stop_actions": actions,
        "proof": {
            "health_row_ref": f"{CONTRACT_HEALTH_REF}#{family_id}",
            "matrix_row_ref": health_row["matrix_row_ref"],
            "catalog_entry_ref": health_row["catalog_entry_ref"],
            "contract_form_catalog_ref": certifying_artifact_ref(
                "published_contract_form", health_row["package_identity"]["identity_kind"]
            ),
            "compatibility_report_ref": READER_WRITER_COMPAT_REF,
            "release_packet_ref": health_row["graph_linkage"]["release_packet_ref"],
        },
        "blocker": blocker,
        "rationale": rationale,
    }


def compute_summary(rows: list[dict]) -> dict:
    def count(pred) -> int:
        return sum(1 for r in rows if pred(r))

    narrowed_states = {"narrowed_row_downgraded", "narrowed_stale", "narrowed_retest_pending"}
    all_pillars = [p for r in rows for p in r["pillars"]]
    return {
        "total_families": len(rows),
        "release_blocking_families": count(lambda r: r["release_blocking"]),
        "certified_families": count(lambda r: r["certification_state"] == "certified"),
        "narrowed_families": count(lambda r: r["certification_state"] in narrowed_states),
        "withheld_families": count(lambda r: r["certification_state"] == "withheld"),
        "families_held": count(lambda r: r["blocker"]["decision"] == "hold"),
        "families_narrowed_below_claim": count(
            lambda r: label_rank(r["certified_label"]) > label_rank(r["claim_label"])
        ),
        "mirror_publishable_families": count(
            lambda r: r["mirror_parity"] in ("current", "not_applicable")
        ),
        "total_pillars_evaluated": len(all_pillars),
        "pillars_current": sum(1 for p in all_pillars if p["evidence_state"] == "current"),
        "pillars_stale": sum(1 for p in all_pillars if p["evidence_state"] == "stale"),
        "pillars_missing": sum(1 for p in all_pillars if p["evidence_state"] == "missing"),
    }


def build_promotion(rows: list[dict]) -> dict:
    blocking = [
        r["family_id"]
        for r in rows
        if r["certification_state"] == "withheld" and r["release_blocking"]
    ]
    decision = "hold" if blocking else "proceed"
    if blocking:
        rationale = (
            "Certification is held: one or more release-blocking M5 public artifact families "
            "have a missing required contract pillar (published contract form, lifecycle "
            "metadata, example corpus, validator coverage, compatibility report, or "
            "release-graph linkage). Publishing the missing contract evidence and rerunning "
            "the gate clears the hold."
        )
    else:
        rationale = (
            "Every claimed M5 public artifact family certifies its public contract claim or "
            "narrows below the cutline without holding a release-blocking promotion; the "
            "certification packet is clear to publish."
        )
    return {
        "promotion_gate": "m5_public_contract_certification",
        "decision": decision,
        "blocking_family_ids": blocking,
        "rationale": rationale,
    }


def build_register() -> dict:
    health = health_register()
    matrix_by_id = matrix_rows()
    rows = [build_row(hr, matrix_by_id) for hr in health["rows"]]
    return {
        "schema_version": SCHEMA_VERSION,
        "record_kind": RECORD_KIND,
        "register_id": REGISTER_ID,
        "status": "published",
        "as_of": AS_OF,
        "overview_page": OVERVIEW_PAGE,
        "evidence_page": EVIDENCE_PAGE,
        "help_page": HELP_PAGE_REF,
        "report_page": REPORT_REF,
        "shiproom_dashboard_page": SHIPROOM_REF,
        "contract_health_ref": CONTRACT_HEALTH_REF,
        "publication_matrix_ref": MATRIX_REF,
        "contract_catalog_ref": CONTRACT_CATALOG_REF,
        "json_schema_catalog_ref": JSON_SCHEMA_CATALOG_REF,
        "openapi_catalog_ref": OPENAPI_CATALOG_REF,
        "wit_publication_ref": WIT_PUBLICATION_REF,
        "reader_writer_compat_ref": READER_WRITER_COMPAT_REF,
        "interchange_conformance_ref": INTERCHANGE_CONFORMANCE_REF,
        "evidence_index_ref": EVIDENCE_INDEX_REF,
        "build_identity_ref": BUILD_IDENTITY_REF,
        "build_identity": health["build_identity"],
        "lifecycle_labels": list(LIFECYCLE_LABELS),
        "contract_forms": list(CONTRACT_FORMS),
        "pillar_kinds": list(PILLAR_KINDS),
        "evidence_states": list(EVIDENCE_STATES),
        "certification_states": list(CERTIFICATION_STATES),
        "certification_reasons": list(CERTIFICATION_REASONS),
        "stop_actions": list(STOP_ACTIONS),
        "mirror_parity_states": list(MIRROR_PARITY_STATES),
        "consumer_surfaces": list(CONSUMER_SURFACES),
        "decision_states": list(DECISION_STATES),
        "launch_cutline": LAUNCH_CUTLINE,
        "release_blocking_family_refs": [
            r["family_id"] for r in rows if r["release_blocking"]
        ],
        "stop_rules": STOP_RULES,
        "rows": rows,
        "promotion": build_promotion(rows),
        "summary": compute_summary(rows),
    }


# --------------------------------------------------------------------------- docs/report


def _state_glyph(state: str) -> str:
    return {
        "certified": "certified",
        "narrowed_row_downgraded": "narrowed (row downgraded)",
        "narrowed_stale": "narrowed (stale)",
        "narrowed_retest_pending": "narrowed (retest pending)",
        "withheld": "withheld",
    }[state]


def build_report(register: dict) -> str:
    summary = register["summary"]
    promotion = register["promotion"]
    lines: list[str] = []
    lines.append("# M5 public-contract certification report")
    lines.append("")
    lines.append(
        "Closeout certification for the full M5 public-contract publication lane. For every "
        "claimed M5 public artifact family it binds the published contract form, lifecycle "
        "metadata, example corpus, validator coverage, compatibility report, and release-graph "
        "linkage into one certification verdict. It is rendered from one source — the "
        f"certification register at `{REGISTER_REF}` — by `{REGENERATOR_REF}`, which joins the "
        "contract-health register, the publication matrix, and the per-form contract catalogs, "
        "so shiproom, support, docs, SDK, and partner review resolve one certification truth "
        "per family instead of restating field semantics. If this report and the register "
        "disagree, the register wins and both are regenerated together."
    )
    lines.append("")
    lines.append(f"- Register: `{REGISTER_REF}`")
    lines.append(f"- Shiproom dashboard: `{SHIPROOM_REF}`")
    lines.append(f"- Contract-health register: `{CONTRACT_HEALTH_REF}`")
    lines.append(f"- Publication matrix: `{MATRIX_REF}`")
    lines.append(f"- Canonical M5 evidence index: `{EVIDENCE_INDEX_REF}`")
    lines.append(f"- Current as of: `{register['as_of']}`")
    lines.append("")
    lines.append("## Certification decision")
    lines.append("")
    lines.append(f"**{promotion['decision'].upper()}** — {promotion['rationale']}")
    lines.append("")
    if promotion["blocking_family_ids"]:
        lines.append(
            "Withheld release-blocking families: "
            + ", ".join(f"`{f}`" for f in promotion["blocking_family_ids"])
            + "."
        )
        lines.append("")
    lines.append("## Family certification")
    lines.append("")
    lines.append(
        "| Family | Form | Claim | Certified | Ver | State | Pillars (cur/stale/missing) | Decision |"
    )
    lines.append("| --- | --- | --- | --- | --- | --- | --- | --- |")
    for row in register["rows"]:
        cur = sum(1 for p in row["pillars"] if p["evidence_state"] == "current")
        stale = sum(1 for p in row["pillars"] if p["evidence_state"] == "stale")
        missing = sum(1 for p in row["pillars"] if p["evidence_state"] == "missing")
        lines.append(
            f"| `{row['family_id']}` | `{row['contract_form']}` | {row['claim_label']} | "
            f"{row['certified_label']} | v{row['contract_version']} | "
            f"{_state_glyph(row['certification_state'])} | {cur}/{stale}/{missing} | "
            f"{row['blocker']['decision']} |"
        )
    lines.append("")
    lines.append("## Contract pillars")
    lines.append("")
    lines.append(
        "Each family is certified on one cell per pillar. Every pillar is required; a missing "
        "required pillar on a release-blocking family withholds certification and holds "
        "promotion, while a stale pillar narrows the family below the cutline without "
        "inheriting an adjacent family's claim."
    )
    lines.append("")
    lines.append("| Pillar | What it certifies | Certifying artifact |")
    lines.append("| --- | --- | --- |")
    for kind in PILLAR_KINDS:
        artifact = certifying_artifact_ref(kind, "json_schema")
        lines.append(f"| `{kind}` | {PILLAR_TITLES[kind]} | `{artifact}` |")
    lines.append("")
    lines.append("## Counts")
    lines.append("")
    lines.append(
        f"- Families: {summary['total_families']} "
        f"({summary['release_blocking_families']} release-blocking)"
    )
    lines.append(
        f"- Certification: {summary['certified_families']} certified, "
        f"{summary['narrowed_families']} narrowed, {summary['withheld_families']} withheld "
        f"({summary['families_narrowed_below_claim']} narrowed below the marketed claim)"
    )
    lines.append(
        f"- Pillars: {summary['total_pillars_evaluated']} evaluated "
        f"({summary['pillars_current']} current, {summary['pillars_stale']} stale, "
        f"{summary['pillars_missing']} missing)"
    )
    lines.append(
        f"- Mirror/offline publishable families: {summary['mirror_publishable_families']}"
    )
    lines.append("")
    return "\n".join(lines)


def build_shiproom_dashboard(register: dict) -> str:
    summary = register["summary"]
    promotion = register["promotion"]
    lines: list[str] = []
    lines.append("# Shiproom — M5 public-contract certification")
    lines.append("")
    lines.append(
        "Single-screen certification status for the M5 public-contract publication lane, "
        f"rendered from `{REGISTER_REF}` by `{REGENERATOR_REF}`. The certification packet is "
        "the closeout proof that every claimed M5 public artifact family has the right "
        "published contract form, lifecycle metadata, example corpus, validator coverage, "
        "compatibility report, and release-graph linkage — or has narrowed below the cutline."
    )
    lines.append("")
    lines.append(f"**Certification decision: {promotion['decision'].upper()}**")
    lines.append("")
    lines.append(promotion["rationale"])
    lines.append("")
    lines.append("## At a glance")
    lines.append("")
    lines.append(f"- Families: **{summary['total_families']}** ({summary['release_blocking_families']} release-blocking)")
    lines.append(f"- Certified: **{summary['certified_families']}**")
    lines.append(f"- Narrowed: **{summary['narrowed_families']}**")
    lines.append(f"- Withheld: **{summary['withheld_families']}**")
    lines.append(f"- Held (promotion): **{summary['families_held']}**")
    lines.append(
        f"- Pillars: **{summary['pillars_current']}** current / "
        f"**{summary['pillars_stale']}** stale / **{summary['pillars_missing']}** missing"
    )
    lines.append("")
    held = [r for r in register["rows"] if r["blocker"]["decision"] == "hold"]
    lines.append("## Blockers")
    lines.append("")
    if not held:
        lines.append("No certification blockers. Every claimed family certifies or narrows cleanly.")
    else:
        lines.append("| Family | Claim | Certified | Missing pillars | Reasons | Stop actions |")
        lines.append("| --- | --- | --- | --- | --- | --- |")
        for row in held:
            missing = ", ".join(f"`{p}`" for p in row["blocker"]["blocking_pillar_kinds"]) or "—"
            reasons = ", ".join(f"`{r}`" for r in row["active_certification_reasons"]) or "—"
            actions = ", ".join(f"`{a}`" for a in row["stop_actions"]) or "—"
            lines.append(
                f"| `{row['family_id']}` | {row['claim_label']} | {row['certified_label']} | "
                f"{missing} | {reasons} | {actions} |"
            )
    lines.append("")
    lines.append("## Narrowed below the marketed claim")
    lines.append("")
    narrowed = [
        r for r in register["rows"]
        if label_rank(r["certified_label"]) > label_rank(r["claim_label"])
    ]
    if not narrowed:
        lines.append("No family certifies below its marketed claim.")
    else:
        for row in narrowed:
            lines.append(
                f"- `{row['family_id']}`: marketed `{row['claim_label']}` → certified "
                f"`{row['certified_label']}` ({_state_glyph(row['certification_state'])})."
            )
    lines.append("")
    lines.append("## Sources")
    lines.append("")
    lines.append(f"- Certification report: `{REPORT_REF}`")
    lines.append(f"- Contract-health register: `{CONTRACT_HEALTH_REF}`")
    lines.append(f"- Publication matrix: `{MATRIX_REF}`")
    lines.append(f"- Help-center page: `{HELP_PAGE_REF}`")
    lines.append("")
    return "\n".join(lines)


def build_help_doc(register: dict) -> str:
    lines: list[str] = []
    lines.append("# M5 public-contract certification")
    lines.append("")
    lines.append(
        "Aureline certifies every claimed public artifact family with a single certification "
        "packet. For each family the packet records its published contract form, its lifecycle "
        "metadata (version + stability label), its example payload corpus, the validator that "
        "guards it, its compatibility report, and the release packet and build identity it "
        "shipped with — then decides whether the family certifies its contract claim or has "
        "narrowed below the certification cutline."
    )
    lines.append("")
    lines.append("## Where the packet lives")
    lines.append("")
    lines.append(f"- Machine-readable register: `{REGISTER_REF}`")
    lines.append(f"- Human-readable report: `{REPORT_REF}`")
    lines.append(f"- Shiproom dashboard: `{SHIPROOM_REF}`")
    lines.append(f"- JSON Schema: `{SCHEMA_REF}`")
    lines.append("")
    lines.append("## How to read a certification row")
    lines.append("")
    lines.append(
        "- **Claim** is the marketed lifecycle label; **certified** is the label the family "
        "may actually carry. When they differ, the family narrowed."
    )
    lines.append(
        "- **Pillars** are the six contract assets a family must publish to certify a contract "
        "claim. A `missing` required pillar on a release-blocking family withholds "
        "certification and holds promotion."
    )
    lines.append(
        "- **State** is one of: `certified`, `narrowed_row_downgraded`, `narrowed_stale`, "
        "`narrowed_retest_pending`, or `withheld`."
    )
    lines.append("")
    lines.append("## Inspect it locally")
    lines.append("")
    lines.append("```sh")
    lines.append(
        "cargo run -q -p aureline-release \\\n"
        "  --bin aureline_release_certify_schema_publication_wit_openapi "
        "-- inspect command_descriptors"
    )
    lines.append("```")
    lines.append("")
    lines.append(
        f"The certification packet is consumed by claim-publication, release-center, "
        f"support-center, and SDK/docs publication flows; it joins the contract-health register "
        f"(`{CONTRACT_HEALTH_REF}`) and the publication matrix (`{MATRIX_REF}`) rather than "
        f"restating their field semantics."
    )
    lines.append("")
    return "\n".join(lines)


def build_overview_doc(register: dict) -> str:
    summary = register["summary"]
    promotion = register["promotion"]
    lines: list[str] = []
    lines.append(
        "# Certify schema publication, WIT/OpenAPI packaging, validator coverage, and "
        "compatibility truth on every claimed M5 public artifact family"
    )
    lines.append("")
    lines.append(
        "This is the closeout row for the M5 public-contract publication lane. Earlier rows "
        "publish the individual contract forms — the JSON Schema catalog, the OpenAPI catalog, "
        "the WIT publication, the reader/writer compatibility suite, and the "
        "interchange-conformance register — plus the publication matrix that records whether "
        "each family published its required forms and the contract-health register that "
        "enforces those forms with CI gates and a release-graph linkage. This row certifies "
        "the whole lane: it joins all of them into one certification packet that proves every "
        "claimed M5 public artifact family has the contract assets its claim requires, and "
        "narrows or withholds any family whose contract packages are missing, stale, or "
        "mirror-incomplete."
    )
    lines.append("")
    lines.append("## What the certification packet binds")
    lines.append("")
    lines.append(
        "For every claimed family the packet records one row binding the family to its "
        "**published contract form**, its **lifecycle metadata** (explicit version field and "
        "stability label), its **example corpus**, its **validator coverage**, its "
        "**compatibility report**, and its **release-graph linkage** (release packet plus the "
        "one build identity the candidate shipped). Each pillar carries its own evidence state "
        "(`current`, `stale`, or `missing`), so a stale compatibility report narrows a family "
        "while its schema and validator pillars stay current."
    )
    lines.append("")
    lines.append("## How narrowing works")
    lines.append("")
    lines.append(
        "The certification reuses the contract-health register's per-family gate evaluation and "
        "the publication matrix's lifecycle labels rather than minting a new vocabulary. A "
        "family certifies only when every required pillar is current and its published label "
        "matches its public claim. A family may never certify a greener label than its public "
        "claim; a family whose public claim already narrowed inherits that narrowing; and a "
        "release-blocking family missing a required pillar withholds certification and holds "
        "promotion."
    )
    lines.append("")
    lines.append("## Current state")
    lines.append("")
    lines.append(f"**Certification decision: {promotion['decision'].upper()}.** {promotion['rationale']}")
    lines.append("")
    lines.append(
        f"- {summary['total_families']} claimed families "
        f"({summary['release_blocking_families']} release-blocking)."
    )
    lines.append(
        f"- {summary['certified_families']} certified, {summary['narrowed_families']} narrowed, "
        f"{summary['withheld_families']} withheld."
    )
    lines.append(
        f"- {summary['pillars_current']} pillars current, {summary['pillars_stale']} stale, "
        f"{summary['pillars_missing']} missing across {summary['total_pillars_evaluated']} "
        f"evaluated."
    )
    lines.append("")
    lines.append("## Sources and consumers")
    lines.append("")
    lines.append(f"- Register (truth source): `{REGISTER_REF}`")
    lines.append(f"- Report: `{REPORT_REF}`")
    lines.append(f"- Shiproom dashboard: `{SHIPROOM_REF}`")
    lines.append(f"- Help-center page: `{HELP_PAGE_REF}`")
    lines.append(f"- JSON Schema: `{SCHEMA_REF}`")
    lines.append(f"- Validator: `{VALIDATOR_REF}`")
    lines.append(f"- Regenerator: `{REGENERATOR_REF}`")
    lines.append("")
    lines.append(
        "The packet is consumed by claim-publication, release-center, support-center, and "
        "SDK/docs publication flows; it is referenced against the canonical M5 evidence index "
        f"at `{EVIDENCE_INDEX_REF}`."
    )
    lines.append("")
    return "\n".join(lines)


def build_evidence_doc(register: dict) -> str:
    summary = register["summary"]
    promotion = register["promotion"]
    lines: list[str] = []
    lines.append(
        "# Evidence — certify schema publication, WIT/OpenAPI packaging, validator coverage, "
        "and compatibility truth on every claimed M5 public artifact family"
    )
    lines.append("")
    lines.append(
        "Proof packet for the M5 public-contract certification closeout. It lists the checked-in "
        "artifacts, the upstream truth they join, and the automated and operator-facing proof "
        "that exercises them."
    )
    lines.append("")
    lines.append(f"- Current as of: `{register['as_of']}`")
    lines.append(f"- Certification decision: **{promotion['decision'].upper()}**")
    lines.append("")
    lines.append("## Checked-in artifacts")
    lines.append("")
    lines.append(f"- Certification register: `{REGISTER_REF}`")
    lines.append(f"- Certification report: `{REPORT_REF}`")
    lines.append(f"- Shiproom dashboard: `{SHIPROOM_REF}`")
    lines.append(f"- Help-center page: `{HELP_PAGE_REF}`")
    lines.append(f"- JSON Schema: `{SCHEMA_REF}`")
    lines.append(f"- Negative fixtures: `fixtures/contracts/m5-public-contract-certification/`")
    lines.append(f"- CI capture: `artifacts/release/captures/{NAME}_validation_capture.json`")
    lines.append("")
    lines.append("## Upstream contract truth joined")
    lines.append("")
    lines.append(f"- Contract-health register: `{CONTRACT_HEALTH_REF}`")
    lines.append(f"- Publication matrix: `{MATRIX_REF}`")
    lines.append(f"- Contract catalog: `{CONTRACT_CATALOG_REF}`")
    lines.append(f"- JSON Schema catalog: `{JSON_SCHEMA_CATALOG_REF}`")
    lines.append(f"- OpenAPI catalog: `{OPENAPI_CATALOG_REF}`")
    lines.append(f"- WIT publication: `{WIT_PUBLICATION_REF}`")
    lines.append(f"- Reader/writer compatibility suite: `{READER_WRITER_COMPAT_REF}`")
    lines.append(f"- Interchange-conformance register: `{INTERCHANGE_CONFORMANCE_REF}`")
    lines.append(f"- Canonical M5 evidence index: `{EVIDENCE_INDEX_REF}`")
    lines.append("")
    lines.append("## Proof")
    lines.append("")
    lines.append(f"- Schema + semantic + drift + cross-source validator: `{VALIDATOR_REF}`")
    lines.append(
        f"- Typed Rust consumer + tests: `crates/aureline-release/src/{NAME}/`"
    )
    lines.append(
        f"- In-product inspect surface: `crates/aureline-release/src/bin/aureline_release_{NAME}.rs`"
    )
    lines.append(f"- CI gate: `{CI_WORKFLOW_REF}`")
    lines.append("")
    lines.append("## Result")
    lines.append("")
    lines.append(
        f"{summary['certified_families']} of {summary['total_families']} claimed families "
        f"certify their public contract claim; {summary['narrowed_families']} narrow and "
        f"{summary['withheld_families']} withhold. "
        f"{summary['families_narrowed_below_claim']} family/families certify below the marketed "
        f"claim, demonstrating the automatic narrowing the closeout requires."
    )
    lines.append("")
    return "\n".join(lines)


def build_capture(register: dict) -> dict:
    return {
        "record_kind": "m5_public_contract_certification_validation_capture",
        "register_id": register["register_id"],
        "as_of": register["as_of"],
        "decision": register["promotion"]["decision"],
        "summary": register["summary"],
        "blocking_family_ids": register["promotion"]["blocking_family_ids"],
        "sources": {
            "register": REGISTER_REF,
            "schema": SCHEMA_REF,
            "validator": VALIDATOR_REF,
            "regenerator": REGENERATOR_REF,
            "ci_workflow": CI_WORKFLOW_REF,
        },
    }


# --------------------------------------------------------------------------- negative fixtures


def build_negative_fixtures(register: dict) -> dict[str, dict]:
    """Structurally valid registers that each trip exactly one semantic invariant."""
    fixtures: dict[str, dict] = {}

    # 1) Duplicate family id.
    dup = json.loads(json.dumps(register))
    dup["rows"].append(json.loads(json.dumps(dup["rows"][0])))
    dup["summary"] = compute_summary(dup["rows"])
    fixtures["duplicate_family_id.json"] = dup

    # 2) Certification state disagrees with the pillars: a withheld family relabeled certified.
    state_drift = json.loads(json.dumps(register))
    target = next(
        (r for r in state_drift["rows"] if r["certification_state"] == "withheld"),
        None,
    )
    if target is None:
        target = state_drift["rows"][0]
        target["pillars"][4]["evidence_state"] = "missing"
    target["certification_state"] = "certified"
    fixtures["certification_state_mismatch.json"] = state_drift

    # 3) Certified label greener than the public claim (claim parity broken).
    parity = json.loads(json.dumps(register))
    prow = parity["rows"][0]
    prow["claim_label"] = "beta"
    prow["certified_label"] = "stable"
    fixtures["certified_label_greener_than_claim.json"] = parity

    # 4) Pillar coverage gap: a row drops a required pillar.
    coverage = json.loads(json.dumps(register))
    coverage["rows"][0]["pillars"].pop()
    fixtures["missing_pillar.json"] = coverage

    # 5) Unknown certification state.
    unknown_state = json.loads(json.dumps(register))
    unknown_state["rows"][0]["certification_state"] = "blessed"
    fixtures["unknown_certification_state.json"] = unknown_state

    # 6) Summary count drift.
    summary_drift = json.loads(json.dumps(register))
    summary_drift["summary"]["total_families"] += 1
    fixtures["summary_count_mismatch.json"] = summary_drift

    # 7) Promotion decision disagrees with the withheld rows.
    promo_drift = json.loads(json.dumps(register))
    promo_drift["promotion"]["decision"] = "proceed" if promo_drift["promotion"]["decision"] == "hold" else "hold"
    promo_drift["promotion"]["blocking_family_ids"] = []
    fixtures["promotion_decision_mismatch.json"] = promo_drift

    return fixtures


NEGATIVE_CASES = [
    {"case_id": "fixture:duplicate_family_id", "file": "duplicate_family_id.json", "expected_check": "rows.duplicate_family_id"},
    {"case_id": "fixture:certification_state_mismatch", "file": "certification_state_mismatch.json", "expected_check": "rows.certification_state"},
    {"case_id": "fixture:certified_label_greener_than_claim", "file": "certified_label_greener_than_claim.json", "expected_check": "rows.claim_parity"},
    {"case_id": "fixture:missing_pillar", "file": "missing_pillar.json", "expected_check": "rows.pillar_coverage"},
    {"case_id": "fixture:unknown_certification_state", "file": "unknown_certification_state.json", "expected_check": "rows.unknown_certification_state"},
    {"case_id": "fixture:summary_count_mismatch", "file": "summary_count_mismatch.json", "expected_check": "summary.count_mismatch"},
    {"case_id": "fixture:promotion_decision_mismatch", "file": "promotion_decision_mismatch.json", "expected_check": "promotion.decision"},
]


# --------------------------------------------------------------------------- write


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

    write_text(REPORT_PATH, build_report(register))
    print(f"wrote {REPORT_PATH.relative_to(REPO_ROOT)}")
    write_text(SHIPROOM_PATH, build_shiproom_dashboard(register))
    print(f"wrote {SHIPROOM_PATH.relative_to(REPO_ROOT)}")
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
    write_json(NEGATIVE_DIR / "cases.json", {"cases": NEGATIVE_CASES})
    for filename in list(fixtures) + ["cases.json"]:
        print(f"wrote {(NEGATIVE_DIR / filename).relative_to(REPO_ROOT)}")


if __name__ == "__main__":
    main()
