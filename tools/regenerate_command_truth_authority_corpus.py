#!/usr/bin/env python3
"""Regenerate (or verify) the command-truth and palette-authority corpus.

This generator is the single source for the command-truth and palette-authority
conformance corpus: the positive/negative command-authority scenario fixtures,
the corpus manifest, the parity report, and the release evidence packet all come
from here so the harness expectations and the published evidence cannot drift
apart.

Usage:
    python3 tools/regenerate_command_truth_authority_corpus.py [--write | --check]

    --write   (default) write the generated files to the repository.
    --check   regenerate in memory and fail if any committed file differs.

The conformance test `cargo test -p aureline-qe --test
command_truth_authority_conformance` replays the committed corpus against the
real `aureline_commands::CommandAuthorityScenarioRecord` validator.
"""
import argparse
import json
import os
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))

CORPUS_REL = "fixtures/commands/m3/command_truth_and_authority"
PARITY_REPORT_REL = "artifacts/ux/m3/command_truth_and_authority_parity_report.md"
EVIDENCE_PACKET_REL = "artifacts/release/m3/command_invocation_evidence_packet.json"

CORPUS_ID = "commands.command_truth_and_authority.beta"
SCENARIO_SCHEMA_REL = "../../../../../schemas/commands/command_conformance_result.schema.json"
MANIFEST_SCHEMA_REL = "../../../../schemas/commands/command_conformance_result.schema.json"
PACKET_SCHEMA_REL = "../../../schemas/commands/command_conformance_result.schema.json"

INVOCATION_SCHEMA_REF = "schemas/commands/command_invocation_session.schema.json"
RESULT_SCHEMA_REF = "schemas/commands/command_result_packet.schema.json"

ALL_SURFACES = [
    "menu_or_button",
    "keybinding",
    "command_palette",
    "cli_headless",
    "ai_tool",
    "recipe",
    "voice",
    "browser_companion",
]


# --------------------------------------------------------------------------- #
# Record builders. These mirror the aureline_commands boundary structs exactly.
# --------------------------------------------------------------------------- #
def descriptor(
    command_id,
    verb,
    *,
    lifecycle="stable",
    preview="no_preview_required",
    approval="no_approval_required",
    capability,
    ai_class,
    labels,
    scopes,
    support_class="standard_support",
    release_channel="stable_channel",
    origin_class="core",
    alias_canonical=None,
):
    alias_canonical = alias_canonical or command_id
    return {
        "record_kind": "command_descriptor_record",
        "command_descriptor_schema_version": 1,
        "command_id": command_id,
        "command_revision_ref": f"cmd-rev:{verb}:2026.05.20-01",
        "canonical_verb": verb,
        "primary_label_ref": f"label:{verb}:primary",
        "accessibility_label_path": {
            "primary_label_ref": f"label:{verb}:a11y_primary",
            "short_label_ref": f"label:{verb}:a11y_short",
            "long_description_ref": f"label:{verb}:a11y_long",
            "role_class": "command",
            "keyboard_shortcut_narration_ref": f"label:{verb}:shortcut_narration",
        },
        "docs_help_anchor_ref": {
            "pack_id": "pack:project:aureline:01",
            "anchor_id": f"docs:anchor:{verb}",
            "anchor_kind": "docs_page_anchor",
        },
        "shortcut_narration_hint": {
            "when_bound_narration_ref": f"label:{verb}:shortcut_bound",
            "when_unbound_narration_ref": f"label:{verb}:shortcut_unbound",
            "chord_class_hint": "modifier_plus_key",
        },
        "aliases": [
            {
                "alias_id": f"alias:{verb}:legacy_command_id",
                "alias_kind": "legacy_command_id",
                "canonical_command_id": alias_canonical,
                "replacement_note_ref": f"migration-note:{verb}:legacy",
                "introduced_version": "release:2026.04",
                "deprecation_state": "deprecated",
                "retirement_version": "release:2027.04",
            },
            {
                "alias_id": f"alias:{verb}:cli_verb",
                "alias_kind": "alternate_cli_verb",
                "canonical_command_id": alias_canonical,
                "replacement_note_ref": f"migration-note:{verb}:cli_verb",
                "introduced_version": "release:2026.04",
                "deprecation_state": "active",
                "retirement_version": None,
            },
        ],
        "category_refs": [f"category:{verb.split('.')[0]}", "category:command"],
        "origin": {
            "origin_class": origin_class,
            "source_ref": f"registry-entry:{verb}",
            "publisher_ref": None,
        },
        "invocation_schema_ref": INVOCATION_SCHEMA_REF,
        "result_schema_ref": RESULT_SCHEMA_REF,
        "enablement_rule_refs": [
            f"reason:{verb}:execution_context_unavailable",
            f"reason:{verb}:workspace_trust_restricted",
        ],
        "discoverability_record_refs": [
            f"discover:{verb}:01",
            f"projection:{verb}:palette_row",
            f"projection:{verb}:menu_item",
            f"projection:{verb}:keybinding",
            f"projection:{verb}:cli_help",
            f"projection:{verb}:ai_tool",
        ],
        "automation_labels": list(labels),
        "typed_arguments": [],
        "capability_scope_class": capability,
        "preview_class": preview,
        "approval_posture_class": approval,
        "ai_tool_surfacing_class": ai_class,
        "palette_visibility": "always_visible",
        "ui_slot_hints": [],
        "lifecycle_state": lifecycle,
        "support_class": support_class,
        "release_channel": release_channel,
        "declared_freshness_class": "authoritative_live",
        "client_scopes": list(scopes),
        "result_contract": {
            "result_contract_class": "journal_entry_appended_ref",
            "artifact_kind_ref": f"artifact-kind:{verb}:journal_entry",
            "typed_value_shape_ref": None,
            "evidence_ref_class_required": ["mutation_journal_entry_ref"],
        },
        "default_enablement_repair_hook_ref": None,
        "policy_context": {
            "policy_epoch": "pe:2026-05-20:01",
            "trust_state": "trusted",
            "execution_context_id": "exec:command-registry:01",
        },
        "redaction_class": "metadata_safe_default",
        "minted_at": "2026-05-20T12:00:00Z",
    }


