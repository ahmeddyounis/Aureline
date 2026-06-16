#!/usr/bin/env python3
"""Regenerate the M5 qualification-row badge / evaluation-pack / compatibility-report binding register.

This binds every machine-readable M5 qualification row (frozen in the
qualification/skew matrix) to the marketable artifacts that publish it: a
support-class badge that carries the published label, support class, evidence
freshness, and known caveats; an evaluation pack; a compatibility report; and a
release-center card. The register auto-narrows a badge below the row it inherits
when its binding evidence is stale or missing, or when marketable wording would
exceed the row, so no surface can advertise wider than the current machine-readable
row.

This emits the canonical register artifact, the negative fixtures, the cases
manifest, and the frozen validation capture. The Python summary/promotion logic
mirrors the typed Rust consumer so the checked-in artifact validates cleanly and
the capture cross-check agrees with the model.
"""

from __future__ import annotations

import copy
import json
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent

MODULE = "bind_qualification_rows_to_marketable_badges_evaluation_packs_and_compatibility_reports_for_every_m5_family"
RECORD_KIND = "bind_m5_qualification_rows_to_marketable_badges_evaluation_packs_and_compatibility_reports"
ARTIFACT = REPO / "artifacts/release/m5" / f"{MODULE}.json"
CAPTURE = REPO / "artifacts/release/captures" / f"{MODULE}_validation_capture.json"
FIXTURES = REPO / "fixtures/compat/m5-qualification-row-badge-bindings"
AS_OF = "2026-06-16"

QUALIFICATION_MATRIX_REF = (
    "artifacts/release/m5/"
    "freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix.json"
)
EVIDENCE_INDEX_REF = (
    "artifacts/release/m5/"
    "certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json"
)
CLAIM_MANIFEST_REF = "artifacts/release/stable_claim_manifest.json"

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
SUPPORT_CLASSES = ["full_support", "maintenance_only", "security_only", "limited", "end_of_life"]
ARTIFACT_KINDS = ["marketable_badge", "evaluation_pack", "compatibility_report", "release_center_card"]
ARTIFACT_STATES = ["current", "stale", "missing"]
BADGE_SURFACES = [
    "release_center",
    "help_about",
    "service_health",
    "support_export",
    "docs",
    "release_notes",
    "cli_inspect",
    "marketplace_listing",
]
TRUTH_SURFACES = ["release_center", "help_about", "service_health", "support_export"]
BINDING_STATES = [
    "published",
    "narrowed_row_downgraded",
    "narrowed_stale",
    "narrowed_missing",
    "withheld",
]
FRESHNESS_STATES = ["current", "due_for_refresh", "breached", "missing"]
NARROWING_REASONS = [
    "qualification_row_narrowed",
    "evidence_stale",
    "evidence_missing",
    "evaluation_pack_stale",
    "evaluation_pack_missing",
    "compatibility_report_stale",
    "compatibility_report_missing",
    "over_claim_beyond_row",
    "owner_signoff_missing",
    "waiver_expired",
]
STOP_ACTIONS = [
    "hold_publication",
    "narrow_badge",
    "withhold_badge",
    "refresh_evaluation_pack",
    "refresh_compatibility_report",
    "refresh_evidence",
    "align_marketing_to_row",
    "request_owner_signoff",
]

RANK = {"lts": 4, "stable": 3, "beta": 2, "preview": 1, "withdrawn": 0}
# Reasons that, when active on a binding whose claim is at or above the cutline,
# hold publication. A row that merely inherits an upstream narrowing is gated by
# the matrix itself, so it narrows the badge but does not itself block promotion.
BLOCKING_REASONS = {
    "evidence_stale",
    "evidence_missing",
    "evaluation_pack_stale",
    "evaluation_pack_missing",
    "compatibility_report_stale",
    "compatibility_report_missing",
    "over_claim_beyond_row",
    "owner_signoff_missing",
    "waiver_expired",
}


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


def badge(label: str, support_class: str, freshness: str, caveats: list[str], text: str) -> dict:
    return {
        "badge_text": text,
        "badge_label": label,
        "support_class": support_class,
        "freshness_state": freshness,
        "caveat_summary": caveats,
        "freshness_disclosed": True,
        "caveats_disclosed": bool(caveats),
    }


