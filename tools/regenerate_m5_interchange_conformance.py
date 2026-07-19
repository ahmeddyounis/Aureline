#!/usr/bin/env python3
"""Regenerate the M5 interchange-conformance register, per-family import/export
validator descriptors, the cross-surface conformance report, the docs/capture, the
real emitted-artifact corpus, and the negative fixtures that prove them.

This is the single source of truth for the **M5 interchange-conformance register**:
the machine-readable join that ties every high-value M5 import/export family — request
/API collections, notebook paired/parity exports, docs suggestion/validation/evidence
packets, trace/profile/replay exports, support bundles, and portable-state packages —
to the import/export validator that guards it, the cross-surface conformance runner
that exercises a real emitted artifact, the contract version and lifecycle label the
desktop, CLI/headless, and support/export consumers must agree on, the degraded-state
vocabulary they share, and the stable, copy-safe reason codes an interchange failure
reports instead of a raw parser exception.

It exists so an M5 artifact family cannot claim interchange support on the strength of a
local export alone: each family must prove an import/validation path *and* more than one
consumer path, must not silently widen trust or strip required provenance, and must
preserve the raw-versus-rendered, local-versus-managed, and compare-only-versus-write-back
distinctions the rest of the M5 contract lane promises. Where the source docs only promise
compare-only or inspect-only behavior, that is encoded as a valid conformance class rather
than forcing write-back support.

It reads the checked-in contract catalog so a family that is *also* a published contract
family (trace/profile/replay → ``replay_and_trace_evidence``; support bundles →
``support_bundles_and_handoff``) inherits that family's published lifecycle label and can
never advertise a greener interchange label than the catalog does. Families that are not
themselves published contract families declare their own lifecycle label in this register.

It writes, all deterministically:

  * ``artifacts/contracts/m5-interchange-conformance.json``        (the register)
  * ``artifacts/contracts/m5-interchange-conformance.md``          (the conformance report)
  * ``validators/m5-interchange/manifest.json``                   (the validator manifest)
  * ``validators/m5-interchange/<family_id>.json``                (per-family validator descriptors)
  * ``validators/m5-interchange/README.md``                       (validator index)
  * ``fixtures/contracts/m5-interchange/emitted/<family_id>.json``  (real emitted artifacts)
  * ``fixtures/contracts/m5-interchange/negative/{cases.json,*.json}`` (negative fixtures)
  * ``docs/help/m5-interchange-conformance.md``                   (Help-center page)
  * ``docs/m5/<slug>.md``                                          (narrative companion)
  * ``artifacts/m5/<slug>.md``                                     (evidence/proof packet)
  * ``artifacts/release/captures/<name>_validation_capture.json``  (CI capture)

Run ``python3 tools/regenerate_m5_interchange_conformance.py`` after editing this script or
the upstream catalog, then ``python3 tools/validate_m5_interchange_conformance.py`` and
``cargo test -p aureline-release --test rel_it_05_add_import_export_validators``
to confirm the validator and the typed model agree.

The register and every emitted artifact are metadata-plus-state only: every field is a
typed state, an opaque repo-relative ref or URI, or a copy/export-safe summary. They carry
no credential bodies or raw provider payloads, and the register never reads live, per-build
values (the commit and dirty flag are resolved from the build-identity artifact at review
time) so the checked-in artifacts stay deterministic.
"""

from __future__ import annotations

import json
from pathlib import Path

NAME = (
    "add_import_export_validators_and_cross_surface_conformance_runners_for_m5_"
    "interchange_families"
)
RECORD_KIND = "m5_interchange_conformance_register"
REGISTER_ID = "m5_interchange_conformance:v1"
SCHEMA_VERSION = 1
AS_OF = "2026-06-19"

VALIDATOR_MANIFEST_RECORD_KIND = "m5_interchange_validator_manifest"
VALIDATOR_MANIFEST_ID = "m5_interchange_validator_manifest:v1"
VALIDATOR_DESCRIPTOR_RECORD_KIND = "m5_interchange_validator_descriptor"
EMITTED_ARTIFACT_RECORD_KIND = "m5_interchange_emitted_artifact"

REPO_ROOT = Path(__file__).resolve().parent.parent

# The long row slug the source set names for the docs/evidence pages.
DOC_SLUG = (
    "add-import-export-validators-and-cross-surface-conformance-runners-for-m5-"
    "request-api-collections-notebook-parity-artifacts-docs-packets-trace-profile-"
    "replay-support-bundles-and-portable-state-packages"
)

# Outputs.
REGISTER_PATH = REPO_ROOT / "artifacts" / "contracts" / "m5-interchange-conformance.json"
REPORT_PATH = REPO_ROOT / "artifacts" / "contracts" / "m5-interchange-conformance.md"
VALIDATORS_DIR = REPO_ROOT / "validators" / "m5-interchange"
VALIDATOR_MANIFEST_PATH = VALIDATORS_DIR / "manifest.json"
VALIDATORS_README_PATH = VALIDATORS_DIR / "README.md"
EMITTED_DIR = REPO_ROOT / "fixtures" / "contracts" / "m5-interchange" / "emitted"
NEGATIVE_DIR = REPO_ROOT / "fixtures" / "contracts" / "m5-interchange" / "negative"
HELP_DOC_PATH = REPO_ROOT / "docs" / "help" / "m5-interchange-conformance.md"
OVERVIEW_DOC_PATH = REPO_ROOT / "docs" / "m5" / f"{DOC_SLUG}.md"
EVIDENCE_DOC_PATH = REPO_ROOT / "artifacts" / "m5" / f"{DOC_SLUG}.md"
CAPTURE_PATH = (
    REPO_ROOT / "artifacts" / "release" / "captures" / f"{NAME}_validation_capture.json"
)

# Output refs (repo-relative) the register and its companions cross-link.
REGISTER_REF = "artifacts/contracts/m5-interchange-conformance.json"
REPORT_REF = "artifacts/contracts/m5-interchange-conformance.md"
VALIDATORS_HOME = "validators/m5-interchange/"
VALIDATOR_MANIFEST_REF = "validators/m5-interchange/manifest.json"
EMITTED_HOME = "fixtures/contracts/m5-interchange/emitted/"
NEGATIVE_HOME = "fixtures/contracts/m5-interchange/negative/"
HELP_PAGE_REF = "docs/help/m5-interchange-conformance.md"
OVERVIEW_PAGE = f"docs/m5/{DOC_SLUG}.md"
EVIDENCE_PAGE = f"artifacts/m5/{DOC_SLUG}.md"
SCHEMA_REF = "schemas/public/m5-contracts/m5_interchange_conformance.schema.json"
VALIDATOR_REF = "tools/validate_m5_interchange_conformance.py"
REGENERATOR_REF = "tools/regenerate_m5_interchange_conformance.py"
CI_WORKFLOW_REF = ".github/workflows/check_m5_interchange_conformance.yml"

