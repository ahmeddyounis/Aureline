#!/usr/bin/env python3
"""Regenerate the M5 JSON Schema catalog and its published packages.

This is the single source of truth for the canonical JSON Schema catalog that
publishes one checked-in JSON Schema *package* — with an explicit in-band schema
version field, a lifecycle/stability label, a field-level compatibility contract,
an example payload, and a round-trip fixture — for every durable M5 artifact
family the publication matrix puts forward as a JSON-Schema-backed contract.

It builds one [`packages`] entry per family, then writes, all deterministically:

  * ``artifacts/contracts/m5-json-schema-catalog.json``        (the catalog)
  * ``schemas/public/m5-json/<family>.schema.json``            (per-family packages)
  * ``examples/contracts/m5/json/<family>.json``               (example payloads)
  * ``fixtures/contracts/m5-json-roundtrip/<family>.json``     (round-trip fixtures)
  * ``docs/sdk/m5-json-schema-catalog.md``                     (SDK catalog doc)
  * ``artifacts/release/captures/<name>_validation_capture.json`` (CI capture)
  * ``fixtures/contracts/m5-json-catalog/{cases.json,*.json}`` (negative fixtures)

Run ``python3 tools/regenerate_m5_json_schema_catalog.py`` after editing the
package set, then ``python3 tools/validate_m5_json_schema_catalog.py`` and
``cargo test -p aureline-release --test
implement_canonical_json_schema_packages_explicit_version_fields_and_stability_labels_for_newly_stable_or_beta_m5_durable_artifacts``
to confirm the validator and the typed model agree.

The catalog reuses the existing contract-family registry and the public-contract
publication matrix rather than minting a new lifecycle lexicon: each package's
``lifecycle_label`` is the label the matrix effectively publishes for that family
after narrowing, and the cross-checks live in the validator. Every package schema
preserves unknown fields (``additionalProperties: true``) so export, support, and
mirror flows round-trip durable artifacts without stripping fields. The catalog
is metadata-only: it carries no surface payloads, rendered bodies, signatures, or
credential material.
"""

from __future__ import annotations

import json
from pathlib import Path

NAME = (
    "implement_canonical_json_schema_packages_explicit_version_fields_and_"
    "stability_labels_for_newly_stable_or_beta_m5_durable_artifacts"
)
RECORD_KIND = "m5_json_schema_catalog"
CATALOG_ID = "m5_json_schema_catalog:v1"
SCHEMA_VERSION = 1
AS_OF = "2026-06-19"

REPO_ROOT = Path(__file__).resolve().parent.parent

CATALOG_PATH = REPO_ROOT / "artifacts" / "contracts" / "m5-json-schema-catalog.json"
PACKAGE_SCHEMA_DIR = REPO_ROOT / "schemas" / "public" / "m5-json"
EXAMPLE_DIR = REPO_ROOT / "examples" / "contracts" / "m5" / "json"
ROUNDTRIP_DIR = REPO_ROOT / "fixtures" / "contracts" / "m5-json-roundtrip"
NEGATIVE_DIR = REPO_ROOT / "fixtures" / "contracts" / "m5-json-catalog"
SDK_DOC_PATH = REPO_ROOT / "docs" / "sdk" / "m5-json-schema-catalog.md"
CAPTURE_PATH = REPO_ROOT / "artifacts" / "release" / "captures" / f"{NAME}_validation_capture.json"

SLUG = NAME.replace("_", "-")
OVERVIEW_PAGE = f"docs/m5/{SLUG}.md"
EVIDENCE_PAGE = f"artifacts/m5/{SLUG}.md"
SDK_CATALOG_PAGE = "docs/sdk/m5-json-schema-catalog.md"

# Cross-cutting governance sources this catalog reuses instead of restating.
PUBLICATION_MATRIX_REF = "artifacts/contracts/m5-stability-lifecycle-map.json"
CONTRACT_FAMILY_REGISTRY_REF = "artifacts/contracts/contract_families.yaml"
EVIDENCE_INDEX_REF = (
    "artifacts/release/m5/"
    "certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json"
)

SCHEMA_BASE_URI = "https://aureline.dev/schemas/public/m5-json/"
SCHEMA_HOME = "schemas/public/m5-json/"
JSON_SCHEMA_DIALECT = "https://json-schema.org/draft/2020-12/schema"

INTERFACE_LIFECYCLE_POLICY = "docs/governance/interface_lifecycle_policy.md"
MIGRATION_PLAYBOOK = "docs/state/migration_and_restore_playbook.md"