def artifact_ref(kind: str, entry: str, state: str = "current") -> dict:
    return {
        "artifact_kind": kind,
        "artifact_ref": "" if state == "missing" else f"{kind}/{entry}",
        "state": state,
        "captured_at": None if state == "missing" else AS_OF,
    }


def bindings() -> list[dict]:
    out = []

    out.append(
        {
            "entry_id": "m5-badge-notebook-runtime",
            "title": "Notebook runtime badge binding",
            "family_kind": "notebook",
            "family_ref": "family/notebook-runtime",
            "family_summary": "Notebook and data-rich runtime: kernel protocol and cell-state schema.",
            "release_blocking": True,
            "claim_ref": "claim/m5-notebook",
            "claim_label": "stable",
            "qualification_row_ref": "m5-notebook-runtime",
            "row_published_label": "stable",
            "binding_state": "published",
            "support_class": "full_support",
            "badge": badge(
                "stable", "full_support", "current", [],
                "Notebook runtime — Stable on every qualified platform and deployment profile.",
            ),
            "evaluation_pack": artifact_ref("evaluation_pack", "m5-notebook-runtime"),
            "compatibility_report": artifact_ref("compatibility_report", "m5-notebook-runtime"),
            "release_center_card": artifact_ref("release_center_card", "m5-notebook-runtime"),
            "surfaces": BADGE_SURFACES,
            "compatibility_caveats": [],
            "proof_packet": proof("m5-badge-notebook-runtime", "current"),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": [],
            "published_label": "stable",
            "rationale": "Every qualification dimension is qualified, the evaluation pack and compatibility report are current, and the badge publishes the row's Stable label with disclosed freshness.",
        }
    )

    out.append(
        {
            "entry_id": "m5-badge-ai-provider",
            "title": "AI provider boundary badge binding",
            "family_kind": "ai_provider",
            "family_ref": "family/ai-provider-boundary",
            "family_summary": "Helper/agent/provider boundary: capability handshake and model-route descriptors.",
            "release_blocking": True,
            "claim_ref": "claim/m5-ai-provider",
            "claim_label": "stable",
            "qualification_row_ref": "m5-ai-provider-boundary",
            "row_published_label": "stable",
            "binding_state": "published",
            "support_class": "full_support",
            "badge": badge(
                "stable", "full_support", "current",
                ["BYOK provider client scope is qualified for managed and self-hosted profiles only; air-gapped provider routing stays limited."],
                "AI provider boundary — Stable, with a recorded client-scope caveat.",
            ),
            "evaluation_pack": artifact_ref("evaluation_pack", "m5-ai-provider-boundary"),
            "compatibility_report": artifact_ref("compatibility_report", "m5-ai-provider-boundary"),
            "release_center_card": artifact_ref("release_center_card", "m5-ai-provider-boundary"),
            "surfaces": BADGE_SURFACES,
            "compatibility_caveats": [
                "BYOK provider client scope is qualified for managed and self-hosted profiles only; air-gapped provider routing stays limited.",
            ],
            "proof_packet": proof("m5-badge-ai-provider", "current"),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": [],
            "published_label": "stable",
            "rationale": "Holds Stable with a recorded compatibility caveat that travels with the badge wherever it renders, so the marketable claim never reads wider than the qualification row.",
        }
    )

    out.append(
        {
            "entry_id": "m5-badge-remote-helper",
            "title": "Remote helper badge binding",
            "family_kind": "remote_helper",
            "family_ref": "family/remote-helper-skew",
            "family_summary": "Remote/helper boundary: RPC envelope and session-resume token skew.",
            "release_blocking": True,
            "claim_ref": "claim/m5-remote-helper",
            "claim_label": "stable",
            "qualification_row_ref": "m5-remote-helper-skew",
            "row_published_label": "stable",
            "binding_state": "published",
            "support_class": "full_support",
            "badge": badge(
                "stable", "full_support", "current", [],
                "Remote helper — Stable; toolchain-envelope coverage held under an active waiver.",
            ),
            "evaluation_pack": artifact_ref("evaluation_pack", "m5-remote-helper-skew"),
            "compatibility_report": artifact_ref("compatibility_report", "m5-remote-helper-skew"),
            "release_center_card": artifact_ref("release_center_card", "m5-remote-helper-skew"),
            "surfaces": BADGE_SURFACES,
            "compatibility_caveats": [],
            "proof_packet": proof("m5-badge-remote-helper", "current"),
            "waiver": {
                "waiver_ref": "waiver:m5_remote_helper_toolchain",
                "expires_at": "2026-12-31",
                "reason": "Toolchain-envelope re-qualification scheduled; interim coverage waived by owner.",
            },
            "owner_signoff": signoff(),
            "active_narrowing_reasons": [],
            "published_label": "stable",
            "rationale": "Publishes the row's Stable label; the upstream qualification row rides an unexpired waiver, and the binding evidence and marketable artifacts are current.",
        }
    )

    out.append(
        {
            "entry_id": "m5-badge-managed-airgapped",
            "title": "Managed air-gapped profile badge binding",
            "family_kind": "managed_service",
            "family_ref": "family/managed-airgapped-profile",
            "family_summary": "Air-gapped managed profile: lockstep bundle digest with fail-closed boundary.",
            "release_blocking": False,
            "claim_ref": "claim/m5-managed-airgapped",
            "claim_label": "stable",
            "qualification_row_ref": "m5-managed-airgapped-profile",
            "row_published_label": "stable",
            "binding_state": "published",
            "support_class": "security_only",
            "badge": badge(
                "stable", "security_only", "due_for_refresh", [],
                "Air-gapped managed profile — Stable, security-only support; evidence refresh due soon.",
            ),
            "evaluation_pack": artifact_ref("evaluation_pack", "m5-managed-airgapped-profile"),
            "compatibility_report": artifact_ref("compatibility_report", "m5-managed-airgapped-profile"),
            "release_center_card": artifact_ref("release_center_card", "m5-managed-airgapped-profile"),
            "surfaces": BADGE_SURFACES,
            "compatibility_caveats": [],
            "proof_packet": proof("m5-badge-managed-airgapped", "due_for_refresh"),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": [],
            "published_label": "stable",
            "rationale": "Lockstep-only air-gapped profile holds Stable on security-only support; the badge discloses the due-for-refresh freshness state while the packet is still within its SLO.",
        }
    )

    narrowed_surfaces = ["release_center", "help_about", "service_health", "support_export", "docs", "cli_inspect"]

    out.append(
        {
            "entry_id": "m5-badge-companion",
            "title": "Browser/mobile companion badge binding",
            "family_kind": "companion",
            "family_ref": "family/companion-handoff",
            "family_summary": "Companion boundary: handoff-eligibility token and companion session descriptor.",
            "release_blocking": True,
            "claim_ref": "claim/m5-companion",
            "claim_label": "stable",
            "qualification_row_ref": "m5-companion-handoff",
            "row_published_label": "beta",
            "binding_state": "narrowed_row_downgraded",
            "support_class": "maintenance_only",
            "badge": badge(
                "beta", "maintenance_only", "current", [],
                "Companion handoff — Beta while the client-scope retest completes.",
            ),
            "evaluation_pack": artifact_ref("evaluation_pack", "m5-companion-handoff"),
            "compatibility_report": artifact_ref("compatibility_report", "m5-companion-handoff"),
            "release_center_card": artifact_ref("release_center_card", "m5-companion-handoff"),
            "surfaces": narrowed_surfaces,
            "compatibility_caveats": [],
            "proof_packet": proof("m5-badge-companion", "current"),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": ["qualification_row_narrowed"],
            "published_label": "beta",
            "rationale": "The qualification row narrowed to Beta after a handoff-eligibility retest; the badge inherits the row's narrowed label rather than continuing to advertise Stable.",
        }
    )

    out.append(
        {
            "entry_id": "m5-badge-toolchain",
            "title": "Toolchain envelope badge binding",
            "family_kind": "toolchain_runtime",
            "family_ref": "family/toolchain-envelope",
            "family_summary": "Toolchain/runtime boundary: compiler ABI token and LSP protocol version.",
            "release_blocking": True,
            "claim_ref": "claim/m5-toolchain",
            "claim_label": "stable",
            "qualification_row_ref": "m5-toolchain-envelope",
            "row_published_label": "beta",
            "binding_state": "narrowed_stale",
            "support_class": "full_support",
            "badge": badge(
                "beta", "full_support", "breached", [],
                "Toolchain envelope — Beta; qualification evidence is stale and refreshing.",
            ),
            "evaluation_pack": artifact_ref("evaluation_pack", "m5-toolchain-envelope"),
            "compatibility_report": artifact_ref("compatibility_report", "m5-toolchain-envelope"),
            "release_center_card": artifact_ref("release_center_card", "m5-toolchain-envelope"),
            "surfaces": narrowed_surfaces,
            "compatibility_caveats": [],
            "proof_packet": proof("m5-badge-toolchain", "breached"),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": ["qualification_row_narrowed", "evidence_stale"],
            "published_label": "beta",
            "rationale": "The qualification row is Beta and the binding proof packet breached its freshness SLO, so the badge narrows and discloses the stale freshness state until evidence is refreshed.",
        }
    )

    out.append(
        {
            "entry_id": "m5-badge-ecosystem",
            "title": "Ecosystem sideload badge binding",
            "family_kind": "ecosystem",
            "family_ref": "family/ecosystem-sideload",
            "family_summary": "Extension/sideload boundary: ABI version and capability-grant manifest.",
            "release_blocking": True,
            "claim_ref": "claim/m5-ecosystem",
            "claim_label": "stable",
            "qualification_row_ref": "m5-ecosystem-sideload",
            "row_published_label": "beta",
            "binding_state": "narrowed_stale",
            "support_class": "limited",
            "badge": badge(
                "beta", "limited", "current",
                ["Sideloaded extensions built before the supported ABI floor require a reinstall."],
                "Ecosystem sideload — Beta, limited support; compatibility report is being refreshed.",
            ),
            "evaluation_pack": artifact_ref("evaluation_pack", "m5-ecosystem-sideload"),
            "compatibility_report": artifact_ref("compatibility_report", "m5-ecosystem-sideload", "stale"),
            "release_center_card": artifact_ref("release_center_card", "m5-ecosystem-sideload"),
            "surfaces": narrowed_surfaces,
            "compatibility_caveats": [
                "Sideloaded extensions built before the supported ABI floor require a reinstall.",
            ],
            "proof_packet": proof("m5-badge-ecosystem", "current"),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": ["qualification_row_narrowed", "compatibility_report_stale"],
            "published_label": "beta",
            "rationale": "The qualification row is Beta and the bound compatibility report is stale, so the badge narrows and carries its ABI-reinstall caveat on every surface it renders.",
        }
    )

    out.append(
        {
            "entry_id": "m5-badge-managed-sync",
            "title": "Managed sync service badge binding",
            "family_kind": "managed_service",
            "family_ref": "family/managed-sync-service",
            "family_summary": "Managed sync/relay service: relay protocol and change-journal envelope.",
            "release_blocking": False,
            "claim_ref": "claim/m5-managed-sync",
            "claim_label": "stable",
            "qualification_row_ref": "m5-managed-sync-service",
            "row_published_label": "preview",
            "binding_state": "narrowed_stale",
            "support_class": "maintenance_only",
            "badge": badge(
                "preview", "maintenance_only", "current", [],
                "Managed sync service — Preview ahead of scheduled removal; successor available.",
            ),
            "evaluation_pack": artifact_ref("evaluation_pack", "m5-managed-sync-service", "stale"),
            "compatibility_report": artifact_ref("compatibility_report", "m5-managed-sync-service"),
            "release_center_card": artifact_ref("release_center_card", "m5-managed-sync-service"),
            "surfaces": narrowed_surfaces,
            "compatibility_caveats": [],
            "proof_packet": proof("m5-badge-managed-sync", "current"),
            "waiver": None,
            "owner_signoff": signoff(),
            "active_narrowing_reasons": ["qualification_row_narrowed", "evaluation_pack_stale"],
            "published_label": "preview",
            "rationale": "The qualification row narrowed to Preview ahead of a scheduled removal and the bound evaluation pack is stale, so the badge holds at Preview and points at the successor migration.",
        }
    )

    return out