def surface(
    surface_class,
    session,
    *,
    decision="enabled",
    reason=None,
    preview="no_preview_required",
    approval="no_approval_required",
    authority="user_initiated_local",
    dispatched=False,
    outcome=None,
    resolves=None,
):
    return {
        "surface_class": surface_class,
        "enablement_decision_class": decision,
        "disabled_reason_code": reason,
        "preview_class_declared": preview,
        "approval_posture_class_declared": approval,
        "authority_class": authority,
        "dispatched": dispatched,
        "outcome_code": outcome,
        "invocation_session_id": session,
        "resolves_to_canonical_command_id": resolves,
    }


def lineage(
    command_id,
    session,
    result_id,
    outcome,
    *,
    evidence,
    notifications=None,
    activities=None,
    rollback_posture="not_reversible_by_contract",
    rollback_id=None,
    support_bundle=None,
):
    return {
        "command_id": command_id,
        "invocation_session_id": session,
        "result_packet_id": result_id,
        "result_outcome_code": outcome,
        "evidence_refs": list(evidence),
        "notification_refs": list(notifications or []),
        "activity_refs": list(activities or []),
        "rollback_handle_posture": rollback_posture,
        "rollback_handle_id": rollback_id,
        "support_bundle_ref": support_bundle,
    }


def scenario(scenario_id, desc, surfaces, lin):
    return {
        "record_kind": "command_authority_scenario_record",
        "schema_version": 1,
        "scenario_id": scenario_id,
        "canonical_descriptor": desc,
        "surfaces": surfaces,
        "lineage": lin,
    }


def lineage_chain(lin):
    chain = [lin["command_id"], lin["invocation_session_id"], lin["result_packet_id"]]
    chain.extend(lin["evidence_refs"])
    chain.extend(lin["notification_refs"])
    chain.extend(lin["activity_refs"])
    if lin["rollback_handle_id"] is not None:
        chain.append(lin["rollback_handle_id"])
    if lin["support_bundle_ref"] is not None:
        chain.append(lin["support_bundle_ref"])
    return chain


# --------------------------------------------------------------------------- #
# Scenario specifications.
# --------------------------------------------------------------------------- #
OPEN_FOLDER_LABELS = ["macro_safe", "recipe_safe", "headless_safe"]
OPEN_FOLDER_SCOPES = [
    "desktop_product",
    "cli",
    "companion_surface",
    "remote_agent",
    "sdk_or_api",
]
HIGH_RISK_LABELS = ["recipe_safe", "headless_safe", "approval_required"]
HIGH_RISK_SCOPES = ["desktop_product", "cli", "remote_agent", "sdk_or_api"]


def open_folder_descriptor(**overrides):
    base = dict(
        capability="reversible_local_mutation",
        ai_class="ai_callable_reversible_mutation",
        labels=OPEN_FOLDER_LABELS,
        scopes=OPEN_FOLDER_SCOPES,
    )
    base.update(overrides)
    return descriptor("cmd:workspace.open_folder", "workspace.open_folder", **base)


def import_profile_descriptor(**overrides):
    base = dict(
        preview="structured_diff_preview",
        approval="explicit_confirmation_required",
        capability="recoverable_durable_mutation",
        ai_class="not_ai_callable",
        labels=HIGH_RISK_LABELS,
        scopes=HIGH_RISK_SCOPES,
    )
    base.update(overrides)
    return descriptor("cmd:workspace.import_profile", "workspace.import_profile", **base)