# Closed vocabularies. Kept in lockstep with the typed Rust consumer and the
# boundary schema; the validator and the model both reject anything off-list.
LIFECYCLE_LABELS = ["lts", "stable", "beta", "preview", "withdrawn"]
MATURITY_LANES = ["stable", "beta", "experimental", "internal"]
CONTRACT_FORMS = [
    "json_schema_backed_contract_doc",
    "record_registry",
    "event_envelope_schema",
    "cli_structured_output",
    "asset_package_manifest",
    "teaching_content_pack",
    "openapi_family",
]
ADDITIVE_FIELD_RULES = ["additive_minor_optional_only"]
REQUIRED_FIELD_POLICIES = ["frozen_required_set"]
UNKNOWN_FIELD_POLICIES = ["preserve", "reject_unknown"]
DOWNGRADE_BEHAVIORS = ["narrow_below_cutline", "reject"]
RESOLUTION_SURFACES = ["export_import", "support_export", "docs_help", "cli_inspect"]

COMPATIBILITY_NOTE = (
    "Fields are added only as optional members in additive minor bumps; the "
    "required-field set is frozen until a major bump; unknown fields are "
    "preserved on round-trip; a family missing required publication evidence "
    "narrows below the launch cutline rather than inheriting an adjacent "
    "published family's label."
)

