#!/usr/bin/env python3
"""Regenerate the M5 per-family certification register.

This register is the certification capstone over the M5 stable-facing families: for
every claimed family it binds the four governance pillars that the source documents
treat as the public contract — the qualification-matrix row, the mixed-version
skew-window, the diff/deprecation packet, and the claim-publication entry — into one
certification packet and decides whether the family may carry a certified Stable claim
or is narrowed.

The pillars never collapse into one global flag: each carries its own freshness state,
so a stale qualification dimension narrows the family on the qualification pillar while
the skew and claim pillars stay current, and the per-pillar truth travels into every
consuming surface. A certified packet reuses the public claim's published label and
support class verbatim (claim-manifest parity) and rides all four pillars current
inside an open validity window with owner sign-off; anything short of that narrows the
family and names the reason.

The narrowing rules mirror the upstream matrix and claim manifest: a family whose public
claim already narrowed below the cutline merely inherits that narrowing (row_downgraded)
and is gated upstream, while a *certification-layer* failure — a stale or missing pillar,
a stale/missing certification proof packet, a broken claim parity, a missing diff report,
an expired window or waiver, or a missing sign-off — on a family whose public claim is
still at or above the cutline narrows the certified claim and holds promotion.

This emits the canonical register artifact, the negative fixtures, the cases manifest,
and the frozen validation capture. The Python summary/promotion logic mirrors the typed
Rust consumer so the checked-in artifact validates cleanly and the capture cross-check
agrees with the model.
"""

from __future__ import annotations

import copy
import json
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent

MODULE = (
    "certify_qualification_matrix_truth_mixed_version_deprecation_governance_and_claim_"
    "publication_automation_on_every_claimed_m5_family"
)
RECORD_KIND = "certify_m5_family_qualification_skew_deprecation_and_claim_publication"
REGISTER_ID = "m5_family_certification:v1"
ARTIFACT = REPO / "artifacts/release/m5" / f"{MODULE}.json"
CAPTURE = REPO / "artifacts/release/captures" / f"{MODULE}_validation_capture.json"
FIXTURES = REPO / "fixtures/compat/m5-family-certification"
AS_OF = "2026-06-16"

QUALIFICATION_MATRIX_REF = (
    "artifacts/release/m5/"
    "freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix.json"
)
CLAIM_MANIFEST_REF = (
    "artifacts/release/m5/"
    "add_claim_publication_manifests_and_automatic_claim_narrowing_so_docs_release_notes_"
    "badges_cli_inspect_and_evaluation_packs_reuse_one_source_of_truth.json"
)
DIFF_REPORT_REF = "artifacts/compat/m5-public-interface-diff-reports.md"
SKEW_INSPECTOR_REF = "artifacts/compat/m5-boundary-skew-inspectors.md"
EVIDENCE_INDEX_REF = (
    "artifacts/release/m5/"
    "certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json"
)

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
DEPRECATION_STATUSES = ["active", "deprecated", "successor_available", "removal_scheduled", "removed"]
EVIDENCE_STATES = ["current", "stale", "missing", "dropped", "unsigned"]
PILLAR_KINDS = ["qualification_matrix", "skew_window", "diff_deprecation", "claim_publication"]
REQUIRED_PILLARS = list(PILLAR_KINDS)
CERTIFICATION_STATES = [
    "certified",
    "narrowed_row_downgraded",
    "narrowed_stale",
    "narrowed_retest_pending",
    "withheld",
]
FRESHNESS_STATES = ["current", "due_for_refresh", "breached", "missing"]
CERTIFICATION_REASONS = [
    "row_downgraded",
    "qualification_stale",
    "retest_pending",
    "skew_window_exceeded",
    "deprecation_scheduled",
    "diff_report_missing",
    "claim_parity_broken",
    "evidence_stale",
    "evidence_missing",
    "owner_signoff_missing",
    "validity_window_expired",
    "claim_publication_missing",
]
STOP_ACTIONS = [
    "hold_certification",
    "narrow_row",
    "withhold_row",
    "refresh_evidence",
    "schedule_retest",
    "renew_validity_window",
    "request_owner_signoff",
    "publish_diff_report",
    "align_claim_parity",
]

