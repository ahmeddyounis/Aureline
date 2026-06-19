#!/usr/bin/env python3
"""Regenerate the M5 extension-host WIT contract publication packet.

This is the single source of truth for the checked-in WIT contract publication
artifacts for the M5 extension-host / bridge-backed public-contract family. It
builds one publication packet that:

  * publishes every reserved capability-world WIT package as a versioned entry
    with lifecycle metadata and a compatibility note (including a worked
    additive-minor successor, ``aureline:editor-read@0.2.0``),
  * records the host/guest negotiation fixtures (supported, downgraded,
    deprecated, and unsupported-skew) that prove capability narrowing,
    deprecated-world handling, and fail-closed unsupported-skew behavior, and
  * records the capability diffs between published versions so extension
    authors, reviewers, and release managers can see what changed without
    reverse-engineering host code.

It writes:

  * ``artifacts/contracts/m5-wit-contract-publication.json``  (the packet)
  * ``artifacts/contracts/m5-wit-capability-diff.md``         (human projection)
  * ``fixtures/contracts/m5-wit-negotiation/{supported,downgraded,deprecated,
    unsupported_skew}.json`` and ``cases.json``               (negotiation fixtures)
  * ``artifacts/release/captures/<name>_validation_capture.json`` (CI capture)

Run ``python3 tools/regenerate_m5_wit_contract_publication.py`` after editing the
package / fixture / diff set, then ``python3
tools/validate_m5_wit_contract_publication.py`` and ``cargo test -p
aureline-extensions --test
implement_versioned_wit_packages_host_guest_negotiation_fixtures_and_capability_diff_reports_for_m5_wasm_extension_and_bridge_backed_public_contracts``
to confirm the validator and the typed Rust consumer agree.

The packet is descriptive metadata. It reuses the ADR-0019 capability-world
registry, the host-negotiation vocabulary, and the M5 public-contract matrix
rather than minting a new contract lexicon. Every field is a typed state or an
opaque repo-relative ref / world identity; the packet carries no raw component
bytes, raw bridge-shim payloads, signing-key material, or policy-bundle bytes.
"""

from __future__ import annotations

import json
from pathlib import Path

NAME = (
    "implement_versioned_wit_packages_host_guest_negotiation_fixtures_and_"
    "capability_diff_reports_for_m5_wasm_extension_and_bridge_backed_public_contracts"
)
RECORD_KIND = "m5_wit_contract_publication"
PACKET_ID = "m5_wit_contract_publication:v1"
SCHEMA_VERSION = 1
AS_OF = "2026-06-19"

REPO_ROOT = Path(__file__).resolve().parent.parent

PACKET_PATH = REPO_ROOT / "artifacts" / "contracts" / "m5-wit-contract-publication.json"
DIFF_MD_PATH = REPO_ROOT / "artifacts" / "contracts" / "m5-wit-capability-diff.md"
FIXTURES_DIR = REPO_ROOT / "fixtures" / "contracts" / "m5-wit-negotiation"
CAPTURE_PATH = (
    REPO_ROOT / "artifacts" / "release" / "captures" / f"{NAME}_validation_capture.json"
)

OVERVIEW_PAGE = f"docs/m5/{NAME.replace('_', '-')}.md"
EVIDENCE_PAGE = f"artifacts/m5/{NAME.replace('_', '-')}.md"
SCHEMA_REF = "schemas/public/m5-contracts/m5_wit_contract_publication.schema.json"

# Cross-cutting sources this packet reuses instead of restating.
CONTRACT_MATRIX_REF = "artifacts/contracts/m5-stability-lifecycle-map.json"
CONTRACT_MATRIX_ROW = "extension_host_wit_world"
CAPABILITY_WORLD_REGISTRY_REF = "artifacts/extensions/capability_worlds.yaml"
NEGOTIATION_SCHEMA_REF = "schemas/extensions/host_negotiation.schema.json"
ADR_REF = "docs/adr/0019-wasm-wit-extension-host-and-capability-worlds.md"
ROOT_PACKAGE_REF = "wit/aureline/aureline.wit"
WIT_INDEX_REF = "wit/m5-contracts/README.md"

# --- Closed vocabularies, in canonical (declaration) order. -----------------
# These must match the `ALL` arrays in the typed Rust model and the schema enums.

# Package-version publication lifecycle. A version can be deprecated while its
# world slug stays active (the registry tracks the slug; this packet tracks the
# published version).
LIFECYCLE_LABELS = ["stable", "beta", "experimental", "deprecated", "retired"]

# Registry status of the underlying world slug (ADR-0019 retirement_policy).
REGISTRY_STATUSES = ["active", "deprecated", "retired"]

# Reuse the M5 public-contract reader/writer posture vocabulary verbatim.
READER_WRITER_POSTURES = [
    "reader_only",
    "writer_only",
    "read_write",
    "bidirectional_interchange",
]