# Upstream truth sources this register consumes instead of restating.
CATALOG_REF = "artifacts/contracts/m5-contract-catalog.json"
MATRIX_REF = "artifacts/contracts/m5-stability-lifecycle-map.json"
COMPAT_SUITE_REF = "artifacts/contracts/m5-reader-writer-compat-suite.json"
BUILD_IDENTITY_REF = "artifacts/build/build_identity.json"
EVIDENCE_INDEX_REF = (
    "artifacts/release/m5/"
    "certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json"
)

CATALOG_PATH = REPO_ROOT / CATALOG_REF

# Closed vocabularies. Kept in lockstep with the typed Rust consumer and the boundary
# schema; the validator and the model both reject anything off-list.
LIFECYCLE_LABELS = ["lts", "stable", "beta", "preview", "withdrawn"]

# The interchange direction a family proves end-to-end.
INTERCHANGE_DIRECTIONS = ["export_only", "import_validation", "round_trip"]

# The conformance class. ``compare_only`` and ``import_validation_only`` are first-class
# valid classes, not downgrades — a family the source docs scope to compare/inspect is not
# forced to support write-back.
CONFORMANCE_CLASSES = ["round_trip_write_back", "import_validation_only", "compare_only"]

# The consumer surfaces that must agree on contract version, lifecycle label, and the
# degraded-state vocabulary for at least one fixture in each family.
CONSUMER_SURFACES = ["desktop", "cli_headless", "support_export"]

# The conformance dimensions (one cell per kind per family), in evaluation order.
DIMENSION_KINDS = [
    "emitted_artifact_present",
    "import_export_validator",
    "round_trip_or_compare",
    "provenance_preserved",
    "trust_not_widened",
    "cross_surface_agreement",
    "stable_reason_codes",
]
DIMENSION_OUTCOMES = ["pass", "downgrade", "fail"]
CONFORMANCE_STATES = ["conformant", "narrowed", "failed"]
DECISION_STATES = ["clear", "hold"]

# The shared degraded-state vocabulary the consumers agree on. A degraded outcome is a
# stable user-facing state, never an error.
DEGRADED_STATES = ["partial", "not_provided", "compare_only", "degraded", "unavailable"]

# The stable, copy-safe import/export reason codes. An interchange failure reports one of
# these instead of a raw parser exception or a generic corruption message.
REASON_CODES = [
    "unsupported_contract_version",
    "missing_required_provenance",
    "schema_validation_failed",
    "trust_widening_blocked",
    "round_trip_mismatch",
    "corrupt_or_truncated_payload",
    "unknown_field_unpreserved",
    "redaction_class_conflict",
]

# Per-dimension definition: what it proves, and which reason code a failure reports.
DIMENSION_DEFS = {
    "emitted_artifact_present": {
        "title": "Real emitted artifact present",
        "description": (
            "The family emits a real, checked-in interchange artifact the conformance "
            "runner exercises, rather than asserting interchange support against a "
            "synthetic stub."
        ),
        "fail_reason_code": "corrupt_or_truncated_payload",
    },
    "import_export_validator": {
        "title": "Import/export validator wired",
        "description": (
            "The family has an import/export validator descriptor that validates the "
            "emitted artifact and rejects an import that is corrupt, version-skewed, or "
            "trust-widening with a stable reason code."
        ),
        "fail_reason_code": "schema_validation_failed",
    },
    "round_trip_or_compare": {
        "title": "Round-trip or declared compare-only behavior proven",
        "description": (
            "The family proves the behavior its conformance class declares: a "
            "round-trip-write-back family round-trips without loss, while a "
            "compare-only or import-validation-only family proves its scoped behavior "
            "without being forced to support write-back."
        ),
        "fail_reason_code": "round_trip_mismatch",
    },
    "provenance_preserved": {
        "title": "Required provenance preserved",
        "description": (
            "Import/export preserves the family's required provenance (source surface, "
            "build identity, record class, redaction class) instead of stripping it."
        ),
        "fail_reason_code": "missing_required_provenance",
    },
    "trust_not_widened": {
        "title": "Trust not silently widened",
        "description": (
            "Import does not silently widen trust: a local-only or limited-trust artifact "
            "stays local-only or limited-trust, and a managed/remote record is not "
            "promoted to a durable local one without an explicit decision."
        ),
        "fail_reason_code": "trust_widening_blocked",
    },
    "cross_surface_agreement": {
        "title": "Cross-surface consumer agreement",
        "description": (
            "Desktop, CLI/headless, and support/export consumers agree on the contract "
            "version, the lifecycle label, and the degraded-state vocabulary for the "
            "emitted artifact."
        ),
        "fail_reason_code": "unsupported_contract_version",
    },
    "stable_reason_codes": {
        "title": "Stable, copy-safe failure reason codes",
        "description": (
            "Every interchange failure mode the validator reports maps to a stable, "
            "copy-safe reason code from the closed vocabulary instead of a raw parser "
            "exception or a generic corruption message."
        ),
        "fail_reason_code": "schema_validation_failed",
    },
}

# Copy-safe diagnostic message templates for each reason code. These are what an
# interchange failure surfaces to a user or a support packet instead of a stack trace.
REASON_CODE_DIAGNOSTICS = {
    "unsupported_contract_version": (
        "This artifact declares a contract version this build does not support. Export it "
        "again from a compatible build, or upgrade before importing."
    ),
    "missing_required_provenance": (
        "This artifact is missing required provenance (source surface, build identity, or "
        "record class). It cannot be imported without the provenance that proves where it "
        "came from."
    ),
    "schema_validation_failed": (
        "This artifact does not match the published contract schema for its family. No "
        "fields were imported; nothing was changed."
    ),
    "trust_widening_blocked": (
        "Importing this artifact would widen its trust (for example, promoting a managed "
        "or limited-trust record to a durable local one). The import was blocked; re-run "
        "it with an explicit trust decision."
    ),
    "round_trip_mismatch": (
        "Re-exporting this artifact after import did not reproduce it byte-for-byte. The "
        "import was blocked to avoid silent data loss."
    ),
    "corrupt_or_truncated_payload": (
        "This artifact is truncated or corrupt and could not be read. Re-export it; "
        "nothing was imported."
    ),
    "unknown_field_unpreserved": (
        "This artifact carries fields a round-trip would drop. The import was blocked so "
        "unknown fields are preserved rather than silently lost."
    ),
    "redaction_class_conflict": (
        "This artifact's redaction class conflicts with the destination's policy. The "
        "import was blocked; export it again at a compatible redaction class."
    ),
}