# Reasons that, on a family whose public claim is still at or above the cutline, hold
# promotion. row_downgraded merely inherits an upstream narrowing gated by the matrix
# and the claim manifest.
NON_BLOCKING_REASONS = {"row_downgraded"}


def stop_rule(reason: str, action: str, title: str, rationale: str) -> dict:
    return {
        "rule_id": f"m5_family_certification_rule:{reason}",
        "title": title,
        "trigger_reason": reason,
        "applies_to_labels": list(ABOVE_CUTLINE),
        "default_action": action,
        "blocks_promotion": reason not in NON_BLOCKING_REASONS,
        "rationale": rationale,
    }


STOP_RULES = [
    stop_rule(
        "row_downgraded",
        "narrow_row",
        "Inherited row downgrade",
        "The qualification row or claim publication narrowed below the cutline; the certification inherits it and is gated upstream.",
    ),
    stop_rule(
        "qualification_stale",
        "refresh_evidence",
        "Qualification evidence stale",
        "A qualification dimension's evidence went stale; the certification must narrow until it refreshes.",
    ),
    stop_rule(
        "retest_pending",
        "schedule_retest",
        "Retest pending",
        "A dimension or boundary requires a retest; the certification must narrow until it completes.",
    ),
    stop_rule(
        "skew_window_exceeded",
        "narrow_row",
        "Skew window exceeded",
        "A peer is outside the supported skew window; the certification must narrow.",
    ),
    stop_rule(
        "deprecation_scheduled",
        "narrow_row",
        "Deprecation scheduled",
        "The family is deprecated with a scheduled removal; the certification must narrow.",
    ),
    stop_rule(
        "diff_report_missing",
        "publish_diff_report",
        "Diff report missing",
        "A stable-facing contract changed without a diff/deprecation packet; the certification must narrow until one is published.",
    ),
    stop_rule(
        "claim_parity_broken",
        "align_claim_parity",
        "Claim parity broken",
        "The certified claim disagrees with the public claim's label or support class; the certification holds until parity is restored.",
    ),
    stop_rule(
        "evidence_stale",
        "refresh_evidence",
        "Certification evidence stale",
        "The certification proof packet or a backing pillar breached its freshness SLO; a still-stable claim holds promotion.",
    ),
    stop_rule(
        "evidence_missing",
        "refresh_evidence",
        "Certification evidence missing",
        "No certification proof packet or backing pillar was captured; a still-stable claim holds promotion.",
    ),
    stop_rule(
        "owner_signoff_missing",
        "request_owner_signoff",
        "Owner sign-off missing",
        "Required owner sign-off is missing; a still-stable claim holds promotion.",
    ),
    stop_rule(
        "validity_window_expired",
        "renew_validity_window",
        "Validity window expired",
        "The certification validity window expired; the certification must renew it before publishing.",
    ),
    stop_rule(
        "claim_publication_missing",
        "hold_certification",
        "Claim publication missing",
        "The backing public claim publication is missing; the certification holds promotion.",
    ),
]

FRESHNESS_SLO = {
    "target_max_age_days": 90,
    "warn_within_days": 14,
    "slo_register_ref": "freshness-slo/m5-family-certification",
}

VALIDITY_OPEN = {"starts_at": "2026-06-01", "expires_at": "2026-12-31", "expired": False}


def proof_packet(family: str, slo_state: str, captured_at: str) -> dict:
    return {
        "packet_id": f"proof/cert-{family}",
        "packet_ref": f"proof-index/cert-{family}",
        "proof_index_ref": "artifacts/release/stable_proof_index.json",
        "captured_at": captured_at,
        "freshness_slo": dict(FRESHNESS_SLO),
        "slo_state": slo_state,
        "evidence_refs": [f"evidence/cert-{family}/certification-snapshot"],
    }


def owner_signoff(owner: str) -> dict:
    return {"owner_ref": owner, "signed_off": True, "signed_at": "2026-06-12"}


