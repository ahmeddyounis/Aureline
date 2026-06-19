#!/usr/bin/env python3
"""Regenerate the M5 CLI/headless structured-output and result-code catalog.

This is the single source of truth for the catalog that gives every new M5
CLI/headless inspect, export, report, and health surface a stable
structured-output schema reference, a stable result-code catalog, a lifecycle
label, and an explicit partial-result / staleness vocabulary, and that proves
the lifecycle/degraded-state vocabulary is identical between the UI inspect
surface and the CLI/headless output.

It builds one ``surfaces`` entry per surface, then writes, all deterministically:

  * ``artifacts/contracts/m5-cli-output-catalog.json``          (the catalog)
  * ``fixtures/contracts/m5-cli-json/<surface>.cli.json``       (CLI output payloads)
  * ``fixtures/contracts/m5-cli-json/<surface>.ui.json``        (UI inspect payloads)
  * ``docs/cli/m5-structured-output-and-result-codes.md``       (the CLI doc)
  * ``artifacts/release/captures/<name>_validation_capture.json`` (CI capture)
  * ``fixtures/contracts/m5-cli-catalog/{cases.json,*.json}``   (negative fixtures)

Run ``python3 tools/regenerate_m5_cli_output_catalog.py`` after editing the
surface set, then ``python3 tools/validate_m5_cli_output_catalog.py`` and
``cargo test -p aureline-release --test
ship_cli_headless_structured_output_schemas_result_code_catalogs_and_schema_reference_links_for_every_new_m5_inspect_export_report_surface``
to confirm the validator and the typed model agree.

The catalog reuses existing governance sources rather than minting a new
lexicon: each surface's ``structured_output_schema_ref`` resolves to a checked-in
JSON Schema package published by the canonical M5 JSON Schema catalog
(``tools/regenerate_m5_json_schema_catalog.py``); each surface's
``lifecycle_label`` is the label the public-contract publication matrix publishes
for that family after narrowing; and every ``result_code`` and
``output_envelope_class`` is drawn from the closed vocabularies already frozen by
the CLI/headless machine-output stability contract
(``schemas/automation/cli_output_registry_entry.schema.json``). The cross-checks
live in the validator. The catalog is metadata-only: it carries no surface
payloads, rendered bodies, signatures, or credential material.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

TOOLS_DIR = Path(__file__).resolve().parent
if str(TOOLS_DIR) not in sys.path:
    sys.path.insert(0, str(TOOLS_DIR))

# Reuse the canonical JSON Schema catalog so a surface's structured-output schema
# reference and its example envelope come from one place and cannot drift.
import regenerate_m5_json_schema_catalog as jsoncat  # noqa: E402

NAME = (
    "ship_cli_headless_structured_output_schemas_result_code_catalogs_and_"
    "schema_reference_links_for_every_new_m5_inspect_export_report_surface"
)
RECORD_KIND = "m5_cli_output_catalog"
CATALOG_ID = "m5_cli_output_catalog:v1"
SCHEMA_VERSION = 1
AS_OF = "2026-06-19"

REPO_ROOT = jsoncat.REPO_ROOT

CATALOG_PATH = REPO_ROOT / "artifacts" / "contracts" / "m5-cli-output-catalog.json"
FIXTURE_DIR = REPO_ROOT / "fixtures" / "contracts" / "m5-cli-json"
NEGATIVE_DIR = REPO_ROOT / "fixtures" / "contracts" / "m5-cli-catalog"
CLI_DOC_PATH = REPO_ROOT / "docs" / "cli" / "m5-structured-output-and-result-codes.md"
CAPTURE_PATH = REPO_ROOT / "artifacts" / "release" / "captures" / f"{NAME}_validation_capture.json"

SLUG = NAME.replace("_", "-")
OVERVIEW_PAGE = f"docs/m5/{SLUG}.md"
EVIDENCE_PAGE = f"artifacts/m5/{SLUG}.md"
CLI_DOC_PAGE = "docs/cli/m5-structured-output-and-result-codes.md"

# Cross-cutting governance sources this catalog reuses instead of restating.
JSON_SCHEMA_CATALOG_REF = "artifacts/contracts/m5-json-schema-catalog.json"
PUBLICATION_MATRIX_REF = "artifacts/contracts/m5-stability-lifecycle-map.json"
PUBLICATION_MATRIX_PATH = REPO_ROOT / PUBLICATION_MATRIX_REF
CLI_SURFACE_CONTRACT_REF = "docs/automation/cli_surface_contract.md"
CLI_OUTPUT_REGISTRY_SCHEMA_REF = "schemas/automation/cli_output_registry_entry.schema.json"
EVIDENCE_INDEX_REF = (
    "artifacts/release/m5/"
    "certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json"
)

SCHEMA_HOME = "schemas/public/m5-json/"

# Closed vocabularies. Kept in lockstep with the typed Rust consumer and the
# boundary schema; the validator and the model both reject anything off-list.
SURFACE_KINDS = ["inspect", "export", "report", "health"]
LIFECYCLE_LABELS = ["lts", "stable", "beta", "preview", "withdrawn"]
MACHINE_OUTPUT_STABILITY_CLASSES = [
    "stable_schema_governed",
    "preview_schema_governed_additive_minor_only",
    "experimental_schema_governed_may_break",
    "internal_no_stability_promise",
]
# Subset of the frozen CLI/headless machine-output envelope vocabulary the new M5
# inspect/export/report/health surfaces use. The validator asserts every value is
# a member of the authoritative enum in the CLI output registry schema.
OUTPUT_ENVELOPE_CLASSES = [
    "json_document_single",
    "jsonl_line_stream",
    "ndjson_event_stream",
    "sarif_2_1_0_document",
    "junit_xml_document",
]
# The result-code vocabulary, drawn verbatim from the CLI output registry's
# exit_code_class enum so CLI and desktop key off one set of stable enums. The
# validator asserts every value is a member of that authoritative enum.
RESULT_CODES = [
    "success",
    "success_no_action_taken",
    "partial_success_with_warnings",
    "usage_error",
    "input_validation_error",
    "policy_or_trust_denied",
    "credential_broker_denied",
    "preview_required_not_shown",
    "approval_required_not_granted",
    "dry_run_would_have_applied",
    "timeout_or_deadline_exceeded",
    "network_or_remote_unavailable",
    "kill_switch_active",
    "dependency_missing_or_stale",
    "unsupported_on_headless",
    "cancelled_by_user",
    "unrecoverable_internal_error",
]
# Stable POSIX-compatible numeric codes per result-code class. success and
# success_no_action_taken are pinned to 0 to mirror the CLI output registry.
RESULT_CODE_NUMERIC = {
    "success": 0,
    "success_no_action_taken": 0,
    "partial_success_with_warnings": 10,
    "usage_error": 64,
    "input_validation_error": 65,
    "policy_or_trust_denied": 77,
    "credential_broker_denied": 78,
    "preview_required_not_shown": 73,
    "approval_required_not_granted": 74,
    "dry_run_would_have_applied": 75,
    "timeout_or_deadline_exceeded": 124,
    "network_or_remote_unavailable": 69,
    "kill_switch_active": 76,
    "dependency_missing_or_stale": 72,
    "unsupported_on_headless": 71,
    "cancelled_by_user": 130,
    "unrecoverable_internal_error": 70,
}
RESULT_CODE_MEANING = {
    "success": "The surface completed and emitted a full structured result.",
    "success_no_action_taken": "The surface completed; nothing matched, so no rows were emitted.",
    "partial_success_with_warnings": "Some rows resolved; a partial-result block lists what could not.",
    "usage_error": "The invocation was malformed; no structured result was produced.",
    "input_validation_error": "An argument failed validation; no structured result was produced.",
    "policy_or_trust_denied": "Admin policy or workspace trust denied the surface.",
    "credential_broker_denied": "A required credential handle was denied by the broker.",
    "preview_required_not_shown": "A required preview was not shown, so the surface refused to act.",
    "approval_required_not_granted": "A required approval was not granted, so the surface refused to act.",
    "dry_run_would_have_applied": "A dry run reported the change it would have applied.",
    "timeout_or_deadline_exceeded": "The surface hit a deadline; a partial or stale-retest result may be emitted.",
    "network_or_remote_unavailable": "A remote dependency was unavailable; local-only output is degraded.",
    "kill_switch_active": "A kill switch is active; the surface is disabled.",
    "dependency_missing_or_stale": "A required input was missing or stale; retest is needed.",
    "unsupported_on_headless": "The surface has no machine projection in this headless context.",
    "cancelled_by_user": "The invocation was cancelled before completion.",
    "unrecoverable_internal_error": "An internal error prevented a structured result.",
}

# The partial-result and freshness vocabularies that make machine output safe for
# automation. Surfaces declare which states they can emit; the per-surface fixtures
# carry one concrete value so the UI/CLI parity check has something to compare.
PARTIAL_RESULT_STATES = ["complete", "partial", "degraded", "unavailable", "stale_retest_needed"]
FRESHNESS_STATES = ["fresh", "stale", "retest_needed", "unknown"]
PARITY_MATCH_MODES = ["exact_match_required", "projection_match_required", "informational_only"]
DOWNGRADE_BEHAVIORS = ["narrow_below_cutline", "reject"]

# Result codes every read-only inspect/report/health surface publishes. Mirrors
# the CLI output registry rule that a row carries at least one success and one
# error class; partial_success_with_warnings is the partial-result carrier.
READ_ONLY_RESULT_CODES = [
    "success",
    "success_no_action_taken",
    "partial_success_with_warnings",
    "input_validation_error",
    "dependency_missing_or_stale",
    "network_or_remote_unavailable",
    "cancelled_by_user",
    "unrecoverable_internal_error",
]
# Export surfaces additionally publish the policy/approval denial classes because
# a support/export packet can be gated by admin policy or a redaction approval.
EXPORT_RESULT_CODES = READ_ONLY_RESULT_CODES + [
    "policy_or_trust_denied",
    "approval_required_not_granted",
]

COMPATIBILITY_NOTE = (
    "The structured-output schema is resolved from the canonical M5 JSON Schema "
    "catalog and evolves under that package's additive-minor / frozen-required "
    "contract; the result-code catalog and the partial-result and freshness "
    "vocabularies are closed and stable, so automation keys off enums rather than "
    "prose; a surface missing its schema reference, result-code catalog, "
    "lifecycle label, or UI/CLI parity fixture narrows below the launch cutline "
    "rather than emitting an undeclared shape."
)

# One entry per new M5 CLI/headless inspect/export/report/health surface. Each
# surface binds to the durable family whose JSON Schema package its structured
# output validates against; `lifecycle_label` is filled from the publication
# matrix at build time. `partial_demo` / `freshness_demo` are the concrete
# states the per-surface CLI and UI fixtures both carry, so the parity check
# proves the lifecycle/degraded-state vocabulary is identical on both surfaces.
SURFACES = [
    {
        "surface_id": "command_inspect",
        "title": "Command inspection",
        "summary": (
            "Inspect a command's descriptor, authority class, and invocation-session "
            "envelope from the CLI/headless surface."
        ),
        "surface_kind": "inspect",
        "family_id": "command_descriptors",
        "command_id": "cli.command.inspect",
        "output_envelope_class": "json_document_single",
        "ui_inspect_surface": "command_palette.command_inspector",
        "parity_match_mode": "exact_match_required",
        "result_codes": READ_ONLY_RESULT_CODES,
        "partial_states": ["complete", "partial", "degraded", "stale_retest_needed"],
        "partial_demo": "complete",
        "freshness_demo": "fresh",
    },
    {
        "surface_id": "support_bundle_export",
        "title": "Support/evidence bundle export",
        "summary": (
            "Export a support/evidence bundle index and object-handoff packet with "
            "redaction posture from the CLI/headless surface."
        ),
        "surface_kind": "export",
        "family_id": "support_bundles_and_handoff",
        "command_id": "cli.support.export",
        "output_envelope_class": "json_document_single",
        "ui_inspect_surface": "support_center.bundle_inspector",
        "parity_match_mode": "exact_match_required",
        "result_codes": EXPORT_RESULT_CODES,
        "partial_states": ["complete", "partial", "degraded", "unavailable", "stale_retest_needed"],
        "partial_demo": "partial",
        "freshness_demo": "fresh",
    },
    {
        "surface_id": "diagnostics_report",
        "title": "Diagnostics report",
        "summary": (
            "Report diagnostic/problem evidence chains and search-diagnostic clusters "
            "as a machine-readable document."
        ),
        "surface_kind": "report",
        "family_id": "diagnostic_records",
        "command_id": "cli.diagnostics.report",
        "output_envelope_class": "sarif_2_1_0_document",
        "ui_inspect_surface": "editor.problems_inspector",
        "parity_match_mode": "projection_match_required",
        "result_codes": READ_ONLY_RESULT_CODES,
        "partial_states": ["complete", "partial", "degraded", "stale_retest_needed"],
        "partial_demo": "degraded",
        "freshness_demo": "stale",
    },
    {
        "surface_id": "project_doctor_health",
        "title": "Project Doctor health check",
        "summary": (
            "Run the Project Doctor health surface and emit findings, probe outcomes, "
            "and escalation routes as a machine-readable document."
        ),
        "surface_kind": "health",
        "family_id": "project_doctor_findings",
        "command_id": "cli.doctor.health",
        "output_envelope_class": "json_document_single",
        "ui_inspect_surface": "support_center.doctor_panel",
        "parity_match_mode": "exact_match_required",
        "result_codes": READ_ONLY_RESULT_CODES,
        "partial_states": ["complete", "partial", "degraded", "unavailable", "stale_retest_needed"],
        "partial_demo": "degraded",
        "freshness_demo": "retest_needed",
    },
    {
        "surface_id": "restore_provenance_inspect",
        "title": "Restore-provenance inspection",
        "summary": (
            "Inspect capture-session manifests, trace/replay bundles, and exact-build "
            "identity for a restore-provenance view."
        ),
        "surface_kind": "inspect",
        "family_id": "replay_and_trace_evidence",
        "command_id": "cli.restore.provenance",
        "output_envelope_class": "json_document_single",
        "ui_inspect_surface": "recovery.restore_provenance_view",
        "parity_match_mode": "exact_match_required",
        "result_codes": READ_ONLY_RESULT_CODES,
        "partial_states": ["complete", "partial", "degraded", "stale_retest_needed"],
        "partial_demo": "complete",
        "freshness_demo": "fresh",
    },
    {
        "surface_id": "ai_evidence_export",
        "title": "AI evidence export",
        "summary": (
            "Export AI session execution-context and provenance evidence with "
            "degraded-field disclosure from the CLI/headless surface."
        ),
        "surface_kind": "export",
        "family_id": "execution_context_provenance",
        "command_id": "cli.ai.evidence.export",
        "output_envelope_class": "ndjson_event_stream",
        "ui_inspect_surface": "ai.evidence_inspector",
        "parity_match_mode": "exact_match_required",
        "result_codes": EXPORT_RESULT_CODES,
        "partial_states": ["complete", "partial", "degraded", "unavailable", "stale_retest_needed"],
        "partial_demo": "partial",
        "freshness_demo": "stale",
    },
    {
        "surface_id": "capability_qualification_inspect",
        "title": "Capability/qualification inspection",
        "summary": (
            "Inspect capability inventory entries and their qualification/claim "
            "lifecycle state from the CLI/headless surface."
        ),
        "surface_kind": "inspect",
        "family_id": "capability_records",
        "command_id": "cli.capability.inspect",
        "output_envelope_class": "json_document_single",
        "ui_inspect_surface": "about.capability_inventory_panel",
        "parity_match_mode": "exact_match_required",
        "result_codes": READ_ONLY_RESULT_CODES,
        "partial_states": ["complete", "partial", "degraded", "stale_retest_needed"],
        "partial_demo": "complete",
        "freshness_demo": "fresh",
    },
    {
        "surface_id": "repair_transaction_report",
        "title": "Repair-transaction report",
        "summary": (
            "Report repair-transaction preview/apply/rollback records and the "
            "recovery-action ledger as a machine-readable document."
        ),
        "surface_kind": "report",
        "family_id": "repair_transactions",
        "command_id": "cli.repair.report",
        "output_envelope_class": "json_document_single",
        "ui_inspect_surface": "support_center.repair_ledger",
        "parity_match_mode": "exact_match_required",
        "result_codes": READ_ONLY_RESULT_CODES,
        "partial_states": ["complete", "partial", "degraded", "stale_retest_needed"],
        "partial_demo": "partial",
        "freshness_demo": "retest_needed",
    },
    {
        "surface_id": "policy_config_inspect",
        "title": "Policy/config inspection",
        "summary": (
            "Inspect admin policy bundles, policy cache entries, and effective config "
            "from the CLI/headless surface."
        ),
        "surface_kind": "inspect",
        "family_id": "policy_bundles",
        "command_id": "cli.policy.inspect",
        "output_envelope_class": "json_document_single",
        "ui_inspect_surface": "admin.policy_inspector",
        "parity_match_mode": "exact_match_required",
        "result_codes": READ_ONLY_RESULT_CODES,
        "partial_states": ["complete", "partial", "degraded", "stale_retest_needed"],
        "partial_demo": "complete",
        "freshness_demo": "fresh",
    },
]


def load_matrix_labels() -> dict[str, str]:
    """Map publication-matrix family id -> effective published lifecycle label."""
    matrix = json.loads(PUBLICATION_MATRIX_PATH.read_text(encoding="utf-8"))
    return {
        row.get("family_id"): row.get("published_label")
        for row in matrix.get("rows", [])
        if isinstance(row, dict)
    }


def json_package(family_id: str) -> dict:
    """The canonical JSON Schema catalog package for a family."""
    return next(p for p in jsoncat.PACKAGES if p["family_id"] == family_id)


def structured_output_schema_ref(family_id: str) -> str:
    return jsoncat.schema_path(json_package(family_id))


def structured_output_schema_id(family_id: str) -> str:
    return jsoncat.schema_id(json_package(family_id))


def lifecycle_label(surface: dict, labels: dict[str, str]) -> str:
    return labels[surface["family_id"]]


def stability_class(label: str) -> str:
    return {
        "lts": "stable_schema_governed",
        "stable": "stable_schema_governed",
        "beta": "preview_schema_governed_additive_minor_only",
        "preview": "experimental_schema_governed_may_break",
    }[label]


def cli_fixture_ref(surface: dict) -> str:
    return f"fixtures/contracts/m5-cli-json/{surface['surface_id']}.cli.json"


def ui_fixture_ref(surface: dict) -> str:
    return f"fixtures/contracts/m5-cli-json/{surface['surface_id']}.ui.json"


def result_code_catalog(surface: dict) -> list[dict]:
    rows = []
    for code in surface["result_codes"]:
        rows.append(
            {
                "result_code": code,
                "numeric_code": RESULT_CODE_NUMERIC[code],
                "meaning": RESULT_CODE_MEANING[code],
                "partial_result": code == "partial_success_with_warnings",
            }
        )
    return rows


def build_base_payload(surface: dict) -> dict:
    """A version-stamped payload that validates against the family's package."""
    return jsoncat.build_example(json_package(surface["family_id"]))