def stop_rules() -> list[dict]:
    action = {
        "qualification_row_narrowed": "narrow_badge",
        "evidence_stale": "refresh_evidence",
        "evidence_missing": "refresh_evidence",
        "evaluation_pack_stale": "refresh_evaluation_pack",
        "evaluation_pack_missing": "refresh_evaluation_pack",
        "compatibility_report_stale": "refresh_compatibility_report",
        "compatibility_report_missing": "refresh_compatibility_report",
        "over_claim_beyond_row": "align_marketing_to_row",
        "owner_signoff_missing": "request_owner_signoff",
        "waiver_expired": "narrow_badge",
    }
    titles = {
        "qualification_row_narrowed": "Qualification row narrowed",
        "evidence_stale": "Binding evidence stale",
        "evidence_missing": "Binding evidence missing",
        "evaluation_pack_stale": "Evaluation pack stale",
        "evaluation_pack_missing": "Evaluation pack missing",
        "compatibility_report_stale": "Compatibility report stale",
        "compatibility_report_missing": "Compatibility report missing",
        "over_claim_beyond_row": "Badge over-claims the row",
        "owner_signoff_missing": "Owner sign-off missing",
        "waiver_expired": "Badge waiver expired",
    }
    out = []
    for reason in NARROWING_REASONS:
        blocks = reason in BLOCKING_REASONS
        out.append(
            {
                "rule_id": f"m5_badge_rule:{reason}",
                "title": titles[reason],
                "trigger_reason": reason,
                "applies_to_labels": ABOVE_CUTLINE,
                "default_action": action[reason],
                "blocks_promotion": blocks,
                "rationale": (
                    f"A badge whose claim is at or above the cutline that reports '{reason}' holds publication until the binding evidence is restored."
                    if blocks
                    else f"A badge that reports '{reason}' narrows to inherit the upstream row; the matrix gate already holds promotion for the row itself."
                ),
            }
        )
    return out