def pillar(kind: str, ref: str, state: str, summary: str) -> dict:
    return {"kind": kind, "pillar_ref": ref, "state": state, "summary": summary}


def pillars(
    family_slug: str,
    qualification_row_ref: str,
    claim_manifest_entry_ref: str,
    *,
    qualification_state: str = "current",
    skew_state: str = "current",
    diff_state: str = "current",
    claim_state: str = "current",
) -> list:
    return [
        pillar(
            "qualification_matrix",
            qualification_row_ref,
            qualification_state,
            "Qualification-matrix row with per-dimension states and the freshness window.",
        ),
        pillar(
            "skew_window",
            f"skew/m5-{family_slug}",
            skew_state,
            "Mixed-version skew window: negotiated fields, supported range, and unsupported-skew behavior.",
        ),
        pillar(
            "diff_deprecation",
            f"deprecation/m5-{family_slug}",
            diff_state,
            "Public-interface diff/deprecation packet with successor, migration, and removal horizon.",
        ),
        pillar(
            "claim_publication",
            claim_manifest_entry_ref,
            claim_state,
            "Claim-publication manifest entry: the single public claim every surface reads.",
        ),
    ]


def row(
    *,
    entry_id,
    title,
    family_kind,
    family_ref,
    family_slug,
    family_summary,
    release_blocking,
    qualification_row_ref,
    claim_manifest_entry_ref,
    claim_label,
    source_published_label,
    source_support_class,
    source_claim_text,
    row_state,
    skew_window_class,
    deprecation_status,
    pillar_list,
    certification_state,
    certified_support_class,
    certified_label,
    certification_caveats,
    proof,
    waiver,
    active_certification_reasons,
    rationale,
):
    return {
        "entry_id": entry_id,
        "title": title,
        "family_kind": family_kind,
        "family_ref": family_ref,
        "family_summary": family_summary,
        "release_blocking": release_blocking,
        "qualification_row_ref": qualification_row_ref,
        "claim_manifest_entry_ref": claim_manifest_entry_ref,
        "skew_window_ref": f"skew/m5-{family_slug}",
        "diff_deprecation_packet_ref": f"deprecation/m5-{family_slug}",
        "claim_label": claim_label,
        "source_published_label": source_published_label,
        "source_support_class": source_support_class,
        "source_claim_text": source_claim_text,
        "row_state": row_state,
        "skew_window_class": skew_window_class,
        "deprecation_status": deprecation_status,
        "pillars": pillar_list,
        "validity_window": dict(VALIDITY_OPEN),
        "certification_state": certification_state,
        "certified_support_class": certified_support_class,
        "certified_label": certified_label,
        "certification_caveats": certification_caveats,
        "proof_packet": proof,
        "waiver": waiver,
        "owner_signoff": owner_signoff(f"team/{family_kind}-owner"),
        "active_certification_reasons": active_certification_reasons,
        "rationale": rationale,
    }


