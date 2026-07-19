#!/usr/bin/env python3
"""Regenerate the M5 public-contract publication matrix artifacts.

This is the single source of truth for the checked-in matrix, its flat CSV and
Markdown projections, the CI validation capture, and the negative fixtures. It
builds one row per M5 public-contract family, derives each row's publication
state, narrowing reasons, and effective label exactly as the typed Rust consumer
does, then writes:

  * ``artifacts/contracts/m5-stability-lifecycle-map.json``   (the matrix)
  * ``artifacts/contracts/m5-public-contract-inventory.csv``  (flat projection)
  * ``artifacts/contracts/m5-public-contract-matrix.md``      (human projection)
  * ``artifacts/release/captures/<name>_validation_capture.json`` (CI capture)
  * ``fixtures/contracts/m5/{cases.json,*.json}``             (negative fixtures)

Run ``python3 tools/regenerate_m5_public_contract_matrix.py`` after editing the
row set, then ``python3 tools/validate_m5_public_contract_matrix.py`` and
``cargo test -p aureline-release --test rel_it_18_freeze_m5_public_contract``
to confirm the validator and the typed model agree.

The matrix is descriptive metadata. It reuses the existing contract-family
registry, the compatibility-surface inventory, the qualification matrix, the
stability/lifecycle vocabulary, and the claim manifest rather than minting a new
contract-status lexicon. Every field is a typed state or an opaque repo-relative
ref; the matrix carries no surface payloads, rendered bodies, signatures, or
credential material.
"""

from __future__ import annotations

import csv
import io
import json
from pathlib import Path

NAME = "freeze_the_m5_public_contract_schema_publication_wit_openapi_and_interchange_conformance_matrix"
RECORD_KIND = "m5_public_contract_matrix"
MATRIX_ID = "m5_public_contract_matrix:v1"
SCHEMA_VERSION = 1
AS_OF = "2026-06-19"

REPO_ROOT = Path(__file__).resolve().parent.parent

MATRIX_PATH = REPO_ROOT / "artifacts" / "contracts" / "m5-stability-lifecycle-map.json"
CSV_PATH = REPO_ROOT / "artifacts" / "contracts" / "m5-public-contract-inventory.csv"
MD_PATH = REPO_ROOT / "artifacts" / "contracts" / "m5-public-contract-matrix.md"
CAPTURE_PATH = REPO_ROOT / "artifacts" / "release" / "captures" / f"{NAME}_validation_capture.json"
FIXTURES_DIR = REPO_ROOT / "fixtures" / "contracts" / "m5"

OVERVIEW_PAGE = f"docs/m5/{NAME.replace('_', '-')}.md"
EVIDENCE_PAGE = f"artifacts/m5/{NAME.replace('_', '-')}.md"

# Cross-cutting governance sources this matrix reuses instead of restating.
CLAIM_MANIFEST_REF = "artifacts/release/stable_claim_manifest.json"
CONTRACT_FAMILY_REGISTRY_REF = "artifacts/contracts/contract_families.yaml"
COMPAT_SURFACE_INVENTORY_REF = "artifacts/governance/compatibility_surfaces.yaml"
QUALIFICATION_MATRIX_REF = "artifacts/compat/qualification_matrix_seed.yaml"
EVIDENCE_INDEX_REF = (
    "artifacts/release/m5/"
    "certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json"
)

# --- Closed vocabularies, in canonical (declaration) order. ------------------
# These must match the `ALL` arrays in the typed Rust model and the schema enums.

# Reuse compatibility_surfaces.yaml#contract_form_values verbatim.
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

# Reuse compatibility_surfaces.yaml#category_values verbatim.
CONTRACT_CATEGORIES = [
    "settings_and_profile",
    "workspace_and_state",
    "extensions_and_host",
    "command_and_automation",
    "ai_and_language",
    "editor_and_text",
    "terminal_and_run",
    "debug_and_diagnostics",
    "merge_and_history",
    "portability_and_migration",
    "locale_and_translation",
    "design_and_theme",
    "accessibility_and_input",
    "voice_and_consent",
    "service_and_api",
    "review_and_hosted",
    "release_and_build",
    "support_and_export",
    "governance_and_policy",
    "docs_and_teaching",
    "notification_and_attention",
    "certification_and_reference",
]

# Reuse compatibility_surfaces.yaml#maturity_lane_values verbatim (the B43/B28
# stability lane vocabulary).
MATURITY_LANES = ["stable", "beta", "experimental", "internal"]

# Reuse the stable claim lifecycle labels (claim manifest / cutline vocabulary).
LIFECYCLE_LABELS = ["lts", "stable", "beta", "preview", "withdrawn"]
ABOVE_CUTLINE = ["lts", "stable"]
BELOW_CUTLINE = ["beta", "preview", "withdrawn"]
RANK = {"lts": 4, "stable": 3, "beta": 2, "preview": 1, "withdrawn": 0}

READER_WRITER_POSTURES = [
    "reader_only",
    "writer_only",
    "read_write",
    "bidirectional_interchange",
]

# Reuse the deployment-profile tokens already used by the dependency/quality
# parity matrix instead of a new packaging lexicon.
PACKAGING_NEEDS = ["local_only", "mirrored", "managed", "browser_handoff"]

PUBLICATION_ARTIFACT_KINDS = [
    "json_schema",
    "wit_world",
    "openapi_spec",
    "markdown_summary",
    "example_payloads",
    "migration_notes",
    "validator_suite",
]

PUBLICATION_STATES = ["published", "partial", "missing", "not_applicable"]

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

# artifact kind -> gap reason raised when a required publication is absent.
KIND_TO_GAP = {
    "json_schema": "json_schema_unpublished",
    "wit_world": "wit_world_unpublished",
    "openapi_spec": "openapi_spec_unpublished",
    "markdown_summary": "markdown_summary_unpublished",
    "example_payloads": "example_payloads_unpublished",
    "migration_notes": "migration_notes_unpublished",
    "validator_suite": "validator_suite_unpublished",
}

# gap reason -> default remediation action a stop rule prescribes.
GAP_TO_ACTION = {
    "json_schema_unpublished": "publish_contract_form",
    "wit_world_unpublished": "publish_contract_form",
    "openapi_spec_unpublished": "publish_contract_form",
    "markdown_summary_unpublished": "publish_contract_form",
    "example_payloads_unpublished": "publish_example_payloads",
    "migration_notes_unpublished": "publish_contract_form",
    "validator_suite_unpublished": "wire_validator_suite",
    "release_packet_unlinked": "link_release_packet",
}

# A "published" requirement is satisfied; every other state is a gap.
PUBLISHED = "published"

# Shared validator suite that gates every contract family in this matrix.
COMMON_VALIDATORS = [
    "tools/validate_contract_family_registry.py",
    "tools/validate_m5_public_contract_matrix.py",
    "ci/contract_validation.sh",
]

MIGRATION_PLAYBOOK = "docs/state/migration_and_restore_playbook.md"
LIFECYCLE_POLICY = "docs/governance/interface_lifecycle_policy.md"


def req(kind: str, required: bool, state: str, refs: list[str]) -> dict:
    """Build one publication requirement cell."""
    assert kind in PUBLICATION_ARTIFACT_KINDS, kind
    assert state in PUBLICATION_STATES, state
    return {
        "artifact_kind": kind,
        "required": required,
        "state": state,
        "refs": refs,
    }