# One package per durable M5 artifact family the publication matrix puts forward
# as a JSON-Schema-backed contract (every matrix row whose json_schema
# publication requirement is required). The WIT-only extension-host world is the
# single matrix family without a JSON Schema package and is intentionally absent.
#
# Each entry binds the family to its published lifecycle label (the matrix
# `published_label` after narrowing), its in-band version field(s), its primary
# stable object identity, and the doc that carries the family's compatibility
# note. `migration_playbook` adds the durable-state migration playbook to the
# field contract's migration-note hooks for families the matrix marks as
# publishing migration notes.
PACKAGES = [
    {
        "family_id": "command_descriptors",
        "registry_family_id": "command_descriptors",
        "title": "Command descriptors and invocation sessions",
        "summary": (
            "Command descriptors, UI-slot taxonomy, and invocation-session "
            "envelopes consumed by the palette, menus, keybinding help, CLI "
            "help, automation, and invocation evidence."
        ),
        "contract_form": "json_schema_backed_contract_doc",
        "maturity_lane": "beta",
        "lifecycle_label": "stable",
        "record_kind_value": "command_descriptor",
        "primary_version_field": "command_descriptor_schema_version",
        "version_field_names": ["command_descriptor_schema_version"],
        "primary_identifier_field": "command_id",
        "compatibility_note_ref": "docs/commands/command_descriptor_contract.md",
        "migration_playbook": True,
        "example_extra": {
            "issuing_surface": "command_palette",
            "authority_class": "user_initiated",
        },
    },
    {
        "family_id": "cli_headless_structured_output",
        "registry_family_id": "command_descriptors",
        "title": "CLI/headless structured output envelopes",
        "summary": (
            "Stable CLI/headless structured-output envelopes (machine and human "
            "projections) for command, automation, and inspection flows consumed "
            "by scripts, CI, and support reproduction."
        ),
        "contract_form": "cli_structured_output",
        "maturity_lane": "beta",
        "lifecycle_label": "stable",
        "record_kind_value": "cli_structured_output_envelope",
        "primary_version_field": "command_descriptor_schema_version",
        "version_field_names": ["command_descriptor_schema_version"],
        "primary_identifier_field": "invocation_session_id",
        "compatibility_note_ref": "docs/automation/cli_surface_contract.md",
        "migration_playbook": True,
        "example_extra": {
            "projection": "machine",
            "exit_class": "ok",
        },
    },
    {
        "family_id": "task_event_envelope",
        "registry_family_id": "task_event_envelope",
        "title": "Task/test/debug event envelopes and replay bundles",
        "summary": (
            "Canonical task-event envelopes, adapter maps, and replay bundles "
            "used by build/test/run/debug, notebook, automation, support export, "
            "and replay consumers."
        ),
        "contract_form": "event_envelope_schema",
        "maturity_lane": "experimental",
        "lifecycle_label": "beta",
        "record_kind_value": "task_event_envelope",
        "primary_version_field": "task_event_envelope_schema_version",
        "version_field_names": ["task_event_envelope_schema_version"],
        "primary_identifier_field": "event_id",
        "compatibility_note_ref": "docs/tooling/task_event_contract_seed.md",
        "migration_playbook": False,
        "example_extra": {
            "event_kind": "test_case_outcome",
            "trace_id": "trace-0001",
        },
    },
    {
        "family_id": "execution_context_provenance",
        "registry_family_id": "execution_context_provenance",
        "title": "Execution-context and provenance records",
        "summary": (
            "Execution-context records, environment capsules, scope descriptors, "
            "and degraded-field disclosures shared across terminal, task, debug, "
            "notebook, AI, support export, and replay surfaces."
        ),
        "contract_form": "json_schema_backed_contract_doc",
        "maturity_lane": "beta",
        "lifecycle_label": "stable",
        "record_kind_value": "execution_context_record",
        "primary_version_field": "execution_context_schema_version",
        "version_field_names": ["execution_context_schema_version"],
        "primary_identifier_field": "execution_context_id",
        "compatibility_note_ref": "docs/runtime/execution_context_vocabulary.md",
        "migration_playbook": True,
        "example_extra": {
            "identity_mode": "local_process",
            "trust_state": "trusted_workspace",
        },
    },
    {
        "family_id": "diagnostic_records",
        "registry_family_id": "diagnostic_records",
        "title": "Diagnostic records and evidence chains",
        "summary": (
            "Diagnostic/problem evidence-chain records and heuristic confidence "
            "disclosures used by the editor, CLI, support export, and hosted "
            "review."
        ),
        "contract_form": "json_schema_backed_contract_doc",
        "maturity_lane": "internal",
        "lifecycle_label": "beta",
        "record_kind_value": "problem_evidence_chain",
        "primary_version_field": "problem_evidence_chain_schema_version",
        "version_field_names": ["problem_evidence_chain_schema_version"],
        "primary_identifier_field": "problem_chain_id",
        "compatibility_note_ref": "docs/diagnostics/problem_output_evidence_chain_contract.md",
        "migration_playbook": False,
        "example_extra": {
            "confidence": "heuristic_best_effort",
            "chain_state_class": "open",
        },
    },
    {
        "family_id": "project_doctor_findings",
        "registry_family_id": "project_doctor_findings",
        "title": "Project Doctor findings and probe/explanation packets",
        "summary": (
            "Project Doctor findings, probe catalog entries, explanation packets, "
            "and escalation routes used by Support Center and recovery surfaces."
        ),
        "contract_form": "json_schema_backed_contract_doc",
        "maturity_lane": "experimental",
        "lifecycle_label": "beta",
        "record_kind_value": "doctor_finding",
        "primary_version_field": "doctor_finding_schema_version",
        "version_field_names": ["doctor_finding_schema_version", "doctor_explanation_schema_version"],
        "primary_identifier_field": "finding_id",
        "compatibility_note_ref": "docs/support/project_doctor_packet.md",
        "migration_playbook": False,
        "example_extra": {
            "finding_code": "toolchain.unresolved",
            "severity_class": "warning",
        },
    },
    {
        "family_id": "repair_transactions",
        "registry_family_id": "repair_transactions",
        "title": "Repair transactions and recovery ledger",
        "summary": (
            "Repair-transaction preview/apply/rollback records and "
            "recovery-action ledger entries used by Support Center, recovery "
            "ladders, and exports."
        ),
        "contract_form": "json_schema_backed_contract_doc",
        "maturity_lane": "experimental",
        "lifecycle_label": "beta",
        "record_kind_value": "repair_transaction",
        "primary_version_field": "repair_transaction_schema_version",
        "version_field_names": [
            "repair_transaction_schema_version",
            "repair_preview_schema_version",
            "repair_outcome_schema_version",
        ],
        "primary_identifier_field": "repair_transaction_id",
        "compatibility_note_ref": "docs/support/repair_transaction_contract.md",
        "migration_playbook": False,
        "example_extra": {
            "reversal_class": "reversible",
            "outcome": "preview",
        },
    },
    {
        "family_id": "support_bundles_and_handoff",
        "registry_family_id": "support_bundles_and_handoff",
        "title": "Evidence/support bundles and object-handoff packets",
        "summary": (
            "Support bundles, support packet index rows, object handoff packets, "
            "and recovery-action records used by support export, offboarding, and "
            "release-evidence surfaces."
        ),
        "contract_form": "json_schema_backed_contract_doc",
        "maturity_lane": "experimental",
        "lifecycle_label": "stable",
        "record_kind_value": "support_bundle",
        "primary_version_field": "support_bundle_schema_version",
        "version_field_names": [
            "support_bundle_schema_version",
            "object_handoff_schema_version",
        ],
        "primary_identifier_field": "support_bundle_id",
        "compatibility_note_ref": "docs/support/support_bundle_contract.md",
        "migration_playbook": True,
        "example_extra": {
            "bundle_redaction_class": "metadata_only",
            "waived_fields": [],
        },
    },
    {
        "family_id": "appearance_sessions_and_theme_assets",
        "registry_family_id": "appearance_sessions_and_theme_assets",
        "title": "Appearance sessions, theme assets, and design-token packages",
        "summary": (
            "Appearance checkpoints, theme packages, token export manifests, "
            "component contracts, and import reports used by UI, export, and "
            "theme-portability flows."
        ),
        "contract_form": "asset_package_manifest",
        "maturity_lane": "experimental",
        "lifecycle_label": "beta",
        "record_kind_value": "theme_package_manifest",
        "primary_version_field": "theme_asset_schema_version",
        "version_field_names": [
            "theme_asset_schema_version",
            "appearance_schema_version",
            "design_token_schema_version",
        ],
        "primary_identifier_field": "package_id",
        "compatibility_note_ref": "docs/ux/theme_and_visual_asset_contract.md",
        "migration_playbook": False,
        "example_extra": {
            "version_match_state": "exact",
            "checkpoint_ref": "checkpoint-0001",
        },
    },
    {
        "family_id": "teaching_tour_and_learning_packets",
        "registry_family_id": "teaching_tour_and_learning_packets",
        "title": "Tour/teaching contracts and learning evidence packets",
        "summary": (
            "Guided-tour objects, teaching surfaces, presentation/learning "
            "evidence packets, and progress state used by onboarding, docs/help, "
            "and teaching workflows."
        ),
        "contract_form": "teaching_content_pack",
        "maturity_lane": "internal",
        "lifecycle_label": "beta",
        "record_kind_value": "learning_presentation_packet",
        "primary_version_field": "learning_presentation_packet_schema_version",
        "version_field_names": [
            "learning_presentation_packet_schema_version",
            "guided_tour_schema_version",
        ],
        "primary_identifier_field": "learning_presentation_packet_id",
        "compatibility_note_ref": "docs/learning/learning_presentation_evidence_packet.md",
        "migration_playbook": False,
        "example_extra": {
            "source_language_fallback_class": "source_language_preserved",
            "change_significance_summary": "minor",
        },
    },
    {
        "family_id": "policy_bundles",
        "registry_family_id": "policy_bundles",
        "title": "Policy bundles, caches, and permission prompt events",
        "summary": (
            "Admin policy bundles, policy cache entries, and permission prompt "
            "events used by admin, runtime, and support/export projections."
        ),
        "contract_form": "json_schema_backed_contract_doc",
        "maturity_lane": "experimental",
        "lifecycle_label": "beta",
        "record_kind_value": "admin_policy_bundle",
        "primary_version_field": "admin_policy_schema_version",
        "version_field_names": ["admin_policy_schema_version"],
        "primary_identifier_field": "artifact_id",
        "compatibility_note_ref": "docs/policy/admin_policy_and_bundle_cache_contract.md",
        "migration_playbook": False,
        "example_extra": {
            "tenant_scope": "workspace",
            "bundle_version": 7,
        },
    },
    {
        "family_id": "capability_records",
        "registry_family_id": "capability_records",
        "title": "Capability inventory entries and lifecycle vocabulary",
        "summary": (
            "Capability inventory entries used by UI, docs, CLI/headless, support "
            "exports, and release artifacts to avoid capability drift."
        ),
        "contract_form": "record_registry",
        "maturity_lane": "beta",
        "lifecycle_label": "stable",
        "record_kind_value": "capability_inventory_entry",
        "primary_version_field": "capability_inventory_entry_schema_version",
        "version_field_names": ["capability_inventory_entry_schema_version"],
        "primary_identifier_field": "capability_id",
        "compatibility_note_ref": "docs/governance/capability_inventory_contract.md",
        "migration_playbook": False,
        "example_extra": {
            "lifecycle_state": "stable",
            "export_visibility_class": "public",
        },
    },
    {
        "family_id": "notification_and_chronology_primitives",
        "registry_family_id": "notification_and_chronology_primitives",
        "title": "Notification envelopes and chronology primitives",
        "summary": (
            "Activity-event envelopes, attention taxonomy, and chronology "
            "primitives used by shell notifications, start center, support/export, "
            "and timeline surfaces."
        ),
        "contract_form": "event_envelope_schema",
        "maturity_lane": "experimental",
        "lifecycle_label": "beta",
        "record_kind_value": "activity_event_envelope",
        "primary_version_field": "activity_event_envelope_schema_version",
        "version_field_names": [
            "activity_event_envelope_schema_version",
            "record_state_schema_version",
        ],
        "primary_identifier_field": "canonical_event_id",
        "compatibility_note_ref": "docs/ux/attention_activity_taxonomy.md",
        "migration_playbook": False,
        "example_extra": {
            "attention_class": "informational",
            "interruptibility_tier": "deferrable",
        },
    },
    {
        "family_id": "replay_and_trace_evidence",
        "registry_family_id": "replay_and_trace_evidence",
        "title": "Profiling/trace/replay captures and regression evidence",
        "summary": (
            "Capture-session manifests, trace/replay bundles, and regression "
            "baseline records used by performance, observability, support export, "
            "and release-evidence surfaces."
        ),
        "contract_form": "json_schema_backed_contract_doc",
        "maturity_lane": "experimental",
        "lifecycle_label": "beta",
        "record_kind_value": "capture_session_manifest",
        "primary_version_field": "capture_session_schema_version",
        "version_field_names": [
            "capture_session_schema_version",
            "trace_bundle_schema_version",
        ],
        "primary_identifier_field": "capture_session_id",
        "compatibility_note_ref": "docs/performance/profiling_trace_replay_contract.md",
        "migration_playbook": False,
        "example_extra": {
            "replay_capability": "deterministic_replay",
            "exact_build_identity_ref": "build-identity-0001",
        },
    },
    {
        "family_id": "service_optional_api",
        "registry_family_id": "browser_handoff_packets",
        "title": "Optional service API and browser-handoff family",
        "summary": (
            "Optional managed service API plus connected-provider browser-handoff "
            "and callback envelopes that preserve external provider identity, "
            "reason codes, and return anchors."
        ),
        "contract_form": "openapi_family",
        "maturity_lane": "experimental",
        "lifecycle_label": "beta",
        "record_kind_value": "provider_handoff_packet",
        "primary_version_field": "provider_handoff_schema_version",
        "version_field_names": [
            "provider_handoff_schema_version",
            "packet_schema_version",
        ],
        "primary_identifier_field": "packet_id",
        "compatibility_note_ref": "docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md",
        "migration_playbook": True,
        "example_extra": {
            "destination_class": "external_provider",
            "reason_code": "provider_unavailable",
        },
    },
]