# Reuse the ADR-0019 trust-state gating postures verbatim.
TRUST_STATE_POSTURES = ["admitted_in_restricted", "blocked_in_restricted"]

PUBLICATION_STATES = ["published", "partial", "missing", "not_applicable"]

# Negotiation-outcome classes the fixtures must cover (acceptance).
NEGOTIATION_OUTCOMES = ["supported", "downgraded", "deprecated", "unsupported_skew"]

# Capability-diff change classes, in escalating-impact order.
CHANGE_CLASSES = ["additive_minor", "deprecation", "breaking_major", "retirement"]

# Capability-diff compatibility verdicts.
COMPATIBILITY_VERDICTS = [
    "backward_compatible",
    "deprecated_superseded",
    "breaking",
]

# Guest action a diff requires.
GUEST_ACTIONS = ["none", "upgrade_recommended", "upgrade_required"]

# Reuse the ADR-0019 narrowing-reason vocabulary verbatim.
NARROWING_REASONS = [
    "workspace_trust_restricted",
    "admin_policy_deny_list",
    "admin_policy_permission_floor",
    "admin_policy_egress_host_narrowing",
    "capability_lifecycle_degraded",
    "world_vocabulary_version_unknown",
    "host_abi_range_mismatch",
    "guest_abi_range_mismatch",
    "compatibility_bridge_profile_unbound",
    "budget_declaration_unacceptable",
]

# Reuse the ADR-0019 unsupported-world reason vocabulary verbatim.
UNSUPPORTED_WORLD_REASONS = [
    "world_retired",
    "world_not_shipped_on_host",
    "bridge_refused",
    "host_abi_range_mismatch",
    "guest_abi_range_mismatch",
    "world_vocabulary_version_unknown",
]

CONSUMING_SURFACES = [
    "release_center_wit_publication_inspector",
    "sdk_docs_capability_world_reference",
    "help_center_extension_compatibility",
    "support_export_extension_host_contract",
    "install_review_negotiation_disclosure",
]


# -----------------------------------------------------------------------------
# Versioned WIT packages.
# -----------------------------------------------------------------------------
def registry_ref(identity: str) -> str:
    return f"{CAPABILITY_WORLD_REGISTRY_REF}#{identity}"