def schema_backed_requirements(
    *,
    schema_homes: list[str],
    doc_refs: list[str],
    example_refs: list[str],
    migration_refs: list[str] | None,
    wit_refs: list[str] | None = None,
    openapi_refs: list[str] | None = None,
    migration_required: bool = True,
    openapi_state: str | None = None,
    extra_validators: list[str] | None = None,
) -> list[dict]:
    """Default publication-requirement set for a JSON-Schema-backed family.

    ``migration_refs`` may be ``None`` to leave migration notes unpublished while
    still requiring them (the family then narrows). ``wit_refs``/``openapi_refs``
    promote those forms from ``not_applicable`` to required+published.
    """
    requirements = [
        req("json_schema", True, PUBLISHED, list(schema_homes)),
    ]

    if wit_refs:
        requirements.append(req("wit_world", True, PUBLISHED, list(wit_refs)))
    else:
        requirements.append(req("wit_world", False, "not_applicable", []))

    if openapi_refs:
        state = openapi_state or PUBLISHED
        requirements.append(req("openapi_spec", True, state, list(openapi_refs)))
    else:
        requirements.append(req("openapi_spec", False, "not_applicable", []))

    requirements.append(req("markdown_summary", True, PUBLISHED, list(doc_refs)))
    requirements.append(req("example_payloads", True, PUBLISHED, list(example_refs)))

    if migration_required:
        if migration_refs:
            requirements.append(req("migration_notes", True, PUBLISHED, list(migration_refs)))
        else:
            requirements.append(req("migration_notes", True, "missing", []))
    else:
        requirements.append(req("migration_notes", False, "not_applicable", []))

    validators = list(COMMON_VALIDATORS) + list(extra_validators or [])
    requirements.append(req("validator_suite", True, PUBLISHED, validators))
    return requirements


def compute_gaps(requirements: list[dict], release_packet_ref: str) -> list[str]:
    """Derive active gap reasons from the requirement cells, in canonical order."""
    gaps: set[str] = set()
    for cell in requirements:
        if cell["required"] and cell["state"] != PUBLISHED:
            gaps.add(KIND_TO_GAP[cell["artifact_kind"]])
    if not release_packet_ref.strip():
        gaps.add("release_packet_unlinked")
    return [reason for reason in GAP_REASONS if reason in gaps]


def narrow_floor(claim_label: str) -> str:
    """The label a gapped row narrows to: one step below the cutline."""
    # A claim at or above the cutline narrows to beta; a claim already below the
    # cutline narrows one rank lower (but never below withdrawn).
    if RANK[claim_label] >= RANK["stable"]:
        return "beta"
    return BELOW_CUTLINE[min(BELOW_CUTLINE.index(claim_label) + 1, len(BELOW_CUTLINE) - 1)]


def row(
    *,
    family_id: str,
    title: str,
    summary: str,
    owning_package: str,
    owner_dri: str,
    category: str,
    contract_form: str,
    maturity_lane: str,
    reader_writer_posture: str,
    packaging_need: str,
    claim_label: str,
    release_blocking: bool,
    release_packet_ref: str,
    compatibility_surface_ref: str,
    qualification_row_ref: str | None,
    contract_family_ref: str,
    example_corpus_refs: list[str],
    requirements: list[dict],
    rationale_published: str,
    rationale_narrowed: str,
) -> dict:
    """Assemble one matrix row, deriving its gap reasons and effective label."""
    assert category in CONTRACT_CATEGORIES, category
    assert contract_form in CONTRACT_FORMS, contract_form
    assert maturity_lane in MATURITY_LANES, maturity_lane
    assert reader_writer_posture in READER_WRITER_POSTURES, reader_writer_posture
    assert packaging_need in PACKAGING_NEEDS, packaging_need
    assert claim_label in LIFECYCLE_LABELS, claim_label

    gaps = compute_gaps(requirements, release_packet_ref)
    if gaps:
        row_state = "narrowed"
        published_label = narrow_floor(claim_label)
        rationale = rationale_narrowed
    else:
        row_state = "published"
        published_label = claim_label
        rationale = rationale_published

    validator_suite_refs = []
    for cell in requirements:
        if cell["artifact_kind"] == "validator_suite":
            validator_suite_refs = list(cell["refs"])

    return {
        "family_id": family_id,
        "title": title,
        "summary": summary,
        "owning_package": owning_package,
        "owner_dri": owner_dri,
        "category": category,
        "contract_form": contract_form,
        "maturity_lane": maturity_lane,
        "reader_writer_posture": reader_writer_posture,
        "packaging_need": packaging_need,
        "claim_label": claim_label,
        "published_label": published_label,
        "row_state": row_state,
        "release_blocking": release_blocking,
        "contract_family_ref": contract_family_ref,
        "compatibility_surface_ref": compatibility_surface_ref,
        "qualification_row_ref": qualification_row_ref,
        "release_packet_ref": release_packet_ref,
        "example_corpus_refs": example_corpus_refs,
        "validator_suite_refs": validator_suite_refs,
        "publication_requirements": requirements,
        "active_gap_reasons": gaps,
        "publication_destinations": ["docs/m5", "help/about", "support/export"],
        "rationale": rationale,
    }


def surface_ref(surface_id: str) -> str:
    return f"{COMPAT_SURFACE_INVENTORY_REF}#{surface_id}"


def family_ref(family_id: str) -> str:
    return f"{CONTRACT_FAMILY_REGISTRY_REF}#{family_id}"


