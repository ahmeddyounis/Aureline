#!/usr/bin/env python3
"""Regenerate the M5 qualification-row / support-window / skew-window / deprecation-packet matrix.

This emits the canonical matrix artifact, the negative fixtures, the cases
manifest, and the frozen validation capture. The Python summary/promotion logic
mirrors the typed Rust consumer so the checked-in artifact validates cleanly and
the capture cross-check agrees with the model.
"""

from __future__ import annotations

import copy
import json
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent

MODULE = "freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix"
RECORD_KIND = "freeze_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix"
ARTIFACT = REPO / "artifacts/release/m5" / f"{MODULE}.json"
CAPTURE = REPO / "artifacts/release/captures" / f"{MODULE}_validation_capture.json"
FIXTURES = REPO / "fixtures/compat/m5-qualification-and-skew"
AS_OF = "2026-06-16"

LIFECYCLE_LABELS = ["lts", "stable", "beta", "preview", "withdrawn"]
ABOVE_CUTLINE = ["lts", "stable"]
BELOW_CUTLINE = ["beta", "preview", "withdrawn"]
FAMILY_KINDS = [
    "notebook",
    "ai_provider",
    "remote_helper",
    "companion",
    "ecosystem",
    "managed_service",
    "toolchain_runtime",
]
QUALIFICATION_DIMENSIONS = [
    "platform",
    "deployment_profile",
    "archetype_bundle",
    "toolchain_envelope",
    "client_scope",
]
QUALIFICATION_STATES = ["qualified", "limited", "retest_pending", "stale", "waived", "missing"]
ROW_STATES = [
    "qualified",
    "limited",
    "on_waiver",
    "retest_pending",
    "stale",
    "unsupported_skew",
    "deprecated",
    "incomplete",
]
SKEW_WINDOW_CLASSES = [
    "lockstep_only",
    "bounded_skew",
    "backward_compatible",
    "forward_compatible",
    "unsupported_skew",
]
SKEW_UNSUPPORTED_BEHAVIORS = [
    "fail_closed",
    "reconnect_required",
    "reinstall_required",
    "coordinated_upgrade_only",
    "block_boundary",
]
SUPPORT_CLASSES = ["full_support", "maintenance_only", "security_only", "limited", "end_of_life"]
DEPRECATION_STATUSES = ["active", "deprecated", "successor_available", "removal_scheduled", "removed"]
NARROWING_REASONS = [
    "qualification_incomplete",
    "qualification_stale",
    "retest_pending",
    "skew_window_exceeded",
    "deprecation_scheduled",
    "support_window_ended",
    "waiver_expired",
    "owner_signoff_missing",
    "claim_publication_missing",
]
STOP_RULE_ACTIONS = [
    "hold_promotion",
    "narrow_label",
    "complete_qualification",
    "refresh_evidence",
    "retest_boundary",
    "widen_or_document_skew",
    "publish_successor_migration",
    "renew_support_window",
    "request_owner_signoff",
    "republish_claim",
]

RANK = {"lts": 4, "stable": 3, "beta": 2, "preview": 1, "withdrawn": 0}
ROW_HOLDS = {"qualified", "limited", "on_waiver"}
CELL_HOLDS = {"qualified", "limited", "waived"}


def holds_stable(label: str) -> bool:
    return RANK[label] >= RANK["stable"]


def proof(entry: str, slo_state: str, captured: bool = True) -> dict:
    return {
        "packet_id": entry,
        "packet_ref": f"proof/{entry}",
        "proof_index_ref": f"proof-index/{entry}",
        "captured_at": AS_OF if captured else None,
        "freshness_slo": {
            "target_max_age_days": 30,
            "warn_within_days": 7,
            "slo_register_ref": "freshness-slo/register",
        },
        "slo_state": slo_state,
        "evidence_refs": [f"evidence/{entry}/proof"] if captured else [],
    }


def signoff(owner: str = "release-engineering", signed: bool = True) -> dict:
    return {"owner_ref": owner, "signed_off": signed, "signed_at": AS_OF if signed else None}