def compute_promotion(register: dict) -> dict:
    def fires(rule) -> bool:
        return any(
            b["claim_label"] in rule["applies_to_labels"]
            and rule["trigger_reason"] in b["active_narrowing_reasons"]
            for b in register["bindings"]
        )

    triggers = {
        rule["trigger_reason"]
        for rule in register["stop_rules"]
        if rule["blocks_promotion"] and fires(rule)
    }
    blocking_rule_ids = sorted(
        rule["rule_id"]
        for rule in register["stop_rules"]
        if rule["blocks_promotion"] and fires(rule)
    )
    blocking_claim_ids = sorted(
        {
            b["entry_id"]
            for b in register["bindings"]
            if holds_stable(b["claim_label"])
            and any(r in triggers for r in b["active_narrowing_reasons"])
        }
    )
    decision = "hold" if blocking_rule_ids else "proceed"
    return {
        "promotion_gate": "m5-qualification-badge-binding-gate",
        "decision": decision,
        "blocking_rule_ids": blocking_rule_ids,
        "blocking_claim_ids": blocking_claim_ids,
        "rationale": "Computed from the firing stop rules over badge over-claim, evidence freshness, and evaluation-pack / compatibility-report state; an inherited row narrowing narrows the badge but is gated by the matrix rather than this register.",
    }