def build_packages() -> list[dict]:
    return [
        {
            "package_identity": "aureline:editor-read@0.1.0",
            "world_slug": "editor-read",
            "world_semver": "0.1.0",
            "wit_package_ref": "wit/aureline/editor-read.wit",
            "lifecycle_label": "deprecated",
            "registry_status": "active",
            "reader_writer_posture": "reader_only",
            "trust_state_gating_posture": "admitted_in_restricted",
            "permission_scope_projection": [
                "ui_command_contribute",
                "subscription_subscribe",
            ],
            "supported_host_families": [
                "wasm_component_model",
                "wasm_core_module",
                "external_host_process",
                "helper_binary",
                "remote_side_component",
                "compatibility_bridge",
            ],
            "registry_row_ref": registry_ref("aureline:editor-read@0.1.0"),
            "predecessor_package_ref": None,
            "successor_package_ref": "aureline:editor-read@0.2.0",
            "publication_state": "published",
            "compatibility_note": (
                "Initial published version. Deprecated in favour of "
                "aureline:editor-read@0.2.0, which is additive-minor: every 0.1.0 "
                "item is preserved byte-compatible. The world slug stays active; "
                "only this version is deprecated. Hosts admit 0.1.0 guests with a "
                "deprecation notice and a successor pointer."
            ),
        },
        {
            "package_identity": "aureline:editor-read@0.2.0",
            "world_slug": "editor-read",
            "world_semver": "0.2.0",
            "wit_package_ref": "wit/m5-contracts/editor-read-0.2.0.wit",
            "lifecycle_label": "beta",
            "registry_status": "active",
            "reader_writer_posture": "reader_only",
            "trust_state_gating_posture": "admitted_in_restricted",
            "permission_scope_projection": [
                "ui_command_contribute",
                "subscription_subscribe",
            ],
            "supported_host_families": [
                "wasm_component_model",
                "wasm_core_module",
                "external_host_process",
                "helper_binary",
                "remote_side_component",
                "compatibility_bridge",
            ],
            "registry_row_ref": registry_ref("aureline:editor-read@0.1.0"),
            "predecessor_package_ref": "aureline:editor-read@0.1.0",
            "successor_package_ref": None,
            "publication_state": "published",
            "compatibility_note": (
                "Additive-minor successor to 0.1.0: adds visible-range, word-at, "
                "and the visibility-range record. No item is removed or repurposed, "
                "so a 0.1.0 guest is satisfied by a 0.2.0 host, and a 0.1.0 host "
                "narrows a 0.2.0 guest to the 0.1.0 surface rather than denying it. "
                "The same permission scopes and budgets apply."
            ),
        },
        {
            "package_identity": "aureline:workspace-read@0.1.0",
            "world_slug": "workspace-read",
            "world_semver": "0.1.0",
            "wit_package_ref": "wit/aureline/workspace-read.wit",
            "lifecycle_label": "beta",
            "registry_status": "active",
            "reader_writer_posture": "reader_only",
            "trust_state_gating_posture": "admitted_in_restricted",
            "permission_scope_projection": [
                "filesystem_read",
                "workspace_settings_read",
                "subscription_subscribe",
            ],
            "supported_host_families": [
                "wasm_component_model",
                "wasm_core_module",
                "external_host_process",
                "helper_binary",
                "remote_side_component",
                "compatibility_bridge",
            ],
            "registry_row_ref": registry_ref("aureline:workspace-read@0.1.0"),
            "predecessor_package_ref": None,
            "successor_package_ref": None,
            "publication_state": "published",
            "compatibility_note": (
                "Read-only workspace traversal bounded to declared prefixes. No "
                "successor published yet; additive changes will bump the minor "
                "version and reuse the same identity slug."
            ),
        },
        {
            "package_identity": "aureline:diff-apply-preview@0.1.0",
            "world_slug": "diff-apply-preview",
            "world_semver": "0.1.0",
            "wit_package_ref": "wit/aureline/diff-apply-preview.wit",
            "lifecycle_label": "experimental",
            "registry_status": "active",
            "reader_writer_posture": "read_write",
            "trust_state_gating_posture": "blocked_in_restricted",
            "permission_scope_projection": [
                "filesystem_write",
                "ui_command_contribute",
                "subscription_subscribe",
            ],
            "supported_host_families": [
                "wasm_component_model",
                "wasm_core_module",
                "external_host_process",
                "helper_binary",
                "remote_side_component",
                "compatibility_bridge",
            ],
            "registry_row_ref": registry_ref("aureline:diff-apply-preview@0.1.0"),
            "predecessor_package_ref": None,
            "successor_package_ref": None,
            "publication_state": "published",
            "compatibility_note": (
                "Host-side apply with explicit approval; blocked under restricted "
                "trust. An extension must not rely on the apply path firing. "
                "Narrows to no admission under a restricted trust state."
            ),
        },
        {
            "package_identity": "aureline:terminal-observe@0.1.0",
            "world_slug": "terminal-observe",
            "world_semver": "0.1.0",
            "wit_package_ref": "wit/aureline/terminal-observe.wit",
            "lifecycle_label": "beta",
            "registry_status": "active",
            "reader_writer_posture": "reader_only",
            "trust_state_gating_posture": "admitted_in_restricted",
            "permission_scope_projection": ["subscription_subscribe"],
            "supported_host_families": [
                "wasm_component_model",
                "wasm_core_module",
                "external_host_process",
                "helper_binary",
                "remote_side_component",
                "compatibility_bridge",
            ],
            "registry_row_ref": registry_ref("aureline:terminal-observe@0.1.0"),
            "predecessor_package_ref": None,
            "successor_package_ref": None,
            "publication_state": "published",
            "compatibility_note": (
                "Observe an existing terminal's output / status / exit code; "
                "launching, input injection, and recipe execution are out of scope."
            ),
        },
        {
            "package_identity": "aureline:network-egress@0.1.0",
            "world_slug": "network-egress",
            "world_semver": "0.1.0",
            "wit_package_ref": "wit/aureline/network-egress.wit",
            "lifecycle_label": "experimental",
            "registry_status": "active",
            "reader_writer_posture": "read_write",
            "trust_state_gating_posture": "blocked_in_restricted",
            "permission_scope_projection": ["network_egress"],
            "supported_host_families": [
                "wasm_component_model",
                "wasm_core_module",
                "external_host_process",
                "helper_binary",
                "remote_side_component",
                "compatibility_bridge",
            ],
            "registry_row_ref": registry_ref("aureline:network-egress@0.1.0"),
            "predecessor_package_ref": None,
            "successor_package_ref": None,
            "publication_state": "published",
            "compatibility_note": (
                "Outbound egress bounded to a declared allow-list, re-resolved at "
                "every call; blocked under restricted trust. A host that narrows an "
                "allow-list after admission denies in-flight egress rather than "
                "draining it."
            ),
        },
    ]