def build_rows() -> list:
    rows = [
        row(
            family_id="command_descriptors",
            title="Command descriptors and invocation sessions",
            summary=(
                "Command descriptors, UI-slot taxonomy, and invocation-session "
                "envelopes consumed by the palette, menus, keybinding help, CLI "
                "help, automation, and invocation evidence."
            ),
            owning_package="aureline-shell",
            owner_dri="@ahmeddyounis",
            category="command_and_automation",
            contract_form="json_schema_backed_contract_doc",
            maturity_lane="beta",
            reader_writer_posture="read_write",
            packaging_need="local_only",
            claim_label="stable",
            release_blocking=True,
            release_packet_ref="manifest_entry:provider_aware_language_intelligence",
            compatibility_surface_ref=surface_ref(
                "command_plane.command_graph_and_ui_slot_schema"
            ),
            qualification_row_ref="compat_row:command_plane.command_descriptor_schema",
            contract_family_ref=family_ref("command_descriptors"),
            example_corpus_refs=[
                "fixtures/commands/command_descriptor_examples/",
                "fixtures/contracts/contract_family_examples/command_descriptors.yaml",
            ],
            requirements=schema_backed_requirements(
                schema_homes=["schemas/commands/"],
                doc_refs=[
                    "docs/commands/command_descriptor_contract.md",
                    "docs/automation/cli_surface_contract.md",
                ],
                example_refs=[
                    "fixtures/commands/command_descriptor_examples/",
                    "fixtures/contracts/contract_family_examples/command_descriptors.yaml",
                ],
                migration_refs=[MIGRATION_PLAYBOOK, LIFECYCLE_POLICY],
            ),
            rationale_published=(
                "The command-descriptor contract publishes its schema, summary, "
                "examples, migration notes, and validator suite and is release-linked, "
                "so it holds its Stable contract claim."
            ),
            rationale_narrowed="",
        ),
        row(
            family_id="cli_headless_structured_output",
            title="CLI/headless structured output contract",
            summary=(
                "Stable CLI/headless structured-output envelopes (machine and "
                "human projections) for command, automation, and inspection flows "
                "consumed by scripts, CI, and support reproduction."
            ),
            owning_package="aureline-shell",
            owner_dri="@ahmeddyounis",
            category="command_and_automation",
            contract_form="cli_structured_output",
            maturity_lane="beta",
            reader_writer_posture="reader_only",
            packaging_need="local_only",
            claim_label="stable",
            release_blocking=True,
            release_packet_ref="manifest_entry:provider_aware_language_intelligence",
            compatibility_surface_ref=surface_ref(
                "automation.cli_structured_output_contract"
            ),
            qualification_row_ref="compat_row:automation.cli_headless_contract",
            contract_family_ref=family_ref("command_descriptors"),
            example_corpus_refs=[
                "fixtures/contracts/contract_family_examples/command_descriptors.yaml",
            ],
            requirements=schema_backed_requirements(
                schema_homes=["schemas/commands/", "schemas/automation/"],
                doc_refs=["docs/automation/cli_surface_contract.md"],
                example_refs=[
                    "fixtures/contracts/contract_family_examples/command_descriptors.yaml",
                ],
                migration_refs=[MIGRATION_PLAYBOOK],
            ),
            rationale_published=(
                "The CLI/headless structured-output contract publishes its schema, "
                "summary, examples, migration notes, and validator suite and is "
                "release-linked, so it holds its Stable contract claim."
            ),
            rationale_narrowed="",
        ),
        row(
            family_id="task_event_envelope",
            title="Task/test/debug event envelopes and replay bundles",
            summary=(
                "Canonical task-event envelopes, adapter maps, and replay bundles "
                "used by build/test/run/debug, notebook, automation, support export, "
                "and replay consumers."
            ),
            owning_package="aureline-tooling",
            owner_dri="@ahmeddyounis",
            category="terminal_and_run",
            contract_form="event_envelope_schema",
            maturity_lane="experimental",
            reader_writer_posture="bidirectional_interchange",
            packaging_need="mirrored",
            claim_label="stable",
            release_blocking=True,
            release_packet_ref="manifest_entry:repair_and_rollback_safety",
            compatibility_surface_ref=surface_ref("tooling.task_event_envelope"),
            qualification_row_ref="compat_row:tooling.task_event_envelope",
            contract_family_ref=family_ref("task_event_envelope"),
            example_corpus_refs=[
                "fixtures/tooling/task_event_replay/",
                "fixtures/contracts/contract_family_examples/task_event_envelope.yaml",
            ],
            requirements=schema_backed_requirements(
                schema_homes=["schemas/tooling/"],
                doc_refs=[
                    "docs/tooling/task_event_contract_seed.md",
                    "docs/execution/context_inspector_packet.md",
                ],
                example_refs=[
                    "fixtures/tooling/task_event_replay/",
                    "fixtures/contracts/contract_family_examples/task_event_envelope.yaml",
                ],
                migration_refs=None,
            ),
            rationale_published="",
            rationale_narrowed=(
                "The task-event envelope is put forward for a Stable contract claim "
                "but publishes no migration/deprecation notes yet, so it narrows to "
                "Beta until the migration notes land."
            ),
        ),
        row(
            family_id="execution_context_provenance",
            title="Execution-context and provenance records",
            summary=(
                "Execution-context records, environment capsules, scope descriptors, "
                "and degraded-field disclosures shared across terminal, task, debug, "
                "notebook, AI, support export, and replay surfaces."
            ),
            owning_package="aureline-runtime",
            owner_dri="@ahmeddyounis",
            category="terminal_and_run",
            contract_form="json_schema_backed_contract_doc",
            maturity_lane="beta",
            reader_writer_posture="reader_only",
            packaging_need="local_only",
            claim_label="stable",
            release_blocking=True,
            release_packet_ref="manifest_entry:repair_and_rollback_safety",
            compatibility_surface_ref=surface_ref(
                "runtime.execution_context_and_provenance"
            ),
            qualification_row_ref="compat_row:tooling.task_event_envelope",
            contract_family_ref=family_ref("execution_context_provenance"),
            example_corpus_refs=[
                "fixtures/execution/context_diff_cases/",
                "fixtures/contracts/contract_family_examples/execution_context_provenance.yaml",
            ],
            requirements=schema_backed_requirements(
                schema_homes=["schemas/runtime/", "schemas/execution/"],
                doc_refs=[
                    "docs/runtime/execution_context_vocabulary.md",
                    "docs/execution/context_inspector_packet.md",
                ],
                example_refs=[
                    "fixtures/execution/context_diff_cases/",
                    "fixtures/contracts/contract_family_examples/execution_context_provenance.yaml",
                ],
                migration_refs=[MIGRATION_PLAYBOOK],
            ),
            rationale_published=(
                "The execution-context contract publishes its schema, summary, "
                "examples, migration notes, and validator suite and is release-linked, "
                "so it holds its Stable contract claim."
            ),
            rationale_narrowed="",
        ),
        row(
            family_id="diagnostic_records",
            title="Diagnostic records and evidence chains",
            summary=(
                "Diagnostic/problem evidence-chain records and heuristic confidence "
                "disclosures used by the editor, CLI, support export, and hosted review."
            ),
            owning_package="aureline-diagnostics",
            owner_dri="@ahmeddyounis",
            category="debug_and_diagnostics",
            contract_form="json_schema_backed_contract_doc",
            maturity_lane="internal",
            reader_writer_posture="reader_only",
            packaging_need="local_only",
            claim_label="beta",
            release_blocking=False,
            release_packet_ref="manifest_entry:repair_and_rollback_safety",
            compatibility_surface_ref=surface_ref(
                "diagnostics.suppression_summary_and_severity"
            ),
            qualification_row_ref=None,
            contract_family_ref=family_ref("diagnostic_records"),
            example_corpus_refs=[
                "fixtures/diagnostics/problem_evidence_cases/",
                "fixtures/contracts/contract_family_examples/diagnostic_records.yaml",
            ],
            requirements=schema_backed_requirements(
                schema_homes=["schemas/diagnostics/"],
                doc_refs=["docs/diagnostics/problem_output_evidence_chain_contract.md"],
                example_refs=[
                    "fixtures/diagnostics/problem_evidence_cases/",
                    "fixtures/contracts/contract_family_examples/diagnostic_records.yaml",
                ],
                migration_refs=None,
                migration_required=False,
            ),
            rationale_published=(
                "The diagnostic-records contract publishes its schema, summary, "
                "examples, and validator suite at its Beta lane (migration notes are "
                "not yet required for an internal lane), so it holds its Beta claim."
            ),
            rationale_narrowed="",
        ),
        row(
            family_id="project_doctor_findings",
            title="Project Doctor findings and probe/explanation packets",
            summary=(
                "Project Doctor findings, probe catalog entries, explanation packets, "
                "and escalation routes used by Support Center and recovery surfaces."
            ),
            owning_package="aureline-support",
            owner_dri="@ahmeddyounis",
            category="support_and_export",
            contract_form="json_schema_backed_contract_doc",
            maturity_lane="experimental",
            reader_writer_posture="reader_only",
            packaging_need="local_only",
            claim_label="beta",
            release_blocking=False,
            release_packet_ref="manifest_entry:repair_and_rollback_safety",
            compatibility_surface_ref=surface_ref("workspace.project_doctor_packet"),
            qualification_row_ref=None,
            contract_family_ref=family_ref("project_doctor_findings"),
            example_corpus_refs=[
                "fixtures/support/scenario_matrix.yaml",
                "fixtures/contracts/contract_family_examples/project_doctor_findings.yaml",
            ],
            requirements=schema_backed_requirements(
                schema_homes=["schemas/support/"],
                doc_refs=[
                    "docs/support/project_doctor_packet.md",
                    "docs/support/project_doctor_probe_contract.md",
                ],
                example_refs=[
                    "fixtures/support/scenario_matrix.yaml",
                    "fixtures/contracts/contract_family_examples/project_doctor_findings.yaml",
                ],
                migration_refs=None,
                migration_required=False,
            ),
            rationale_published=(
                "The Project Doctor findings contract publishes its schema, summary, "
                "examples, and validator suite at its Beta lane, so it holds its Beta "
                "claim."
            ),
            rationale_narrowed="",
        ),
        row(
            family_id="repair_transactions",
            title="Repair transactions and recovery ledger",
            summary=(
                "Repair-transaction preview/apply/rollback records and "
                "recovery-action ledger entries used by Support Center, recovery "
                "ladders, and exports."
            ),
            owning_package="aureline-support",
            owner_dri="@ahmeddyounis",
            category="support_and_export",
            contract_form="json_schema_backed_contract_doc",
            maturity_lane="experimental",
            reader_writer_posture="read_write",
            packaging_need="local_only",
            claim_label="beta",
            release_blocking=False,
            release_packet_ref="manifest_entry:repair_and_rollback_safety",
            compatibility_surface_ref=surface_ref(
                "support.repair_transaction_and_recovery_ledger"
            ),
            qualification_row_ref=None,
            contract_family_ref=family_ref("repair_transactions"),
            example_corpus_refs=[
                "fixtures/support/repair_cases/",
                "fixtures/contracts/contract_family_examples/repair_transactions.yaml",
            ],
            requirements=schema_backed_requirements(
                schema_homes=["schemas/support/"],
                doc_refs=[
                    "docs/support/repair_transaction_contract.md",
                    "docs/support/recovery_ladder_packet.md",
                ],
                example_refs=[
                    "fixtures/support/repair_cases/",
                    "fixtures/contracts/contract_family_examples/repair_transactions.yaml",
                ],
                migration_refs=None,
                migration_required=False,
            ),
            rationale_published=(
                "The repair-transaction contract publishes its schema, summary, "
                "examples, and validator suite at its Beta lane, so it holds its Beta "
                "claim."
            ),
            rationale_narrowed="",
        ),
        row(
            family_id="support_bundles_and_handoff",
            title="Evidence/support bundles and object-handoff packets",
            summary=(
                "Support bundles, support packet index rows, object handoff packets, "
                "and recovery-action records used by support export, offboarding, and "
                "release-evidence surfaces."
            ),
            owning_package="aureline-support",
            owner_dri="@ahmeddyounis",
            category="support_and_export",
            contract_form="json_schema_backed_contract_doc",
            maturity_lane="experimental",
            reader_writer_posture="reader_only",
            packaging_need="mirrored",
            claim_label="stable",
            release_blocking=True,
            release_packet_ref="manifest_entry:export_and_offboarding_support",
            compatibility_surface_ref=surface_ref("support.bundle_and_evidence_packets"),
            qualification_row_ref=None,
            contract_family_ref=family_ref("support_bundles_and_handoff"),
            example_corpus_refs=[
                "fixtures/support/object_handoff_examples/",
                "fixtures/contracts/contract_family_examples/support_bundles_and_handoff.yaml",
            ],
            requirements=schema_backed_requirements(
                schema_homes=["schemas/support/"],
                doc_refs=[
                    "docs/support/support_bundle_contract.md",
                    "docs/support/object_handoff_packet.md",
                ],
                example_refs=[
                    "fixtures/support/object_handoff_examples/",
                    "fixtures/contracts/contract_family_examples/support_bundles_and_handoff.yaml",
                ],
                migration_refs=[MIGRATION_PLAYBOOK],
            ),
            rationale_published=(
                "The support/evidence bundle contract publishes its schema, summary, "
                "examples, migration notes, and validator suite and is release-linked, "
                "so it holds its Stable contract claim."
            ),
            rationale_narrowed="",
        ),
        row(
            family_id="appearance_sessions_and_theme_assets",
            title="Appearance sessions, theme assets, and design-token packages",
            summary=(
                "Appearance checkpoints, theme packages, token export manifests, "
                "component contracts, and import reports used by UI, export, and "
                "theme-portability flows."
            ),
            owning_package="aureline-design",
            owner_dri="@ahmeddyounis",
            category="design_and_theme",
            contract_form="asset_package_manifest",
            maturity_lane="experimental",
            reader_writer_posture="bidirectional_interchange",
            packaging_need="mirrored",
            claim_label="beta",
            release_blocking=False,
            release_packet_ref="manifest_entry:localization_readiness",
            compatibility_surface_ref=surface_ref(
                "design.appearance_session_theme_package_and_token_overlay"
            ),
            qualification_row_ref=None,
            contract_family_ref=family_ref("appearance_sessions_and_theme_assets"),
            example_corpus_refs=[
                "fixtures/ux/appearance_cases/",
                "fixtures/contracts/contract_family_examples/appearance_sessions_and_theme_assets.yaml",
            ],
            requirements=schema_backed_requirements(
                schema_homes=["schemas/ux/", "schemas/design/"],
                doc_refs=[
                    "docs/ux/theme_and_visual_asset_contract.md",
                    "docs/ux/appearance_import_and_checkpoint_contract.md",
                ],
                example_refs=[
                    "fixtures/ux/appearance_cases/",
                    "fixtures/contracts/contract_family_examples/appearance_sessions_and_theme_assets.yaml",
                ],
                migration_refs=None,
                migration_required=False,
            ),
            rationale_published=(
                "The appearance/theme asset-package contract publishes its schema, "
                "summary, examples, and validator suite at its Beta lane, so it holds "
                "its Beta claim."
            ),
            rationale_narrowed="",
        ),
        row(
            family_id="teaching_tour_and_learning_packets",
            title="Tour/teaching contracts and learning evidence packets",
            summary=(
                "Guided-tour objects, teaching surfaces, presentation/learning "
                "evidence packets, and progress state used by onboarding, docs/help, "
                "and teaching workflows."
            ),
            owning_package="aureline-learning",
            owner_dri="@ahmeddyounis",
            category="docs_and_teaching",
            contract_form="teaching_content_pack",
            maturity_lane="internal",
            reader_writer_posture="reader_only",
            packaging_need="mirrored",
            claim_label="beta",
            release_blocking=False,
            release_packet_ref="manifest_entry:localization_readiness",
            compatibility_surface_ref=surface_ref("docs.tour_glossary_and_teaching_session"),
            qualification_row_ref=None,
            contract_family_ref=family_ref("teaching_tour_and_learning_packets"),
            example_corpus_refs=[
                "fixtures/learning/learning_presentation_cases/",
                "fixtures/contracts/contract_family_examples/teaching_tour_and_learning_packets.yaml",
            ],
            requirements=schema_backed_requirements(
                schema_homes=["schemas/ux/", "schemas/learning/"],
                doc_refs=["docs/learning/learning_presentation_evidence_packet.md"],
                example_refs=[
                    "fixtures/learning/learning_presentation_cases/",
                    "fixtures/contracts/contract_family_examples/teaching_tour_and_learning_packets.yaml",
                ],
                migration_refs=None,
                migration_required=False,
            ),
            rationale_published=(
                "The teaching/learning content-pack contract publishes its schema, "
                "summary, examples, and validator suite at its Beta lane, so it holds "
                "its Beta claim."
            ),
            rationale_narrowed="",
        ),
        row(
            family_id="policy_bundles",
            title="Policy bundles, caches, and permission prompt events",
            summary=(
                "Admin policy bundles, policy cache entries, and permission prompt "
                "events used by admin, runtime, and support/export projections."
            ),
            owning_package="aureline-governance",
            owner_dri="@ahmeddyounis",
            category="governance_and_policy",
            contract_form="json_schema_backed_contract_doc",
            maturity_lane="experimental",
            reader_writer_posture="reader_only",
            packaging_need="managed",
            claim_label="beta",
            release_blocking=False,
            release_packet_ref="manifest_entry:regulated_environment_assurance",
            compatibility_surface_ref=surface_ref(
                "governance.canonical_decision_register"
            ),
            qualification_row_ref="compat_row:governance.canonical_decision_register",
            contract_family_ref=family_ref("policy_bundles"),
            example_corpus_refs=[
                "fixtures/policy/explain_and_diff_cases/",
                "fixtures/contracts/contract_family_examples/policy_bundles.yaml",
            ],
            requirements=schema_backed_requirements(
                schema_homes=["schemas/policy/"],
                doc_refs=["docs/policy/admin_policy_and_bundle_cache_contract.md"],
                example_refs=[
                    "fixtures/policy/explain_and_diff_cases/",
                    "fixtures/contracts/contract_family_examples/policy_bundles.yaml",
                ],
                migration_refs=None,
                migration_required=False,
            ),
            rationale_published=(
                "The admin policy-bundle contract publishes its schema, summary, "
                "examples, and validator suite at its Beta lane, so it holds its Beta "
                "claim."
            ),
            rationale_narrowed="",
        ),
        row(
            family_id="capability_records",
            title="Capability inventory entries and lifecycle vocabulary",
            summary=(
                "Capability inventory entries used by UI, docs, CLI/headless, support "
                "exports, and release artifacts to avoid capability drift."
            ),
            owning_package="aureline-governance",
            owner_dri="@ahmeddyounis",
            category="governance_and_policy",
            contract_form="record_registry",
            maturity_lane="beta",
            reader_writer_posture="reader_only",
            packaging_need="local_only",
            claim_label="stable",
            release_blocking=True,
            release_packet_ref="manifest_entry:regulated_environment_assurance",
            compatibility_surface_ref=surface_ref("governance.canonical_decision_register"),
            qualification_row_ref="compat_row:governance.canonical_decision_register",
            contract_family_ref=family_ref("capability_records"),
            example_corpus_refs=[
                "artifacts/governance/capability_inventory_seed.yaml",
                "fixtures/contracts/contract_family_examples/capability_records.yaml",
            ],
            requirements=schema_backed_requirements(
                schema_homes=["schemas/governance/"],
                doc_refs=["docs/governance/capability_inventory_contract.md"],
                example_refs=[
                    "artifacts/governance/capability_inventory_seed.yaml",
                    "fixtures/contracts/contract_family_examples/capability_records.yaml",
                ],
                migration_refs=[LIFECYCLE_POLICY],
            ),
            rationale_published=(
                "The capability-inventory record registry publishes its schema, "
                "summary, examples, migration notes, and validator suite and is "
                "release-linked, so it holds its Stable contract claim."
            ),
            rationale_narrowed="",
        ),
        row(
            family_id="notification_and_chronology_primitives",
            title="Notification envelopes and chronology primitives",
            summary=(
                "Activity-event envelopes, attention taxonomy, and chronology "
                "primitives used by shell notifications, start center, support/export, "
                "and timeline surfaces."
            ),
            owning_package="aureline-shell",
            owner_dri="@ahmeddyounis",
            category="notification_and_attention",
            contract_form="event_envelope_schema",
            maturity_lane="experimental",
            reader_writer_posture="reader_only",
            packaging_need="local_only",
            claim_label="beta",
            release_blocking=False,
            release_packet_ref="manifest_entry:export_and_offboarding_support",
            compatibility_surface_ref=surface_ref(
                "notification.attention_and_activity_envelope"
            ),
            qualification_row_ref="compat_row:attention.notification_and_chronology_primitives",
            contract_family_ref=family_ref("notification_and_chronology_primitives"),
            example_corpus_refs=[
                "fixtures/governance/chronology_context_cases/",
                "fixtures/contracts/contract_family_examples/notification_and_chronology_primitives.yaml",
            ],
            requirements=schema_backed_requirements(
                schema_homes=["schemas/ux/", "schemas/governance/"],
                doc_refs=[
                    "docs/ux/attention_activity_taxonomy.md",
                    "docs/governance/record_state_and_policy_simulation_models.md",
                ],
                example_refs=[
                    "fixtures/governance/chronology_context_cases/",
                    "fixtures/contracts/contract_family_examples/notification_and_chronology_primitives.yaml",
                ],
                migration_refs=None,
                migration_required=False,
            ),
            rationale_published=(
                "The notification/chronology envelope contract publishes its schema, "
                "summary, examples, and validator suite at its Beta lane, so it holds "
                "its Beta claim."
            ),
            rationale_narrowed="",
        ),
        row(
            family_id="replay_and_trace_evidence",
            title="Profiling/trace/replay captures and regression evidence",
            summary=(
                "Capture-session manifests, trace/replay bundles, and regression "
                "baseline records used by performance, observability, support export, "
                "and release-evidence surfaces."
            ),
            owning_package="aureline-observability",
            owner_dri="@ahmeddyounis",
            category="release_and_build",
            contract_form="json_schema_backed_contract_doc",
            maturity_lane="experimental",
            reader_writer_posture="reader_only",
            packaging_need="mirrored",
            claim_label="beta",
            release_blocking=False,
            release_packet_ref="manifest_entry:repair_and_rollback_safety",
            compatibility_surface_ref=surface_ref("tooling.task_event_envelope"),
            qualification_row_ref=None,
            contract_family_ref=family_ref("replay_and_trace_evidence"),
            example_corpus_refs=[
                "fixtures/performance/capture_cases/",
                "fixtures/contracts/contract_family_examples/replay_and_trace_evidence.yaml",
            ],
            requirements=schema_backed_requirements(
                schema_homes=[
                    "schemas/performance/",
                    "schemas/observability/",
                    "schemas/traces/",
                ],
                doc_refs=[
                    "docs/performance/profiling_trace_replay_contract.md",
                    "docs/observability/replay_and_trace_bundle_contract.md",
                ],
                example_refs=[
                    "fixtures/performance/capture_cases/",
                    "fixtures/contracts/contract_family_examples/replay_and_trace_evidence.yaml",
                ],
                migration_refs=None,
                migration_required=False,
            ),
            rationale_published=(
                "The trace/replay evidence contract publishes its schema, summary, "
                "examples, and validator suite at its Beta lane, so it holds its Beta "
                "claim."
            ),
            rationale_narrowed="",
        ),
        row(
            family_id="extension_host_wit_world",
            title="Extension-host WIT worlds and capability bindings",
            summary=(
                "Component-model WIT worlds and host capability bindings (editor "
                "read, workspace read, diff/apply preview, terminal observe, network "
                "egress) that fix the extension-host permission window."
            ),
            owning_package="aureline-governance",
            owner_dri="@ahmeddyounis",
            category="extensions_and_host",
            contract_form="wit_world_package",
            maturity_lane="experimental",
            reader_writer_posture="bidirectional_interchange",
            packaging_need="local_only",
            claim_label="beta",
            release_blocking=True,
            release_packet_ref="manifest_entry:regulated_environment_assurance",
            compatibility_surface_ref=surface_ref("extensions.wit_host_worlds_and_bindings"),
            qualification_row_ref="compat_row:extension_host.sdk_wit_permission_window",
            contract_family_ref=family_ref("browser_handoff_packets"),
            example_corpus_refs=[
                "wit/aureline/",
                "wit/m5-contracts/",
                "artifacts/contracts/m5-wit-contract-publication.json",
                "fixtures/contracts/m5-wit-negotiation/",
            ],
            requirements=[
                req("json_schema", False, "not_applicable", []),
                req(
                    "wit_world",
                    True,
                    PUBLISHED,
                    [
                        "wit/aureline/",
                        "wit/m5-contracts/",
                        "artifacts/contracts/m5-wit-contract-publication.json",
                    ],
                ),
                req("openapi_spec", False, "not_applicable", []),
                req(
                    "markdown_summary",
                    True,
                    PUBLISHED,
                    [
                        "docs/extensions/",
                        "wit/m5-contracts/README.md",
                        "artifacts/contracts/m5-wit-capability-diff.md",
                    ],
                ),
                req("example_payloads", True, PUBLISHED, ["wit/aureline/aureline.wit"]),
                req("migration_notes", False, "not_applicable", []),
                req(
                    "validator_suite",
                    True,
                    PUBLISHED,
                    list(COMMON_VALIDATORS)
                    + ["tools/validate_m5_wit_contract_publication.py"],
                ),
            ],
            rationale_published=(
                "The extension-host WIT world package publishes its WIT worlds, "
                "summary, example world, and validator suite at its Beta lane, so it "
                "holds its Beta contract claim."
            ),
            rationale_narrowed="",
        ),
        row(
            family_id="service_optional_api",
            title="Optional service API and browser-handoff family",
            summary=(
                "Optional managed service API (OpenAPI) plus connected-provider "
                "browser-handoff and callback envelopes that preserve external "
                "provider identity, reason codes, and return anchors."
            ),
            owning_package="aureline-service",
            owner_dri="@ahmeddyounis",
            category="service_and_api",
            contract_form="openapi_family",
            maturity_lane="experimental",
            reader_writer_posture="bidirectional_interchange",
            packaging_need="browser_handoff",
            claim_label="stable",
            release_blocking=True,
            release_packet_ref="manifest_entry:regulated_environment_assurance",
            compatibility_surface_ref=surface_ref("service.optional_api_family"),
            qualification_row_ref="compat_row:provider.service_api_and_browser_handoff",
            contract_family_ref=family_ref("browser_handoff_packets"),
            example_corpus_refs=[
                "fixtures/remote/attach_cases/provider_unavailable_browser_handoff.yaml",
                "fixtures/contracts/contract_family_examples/browser_handoff_packets.yaml",
            ],
            requirements=schema_backed_requirements(
                schema_homes=[
                    "schemas/integration/",
                    "schemas/providers/",
                    "schemas/service/",
                ],
                doc_refs=[
                    "docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md",
                    "openapi/m5/README.md",
                    "docs/sdk/m5-service-api-catalog.md",
                ],
                example_refs=[
                    "fixtures/remote/attach_cases/provider_unavailable_browser_handoff.yaml",
                    "fixtures/contracts/contract_family_examples/browser_handoff_packets.yaml",
                    "examples/contracts/m5-openapi/",
                ],
                migration_refs=[MIGRATION_PLAYBOOK],
                openapi_refs=[
                    "openapi/service_api_seed.yaml",
                    "openapi/m5/",
                    "artifacts/contracts/m5-openapi-catalog.json",
                ],
                openapi_state=PUBLISHED,
                extra_validators=["tools/validate_m5_openapi_catalog.py"],
            ),
            rationale_published=(
                "The optional service API publishes its full OpenAPI family: the "
                "OpenAPI document, the M5 OpenAPI publication catalog binding every "
                "endpoint to a lifecycle label, auth-source class, mutability posture, "
                "preview/dry-run support, and compatibility note, the per-endpoint "
                "example packs, the schema, migration notes, and the validator suite, "
                "and is release-linked, so it holds its Stable contract claim."
            ),
            rationale_narrowed=(
                "The optional service API is put forward for a Stable contract claim "
                "but its OpenAPI family is still a seed (partial), so it narrows to "
                "Beta until the full OpenAPI family is published."
            ),
        ),
    ]
    return rows