def build_positive():
    drills = []

    # P1 — full cross-surface parity for a reversible command on every surface.
    p1_session = "inv:workspace.open_folder:2026-05-20T12-10-00Z:01"
    p1_surfaces = []
    for sc in ALL_SURFACES:
        p1_surfaces.append(
            surface(
                sc,
                p1_session if sc == "command_palette"
                else f"inv:workspace.open_folder:{sc}:01",
                dispatched=(sc == "command_palette"),
                outcome="succeeded" if sc == "command_palette" else None,
                authority="user_initiated_local"
                if sc not in ("ai_tool", "recipe", "cli_headless")
                else "delegated_automation",
            )
        )
    p1_lineage = lineage(
        "cmd:workspace.open_folder",
        p1_session,
        "result:workspace.open_folder:2026-05-20T12-10-00Z:01",
        "succeeded",
        evidence=["journal-entry:workspace.open_folder:01"],
        activities=["activity:workspace.open_folder:01"],
        support_bundle="support-bundle:workspace.open_folder:01",
    )
    drills.append((
        "positive.full_cross_surface_parity",
        "positive/full_cross_surface_parity.json",
        scenario(
            "scenario:command_authority:full_cross_surface_parity",
            open_folder_descriptor(),
            p1_surfaces,
            p1_lineage,
        ),
        "Reversible command keeps one enablement decision, one preview/approval "
        "posture, and one result contract across menu, keybinding, palette, "
        "CLI/headless, AI, recipe, voice, and browser-companion surfaces.",
        dict(
            command_id="cmd:workspace.open_folder",
            lifecycle="stable",
            preview="no_preview_required",
            approval="no_approval_required",
            enablement="enabled",
            surfaces=list(ALL_SURFACES),
            labels=OPEN_FOLDER_LABELS,
            lineage_complete=True,
            rollback_required=False,
        ),
    ))

    # P2 — high-risk command keeps a structured-diff preview + explicit approval
    # on every surface and joins a reversible rollback handle into its lineage.
    p2_session = "inv:workspace.import_profile:2026-05-20T12-20-00Z:01"
    p2_surface_classes = ["menu_or_button", "keybinding", "command_palette", "cli_headless", "recipe"]
    p2_surfaces = [
        surface(
            sc,
            p2_session if sc == "command_palette"
            else f"inv:workspace.import_profile:{sc}:01",
            preview="structured_diff_preview",
            approval="explicit_confirmation_required",
            dispatched=(sc == "command_palette"),
            outcome="succeeded" if sc == "command_palette" else None,
            authority="delegated_automation" if sc in ("cli_headless", "recipe") else "user_initiated_local",
        )
        for sc in p2_surface_classes
    ]
    p2_lineage = lineage(
        "cmd:workspace.import_profile",
        p2_session,
        "result:workspace.import_profile:2026-05-20T12-20-00Z:01",
        "succeeded",
        evidence=[
            "preview-record:workspace.import_profile:01",
            "journal-entry:workspace.import_profile:01",
        ],
        notifications=["notification:workspace.import_profile:approval_granted:01"],
        activities=["activity:workspace.import_profile:01"],
        rollback_posture="reversible_handle",
        rollback_id="rollback-handle:workspace.import_profile:01",
        support_bundle="support-bundle:workspace.import_profile:01",
    )
    drills.append((
        "positive.high_risk_preview_approval_parity",
        "positive/high_risk_preview_approval_parity.json",
        scenario(
            "scenario:command_authority:high_risk_preview_approval_parity",
            import_profile_descriptor(),
            p2_surfaces,
            p2_lineage,
        ),
        "High-risk durable command preserves its structured-diff preview and "
        "explicit-approval requirement on every surface, stays off the AI tool "
        "surface, and joins a reversible rollback handle into its lineage.",
        dict(
            command_id="cmd:workspace.import_profile",
            lifecycle="stable",
            preview="structured_diff_preview",
            approval="explicit_confirmation_required",
            enablement="enabled",
            surfaces=p2_surface_classes,
            labels=HIGH_RISK_LABELS,
            lineage_complete=True,
            rollback_required=True,
        ),
    ))

    # P3 — disabled-with-reason parity: every surface agrees on the same denial.
    p3_session = "inv:workspace.restore_from_checkpoint:2026-05-20T12-30-00Z:01"
    p3_surface_classes = ["menu_or_button", "keybinding", "command_palette", "cli_headless", "recipe"]
    p3_surfaces = [
        surface(
            sc,
            p3_session if sc == "command_palette"
            else f"inv:workspace.restore_from_checkpoint:{sc}:01",
            decision="disabled_with_reason",
            reason="workspace_trust_restricted",
            preview="structured_diff_preview",
            approval="explicit_confirmation_required",
            dispatched=(sc == "command_palette"),
            outcome="denied_disabled" if sc == "command_palette" else None,
            authority="delegated_automation" if sc in ("cli_headless", "recipe") else "user_initiated_local",
        )
        for sc in p3_surface_classes
    ]
    p3_lineage = lineage(
        "cmd:workspace.restore_from_checkpoint",
        p3_session,
        "result:workspace.restore_from_checkpoint:2026-05-20T12-30-00Z:01",
        "denied",
        evidence=["denial-evidence:workspace.restore_from_checkpoint:trust:01"],
        activities=["activity:workspace.restore_from_checkpoint:denied:01"],
        support_bundle="support-bundle:workspace.restore_from_checkpoint:01",
    )
    drills.append((
        "positive.disabled_with_reason_parity",
        "positive/disabled_with_reason_parity.json",
        scenario(
            "scenario:command_authority:disabled_with_reason_parity",
            descriptor(
                "cmd:workspace.restore_from_checkpoint",
                "workspace.restore_from_checkpoint",
                preview="structured_diff_preview",
                approval="explicit_confirmation_required",
                capability="recoverable_durable_mutation",
                ai_class="not_ai_callable",
                labels=HIGH_RISK_LABELS,
                scopes=HIGH_RISK_SCOPES,
            ),
            p3_surfaces,
            p3_lineage,
        ),
        "When the command is unavailable, every surface reports the same "
        "disabled-with-reason decision and disabled-reason code, and the denied "
        "attempt still joins a support-reconstructable lineage that needs no "
        "rollback handle because nothing was applied.",
        dict(
            command_id="cmd:workspace.restore_from_checkpoint",
            lifecycle="stable",
            preview="structured_diff_preview",
            approval="explicit_confirmation_required",
            enablement="disabled_with_reason",
            surfaces=p3_surface_classes,
            labels=HIGH_RISK_LABELS,
            lineage_complete=True,
            rollback_required=False,
        ),
    ))

    # P4 — UI-only command stays explicitly narrowed off automation surfaces.
    p4_session = "inv:command_palette.open:2026-05-20T12-40-00Z:01"
    p4_surface_classes = ["menu_or_button", "keybinding", "command_palette"]
    p4_surfaces = [
        surface(
            sc,
            p4_session if sc == "command_palette"
            else f"inv:command_palette.open:{sc}:01",
            dispatched=(sc == "command_palette"),
            outcome="succeeded" if sc == "command_palette" else None,
        )
        for sc in p4_surface_classes
    ]
    p4_lineage = lineage(
        "cmd:command_palette.open",
        p4_session,
        "result:command_palette.open:2026-05-20T12-40-00Z:01",
        "succeeded",
        evidence=["ui-event-evidence:command_palette.open:01"],
        activities=["activity:command_palette.open:01"],
        support_bundle="support-bundle:command_palette.open:01",
    )
    drills.append((
        "positive.ui_only_narrowed",
        "positive/ui_only_narrowed.json",
        scenario(
            "scenario:command_authority:ui_only_narrowed",
            descriptor(
                "cmd:command_palette.open",
                "command_palette.open",
                capability="inert_metadata_only",
                ai_class="not_ai_callable",
                labels=["ui_only"],
                scopes=["desktop_product"],
            ),
            p4_surfaces,
            p4_lineage,
        ),
        "A UI-only command advertises the ui_only label, stays off every "
        "automation surface (the absence is an explicit narrowing, not a gap), "
        "and still reconstructs its lineage.",
        dict(
            command_id="cmd:command_palette.open",
            lifecycle="stable",
            preview="no_preview_required",
            approval="no_approval_required",
            enablement="enabled",
            surfaces=p4_surface_classes,
            labels=["ui_only"],
            lineage_complete=True,
            rollback_required=False,
        ),
    ))

    # P5 — deprecated alias resolves to the canonical command without widening.
    p5_session = "inv:workspace.open_folder:2026-05-20T12-50-00Z:01"
    p5_surface_classes = ["command_palette", "cli_headless"]
    p5_surfaces = [
        surface(
            "command_palette",
            "inv:workspace.open_folder:command_palette:02",
        ),
        surface(
            "cli_headless",
            p5_session,
            authority="delegated_automation",
            dispatched=True,
            outcome="succeeded_with_warnings",
            resolves="cmd:workspace.open_folder",
        ),
    ]
    p5_lineage = lineage(
        "cmd:workspace.open_folder",
        p5_session,
        "result:workspace.open_folder:2026-05-20T12-50-00Z:01",
        "succeeded_with_warnings",
        evidence=[
            "journal-entry:workspace.open_folder:02",
            "support-export-row:workspace.open_folder:deprecated-alias:01",
        ],
        activities=["activity:workspace.open_folder:02"],
        support_bundle="support-bundle:workspace.open_folder:deprecated-alias:01",
    )
    drills.append((
        "positive.deprecated_alias_canonicalization",
        "positive/deprecated_alias_canonicalization.json",
        scenario(
            "scenario:command_authority:deprecated_alias_canonicalization",
            open_folder_descriptor(),
            p5_surfaces,
            p5_lineage,
        ),
        "A CLI invocation through a deprecated alias resolves to the canonical "
        "command id, records the alias warning in the result outcome, and keeps "
        "the same authority as the palette invocation.",
        dict(
            command_id="cmd:workspace.open_folder",
            lifecycle="stable",
            preview="no_preview_required",
            approval="no_approval_required",
            enablement="enabled",
            surfaces=p5_surface_classes,
            labels=OPEN_FOLDER_LABELS,
            lineage_complete=True,
            rollback_required=False,
        ),
    ))

    return drills