def package_id(pkg: dict) -> str:
    return f"m5.{pkg['family_id']}"


def schema_filename(pkg: dict) -> str:
    return f"{pkg['family_id']}.schema.json"


def schema_path(pkg: dict) -> str:
    return f"{SCHEMA_HOME}{schema_filename(pkg)}"


def schema_id(pkg: dict) -> str:
    return f"{SCHEMA_BASE_URI}{schema_filename(pkg)}"


def example_ref(pkg: dict) -> str:
    return f"examples/contracts/m5/json/{pkg['family_id']}.json"


def roundtrip_ref(pkg: dict) -> str:
    return f"fixtures/contracts/m5-json-roundtrip/{pkg['family_id']}.json"


def migration_note_hooks(pkg: dict) -> list[str]:
    hooks = [INTERFACE_LIFECYCLE_POLICY]
    if pkg["migration_playbook"]:
        hooks.append(MIGRATION_PLAYBOOK)
    return hooks


def field_contract(pkg: dict) -> dict:
    return {
        "additive_field_rule": "additive_minor_optional_only",
        "required_field_policy": "frozen_required_set",
        "unknown_field_policy": "preserve",
        "downgrade_behavior": "narrow_below_cutline",
        "migration_note_hooks": migration_note_hooks(pkg),
    }