# -----------------------------------------------------------------------------
# Capability diffs between published versions.
# -----------------------------------------------------------------------------
def build_capability_diffs() -> list[dict]:
    return [
        {
            "diff_id": "m5_wit_capability_diff:editor-read@0.1.0->0.2.0",
            "world_slug": "editor-read",
            "from_package_ref": "aureline:editor-read@0.1.0",
            "to_package_ref": "aureline:editor-read@0.2.0",
            "from_version": "0.1.0",
            "to_version": "0.2.0",
            "change_class": "additive_minor",
            "compatibility_verdict": "backward_compatible",
            "guest_action_required": "none",
            "added_capabilities": [
                "interface editor-read: func visible-range -> option<visibility-range>",
                "interface editor-read: func word-at(position) -> result<option<string>, read-error>",
                "interface editor-read: record visibility-range",
            ],
            "removed_capabilities": [],
            "changed_capabilities": [],
            "notes": (
                "Additive-minor bump. Every 0.1.0 item is preserved verbatim, so a "
                "0.1.0 guest runs unchanged on a 0.2.0 host. A 0.1.0 host narrows a "
                "0.2.0 guest by withholding the added items, not by denying the world."
            ),
        },
        {
            "diff_id": "m5_wit_capability_diff:editor-read@0.1.0-deprecation",
            "world_slug": "editor-read",
            "from_package_ref": "aureline:editor-read@0.1.0",
            "to_package_ref": "aureline:editor-read@0.2.0",
            "from_version": "0.1.0",
            "to_version": "0.2.0",
            "change_class": "deprecation",
            "compatibility_verdict": "deprecated_superseded",
            "guest_action_required": "upgrade_recommended",
            "added_capabilities": [],
            "removed_capabilities": [],
            "changed_capabilities": [],
            "notes": (
                "Version 0.1.0 is deprecated in favour of 0.2.0. Hosts continue to "
                "admit 0.1.0 guests but emit a deprecation notice carrying the "
                "successor identity and a repair affordance. The world slug stays "
                "active; no removal is implied by the deprecation."
            ),
        },
    ]


# -----------------------------------------------------------------------------
# Host/guest negotiation fixtures.
# -----------------------------------------------------------------------------
def fixture_supported() -> dict:
    return {
        "schema_version": SCHEMA_VERSION,
        "record_kind": "m5_wit_negotiation_fixture",
        "fixture_id": "m5_wit_negotiation:supported",
        "outcome": "supported",
        "title": "Trusted component-model guest, full world set admitted",
        "negotiation_id": "neg-supported-0001",
        "extension_identity_ref": "artifacts/extensions/fixtures#sample.editor-companion",
        "extension_version": "1.4.0",
        "host_contract_family": "wasm_component_model",
        "host_abi_range": "component-model-0.2.x",
        "guest_abi_range": "component-model-0.2.x",
        "world_vocabulary_version": 1,
        "trust_state": "trusted",
        "declared_capability_worlds": [
            "aureline:editor-read@0.2.0",
            "aureline:workspace-read@0.1.0",
            "aureline:terminal-observe@0.1.0",
        ],
        "offered_capability_worlds": [
            "aureline:editor-read@0.2.0",
            "aureline:workspace-read@0.1.0",
            "aureline:terminal-observe@0.1.0",
        ],
        "negotiated_capability_worlds": [
            "aureline:editor-read@0.2.0",
            "aureline:workspace-read@0.1.0",
            "aureline:terminal-observe@0.1.0",
        ],
        "narrowing_reasons": [],
        "unsupported_world_decisions": [],
        "deprecated_world_notices": [],
        "fail_closed": False,
        "guest_authority_widened": False,
        "expected_audit_events": [
            "host_negotiation_opened",
            "host_negotiation_completed",
        ],
        "narrative": (
            "A trusted component-model extension declares three read-only worlds. "
            "Host and guest ABI ranges and world-vocabulary versions agree, so the "
            "host offers and admits the full declared set. Nothing is narrowed and "
            "no world is widened beyond what the manifest declared."
        ),
    }


def fixture_downgraded() -> dict:
    return {
        "schema_version": SCHEMA_VERSION,
        "record_kind": "m5_wit_negotiation_fixture",
        "fixture_id": "m5_wit_negotiation:downgraded",
        "outcome": "downgraded",
        "title": "Restricted trust narrows the apply and egress worlds",
        "negotiation_id": "neg-downgraded-0001",
        "extension_identity_ref": "artifacts/extensions/fixtures#sample.refactor-helper",
        "extension_version": "2.0.1",
        "host_contract_family": "wasm_component_model",
        "host_abi_range": "component-model-0.2.x",
        "guest_abi_range": "component-model-0.2.x",
        "world_vocabulary_version": 1,
        "trust_state": "restricted",
        "declared_capability_worlds": [
            "aureline:editor-read@0.2.0",
            "aureline:diff-apply-preview@0.1.0",
            "aureline:network-egress@0.1.0",
        ],
        "offered_capability_worlds": [
            "aureline:editor-read@0.2.0",
            "aureline:diff-apply-preview@0.1.0",
            "aureline:network-egress@0.1.0",
        ],
        "negotiated_capability_worlds": [
            "aureline:editor-read@0.2.0",
        ],
        "narrowing_reasons": [
            {
                "world": "aureline:diff-apply-preview@0.1.0",
                "reason": "workspace_trust_restricted",
                "repair_affordance_label": "grant workspace trust to enable diff apply preview",
            },
            {
                "world": "aureline:network-egress@0.1.0",
                "reason": "workspace_trust_restricted",
                "repair_affordance_label": "grant workspace trust to enable network egress",
            },
        ],
        "unsupported_world_decisions": [],
        "deprecated_world_notices": [],
        "fail_closed": True,
        "guest_authority_widened": False,
        "expected_audit_events": [
            "host_negotiation_opened",
            "host_negotiation_worlds_narrowed",
            "host_negotiation_world_denied",
            "host_negotiation_trust_state_denied",
            "host_negotiation_completed",
        ],
        "narrative": (
            "Under a restricted trust state the two blocked-in-restricted worlds "
            "(diff-apply-preview and network-egress) are narrowed out with the "
            "workspace_trust_restricted reason and a repair affordance each. The "
            "read-only editor world survives. Authority is never widened to "
            "compensate for the narrowed worlds; the negotiation fails closed."
        ),
    }