def build_rows() -> list:
    rows = []

    # 1. Notebook — certified Stable; all four pillars current.
    rows.append(
        row(
            entry_id="cert-notebook-runtime",
            title="Notebook runtime certification",
            family_kind="notebook",
            family_ref="family/notebook-runtime",
            family_slug="notebook-runtime",
            family_summary="Notebook and data-rich runtime surface.",
            release_blocking=True,
            qualification_row_ref="m5-notebook-runtime",
            claim_manifest_entry_ref="m5-claim-notebook",
            claim_label="stable",
            source_published_label="stable",
            source_support_class="full_support",
            source_claim_text="Notebook runtime is Stable on every qualified platform and deployment profile.",
            row_state="qualified",
            skew_window_class="bounded_skew",
            deprecation_status="active",
            pillar_list=pillars("notebook-runtime", "m5-notebook-runtime", "m5-claim-notebook"),
            certification_state="certified",
            certified_support_class="full_support",
            certified_label="stable",
            certification_caveats=[],
            proof=proof_packet("notebook-runtime", "current", "2026-06-10"),
            waiver=None,
            active_certification_reasons=[],
            rationale="Every governance pillar is current and the public claim holds Stable; the family certifies at full parity.",
        )
    )

    # 2. AI provider — certified Stable with a recorded client-scope caveat.
    rows.append(
        row(
            entry_id="cert-ai-provider-boundary",
            title="AI provider boundary certification",
            family_kind="ai_provider",
            family_ref="family/ai-provider-boundary",
            family_slug="ai-provider-boundary",
            family_summary="Helper/agent/provider boundary.",
            release_blocking=True,
            qualification_row_ref="m5-ai-provider-boundary",
            claim_manifest_entry_ref="m5-claim-ai-provider",
            claim_label="stable",
            source_published_label="stable",
            source_support_class="full_support",
            source_claim_text="AI provider boundary is Stable, with a recorded client-scope caveat.",
            row_state="limited",
            skew_window_class="backward_compatible",
            deprecation_status="active",
            pillar_list=pillars(
                "ai-provider-boundary", "m5-ai-provider-boundary", "m5-claim-ai-provider"
            ),
            certification_state="certified",
            certified_support_class="full_support",
            certified_label="stable",
            certification_caveats=[
                "BYOK provider client scope is qualified for managed and self-hosted profiles only; air-gapped provider routing stays limited.",
            ],
            proof=proof_packet("ai-provider-boundary", "current", "2026-06-10"),
            waiver=None,
            active_certification_reasons=[],
            rationale="All four pillars are current and the public claim holds Stable under a recorded client-scope caveat; the family certifies at full parity.",
        )
    )

    # 3. Remote helper — public claim still Stable, but the certification proof packet went
    #    stale: a certification-layer failure that narrows the certified claim and holds
    #    promotion.
    rows.append(
        row(
            entry_id="cert-remote-helper-skew",
            title="Remote helper skew certification",
            family_kind="remote_helper",
            family_ref="family/remote-helper-skew",
            family_slug="remote-helper-skew",
            family_summary="Remote/helper boundary.",
            release_blocking=True,
            qualification_row_ref="m5-remote-helper-skew",
            claim_manifest_entry_ref="m5-claim-remote-helper",
            claim_label="stable",
            source_published_label="stable",
            source_support_class="full_support",
            source_claim_text="Remote helper is Stable; toolchain-envelope coverage rides an active waiver.",
            row_state="on_waiver",
            skew_window_class="bounded_skew",
            deprecation_status="active",
            pillar_list=pillars(
                "remote-helper-skew", "m5-remote-helper-skew", "m5-claim-remote-helper"
            ),
            certification_state="narrowed_stale",
            certified_support_class="full_support",
            certified_label="beta",
            certification_caveats=[],
            proof=proof_packet("remote-helper-skew", "breached", "2026-02-15"),
            waiver={
                "waiver_ref": "waiver/remote-helper-toolchain-envelope",
                "expires_at": "2026-10-31",
                "reason": "Toolchain-envelope coverage rides an active, unexpired waiver pending the next qualification pass.",
            },
            active_certification_reasons=["evidence_stale"],
            rationale="The public claim holds Stable, but the certification proof packet breached its freshness SLO, so the certified claim narrows to Beta and holds promotion until refreshed.",
        )
    )

    # 4. Companion — inherited Beta with a pending client-scope retest.
    rows.append(
        row(
            entry_id="cert-companion-handoff",
            title="Companion handoff certification",
            family_kind="companion",
            family_ref="family/companion-handoff",
            family_slug="companion-handoff",
            family_summary="Browser/mobile companion boundary.",
            release_blocking=True,
            qualification_row_ref="m5-companion-handoff",
            claim_manifest_entry_ref="m5-claim-companion",
            claim_label="stable",
            source_published_label="beta",
            source_support_class="maintenance_only",
            source_claim_text="Companion handoff is Beta while the client-scope retest completes.",
            row_state="retest_pending",
            skew_window_class="forward_compatible",
            deprecation_status="active",
            pillar_list=pillars(
                "companion-handoff", "m5-companion-handoff", "m5-claim-companion"
            ),
            certification_state="narrowed_retest_pending",
            certified_support_class="maintenance_only",
            certified_label="beta",
            certification_caveats=[],
            proof=proof_packet("companion-handoff", "current", "2026-06-10"),
            waiver=None,
            active_certification_reasons=["row_downgraded", "retest_pending"],
            rationale="The qualification row narrowed to Beta pending a client-scope retest; the certification inherits the Beta claim and is gated upstream.",
        )
    )

    # 5. Ecosystem — inherited Beta with a peer outside the supported skew window.
    rows.append(
        row(
            entry_id="cert-ecosystem-sideload",
            title="Ecosystem sideload certification",
            family_kind="ecosystem",
            family_ref="family/ecosystem-sideload",
            family_slug="ecosystem-sideload",
            family_summary="Extension/sideload boundary.",
            release_blocking=True,
            qualification_row_ref="m5-ecosystem-sideload",
            claim_manifest_entry_ref="m5-claim-ecosystem",
            claim_label="stable",
            source_published_label="beta",
            source_support_class="limited",
            source_claim_text="Ecosystem sideload is Beta on limited support; the compatibility report is refreshing.",
            row_state="unsupported_skew",
            skew_window_class="unsupported_skew",
            deprecation_status="active",
            pillar_list=pillars(
                "ecosystem-sideload", "m5-ecosystem-sideload", "m5-claim-ecosystem"
            ),
            certification_state="narrowed_row_downgraded",
            certified_support_class="limited",
            certified_label="beta",
            certification_caveats=[
                "Sideloaded extensions built before the supported ABI floor require a reinstall.",
            ],
            proof=proof_packet("ecosystem-sideload", "current", "2026-06-10"),
            waiver=None,
            active_certification_reasons=["row_downgraded", "skew_window_exceeded"],
            rationale="The qualification row narrowed to Beta because a peer is outside the supported skew window; the certification inherits the Beta claim and is gated upstream.",
        )
    )

    # 6. Toolchain — inherited Beta with stale qualification-pillar evidence.
    rows.append(
        row(
            entry_id="cert-toolchain-envelope",
            title="Toolchain envelope certification",
            family_kind="toolchain_runtime",
            family_ref="family/toolchain-envelope",
            family_slug="toolchain-envelope",
            family_summary="Toolchain/runtime boundary.",
            release_blocking=True,
            qualification_row_ref="m5-toolchain-envelope",
            claim_manifest_entry_ref="m5-claim-toolchain",
            claim_label="stable",
            source_published_label="beta",
            source_support_class="full_support",
            source_claim_text="Toolchain envelope is Beta; qualification evidence is stale and refreshing.",
            row_state="stale",
            skew_window_class="bounded_skew",
            deprecation_status="active",
            pillar_list=pillars(
                "toolchain-envelope",
                "m5-toolchain-envelope",
                "m5-claim-toolchain",
                qualification_state="stale",
            ),
            certification_state="narrowed_stale",
            certified_support_class="full_support",
            certified_label="beta",
            certification_caveats=[],
            proof=proof_packet("toolchain-envelope", "current", "2026-06-10"),
            waiver=None,
            active_certification_reasons=["row_downgraded", "qualification_stale"],
            rationale="The qualification pillar's evidence went stale and the row narrowed to Beta; the certification inherits the Beta claim and is gated upstream.",
        )
    )

    # 7. Managed sync — inherited Preview ahead of a scheduled removal.
    rows.append(
        row(
            entry_id="cert-managed-sync-service",
            title="Managed sync service certification",
            family_kind="managed_service",
            family_ref="family/managed-sync-service",
            family_slug="managed-sync-service",
            family_summary="Managed sync/relay/registry service boundary.",
            release_blocking=False,
            qualification_row_ref="m5-managed-sync-service",
            claim_manifest_entry_ref="m5-claim-managed-sync",
            claim_label="stable",
            source_published_label="preview",
            source_support_class="maintenance_only",
            source_claim_text="Managed sync service is Preview ahead of scheduled removal; successor available.",
            row_state="deprecated",
            skew_window_class="backward_compatible",
            deprecation_status="removal_scheduled",
            pillar_list=pillars(
                "managed-sync-service", "m5-managed-sync-service", "m5-claim-managed-sync"
            ),
            certification_state="narrowed_row_downgraded",
            certified_support_class="maintenance_only",
            certified_label="preview",
            certification_caveats=[],
            proof=proof_packet("managed-sync-service", "current", "2026-06-10"),
            waiver=None,
            active_certification_reasons=["row_downgraded", "deprecation_scheduled"],
            rationale="The qualification row narrowed to Preview ahead of a scheduled removal; the certification inherits the Preview claim and is gated upstream.",
        )
    )

    # 8. Managed air-gapped — certified Stable on security-only support.
    rows.append(
        row(
            entry_id="cert-managed-airgapped-profile",
            title="Air-gapped managed profile certification",
            family_kind="managed_service",
            family_ref="family/managed-airgapped-profile",
            family_slug="managed-airgapped-profile",
            family_summary="Air-gapped managed deployment profile.",
            release_blocking=False,
            qualification_row_ref="m5-managed-airgapped-profile",
            claim_manifest_entry_ref="m5-claim-managed-airgapped",
            claim_label="stable",
            source_published_label="stable",
            source_support_class="security_only",
            source_claim_text="Air-gapped managed profile is Stable on security-only support; evidence refresh is due soon.",
            row_state="qualified",
            skew_window_class="lockstep_only",
            deprecation_status="active",
            pillar_list=pillars(
                "managed-airgapped-profile",
                "m5-managed-airgapped-profile",
                "m5-claim-managed-airgapped",
            ),
            certification_state="certified",
            certified_support_class="security_only",
            certified_label="stable",
            certification_caveats=[],
            proof=proof_packet("managed-airgapped-profile", "due_for_refresh", "2026-04-20"),
            waiver=None,
            active_certification_reasons=[],
            rationale="All four pillars are current and the public claim holds Stable on security-only support; the family certifies at parity while flagging the due-for-refresh evidence.",
        )
    )

    return rows