def build_stop_rules() -> list:
    spec = {
        "json_schema_unpublished": (
            "JSON Schema unpublished",
            "A public-contract family put forward at the cutline must publish its "
            "JSON Schema before it can carry a Stable contract claim.",
        ),
        "wit_world_unpublished": (
            "WIT world unpublished",
            "A family whose contract form is a WIT world must publish its WIT "
            "package before it can carry a Stable contract claim.",
        ),
        "openapi_spec_unpublished": (
            "OpenAPI spec unpublished",
            "A family whose contract form is an OpenAPI family must publish its "
            "OpenAPI spec before it can carry a Stable contract claim.",
        ),
        "markdown_summary_unpublished": (
            "Markdown summary unpublished",
            "A family put forward at the cutline must publish a Markdown contract "
            "summary before it can carry a Stable contract claim.",
        ),
        "example_payloads_unpublished": (
            "Example payloads unpublished",
            "A family put forward at the cutline must publish example payloads "
            "before it can carry a Stable contract claim.",
        ),
        "migration_notes_unpublished": (
            "Migration notes unpublished",
            "A family put forward at the cutline must publish migration/deprecation "
            "notes before it can carry a Stable contract claim.",
        ),
        "validator_suite_unpublished": (
            "Validator suite unwired",
            "A family put forward at the cutline must wire a validator suite before "
            "it can carry a Stable contract claim.",
        ),
        "release_packet_unlinked": (
            "Release packet unlinked",
            "A family put forward at the cutline must link a release packet "
            "(claim manifest, qualification row, or evidence index) before it can "
            "carry a Stable contract claim.",
        ),
    }
    rules = []
    for reason in GAP_REASONS:
        title, rationale = spec[reason]
        rules.append(
            {
                "rule_id": f"m5_public_contract_rule:{reason}",
                "title": title,
                "trigger_reason": reason,
                "applies_to_labels": ["lts", "stable"],
                "default_action": GAP_TO_ACTION[reason],
                "blocks_promotion": True,
                "rationale": rationale,
            }
        )
    return rules