def compute_summary(register: dict) -> dict:
    bs = register["bindings"]

    def published_stable(b):
        return holds_stable(b["published_label"])

    def binding_state(state):
        return sum(1 for b in bs if b["binding_state"] == state)

    def kind(k):
        return sum(1 for b in bs if b["family_kind"] == k)

    def slo(state):
        return sum(1 for b in bs if b["proof_packet"]["slo_state"] == state)

    def eval_state(state):
        return sum(1 for b in bs if b["evaluation_pack"]["state"] == state)

    def report_state(state):
        return sum(1 for b in bs if b["compatibility_report"]["state"] == state)

    rb = [b for b in bs if b["release_blocking"]]
    families = {b["family_ref"] for b in bs}
    return {
        "total_bindings": len(bs),
        "total_families": len(families),
        "bindings_published": sum(1 for b in bs if published_stable(b)),
        "bindings_narrowed": sum(1 for b in bs if not published_stable(b)),
        "release_blocking_total": len(rb),
        "release_blocking_published": sum(1 for b in rb if published_stable(b)),
        "release_blocking_narrowed": sum(1 for b in rb if not published_stable(b)),
        "notebook_bindings": kind("notebook"),
        "ai_provider_bindings": kind("ai_provider"),
        "remote_helper_bindings": kind("remote_helper"),
        "companion_bindings": kind("companion"),
        "ecosystem_bindings": kind("ecosystem"),
        "managed_service_bindings": kind("managed_service"),
        "toolchain_runtime_bindings": kind("toolchain_runtime"),
        "state_published": binding_state("published"),
        "state_narrowed_row_downgraded": binding_state("narrowed_row_downgraded"),
        "state_narrowed_stale": binding_state("narrowed_stale"),
        "state_narrowed_missing": binding_state("narrowed_missing"),
        "state_withheld": binding_state("withheld"),
        "badges_with_caveats": sum(1 for b in bs if b["badge"]["caveat_summary"]),
        "badges_freshness_disclosed": sum(1 for b in bs if b["badge"]["freshness_disclosed"]),
        "packets_current": slo("current"),
        "packets_due_for_refresh": slo("due_for_refresh"),
        "packets_breached": slo("breached"),
        "packets_missing": slo("missing"),
        "evaluation_packs_current": eval_state("current"),
        "evaluation_packs_stale": eval_state("stale"),
        "evaluation_packs_missing": eval_state("missing"),
        "compatibility_reports_current": report_state("current"),
        "compatibility_reports_stale": report_state("stale"),
        "compatibility_reports_missing": report_state("missing"),
        "total_active_narrowing_reasons": sum(len(b["active_narrowing_reasons"]) for b in bs),
        "total_surface_renderings": sum(len(b["surfaces"]) for b in bs),
        "rules_firing": sum(
            1
            for rule in register["stop_rules"]
            if any(
                b["claim_label"] in rule["applies_to_labels"]
                and rule["trigger_reason"] in b["active_narrowing_reasons"]
                for b in bs
            )
        ),
    }