def fixture_deprecated() -> dict:
    return {
        "schema_version": SCHEMA_VERSION,
        "record_kind": "m5_wit_negotiation_fixture",
        "fixture_id": "m5_wit_negotiation:deprecated",
        "outcome": "deprecated",
        "title": "Deprecated editor-read@0.1.0 admitted with successor notice",
        "negotiation_id": "neg-deprecated-0001",
        "extension_identity_ref": "artifacts/extensions/fixtures#sample.legacy-reader",
        "extension_version": "0.9.3",
        "host_contract_family": "wasm_component_model",
        "host_abi_range": "component-model-0.2.x",
        "guest_abi_range": "component-model-0.2.x",
        "world_vocabulary_version": 1,
        "trust_state": "trusted",
        "declared_capability_worlds": [
            "aureline:editor-read@0.1.0",
            "aureline:workspace-read@0.1.0",
        ],
        "offered_capability_worlds": [
            "aureline:editor-read@0.1.0",
            "aureline:workspace-read@0.1.0",
        ],
        "negotiated_capability_worlds": [
            "aureline:editor-read@0.1.0",
            "aureline:workspace-read@0.1.0",
        ],
        "narrowing_reasons": [],
        "unsupported_world_decisions": [],
        "deprecated_world_notices": [
            {
                "world": "aureline:editor-read@0.1.0",
                "successor_world_ref": "aureline:editor-read@0.2.0",
                "repair_affordance_label": "upgrade extension to declare aureline:editor-read@0.2.0",
            }
        ],
        "fail_closed": False,
        "guest_authority_widened": False,
        "expected_audit_events": [
            "host_negotiation_opened",
            "host_negotiation_completed",
        ],
        "narrative": (
            "The guest still declares the deprecated 0.1.0 editor-read version. The "
            "host admits it — deprecation is not removal — but emits an explicit "
            "deprecated-world notice carrying the 0.2.0 successor identity and a "
            "repair affordance, so install review and SDK docs render a specific "
            "upgrade path rather than a silent pass-through."
        ),
    }


def fixture_unsupported_skew() -> dict:
    return {
        "schema_version": SCHEMA_VERSION,
        "record_kind": "m5_wit_negotiation_fixture",
        "fixture_id": "m5_wit_negotiation:unsupported_skew",
        "outcome": "unsupported_skew",
        "title": "Guest ABI ahead of host; skewed world denied fail-closed",
        "negotiation_id": "neg-unsupported-skew-0001",
        "extension_identity_ref": "artifacts/extensions/fixtures#sample.early-adopter",
        "extension_version": "3.0.0",
        "host_contract_family": "wasm_component_model",
        "host_abi_range": "component-model-0.2.x",
        "guest_abi_range": "component-model-0.3.x",
        "world_vocabulary_version": 1,
        "trust_state": "trusted",
        "declared_capability_worlds": [
            "aureline:editor-read@0.2.0",
            "aureline:workspace-read@0.1.0",
        ],
        "offered_capability_worlds": [
            "aureline:editor-read@0.2.0",
            "aureline:workspace-read@0.1.0",
        ],
        "negotiated_capability_worlds": [
            "aureline:workspace-read@0.1.0",
        ],
        "narrowing_reasons": [],
        "unsupported_world_decisions": [
            {
                "declared_world_ref": "aureline:editor-read@0.2.0",
                "unsupported_reason": "guest_abi_range_mismatch",
                "successor_world_ref": None,
                "repair_affordance_label": "rebuild extension against component-model 0.2.x or wait for host 0.3.x support",
            }
        ],
        "deprecated_world_notices": [],
        "fail_closed": True,
        "guest_authority_widened": False,
        "expected_audit_events": [
            "host_negotiation_opened",
            "host_negotiation_abi_mismatch",
            "host_negotiation_world_denied",
            "host_negotiation_completed",
        ],
        "narrative": (
            "The guest declares a component-model 0.3.x ABI the host (0.2.x) does "
            "not implement. The skewed editor-read@0.2.0 world is denied with a "
            "typed guest_abi_range_mismatch decision and a repair affordance; it "
            "never enters the negotiated set. The unaffected workspace-read world "
            "is admitted. The host fails closed: it denies rather than widening or "
            "silently dropping the unsupported world."
        ),
    }