def palette_surface(session="inv:workspace.open_folder:command_palette:neg", **kw):
    return surface("command_palette", session, dispatched=True, outcome="succeeded", **kw)


def clean_open_folder_lineage(outcome="succeeded", **kw):
    base = dict(
        evidence=["journal-entry:workspace.open_folder:neg"],
        activities=["activity:workspace.open_folder:neg"],
        support_bundle="support-bundle:workspace.open_folder:neg",
    )
    base.update(kw)
    return lineage(
        "cmd:workspace.open_folder",
        "inv:workspace.open_folder:command_palette:neg",
        "result:workspace.open_folder:neg",
        outcome,
        **base,
    )


def build_negative():
    drills = []

    # N1 — AI surface reaches a command the descriptor does not surface to AI.
    drills.append((
        "negative.ai_tool_widens_authority",
        "negative/ai_tool_widens_authority.json",
        scenario(
            "scenario:command_authority:ai_tool_widens_authority",
            open_folder_descriptor(ai_class="not_ai_callable"),
            [
                palette_surface(),
                surface(
                    "ai_tool",
                    "inv:workspace.open_folder:ai_tool:neg",
                    authority="delegated_automation",
                ),
            ],
            clean_open_folder_lineage(),
        ),
        "widens authority",
        ["authority_widening", "ai_tool"],
    ))

    # N2 — CLI surface reaches a command that is not headless_safe.
    drills.append((
        "negative.cli_headless_widens_authority",
        "negative/cli_headless_widens_authority.json",
        scenario(
            "scenario:command_authority:cli_headless_widens_authority",
            open_folder_descriptor(labels=["macro_safe", "recipe_safe"]),
            [
                palette_surface(),
                surface(
                    "cli_headless",
                    "inv:workspace.open_folder:cli_headless:neg",
                    authority="delegated_automation",
                ),
            ],
            clean_open_folder_lineage(),
        ),
        "widens authority",
        ["authority_widening", "cli_headless", "headless_safe"],
    ))

    # N3 — a surface suppresses the structured-diff preview requirement.
    n3_lineage = lineage(
        "cmd:workspace.import_profile",
        "inv:workspace.import_profile:command_palette:neg",
        "result:workspace.import_profile:neg",
        "succeeded",
        evidence=["journal-entry:workspace.import_profile:neg"],
        activities=["activity:workspace.import_profile:neg"],
        rollback_posture="reversible_handle",
        rollback_id="rollback-handle:workspace.import_profile:neg",
    )
    drills.append((
        "negative.surface_suppresses_preview",
        "negative/surface_suppresses_preview.json",
        scenario(
            "scenario:command_authority:surface_suppresses_preview",
            import_profile_descriptor(),
            [
                surface(
                    "command_palette",
                    "inv:workspace.import_profile:command_palette:neg",
                    preview="structured_diff_preview",
                    approval="explicit_confirmation_required",
                    dispatched=True,
                    outcome="succeeded",
                ),
                surface(
                    "cli_headless",
                    "inv:workspace.import_profile:cli_headless:neg",
                    preview="no_preview_required",
                    approval="explicit_confirmation_required",
                    authority="delegated_automation",
                ),
            ],
            n3_lineage,
        ),
        "suppresses the preview requirement",
        ["preview_suppression"],
    ))

    # N4 — a surface suppresses the explicit-approval requirement.
    drills.append((
        "negative.surface_suppresses_approval",
        "negative/surface_suppresses_approval.json",
        scenario(
            "scenario:command_authority:surface_suppresses_approval",
            import_profile_descriptor(),
            [
                surface(
                    "command_palette",
                    "inv:workspace.import_profile:command_palette:neg",
                    preview="structured_diff_preview",
                    approval="explicit_confirmation_required",
                    dispatched=True,
                    outcome="succeeded",
                ),
                surface(
                    "cli_headless",
                    "inv:workspace.import_profile:cli_headless:neg",
                    preview="structured_diff_preview",
                    approval="no_approval_required",
                    authority="delegated_automation",
                ),
            ],
            n3_lineage,
        ),
        "suppresses the approval requirement",
        ["approval_suppression"],
    ))

    # N5 — surfaces disagree on the enablement decision.
    drills.append((
        "negative.enablement_divergence",
        "negative/enablement_divergence.json",
        scenario(
            "scenario:command_authority:enablement_divergence",
            open_folder_descriptor(),
            [
                palette_surface(),
                surface(
                    "menu_or_button",
                    "inv:workspace.open_folder:menu:neg",
                    decision="disabled_with_reason",
                    reason="workspace_trust_restricted",
                ),
            ],
            clean_open_folder_lineage(),
        ),
        "diverges from the canonical enablement decision",
        ["enablement_divergence"],
    ))

    # N6 — the approval_required label disagrees with the descriptor posture.
    drills.append((
        "negative.approval_label_mismatch",
        "negative/approval_label_mismatch.json",
        scenario(
            "scenario:command_authority:approval_label_mismatch",
            open_folder_descriptor(
                labels=["macro_safe", "recipe_safe", "headless_safe", "approval_required"]
            ),
            [palette_surface()],
            clean_open_folder_lineage(),
        ),
        "approval_required disagrees",
        ["automation_label_honesty", "approval_required"],
    ))

    # N7 — a UI-only command exposes a non-UI automation surface.
    drills.append((
        "negative.ui_only_exposes_automation",
        "negative/ui_only_exposes_automation.json",
        scenario(
            "scenario:command_authority:ui_only_exposes_automation",
            descriptor(
                "cmd:command_palette.open",
                "command_palette.open",
                capability="inert_metadata_only",
                ai_class="not_ai_callable",
                labels=["ui_only"],
                scopes=["desktop_product"],
            ),
            [
                surface(
                    "command_palette",
                    "inv:command_palette.open:command_palette:neg",
                    dispatched=True,
                    outcome="succeeded",
                ),
                surface(
                    "cli_headless",
                    "inv:command_palette.open:cli_headless:neg",
                    authority="delegated_automation",
                ),
            ],
            lineage(
                "cmd:command_palette.open",
                "inv:command_palette.open:command_palette:neg",
                "result:command_palette.open:neg",
                "succeeded",
                evidence=["ui-event-evidence:command_palette.open:neg"],
                activities=["activity:command_palette.open:neg"],
            ),
        ),
        "exposes a non-UI automation surface",
        ["automation_label_honesty", "ui_only"],
    ))

    # N8 — the lineage cannot be reconstructed without an evidence ref.
    drills.append((
        "negative.lineage_missing_evidence",
        "negative/lineage_missing_evidence.json",
        scenario(
            "scenario:command_authority:lineage_missing_evidence",
            open_folder_descriptor(),
            [palette_surface()],
            clean_open_folder_lineage(evidence=[]),
        ),
        "without an evidence ref",
        ["lineage_break"],
    ))

    # N9 — an alias does not resolve to the canonical command id.
    drills.append((
        "negative.alias_noncanonical",
        "negative/alias_noncanonical.json",
        scenario(
            "scenario:command_authority:alias_noncanonical",
            open_folder_descriptor(alias_canonical="cmd:workspace.open_other"),
            [palette_surface()],
            clean_open_folder_lineage(),
        ),
        "does not resolve to canonical command id",
        ["alias_canonicalization"],
    ))

    # N10 — a durable applied command drops its rollback handle id.
    drills.append((
        "negative.lineage_missing_rollback",
        "negative/lineage_missing_rollback.json",
        scenario(
            "scenario:command_authority:lineage_missing_rollback",
            import_profile_descriptor(),
            [
                surface(
                    "command_palette",
                    "inv:workspace.import_profile:command_palette:neg",
                    preview="structured_diff_preview",
                    approval="explicit_confirmation_required",
                    dispatched=True,
                    outcome="succeeded",
                ),
            ],
            lineage(
                "cmd:workspace.import_profile",
                "inv:workspace.import_profile:command_palette:neg",
                "result:workspace.import_profile:neg",
                "succeeded",
                evidence=["journal-entry:workspace.import_profile:neg"],
                activities=["activity:workspace.import_profile:neg"],
                rollback_posture="reversible_handle",
                rollback_id=None,
            ),
        ),
        "rollback_handle_id",
        ["lineage_break", "rollback"],
    ))

    # N11 — a stable command is missing machine-readable automation metadata.
    drills.append((
        "negative.stable_missing_automation_metadata",
        "negative/stable_missing_automation_metadata.json",
        scenario(
            "scenario:command_authority:stable_missing_automation_metadata",
            open_folder_descriptor(labels=[]),
            [palette_surface()],
            clean_open_folder_lineage(),
        ),
        "missing machine-readable automation metadata",
        ["machine_readable_contract"],
    ))

    return drills