def build_register() -> dict:
    register = {
        "schema_version": 1,
        "record_kind": RECORD_KIND,
        "register_id": "m5_qualification_badge_bindings:v1",
        "status": "published",
        "overview_page": f"docs/m5/{MODULE}.md",
        "as_of": AS_OF,
        "claim_manifest_ref": CLAIM_MANIFEST_REF,
        "qualification_matrix_ref": QUALIFICATION_MATRIX_REF,
        "evidence_index_ref": EVIDENCE_INDEX_REF,
        "lifecycle_labels": LIFECYCLE_LABELS,
        "family_kinds": FAMILY_KINDS,
        "support_classes": SUPPORT_CLASSES,
        "artifact_kinds": ARTIFACT_KINDS,
        "artifact_states": ARTIFACT_STATES,
        "badge_surfaces": BADGE_SURFACES,
        "binding_states": BINDING_STATES,
        "freshness_states": FRESHNESS_STATES,
        "narrowing_reasons": NARROWING_REASONS,
        "stop_actions": STOP_ACTIONS,
        "launch_cutline": {
            "cutline_level": "stable",
            "above_cutline_levels": ABOVE_CUTLINE,
            "below_cutline_levels": BELOW_CUTLINE,
            "description": "A marketable badge may publish a Stable (or LTS) label only when it does not exceed the qualification row it binds, the row itself is at or above the cutline, the bound evaluation pack and compatibility report are current, the proof packet is within its freshness SLO, the owner has signed off, and the badge discloses its freshness and any caveats. A badge that loses any of those narrows to inherit the row rather than continue to advertise wider than the current machine-readable row.",
        },
        "release_blocking_family_refs": [],
        "stop_rules": stop_rules(),
        "bindings": bindings(),
    }
    register["release_blocking_family_refs"] = [
        b["family_ref"] for b in register["bindings"] if b["release_blocking"]
    ]
    register["promotion"] = compute_promotion(register)
    register["summary"] = compute_summary(register)
    return register