def build_fixtures() -> list[dict]:
    return [
        fixture_supported(),
        fixture_downgraded(),
        fixture_deprecated(),
        fixture_unsupported_skew(),
    ]


# -----------------------------------------------------------------------------
# Derivation helpers shared with the validator (so the two cannot drift).
# -----------------------------------------------------------------------------
def derive_fail_closed(fixture: dict) -> bool:
    """A negotiation fails closed when at least one declared world was narrowed
    or denied (rather than widened or silently dropped)."""
    return bool(fixture["narrowing_reasons"]) or bool(
        fixture["unsupported_world_decisions"]
    )


def fixture_issues(fixture: dict) -> list[str]:
    """Return the list of semantic-invariant violations for one fixture.

    Mirrors the typed Rust ``NegotiationFixture::issues``. An empty list means
    the fixture is conforming.
    """
    issues: list[str] = []
    declared = set(fixture["declared_capability_worlds"])
    offered = set(fixture["offered_capability_worlds"])
    negotiated = set(fixture["negotiated_capability_worlds"])

    if not offered <= declared:
        issues.append("offered_not_subset_of_declared")
    if not negotiated <= offered:
        issues.append("negotiated_not_subset_of_offered")

    # No widening: a negotiated world must have been declared.
    if not negotiated <= declared:
        issues.append("negotiated_widens_beyond_declared")
    if fixture["guest_authority_widened"]:
        issues.append("guest_authority_widened")

    # Every declared-but-not-negotiated world must carry exactly one disposition
    # (a narrowing reason, an unsupported decision, or a deprecated notice that
    # still admits it). No silent drop.
    narrowed = {entry["world"] for entry in fixture["narrowing_reasons"]}
    unsupported = {
        entry["declared_world_ref"]
        for entry in fixture["unsupported_world_decisions"]
    }
    for world in declared - negotiated:
        if world not in narrowed and world not in unsupported:
            issues.append(f"silent_drop:{world}")

    # Narrowing reasons and unsupported decisions only name declared worlds.
    for world in narrowed:
        if world not in declared:
            issues.append(f"narrowing_reason_undeclared:{world}")
        if world in negotiated:
            issues.append(f"narrowed_world_still_negotiated:{world}")
    for world in unsupported:
        if world not in declared:
            issues.append(f"unsupported_decision_undeclared:{world}")
        if world in negotiated:
            issues.append(f"unsupported_world_still_negotiated:{world}")

    # Vocabulary checks.
    for entry in fixture["narrowing_reasons"]:
        if entry["reason"] not in NARROWING_REASONS:
            issues.append(f"unknown_narrowing_reason:{entry['reason']}")
        if not entry.get("repair_affordance_label"):
            issues.append(f"narrowing_reason_missing_repair:{entry['world']}")
    for entry in fixture["unsupported_world_decisions"]:
        if entry["unsupported_reason"] not in UNSUPPORTED_WORLD_REASONS:
            issues.append(f"unknown_unsupported_reason:{entry['unsupported_reason']}")
        if not entry.get("repair_affordance_label"):
            issues.append(
                f"unsupported_decision_missing_repair:{entry['declared_world_ref']}"
            )

    # Deprecated notices must carry a successor and a repair affordance, and the
    # world they name must still be admitted (deprecation is not removal).
    for entry in fixture["deprecated_world_notices"]:
        if not entry.get("successor_world_ref"):
            issues.append(f"deprecated_notice_missing_successor:{entry['world']}")
        if not entry.get("repair_affordance_label"):
            issues.append(f"deprecated_notice_missing_repair:{entry['world']}")
        if entry["world"] not in negotiated:
            issues.append(f"deprecated_world_not_admitted:{entry['world']}")

    # Recorded fail_closed must match the derived value.
    if fixture["fail_closed"] != derive_fail_closed(fixture):
        issues.append("fail_closed_mismatch")

    # Outcome-specific shape.
    outcome = fixture["outcome"]
    if outcome == "supported":
        if negotiated != declared:
            issues.append("supported_did_not_admit_all")
    elif outcome == "downgraded":
        if not fixture["narrowing_reasons"]:
            issues.append("downgraded_without_narrowing")
        if negotiated == declared:
            issues.append("downgraded_admitted_all")
    elif outcome == "deprecated":
        if not fixture["deprecated_world_notices"]:
            issues.append("deprecated_without_notice")
    elif outcome == "unsupported_skew":
        if not fixture["unsupported_world_decisions"]:
            issues.append("unsupported_skew_without_decision")
        skew_reasons = {
            "host_abi_range_mismatch",
            "guest_abi_range_mismatch",
            "world_vocabulary_version_unknown",
        }
        if not any(
            entry["unsupported_reason"] in skew_reasons
            for entry in fixture["unsupported_world_decisions"]
        ):
            issues.append("unsupported_skew_without_skew_reason")

    return issues