# --------------------------------------------------------------------------- #
# Output assembly.
# --------------------------------------------------------------------------- #
def json_bytes(obj):
    return json.dumps(obj, indent=2) + "\n"


def fixture_payload(record, name, scenario_desc):
    payload = {
        "$schema": SCENARIO_SCHEMA_REL,
        "__fixture__": {"name": name, "scenario": scenario_desc},
    }
    payload.update(record)
    return payload


def build_outputs():
    positive = build_positive()
    negative = build_negative()
    outputs = {}

    for drill_id, filename, record, scenario_desc, _exp in positive:
        name = drill_id.split(".", 1)[1]
        outputs[f"{CORPUS_REL}/{filename}"] = json_bytes(
            fixture_payload(record, name, scenario_desc)
        )
    for drill_id, filename, record, _substring, _covers in negative:
        name = drill_id.split(".", 1)[1]
        scenario_desc = (
            "Negative drill: validation MUST fail with a message containing "
            f"`{_substring}`."
        )
        outputs[f"{CORPUS_REL}/{filename}"] = json_bytes(
            fixture_payload(record, name, scenario_desc)
        )

    # Manifest.
    positive_specs = []
    for drill_id, filename, _record, _desc, exp in positive:
        positive_specs.append({
            "drill_id": drill_id,
            "fixture": filename,
            "kind": "command_authority_scenario",
            "expected_command_id": exp["command_id"],
            "expected_lifecycle_state": exp["lifecycle"],
            "expected_preview_class": exp["preview"],
            "expected_approval_posture_class": exp["approval"],
            "expected_enablement_decision_class": exp["enablement"],
            "expected_surface_classes": exp["surfaces"],
            "expected_automation_labels": exp["labels"],
            "expected_lineage_complete": exp["lineage_complete"],
            "expected_rollback_required": exp["rollback_required"],
        })
    negative_specs = []
    for drill_id, filename, _record, substring, covers in negative:
        negative_specs.append({
            "drill_id": drill_id,
            "fixture": filename,
            "kind": "command_authority_scenario",
            "expected_failure_substring": substring,
            "covers": covers,
        })

    manifest = {
        "$schema": MANIFEST_SCHEMA_REL,
        "record_kind": "command_authority_conformance_corpus_record",
        "corpus_id": CORPUS_ID,
        "schema_version": 1,
        "description": (
            "Conformance and interoperability corpus for the M3 command-truth and "
            "palette-authority beta boundary owned by aureline-commands "
            "(CommandAuthorityScenarioRecord). This manifest is the single source of "
            "truth: every positive drill MUST parse, validate, project, and match every "
            "expected_* field below; every negative drill MUST FAIL validation with an "
            "error whose message contains expected_failure_substring. The corpus proves "
            "the one-command-graph promise: the same canonical command keeps one "
            "enablement decision, one preview/approval posture, and one result contract "
            "across menu/button, keybinding, palette, CLI/headless, AI, recipe, voice, and "
            "browser-companion surfaces; no surface widens authority or suppresses a preview "
            "or approval requirement; automation labels (macro_safe, recipe_safe, "
            "headless_safe, ui_only, approval_required) stay honest; migration aliases stay "
            "generated from the canonical descriptor; and a support export can reconstruct "
            "invocation lineage from a command id through the result packet, evidence ref, "
            "notification/activity row, and rollback handle."),
        "boundary": {
            "runtime_model": "aureline_commands::CommandAuthorityScenarioRecord",
            "harness": "crates/aureline-qe/src/command_truth_authority/",
            "parity_report_ref": PARITY_REPORT_REL,
            "evidence_packet_ref": EVIDENCE_PACKET_REL,
        },
        "positive_drills": positive_specs,
        "negative_drills": negative_specs,
    }
    outputs[f"{CORPUS_REL}/manifest.json"] = json_bytes(manifest)

    # README.
    outputs[f"{CORPUS_REL}/README.md"] = build_readme(positive, negative)

    # Release evidence packet.
    results = []
    for drill_id, _filename, record, _desc, exp in positive:
        lin = record["lineage"]
        desc = record["canonical_descriptor"]
        results.append({
            "drill_id": drill_id,
            "scenario_id": record["scenario_id"],
            "command_id": desc["command_id"],
            "canonical_verb": desc["canonical_verb"],
            "lifecycle_state": desc["lifecycle_state"],
            "preview_class": desc["preview_class"],
            "approval_posture_class": desc["approval_posture_class"],
            "agreed_enablement_decision_class": exp["enablement"],
            "automation_labels": desc["automation_labels"],
            "surface_classes_covered": sorted({s["surface_class"] for s in record["surfaces"]}),
            "parity_clean": True,
            "lineage_complete": exp["lineage_complete"],
            "rollback_required": exp["rollback_required"],
            "lineage_chain": lineage_chain(lin),
        })
    packet = {
        "$schema": PACKET_SCHEMA_REL,
        "record_kind": "command_invocation_evidence_packet_record",
        "schema_version": 1,
        "packet_id": "command_invocation_evidence_packet.m3.01",
        "corpus_id": CORPUS_ID,
        "generated_by": "tools/regenerate_command_truth_authority_corpus.py",
        "summary": {
            "command_count": len(results),
            "surfaces_proven": list(ALL_SURFACES),
            "negative_drill_count": len(negative),
            "all_parity_clean": True,
        },
        "results": results,
        "negative_drills": [
            {"drill_id": d, "guards": substring}
            for d, _f, _r, substring, _c in negative
        ],
    }
    outputs[EVIDENCE_PACKET_REL] = json_bytes(packet)

    # UX parity report.
    outputs[PARITY_REPORT_REL] = build_parity_report(positive, negative, results)

    return outputs