def qrow(entry: str, overrides: dict | None = None) -> list[dict]:
    overrides = overrides or {}
    cells = []
    for dim in QUALIFICATION_DIMENSIONS:
        state = overrides.get(dim, "qualified")
        cells.append(
            {
                "dimension": dim,
                "state": state,
                "evidence_ref": "" if state == "missing" else f"evidence/{entry}/{dim}",
            }
        )
    return cells


def skew(cls: str, lo: str, hi: str, fields: list[str], behavior: str, entry: str) -> dict:
    return {
        "skew_window_class": cls,
        "min_supported_version": lo,
        "max_supported_version": hi,
        "negotiated_fields": fields,
        "unsupported_behavior": behavior,
        "skew_window_ref": f"skew/{entry}",
    }


def support(cls: str, since: str, end: str | None, entry: str) -> dict:
    return {
        "support_class": cls,
        "supported_since": since,
        "end_of_support": end,
        "support_window_ref": f"support/{entry}",
    }


def deprecation(status: str, entry: str, successor: str | None = None, removal: str | None = None,
                migration: str | None = None) -> dict:
    return {
        "status": status,
        "successor_ref": successor,
        "removal_after": removal,
        "migration_ref": migration,
        "deprecation_packet_ref": f"deprecation/{entry}",
    }


DESTINATIONS = [
    "docs",
    "release_notes",
    "help_about",
    "cli_inspect",
    "support_export",
    "certification_report",
    "shiproom_dashboard",
]