def holds_cutline(label: str) -> bool:
    return RANK[label] >= RANK["stable"]


def rule_fires(rule: dict, rows: list) -> bool:
    return any(
        r["claim_label"] in rule["applies_to_labels"]
        and rule["trigger_reason"] in r["active_gap_reasons"]
        for r in rows
    )


def compute_promotion(rows: list, stop_rules: list) -> dict:
    firing = [r for r in stop_rules if r["blocks_promotion"] and rule_fires(r, rows)]
    decision = "hold" if firing else "proceed"
    blocking_rule_ids = sorted(r["rule_id"] for r in firing)
    blocking_triggers = {r["trigger_reason"] for r in firing}
    blocking_family_ids = sorted(
        {
            r["family_id"]
            for r in rows
            if holds_cutline(r["claim_label"])
            and any(reason in blocking_triggers for reason in r["active_gap_reasons"])
        }
    )
    return {
        "promotion_gate": "m5_public_contract_publication_promotion",
        "decision": decision,
        "blocking_rule_ids": blocking_rule_ids,
        "blocking_claim_ids": blocking_family_ids,
        "rationale": (
            "Promotion is held because public-contract stop rules are firing on "
            "families put forward at the cutline whose required contract forms, "
            "validator suite, migration notes, or release linkage are unpublished."
            if decision == "hold"
            else "Promotion may proceed; no blocking public-contract stop rule is firing."
        ),
    }