def build_readme(positive, negative):
    lines = [
        "# Command-truth and palette-authority corpus",
        "",
        "Conformance / interoperability corpus for the M3 command-truth and",
        "palette-authority beta boundary owned by",
        "`aureline_commands::CommandAuthorityScenarioRecord`.",
        "",
        "`manifest.json` is authoritative. Positive drills MUST parse, validate,",
        "project, and match **every** `expected_*` field in the manifest. Negative",
        "drills MUST FAIL validation with an error whose message contains",
        "`expected_failure_substring`. The fixtures carry only the scenario records",
        "(plus a `$schema`/`__fixture__` prelude) — they do **not** restate the",
        "expectations, so there is exactly one place to read the pinned truth.",
        "",
        "Replay: `cargo test -p aureline-qe --test command_truth_authority_conformance`.",
        "Regenerate: `python3 tools/regenerate_command_truth_authority_corpus.py --write`.",
        "",
        "## Positive scenarios",
        "",
    ]
    for drill_id, filename, _record, scenario_desc, _exp in positive:
        lines.append(f"- `{drill_id}` (`{filename}`): {scenario_desc}")
    lines.extend(["", "## Negative scenarios", ""])
    for drill_id, filename, _record, substring, _covers in negative:
        lines.append(f"- `{drill_id}` (`{filename}`): rejected with `{substring}`.")
    lines.extend([
        "",
        "## Redaction",
        "",
        "Every fixture is metadata-safe: only opaque refs and typed labels cross the",
        "boundary. Raw secrets, private keys, credentials, and raw local paths never",
        "appear; the runner scans each fixture for forbidden raw-content tokens before",
        "validation.",
        "",
    ])
    return "\n".join(lines)