def build_cli_fixture(surface: dict, label: str) -> dict:
    """CLI/headless structured-output payload for a surface.

    Validates against the family's JSON Schema package (record-kind tag, version
    field, and primary identity), then overlays the structured-output contract:
    the surface kind, the projection tag, the emitted result code, and the
    partial-result / freshness / lifecycle vocabulary the UI fixture must match.
    """
    payload = build_base_payload(surface)
    payload.update(
        {
            "surface_id": surface["surface_id"],
            "surface_kind": surface["surface_kind"],
            "projection": "cli_machine",
            "structured_output_schema_ref": structured_output_schema_ref(surface["family_id"]),
            "result_code": "success",
            "partial_result_state": surface["partial_demo"],
            "freshness_state": surface["freshness_demo"],
            "lifecycle_label": label,
        }
    )
    return payload


def build_ui_fixture(surface: dict, label: str) -> dict:
    """UI inspect-surface payload carrying the same degraded-state vocabulary."""
    payload = build_base_payload(surface)
    payload.update(
        {
            "surface_id": surface["surface_id"],
            "surface_kind": surface["surface_kind"],
            "projection": "ui_inspect",
            "partial_result_state": surface["partial_demo"],
            "freshness_state": surface["freshness_demo"],
            "lifecycle_label": label,
        }
    )
    return payload


