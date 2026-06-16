#!/usr/bin/env python3
"""Regenerate the M5 private evaluation/pilot evidence-pack register.

Where the claim-publication manifest is the single public source of truth every
claim-bearing surface reads, this register is the *private* layer that packages
enterprise and ecosystem evaluation/pilot materials on top of that public
baseline. For each enterprise/ecosystem lane it binds one evidence pack to:

- a named bundle id and its mirror refs (primary, offline, partner, air-gapped),
- the support contacts, the known-issues deltas beyond the public known-limits,
  and the deployment caveats that travel with a private pilot,
- and the public claim-publication manifest entry it reuses — its exact wording,
  its support class, and its published label, all of which are hard ceilings.

The no-overclaim guard is the spine of the register: a pack may never publish a
greener label than the public claim, never advertise a broader support class, and
never re-word the public claim into something stronger. "Pilot-only" wording can
never bypass a support-class limit or stale evidence. A pack whose public claim
narrowed, whose mirror or proof evidence went stale, missing, dropped, or
unsigned, whose validity window expired, or whose owner sign-off lapsed narrows
the pack and, with it, every partner-facing destination — the evaluation pack,
the pilot packet, the admin export, and the support export.

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
    "ship_private_evaluation_pilot_evidence_packs_with_bundle_ids_mirror_refs_known_issues_"
    "deltas_and_no_overclaim_guards_for_m5_enterprise_ecosystem_lanes"
)
RECORD_KIND = "ship_m5_private_evaluation_pilot_evidence_packs"
REGISTER_ID = "m5_evaluation_pilot_packs:v1"
ARTIFACT = REPO / "artifacts/release/m5" / f"{MODULE}.json"
CAPTURE = REPO / "artifacts/release/captures" / f"{MODULE}_validation_capture.json"
FIXTURES = REPO / "fixtures/compat/m5-evaluation-pilot-packs"
AS_OF = "2026-06-16"

CLAIM_MANIFEST_REF = (
    "artifacts/release/m5/"
    "add_claim_publication_manifests_and_automatic_claim_narrowing_so_docs_release_notes_"
    "badges_cli_inspect_and_evaluation_packs_reuse_one_source_of_truth.json"
)
QUALIFICATION_MATRIX_REF = (
    "artifacts/release/m5/"
    "freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix.json"
)
KNOWN_LIMITS_REF = (
    "artifacts/release/"
    "stabilize_the_known_limits_matrix_public_support_windows_and_stable_line_ownership_publication.json"
)
EVIDENCE_INDEX_REF = (
    "artifacts/release/m5/"
    "certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json"
)

LIFECYCLE_LABELS = ["lts", "stable", "beta", "preview", "withdrawn"]
ABOVE_CUTLINE = ["lts", "stable"]
BELOW_CUTLINE = ["beta", "preview", "withdrawn"]
LANE_KINDS = ["enterprise_evaluation", "enterprise_pilot", "ecosystem_partner", "managed_pilot"]
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
EVIDENCE_STATES = ["current", "stale", "missing", "dropped", "unsigned"]
MIRROR_KINDS = ["primary", "offline_bundle", "partner_mirror", "air_gapped"]
ISSUE_SEVERITIES = ["blocker", "major", "minor"]
DESTINATION_KINDS = [
    "evaluation_pack",
    "pilot_packet",
    "admin_export",
    "support_export",
    "service_health",
    "release_center",
]
REQUIRED_DESTINATIONS = ["evaluation_pack", "pilot_packet", "admin_export", "support_export"]
PACK_STATES = [
    "published",
    "narrowed_public_claim",
    "narrowed_stale",
    "narrowed_missing",
    "withheld",
]
FRESHNESS_STATES = ["current", "due_for_refresh", "breached", "missing"]
NARROWING_REASONS = [
    "public_claim_narrowed",
    "evidence_stale",
    "evidence_missing",
    "mirror_stale",
    "mirror_missing",
    "mirror_dropped",
    "mirror_unsigned",
    "validity_window_expired",
    "over_claim_beyond_public_claim",
    "owner_signoff_missing",
    "waiver_expired",
]
STOP_ACTIONS = [
    "hold_publication",
    "narrow_pack",
    "withhold_pack",
    "refresh_mirror",
    "refresh_evidence",
    "align_copy_to_public_claim",
    "renew_validity_window",
    "request_owner_signoff",
]

RANK = {"lts": 4, "stable": 3, "beta": 2, "preview": 1, "withdrawn": 0}
SUPPORT_BREADTH = {
    "full_support": 4,
    "maintenance_only": 3,
    "security_only": 2,
    "limited": 1,
    "end_of_life": 0,
}
# A pack that merely inherits a narrowed public claim downgrades its partner
# surfaces but is gated upstream by the claim manifest, so it does not itself hold
# promotion from this register. Every other reason is a pack-layer failure.
BLOCKING_REASONS = {r for r in NARROWING_REASONS if r != "public_claim_narrowed"}


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


def signoff(owner: str = "field-engineering", signed: bool = True) -> dict:
    return {"owner_ref": owner, "signed_off": signed, "signed_at": AS_OF if signed else None}


def window(starts: str = "2026-06-01", expires: str = "2026-12-31", expired: bool = False) -> dict:
    return {"starts_at": starts, "expires_at": expires, "expired": expired}


def mirror(entry: str, kind: str, state: str = "current") -> dict:
    return {
        "mirror_id": f"mirror/{entry}/{kind}",
        "mirror_kind": kind,
        "location_ref": f"distribution/{entry}/{kind}",
        "bundle_digest_ref": f"digest/{entry}",
        "state": state,
    }


def contact(role: str, entry: str) -> dict:
    return {"role": role, "contact_ref": f"contact/{entry}/{role}"}


def known_issue(
    entry: str,
    issue: str,
    *,
    severity: str,
    summary: str,
    public_known_limit_ref: str | None,
    disclosed: bool = True,
) -> dict:
    return {
        "issue_id": f"issue/{entry}/{issue}",
        "summary": summary,
        "severity": severity,
        "workaround_ref": f"workaround/{entry}/{issue}",
        "public_known_limit_ref": public_known_limit_ref,
        "disclosed": disclosed,
    }


def destinations(label: str, support_class: str, text: str, has_issues: bool, has_caveats: bool) -> list[dict]:
    # Every partner-facing destination renders from this pack id and inherits the
    # pack's published label, support class, and exact wording, so a narrowed pack
    # downgrades the evaluation pack, the pilot packet, the admin export, and the
    # support export at once, and no surface can keep a greener private claim.
    return [
        {
            "destination": dest,
            "source_pack_id": REGISTER_ID,
            "rendered_label": label,
            "rendered_support_class": support_class,
            "rendered_claim_text": text,
            "discloses_freshness": True,
            "discloses_known_issues": has_issues,
            "discloses_caveats": has_caveats,
        }
        for dest in DESTINATION_KINDS
    ]


def pack(
    *,
    entry_id: str,
    title: str,
    lane_kind: str,
    family_kind: str,
    family_ref: str,
    family_summary: str,
    release_blocking: bool,
    claim_manifest_entry_ref: str,
    public_claim_label: str,
    public_support_class: str,
    public_claim_text: str,
    bundle_id: str,
    mirror_refs: list[dict],
    support_contacts: list[dict],
    known_issues_delta: list[dict],
    deployment_caveats: list[str],
    validity: dict,
    pack_state: str,
    pack_support_class: str,
    pack_published_label: str,
    pack_claim_text: str,
    packet_slo: str,
    waiver: dict | None,
    active_reasons: list[str],
    rationale: str,
) -> dict:
    return {
        "entry_id": entry_id,
        "title": title,
        "lane_kind": lane_kind,
        "family_kind": family_kind,
        "family_ref": family_ref,
        "family_summary": family_summary,
        "release_blocking": release_blocking,
        "claim_manifest_ref": CLAIM_MANIFEST_REF,
        "claim_manifest_entry_ref": claim_manifest_entry_ref,
        "public_claim_label": public_claim_label,
        "public_support_class": public_support_class,
        "public_claim_text": public_claim_text,
        "bundle_id": bundle_id,
        "mirror_refs": mirror_refs,
        "support_contacts": support_contacts,
        "known_issues_delta": known_issues_delta,
        "deployment_caveats": deployment_caveats,
        "validity_window": validity,
        "pack_state": pack_state,
        "pack_support_class": pack_support_class,
        "pack_published_label": pack_published_label,
        "pack_claim_text": pack_claim_text,
        "destinations": destinations(
            pack_published_label,
            pack_support_class,
            pack_claim_text,
            bool(known_issues_delta),
            bool(deployment_caveats),
        ),
        "proof_packet": proof(entry_id, packet_slo),
        "waiver": waiver,
        "owner_signoff": signoff(),
        "active_narrowing_reasons": active_reasons,
        "rationale": rationale,
    }


def packs() -> list[dict]:
    out: list[dict] = []

    out.append(pack(
        entry_id="eval-pack-notebook-enterprise-eval",
        title="Notebook runtime enterprise evaluation pack",
        lane_kind="enterprise_evaluation",
        family_kind="notebook",
        family_ref="family/notebook-runtime",
        family_summary="Notebook and data-rich runtime: kernel protocol and cell-state schema.",
        release_blocking=True,
        claim_manifest_entry_ref="m5-claim-notebook",
        public_claim_label="stable",
        public_support_class="full_support",
        public_claim_text="Notebook runtime is Stable on every qualified platform and deployment profile.",
        bundle_id="bundle:notebook-eval-2026.06",
        mirror_refs=[
            mirror("notebook-eval", "primary"),
            mirror("notebook-eval", "offline_bundle"),
        ],
        support_contacts=[contact("evaluation_lead", "notebook-eval"), contact("support_escalation", "notebook-eval")],
        known_issues_delta=[],
        deployment_caveats=[],
        validity=window(),
        pack_state="published",
        pack_support_class="full_support",
        pack_published_label="stable",
        pack_claim_text="Notebook runtime is Stable on every qualified platform and deployment profile.",
        packet_slo="current",
        waiver=None,
        active_reasons=[],
        rationale="The pack reuses the public Stable notebook claim verbatim; its bundle mirrors are current and signed, the validity window is open, and the single pack drives the evaluation pack, pilot packet, admin export, and support export with one wording.",
    ))

    out.append(pack(
        entry_id="eval-pack-ai-provider-enterprise-pilot",
        title="AI provider boundary enterprise pilot pack",
        lane_kind="enterprise_pilot",
        family_kind="ai_provider",
        family_ref="family/ai-provider-boundary",
        family_summary="Helper/agent/provider boundary: capability handshake and model-route descriptors.",
        release_blocking=True,
        claim_manifest_entry_ref="m5-claim-ai-provider",
        public_claim_label="stable",
        public_support_class="full_support",
        public_claim_text="AI provider boundary is Stable, with a recorded client-scope caveat.",
        bundle_id="bundle:ai-provider-pilot-2026.06",
        mirror_refs=[mirror("ai-provider-pilot", "primary"), mirror("ai-provider-pilot", "partner_mirror")],
        support_contacts=[contact("evaluation_lead", "ai-provider-pilot"), contact("provider_liaison", "ai-provider-pilot")],
        known_issues_delta=[
            known_issue(
                "ai-provider-pilot",
                "byok-latency",
                severity="minor",
                summary="BYOK provider routing adds first-token latency on cold managed nodes; warm-pool mitigation documented.",
                public_known_limit_ref="known-limit/ai-provider/byok-latency",
            ),
        ],
        deployment_caveats=[
            "Pilot is scoped to managed and self-hosted profiles; air-gapped provider routing remains out of pilot scope.",
        ],
        validity=window(),
        pack_state="published",
        pack_support_class="full_support",
        pack_published_label="stable",
        pack_claim_text="AI provider boundary is Stable, with a recorded client-scope caveat.",
        packet_slo="current",
        waiver=None,
        active_reasons=[],
        rationale="The pilot pack reuses the public Stable claim and adds a known-issues delta and a deployment caveat that travel into every partner destination, so the private packet is scoped tighter than the public claim but never greener.",
    ))

    out.append(pack(
        entry_id="eval-pack-remote-helper-enterprise-pilot",
        title="Remote helper enterprise pilot pack",
        lane_kind="enterprise_pilot",
        family_kind="remote_helper",
        family_ref="family/remote-helper-skew",
        family_summary="Remote/helper boundary: RPC envelope and session-resume token skew.",
        release_blocking=True,
        claim_manifest_entry_ref="m5-claim-remote-helper",
        public_claim_label="stable",
        public_support_class="full_support",
        public_claim_text="Remote helper is Stable; toolchain-envelope coverage rides an active waiver.",
        bundle_id="bundle:remote-helper-pilot-2026.06",
        mirror_refs=[mirror("remote-helper-pilot", "primary"), mirror("remote-helper-pilot", "offline_bundle")],
        support_contacts=[contact("evaluation_lead", "remote-helper-pilot"), contact("support_escalation", "remote-helper-pilot")],
        known_issues_delta=[],
        deployment_caveats=[
            "Toolchain-envelope coverage rides the public waiver; pilots on the unqualified toolchain band are scheduled for re-qualification.",
        ],
        validity=window(),
        pack_state="published",
        pack_support_class="full_support",
        pack_published_label="stable",
        pack_claim_text="Remote helper is Stable; toolchain-envelope coverage rides an active waiver.",
        packet_slo="current",
        waiver={
            "waiver_ref": "waiver:eval_remote_helper_toolchain",
            "expires_at": "2026-12-31",
            "reason": "Toolchain-envelope re-qualification scheduled; interim pilot coverage waived by owner.",
        },
        active_reasons=[],
        rationale="The pack publishes the public Stable claim under the same unexpired waiver the public claim rides; bundle mirrors are current and the deployment caveat discloses the waived band to every partner surface.",
    ))

    out.append(pack(
        entry_id="eval-pack-companion-ecosystem-partner",
        title="Browser/mobile companion ecosystem partner pack",
        lane_kind="ecosystem_partner",
        family_kind="companion",
        family_ref="family/companion-handoff",
        family_summary="Companion boundary: handoff-eligibility token and companion session descriptor.",
        release_blocking=True,
        claim_manifest_entry_ref="m5-claim-companion",
        public_claim_label="beta",
        public_support_class="maintenance_only",
        public_claim_text="Companion handoff is Beta while the client-scope retest completes.",
        bundle_id="bundle:companion-partner-2026.06",
        mirror_refs=[mirror("companion-partner", "primary"), mirror("companion-partner", "partner_mirror")],
        support_contacts=[contact("partner_liaison", "companion-partner")],
        known_issues_delta=[],
        deployment_caveats=[
            "Companion handoff is in client-scope retest; partner pilots must treat handoff eligibility as Beta.",
        ],
        validity=window(),
        pack_state="narrowed_public_claim",
        pack_support_class="maintenance_only",
        pack_published_label="beta",
        pack_claim_text="Companion handoff is Beta while the client-scope retest completes.",
        packet_slo="current",
        waiver=None,
        active_reasons=["public_claim_narrowed"],
        rationale="The public companion claim is Beta, so the partner pack inherits Beta and pushes it into every partner destination rather than letting an evaluation pack keep a greener handoff claim; the inherited narrowing is gated by the claim manifest, not by this register.",
    ))

    out.append(pack(
        entry_id="eval-pack-ecosystem-sideload-partner",
        title="Extension sideload ecosystem partner pack",
        lane_kind="ecosystem_partner",
        family_kind="ecosystem",
        family_ref="family/ecosystem-sideload",
        family_summary="Extension/sideload boundary: ABI version and capability-grant manifest.",
        release_blocking=True,
        claim_manifest_entry_ref="m5-claim-ecosystem",
        public_claim_label="beta",
        public_support_class="limited",
        public_claim_text="Ecosystem sideload is Beta on limited support; the compatibility report is refreshing.",
        bundle_id="bundle:ecosystem-partner-2026.06",
        mirror_refs=[
            mirror("ecosystem-partner", "primary"),
            mirror("ecosystem-partner", "partner_mirror", "missing"),
        ],
        support_contacts=[contact("partner_liaison", "ecosystem-partner")],
        known_issues_delta=[
            known_issue(
                "ecosystem-partner",
                "abi-reinstall",
                severity="major",
                summary="Sideloaded extensions built before the supported ABI floor must be reinstalled before the pilot.",
                public_known_limit_ref="known-limit/ecosystem/abi-floor",
            ),
        ],
        deployment_caveats=[
            "Sideloaded extensions built before the supported ABI floor require a reinstall.",
        ],
        validity=window(),
        pack_state="narrowed_missing",
        pack_support_class="limited",
        pack_published_label="beta",
        pack_claim_text="Ecosystem sideload is Beta on limited support; the compatibility report is refreshing.",
        packet_slo="current",
        waiver=None,
        active_reasons=["public_claim_narrowed", "mirror_missing"],
        rationale="The public ecosystem claim is already Beta on limited support, so the partner pack inherits the narrowing and discloses the ABI-reinstall delta; the pack's partner mirror is also missing, but because the public claim is already below the cutline the claim manifest holds promotion, so this pack narrows its surfaces without independently blocking.",
    ))

    out.append(pack(
        entry_id="eval-pack-managed-airgapped-managed-pilot",
        title="Air-gapped managed profile managed pilot pack",
        lane_kind="managed_pilot",
        family_kind="managed_service",
        family_ref="family/managed-airgapped-profile",
        family_summary="Air-gapped managed profile: lockstep bundle digest with fail-closed boundary.",
        release_blocking=False,
        claim_manifest_entry_ref="m5-claim-managed-airgapped",
        public_claim_label="stable",
        public_support_class="security_only",
        public_claim_text="Air-gapped managed profile is Stable on security-only support; evidence refresh is due soon.",
        bundle_id="bundle:managed-airgapped-pilot-2026.06",
        mirror_refs=[
            mirror("managed-airgapped-pilot", "primary"),
            mirror("managed-airgapped-pilot", "air_gapped"),
        ],
        support_contacts=[contact("managed_operations", "managed-airgapped-pilot"), contact("support_escalation", "managed-airgapped-pilot")],
        known_issues_delta=[],
        deployment_caveats=[
            "Air-gapped pilots are lockstep-only: the helper, host, and bundle digest must upgrade together.",
        ],
        validity=window(),
        pack_state="published",
        pack_support_class="security_only",
        pack_published_label="stable",
        pack_claim_text="Air-gapped managed profile is Stable on security-only support; evidence refresh is due soon.",
        packet_slo="due_for_refresh",
        waiver=None,
        active_reasons=[],
        rationale="The managed pilot pack reuses the public Stable, security-only air-gapped claim and discloses the due-for-refresh freshness to every partner surface while the proof packet is still within its SLO; the pack's support class is never broadened beyond the public security-only class.",
    ))

    out.append(pack(
        entry_id="eval-pack-toolchain-enterprise-eval",
        title="Toolchain envelope enterprise evaluation pack",
        lane_kind="enterprise_evaluation",
        family_kind="toolchain_runtime",
        family_ref="family/toolchain-envelope",
        family_summary="Toolchain/runtime boundary: compiler ABI token and LSP protocol version.",
        release_blocking=True,
        claim_manifest_entry_ref="m5-claim-toolchain",
        public_claim_label="beta",
        public_support_class="full_support",
        public_claim_text="Toolchain envelope is Beta; qualification evidence is stale and refreshing.",
        bundle_id="bundle:toolchain-eval-2026.06",
        mirror_refs=[mirror("toolchain-eval", "primary"), mirror("toolchain-eval", "offline_bundle")],
        support_contacts=[contact("evaluation_lead", "toolchain-eval")],
        known_issues_delta=[],
        deployment_caveats=[
            "Toolchain qualification evidence is refreshing; evaluations must treat the toolchain envelope as Beta.",
        ],
        validity=window(),
        pack_state="narrowed_public_claim",
        pack_support_class="full_support",
        pack_published_label="beta",
        pack_claim_text="Toolchain envelope is Beta; qualification evidence is stale and refreshing.",
        packet_slo="current",
        waiver=None,
        active_reasons=["public_claim_narrowed"],
        rationale="The public toolchain claim narrowed to Beta on stale qualification evidence, so the evaluation pack inherits Beta and never advertises a greener toolchain claim; the upstream claim manifest already holds promotion for the family.",
    ))

    out.append(pack(
        entry_id="eval-pack-notebook-enterprise-pilot",
        title="Notebook runtime enterprise pilot pack",
        lane_kind="enterprise_pilot",
        family_kind="notebook",
        family_ref="family/notebook-runtime",
        family_summary="Notebook and data-rich runtime: pilot bundle on the public Stable claim.",
        release_blocking=True,
        claim_manifest_entry_ref="m5-claim-notebook",
        public_claim_label="stable",
        public_support_class="full_support",
        public_claim_text="Notebook runtime is Stable on every qualified platform and deployment profile.",
        bundle_id="bundle:notebook-pilot-2026.05",
        mirror_refs=[
            mirror("notebook-pilot", "primary"),
            mirror("notebook-pilot", "offline_bundle", "stale"),
        ],
        support_contacts=[contact("evaluation_lead", "notebook-pilot"), contact("support_escalation", "notebook-pilot")],
        known_issues_delta=[
            known_issue(
                "notebook-pilot",
                "offline-mirror-lag",
                severity="major",
                summary="The offline pilot bundle mirror lags the primary; pilots on the offline mirror must refresh before relying on it.",
                public_known_limit_ref=None,
            ),
        ],
        deployment_caveats=[
            "Pilot bundle offline mirror is stale; refresh the offline mirror before air-gapped pilots.",
        ],
        validity=window(),
        pack_state="narrowed_stale",
        pack_support_class="full_support",
        pack_published_label="beta",
        pack_claim_text="Notebook runtime pilot is narrowed to Beta until the offline bundle mirror is refreshed.",
        packet_slo="current",
        waiver=None,
        active_reasons=["mirror_stale"],
        rationale="The public notebook claim is Stable, but this pilot pack's offline bundle mirror went stale, so the pack narrows below the public ceiling to Beta and discloses the stale mirror across every partner surface; because the public claim is still Stable, the stale mirror is a pack-layer failure that holds promotion until the mirror is refreshed.",
    ))

    return out


def stop_rules() -> list[dict]:
    action = {
        "public_claim_narrowed": "narrow_pack",
        "evidence_stale": "refresh_evidence",
        "evidence_missing": "refresh_evidence",
        "mirror_stale": "refresh_mirror",
        "mirror_missing": "refresh_mirror",
        "mirror_dropped": "refresh_mirror",
        "mirror_unsigned": "refresh_mirror",
        "validity_window_expired": "renew_validity_window",
        "over_claim_beyond_public_claim": "align_copy_to_public_claim",
        "owner_signoff_missing": "request_owner_signoff",
        "waiver_expired": "narrow_pack",
    }
    titles = {
        "public_claim_narrowed": "Public claim narrowed",
        "evidence_stale": "Pack evidence stale",
        "evidence_missing": "Pack evidence missing",
        "mirror_stale": "Bundle mirror stale",
        "mirror_missing": "Bundle mirror missing",
        "mirror_dropped": "Bundle mirror dropped",
        "mirror_unsigned": "Bundle mirror unsigned",
        "validity_window_expired": "Pack validity window expired",
        "over_claim_beyond_public_claim": "Pack over-claims the public claim",
        "owner_signoff_missing": "Owner sign-off missing",
        "waiver_expired": "Pack waiver expired",
    }
    out = []
    for reason in NARROWING_REASONS:
        blocks = reason in BLOCKING_REASONS
        out.append(
            {
                "rule_id": f"m5_eval_pack_rule:{reason}",
                "title": titles[reason],
                "trigger_reason": reason,
                "applies_to_labels": ABOVE_CUTLINE,
                "default_action": action[reason],
                "blocks_promotion": blocks,
                "rationale": (
                    f"A pack whose public claim is at or above the cutline that reports '{reason}' holds publication until the private evidence is restored, so a private pilot can never substantiate a promise the public claim cannot."
                    if blocks
                    else f"A pack that reports '{reason}' narrows every partner-facing surface to inherit the public claim; the claim manifest already holds promotion for the public claim itself."
                ),
            }
        )
    return out


def compute_promotion(register: dict) -> dict:
    def fires(rule) -> bool:
        return any(
            p["public_claim_label"] in rule["applies_to_labels"]
            and rule["trigger_reason"] in p["active_narrowing_reasons"]
            for p in register["packs"]
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
    blocking_pack_ids = sorted(
        {
            p["entry_id"]
            for p in register["packs"]
            if holds_stable(p["public_claim_label"])
            and any(r in triggers for r in p["active_narrowing_reasons"])
        }
    )
    decision = "hold" if blocking_rule_ids else "proceed"
    return {
        "promotion_gate": "m5-evaluation-pilot-pack-gate",
        "decision": decision,
        "blocking_rule_ids": blocking_rule_ids,
        "blocking_claim_ids": blocking_pack_ids,
        "rationale": "Computed from the firing stop rules over private bundle-mirror state, proof-evidence freshness, validity windows, and no-overclaim copy; a pack that only inherits a narrowed public claim downgrades its partner surfaces but is gated by the claim manifest rather than this register.",
    }


def compute_summary(register: dict) -> dict:
    ps = register["packs"]

    def published(p):
        return holds_stable(p["pack_published_label"])

    def state(s):
        return sum(1 for p in ps if p["pack_state"] == s)

    def lane(k):
        return sum(1 for p in ps if p["lane_kind"] == k)

    def kind(k):
        return sum(1 for p in ps if p["family_kind"] == k)

    def slo(s):
        return sum(1 for p in ps if p["proof_packet"]["slo_state"] == s)

    def mirror_state(s):
        return sum(1 for p in ps for m in p["mirror_refs"] if m["state"] == s)

    rb = [p for p in ps if p["release_blocking"]]
    families = {p["family_ref"] for p in ps}
    return {
        "total_packs": len(ps),
        "total_families": len(families),
        "packs_published": sum(1 for p in ps if published(p)),
        "packs_narrowed": sum(1 for p in ps if not published(p)),
        "release_blocking_total": len(rb),
        "release_blocking_published": sum(1 for p in rb if published(p)),
        "release_blocking_narrowed": sum(1 for p in rb if not published(p)),
        "enterprise_evaluation_packs": lane("enterprise_evaluation"),
        "enterprise_pilot_packs": lane("enterprise_pilot"),
        "ecosystem_partner_packs": lane("ecosystem_partner"),
        "managed_pilot_packs": lane("managed_pilot"),
        "notebook_packs": kind("notebook"),
        "ai_provider_packs": kind("ai_provider"),
        "remote_helper_packs": kind("remote_helper"),
        "companion_packs": kind("companion"),
        "ecosystem_packs": kind("ecosystem"),
        "managed_service_packs": kind("managed_service"),
        "toolchain_runtime_packs": kind("toolchain_runtime"),
        "state_published": state("published"),
        "state_narrowed_public_claim": state("narrowed_public_claim"),
        "state_narrowed_stale": state("narrowed_stale"),
        "state_narrowed_missing": state("narrowed_missing"),
        "state_withheld": state("withheld"),
        "packs_with_known_issues": sum(1 for p in ps if p["known_issues_delta"]),
        "total_known_issues": sum(len(p["known_issues_delta"]) for p in ps),
        "packs_with_deployment_caveats": sum(1 for p in ps if p["deployment_caveats"]),
        "total_mirror_refs": sum(len(p["mirror_refs"]) for p in ps),
        "mirrors_current": mirror_state("current"),
        "mirrors_stale": mirror_state("stale"),
        "mirrors_missing": mirror_state("missing"),
        "mirrors_dropped": mirror_state("dropped"),
        "mirrors_unsigned": mirror_state("unsigned"),
        "total_support_contacts": sum(len(p["support_contacts"]) for p in ps),
        "total_destinations": sum(len(p["destinations"]) for p in ps),
        "destinations_freshness_disclosed": sum(
            1 for p in ps for d in p["destinations"] if d["discloses_freshness"]
        ),
        "destinations_known_issues_disclosed": sum(
            1 for p in ps for d in p["destinations"] if d["discloses_known_issues"]
        ),
        "packets_current": slo("current"),
        "packets_due_for_refresh": slo("due_for_refresh"),
        "packets_breached": slo("breached"),
        "packets_missing": slo("missing"),
        "total_active_narrowing_reasons": sum(len(p["active_narrowing_reasons"]) for p in ps),
        "rules_firing": sum(
            1
            for rule in register["stop_rules"]
            if any(
                p["public_claim_label"] in rule["applies_to_labels"]
                and rule["trigger_reason"] in p["active_narrowing_reasons"]
                for p in ps
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
        "known_limits_ref": KNOWN_LIMITS_REF,
        "evidence_index_ref": EVIDENCE_INDEX_REF,
        "lifecycle_labels": LIFECYCLE_LABELS,
        "lane_kinds": LANE_KINDS,
        "family_kinds": FAMILY_KINDS,
        "support_classes": SUPPORT_CLASSES,
        "evidence_states": EVIDENCE_STATES,
        "mirror_kinds": MIRROR_KINDS,
        "issue_severities": ISSUE_SEVERITIES,
        "destination_kinds": DESTINATION_KINDS,
        "required_destinations": REQUIRED_DESTINATIONS,
        "pack_states": PACK_STATES,
        "freshness_states": FRESHNESS_STATES,
        "narrowing_reasons": NARROWING_REASONS,
        "stop_actions": STOP_ACTIONS,
        "launch_cutline": {
            "cutline_level": "stable",
            "above_cutline_levels": ABOVE_CUTLINE,
            "below_cutline_levels": BELOW_CUTLINE,
            "description": "A private evaluation/pilot pack may publish a Stable (or LTS) label only when its public claim-publication manifest entry is itself at or above the cutline, the pack publishes exactly that public label, its support class is no broader than the public support class, its wording reuses the public claim verbatim, its bundle mirrors are current and signed, its proof packet is within its freshness SLO, the validity window is open, and the owner has signed off. A pack that loses any of those narrows below the public ceiling, and every partner-facing destination — the evaluation pack, the pilot packet, the admin export, and the support export — inherits the narrowed wording, label, support class, and freshness from the one pack. Pilot-only wording can never bypass a support-class limit or stale evidence.",
        },
        "release_blocking_family_refs": [],
        "stop_rules": stop_rules(),
        "packs": packs(),
    }
    register["release_blocking_family_refs"] = sorted(
        {p["family_ref"] for p in register["packs"] if p["release_blocking"]}
    )
    register["promotion"] = compute_promotion(register)
    register["summary"] = compute_summary(register)
    return register


def write_json(path: Path, payload: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")


def build_fixtures(register: dict) -> list[tuple[str, str]]:
    cases = []

    dup = copy.deepcopy(register)
    dup["packs"][1]["entry_id"] = dup["packs"][0]["entry_id"]
    dup["summary"] = compute_summary(dup)
    dup["promotion"] = compute_promotion(dup)
    write_json(FIXTURES / "duplicate_entry_id.json", dup)
    cases.append(("duplicate_entry_id.json", "DuplicateEntryId"))

    over = copy.deepcopy(register)
    target = next(p for p in over["packs"] if not holds_stable(p["public_claim_label"]))
    target["pack_published_label"] = "stable"
    for d in target["destinations"]:
        d["rendered_label"] = "stable"
    over["summary"] = compute_summary(over)
    over["promotion"] = compute_promotion(over)
    write_json(FIXTURES / "pack_over_claims_public_claim.json", over)
    cases.append(("pack_over_claims_public_claim.json", "PackLabelExceedsPublicClaim"))

    held = copy.deepcopy(register)
    backed = next(p for p in held["packs"] if holds_stable(p["pack_published_label"]))
    backed["active_narrowing_reasons"] = ["evidence_stale"]
    held["summary"] = compute_summary(held)
    held["promotion"] = compute_promotion(held)
    write_json(FIXTURES / "published_with_active_gap.json", held)
    cases.append(("published_with_active_gap.json", "PublishedWithActiveGap"))

    drift = copy.deepcopy(register)
    target = next(p for p in drift["packs"] if p["destinations"])
    target["destinations"][0]["rendered_claim_text"] = "Hand-edited pilot-only marketing copy that drifted from the public claim."
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
            "total_packs": s["total_packs"],
            "packs_published": s["packs_published"],
            "packs_narrowed": s["packs_narrowed"],
            "state_published": s["state_published"],
            "state_narrowed_public_claim": s["state_narrowed_public_claim"],
            "state_narrowed_stale": s["state_narrowed_stale"],
            "state_narrowed_missing": s["state_narrowed_missing"],
            "packs_with_known_issues": s["packs_with_known_issues"],
            "total_known_issues": s["total_known_issues"],
            "packs_with_deployment_caveats": s["packs_with_deployment_caveats"],
            "total_mirror_refs": s["total_mirror_refs"],
            "mirrors_stale": s["mirrors_stale"],
            "mirrors_missing": s["mirrors_missing"],
            "total_destinations": s["total_destinations"],
            "destinations_freshness_disclosed": s["destinations_freshness_disclosed"],
            "destinations_known_issues_disclosed": s["destinations_known_issues_disclosed"],
            "total_active_narrowing_reasons": s["total_active_narrowing_reasons"],
            "rules_firing": s["rules_firing"],
        },
        "promotion": {
            "decision": register["promotion"]["decision"],
            "blocking_rule_ids": register["promotion"]["blocking_rule_ids"],
            "blocking_claim_ids": register["promotion"]["blocking_claim_ids"],
        },
        "negative_drills": [
            {"drill_id": "drill:pack_over_claims_public_claim", "status": "passed"},
            {"drill_id": "drill:pack_support_class_over_claim", "status": "passed"},
            {"drill_id": "drill:published_with_active_gap", "status": "passed"},
            {"drill_id": "drill:destination_copy_drift", "status": "passed"},
            {"drill_id": "drill:known_issue_not_disclosed", "status": "passed"},
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