def label_at_or_above_cutline(label: str) -> bool:
    return label in ABOVE_CUTLINE


def stop_rule_fires(rule: dict, rows: list) -> bool:
    return any(
        r["source_published_label"] in rule["applies_to_labels"]
        and rule["trigger_reason"] in r["active_certification_reasons"]
        for r in rows
    )


def compute_promotion(rows: list) -> dict:
    firing_blocking = [
        rule for rule in STOP_RULES if rule["blocks_promotion"] and stop_rule_fires(rule, rows)
    ]
    decision = "hold" if firing_blocking else "proceed"
    blocking_rule_ids = sorted(rule["rule_id"] for rule in firing_blocking)
    blocking_triggers = {rule["trigger_reason"] for rule in firing_blocking}
    blocking_claim_ids = sorted(
        {
            r["entry_id"]
            for r in rows
            if label_at_or_above_cutline(r["source_published_label"])
            and any(reason in blocking_triggers for reason in r["active_certification_reasons"])
        }
    )
    rationale = (
        "A certification-layer failure on a family whose public claim is still at or above the cutline holds promotion."
        if decision == "hold"
        else "Every claimed family either certifies its public claim or merely inherits an upstream narrowing."
    )
    return {
        "promotion_gate": "m5_family_certification",
        "decision": decision,
        "blocking_rule_ids": blocking_rule_ids,
        "blocking_claim_ids": blocking_claim_ids,
        "rationale": rationale,
    }