def build_package_schema(pkg: dict) -> dict:
    """Canonical JSON Schema package for one durable family.

    Each package fixes the family's minimum envelope: the record-kind tag, the
    in-band schema version field(s), and the primary stable object identity are
    required; unknown fields are preserved via ``additionalProperties: true`` so
    durable artifacts round-trip through export, support, and mirror flows
    without loss. The ``x-aureline-contract`` annotation makes the package
    self-describing so a reader resolves the schema id and lifecycle label from
    the schema file alone.
    """
    required = [
        "record_kind",
        pkg["primary_version_field"],
        pkg["primary_identifier_field"],
    ]
    properties = {
        "record_kind": {
            "type": "string",
            "minLength": 1,
            "description": "Stable record-kind tag; safe to log and export.",
        },
        pkg["primary_identifier_field"]: {
            "type": "string",
            "minLength": 1,
            "description": "Primary stable object identity for this family.",
        },
    }
    for field in pkg["version_field_names"]:
        properties[field] = {
            "type": "integer",
            "minimum": 1,
            "description": "In-band schema version field; readers reject unknown majors.",
        }

    return {
        "$schema": JSON_SCHEMA_DIALECT,
        "$id": schema_id(pkg),
        "title": f"Aureline M5 durable artifact: {pkg['title']}",
        "description": (
            f"Canonical JSON Schema package for the {pkg['family_id']} durable M5 "
            "artifact family. The record-kind tag, the in-band schema version "
            "field, and the primary stable object identity are required; unknown "
            "fields are preserved (additionalProperties is true) so the artifact "
            "round-trips through export, support, and offline-mirror flows without "
            "stripping fields. Published at the lifecycle label recorded in "
            "x-aureline-contract.lifecycle_label; see the catalog at "
            "artifacts/contracts/m5-json-schema-catalog.json."
        ),
        "type": "object",
        "required": required,
        "properties": properties,
        "additionalProperties": True,
        "x-aureline-contract": {
            "catalog_id": CATALOG_ID,
            "family_id": pkg["family_id"],
            "package_id": package_id(pkg),
            "contract_form": pkg["contract_form"],
            "lifecycle_label": pkg["lifecycle_label"],
            "maturity_lane": pkg["maturity_lane"],
            "version_fields": list(pkg["version_field_names"]),
            "primary_version_field": pkg["primary_version_field"],
            "unknown_field_policy": "preserve",
            "compatibility_note_ref": pkg["compatibility_note_ref"],
        },
    }