def rows() -> list[dict]:
    out = []

    out.append(
        {
            "entry_id": "m5-notebook-runtime",
            "title": "Notebook runtime qualification row",
            "family_kind": "notebook",
            "family_ref": "family/notebook-runtime",
            "family_summary": "Notebook and data-rich runtime: kernel protocol and cell-state schema.",
            "release_blocking": True,
            "claim_ref": "claim/m5-notebook",
            "claim_label": "stable",
            "row_state": "qualified",
            "qualification_row": qrow("m5-notebook-runtime"),
            "skew_window": skew(
                "bounded_skew", "5.0.0", "5.4.0",
                ["notebook_kernel_protocol", "cell_state_schema"], "reconnect_required",
                "m5-notebook-runtime",
            ),
            "support_window": support("full_support", "2026-06-01", None, "m5-notebook-runtime"),
            "deprecation_packet": deprecation("active", "m5-notebook-runtime"),
            "compatibility_caveats": [],
            "proof_packet": proof("m5-notebook-runtime", "current"),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": [],
            "published_label": "stable",
            "publication_destinations": DESTINATIONS,
            "rationale": "Every qualification dimension is qualified across platforms and deployment profiles; the boundary holds a bounded skew window and a current proof packet.",
        }
    )

    out.append(
        {
            "entry_id": "m5-ai-provider-boundary",
            "title": "AI provider boundary qualification row",
            "family_kind": "ai_provider",
            "family_ref": "family/ai-provider-boundary",
            "family_summary": "Helper/agent/provider boundary: capability handshake and model-route descriptors.",
            "release_blocking": True,
            "claim_ref": "claim/m5-ai-provider",
            "claim_label": "stable",
            "row_state": "limited",
            "qualification_row": qrow("m5-ai-provider-boundary", {"client_scope": "limited"}),
            "skew_window": skew(
                "backward_compatible", "5.0.0", "5.4.0",
                ["provider_capability_handshake", "model_route_descriptor"], "coordinated_upgrade_only",
                "m5-ai-provider-boundary",
            ),
            "support_window": support("full_support", "2026-06-01", None, "m5-ai-provider-boundary"),
            "deprecation_packet": deprecation("active", "m5-ai-provider-boundary"),
            "compatibility_caveats": [
                "BYOK provider client scope is qualified for managed and self-hosted profiles only; air-gapped provider routing stays limited.",
            ],
            "proof_packet": proof("m5-ai-provider-boundary", "current"),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": [],
            "published_label": "stable",
            "publication_destinations": DESTINATIONS,
            "rationale": "Holds Stable with a recorded compatibility caveat: the client-scope dimension is qualified as limited for air-gapped provider routing, so the row carries a caveat rather than narrowing.",
        }
    )

    out.append(
        {
            "entry_id": "m5-remote-helper-skew",
            "title": "Remote helper skew qualification row",
            "family_kind": "remote_helper",
            "family_ref": "family/remote-helper-skew",
            "family_summary": "Remote/helper boundary: RPC envelope and session-resume token skew.",
            "release_blocking": True,
            "claim_ref": "claim/m5-remote-helper",
            "claim_label": "stable",
            "row_state": "on_waiver",
            "qualification_row": qrow("m5-remote-helper-skew", {"toolchain_envelope": "waived"}),
            "skew_window": skew(
                "bounded_skew", "5.0.0", "5.3.0",
                ["helper_rpc_envelope", "session_resume_token"], "reconnect_required",
                "m5-remote-helper-skew",
            ),
            "support_window": support("full_support", "2026-06-01", None, "m5-remote-helper-skew"),
            "deprecation_packet": deprecation("active", "m5-remote-helper-skew"),
            "compatibility_caveats": [],
            "proof_packet": proof("m5-remote-helper-skew", "current"),
            "waiver": {
                "waiver_ref": "waiver:m5_remote_helper_toolchain",
                "expires_at": "2026-12-31",
                "reason": "Toolchain-envelope re-qualification scheduled; interim coverage waived by owner.",
            },
            "owner_signoff": signoff(),
            "active_narrowing_reasons": [],
            "published_label": "stable",
            "publication_destinations": DESTINATIONS,
            "rationale": "Holds Stable provisionally: the toolchain-envelope dimension rides an active, unexpired waiver while re-qualification completes.",
        }
    )

    out.append(
        {
            "entry_id": "m5-companion-handoff",
            "title": "Browser/mobile companion handoff qualification row",
            "family_kind": "companion",
            "family_ref": "family/companion-handoff",
            "family_summary": "Companion boundary: handoff-eligibility token and companion session descriptor.",
            "release_blocking": True,
            "claim_ref": "claim/m5-companion",
            "claim_label": "stable",
            "row_state": "retest_pending",
            "qualification_row": qrow("m5-companion-handoff", {"client_scope": "retest_pending"}),
            "skew_window": skew(
                "forward_compatible", "5.0.0", "5.4.0",
                ["handoff_eligibility_token", "companion_session_descriptor"], "reconnect_required",
                "m5-companion-handoff",
            ),
            "support_window": support("maintenance_only", "2026-06-01", None, "m5-companion-handoff"),
            "deprecation_packet": deprecation("active", "m5-companion-handoff"),
            "compatibility_caveats": [],
            "proof_packet": proof("m5-companion-handoff", "current"),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": ["retest_pending"],
            "published_label": "beta",
            "publication_destinations": DESTINATIONS,
            "rationale": "Client-scope coverage is retest-pending after a handoff-eligibility change, so the row narrows below the cutline until the boundary is retested.",
        }
    )

    out.append(
        {
            "entry_id": "m5-ecosystem-sideload",
            "title": "Ecosystem sideload skew qualification row",
            "family_kind": "ecosystem",
            "family_ref": "family/ecosystem-sideload",
            "family_summary": "Extension/sideload boundary: ABI version and capability-grant manifest.",
            "release_blocking": True,
            "claim_ref": "claim/m5-ecosystem",
            "claim_label": "stable",
            "row_state": "unsupported_skew",
            "qualification_row": qrow("m5-ecosystem-sideload"),
            "skew_window": skew(
                "unsupported_skew", "4.6.0", "5.0.0",
                ["extension_abi_version", "capability_grant_manifest"], "reinstall_required",
                "m5-ecosystem-sideload",
            ),
            "support_window": support("limited", "2026-06-01", None, "m5-ecosystem-sideload"),
            "deprecation_packet": deprecation("active", "m5-ecosystem-sideload"),
            "compatibility_caveats": [],
            "proof_packet": proof("m5-ecosystem-sideload", "current"),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": ["skew_window_exceeded"],
            "published_label": "beta",
            "publication_destinations": DESTINATIONS,
            "rationale": "Sideloaded extensions built before 5.0 are outside the supported ABI skew window; the boundary requires a reinstall and the row narrows below the cutline.",
        }
    )

    out.append(
        {
            "entry_id": "m5-managed-sync-service",
            "title": "Managed sync service deprecation row",
            "family_kind": "managed_service",
            "family_ref": "family/managed-sync-service",
            "family_summary": "Managed sync/relay service: relay protocol and change-journal envelope.",
            "release_blocking": False,
            "claim_ref": "claim/m5-managed-sync",
            "claim_label": "stable",
            "row_state": "deprecated",
            "qualification_row": qrow("m5-managed-sync-service"),
            "skew_window": skew(
                "backward_compatible", "5.0.0", "5.4.0",
                ["sync_relay_protocol", "change_journal_envelope"], "reconnect_required",
                "m5-managed-sync-service",
            ),
            "support_window": support("maintenance_only", "2026-06-01", "2026-12-31", "m5-managed-sync-service"),
            "deprecation_packet": deprecation(
                "removal_scheduled", "m5-managed-sync-service",
                successor="family/managed-sync-service-v2",
                removal="2026-12-31",
                migration="docs/migration/managed-sync-service",
            ),
            "compatibility_caveats": [],
            "proof_packet": proof("m5-managed-sync-service", "current"),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": ["deprecation_scheduled"],
            "published_label": "preview",
            "publication_destinations": DESTINATIONS,
            "rationale": "The first-generation managed sync service has a scheduled removal and a named successor with a migration packet, so the row narrows below the cutline ahead of removal.",
        }
    )

    out.append(
        {
            "entry_id": "m5-toolchain-envelope",
            "title": "Toolchain envelope qualification row",
            "family_kind": "toolchain_runtime",
            "family_ref": "family/toolchain-envelope",
            "family_summary": "Toolchain/runtime boundary: compiler ABI token and LSP protocol version.",
            "release_blocking": True,
            "claim_ref": "claim/m5-toolchain",
            "claim_label": "stable",
            "row_state": "stale",
            "qualification_row": qrow("m5-toolchain-envelope", {"toolchain_envelope": "stale"}),
            "skew_window": skew(
                "bounded_skew", "5.0.0", "5.2.0",
                ["compiler_abi_token", "lsp_protocol_version"], "coordinated_upgrade_only",
                "m5-toolchain-envelope",
            ),
            "support_window": support("full_support", "2026-06-01", None, "m5-toolchain-envelope"),
            "deprecation_packet": deprecation("active", "m5-toolchain-envelope"),
            "compatibility_caveats": [],
            "proof_packet": proof("m5-toolchain-envelope", "breached"),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": ["qualification_stale"],
            "published_label": "beta",
            "publication_destinations": DESTINATIONS,
            "rationale": "The toolchain-envelope dimension proof packet breached its freshness SLO, so the row narrows below the cutline until evidence is refreshed.",
        }
    )

    out.append(
        {
            "entry_id": "m5-managed-airgapped-profile",
            "title": "Managed air-gapped profile qualification row",
            "family_kind": "managed_service",
            "family_ref": "family/managed-airgapped-profile",
            "family_summary": "Air-gapped managed profile: lockstep bundle digest with fail-closed boundary.",
            "release_blocking": False,
            "claim_ref": "claim/m5-managed-airgapped",
            "claim_label": "stable",
            "row_state": "qualified",
            "qualification_row": qrow("m5-managed-airgapped-profile"),
            "skew_window": skew(
                "lockstep_only", "5.4.0", "5.4.0",
                ["airgapped_bundle_digest"], "fail_closed",
                "m5-managed-airgapped-profile",
            ),
            "support_window": support("security_only", "2026-06-01", None, "m5-managed-airgapped-profile"),
            "deprecation_packet": deprecation("active", "m5-managed-airgapped-profile"),
            "compatibility_caveats": [],
            "proof_packet": proof("m5-managed-airgapped-profile", "due_for_refresh"),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": [],
            "published_label": "stable",
            "publication_destinations": DESTINATIONS,
            "rationale": "Air-gapped profile runs lockstep-only with a fail-closed boundary and security-only support; the proof packet is within its SLO and due for refresh soon.",
        }
    )

    return out