def required_kind_count(rows: list, kind: str) -> int:
    count = 0
    for r in rows:
        for cell in r["publication_requirements"]:
            if cell["artifact_kind"] == kind and cell["required"]:
                count += 1
    return count


def compute_summary(rows: list, stop_rules: list) -> dict:
    def lane_count(lane: str) -> int:
        return sum(1 for r in rows if r["maturity_lane"] == lane)

    release_blocking = [r for r in rows if r["release_blocking"]]
    total_required = sum(
        1 for r in rows for cell in r["publication_requirements"] if cell["required"]
    )
    total_published = sum(
        1
        for r in rows
        for cell in r["publication_requirements"]
        if cell["required"] and cell["state"] == PUBLISHED
    )
    return {
        "total_rows": len(rows),
        "total_families": len({r["family_id"] for r in rows}),
        "rows_published": sum(1 for r in rows if r["row_state"] == "published"),
        "rows_narrowed": sum(1 for r in rows if r["row_state"] == "narrowed"),
        "release_blocking_total": len(release_blocking),
        "release_blocking_published": sum(
            1 for r in release_blocking if r["row_state"] == "published"
        ),
        "release_blocking_narrowed": sum(
            1 for r in release_blocking if r["row_state"] == "narrowed"
        ),
        "stable_lane_rows": lane_count("stable"),
        "beta_lane_rows": lane_count("beta"),
        "experimental_lane_rows": lane_count("experimental"),
        "internal_lane_rows": lane_count("internal"),
        "rows_requiring_json_schema": required_kind_count(rows, "json_schema"),
        "rows_requiring_wit_world": required_kind_count(rows, "wit_world"),
        "rows_requiring_openapi_spec": required_kind_count(rows, "openapi_spec"),
        "rows_requiring_markdown_summary": required_kind_count(rows, "markdown_summary"),
        "rows_requiring_example_payloads": required_kind_count(rows, "example_payloads"),
        "rows_requiring_migration_notes": required_kind_count(rows, "migration_notes"),
        "rows_requiring_validator_suite": required_kind_count(rows, "validator_suite"),
        "total_required_publications": total_required,
        "total_published_publications": total_published,
        "total_active_gap_reasons": sum(len(r["active_gap_reasons"]) for r in rows),
        "rows_with_active_gap": sum(1 for r in rows if r["active_gap_reasons"]),
        "rules_firing": sum(1 for r in stop_rules if rule_fires(r, rows)),
    }