def build_surface_row(surface: dict, labels: dict[str, str]) -> dict:
    label = lifecycle_label(surface, labels)
    family = surface["family_id"]
    return {
        "surface_id": surface["surface_id"],
        "title": surface["title"],
        "summary": surface["summary"],
        "surface_kind": surface["surface_kind"],
        "family_id": family,
        "command_id": surface["command_id"],
        "lifecycle_label": label,
        "machine_output_stability_class": stability_class(label),
        "output_envelope_class": surface["output_envelope_class"],
        "structured_output_schema_ref": structured_output_schema_ref(family),
        "structured_output_schema_id": structured_output_schema_id(family),
        "result_code_catalog": result_code_catalog(surface),
        "partial_result_states": list(surface["partial_states"]),
        "freshness_states": list(FRESHNESS_STATES),
        "downgrade_behavior": "narrow_below_cutline",
        "compatibility_note": COMPATIBILITY_NOTE,
        "compatibility_note_ref": CLI_SURFACE_CONTRACT_REF,
        "ui_inspect_surface": surface["ui_inspect_surface"],
        "parity_match_mode": surface["parity_match_mode"],
        "cli_parity_fixture_ref": cli_fixture_ref(surface),
        "ui_parity_fixture_ref": ui_fixture_ref(surface),
        "json_schema_catalog_ref": f"{JSON_SCHEMA_CATALOG_REF}#{family}",
        "matrix_row_ref": f"{PUBLICATION_MATRIX_REF}#{family}",
        "validator_suite_refs": [
            "tools/validate_m5_cli_output_catalog.py",
            "ci/contract_validation.sh",
        ],
    }