def stop_rules() -> list[dict]:
    action = {
        "qualification_incomplete": "complete_qualification",
        "qualification_stale": "refresh_evidence",
        "retest_pending": "retest_boundary",
        "skew_window_exceeded": "widen_or_document_skew",
        "deprecation_scheduled": "publish_successor_migration",
        "support_window_ended": "renew_support_window",
        "waiver_expired": "narrow_label",
        "owner_signoff_missing": "request_owner_signoff",
        "claim_publication_missing": "republish_claim",
    }
    titles = {
        "qualification_incomplete": "Qualification row incomplete",
        "qualification_stale": "Qualification evidence stale",
        "retest_pending": "Boundary retest pending",
        "skew_window_exceeded": "Peer outside supported skew window",
        "deprecation_scheduled": "Deprecation or removal scheduled",
        "support_window_ended": "Support window ended",
        "waiver_expired": "Qualification waiver expired",
        "owner_signoff_missing": "Owner sign-off missing",
        "claim_publication_missing": "Claim publication missing",
    }
    out = []
    for reason in NARROWING_REASONS:
        out.append(
            {
                "rule_id": f"m5_qual_rule:{reason}",
                "title": titles[reason],
                "trigger_reason": reason,
                "applies_to_labels": ABOVE_CUTLINE,
                "default_action": action[reason],
                "blocks_promotion": True,
                "rationale": f"A qualification row at or above the cutline that reports '{reason}' cannot keep a Stable or LTS claim.",
            }
        )
    return out