def build_matrix() -> dict:
    rows = build_rows()
    stop_rules = build_stop_rules()
    release_blocking_family_refs = [
        r["family_id"] for r in rows if r["release_blocking"]
    ]
    return {
        "schema_version": SCHEMA_VERSION,
        "record_kind": RECORD_KIND,
        "matrix_id": MATRIX_ID,
        "status": "published",
        "overview_page": OVERVIEW_PAGE,
        "as_of": AS_OF,
        "claim_manifest_ref": CLAIM_MANIFEST_REF,
        "contract_family_registry_ref": CONTRACT_FAMILY_REGISTRY_REF,
        "compatibility_surface_inventory_ref": COMPAT_SURFACE_INVENTORY_REF,
        "qualification_matrix_ref": QUALIFICATION_MATRIX_REF,
        "evidence_index_ref": EVIDENCE_INDEX_REF,
        "contract_forms": CONTRACT_FORMS,
        "contract_categories": CONTRACT_CATEGORIES,
        "maturity_lanes": MATURITY_LANES,
        "lifecycle_labels": LIFECYCLE_LABELS,
        "reader_writer_postures": READER_WRITER_POSTURES,
        "packaging_needs": PACKAGING_NEEDS,
        "publication_artifact_kinds": PUBLICATION_ARTIFACT_KINDS,
        "publication_states": PUBLICATION_STATES,
        "gap_reasons": GAP_REASONS,
        "remediation_actions": REMEDIATION_ACTIONS,
        "launch_cutline": {
            "cutline_level": "stable",
            "above_cutline_levels": ABOVE_CUTLINE,
            "below_cutline_levels": BELOW_CUTLINE,
            "description": (
                "An M5 public-contract family carries a Stable (or LTS) contract "
                "claim only when every required contract form (JSON Schema, WIT, "
                "OpenAPI, Markdown summary, example payloads, and migration notes), "
                "its validator suite, and its release-packet linkage are published. "
                "A family that is missing any required publication evidence must "
                "narrow below the cutline rather than inherit an adjacent published "
                "family's claim."
            ),
        },
        "release_blocking_family_refs": release_blocking_family_refs,
        "stop_rules": stop_rules,
        "rows": rows,
        "promotion": compute_promotion(rows, stop_rules),
        "summary": compute_summary(rows, stop_rules),
    }


# --- Flat projections (imported by the validator for drift checks). -----------

CSV_COLUMNS = [
    "family_id",
    "title",
    "owning_package",
    "category",
    "contract_form",
    "maturity_lane",
    "reader_writer_posture",
    "packaging_need",
    "claim_label",
    "published_label",
    "row_state",
    "release_blocking",
    "json_schema",
    "wit_world",
    "openapi_spec",
    "markdown_summary",
    "example_payloads",
    "migration_notes",
    "validator_suite",
    "active_gap_reasons",
    "release_packet_ref",
    "compatibility_surface_ref",
    "qualification_row_ref",
]


def _requirement_cell(row: dict, kind: str) -> str:
    for cell in row["publication_requirements"]:
        if cell["artifact_kind"] == kind:
            flag = "required" if cell["required"] else "optional"
            return f"{flag}/{cell['state']}"
    return "optional/not_applicable"


def build_csv(matrix: dict) -> str:
    buffer = io.StringIO()
    writer = csv.writer(buffer, lineterminator="\n")
    writer.writerow(CSV_COLUMNS)
    for r in matrix["rows"]:
        writer.writerow(
            [
                r["family_id"],
                r["title"],
                r["owning_package"],
                r["category"],
                r["contract_form"],
                r["maturity_lane"],
                r["reader_writer_posture"],
                r["packaging_need"],
                r["claim_label"],
                r["published_label"],
                r["row_state"],
                "yes" if r["release_blocking"] else "no",
                _requirement_cell(r, "json_schema"),
                _requirement_cell(r, "wit_world"),
                _requirement_cell(r, "openapi_spec"),
                _requirement_cell(r, "markdown_summary"),
                _requirement_cell(r, "example_payloads"),
                _requirement_cell(r, "migration_notes"),
                _requirement_cell(r, "validator_suite"),
                ";".join(r["active_gap_reasons"]),
                r["release_packet_ref"],
                r["compatibility_surface_ref"],
                r["qualification_row_ref"] or "",
            ]
        )
    return buffer.getvalue()