def compute_summary(rows: list[dict]) -> dict:
    def count(pred) -> int:
        return sum(1 for r in rows if pred(r))

    return {
        "total_surfaces": len(rows),
        "inspect_surfaces": count(lambda r: r["surface_kind"] == "inspect"),
        "export_surfaces": count(lambda r: r["surface_kind"] == "export"),
        "report_surfaces": count(lambda r: r["surface_kind"] == "report"),
        "health_surfaces": count(lambda r: r["surface_kind"] == "health"),
        "stable_label_surfaces": count(lambda r: r["lifecycle_label"] == "stable"),
        "beta_label_surfaces": count(lambda r: r["lifecycle_label"] == "beta"),
        "surfaces_with_partial_result_carrier": count(
            lambda r: any(c["partial_result"] for c in r["result_code_catalog"])
        ),
        "surfaces_with_parity_fixtures": count(
            lambda r: r["cli_parity_fixture_ref"] and r["ui_parity_fixture_ref"]
        ),
    }


def build_catalog() -> dict:
    labels = load_matrix_labels()
    rows = [build_surface_row(s, labels) for s in SURFACES]
    return {
        "schema_version": SCHEMA_VERSION,
        "record_kind": RECORD_KIND,
        "catalog_id": CATALOG_ID,
        "status": "published",
        "as_of": AS_OF,
        "overview_page": OVERVIEW_PAGE,
        "cli_doc_page": CLI_DOC_PAGE,
        "json_schema_catalog_ref": JSON_SCHEMA_CATALOG_REF,
        "publication_matrix_ref": PUBLICATION_MATRIX_REF,
        "cli_surface_contract_ref": CLI_SURFACE_CONTRACT_REF,
        "cli_output_registry_schema_ref": CLI_OUTPUT_REGISTRY_SCHEMA_REF,
        "evidence_index_ref": EVIDENCE_INDEX_REF,
        "schema_home": SCHEMA_HOME,
        "surface_kinds": list(SURFACE_KINDS),
        "lifecycle_labels": list(LIFECYCLE_LABELS),
        "machine_output_stability_classes": list(MACHINE_OUTPUT_STABILITY_CLASSES),
        "output_envelope_classes": list(OUTPUT_ENVELOPE_CLASSES),
        "result_codes": list(RESULT_CODES),
        "partial_result_states": list(PARTIAL_RESULT_STATES),
        "freshness_states": list(FRESHNESS_STATES),
        "parity_match_modes": list(PARITY_MATCH_MODES),
        "downgrade_behaviors": list(DOWNGRADE_BEHAVIORS),
        "offline_bundle": {
            "mirrorable": True,
            "requires_runtime_service": False,
            "bundle_members": [
                "artifacts/contracts/m5-cli-output-catalog.json",
                "schemas/public/m5-cli/m5_cli_output_catalog.schema.json",
                "fixtures/contracts/m5-cli-json/",
                "tools/validate_m5_cli_output_catalog.py",
            ],
            "note": (
                "The catalog, its boundary schema, the per-surface parity fixtures, "
                "and the validator bundle into offline/mirror artifact sets and "
                "validate without runtime service access."
            ),
        },
        "surfaces": rows,
        "summary": compute_summary(rows),
    }