def build_parity_report(positive, negative, results):
    lines = [
        "# Command-truth and palette-authority parity report (beta)",
        "",
        f"Corpus: `{CORPUS_ID}`",
        "",
        "Generated by `tools/regenerate_command_truth_authority_corpus.py`; replayed by",
        "`cargo test -p aureline-qe --test command_truth_authority_conformance` against the",
        "real `aureline_commands::CommandAuthorityScenarioRecord` validator. This report and",
        "the release evidence packet are generated from the same corpus, so they cannot drift",
        "from the harness expectations.",
        "",
        "## What the corpus proves",
        "",
        "- The same canonical command keeps one enablement decision, one preview/approval",
        "  posture, and one result contract across menu/button, keybinding, palette,",
        "  CLI/headless, AI, recipe, voice, and browser-companion surfaces.",
        "- No invocation surface widens authority or suppresses a preview or approval",
        "  requirement relative to the canonical descriptor.",
        "- Automation labels (`macro_safe`, `recipe_safe`, `headless_safe`, `ui_only`,",
        "  `approval_required`) stay honest, and migration aliases stay generated from the",
        "  canonical descriptor record instead of hand-maintained shadow data.",
        "- A support export reconstructs invocation lineage from a command id through the",
        "  result packet, evidence ref, notification/activity row, and rollback handle.",
        "",
        "## Command-truth rows",
        "",
        "| Drill | Command | Lifecycle | Preview | Approval | Enablement | Surfaces | Automation labels | Lineage | Rollback |",
        "| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |",
    ]
    for r in results:
        lines.append(
            "| `{drill}` | `{cmd}` | {life} | {prev} | {appr} | {enable} | {surf} | {labels} | {lineage} | {rb} |".format(
                drill=r["drill_id"],
                cmd=r["command_id"],
                life=r["lifecycle_state"],
                prev=r["preview_class"],
                appr=r["approval_posture_class"],
                enable=r["agreed_enablement_decision_class"],
                surf=str(len(r["surface_classes_covered"])),
                labels=", ".join(r["automation_labels"]),
                lineage="complete" if r["lineage_complete"] else "INCOMPLETE",
                rb="required" if r["rollback_required"] else "n/a",
            )
        )
    lines.extend([
        "",
        "## Negative guards",
        "",
        "Each negative drill proves the validator rejects an authority regression before a",
        "beta command row hardens.",
        "",
        "| Drill | Rejected with |",
        "| --- | --- |",
    ])
    for drill_id, _filename, _record, substring, _covers in negative:
        lines.append(f"| `{drill_id}` | `{substring}` |")
    lines.append("")
    return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    group = parser.add_mutually_exclusive_group()
    group.add_argument("--write", action="store_true", help="write generated files (default)")
    group.add_argument("--check", action="store_true",
                       help="fail if any committed file differs from the generated output")
    args = parser.parse_args()

    outputs = build_outputs()

    if args.check:
        drift = []
        for rel, content in sorted(outputs.items()):
            path = os.path.join(ROOT, rel)
            try:
                with open(path, "r", encoding="utf-8") as fh:
                    current = fh.read()
            except FileNotFoundError:
                drift.append(f"missing: {rel}")
                continue
            if current != content:
                drift.append(f"out of date: {rel}")
        if drift:
            print("command-truth and palette-authority corpus is out of sync with its generator:")
            for line in drift:
                print(f"  - {line}")
            print("run: python3 tools/regenerate_command_truth_authority_corpus.py --write")
            return 1
        print(f"command-truth and palette-authority corpus matches generator ({len(outputs)} files)")
        return 0

    for rel, content in sorted(outputs.items()):
        path = os.path.join(ROOT, rel)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w", encoding="utf-8") as fh:
            fh.write(content)
    print(f"wrote {len(outputs)} files for the command-truth and palette-authority corpus")
    return 0


if __name__ == "__main__":
    sys.exit(main())