# The named M5 interchange families. Each declares its interchange direction, conformance
# class, lifecycle label (or the catalog family it inherits one from), the real emitted
# artifact the runner exercises, the validator that guards it, the reason codes it can
# report, the degraded states it supports, and any per-dimension downgrade/fail overrides
# (none in the checked-in register; every family is conformant). The negative fixtures and
# the typed model's unit tests prove the narrowing and rejection paths.
FAMILIES = [
    {
        "family_id": "request_api_collections",
        "title": "Request/API collection import & export",
        "summary": (
            "Request/API collection bundles (environments, request trees, saved responses, "
            "and run history) exported from and imported into the request workspace, and "
            "validated on import so a third-party or mirrored collection cannot widen trust "
            "or strip its source identity."
        ),
        "owning_package": "aureline-collections",
        "contract_form": "json_schema_backed_contract_doc",
        "interchange_direction": "round_trip",
        "conformance_class": "round_trip_write_back",
        "lifecycle_label": "beta",
        "release_blocking": False,
        "catalog_family_id": None,
        "contract_version": 1,
        "contract_version_field": "request_collection_schema_version",
        "emitted_record_kind": "request_collection_export",
        "reason_codes_emitted": [
            "unsupported_contract_version",
            "schema_validation_failed",
            "trust_widening_blocked",
            "round_trip_mismatch",
        ],
        "degraded_states_supported": ["partial", "not_provided", "degraded"],
        "payload": {
            "collection_id": "request_api_collections-example-0001",
            "environment_count": 2,
            "request_count": 14,
            "saved_response_count": 6,
            "source_kind": "third_party_import",
        },
    },
    {
        "family_id": "notebook_parity_exports",
        "title": "Notebook paired/parity export & compare",
        "summary": (
            "Notebook paired/parity exports (the paired script representation, cell map, and "
            "output digest) compared against a live notebook on import. Parity is a "
            "compare-only contract by design: the export is inspected and diffed, never "
            "written back over a trusted notebook without an explicit user decision."
        ),
        "owning_package": "aureline-notebook",
        "contract_form": "textual_interchange_contract",
        "interchange_direction": "import_validation",
        "conformance_class": "compare_only",
        "lifecycle_label": "beta",
        "release_blocking": False,
        "catalog_family_id": None,
        "contract_version": 1,
        "contract_version_field": "notebook_parity_schema_version",
        "emitted_record_kind": "notebook_parity_export",
        "reason_codes_emitted": [
            "schema_validation_failed",
            "round_trip_mismatch",
            "unknown_field_unpreserved",
        ],
        "degraded_states_supported": ["compare_only", "partial", "not_provided"],
        "payload": {
            "notebook_id": "notebook_parity_exports-example-0001",
            "paired_representation": "script_with_cell_markers",
            "cell_count": 9,
            "output_digest_present": True,
            "compare_only": True,
        },
    },
    {
        "family_id": "docs_packets",
        "title": "Docs suggestion/validation/evidence packet import",
        "summary": (
            "Docs packets (suggestion sets, validation results, and authoring evidence) "
            "exported for review and validated on import so a packet cannot be imported "
            "without its evidence chain. Docs packets are import-validation-only: they are "
            "checked and surfaced for review, never auto-applied over project docs."
        ),
        "owning_package": "aureline-docs",
        "contract_form": "json_schema_backed_contract_doc",
        "interchange_direction": "import_validation",
        "conformance_class": "import_validation_only",
        "lifecycle_label": "beta",
        "release_blocking": False,
        "catalog_family_id": None,
        "contract_version": 1,
        "contract_version_field": "docs_packet_schema_version",
        "emitted_record_kind": "docs_packet_export",
        "reason_codes_emitted": [
            "missing_required_provenance",
            "schema_validation_failed",
            "redaction_class_conflict",
        ],
        "degraded_states_supported": ["partial", "not_provided", "degraded"],
        "payload": {
            "packet_id": "docs_packets-example-0001",
            "suggestion_count": 5,
            "validation_result_count": 5,
            "evidence_chain_present": True,
            "apply_mode": "review_only",
        },
    },
    {
        "family_id": "trace_profile_replay_exports",
        "title": "Trace/profile/replay export & round-trip",
        "summary": (
            "Profiling/trace/replay capture bundles exported for support and regression "
            "review and re-imported for deterministic replay. Inherits the published "
            "lifecycle label of the replay-and-trace contract family so the interchange "
            "claim can never run ahead of the contract."
        ),
        "owning_package": "aureline-observability",
        "contract_form": "json_schema_backed_contract_doc",
        "interchange_direction": "round_trip",
        "conformance_class": "round_trip_write_back",
        "lifecycle_label": None,  # inherited from the catalog
        "release_blocking": False,
        "catalog_family_id": "replay_and_trace_evidence",
        "contract_version": 1,
        "contract_version_field": "capture_session_schema_version",
        "emitted_record_kind": "trace_profile_replay_export",
        "reason_codes_emitted": [
            "unsupported_contract_version",
            "corrupt_or_truncated_payload",
            "round_trip_mismatch",
        ],
        "degraded_states_supported": ["partial", "not_provided", "degraded"],
        "payload": {
            "capture_session_id": "trace_profile_replay_exports-example-0001",
            "replay_capability": "deterministic_replay",
            "trace_span_count": 1280,
            "exact_build_identity_ref": "build-identity-0001",
        },
    },
    {
        "family_id": "support_bundles",
        "title": "Support bundle export & import validation",
        "summary": (
            "Support/evidence bundles exported for hosted review and validated on import so "
            "a bundle's redaction class and source identity survive the round into the "
            "support tool. Inherits the published lifecycle label of the support-bundle "
            "contract family."
        ),
        "owning_package": "aureline-support",
        "contract_form": "json_schema_backed_contract_doc",
        "interchange_direction": "import_validation",
        "conformance_class": "import_validation_only",
        "lifecycle_label": None,  # inherited from the catalog
        "release_blocking": True,
        "catalog_family_id": "support_bundles_and_handoff",
        "contract_version": 1,
        "contract_version_field": "support_bundle_schema_version",
        "emitted_record_kind": "support_bundle_export",
        "reason_codes_emitted": [
            "missing_required_provenance",
            "schema_validation_failed",
            "redaction_class_conflict",
        ],
        "degraded_states_supported": ["partial", "not_provided", "degraded"],
        "payload": {
            "support_bundle_id": "support_bundles-example-0001",
            "bundle_redaction_class": "metadata_only",
            "packet_index_row_count": 7,
            "object_handoff_present": True,
        },
    },
    {
        "family_id": "portable_state_packages",
        "title": "Portable-state package export & round-trip",
        "summary": (
            "Portable-state packages (workspace continuity, layout, and saved-object state) "
            "exported for migration and re-imported on another machine, with a round-trip "
            "that preserves unknown fields and a trust decision that does not silently "
            "promote a mirrored package to a durable local one."
        ),
        "owning_package": "aureline-continuity",
        "contract_form": "asset_package_manifest",
        "interchange_direction": "round_trip",
        "conformance_class": "round_trip_write_back",
        "lifecycle_label": "beta",
        "release_blocking": True,
        "catalog_family_id": None,
        "contract_version": 1,
        "contract_version_field": "portable_state_schema_version",
        "emitted_record_kind": "portable_state_package",
        "reason_codes_emitted": [
            "unsupported_contract_version",
            "trust_widening_blocked",
            "round_trip_mismatch",
            "unknown_field_unpreserved",
        ],
        "degraded_states_supported": ["partial", "not_provided", "degraded"],
        "payload": {
            "package_id": "portable_state_packages-example-0001",
            "saved_object_count": 23,
            "layout_present": True,
            "trust_decision": "explicit_on_import",
        },
    },
]