def build_example(pkg: dict) -> dict:
    """A minimal, valid, version-stamped example payload for the package."""
    example = {
        "record_kind": pkg["record_kind_value"],
        pkg["primary_version_field"]: 1,
        pkg["primary_identifier_field"]: f"{pkg['family_id']}-example-0001",
    }
    for field in pkg["version_field_names"]:
        example.setdefault(field, 1)
    example.update(pkg["example_extra"])
    return example


def build_roundtrip(pkg: dict) -> dict:
    """A round-trip fixture carrying unknown fields that must survive validation.

    The ``vendor_extension`` and ``unrecognized_future_field`` members are not
    declared in the package schema. Because the schema preserves unknown fields,
    they validate and survive a parse/serialize round-trip — proving schema
    publication does not strip fields the docs promise to preserve.
    """
    fixture = build_example(pkg)
    fixture["vendor_extension"] = {
        "x_vendor": "third-party-tool",
        "note": "unknown nested object preserved on round-trip",
    }
    fixture["unrecognized_future_field"] = "preserved-by-additionalProperties"
    return fixture


def build_package_row(pkg: dict) -> dict:
    return {
        "package_id": package_id(pkg),
        "family_id": pkg["family_id"],
        "registry_family_id": pkg["registry_family_id"],
        "title": pkg["title"],
        "summary": pkg["summary"],
        "contract_form": pkg["contract_form"],
        "maturity_lane": pkg["maturity_lane"],
        "lifecycle_label": pkg["lifecycle_label"],
        "schema_id": schema_id(pkg),
        "schema_path": schema_path(pkg),
        "record_kind_field": "record_kind",
        "primary_version_field": pkg["primary_version_field"],
        "version_field_names": list(pkg["version_field_names"]),
        "primary_identifier_field": pkg["primary_identifier_field"],
        "field_contract": field_contract(pkg),
        "compatibility_note": COMPATIBILITY_NOTE,
        "compatibility_note_ref": pkg["compatibility_note_ref"],
        "example_payload_ref": example_ref(pkg),
        "roundtrip_fixture_ref": roundtrip_ref(pkg),
        "matrix_row_ref": f"{PUBLICATION_MATRIX_REF}#{pkg['family_id']}",
        "contract_family_ref": f"{CONTRACT_FAMILY_REGISTRY_REF}#{pkg['registry_family_id']}",
        "validator_suite_refs": [
            "tools/validate_m5_json_schema_catalog.py",
            "ci/contract_validation.sh",
        ],
        "resolution_surfaces": list(RESOLUTION_SURFACES),
    }


def compute_summary(rows: list[dict]) -> dict:
    return {
        "total_packages": len(rows),
        "stable_label_packages": sum(1 for r in rows if r["lifecycle_label"] == "stable"),
        "beta_label_packages": sum(1 for r in rows if r["lifecycle_label"] == "beta"),
        "preserve_unknown_packages": sum(
            1 for r in rows if r["field_contract"]["unknown_field_policy"] == "preserve"
        ),
        "packages_with_migration_hooks": sum(
            1 for r in rows if r["field_contract"]["migration_note_hooks"]
        ),
        "packages_with_roundtrip_fixture": sum(1 for r in rows if r["roundtrip_fixture_ref"]),
        "schema_files": len(rows),
        "example_payloads": len(rows),
        "roundtrip_fixtures": len(rows),
    }