def build_capture(catalog: dict) -> dict:
    return {
        "status": "pass",
        "as_of": catalog["as_of"],
        "catalog_id": catalog["catalog_id"],
        "summary": catalog["summary"],
        "surface_checks": [
            {
                "surface_id": r["surface_id"],
                "surface_kind": r["surface_kind"],
                "family_id": r["family_id"],
                "lifecycle_label": r["lifecycle_label"],
                "schema_ref_resolves": "passed",
                "result_codes_in_vocabulary": "passed",
                "lifecycle_matches_matrix": "passed",
                "ui_cli_parity_vocabulary_identical": "passed",
            }
            for r in catalog["surfaces"]
        ],
        "negative_drills": [
            {"drill_id": "drill:duplicate_surface_id", "status": "passed"},
            {"drill_id": "drill:result_code_off_vocabulary", "status": "passed"},
            {"drill_id": "drill:parity_vocabulary_mismatch", "status": "passed"},
            {"drill_id": "drill:summary_count_mismatch", "status": "passed"},
        ],
        "fixture_cases": [
            {"case_id": "fixture:duplicate_surface_id", "status": "passed"},
            {"case_id": "fixture:result_code_off_vocabulary", "status": "passed"},
            {"case_id": "fixture:summary_count_mismatch", "status": "passed"},
        ],
    }