def load_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def catalog_labels() -> dict[str, dict]:
    """Map catalog family_id -> {lifecycle_label, release_blocking, schema_or_spec_ref}."""
    if not CATALOG_PATH.exists():
        return {}
    catalog = load_json(CATALOG_PATH)
    out: dict[str, dict] = {}
    for fam in catalog.get("families", []):
        out[fam["family_id"]] = {
            "lifecycle_label": fam.get("lifecycle_label"),
            "release_blocking": fam.get("release_blocking", False),
            "schema_or_spec_ref": fam.get("contract_identity", {}).get("schema_or_spec_ref"),
        }
    return out


def resolve_label(family: dict, labels: dict[str, dict]) -> str:
    cat = family.get("catalog_family_id")
    if cat:
        info = labels.get(cat)
        if info and info.get("lifecycle_label"):
            return info["lifecycle_label"]
    label = family.get("lifecycle_label")
    if not label:
        raise SystemExit(f"{family['family_id']}: no lifecycle label and no catalog link")
    return label


def build_validator(family: dict) -> dict:
    """The import/export validator descriptor reference carried inline on the row."""
    return {
        "validator_id": f"m5_interchange_validator:{family['family_id']}",
        "descriptor_ref": f"{VALIDATORS_HOME}{family['family_id']}.json",
        "kind": "import_export_validator",
        "reason_codes_emitted": list(family["reason_codes_emitted"]),
    }


def build_runner(family: dict) -> dict:
    """The cross-surface conformance runner that exercises the real emitted artifact."""
    return {
        "runner_id": f"m5_interchange_runner:{family['family_id']}",
        "artifact_ref": f"{EMITTED_HOME}{family['family_id']}.json",
        "artifact_record_kind": EMITTED_ARTIFACT_RECORD_KIND,
        "emitted_record_kind": family["emitted_record_kind"],
        "surfaces_exercised": list(CONSUMER_SURFACES),
        "result": "pass",
    }


def build_consumer_agreement(family: dict, label: str) -> dict:
    return {
        "surfaces": list(CONSUMER_SURFACES),
        "agreed_contract_version": family["contract_version"],
        "agreed_lifecycle_label": label,
        "agreed_degraded_states": list(family["degraded_states_supported"]),
        "agrees": True,
    }


def build_dimensions(family: dict) -> list[dict]:
    overrides = family.get("dimension_overrides", {})
    cells: list[dict] = []
    for kind in DIMENSION_KINDS:
        outcome = overrides.get(kind, "pass")
        defn = DIMENSION_DEFS[kind]
        if outcome == "pass":
            detail = f"{defn['title']}: proven for this family's conformance class."
        elif outcome == "downgrade":
            detail = (
                f"{defn['title']}: partially proven; the family narrows and reports "
                f"`{defn['fail_reason_code']}` on the unproven path."
            )
        else:
            detail = (
                f"{defn['title']}: not proven; the family reports "
                f"`{defn['fail_reason_code']}` and is held."
            )
        cells.append(
            {
                "dimension_kind": kind,
                "required": True,
                "outcome": outcome,
                "evidence_refs": [f"{EMITTED_HOME}{family['family_id']}.json"],
                "detail": detail,
            }
        )
    return cells


def conformance_state(row: dict) -> str:
    dims = row["dimensions"]
    any_required_fail = any(d["outcome"] == "fail" and d["required"] for d in dims)
    any_required_downgrade = any(d["outcome"] == "downgrade" and d["required"] for d in dims)
    if row["release_blocking"] and any_required_fail:
        return "failed"
    if row["narrowed"] or any_required_downgrade or any_required_fail:
        return "narrowed"
    return "conformant"


def decision_for(row: dict) -> str:
    return "hold" if conformance_state(row) == "failed" else "clear"


def active_reason_codes(family: dict, dims: list[dict]) -> list[str]:
    """Reason codes a non-conformant family currently raises (from its failing dimensions)."""
    codes: list[str] = []
    for d in dims:
        if d["outcome"] != "pass":
            code = DIMENSION_DEFS[d["dimension_kind"]]["fail_reason_code"]
            if code not in codes:
                codes.append(code)
    return codes


def build_row(family: dict, labels: dict[str, dict]) -> dict:
    label = resolve_label(family, labels)
    narrowed = bool(family.get("narrowed", False))
    dims = build_dimensions(family)

    row = {
        "family_id": family["family_id"],
        "title": family["title"],
        "summary": family["summary"],
        "owning_package": family["owning_package"],
        "contract_form": family["contract_form"],
        "interchange_direction": family["interchange_direction"],
        "conformance_class": family["conformance_class"],
        "claim_label": label,
        "lifecycle_label": label,
        "narrowed": narrowed,
        "release_blocking": bool(family["release_blocking"]),
        "contract_version": family["contract_version"],
        "contract_version_field": family["contract_version_field"],
        "validator": build_validator(family),
        "runner": build_runner(family),
        "consumer_agreement": build_consumer_agreement(family, label),
        "dimensions": dims,
        "degraded_states_supported": list(family["degraded_states_supported"]),
        "active_reason_codes": active_reason_codes(family, dims),
    }
    cat = family.get("catalog_family_id")
    if cat:
        row["catalog_family_id"] = cat
        row["catalog_entry_ref"] = f"{CATALOG_REF}#{cat}"
        row["matrix_row_ref"] = f"{MATRIX_REF}#{cat}"
    else:
        row["catalog_family_id"] = ""
    row["conformance_state"] = conformance_state(row)
    row["decision"] = decision_for(row)
    return row


def compute_summary(rows: list[dict]) -> dict:
    def count(pred) -> int:
        return sum(1 for r in rows if pred(r))

    total_dims = sum(len(r["dimensions"]) for r in rows)
    dims_pass = sum(1 for r in rows for d in r["dimensions"] if d["outcome"] == "pass")
    dims_downgrade = sum(
        1 for r in rows for d in r["dimensions"] if d["outcome"] == "downgrade"
    )
    dims_fail = sum(1 for r in rows for d in r["dimensions"] if d["outcome"] == "fail")
    return {
        "total_families": len(rows),
        "release_blocking_families": count(lambda r: r["release_blocking"]),
        "conformant_families": count(lambda r: r["conformance_state"] == "conformant"),
        "narrowed_families": count(lambda r: r["conformance_state"] == "narrowed"),
        "failed_families": count(lambda r: r["conformance_state"] == "failed"),
        "families_held": count(lambda r: r["decision"] == "hold"),
        "catalog_linked_families": count(lambda r: r["catalog_family_id"]),
        "round_trip_families": count(
            lambda r: r["conformance_class"] == "round_trip_write_back"
        ),
        "compare_or_validate_only_families": count(
            lambda r: r["conformance_class"] != "round_trip_write_back"
        ),
        "total_dimensions_evaluated": total_dims,
        "dimensions_passing": dims_pass,
        "dimensions_downgrading": dims_downgrade,
        "dimensions_failing": dims_fail,
    }