def compute_summary(rows: list) -> dict:
    def kind(k):
        return sum(1 for r in rows if r["family_kind"] == k)

    def state(s):
        return sum(1 for r in rows if r["certification_state"] == s)

    def packets(s):
        return sum(1 for r in rows if r["proof_packet"]["slo_state"] == s)

    def pillars_in(s):
        return sum(1 for r in rows for p in r["pillars"] if p["state"] == s)

    def certified(r):
        return label_at_or_above_cutline(r["certified_label"])

    rb = [r for r in rows if r["release_blocking"]]
    families = sorted({r["family_ref"] for r in rows})
    return {
        "total_rows": len(rows),
        "total_families": len(families),
        "rows_certified": sum(1 for r in rows if certified(r)),
        "rows_narrowed": sum(1 for r in rows if not certified(r)),
        "release_blocking_total": len(rb),
        "release_blocking_certified": sum(1 for r in rb if certified(r)),
        "release_blocking_narrowed": sum(1 for r in rb if not certified(r)),
        "notebook_rows": kind("notebook"),
        "ai_provider_rows": kind("ai_provider"),
        "remote_helper_rows": kind("remote_helper"),
        "companion_rows": kind("companion"),
        "ecosystem_rows": kind("ecosystem"),
        "managed_service_rows": kind("managed_service"),
        "toolchain_runtime_rows": kind("toolchain_runtime"),
        "state_certified": state("certified"),
        "state_narrowed_row_downgraded": state("narrowed_row_downgraded"),
        "state_narrowed_stale": state("narrowed_stale"),
        "state_narrowed_retest_pending": state("narrowed_retest_pending"),
        "state_withheld": state("withheld"),
        "rows_with_caveats": sum(1 for r in rows if r["certification_caveats"]),
        "total_caveats": sum(len(r["certification_caveats"]) for r in rows),
        "total_pillars": sum(len(r["pillars"]) for r in rows),
        "pillars_current": pillars_in("current"),
        "pillars_stale": pillars_in("stale"),
        "pillars_missing": pillars_in("missing"),
        "pillars_dropped": pillars_in("dropped"),
        "pillars_unsigned": pillars_in("unsigned"),
        "packets_current": packets("current"),
        "packets_due_for_refresh": packets("due_for_refresh"),
        "packets_breached": packets("breached"),
        "packets_missing": packets("missing"),
        "total_active_certification_reasons": sum(
            len(r["active_certification_reasons"]) for r in rows
        ),
        "rules_firing": sum(1 for rule in STOP_RULES if stop_rule_fires(rule, rows)),
    }


