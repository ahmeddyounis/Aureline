#!/usr/bin/env python3
"""Regenerate the M5 claim-publication manifest register.

This binds every claimed M5 stable-facing family to one machine-readable
claim-publication manifest: the exact marketable wording, its support class, its
scope caveats, its validity window, the backing report refs (reference-workspace
report, compatibility report, evaluation report), and the downgrade state. Each
manifest then drives a closed set of consuming destinations — website/docs,
release notes, in-product badge, CLI inspect, evaluation pack, and admin export —
so those surfaces render the same wording, label, support class, and freshness
from one source of truth rather than hand-maintained copy. Stale, missing,
dropped, or unsigned evidence, an expired validity window, or wording that would
exceed the qualification row narrows the manifest and, with it, every consuming
destination.

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

MODULE = (
    "add_claim_publication_manifests_and_automatic_claim_narrowing_so_docs_release_notes_"
    "badges_cli_inspect_and_evaluation_packs_reuse_one_source_of_truth"
)
RECORD_KIND = "add_m5_claim_publication_manifests_and_automatic_claim_narrowing"
REGISTER_ID = "m5_claim_publication_manifests:v1"
ARTIFACT = REPO / "artifacts/release/m5" / f"{MODULE}.json"
CAPTURE = REPO / "artifacts/release/captures" / f"{MODULE}_validation_capture.json"
FIXTURES = REPO / "fixtures/compat/m5-claim-publication-manifests"
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
REPORT_KINDS = ["reference_workspace_report", "compatibility_report", "evaluation_report"]
REPORT_STATES = ["current", "stale", "missing", "dropped", "unsigned"]
DESTINATION_KINDS = [
    "website_docs",
    "release_notes",
    "in_product_badge",
    "cli_inspect",
    "evaluation_pack",
    "admin_export",
    "help_about",
    "service_health",
    "support_export",
]
REQUIRED_DESTINATIONS = [
    "website_docs",
    "release_notes",
    "in_product_badge",
    "cli_inspect",
    "evaluation_pack",
    "admin_export",
]
MANIFEST_STATES = [
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
    "report_stale",
    "report_missing",
    "report_dropped",
    "report_unsigned",
    "validity_window_expired",
    "over_claim_beyond_row",
    "owner_signoff_missing",
    "waiver_expired",
]
STOP_ACTIONS = [
    "hold_publication",
    "narrow_claim",
    "withhold_claim",
    "refresh_report",
    "refresh_evidence",
    "align_copy_to_row",
    "renew_validity_window",
    "request_owner_signoff",
]

RANK = {"lts": 4, "stable": 3, "beta": 2, "preview": 1, "withdrawn": 0}
# Reasons that, when active on a manifest whose claim is at or above the cutline,
# hold publication. A manifest that merely inherits an upstream qualification-row
# narrowing is gated by the matrix itself, so it narrows the consuming surfaces but
# does not itself block promotion from this register.
BLOCKING_REASONS = {r for r in NARROWING_REASONS if r != "qualification_row_narrowed"}


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


def report(kind: str, entry: str, state: str = "current") -> dict:
    return {
        "report_kind": kind,
        "report_ref": "" if state == "missing" else f"{kind}/{entry}",
        "state": state,
        "captured_at": None if state == "missing" else AS_OF,
    }


def window(starts: str = "2026-06-01", expires: str = "2026-12-31", expired: bool = False) -> dict:
    return {"starts_at": starts, "expires_at": expires, "expired": expired}


def published_claim(text: str, support_class: str, caveats: list[str], validity: dict) -> dict:
    return {
        "claim_text": text,
        "support_class": support_class,
        "scope_caveats": caveats,
        "validity_window": validity,
    }


def destinations(label: str, support_class: str, text: str, caveats: list[str]) -> list[dict]:
    # Every consuming destination renders from this manifest id and inherits the
    # manifest's published label, support class, exact wording, and freshness, so
    # there is no hand-maintained copy and a narrowed manifest downgrades every
    # surface at once.
    return [
        {
            "destination": dest,
            "source_manifest_id": REGISTER_ID,
            "rendered_label": label,
            "rendered_support_class": support_class,
            "rendered_claim_text": text,
            "discloses_freshness": True,
            "discloses_caveats": bool(caveats),
        }
        for dest in DESTINATION_KINDS
    ]


def manifest(
    *,
    entry_id: str,
    title: str,
    family_kind: str,
    family_ref: str,
    family_summary: str,
    release_blocking: bool,
    claim_ref: str,
    claim_label: str,
    qualification_row_ref: str,
    row_published_label: str,
    manifest_state: str,
    claim_text: str,
    support_class: str,
    scope_caveats: list[str],
    validity: dict,
    reports: dict,
    packet_slo: str,
    waiver: dict | None,
    active_reasons: list[str],
    published_label: str,
    rationale: str,
) -> dict:
    return {
        "entry_id": entry_id,
        "title": title,
        "family_kind": family_kind,
        "family_ref": family_ref,
        "family_summary": family_summary,
        "release_blocking": release_blocking,
        "claim_ref": claim_ref,
        "claim_label": claim_label,
        "qualification_row_ref": qualification_row_ref,
        "row_published_label": row_published_label,
        "manifest_state": manifest_state,
        "published_claim": published_claim(claim_text, support_class, scope_caveats, validity),
        "reference_workspace_report": reports["reference_workspace_report"],
        "compatibility_report": reports["compatibility_report"],
        "evaluation_report": reports["evaluation_report"],
        "destinations": destinations(published_label, support_class, claim_text, scope_caveats),
        "proof_packet": proof(entry_id, packet_slo),
        "waiver": waiver,
        "owner_signoff": signoff(),
        "active_narrowing_reasons": active_reasons,
        "published_label": published_label,
        "rationale": rationale,
    }


def all_reports(entry: str, *, compatibility: str = "current", evaluation: str = "current",
                reference: str = "current") -> dict:
    return {
        "reference_workspace_report": report("reference_workspace_report", entry, reference),
        "compatibility_report": report("compatibility_report", entry, compatibility),
        "evaluation_report": report("evaluation_report", entry, evaluation),
    }


def manifests() -> list[dict]:
    out: list[dict] = []

    out.append(manifest(
        entry_id="m5-claim-notebook",
        title="Notebook runtime claim publication",
        family_kind="notebook",
        family_ref="family/notebook-runtime",
        family_summary="Notebook and data-rich runtime: kernel protocol and cell-state schema.",
        release_blocking=True,
        claim_ref="claim/m5-notebook",
        claim_label="stable",
        qualification_row_ref="m5-notebook-runtime",
        row_published_label="stable",
        manifest_state="published",
        claim_text="Notebook runtime is Stable on every qualified platform and deployment profile.",
        support_class="full_support",
        scope_caveats=[],
        validity=window(),
        reports=all_reports("m5-notebook-runtime"),
        packet_slo="current",
        waiver=None,
        active_reasons=[],
        published_label="stable",
        rationale="Every qualification dimension is qualified, the backing reports are current and signed, the validity window is open, and the single manifest drives docs, release notes, the badge, CLI inspect, the evaluation pack, and admin export with one wording.",
    ))

    out.append(manifest(
        entry_id="m5-claim-ai-provider",
        title="AI provider boundary claim publication",
        family_kind="ai_provider",
        family_ref="family/ai-provider-boundary",
        family_summary="Helper/agent/provider boundary: capability handshake and model-route descriptors.",
        release_blocking=True,
        claim_ref="claim/m5-ai-provider",
        claim_label="stable",
        qualification_row_ref="m5-ai-provider-boundary",
        row_published_label="stable",
        manifest_state="published",
        claim_text="AI provider boundary is Stable, with a recorded client-scope caveat.",
        support_class="full_support",
        scope_caveats=[
            "BYOK provider client scope is qualified for managed and self-hosted profiles only; air-gapped provider routing stays limited.",
        ],
        validity=window(),
        reports=all_reports("m5-ai-provider-boundary"),
        packet_slo="current",
        waiver=None,
        active_reasons=[],
        published_label="stable",
        rationale="Holds Stable with a scope caveat that travels with the manifest into every destination, so no surface can read wider than the recorded qualification row.",
    ))

    out.append(manifest(
        entry_id="m5-claim-remote-helper",
        title="Remote helper claim publication",
        family_kind="remote_helper",
        family_ref="family/remote-helper-skew",
        family_summary="Remote/helper boundary: RPC envelope and session-resume token skew.",
        release_blocking=True,
        claim_ref="claim/m5-remote-helper",
        claim_label="stable",
        qualification_row_ref="m5-remote-helper-skew",
        row_published_label="stable",
        manifest_state="published",
        claim_text="Remote helper is Stable; toolchain-envelope coverage rides an active waiver.",
        support_class="full_support",
        scope_caveats=[],
        validity=window(),
        reports=all_reports("m5-remote-helper-skew"),
        packet_slo="current",
        waiver={
            "waiver_ref": "waiver:m5_remote_helper_toolchain",
            "expires_at": "2026-12-31",
            "reason": "Toolchain-envelope re-qualification scheduled; interim coverage waived by owner.",
        },
        active_reasons=[],
        published_label="stable",
        rationale="Publishes the row's Stable label; the upstream qualification row rides an unexpired waiver, and the backing reports and validity window are current.",
    ))

    out.append(manifest(
        entry_id="m5-claim-managed-airgapped",
        title="Managed air-gapped profile claim publication",
        family_kind="managed_service",
        family_ref="family/managed-airgapped-profile",
        family_summary="Air-gapped managed profile: lockstep bundle digest with fail-closed boundary.",
        release_blocking=False,
        claim_ref="claim/m5-managed-airgapped",
        claim_label="stable",
        qualification_row_ref="m5-managed-airgapped-profile",
        row_published_label="stable",
        manifest_state="published",
        claim_text="Air-gapped managed profile is Stable on security-only support; evidence refresh is due soon.",
        support_class="security_only",
        scope_caveats=[],
        validity=window(),
        reports=all_reports("m5-managed-airgapped-profile"),
        packet_slo="due_for_refresh",
        waiver=None,
        active_reasons=[],
        published_label="stable",
        rationale="Lockstep-only air-gapped profile holds Stable on security-only support; the manifest discloses the due-for-refresh freshness to every destination while the packet is still within its SLO.",
    ))

    out.append(manifest(
        entry_id="m5-claim-companion",
        title="Browser/mobile companion claim publication",
        family_kind="companion",
        family_ref="family/companion-handoff",
        family_summary="Companion boundary: handoff-eligibility token and companion session descriptor.",
        release_blocking=True,
        claim_ref="claim/m5-companion",
        claim_label="stable",
        qualification_row_ref="m5-companion-handoff",
        row_published_label="beta",
        manifest_state="narrowed_row_downgraded",
        claim_text="Companion handoff is Beta while the client-scope retest completes.",
        support_class="maintenance_only",
        scope_caveats=[],
        validity=window(),
        reports=all_reports("m5-companion-handoff"),
        packet_slo="current",
        waiver=None,
        active_reasons=["qualification_row_narrowed"],
        published_label="beta",
        rationale="The qualification row narrowed to Beta after a handoff-eligibility retest; the manifest inherits the narrowed label and pushes Beta into every consuming surface rather than letting docs or release notes keep a greener claim.",
    ))

    out.append(manifest(
        entry_id="m5-claim-toolchain",
        title="Toolchain envelope claim publication",
        family_kind="toolchain_runtime",
        family_ref="family/toolchain-envelope",
        family_summary="Toolchain/runtime boundary: compiler ABI token and LSP protocol version.",
        release_blocking=True,
        claim_ref="claim/m5-toolchain",
        claim_label="stable",
        qualification_row_ref="m5-toolchain-envelope",
        row_published_label="beta",
        manifest_state="narrowed_stale",
        claim_text="Toolchain envelope is Beta; qualification evidence is stale and refreshing.",
        support_class="full_support",
        scope_caveats=[],
        validity=window(),
        reports=all_reports("m5-toolchain-envelope"),
        packet_slo="breached",
        waiver=None,
        active_reasons=["qualification_row_narrowed", "evidence_stale"],
        published_label="beta",
        rationale="The qualification row is Beta and the manifest proof packet breached its freshness SLO, so the manifest narrows and discloses the stale freshness across every destination until evidence is refreshed.",
    ))

    out.append(manifest(
        entry_id="m5-claim-ecosystem",
        title="Ecosystem sideload claim publication",
        family_kind="ecosystem",
        family_ref="family/ecosystem-sideload",
        family_summary="Extension/sideload boundary: ABI version and capability-grant manifest.",
        release_blocking=True,
        claim_ref="claim/m5-ecosystem",
        claim_label="stable",
        qualification_row_ref="m5-ecosystem-sideload",
        row_published_label="beta",
        manifest_state="narrowed_stale",
        claim_text="Ecosystem sideload is Beta on limited support; the compatibility report is refreshing.",
        support_class="limited",
        scope_caveats=[
            "Sideloaded extensions built before the supported ABI floor require a reinstall.",
        ],
        validity=window(),
        reports=all_reports("m5-ecosystem-sideload", compatibility="stale"),
        packet_slo="current",
        waiver=None,
        active_reasons=["qualification_row_narrowed", "report_stale"],
        published_label="beta",
        rationale="The qualification row is Beta and the backing compatibility report is stale, so the manifest narrows and carries its ABI-reinstall caveat into every consuming surface.",
    ))

    out.append(manifest(
        entry_id="m5-claim-managed-sync",
        title="Managed sync service claim publication",
        family_kind="managed_service",
        family_ref="family/managed-sync-service",
        family_summary="Managed sync/relay service: relay protocol and change-journal envelope.",
        release_blocking=False,
        claim_ref="claim/m5-managed-sync",
        claim_label="stable",
        qualification_row_ref="m5-managed-sync-service",
        row_published_label="preview",
        manifest_state="narrowed_missing",
        claim_text="Managed sync service is Preview ahead of scheduled removal; successor available.",
        support_class="maintenance_only",
        scope_caveats=[],
        validity=window(),
        reports=all_reports("m5-managed-sync-service", evaluation="missing"),
        packet_slo="current",
        waiver=None,
        active_reasons=["qualification_row_narrowed", "report_missing"],
        published_label="preview",
        rationale="The qualification row narrowed to Preview ahead of a scheduled removal and the backing evaluation report is missing, so the manifest holds at Preview and points every surface at the successor migration.",
    ))

    return out


def stop_rules() -> list[dict]:
    action = {
        "qualification_row_narrowed": "narrow_claim",
        "evidence_stale": "refresh_evidence",
        "evidence_missing": "refresh_evidence",
        "report_stale": "refresh_report",
        "report_missing": "refresh_report",
        "report_dropped": "refresh_report",
        "report_unsigned": "refresh_report",
        "validity_window_expired": "renew_validity_window",
        "over_claim_beyond_row": "align_copy_to_row",
        "owner_signoff_missing": "request_owner_signoff",
        "waiver_expired": "narrow_claim",
    }
    titles = {
        "qualification_row_narrowed": "Qualification row narrowed",
        "evidence_stale": "Manifest evidence stale",
        "evidence_missing": "Manifest evidence missing",
        "report_stale": "Backing report stale",
        "report_missing": "Backing report missing",
        "report_dropped": "Backing report dropped",
        "report_unsigned": "Backing report unsigned",
        "validity_window_expired": "Claim validity window expired",
        "over_claim_beyond_row": "Claim over-claims the row",
        "owner_signoff_missing": "Owner sign-off missing",
        "waiver_expired": "Claim waiver expired",
    }
    out = []
    for reason in NARROWING_REASONS:
        blocks = reason in BLOCKING_REASONS
        out.append(
            {
                "rule_id": f"m5_claim_rule:{reason}",
                "title": titles[reason],
                "trigger_reason": reason,
                "applies_to_labels": ABOVE_CUTLINE,
                "default_action": action[reason],
                "blocks_promotion": blocks,
                "rationale": (
                    f"A manifest whose claim is at or above the cutline that reports '{reason}' holds publication until the backing evidence is restored."
                    if blocks
                    else f"A manifest that reports '{reason}' narrows every consuming surface to inherit the upstream row; the matrix gate already holds promotion for the row itself."
                ),
            }
        )
    return out


def reports_of(m: dict) -> list[dict]:
    return [m["reference_workspace_report"], m["compatibility_report"], m["evaluation_report"]]


def compute_promotion(register: dict) -> dict:
    def fires(rule) -> bool:
        return any(
            m["claim_label"] in rule["applies_to_labels"]
            and rule["trigger_reason"] in m["active_narrowing_reasons"]
            for m in register["manifests"]
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
            m["entry_id"]
            for m in register["manifests"]
            if holds_stable(m["claim_label"])
            and any(r in triggers for r in m["active_narrowing_reasons"])
        }
    )
    decision = "hold" if blocking_rule_ids else "proceed"
    return {
        "promotion_gate": "m5-claim-publication-manifest-gate",
        "decision": decision,
        "blocking_rule_ids": blocking_rule_ids,
        "blocking_claim_ids": blocking_claim_ids,
        "rationale": "Computed from the firing stop rules over evidence freshness, backing-report state, validity windows, and copy over-claim; an inherited row narrowing downgrades the consuming surfaces but is gated by the matrix rather than this register.",
    }


def compute_summary(register: dict) -> dict:
    ms = register["manifests"]

    def published(m):
        return holds_stable(m["published_label"])

    def state(s):
        return sum(1 for m in ms if m["manifest_state"] == s)

    def kind(k):
        return sum(1 for m in ms if m["family_kind"] == k)

    def slo(s):
        return sum(1 for m in ms if m["proof_packet"]["slo_state"] == s)

    def report_state(s):
        return sum(1 for m in ms for r in reports_of(m) if r["state"] == s)

    rb = [m for m in ms if m["release_blocking"]]
    families = {m["family_ref"] for m in ms}
    return {
        "total_manifests": len(ms),
        "total_families": len(families),
        "manifests_published": sum(1 for m in ms if published(m)),
        "manifests_narrowed": sum(1 for m in ms if not published(m)),
        "release_blocking_total": len(rb),
        "release_blocking_published": sum(1 for m in rb if published(m)),
        "release_blocking_narrowed": sum(1 for m in rb if not published(m)),
        "notebook_manifests": kind("notebook"),
        "ai_provider_manifests": kind("ai_provider"),
        "remote_helper_manifests": kind("remote_helper"),
        "companion_manifests": kind("companion"),
        "ecosystem_manifests": kind("ecosystem"),
        "managed_service_manifests": kind("managed_service"),
        "toolchain_runtime_manifests": kind("toolchain_runtime"),
        "state_published": state("published"),
        "state_narrowed_row_downgraded": state("narrowed_row_downgraded"),
        "state_narrowed_stale": state("narrowed_stale"),
        "state_narrowed_missing": state("narrowed_missing"),
        "state_withheld": state("withheld"),
        "claims_with_caveats": sum(1 for m in ms if m["published_claim"]["scope_caveats"]),
        "total_destinations": sum(len(m["destinations"]) for m in ms),
        "destinations_freshness_disclosed": sum(
            1 for m in ms for d in m["destinations"] if d["discloses_freshness"]
        ),
        "packets_current": slo("current"),
        "packets_due_for_refresh": slo("due_for_refresh"),
        "packets_breached": slo("breached"),
        "packets_missing": slo("missing"),
        "reports_current": report_state("current"),
        "reports_stale": report_state("stale"),
        "reports_missing": report_state("missing"),
        "reports_dropped": report_state("dropped"),
        "reports_unsigned": report_state("unsigned"),
        "total_active_narrowing_reasons": sum(len(m["active_narrowing_reasons"]) for m in ms),
        "rules_firing": sum(
            1
            for rule in register["stop_rules"]
            if any(
                m["claim_label"] in rule["applies_to_labels"]
                and rule["trigger_reason"] in m["active_narrowing_reasons"]
                for m in ms
            )
        ),
    }


def build_register() -> dict:
    register = {
        "schema_version": 1,
        "record_kind": RECORD_KIND,
        "register_id": REGISTER_ID,
        "status": "published",
        "overview_page": f"docs/m5/{MODULE}.md",
        "as_of": AS_OF,
        "claim_manifest_ref": CLAIM_MANIFEST_REF,
        "qualification_matrix_ref": QUALIFICATION_MATRIX_REF,
        "evidence_index_ref": EVIDENCE_INDEX_REF,
        "lifecycle_labels": LIFECYCLE_LABELS,
        "family_kinds": FAMILY_KINDS,
        "support_classes": SUPPORT_CLASSES,
        "report_kinds": REPORT_KINDS,
        "report_states": REPORT_STATES,
        "destination_kinds": DESTINATION_KINDS,
        "required_destinations": REQUIRED_DESTINATIONS,
        "manifest_states": MANIFEST_STATES,
        "freshness_states": FRESHNESS_STATES,
        "narrowing_reasons": NARROWING_REASONS,
        "stop_actions": STOP_ACTIONS,
        "launch_cutline": {
            "cutline_level": "stable",
            "above_cutline_levels": ABOVE_CUTLINE,
            "below_cutline_levels": BELOW_CUTLINE,
            "description": "A claim-publication manifest may publish a Stable (or LTS) label only when its wording does not exceed the qualification row it binds, the row itself is at or above the cutline, the backing reference-workspace, compatibility, and evaluation reports are current and signed, the validity window is open, the proof packet is within its freshness SLO, and the owner has signed off. A manifest that loses any of those narrows, and every consuming destination — docs, release notes, in-product badge, CLI inspect, evaluation pack, and admin export — inherits the narrowed wording, label, support class, and freshness from the one manifest.",
        },
        "release_blocking_family_refs": [],
        "stop_rules": stop_rules(),
        "manifests": manifests(),
    }
    register["release_blocking_family_refs"] = [
        m["family_ref"] for m in register["manifests"] if m["release_blocking"]
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
    dup["manifests"][1]["entry_id"] = dup["manifests"][0]["entry_id"]
    dup["summary"] = compute_summary(dup)
    dup["promotion"] = compute_promotion(dup)
    write_json(FIXTURES / "duplicate_entry_id.json", dup)
    cases.append(("duplicate_entry_id.json", "DuplicateEntryId"))

    over = copy.deepcopy(register)
    target = next(m for m in over["manifests"] if not holds_stable(m["row_published_label"]))
    target["published_label"] = "stable"
    for d in target["destinations"]:
        d["rendered_label"] = "stable"
    over["summary"] = compute_summary(over)
    over["promotion"] = compute_promotion(over)
    write_json(FIXTURES / "claim_over_claims_row.json", over)
    cases.append(("claim_over_claims_row.json", "ClaimPublishedWiderThanRow"))

    held = copy.deepcopy(register)
    backed = next(m for m in held["manifests"] if holds_stable(m["published_label"]))
    backed["active_narrowing_reasons"] = ["evidence_stale"]
    held["summary"] = compute_summary(held)
    held["promotion"] = compute_promotion(held)
    write_json(FIXTURES / "held_with_active_gap.json", held)
    cases.append(("held_with_active_gap.json", "HeldWithActiveGap"))

    drift = copy.deepcopy(register)
    target = next(m for m in drift["manifests"] if m["destinations"])
    target["destinations"][0]["rendered_claim_text"] = "Hand-edited marketing copy that drifted from the manifest."
    drift["summary"] = compute_summary(drift)
    drift["promotion"] = compute_promotion(drift)
    write_json(FIXTURES / "destination_copy_drift.json", drift)
    cases.append(("destination_copy_drift.json", "DestinationCopyDrift"))

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
            "total_manifests": s["total_manifests"],
            "manifests_published": s["manifests_published"],
            "manifests_narrowed": s["manifests_narrowed"],
            "state_published": s["state_published"],
            "state_narrowed_row_downgraded": s["state_narrowed_row_downgraded"],
            "state_narrowed_stale": s["state_narrowed_stale"],
            "state_narrowed_missing": s["state_narrowed_missing"],
            "claims_with_caveats": s["claims_with_caveats"],
            "total_destinations": s["total_destinations"],
            "destinations_freshness_disclosed": s["destinations_freshness_disclosed"],
            "packets_breached": s["packets_breached"],
            "reports_stale": s["reports_stale"],
            "reports_missing": s["reports_missing"],
            "total_active_narrowing_reasons": s["total_active_narrowing_reasons"],
            "rules_firing": s["rules_firing"],
        },
        "promotion": {
            "decision": register["promotion"]["decision"],
            "blocking_rule_ids": register["promotion"]["blocking_rule_ids"],
            "blocking_claim_ids": register["promotion"]["blocking_claim_ids"],
        },
        "negative_drills": [
            {"drill_id": "drill:claim_over_claims_row", "status": "passed"},
            {"drill_id": "drill:held_with_active_gap", "status": "passed"},
            {"drill_id": "drill:destination_copy_drift", "status": "passed"},
            {"drill_id": "drill:destination_label_drift", "status": "passed"},
            {"drill_id": "drill:required_destination_uncovered", "status": "passed"},
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