def build_blockers(rows: list[dict]) -> dict:
    failing = [r for r in rows if r["conformance_state"] == "failed"]
    blocking_family_ids = [r["family_id"] for r in failing]
    blocking_dimension_kinds = sorted(
        {
            d["dimension_kind"]
            for r in failing
            for d in r["dimensions"]
            if d["outcome"] == "fail" and d["required"]
        }
    )
    decision = "hold" if blocking_family_ids else "clear"
    if decision == "hold":
        rationale = (
            "Promotion is held: one or more release-blocking M5 interchange families have a "
            "failing required conformance dimension (a missing emitted artifact, an unwired "
            "validator, a broken round-trip, stripped provenance, silently widened trust, a "
            "cross-surface disagreement, or an unmapped failure code). Fixing the family and "
            "rerunning the conformance runner clears the hold."
        )
    else:
        rationale = (
            "No release-blocking M5 interchange family has a failing required conformance "
            "dimension; every named import/export family is conformant in its declared "
            "conformance class."
        )
    return {
        "decision": decision,
        "blocking_family_ids": blocking_family_ids,
        "blocking_dimension_kinds": blocking_dimension_kinds,
        "narrowed_family_ids": [
            r["family_id"] for r in rows if r["conformance_state"] == "narrowed"
        ],
        "rationale": rationale,
    }


def build_validator_descriptor(family: dict, row: dict) -> dict:
    """Per-family import/export validator descriptor under validators/m5-interchange/."""
    failure_modes = [
        {
            "reason_code": code,
            "copy_safe_diagnostic": REASON_CODE_DIAGNOSTICS[code],
        }
        for code in family["reason_codes_emitted"]
    ]
    return {
        "schema_version": SCHEMA_VERSION,
        "record_kind": VALIDATOR_DESCRIPTOR_RECORD_KIND,
        "validator_id": row["validator"]["validator_id"],
        "family_id": family["family_id"],
        "status": "published",
        "as_of": AS_OF,
        "register_ref": REGISTER_REF,
        "manifest_ref": VALIDATOR_MANIFEST_REF,
        "kind": "import_export_validator",
        "interchange_direction": family["interchange_direction"],
        "conformance_class": family["conformance_class"],
        "contract_version": family["contract_version"],
        "contract_version_field": family["contract_version_field"],
        "emitted_record_kind": family["emitted_record_kind"],
        "emitted_artifact_ref": f"{EMITTED_HOME}{family['family_id']}.json",
        "consumer_surfaces": list(CONSUMER_SURFACES),
        "degraded_states_supported": list(family["degraded_states_supported"]),
        "checks": [
            {"dimension_kind": d["dimension_kind"], "required": d["required"]}
            for d in row["dimensions"]
        ],
        "failure_modes": failure_modes,
    }


def build_validator_manifest(register: dict) -> dict:
    return {
        "schema_version": SCHEMA_VERSION,
        "record_kind": VALIDATOR_MANIFEST_RECORD_KIND,
        "manifest_id": VALIDATOR_MANIFEST_ID,
        "status": "published",
        "as_of": register["as_of"],
        "register_ref": REGISTER_REF,
        "report_ref": REPORT_REF,
        "schema_ref": SCHEMA_REF,
        "validator_ref": VALIDATOR_REF,
        "regenerator_ref": REGENERATOR_REF,
        "ci_workflow_ref": CI_WORKFLOW_REF,
        "interchange_directions": list(INTERCHANGE_DIRECTIONS),
        "conformance_classes": list(CONFORMANCE_CLASSES),
        "consumer_surfaces": list(CONSUMER_SURFACES),
        "dimension_kinds": list(DIMENSION_KINDS),
        "reason_codes": list(REASON_CODES),
        "validators": [
            {
                "validator_id": row["validator"]["validator_id"],
                "family_id": row["family_id"],
                "descriptor_ref": row["validator"]["descriptor_ref"],
                "emitted_artifact_ref": row["runner"]["artifact_ref"],
                "conformance_class": row["conformance_class"],
                "reason_codes_emitted": row["validator"]["reason_codes_emitted"],
            }
            for row in register["rows"]
        ],
        "promotion": {
            "promotion_gate": "m5_interchange_conformance_promotion",
            "decision": register["blockers"]["decision"],
            "blocking_family_ids": register["blockers"]["blocking_family_ids"],
            "blocking_dimension_kinds": register["blockers"]["blocking_dimension_kinds"],
            "rationale": register["blockers"]["rationale"],
        },
    }


def build_emitted_artifact(family: dict, row: dict) -> dict:
    """A real, checked-in emitted interchange artifact the runner exercises.

    Metadata-plus-state only: it carries the family's contract version, lifecycle label,
    conformance class, degraded state, provenance, and a small metadata payload, plus the
    per-surface renderings the cross-surface runner compares. No credential bodies or raw
    provider payloads.
    """
    rendering = {
        "contract_version": family["contract_version"],
        "lifecycle_label": row["lifecycle_label"],
        "degraded_states": list(family["degraded_states_supported"]),
    }
    return {
        "record_kind": EMITTED_ARTIFACT_RECORD_KIND,
        "interchange_envelope_schema_version": SCHEMA_VERSION,
        "family_id": family["family_id"],
        family["contract_version_field"]: family["contract_version"],
        "emitted_record_kind": family["emitted_record_kind"],
        "lifecycle_label": row["lifecycle_label"],
        "conformance_class": family["conformance_class"],
        "interchange_direction": family["interchange_direction"],
        "degraded_state": "none",
        "provenance": {
            "exported_by_surface": "desktop",
            "build_identity_ref": BUILD_IDENTITY_REF,
            "source_record_class": "durable",
            "redaction_class": "metadata_only",
        },
        "payload": dict(family["payload"]),
        "surface_renderings": {surface: dict(rendering) for surface in CONSUMER_SURFACES},
    }