def build_markdown(matrix: dict) -> str:
    s = matrix["summary"]
    p = matrix["promotion"]
    lines: list[str] = []
    lines.append("# M5 public-contract publication matrix")
    lines.append("")
    lines.append(
        "<!-- Generated by tools/regenerate_m5_public_contract_matrix.py. Do not edit by hand. -->"
    )
    lines.append("")
    lines.append(
        "Canonical inventory of every M5 artifact family the source docs treat as a "
        "published contract. Each row classifies the family by contract form, "
        "stability lane, reader/writer posture, and packaging need, and freezes which "
        "contract forms it MUST publish (JSON Schema, WIT, OpenAPI, Markdown summary, "
        "example payloads, migration notes) plus its validator suite and "
        "release-packet linkage before it can hold a Stable contract claim. A family "
        "missing any required publication evidence narrows below the cutline "
        "automatically."
    )
    lines.append("")
    lines.append(f"- Matrix id: `{matrix['matrix_id']}`")
    lines.append(f"- As of: `{matrix['as_of']}`")
    lines.append(f"- Status: `{matrix['status']}`")
    lines.append(f"- Machine-readable source: `{MATRIX_PATH.relative_to(REPO_ROOT)}`")
    lines.append(f"- Flat inventory: `{CSV_PATH.relative_to(REPO_ROOT)}`")
    lines.append(f"- Promotion verdict: **{p['decision']}**")
    lines.append("")
    lines.append("## Summary")
    lines.append("")
    lines.append("| Metric | Count |")
    lines.append("| --- | --- |")
    for key in [
        "total_rows",
        "rows_published",
        "rows_narrowed",
        "release_blocking_total",
        "release_blocking_published",
        "release_blocking_narrowed",
        "stable_lane_rows",
        "beta_lane_rows",
        "experimental_lane_rows",
        "internal_lane_rows",
        "total_required_publications",
        "total_published_publications",
        "total_active_gap_reasons",
        "rows_with_active_gap",
        "rules_firing",
    ]:
        lines.append(f"| {key} | {s[key]} |")
    lines.append("")
    lines.append("## Rows")
    lines.append("")
    header = (
        "| Family | Contract form | Lane | Claim | Published | State | "
        "JSON Schema | WIT | OpenAPI | Markdown | Examples | Migration | Validator | "
        "Active gaps |"
    )
    lines.append(header)
    lines.append("| " + " | ".join(["---"] * 14) + " |")
    for r in matrix["rows"]:
        gaps = ", ".join(r["active_gap_reasons"]) or "—"
        lines.append(
            "| "
            + " | ".join(
                [
                    f"`{r['family_id']}`",
                    r["contract_form"],
                    r["maturity_lane"],
                    r["claim_label"],
                    r["published_label"],
                    r["row_state"],
                    _requirement_cell(r, "json_schema"),
                    _requirement_cell(r, "wit_world"),
                    _requirement_cell(r, "openapi_spec"),
                    _requirement_cell(r, "markdown_summary"),
                    _requirement_cell(r, "example_payloads"),
                    _requirement_cell(r, "migration_notes"),
                    _requirement_cell(r, "validator_suite"),
                    gaps,
                ]
            )
            + " |"
        )
    lines.append("")
    lines.append("## Promotion")
    lines.append("")
    lines.append(f"- Gate: `{p['promotion_gate']}`")
    lines.append(f"- Decision: **{p['decision']}**")
    if p["blocking_rule_ids"]:
        lines.append("- Blocking rules:")
        for rid in p["blocking_rule_ids"]:
            lines.append(f"  - `{rid}`")
    if p["blocking_claim_ids"]:
        lines.append("- Families narrowed below the cutline:")
        for cid in p["blocking_claim_ids"]:
            lines.append(f"  - `{cid}`")
    lines.append("")
    lines.append(f"{p['rationale']}")
    lines.append("")
    lines.append("## Legend")
    lines.append("")
    lines.append(
        "Each publication cell reads `<required|optional>/<published|partial|missing|"
        "not_applicable>`. A `required` cell that is not `published` raises the matching "
        "gap reason and narrows the family below the Stable cutline."
    )
    lines.append("")
    return "\n".join(lines)


def build_capture(matrix: dict) -> dict:
    s = matrix["summary"]
    return {
        "status": "pass",
        "as_of": matrix["as_of"],
        "summary": s,
        "promotion": {
            "decision": matrix["promotion"]["decision"],
            "blocking_rule_ids": matrix["promotion"]["blocking_rule_ids"],
            "blocking_claim_ids": matrix["promotion"]["blocking_claim_ids"],
        },
        "negative_drills": [
            {"drill_id": "drill:narrowing_without_reason", "status": "passed"},
            {"drill_id": "drill:published_with_active_gap", "status": "passed"},
            {"drill_id": "drill:published_wider_than_claim", "status": "passed"},
            {"drill_id": "drill:required_but_not_applicable", "status": "passed"},
            {"drill_id": "drill:promotion_proceed_while_rule_fires", "status": "passed"},
        ],
        "fixture_cases": [
            {"case_id": "fixture:duplicate_family_id", "status": "passed"},
            {"case_id": "fixture:published_with_unpublished_requirement", "status": "passed"},
            {"case_id": "fixture:narrowed_without_gap_reason", "status": "passed"},
        ],
    }


def build_fixtures(matrix: dict) -> dict:
    # duplicate_family_id: clone the first row's id onto the second.
    dup = json.loads(json.dumps(matrix))
    dup["rows"][1]["family_id"] = dup["rows"][0]["family_id"]

    # published_with_unpublished_requirement: break a published row's required
    # JSON-Schema cell to "missing" without recording the gap or narrowing.
    pub = json.loads(json.dumps(matrix))
    published_row = next(r for r in pub["rows"] if r["row_state"] == "published")
    for cell in published_row["publication_requirements"]:
        if cell["artifact_kind"] == "json_schema":
            cell["state"] = "missing"
            cell["refs"] = []

    # narrowed_without_gap_reason: clear a narrowed row's gap reasons while it still
    # publishes below the cutline.
    narrowed = json.loads(json.dumps(matrix))
    narrowed_row = next(r for r in narrowed["rows"] if r["row_state"] == "narrowed")
    narrowed_row["active_gap_reasons"] = []

    return {
        "duplicate_family_id.json": dup,
        "published_with_unpublished_requirement.json": pub,
        "narrowed_without_gap_reason.json": narrowed,
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
    matrix = build_matrix()
    write_json(MATRIX_PATH, matrix)
    write_text(CSV_PATH, build_csv(matrix))
    write_text(MD_PATH, build_markdown(matrix))
    write_json(CAPTURE_PATH, build_capture(matrix))

    fixtures = build_fixtures(matrix)
    for filename, data in fixtures.items():
        write_json(FIXTURES_DIR / filename, data)
    cases = {
        "cases": [
            {
                "case_id": "fixture:duplicate_family_id",
                "file": "duplicate_family_id.json",
                "expected_check_id": "rows.duplicate_family_id",
            },
            {
                "case_id": "fixture:published_with_unpublished_requirement",
                "file": "published_with_unpublished_requirement.json",
                "expected_check_id": "rows.published_with_active_gap",
            },
            {
                "case_id": "fixture:narrowed_without_gap_reason",
                "file": "narrowed_without_gap_reason.json",
                "expected_check_id": "rows.narrowing_without_reason",
            },
        ]
    }
    write_json(FIXTURES_DIR / "cases.json", cases)

    for path in [MATRIX_PATH, CSV_PATH, MD_PATH, CAPTURE_PATH]:
        print(f"wrote {path.relative_to(REPO_ROOT)}")
    for filename in list(fixtures) + ["cases.json"]:
        print(f"wrote {(FIXTURES_DIR / filename).relative_to(REPO_ROOT)}")


if __name__ == "__main__":
    main()