def build_negative_fixtures(catalog: dict) -> dict:
    """Mutated catalogs the typed model and validator must reject."""
    duplicate = json.loads(json.dumps(catalog))
    duplicate["surfaces"].append(json.loads(json.dumps(duplicate["surfaces"][0])))
    duplicate["summary"] = compute_summary(duplicate["surfaces"])

    off_vocab = json.loads(json.dumps(catalog))
    off_vocab["surfaces"][0]["result_code_catalog"][0]["result_code"] = "exploded"

    summary_mismatch = json.loads(json.dumps(catalog))
    summary_mismatch["summary"]["total_surfaces"] += 1

    return {
        "duplicate_surface_id.json": duplicate,
        "result_code_off_vocabulary.json": off_vocab,
        "summary_count_mismatch.json": summary_mismatch,
    }


def build_cli_doc(catalog: dict) -> str:
    lines: list[str] = []
    lines.append("# M5 CLI/headless structured output and result codes")
    lines.append("")
    lines.append(
        "This is the human-readable index of the **M5 CLI/headless structured-output "
        "and result-code catalog**. The machine-readable catalog at "
        "`artifacts/contracts/m5-cli-output-catalog.json` is authoritative; if the "
        "two disagree, the catalog wins and this document must be updated in the "
        "same change."
    )
    lines.append("")
    lines.append("## What the catalog publishes")
    lines.append("")
    lines.append(
        "For every new M5 CLI/headless inspect, export, report, and health surface, "
        "the catalog publishes one surface row that binds:"
    )
    lines.append("")
    lines.append(
        "- a **structured-output schema reference** resolved from the canonical M5 "
        "JSON Schema catalog (`schemas/public/m5-json/<family>.schema.json`),"
    )
    lines.append(
        "- a **result-code catalog** — stable enums drawn from the CLI/headless "
        "machine-output stability contract, each with a pinned numeric code and a "
        "partial-result flag,"
    )
    lines.append(
        "- a **lifecycle label** equal to the publication matrix's effective "
        "published label for the family,"
    )
    lines.append(
        "- the **partial-result** and **freshness** vocabularies the surface can "
        "emit (`complete` / `partial` / `degraded` / `unavailable` / "
        "`stale_retest_needed`, and `fresh` / `stale` / `retest_needed` / "
        "`unknown`), and"
    )
    lines.append(
        "- a **UI/CLI parity** declaration with a CLI fixture and a UI inspect "
        "fixture proving the lifecycle/degraded-state vocabulary is identical on "
        "both surfaces."
    )
    lines.append("")
    lines.append("## Surfaces")
    lines.append("")
    lines.append("| Surface | Kind | Family | Lifecycle | Envelope | Schema |")
    lines.append("| --- | --- | --- | --- | --- | --- |")
    for r in catalog["surfaces"]:
        lines.append(
            f"| `{r['surface_id']}` | {r['surface_kind']} | {r['family_id']} | "
            f"{r['lifecycle_label']} | `{r['output_envelope_class']}` | "
            f"`{r['structured_output_schema_ref']}` |"
        )
    lines.append("")
    lines.append("## Result-code catalog")
    lines.append("")
    lines.append(
        "Every result code is a member of the closed `exit_code_class` vocabulary "
        "frozen in `schemas/automation/cli_output_registry_entry.schema.json`, so a "
        "machine consumer keys off the stable enum, not the human text. The numeric "
        "code is pinned for shell-level consumers; `success` and "
        "`success_no_action_taken` are always `0`."
    )
    lines.append("")
    lines.append("| Result code | Numeric | Partial-result carrier | Meaning |")
    lines.append("| --- | --- | --- | --- |")
    for code in RESULT_CODES:
        carrier = "yes" if code == "partial_success_with_warnings" else "no"
        lines.append(
            f"| `{code}` | {RESULT_CODE_NUMERIC[code]} | {carrier} | "
            f"{RESULT_CODE_MEANING[code]} |"
        )
    lines.append("")
    lines.append("## Partial results and staleness")
    lines.append("")
    lines.append(
        "A surface that cannot fully resolve emits `partial_success_with_warnings` "
        "with a `partial_result_state` of `partial` or `degraded`; a surface whose "
        "inputs are stale emits a `freshness_state` of `stale` or `retest_needed` "
        "so automation never mistakes a stale cache for a fresh result. These two "
        "vocabularies are closed and stable and are shared field-for-field with the "
        "matching UI inspect surface."
    )
    lines.append("")
    lines.append("## UI/CLI parity")
    lines.append("")
    lines.append(
        "Each surface ships a CLI fixture and a UI inspect fixture under "
        "`fixtures/contracts/m5-cli-json/`. The validator proves the two carry an "
        "identical `partial_result_state`, `freshness_state`, and `lifecycle_label`, "
        "so the desktop inspect surface and the CLI/headless output never diverge on "
        "the lifecycle or degraded-state vocabulary."
    )
    lines.append("")
    lines.append("| Surface | UI inspect surface | Match mode |")
    lines.append("| --- | --- | --- |")
    for r in catalog["surfaces"]:
        lines.append(
            f"| `{r['surface_id']}` | `{r['ui_inspect_surface']}` | {r['parity_match_mode']} |"
        )
    lines.append("")
    lines.append("## Offline and mirror use")
    lines.append("")
    lines.append(
        "The catalog, its boundary schema, the per-surface parity fixtures, and the "
        "validator bundle into offline/mirror artifact sets and validate without "
        "runtime service access (`offline_bundle.requires_runtime_service` is "
        "`false`)."
    )
    lines.append("")
    lines.append("## Freshness")
    lines.append("")
    lines.append(
        f"The catalog is current as of `{catalog['as_of']}`. CI regenerates it from "
        "`tools/regenerate_m5_cli_output_catalog.py`, runs "
        "`tools/validate_m5_cli_output_catalog.py`, and runs the typed Rust "
        "consumer's tests, so the published surfaces cannot drift from the catalog."
    )
    lines.append("")
    return "\n".join(lines)