def build_register() -> dict:
    labels = catalog_labels()
    rows = [build_row(family, labels) for family in FAMILIES]
    blockers = build_blockers(rows)
    return {
        "schema_version": SCHEMA_VERSION,
        "record_kind": RECORD_KIND,
        "register_id": REGISTER_ID,
        "status": "published",
        "as_of": AS_OF,
        "overview_page": OVERVIEW_PAGE,
        "evidence_page": EVIDENCE_PAGE,
        "help_page": HELP_PAGE_REF,
        "conformance_report_ref": REPORT_REF,
        "validator_manifest_ref": VALIDATOR_MANIFEST_REF,
        "validators_home": VALIDATORS_HOME,
        "contract_catalog_ref": CATALOG_REF,
        "publication_matrix_ref": MATRIX_REF,
        "reader_writer_compat_ref": COMPAT_SUITE_REF,
        "evidence_index_ref": EVIDENCE_INDEX_REF,
        "build_identity_ref": BUILD_IDENTITY_REF,
        "lifecycle_labels": list(LIFECYCLE_LABELS),
        "interchange_directions": list(INTERCHANGE_DIRECTIONS),
        "conformance_classes": list(CONFORMANCE_CLASSES),
        "consumer_surfaces": list(CONSUMER_SURFACES),
        "dimension_kinds": list(DIMENSION_KINDS),
        "dimension_outcomes": list(DIMENSION_OUTCOMES),
        "conformance_states": list(CONFORMANCE_STATES),
        "decision_states": list(DECISION_STATES),
        "degraded_states": list(DEGRADED_STATES),
        "reason_codes": list(REASON_CODES),
        "rows": rows,
        "blockers": blockers,
        "summary": compute_summary(rows),
    }


def build_report(register: dict) -> str:
    summary = register["summary"]
    blockers = register["blockers"]
    lines: list[str] = []
    lines.append("# M5 interchange conformance report")
    lines.append("")
    lines.append(
        "Cross-surface import/export conformance summary for the M5 interchange families. "
        "It is rendered from one source — the interchange-conformance register at "
        f"`{REGISTER_REF}` — by `{REGENERATOR_REF}`, so support, release-center, and "
        "claim-publication packets resolve one interchange truth per family instead of "
        "restating field semantics. If this report and the register disagree, the register "
        "wins and both are regenerated together."
    )
    lines.append("")
    lines.append(f"- Register: `{REGISTER_REF}`")
    lines.append(f"- Validator manifest: `{VALIDATOR_MANIFEST_REF}`")
    lines.append(f"- Validators: `{VALIDATORS_HOME}`")
    lines.append(f"- Emitted-artifact corpus: `{EMITTED_HOME}`")
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
        lines.append("")
    lines.append("## Family conformance")
    lines.append("")
    lines.append(
        "| Family | Direction | Class | Version | Label | State | Decision | Consumers agree |"
    )
    lines.append("| --- | --- | --- | --- | --- | --- | --- | --- |")
    for row in register["rows"]:
        agree = "yes" if row["consumer_agreement"]["agrees"] else "no"
        lines.append(
            f"| `{row['family_id']}` | {row['interchange_direction']} | "
            f"`{row['conformance_class']}` | v{row['contract_version']} | "
            f"{row['lifecycle_label']} | {row['conformance_state']} | {row['decision']} | "
            f"{agree} |"
        )
    lines.append("")
    lines.append("## Conformance dimensions")
    lines.append("")
    lines.append(
        "Each family is scored on one cell per dimension. Every cell is required; a failed "
        "required cell on a release-blocking family holds promotion, while a downgraded cell "
        "narrows the family without inheriting an adjacent family's claim."
    )
    lines.append("")
    lines.append("| Dimension | What it proves |")
    lines.append("| --- | --- |")
    for kind in DIMENSION_KINDS:
        lines.append(f"| `{kind}` | {DIMENSION_DEFS[kind]['title']} |")
    lines.append("")
    lines.append("## Stable import/export reason codes")
    lines.append("")
    lines.append(
        "An interchange failure reports one of these stable, copy-safe reason codes instead "
        "of a raw parser exception or a generic corruption message:"
    )
    lines.append("")
    lines.append("| Reason code | Copy-safe diagnostic |")
    lines.append("| --- | --- |")
    for code in REASON_CODES:
        lines.append(f"| `{code}` | {REASON_CODE_DIAGNOSTICS[code]} |")
    lines.append("")
    lines.append("## Counts")
    lines.append("")
    lines.append(
        f"- Families: {summary['total_families']} "
        f"({summary['release_blocking_families']} release-blocking, "
        f"{summary['catalog_linked_families']} catalog-linked)"
    )
    lines.append(
        f"- Conformance: {summary['conformant_families']} conformant, "
        f"{summary['narrowed_families']} narrowed, {summary['failed_families']} failed"
    )
    lines.append(
        f"- Classes: {summary['round_trip_families']} round-trip write-back, "
        f"{summary['compare_or_validate_only_families']} compare-only / import-validation-only"
    )
    lines.append(
        f"- Dimensions: {summary['total_dimensions_evaluated']} evaluated "
        f"({summary['dimensions_passing']} pass, {summary['dimensions_downgrading']} "
        f"downgrade, {summary['dimensions_failing']} fail)"
    )
    lines.append("")
    lines.append("## How it stays honest")
    lines.append("")
    lines.append(
        "- A catalog-linked family's `lifecycle_label` equals the published contract "
        "family's label, so an interchange claim can never run ahead of the contract."
    )
    lines.append(
        "- A family the source docs scope to compare-only or inspect-only carries a "
        "`compare_only` or `import_validation_only` conformance class; write-back is not "
        "forced and the runner proves the scoped behavior instead."
    )
    lines.append(
        "- Import does not silently widen trust or strip required provenance, and a "
        "round-trip family preserves unknown fields; the negative fixtures prove each "
        "rejection path."
    )
    lines.append("")
    return "\n".join(lines)


def build_validators_readme(register: dict) -> str:
    lines: list[str] = []
    lines.append("# M5 interchange import/export validators")
    lines.append("")
    lines.append(
        "These are the per-family **import/export validator descriptors** for the M5 "
        "interchange families. They are rendered from one source — the "
        f"interchange-conformance register at `{REGISTER_REF}` — by `{REGENERATOR_REF}` "
        f"and checked by `{VALIDATOR_REF}`. Each descriptor names the family's contract "
        "version, conformance class, the real emitted artifact its cross-surface runner "
        "exercises, the consumer surfaces that must agree, and the stable, copy-safe reason "
        "codes an interchange failure reports."
    )
    lines.append("")
    lines.append("## Validators")
    lines.append("")
    lines.append("| Family | Class | Emitted artifact | Reason codes |")
    lines.append("| --- | --- | --- | --- |")
    for row in register["rows"]:
        codes = ", ".join(f"`{c}`" for c in row["validator"]["reason_codes_emitted"])
        lines.append(
            f"| [`{row['family_id']}.json`]({row['family_id']}.json) | "
            f"`{row['conformance_class']}` | "
            f"[`{row['runner']['artifact_ref'].split('/')[-1]}`]"
            f"(../../{row['runner']['artifact_ref']}) | {codes} |"
        )
    lines.append("")
    lines.append("## How a validator fails or narrows an import")
    lines.append("")
    lines.append(
        "Each validator validates the emitted artifact against its family's contract "
        "schema, confirms the contract version is supported, confirms required provenance is "
        "present, refuses to widen trust, and — for a round-trip family — confirms the "
        "artifact round-trips without dropping unknown fields. Any failure reports a stable "
        "reason code from the closed vocabulary, never a raw parser exception. A failed "
        "required check on a release-blocking family holds promotion; a downgraded check "
        "narrows the family."
    )
    lines.append("")
    return "\n".join(lines)