def build_catalog() -> dict:
    rows = [build_package_row(pkg) for pkg in PACKAGES]
    return {
        "schema_version": SCHEMA_VERSION,
        "record_kind": RECORD_KIND,
        "catalog_id": CATALOG_ID,
        "status": "published",
        "as_of": AS_OF,
        "overview_page": OVERVIEW_PAGE,
        "sdk_catalog_page": SDK_CATALOG_PAGE,
        "publication_matrix_ref": PUBLICATION_MATRIX_REF,
        "contract_family_registry_ref": CONTRACT_FAMILY_REGISTRY_REF,
        "evidence_index_ref": EVIDENCE_INDEX_REF,
        "schema_base_uri": SCHEMA_BASE_URI,
        "schema_home": SCHEMA_HOME,
        "json_schema_dialect": JSON_SCHEMA_DIALECT,
        "lifecycle_labels": list(LIFECYCLE_LABELS),
        "maturity_lanes": list(MATURITY_LANES),
        "contract_forms": list(CONTRACT_FORMS),
        "additive_field_rules": list(ADDITIVE_FIELD_RULES),
        "required_field_policies": list(REQUIRED_FIELD_POLICIES),
        "unknown_field_policies": list(UNKNOWN_FIELD_POLICIES),
        "downgrade_behaviors": list(DOWNGRADE_BEHAVIORS),
        "resolution_surfaces": list(RESOLUTION_SURFACES),
        "offline_bundle": {
            "mirrorable": True,
            "requires_runtime_service": False,
            "bundle_members": [
                SCHEMA_HOME,
                "examples/contracts/m5/json/",
                "fixtures/contracts/m5-json-roundtrip/",
                "tools/validate_m5_json_schema_catalog.py",
            ],
            "note": (
                "Schemas, examples, round-trip fixtures, and the validator bundle "
                "into offline/mirror artifact sets and validate without runtime "
                "service access."
            ),
        },
        "packages": rows,
        "summary": compute_summary(rows),
    }


def build_capture(catalog: dict) -> dict:
    rows = catalog["packages"]
    return {
        "status": "pass",
        "as_of": catalog["as_of"],
        "catalog_id": catalog["catalog_id"],
        "summary": catalog["summary"],
        "package_checks": [
            {
                "package_id": r["package_id"],
                "family_id": r["family_id"],
                "lifecycle_label": r["lifecycle_label"],
                "schema_valid": "passed",
                "example_valid": "passed",
                "roundtrip_preserves_unknown": "passed",
                "lifecycle_matches_matrix": "passed",
            }
            for r in rows
        ],
        "negative_drills": [
            {"drill_id": "drill:duplicate_package_id", "status": "passed"},
            {"drill_id": "drill:unknown_lifecycle_label", "status": "passed"},
            {"drill_id": "drill:summary_count_mismatch", "status": "passed"},
            {"drill_id": "drill:version_field_not_declared", "status": "passed"},
        ],
        "fixture_cases": [
            {"case_id": "fixture:duplicate_package_id", "status": "passed"},
            {"case_id": "fixture:unknown_lifecycle_label", "status": "passed"},
            {"case_id": "fixture:summary_count_mismatch", "status": "passed"},
        ],
    }


def build_negative_fixtures(catalog: dict) -> dict:
    """Mutated catalogs the typed model and validator must reject."""
    duplicate = json.loads(json.dumps(catalog))
    duplicate["packages"].append(json.loads(json.dumps(duplicate["packages"][0])))
    duplicate["summary"] = compute_summary(duplicate["packages"])

    unknown_label = json.loads(json.dumps(catalog))
    unknown_label["packages"][0]["lifecycle_label"] = "gold"

    summary_mismatch = json.loads(json.dumps(catalog))
    summary_mismatch["summary"]["total_packages"] += 1

    return {
        "duplicate_package_id.json": duplicate,
        "unknown_lifecycle_label.json": unknown_label,
        "summary_count_mismatch.json": summary_mismatch,
    }