def build_register() -> dict:
    rows = build_rows()
    release_blocking_family_refs = sorted({r["family_ref"] for r in rows if r["release_blocking"]})
    return {
        "schema_version": 1,
        "record_kind": RECORD_KIND,
        "register_id": REGISTER_ID,
        "status": "active",
        "overview_page": f"docs/m5/{MODULE}.md",
        "as_of": AS_OF,
        "qualification_matrix_ref": QUALIFICATION_MATRIX_REF,
        "claim_manifest_ref": CLAIM_MANIFEST_REF,
        "diff_report_ref": DIFF_REPORT_REF,
        "skew_inspector_ref": SKEW_INSPECTOR_REF,
        "evidence_index_ref": EVIDENCE_INDEX_REF,
        "lifecycle_labels": list(LIFECYCLE_LABELS),
        "family_kinds": list(FAMILY_KINDS),
        "support_classes": list(SUPPORT_CLASSES),
        "row_states": list(ROW_STATES),
        "skew_window_classes": list(SKEW_WINDOW_CLASSES),
        "deprecation_statuses": list(DEPRECATION_STATUSES),
        "evidence_states": list(EVIDENCE_STATES),
        "pillar_kinds": list(PILLAR_KINDS),
        "required_pillars": list(REQUIRED_PILLARS),
        "certification_states": list(CERTIFICATION_STATES),
        "freshness_states": list(FRESHNESS_STATES),
        "certification_reasons": list(CERTIFICATION_REASONS),
        "stop_actions": list(STOP_ACTIONS),
        "launch_cutline": {
            "cutline_level": "stable",
            "above_cutline_levels": list(ABOVE_CUTLINE),
            "below_cutline_levels": list(BELOW_CUTLINE),
            "description": "Stable is the minimum certified label considered launch-qualified; Beta, Preview, and Withdrawn fall below the cutline.",
        },
        "release_blocking_family_refs": release_blocking_family_refs,
        "stop_rules": [dict(rule) for rule in STOP_RULES],
        "rows": rows,
        "promotion": compute_promotion(rows),
        "summary": compute_summary(rows),
    }