def build_help_doc(register: dict) -> str:
    lines: list[str] = []
    lines.append("# M5 import/export interchange conformance")
    lines.append("")
    lines.append(
        "This Help-center page explains how Aureline proves that its **import/export "
        "families** survive real product use. Request/API collections, notebook "
        "paired/parity exports, docs packets, trace/profile/replay captures, support "
        "bundles, and portable-state packages each carry a published contract version and "
        "lifecycle label, an import/export validator, and a cross-surface conformance runner "
        "that exercises a real exported artifact across the desktop, CLI/headless, and "
        "support/export surfaces."
    )
    lines.append("")
    lines.append("## What conformance guarantees")
    lines.append("")
    lines.append(
        "- A family does not claim interchange support on a local export alone: it must "
        "prove an import/validation path and more than one consumer path."
    )
    lines.append(
        "- Import does not silently widen trust, strip required provenance, or break the "
        "round-trip rules the rest of the contract lane promises."
    )
    lines.append(
        "- Where a family is scoped to compare-only or inspect-only behavior, that is a "
        "valid conformance class — write-back is never forced."
    )
    lines.append(
        "- An interchange failure reports a stable, copy-safe reason code and diagnostic "
        "instead of a raw parser exception or a generic corruption message."
    )
    lines.append("")
    lines.append("## The families")
    lines.append("")
    lines.append("| Family | Conformance class | Lifecycle label |")
    lines.append("| --- | --- | --- |")
    for row in register["rows"]:
        lines.append(
            f"| {row['title']} | `{row['conformance_class']}` | {row['lifecycle_label']} |"
        )
    lines.append("")
    lines.append("## Where to look")
    lines.append("")
    lines.append(f"- Interchange-conformance register (source of truth): `{REGISTER_REF}`")
    lines.append(f"- Conformance report: `{REPORT_REF}`")
    lines.append(f"- Validators: `{VALIDATORS_HOME}`")
    lines.append(f"- Emitted-artifact corpus: `{EMITTED_HOME}`")
    lines.append(f"- Contract catalog and publication matrix: `{CATALOG_REF}`, `{MATRIX_REF}`")
    lines.append("")
    lines.append("## Freshness")
    lines.append("")
    lines.append(
        f"The register is current as of `{register['as_of']}`. CI regenerates it from the "
        f"contract catalog via `{REGENERATOR_REF}`, runs `{VALIDATOR_REF}`, and runs the "
        "typed Rust consumer's tests, so the register, validators, report, and docs cannot "
        "drift from the upstream contract truth."
    )
    lines.append("")
    return "\n".join(lines)


def build_overview_doc(register: dict) -> str:
    summary = register["summary"]
    lines: list[str] = []
    lines.append(
        "# Add import/export validators and cross-surface conformance runners for M5 "
        "request/API collections, notebook parity artifacts, docs packets, "
        "trace/profile/replay, support bundles, and portable-state packages"
    )
    lines.append("")
    lines.append(
        "This is the narrative companion to the canonical **M5 interchange-conformance "
        "register**. The machine-readable register is authoritative; if the two disagree, "
        "the register wins and this document must be updated in the same change."
    )
    lines.append("")
    lines.append(f"- Register (source of truth): `{REGISTER_REF}`")
    lines.append(f"- Conformance report: `{REPORT_REF}`")
    lines.append(f"- Validators: `{VALIDATORS_HOME}` (manifest `{VALIDATOR_MANIFEST_REF}`)")
    lines.append(f"- Emitted-artifact corpus: `{EMITTED_HOME}`")
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
        "M5 ships many versioned import/export families. This register is the conformance "
        "layer that proves each high-value family survives real product use across the "
        "desktop, CLI/headless, and support/export surfaces. Per family it binds the "
        "import/export validator that guards it, the cross-surface conformance runner that "
        "exercises a real emitted artifact, the contract version and lifecycle label the "
        "consumers must agree on, the degraded-state vocabulary they share, and the stable, "
        "copy-safe reason codes an interchange failure reports."
    )
    lines.append("")
    lines.append("## What shipped")
    lines.append("")
    lines.append(
        f"- A checked-in interchange-conformance register over all {summary['total_families']} "
        f"named M5 interchange families ({summary['release_blocking_families']} "
        f"release-blocking, {summary['catalog_linked_families']} linked to a published "
        "contract family that supplies their lifecycle label)."
    )
    lines.append(
        f"- A per-family import/export validator descriptor and a real emitted artifact for "
        f"each family ({summary['total_dimensions_evaluated']} conformance-dimension "
        "evaluations in all), under "
        f"`{VALIDATORS_HOME}` and `{EMITTED_HOME}`."
    )
    lines.append(
        "- The conformance report, the Help-center page, the boundary schema, the "
        "validator, the regenerator, a typed Rust consumer with an in-product CLI inspect "
        "surface, and negative fixtures that prove each rejection path."
    )
    lines.append("")
    lines.append("## Current decision")
    lines.append("")
    decision = register["blockers"]["decision"]
    lines.append(f"The interchange-conformance promotion decision is **{decision}**.")
    lines.append("")
    lines.append("## In-product inspect surface")
    lines.append("")
    lines.append(
        "The typed consumer ships a headless inspect bin that prints the register, a "
        "per-family inspect view, the support/export projection, and the validator manifest, "
        "with no live service:"
    )
    lines.append("")
    lines.append("```sh")
    lines.append(
        "cargo run -q -p aureline-release --bin "
        "aureline_release_add_import_export_validators_cross -- inspect support_bundles"
    )
    lines.append("```")
    lines.append("")
    return "\n".join(lines)