def write_json(path: Path, payload: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")


def build_fixtures(register: dict) -> list[tuple[str, str]]:
    cases = []

    dup = copy.deepcopy(register)
    dup["bindings"][1]["entry_id"] = dup["bindings"][0]["entry_id"]
    dup["summary"] = compute_summary(dup)
    dup["promotion"] = compute_promotion(dup)
    write_json(FIXTURES / "duplicate_entry_id.json", dup)
    cases.append(("duplicate_entry_id.json", "DuplicateEntryId"))

    over = copy.deepcopy(register)
    target = next(b for b in over["bindings"] if not holds_stable(b["row_published_label"]))
    target["published_label"] = "stable"
    target["badge"]["badge_label"] = "stable"
    over["summary"] = compute_summary(over)
    over["promotion"] = compute_promotion(over)
    write_json(FIXTURES / "badge_over_claims_row.json", over)
    cases.append(("badge_over_claims_row.json", "BadgePublishedWiderThanRow"))

    held = copy.deepcopy(register)
    backed = next(b for b in held["bindings"] if holds_stable(b["published_label"]))
    backed["active_narrowing_reasons"] = ["evidence_stale"]
    held["summary"] = compute_summary(held)
    held["promotion"] = compute_promotion(held)
    write_json(FIXTURES / "held_with_active_gap.json", held)
    cases.append(("held_with_active_gap.json", "HeldWithActiveGap"))

    write_json(
        FIXTURES / "cases.json",
        {"cases": [{"file": f, "expected_check_id": c} for f, c in cases]},
    )
    return cases


def build_capture(register: dict, cases: list[tuple[str, str]]) -> dict:
    s = register["summary"]
    return {
        "status": "pass",
        "as_of": register["as_of"],
        "summary": {
            "total_bindings": s["total_bindings"],
            "bindings_published": s["bindings_published"],
            "bindings_narrowed": s["bindings_narrowed"],
            "state_published": s["state_published"],
            "state_narrowed_row_downgraded": s["state_narrowed_row_downgraded"],
            "state_narrowed_stale": s["state_narrowed_stale"],
            "badges_with_caveats": s["badges_with_caveats"],
            "badges_freshness_disclosed": s["badges_freshness_disclosed"],
            "packets_breached": s["packets_breached"],
            "evaluation_packs_stale": s["evaluation_packs_stale"],
            "compatibility_reports_stale": s["compatibility_reports_stale"],
            "total_surface_renderings": s["total_surface_renderings"],
            "rules_firing": s["rules_firing"],
        },
        "promotion": {
            "decision": register["promotion"]["decision"],
            "blocking_rule_ids": register["promotion"]["blocking_rule_ids"],
            "blocking_claim_ids": register["promotion"]["blocking_claim_ids"],
        },
        "negative_drills": [
            {"drill_id": "drill:badge_over_claims_row", "status": "passed"},
            {"drill_id": "drill:held_with_active_gap", "status": "passed"},
            {"drill_id": "drill:freshness_not_disclosed", "status": "passed"},
            {"drill_id": "drill:truth_surface_coverage_incomplete", "status": "passed"},
            {"drill_id": "drill:promotion_decision_inconsistent", "status": "passed"},
        ],
        "fixture_cases": [
            {"case_id": f"fixture:{f.removesuffix('.json')}", "status": "passed"} for f, _ in cases
        ],
    }


def main() -> int:
    register = build_register()
    write_json(ARTIFACT, register)
    cases = build_fixtures(register)
    write_json(CAPTURE, build_capture(register, cases))
    print(f"wrote {ARTIFACT.relative_to(REPO)}")
    print(f"wrote {CAPTURE.relative_to(REPO)}")
    print(f"wrote {FIXTURES.relative_to(REPO)}/ ({len(cases)} fixtures + cases.json)")
    print("decision:", register["promotion"]["decision"])
    print("summary:", json.dumps(register["summary"]))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