def diff_issues(diff: dict) -> list[str]:
    """Return the semantic-invariant violations for one capability diff."""
    issues: list[str] = []
    change_class = diff["change_class"]
    if change_class == "additive_minor":
        if diff["removed_capabilities"] or diff["changed_capabilities"]:
            issues.append("additive_minor_removed_or_changed")
        if not diff["added_capabilities"]:
            issues.append("additive_minor_without_additions")
        if diff["compatibility_verdict"] != "backward_compatible":
            issues.append("additive_minor_not_backward_compatible")
        if diff["guest_action_required"] != "none":
            issues.append("additive_minor_requires_guest_action")
    elif change_class == "deprecation":
        if diff["compatibility_verdict"] != "deprecated_superseded":
            issues.append("deprecation_wrong_verdict")
        if not diff["to_package_ref"]:
            issues.append("deprecation_without_successor")
    elif change_class in ("breaking_major", "retirement"):
        if diff["compatibility_verdict"] != "breaking":
            issues.append("breaking_wrong_verdict")
        if diff["guest_action_required"] != "upgrade_required":
            issues.append("breaking_without_required_upgrade")
    return issues


def compute_summary(packet: dict) -> dict:
    fixtures = packet["negotiation_fixtures"]
    diffs = packet["capability_diffs"]
    return {
        "package_count": len(packet["packages"]),
        "published_package_count": sum(
            1 for p in packet["packages"] if p["publication_state"] == "published"
        ),
        "deprecated_package_count": sum(
            1 for p in packet["packages"] if p["lifecycle_label"] == "deprecated"
        ),
        "negotiation_fixture_count": len(fixtures),
        "outcomes_covered": sorted({f["outcome"] for f in fixtures}),
        "fail_closed_fixture_count": sum(1 for f in fixtures if f["fail_closed"]),
        "capability_diff_count": len(diffs),
        "all_fixtures_conform": all(not fixture_issues(f) for f in fixtures),
        "all_diffs_conform": all(not diff_issues(d) for d in diffs),
    }


# -----------------------------------------------------------------------------
# Packet assembly.
# -----------------------------------------------------------------------------
def build_packet() -> dict:
    packet = {
        "schema_version": SCHEMA_VERSION,
        "record_kind": RECORD_KIND,
        "packet_id": PACKET_ID,
        "status": "published",
        "as_of": AS_OF,
        "family_id": CONTRACT_MATRIX_ROW,
        "overview_page": OVERVIEW_PAGE,
        "evidence_page": EVIDENCE_PAGE,
        "schema_ref": SCHEMA_REF,
        "contract_matrix_ref": CONTRACT_MATRIX_REF,
        "contract_matrix_row": CONTRACT_MATRIX_ROW,
        "capability_world_registry_ref": CAPABILITY_WORLD_REGISTRY_REF,
        "negotiation_schema_ref": NEGOTIATION_SCHEMA_REF,
        "adr_ref": ADR_REF,
        "root_package_ref": ROOT_PACKAGE_REF,
        "wit_index_ref": WIT_INDEX_REF,
        "capability_diff_report_ref": "artifacts/contracts/m5-wit-capability-diff.md",
        "lifecycle_labels": LIFECYCLE_LABELS,
        "registry_statuses": REGISTRY_STATUSES,
        "reader_writer_postures": READER_WRITER_POSTURES,
        "trust_state_postures": TRUST_STATE_POSTURES,
        "publication_states": PUBLICATION_STATES,
        "negotiation_outcomes": NEGOTIATION_OUTCOMES,
        "change_classes": CHANGE_CLASSES,
        "compatibility_verdicts": COMPATIBILITY_VERDICTS,
        "guest_actions": GUEST_ACTIONS,
        "narrowing_reasons": NARROWING_REASONS,
        "unsupported_world_reasons": UNSUPPORTED_WORLD_REASONS,
        "consuming_surfaces": CONSUMING_SURFACES,
        "packages": build_packages(),
        "capability_diffs": build_capability_diffs(),
        "negotiation_fixtures": build_fixtures(),
    }
    packet["summary"] = compute_summary(packet)
    return packet