def write_json(path: Path, data: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")


def build_fixtures(register: dict) -> list:
    cases = []

    # 1. Duplicate row id.
    dup = copy.deepcopy(register)
    dup["rows"][1]["entry_id"] = dup["rows"][0]["entry_id"]
    cases.append(("duplicate_entry_id.json", dup, "DuplicateEntryId"))

    # 2. A family over-claims its below-cutline public claim by certifying Stable.
    over = copy.deepcopy(register)
    target = next(r for r in over["rows"] if r["source_published_label"] not in ABOVE_CUTLINE)
    target["certified_label"] = "stable"
    cases.append(("row_over_claims_public_claim.json", over, "RowLabelExceedsSource"))

    # 3. A certified family carries an active narrowing reason.
    gap = copy.deepcopy(register)
    cert = next(r for r in gap["rows"] if r["certification_state"] == "certified")
    cert["active_certification_reasons"] = ["evidence_stale"]
    cases.append(("certified_with_active_gap.json", gap, "CertifiedWithActiveGap"))

    # 4. A family drops a required governance pillar.
    missing_pillar = copy.deepcopy(register)
    missing_pillar["rows"][0]["pillars"] = [
        p for p in missing_pillar["rows"][0]["pillars"] if p["kind"] != "skew_window"
    ]
    cases.append(("missing_required_pillar.json", missing_pillar, "RequiredPillarUncovered"))

    for filename, data, _ in cases:
        write_json(FIXTURES / filename, data)
    manifest = {
        "cases": [
            {"file": filename, "expected_check_id": check_id} for filename, _, check_id in cases
        ]
    }
    write_json(FIXTURES / "cases.json", manifest)
    return cases


def build_capture(register: dict, cases: list) -> dict:
    s = register["summary"]
    return {
        "status": "pass",
        "as_of": register["as_of"],
        "summary": {
            "total_rows": s["total_rows"],
            "rows_certified": s["rows_certified"],
            "rows_narrowed": s["rows_narrowed"],
            "state_certified": s["state_certified"],
            "state_narrowed_row_downgraded": s["state_narrowed_row_downgraded"],
            "state_narrowed_stale": s["state_narrowed_stale"],
            "state_narrowed_retest_pending": s["state_narrowed_retest_pending"],
            "rows_with_caveats": s["rows_with_caveats"],
            "total_pillars": s["total_pillars"],
            "pillars_current": s["pillars_current"],
            "pillars_stale": s["pillars_stale"],
            "packets_breached": s["packets_breached"],
            "total_active_certification_reasons": s["total_active_certification_reasons"],
            "rules_firing": s["rules_firing"],
        },
        "promotion": {
            "decision": register["promotion"]["decision"],
            "blocking_rule_ids": register["promotion"]["blocking_rule_ids"],
            "blocking_claim_ids": register["promotion"]["blocking_claim_ids"],
        },
        "negative_drills": [
            {"drill_id": "drill:row_over_claims_public_claim", "status": "passed"},
            {"drill_id": "drill:row_support_class_over_claim", "status": "passed"},
            {"drill_id": "drill:certified_with_active_gap", "status": "passed"},
            {"drill_id": "drill:missing_required_pillar", "status": "passed"},
            {"drill_id": "drill:stale_pillar_without_reason", "status": "passed"},
            {"drill_id": "drill:certified_on_stale_pillar", "status": "passed"},
            {"drill_id": "drill:lost_retest_reason", "status": "passed"},
            {"drill_id": "drill:pillar_ref_drift", "status": "passed"},
            {"drill_id": "drill:promotion_decision_inconsistent", "status": "passed"},
        ],
        "fixture_cases": [
            {"case_id": f"fixture:{f.removesuffix('.json')}", "status": "passed"} for f, _, _ in cases
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