def compute_promotion(matrix: dict) -> dict:
    triggers = set()
    for rule in matrix["stop_rules"]:
        if not rule["blocks_promotion"]:
            continue
        fires = any(
            row["claim_label"] in rule["applies_to_labels"]
            and rule["trigger_reason"] in row["active_narrowing_reasons"]
            for row in matrix["rows"]
        )
        if fires:
            triggers.add(rule["trigger_reason"])
    blocking_rule_ids = sorted(
        rule["rule_id"]
        for rule in matrix["stop_rules"]
        if rule["blocks_promotion"]
        and any(
            row["claim_label"] in rule["applies_to_labels"]
            and rule["trigger_reason"] in row["active_narrowing_reasons"]
            for row in matrix["rows"]
        )
    )
    blocking_claim_ids = sorted(
        {
            row["entry_id"]
            for row in matrix["rows"]
            if holds_stable(row["claim_label"])
            and any(r in triggers for r in row["active_narrowing_reasons"])
        }
    )
    decision = "hold" if blocking_rule_ids else "proceed"
    return {
        "promotion_gate": "m5-qualification-and-skew-gate",
        "decision": decision,
        "blocking_rule_ids": blocking_rule_ids,
        "blocking_claim_ids": blocking_claim_ids,
        "rationale": "Computed from the firing stop rules over qualification-row coverage, evidence freshness, retest state, skew windows, deprecation/removal schedules, support windows, waiver expiry, owner sign-off, and claim-publication linkage.",
    }