# -----------------------------------------------------------------------------
# Capability-diff Markdown projection.
# -----------------------------------------------------------------------------
def build_diff_markdown(packet: dict) -> str:
    lines: list[str] = []
    lines.append("# M5 extension-host WIT capability diff report")
    lines.append("")
    lines.append(
        "Generated from "
        "[`artifacts/contracts/m5-wit-contract-publication.json`]"
        "(m5-wit-contract-publication.json) by "
        "`tools/regenerate_m5_wit_contract_publication.py`. Do not hand-edit; "
        "edit the regenerator and re-run it."
    )
    lines.append("")
    lines.append(
        "This report lets extension authors, reviewers, and release managers see "
        "what changed between published versions of each capability-world WIT "
        "package without reverse-engineering host code. Every row is cross-checked "
        "against the typed host behaviour in "
        "`crates/aureline-extensions/src/"
        + NAME
        + "/`."
    )
    lines.append("")

    lines.append("## Published versions")
    lines.append("")
    lines.append("| Package | Version | Lifecycle | Posture | Successor |")
    lines.append("| --- | --- | --- | --- | --- |")
    for pkg in packet["packages"]:
        successor = pkg["successor_package_ref"] or "—"
        lines.append(
            f"| `{pkg['world_slug']}` | `{pkg['world_semver']}` | "
            f"{pkg['lifecycle_label']} | {pkg['reader_writer_posture']} | "
            f"`{successor}` |"
        )
    lines.append("")

    lines.append("## Capability diffs")
    lines.append("")
    for diff in packet["capability_diffs"]:
        lines.append(
            f"### `{diff['world_slug']}` {diff['from_version']} → {diff['to_version']}"
            f" ({diff['change_class']})"
        )
        lines.append("")
        lines.append(f"- **Compatibility verdict:** {diff['compatibility_verdict']}")
        lines.append(f"- **Guest action required:** {diff['guest_action_required']}")
        if diff["added_capabilities"]:
            lines.append("- **Added:**")
            for item in diff["added_capabilities"]:
                lines.append(f"  - `{item}`")
        if diff["removed_capabilities"]:
            lines.append("- **Removed:**")
            for item in diff["removed_capabilities"]:
                lines.append(f"  - `{item}`")
        if diff["changed_capabilities"]:
            lines.append("- **Changed:**")
            for item in diff["changed_capabilities"]:
                lines.append(f"  - `{item}`")
        lines.append("")
        lines.append(diff["notes"])
        lines.append("")

    lines.append("## Negotiation outcomes proven by fixtures")
    lines.append("")
    lines.append("| Outcome | Declared | Negotiated | Fails closed | Fixture |")
    lines.append("| --- | --- | --- | --- | --- |")
    for fixture in packet["negotiation_fixtures"]:
        ref = f"fixtures/contracts/m5-wit-negotiation/{fixture['outcome']}.json"
        lines.append(
            f"| {fixture['outcome']} | "
            f"{len(fixture['declared_capability_worlds'])} | "
            f"{len(fixture['negotiated_capability_worlds'])} | "
            f"{'yes' if fixture['fail_closed'] else 'no'} | `{ref}` |"
        )
    lines.append("")
    return "\n".join(lines) + "\n"


# -----------------------------------------------------------------------------
# Writers.
# -----------------------------------------------------------------------------
def write_json(path: Path, data: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")


def write_text(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


def write_fixtures(packet: dict) -> None:
    FIXTURES_DIR.mkdir(parents=True, exist_ok=True)
    cases = {
        "schema_version": SCHEMA_VERSION,
        "record_kind": "m5_wit_negotiation_fixture_index",
        "index_id": "m5_wit_negotiation_fixtures:v1",
        "as_of": AS_OF,
        "packet_ref": "artifacts/contracts/m5-wit-contract-publication.json",
        "cases": [],
    }
    for fixture in packet["negotiation_fixtures"]:
        filename = f"{fixture['outcome']}.json"
        write_json(FIXTURES_DIR / filename, fixture)
        cases["cases"].append(
            {
                "file": filename,
                "fixture_id": fixture["fixture_id"],
                "outcome": fixture["outcome"],
                "fail_closed": fixture["fail_closed"],
                "expect_conforming": True,
            }
        )
    write_json(FIXTURES_DIR / "cases.json", cases)


def build_capture(packet: dict) -> dict:
    return {
        "schema_version": SCHEMA_VERSION,
        "record_kind": "m5_wit_contract_publication_validation_capture",
        "name": NAME,
        "as_of": AS_OF,
        "packet_ref": "artifacts/contracts/m5-wit-contract-publication.json",
        "summary": packet["summary"],
        "fixtures": [
            {
                "outcome": f["outcome"],
                "fixture_id": f["fixture_id"],
                "conforming": not fixture_issues(f),
                "fail_closed": f["fail_closed"],
            }
            for f in packet["negotiation_fixtures"]
        ],
        "capability_diffs": [
            {
                "diff_id": d["diff_id"],
                "change_class": d["change_class"],
                "conforming": not diff_issues(d),
            }
            for d in packet["capability_diffs"]
        ],
    }


def main() -> None:
    packet = build_packet()
    write_json(PACKET_PATH, packet)
    write_text(DIFF_MD_PATH, build_diff_markdown(packet))
    write_fixtures(packet)
    write_json(CAPTURE_PATH, build_capture(packet))
    print(f"wrote {PACKET_PATH.relative_to(REPO_ROOT)}")
    print(f"wrote {DIFF_MD_PATH.relative_to(REPO_ROOT)}")
    print(f"wrote {FIXTURES_DIR.relative_to(REPO_ROOT)}/ (4 fixtures + cases.json)")
    print(f"wrote {CAPTURE_PATH.relative_to(REPO_ROOT)}")


if __name__ == "__main__":
    main()