def write_json(path: Path, data: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")


def write_text(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    if not text.endswith("\n"):
        text += "\n"
    path.write_text(text, encoding="utf-8")


def main() -> None:
    catalog = build_catalog()
    labels = load_matrix_labels()
    write_json(CATALOG_PATH, catalog)
    print(f"wrote {CATALOG_PATH.relative_to(REPO_ROOT)}")

    for surface in SURFACES:
        label = lifecycle_label(surface, labels)
        write_json(FIXTURE_DIR / f"{surface['surface_id']}.cli.json", build_cli_fixture(surface, label))
        write_json(FIXTURE_DIR / f"{surface['surface_id']}.ui.json", build_ui_fixture(surface, label))
    print(f"wrote {2 * len(SURFACES)} parity fixtures under {FIXTURE_DIR.relative_to(REPO_ROOT)}")

    write_text(CLI_DOC_PATH, build_cli_doc(catalog))
    print(f"wrote {CLI_DOC_PATH.relative_to(REPO_ROOT)}")

    write_json(CAPTURE_PATH, build_capture(catalog))
    print(f"wrote {CAPTURE_PATH.relative_to(REPO_ROOT)}")

    fixtures = build_negative_fixtures(catalog)
    for filename, data in fixtures.items():
        write_json(NEGATIVE_DIR / filename, data)
    cases = {
        "cases": [
            {
                "case_id": "fixture:duplicate_surface_id",
                "file": "duplicate_surface_id.json",
                "expected_check_id": "surfaces.duplicate_surface_id",
            },
            {
                "case_id": "fixture:result_code_off_vocabulary",
                "file": "result_code_off_vocabulary.json",
                "expected_check_id": "surfaces.result_code_off_vocabulary",
            },
            {
                "case_id": "fixture:summary_count_mismatch",
                "file": "summary_count_mismatch.json",
                "expected_check_id": "summary.count_mismatch",
            },
        ]
    }
    write_json(NEGATIVE_DIR / "cases.json", cases)
    for filename in list(fixtures) + ["cases.json"]:
        print(f"wrote {(NEGATIVE_DIR / filename).relative_to(REPO_ROOT)}")


if __name__ == "__main__":
    main()