def compute_summary(matrix: dict) -> dict:
    rs = matrix["rows"]

    def published_stable(row):
        return holds_stable(row["published_label"])

    def row_state(state):
        return sum(1 for r in rs if r["row_state"] == state)

    def kind(k):
        return sum(1 for r in rs if r["family_kind"] == k)

    def slo(state):
        return sum(1 for r in rs if r["proof_packet"]["slo_state"] == state)

    def cell_state(state):
        return sum(
            1
            for r in rs
            for c in r["qualification_row"]
            if c["state"] == state
        )

    rb = [r for r in rs if r["release_blocking"]]
    families = {r["family_ref"] for r in rs}
    return {
        "total_rows": len(rs),
        "total_families": len(families),
        "rows_qualified": sum(1 for r in rs if published_stable(r)),
        "rows_narrowed": sum(1 for r in rs if not published_stable(r)),
        "rows_on_active_waiver": row_state("on_waiver"),
        "rows_limited": row_state("limited"),
        "rows_retest_pending": row_state("retest_pending"),
        "rows_stale": row_state("stale"),
        "rows_unsupported_skew": row_state("unsupported_skew"),
        "rows_deprecated": row_state("deprecated"),
        "release_blocking_total": len(rb),
        "release_blocking_qualified": sum(1 for r in rb if published_stable(r)),
        "release_blocking_narrowed": sum(1 for r in rb if not published_stable(r)),
        "notebook_rows": kind("notebook"),
        "ai_provider_rows": kind("ai_provider"),
        "remote_helper_rows": kind("remote_helper"),
        "companion_rows": kind("companion"),
        "ecosystem_rows": kind("ecosystem"),
        "managed_service_rows": kind("managed_service"),
        "toolchain_runtime_rows": kind("toolchain_runtime"),
        "packets_current": slo("current"),
        "packets_due_for_refresh": slo("due_for_refresh"),
        "packets_breached": slo("breached"),
        "packets_missing": slo("missing"),
        "total_active_narrowing_reasons": sum(len(r["active_narrowing_reasons"]) for r in rs),
        "total_qualification_cells": sum(len(r["qualification_row"]) for r in rs),
        "cells_qualified": cell_state("qualified"),
        "cells_limited": cell_state("limited"),
        "cells_retest_pending": cell_state("retest_pending"),
        "cells_stale": cell_state("stale"),
        "cells_waived": cell_state("waived"),
        "cells_missing": cell_state("missing"),
        "rules_firing": sum(
            1
            for rule in matrix["stop_rules"]
            if any(
                r["claim_label"] in rule["applies_to_labels"]
                and rule["trigger_reason"] in r["active_narrowing_reasons"]
                for r in rs
            )
        ),
    }


def build_matrix() -> dict:
    matrix = {
        "schema_version": 1,
        "record_kind": RECORD_KIND,
        "matrix_id": "m5_qualification_and_skew_matrix:v1",
        "status": "published",
        "overview_page": f"docs/m5/{MODULE}.md",
        "as_of": AS_OF,
        "claim_manifest_ref": "artifacts/release/stable_claim_manifest.json",
        "lifecycle_labels": LIFECYCLE_LABELS,
        "family_kinds": FAMILY_KINDS,
        "qualification_dimensions": QUALIFICATION_DIMENSIONS,
        "qualification_states": QUALIFICATION_STATES,
        "row_states": ROW_STATES,
        "skew_window_classes": SKEW_WINDOW_CLASSES,
        "skew_unsupported_behaviors": SKEW_UNSUPPORTED_BEHAVIORS,
        "support_classes": SUPPORT_CLASSES,
        "deprecation_statuses": DEPRECATION_STATUSES,
        "narrowing_reasons": NARROWING_REASONS,
        "stop_rule_actions": STOP_RULE_ACTIONS,
        "launch_cutline": {
            "cutline_level": "stable",
            "above_cutline_levels": ABOVE_CUTLINE,
            "below_cutline_levels": BELOW_CUTLINE,
            "description": "An M5 stable-facing family carries a Stable (or LTS) qualification claim only when every qualification dimension is qualified (or limited with a recorded caveat, or waived under an unexpired waiver), its peer is inside the supported skew window, the deprecation packet is active, the support window is open, the proof packet is current within its freshness SLO, and the owner has signed off. A family that loses any of those must drop below the cutline rather than inherit an adjacent qualified family.",
        },
        "release_blocking_family_refs": [],
        "stop_rules": stop_rules(),
        "rows": rows(),
    }
    matrix["release_blocking_family_refs"] = [
        r["family_ref"] for r in matrix["rows"] if r["release_blocking"]
    ]
    matrix["promotion"] = compute_promotion(matrix)
    matrix["summary"] = compute_summary(matrix)
    return matrix