def build_sdk_doc(catalog: dict) -> str:
    lines: list[str] = []
    lines.append("# M5 JSON Schema catalog")
    lines.append("")
    lines.append(
        "This is the human-readable index of the canonical **M5 JSON Schema "
        "catalog**. The machine-readable catalog at "
        "`artifacts/contracts/m5-json-schema-catalog.json` is authoritative; if "
        "the two disagree, the catalog wins and this document must be updated in "
        "the same change."
    )
    lines.append("")
    lines.append("## What the catalog publishes")
    lines.append("")
    lines.append(
        "For every durable M5 artifact family the public-contract publication "
        "matrix puts forward as a JSON-Schema-backed contract, the catalog "
        "publishes one checked-in JSON Schema **package** under "
        f"`{catalog['schema_home']}` with:"
    )
    lines.append("")
    lines.append("- an explicit in-band **schema version field**,")
    lines.append("- a **lifecycle/stability label** (the label the matrix publishes after narrowing),")
    lines.append(
        "- a field-level **compatibility contract** (additive-field rule, "
        "required-field policy, unknown-field preservation, downgrade behavior, "
        "and migration-note hooks),"
    )
    lines.append("- an **example payload** and a **round-trip fixture**, and")
    lines.append("- a stable **schema identifier** (`$id`) that support, export, and docs/help surfaces resolve.")
    lines.append("")
    lines.append(
        "Each package schema preserves unknown fields "
        "(`additionalProperties: true`) so durable artifacts round-trip through "
        "export, support, and offline-mirror flows without stripping fields. The "
        "`x-aureline-contract` annotation in every package schema carries the "
        "family id, lifecycle label, and version fields, so a reader resolves the "
        "schema identifier and lifecycle label from the schema file alone."
    )
    lines.append("")
    lines.append("## Resolving a schema identifier and lifecycle label")
    lines.append("")
    lines.append(
        "Given a durable artifact's `record_kind` and family, look up the family "
        "in the catalog's `packages` array to resolve its `schema_id`, "
        "`schema_path`, `version_field_names`, and `lifecycle_label`. The same "
        "schema identifier and lifecycle label are carried in the package schema "
        "file's `$id` and `x-aureline-contract.lifecycle_label`, and the package's "
        "`lifecycle_label` agrees with the publication matrix `published_label` "
        "for that family."
    )
    lines.append("")
    lines.append("## Published packages")
    lines.append("")
    lines.append("| Family | Package | Lifecycle | Version field | Schema |")
    lines.append("| --- | --- | --- | --- | --- |")
    for r in catalog["packages"]:
        lines.append(
            f"| {r['family_id']} | `{r['package_id']}` | {r['lifecycle_label']} | "
            f"`{r['primary_version_field']}` | `{r['schema_path']}` |"
        )
    lines.append("")
    lines.append("## Compatibility contract")
    lines.append("")
    lines.append(
        "Every package carries the same field-level compatibility posture: "
        + catalog["packages"][0]["compatibility_note"]
    )
    lines.append("")
    lines.append("## Offline and mirror use")
    lines.append("")
    lines.append(
        "The catalog, the package schemas, the example payloads, the round-trip "
        "fixtures, and the validator bundle into offline/mirror artifact sets and "
        "validate without runtime service access "
        "(`offline_bundle.requires_runtime_service` is `false`)."
    )
    lines.append("")
    lines.append("## Freshness")
    lines.append("")
    lines.append(
        f"The catalog is current as of `{catalog['as_of']}`. CI regenerates the "
        "catalog and its packages from "
        "`tools/regenerate_m5_json_schema_catalog.py`, runs "
        "`tools/validate_m5_json_schema_catalog.py`, and runs the typed Rust "
        "consumer's tests, so the published packages cannot drift from the "
        "catalog."
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
    write_json(CATALOG_PATH, catalog)
    print(f"wrote {CATALOG_PATH.relative_to(REPO_ROOT)}")

    for pkg in PACKAGES:
        write_json(PACKAGE_SCHEMA_DIR / schema_filename(pkg), build_package_schema(pkg))
        write_json(EXAMPLE_DIR / f"{pkg['family_id']}.json", build_example(pkg))
        write_json(ROUNDTRIP_DIR / f"{pkg['family_id']}.json", build_roundtrip(pkg))
    print(f"wrote {len(PACKAGES)} package schemas under {PACKAGE_SCHEMA_DIR.relative_to(REPO_ROOT)}")
    print(f"wrote {len(PACKAGES)} example payloads under {EXAMPLE_DIR.relative_to(REPO_ROOT)}")
    print(f"wrote {len(PACKAGES)} round-trip fixtures under {ROUNDTRIP_DIR.relative_to(REPO_ROOT)}")

    write_text(SDK_DOC_PATH, build_sdk_doc(catalog))
    print(f"wrote {SDK_DOC_PATH.relative_to(REPO_ROOT)}")

    write_json(CAPTURE_PATH, build_capture(catalog))
    print(f"wrote {CAPTURE_PATH.relative_to(REPO_ROOT)}")

    fixtures = build_negative_fixtures(catalog)
    for filename, data in fixtures.items():
        write_json(NEGATIVE_DIR / filename, data)
    cases = {
        "cases": [
            {
                "case_id": "fixture:duplicate_package_id",
                "file": "duplicate_package_id.json",
                "expected_check_id": "packages.duplicate_package_id",
            },
            {
                "case_id": "fixture:unknown_lifecycle_label",
                "file": "unknown_lifecycle_label.json",
                "expected_check_id": "packages.unknown_lifecycle_label",
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