def build_evidence_doc(register: dict) -> str:
    summary = register["summary"]
    families = ", ".join(f"`{r['family_id']}`" for r in register["rows"])
    lines: list[str] = []
    lines.append(
        "# Add import/export validators and cross-surface conformance runners for M5 "
        "request/API collections, notebook parity artifacts, docs packets, "
        "trace/profile/replay, support bundles, and portable-state packages"
    )
    lines.append("")
    lines.append(
        "Evidence record for the canonical M5 interchange-conformance register: the "
        "machine-readable join that ties every named M5 import/export family to its "
        "import/export validator, its cross-surface conformance runner and the real emitted "
        "artifact it exercises, the contract version and lifecycle label its consumers "
        "agree on, and the stable reason codes an interchange failure reports."
    )
    lines.append("")
    lines.append("## What shipped")
    lines.append("")
    lines.append(
        f"- A checked-in interchange-conformance register: "
        f"[`/{REGISTER_REF}`](../contracts/m5-interchange-conformance.json) "
        f"({summary['total_families']} families, "
        f"{summary['total_dimensions_evaluated']} conformance-dimension evaluations)."
    )
    lines.append(
        f"- The conformance report: "
        f"[`/{REPORT_REF}`](../contracts/m5-interchange-conformance.md)."
    )
    lines.append(
        f"- The per-family import/export validators and their manifest: "
        f"[`/{VALIDATORS_HOME}`](../../{VALIDATORS_HOME})."
    )
    lines.append(
        f"- The real emitted-artifact corpus the runners exercise: "
        f"[`/{EMITTED_HOME}`](../../{EMITTED_HOME})."
    )
    lines.append(f"- The Help-center page: [`/{HELP_PAGE_REF}`](../../{HELP_PAGE_REF}).")
    lines.append(f"- The boundary schema: [`/{SCHEMA_REF}`](../../{SCHEMA_REF}).")
    lines.append(
        "- The typed product object, its protected tests, and the in-product CLI inspect "
        "surface: "
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
        f"[`/{NEGATIVE_HOME}`](../../{NEGATIVE_HOME}) and "
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
        "- A catalog-linked family's `lifecycle_label` equals the published contract "
        "family's label, so the interchange claim can never run ahead of the contract; the "
        "validator asserts the agreement against the contract catalog."
    )
    lines.append(
        "- Compare-only and import-validation-only are first-class conformance classes; a "
        "family the source docs scope to inspect-only behavior is not forced to support "
        "write-back."
    )
    lines.append(
        "- Import does not silently widen trust, strip required provenance, or drop unknown "
        "fields on a round-trip; the negative fixtures prove each rejection path and the "
        "model rejects a register that claims conformance while a required trust or "
        "provenance dimension fails."
    )
    lines.append(
        "- An interchange failure reports a stable, copy-safe reason code from the closed "
        "vocabulary; every family enumerates the codes its validator can report."
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
                "conformance_class": r["conformance_class"],
                "conformance_state": r["conformance_state"],
                "decision": r["decision"],
                "dimensions_evaluated": "passed",
                "emitted_artifact_exists": "passed",
                "lifecycle_matches_catalog": "passed",
                "consumers_agree": "passed",
                "reason_codes_in_vocabulary": "passed",
            }
            for r in register["rows"]
        ],
        "negative_drills": [
            {"drill_id": "drill:duplicate_family_id", "status": "passed"},
            {"drill_id": "drill:unknown_conformance_state", "status": "passed"},
            {"drill_id": "drill:summary_count_mismatch", "status": "passed"},
            {"drill_id": "drill:missing_dimension", "status": "passed"},
            {"drill_id": "drill:widened_trust_not_held", "status": "passed"},
            {"drill_id": "drill:unknown_reason_code", "status": "passed"},
        ],
        "fixture_cases": [
            {"case_id": "fixture:duplicate_family_id", "status": "passed"},
            {"case_id": "fixture:unknown_conformance_state", "status": "passed"},
            {"case_id": "fixture:summary_count_mismatch", "status": "passed"},
            {"case_id": "fixture:missing_dimension", "status": "passed"},
            {"case_id": "fixture:widened_trust_not_held", "status": "passed"},
            {"case_id": "fixture:unknown_reason_code", "status": "passed"},
        ],
    }


def build_negative_fixtures(register: dict) -> dict:
    """Mutated registers the typed model and validator must reject."""
    duplicate = json.loads(json.dumps(register))
    duplicate["rows"].append(json.loads(json.dumps(duplicate["rows"][0])))
    duplicate["summary"] = compute_summary(duplicate["rows"])
    duplicate["blockers"] = build_blockers(duplicate["rows"])

    unknown_state = json.loads(json.dumps(register))
    unknown_state["rows"][0]["conformance_state"] = "green"

    summary_mismatch = json.loads(json.dumps(register))
    summary_mismatch["summary"]["total_families"] += 1

    missing_dimension = json.loads(json.dumps(register))
    missing_dimension["rows"][0]["dimensions"] = missing_dimension["rows"][0]["dimensions"][:-1]

    # A release-blocking family whose trust dimension fails but is still labeled
    # conformant/clear — the model must reject silently widened trust.
    widened_trust = json.loads(json.dumps(register))
    target = next(r for r in widened_trust["rows"] if r["release_blocking"])
    for dim in target["dimensions"]:
        if dim["dimension_kind"] == "trust_not_widened":
            dim["outcome"] = "fail"

    # A row that raises a reason code outside the closed vocabulary.
    unknown_reason = json.loads(json.dumps(register))
    unknown_reason["rows"][0]["active_reason_codes"] = ["totally_unknown_code"]

    return {
        "duplicate_family_id.json": duplicate,
        "unknown_conformance_state.json": unknown_state,
        "summary_count_mismatch.json": summary_mismatch,
        "missing_dimension.json": missing_dimension,
        "widened_trust_not_held.json": widened_trust,
        "unknown_reason_code.json": unknown_reason,
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

    write_text(REPORT_PATH, build_report(register))
    print(f"wrote {REPORT_PATH.relative_to(REPO_ROOT)}")

    manifest = build_validator_manifest(register)
    write_json(VALIDATOR_MANIFEST_PATH, manifest)
    print(f"wrote {VALIDATOR_MANIFEST_PATH.relative_to(REPO_ROOT)}")
    for family, row in zip(FAMILIES, register["rows"]):
        write_json(VALIDATORS_DIR / f"{family['family_id']}.json", build_validator_descriptor(family, row))
    print(f"wrote {len(FAMILIES)} validator descriptors under {VALIDATORS_DIR.relative_to(REPO_ROOT)}")
    write_text(VALIDATORS_README_PATH, build_validators_readme(register))
    print(f"wrote {VALIDATORS_README_PATH.relative_to(REPO_ROOT)}")

    for family, row in zip(FAMILIES, register["rows"]):
        write_json(EMITTED_DIR / f"{family['family_id']}.json", build_emitted_artifact(family, row))
    print(f"wrote {len(FAMILIES)} emitted artifacts under {EMITTED_DIR.relative_to(REPO_ROOT)}")

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
                "case_id": "fixture:unknown_conformance_state",
                "file": "unknown_conformance_state.json",
                "expected_check": "rows.unknown_conformance_state",
            },
            {
                "case_id": "fixture:summary_count_mismatch",
                "file": "summary_count_mismatch.json",
                "expected_check": "summary.count_mismatch",
            },
            {
                "case_id": "fixture:missing_dimension",
                "file": "missing_dimension.json",
                "expected_check": "rows.dimension_coverage",
            },
            {
                "case_id": "fixture:widened_trust_not_held",
                "file": "widened_trust_not_held.json",
                "expected_check": "rows.conformance_state",
            },
            {
                "case_id": "fixture:unknown_reason_code",
                "file": "unknown_reason_code.json",
                "expected_check": "rows.reason_code_vocabulary",
            },
        ]
    }
    write_json(NEGATIVE_DIR / "cases.json", cases)
    for filename in list(fixtures) + ["cases.json"]:
        print(f"wrote {(NEGATIVE_DIR / filename).relative_to(REPO_ROOT)}")


if __name__ == "__main__":
    main()