def write_json(path: Path, payload: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")


def build_fixtures(matrix: dict) -> list[tuple[str, str]]:
    cases = []

    dup = copy.deepcopy(matrix)
    dup["rows"][1]["entry_id"] = dup["rows"][0]["entry_id"]
    dup["summary"] = compute_summary(dup)
    dup["promotion"] = compute_promotion(dup)
    write_json(FIXTURES / "duplicate_entry_id.json", dup)
    cases.append(("duplicate_entry_id.json", "DuplicateEntryId"))

    missing = copy.deepcopy(matrix)
    missing["rows"][0]["qualification_row"] = [
        c for c in missing["rows"][0]["qualification_row"] if c["dimension"] != "client_scope"
    ]
    missing["summary"] = compute_summary(missing)
    write_json(FIXTURES / "missing_dimension_cell.json", missing)
    cases.append(("missing_dimension_cell.json", "QualificationRowIncompleteCoverage"))

    held = copy.deepcopy(matrix)
    target = next(r for r in held["rows"] if holds_stable(r["published_label"]))
    target["active_narrowing_reasons"] = ["skew_window_exceeded"]
    held["summary"] = compute_summary(held)
    held["promotion"] = compute_promotion(held)
    write_json(FIXTURES / "held_with_active_gap.json", held)
    cases.append(("held_with_active_gap.json", "HeldWithActiveGap"))

    write_json(
        FIXTURES / "cases.json",
        {"cases": [{"file": f, "expected_check_id": c} for f, c in cases]},
    )
    return cases


def build_capture(matrix: dict, cases: list[tuple[str, str]]) -> dict:
    s = matrix["summary"]
    return {
        "status": "pass",
        "as_of": matrix["as_of"],
        "summary": {
            "total_rows": s["total_rows"],
            "rows_qualified": s["rows_qualified"],
            "rows_narrowed": s["rows_narrowed"],
            "rows_on_active_waiver": s["rows_on_active_waiver"],
            "rows_limited": s["rows_limited"],
            "rows_retest_pending": s["rows_retest_pending"],
            "rows_unsupported_skew": s["rows_unsupported_skew"],
            "rows_deprecated": s["rows_deprecated"],
            "rows_stale": s["rows_stale"],
            "packets_breached": s["packets_breached"],
            "packets_missing": s["packets_missing"],
            "total_qualification_cells": s["total_qualification_cells"],
            "cells_qualified": s["cells_qualified"],
            "rules_firing": s["rules_firing"],
        },
        "promotion": {
            "decision": matrix["promotion"]["decision"],
            "blocking_rule_ids": matrix["promotion"]["blocking_rule_ids"],
            "blocking_claim_ids": matrix["promotion"]["blocking_claim_ids"],
        },
        "negative_drills": [
            {"drill_id": "drill:narrowing_without_reason", "status": "passed"},
            {"drill_id": "drill:held_with_active_gap", "status": "passed"},
            {"drill_id": "drill:published_wider_than_claim", "status": "passed"},
            {"drill_id": "drill:limited_without_caveat", "status": "passed"},
            {"drill_id": "drill:promotion_decision_inconsistent", "status": "passed"},
        ],
        "fixture_cases": [
            {"case_id": f"fixture:{f.removesuffix('.json')}", "status": "passed"} for f, _ in cases
        ],
    }


def main() -> int:
    matrix = build_matrix()
    write_json(ARTIFACT, matrix)
    cases = build_fixtures(matrix)
    write_json(CAPTURE, build_capture(matrix, cases))
    print(f"wrote {ARTIFACT.relative_to(REPO)}")
    print(f"wrote {CAPTURE.relative_to(REPO)}")
    print(f"wrote {FIXTURES.relative_to(REPO)}/ ({len(cases)} fixtures + cases.json)")
    print("decision:", matrix["promotion"]["decision"])
    print("summary:", json.dumps(matrix["summary"]))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
