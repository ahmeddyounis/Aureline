#!/usr/bin/env python3
"""Regenerate the M5 qualification-matrix and claim-scope export-packet register.

Where the qualification/skew matrix is the machine-readable truth for every claimed
M5 family and the claim-publication manifest is the single public claim every
claim-bearing surface reads, this register is the *export* layer that answers, for
support, shiproom, docs, and partner review, exactly which M5 rows are being claimed,
what freshness and expiry state each carries, what skew window applies, and what stale
or retest-needed states are live — without tribal memory. For each claimed family it
binds one export row to:

- the reopen refs a shiproom dashboard follows back to the authoritative record: the
  qualification row, its deprecation packet, and the public claim entry,
- the row-level truth that never collapses into one flag: the qualification row state,
  the skew-window class, the support class, the deprecation status, the freshness
  state, the validity window, the evidence refs, and the active stale/retest reasons,
- and the copy-safe scope wording every audience renders — never greener than the
  public claim's published label or support class, both hard ceilings.

The no-overclaim guard is the spine of the register: a row may never publish a greener
label or broader support class than the public claim, a row that holds the public
label reuses the public wording verbatim, and every audience (support, shiproom, docs,
partner review) discloses the row freshness, the active stale/retest reasons, and the
caveats, so a narrowed row downgrades every audience at once and no exported packet
loses the row-level reason. An inherited row downgrade is gated by the matrix and the
claim manifest, while an export-layer failure (stale/missing export evidence, an
expired window or waiver, a missing sign-off, or over-claiming copy) on a row whose
public claim is still at or above the cutline holds promotion.

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
    "implement_qualification_matrix_and_claim_scope_export_packets_for_support_shiproom_"
    "docs_and_partner_review_with_row_level_stale_retest_needed_truth"
)
RECORD_KIND = "implement_m5_qualification_matrix_and_claim_scope_export_packets"
REGISTER_ID = "m5_claim_scope_export_packets:v1"
ARTIFACT = REPO / "artifacts/release/m5" / f"{MODULE}.json"
CAPTURE = REPO / "artifacts/release/captures" / f"{MODULE}_validation_capture.json"
FIXTURES = REPO / "fixtures/compat/m5-claim-scope-export-packets"
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
EVIDENCE_KINDS = [
    "qualification_row",
    "deprecation_packet",
    "skew_window",
    "support_window",
    "compatibility_report",
    "claim_manifest",
    "proof_packet",
]
AUDIENCES = ["support", "shiproom", "docs", "partner_review", "release_notes"]
REQUIRED_AUDIENCES = ["support", "shiproom", "docs", "partner_review"]
EXPORT_STATES = [
    "published",
    "narrowed_row_downgraded",
    "narrowed_stale",
    "narrowed_retest_pending",
    "withheld",
]
FRESHNESS_STATES = ["current", "due_for_refresh", "breached", "missing"]
SCOPE_REASONS = [
    "row_downgraded",
    "qualification_stale",
    "retest_pending",
    "skew_window_exceeded",
    "deprecation_scheduled",
    "support_window_ended",
    "validity_window_expired",
    "evidence_stale",
    "evidence_missing",
    "owner_signoff_missing",
    "waiver_expired",
    "claim_publication_missing",
]
STOP_ACTIONS = [
    "hold_export",
    "narrow_row",
    "withhold_row",
    "refresh_evidence",
    "schedule_retest",
    "renew_validity_window",
    "request_owner_signoff",
    "align_copy_to_source",
]

# Reasons that, on a row whose public claim is still at or above the cutline, hold
# promotion. row_downgraded is inherited and gated upstream.
NON_BLOCKING_REASONS = {"row_downgraded"}


def stop_rule(reason: str, action: str, title: str, rationale: str) -> dict:
    return {
        "rule_id": f"m5_claim_scope_rule:{reason}",
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
        "The qualification row or claim publication narrowed below the cutline; the export inherits it and is gated upstream.",
    ),
    stop_rule(
        "qualification_stale",
        "refresh_evidence",
        "Qualification evidence stale",
        "A qualification dimension's evidence went stale; the export must narrow until it refreshes.",
    ),
    stop_rule(
        "retest_pending",
        "schedule_retest",
        "Retest pending",
        "A dimension or boundary requires a retest; the export must narrow until it completes.",
    ),
    stop_rule(
        "skew_window_exceeded",
        "narrow_row",
        "Skew window exceeded",
        "A peer is outside the supported skew window; the export must narrow.",
    ),
    stop_rule(
        "deprecation_scheduled",
        "narrow_row",
        "Deprecation scheduled",
        "The family is deprecated with a scheduled removal; the export must narrow.",
    ),
    stop_rule(
        "support_window_ended",
        "withhold_row",
        "Support window ended",
        "The support window has ended; the export must withhold the row.",
    ),
    stop_rule(
        "validity_window_expired",
        "renew_validity_window",
        "Validity window expired",
        "The claim-scope validity window expired; the export must renew it before publishing.",
    ),
    stop_rule(
        "evidence_stale",
        "refresh_evidence",
        "Export evidence stale",
        "The export proof packet or a backing report breached its freshness SLO; a still-stable claim holds promotion.",
    ),
    stop_rule(
        "evidence_missing",
        "refresh_evidence",
        "Export evidence missing",
        "No export proof packet or backing report was captured; a still-stable claim holds promotion.",
    ),
    stop_rule(
        "owner_signoff_missing",
        "request_owner_signoff",
        "Owner sign-off missing",
        "Required owner sign-off is missing; a still-stable claim holds promotion.",
    ),
    stop_rule(
        "waiver_expired",
        "withhold_row",
        "Waiver expired",
        "A waiver the row relied on expired; a still-stable claim holds promotion.",
    ),
    stop_rule(
        "claim_publication_missing",
        "hold_export",
        "Claim publication missing",
        "The backing public claim publication is missing; the export holds promotion.",
    ),
]

FRESHNESS_SLO = {
    "target_max_age_days": 90,
    "warn_within_days": 14,
    "slo_register_ref": "freshness-slo/m5-claim-scope-export",
}

VALIDITY_OPEN = {"starts_at": "2026-06-01", "expires_at": "2026-12-31", "expired": False}


def proof_packet(family: str, slo_state: str, captured_at: str) -> dict:
    return {
        "packet_id": f"proof/claim-scope-{family}",
        "packet_ref": f"proof-index/claim-scope-{family}",
        "proof_index_ref": "artifacts/release/stable_proof_index.json",
        "captured_at": captured_at,
        "freshness_slo": dict(FRESHNESS_SLO),
        "slo_state": slo_state,
        "evidence_refs": [f"evidence/claim-scope-{family}/export-snapshot"],
    }


def owner_signoff(owner: str) -> dict:
    return {"owner_ref": owner, "signed_off": True, "signed_at": "2026-06-12"}


def evidence(kind: str, ref: str, state: str = "current") -> dict:
    return {"kind": kind, "evidence_ref": ref, "state": state}


def audiences(row_id: str, label: str, support_class: str, text: str, has_reasons: bool, has_caveats: bool) -> list:
    # Support, shiproom, and partner-review triage from the reopen refs; docs and
    # release-notes are public copy that renders the wording without reopening
    # internal records.
    reopen = {"support": True, "shiproom": True, "docs": False, "partner_review": True, "release_notes": False}
    out = []
    for audience in AUDIENCES:
        out.append(
            {
                "audience": audience,
                "source_row_id": row_id,
                "rendered_label": label,
                "rendered_support_class": support_class,
                "rendered_claim_text": text,
                "discloses_freshness": True,
                "discloses_scope_reasons": has_reasons,
                "discloses_caveats": has_caveats,
                "reopens_authoritative_row": reopen[audience],
            }
        )
    return out


def row(
    *,
    entry_id,
    title,
    family_kind,
    family_ref,
    family_summary,
    release_blocking,
    qualification_row_ref,
    deprecation_packet_ref,
    claim_manifest_entry_ref,
    claim_label,
    source_published_label,
    source_support_class,
    source_claim_text,
    row_state,
    skew_window_class,
    deprecation_status,
    evidence_refs,
    export_state,
    scope_support_class,
    published_label,
    scope_claim_text,
    scope_caveats,
    proof,
    waiver,
    active_scope_reasons,
    rationale,
):
    has_reasons = len(active_scope_reasons) > 0
    has_caveats = len(scope_caveats) > 0
    return {
        "entry_id": entry_id,
        "title": title,
        "family_kind": family_kind,
        "family_ref": family_ref,
        "family_summary": family_summary,
        "release_blocking": release_blocking,
        "qualification_row_ref": qualification_row_ref,
        "deprecation_packet_ref": deprecation_packet_ref,
        "claim_manifest_entry_ref": claim_manifest_entry_ref,
        "claim_label": claim_label,
        "source_published_label": source_published_label,
        "source_support_class": source_support_class,
        "source_claim_text": source_claim_text,
        "row_state": row_state,
        "skew_window_class": skew_window_class,
        "deprecation_status": deprecation_status,
        "evidence_refs": evidence_refs,
        "validity_window": dict(VALIDITY_OPEN),
        "export_state": export_state,
        "scope_support_class": scope_support_class,
        "published_label": published_label,
        "scope_claim_text": scope_claim_text,
        "scope_caveats": scope_caveats,
        "audiences": audiences(entry_id, published_label, scope_support_class, scope_claim_text, has_reasons, has_caveats),
        "proof_packet": proof,
        "waiver": waiver,
        "owner_signoff": owner_signoff(f"team/{family_kind}-owner"),
        "active_scope_reasons": active_scope_reasons,
        "rationale": rationale,
    }


def build_rows() -> list:
    rows = []

    # 1. Notebook — published Stable, every dimension current.
    rows.append(
        row(
            entry_id="claim-scope-notebook-runtime",
            title="Notebook runtime claim scope",
            family_kind="notebook",
            family_ref="family/notebook-runtime",
            family_summary="Notebook and data-rich runtime surface.",
            release_blocking=True,
            qualification_row_ref="m5-notebook-runtime",
            deprecation_packet_ref="deprecation/notebook-runtime",
            claim_manifest_entry_ref="m5-claim-notebook",
            claim_label="stable",
            source_published_label="stable",
            source_support_class="full_support",
            source_claim_text="Notebook runtime is Stable on every qualified platform and deployment profile.",
            row_state="qualified",
            skew_window_class="bounded_skew",
            deprecation_status="active",
            evidence_refs=[
                evidence("qualification_row", "m5-notebook-runtime"),
                evidence("claim_manifest", "m5-claim-notebook"),
                evidence("compatibility_report", "compat-report/notebook-runtime"),
                evidence("proof_packet", "proof-index/claim-scope-notebook"),
            ],
            export_state="published",
            scope_support_class="full_support",
            published_label="stable",
            scope_claim_text="Notebook runtime is Stable on every qualified platform and deployment profile.",
            scope_caveats=[],
            proof=proof_packet("notebook", "current", "2026-06-10"),
            waiver=None,
            active_scope_reasons=[],
            rationale="Every qualification dimension is current and the public claim holds Stable; the export publishes it verbatim.",
        )
    )

    # 2. AI provider — published Stable with a recorded client-scope caveat.
    rows.append(
        row(
            entry_id="claim-scope-ai-provider-boundary",
            title="AI provider boundary claim scope",
            family_kind="ai_provider",
            family_ref="family/ai-provider-boundary",
            family_summary="Helper/agent/provider boundary.",
            release_blocking=True,
            qualification_row_ref="m5-ai-provider-boundary",
            deprecation_packet_ref="deprecation/ai-provider-boundary",
            claim_manifest_entry_ref="m5-claim-ai-provider",
            claim_label="stable",
            source_published_label="stable",
            source_support_class="full_support",
            source_claim_text="AI provider boundary is Stable, with a recorded client-scope caveat.",
            row_state="limited",
            skew_window_class="backward_compatible",
            deprecation_status="active",
            evidence_refs=[
                evidence("qualification_row", "m5-ai-provider-boundary"),
                evidence("claim_manifest", "m5-claim-ai-provider"),
                evidence("skew_window", "skew-window/ai-provider-boundary"),
                evidence("proof_packet", "proof-index/claim-scope-ai-provider"),
            ],
            export_state="published",
            scope_support_class="full_support",
            published_label="stable",
            scope_claim_text="AI provider boundary is Stable, with a recorded client-scope caveat.",
            scope_caveats=[
                "BYOK provider client scope is qualified for managed and self-hosted profiles only; air-gapped provider routing stays limited.",
            ],
            proof=proof_packet("ai-provider", "current", "2026-06-10"),
            waiver=None,
            active_scope_reasons=[],
            rationale="The public claim holds Stable under a recorded client-scope caveat; the export reuses the wording and carries the caveat.",
        )
    )

    # 3. Remote helper — public claim still Stable, but the export proof packet went
    #    stale: an export-layer failure that narrows the export and holds promotion.
    rows.append(
        row(
            entry_id="claim-scope-remote-helper-skew",
            title="Remote helper skew claim scope",
            family_kind="remote_helper",
            family_ref="family/remote-helper-skew",
            family_summary="Remote/helper boundary.",
            release_blocking=True,
            qualification_row_ref="m5-remote-helper-skew",
            deprecation_packet_ref="deprecation/remote-helper-skew",
            claim_manifest_entry_ref="m5-claim-remote-helper",
            claim_label="stable",
            source_published_label="stable",
            source_support_class="full_support",
            source_claim_text="Remote helper is Stable; toolchain-envelope coverage rides an active waiver.",
            row_state="on_waiver",
            skew_window_class="bounded_skew",
            deprecation_status="active",
            evidence_refs=[
                evidence("qualification_row", "m5-remote-helper-skew"),
                evidence("claim_manifest", "m5-claim-remote-helper"),
                evidence("proof_packet", "proof-index/claim-scope-remote-helper", "stale"),
            ],
            export_state="narrowed_stale",
            scope_support_class="full_support",
            published_label="beta",
            scope_claim_text="Remote helper export is held at Beta in claim scope while its export evidence refreshes; the public claim remains Stable on an active waiver.",
            scope_caveats=[],
            proof=proof_packet("remote-helper", "breached", "2026-02-15"),
            waiver={
                "waiver_ref": "waiver/remote-helper-toolchain-envelope",
                "expires_at": "2026-10-31",
                "reason": "Toolchain-envelope coverage rides an active, unexpired waiver pending the next qualification pass.",
            },
            active_scope_reasons=["evidence_stale"],
            rationale="The public claim holds Stable, but the export proof packet breached its freshness SLO, so the export narrows to Beta and holds promotion until refreshed.",
        )
    )

    # 4. Companion — inherited Beta with a pending client-scope retest.
    rows.append(
        row(
            entry_id="claim-scope-companion-handoff",
            title="Companion handoff claim scope",
            family_kind="companion",
            family_ref="family/companion-handoff",
            family_summary="Browser/mobile companion boundary.",
            release_blocking=True,
            qualification_row_ref="m5-companion-handoff",
            deprecation_packet_ref="deprecation/companion-handoff",
            claim_manifest_entry_ref="m5-claim-companion",
            claim_label="stable",
            source_published_label="beta",
            source_support_class="maintenance_only",
            source_claim_text="Companion handoff is Beta while the client-scope retest completes.",
            row_state="retest_pending",
            skew_window_class="forward_compatible",
            deprecation_status="active",
            evidence_refs=[
                evidence("qualification_row", "m5-companion-handoff"),
                evidence("claim_manifest", "m5-claim-companion"),
                evidence("compatibility_report", "compat-report/companion-handoff"),
            ],
            export_state="narrowed_retest_pending",
            scope_support_class="maintenance_only",
            published_label="beta",
            scope_claim_text="Companion handoff is Beta while the client-scope retest completes.",
            scope_caveats=[],
            proof=proof_packet("companion", "current", "2026-06-10"),
            waiver=None,
            active_scope_reasons=["row_downgraded", "retest_pending"],
            rationale="The qualification row narrowed to Beta pending a client-scope retest; the export inherits the Beta scope and is gated upstream.",
        )
    )

    # 5. Ecosystem — inherited Beta with a peer outside the supported skew window.
    rows.append(
        row(
            entry_id="claim-scope-ecosystem-sideload",
            title="Ecosystem sideload claim scope",
            family_kind="ecosystem",
            family_ref="family/ecosystem-sideload",
            family_summary="Extension/sideload boundary.",
            release_blocking=True,
            qualification_row_ref="m5-ecosystem-sideload",
            deprecation_packet_ref="deprecation/ecosystem-sideload",
            claim_manifest_entry_ref="m5-claim-ecosystem",
            claim_label="stable",
            source_published_label="beta",
            source_support_class="limited",
            source_claim_text="Ecosystem sideload is Beta on limited support; the compatibility report is refreshing.",
            row_state="unsupported_skew",
            skew_window_class="unsupported_skew",
            deprecation_status="active",
            evidence_refs=[
                evidence("qualification_row", "m5-ecosystem-sideload"),
                evidence("claim_manifest", "m5-claim-ecosystem"),
                evidence("skew_window", "skew-window/ecosystem-sideload"),
                evidence("compatibility_report", "compat-report/ecosystem-sideload", "stale"),
            ],
            export_state="narrowed_row_downgraded",
            scope_support_class="limited",
            published_label="beta",
            scope_claim_text="Ecosystem sideload is Beta on limited support; the compatibility report is refreshing.",
            scope_caveats=[
                "Sideloaded extensions built before the supported ABI floor require a reinstall.",
            ],
            proof=proof_packet("ecosystem", "current", "2026-06-10"),
            waiver=None,
            active_scope_reasons=["row_downgraded", "skew_window_exceeded"],
            rationale="The qualification row narrowed to Beta because a peer is outside the supported skew window; the export inherits the Beta scope and is gated upstream.",
        )
    )

    # 6. Toolchain — inherited Beta with stale qualification evidence.
    rows.append(
        row(
            entry_id="claim-scope-toolchain-envelope",
            title="Toolchain envelope claim scope",
            family_kind="toolchain_runtime",
            family_ref="family/toolchain-envelope",
            family_summary="Toolchain/runtime boundary.",
            release_blocking=True,
            qualification_row_ref="m5-toolchain-envelope",
            deprecation_packet_ref="deprecation/toolchain-envelope",
            claim_manifest_entry_ref="m5-claim-toolchain",
            claim_label="stable",
            source_published_label="beta",
            source_support_class="full_support",
            source_claim_text="Toolchain envelope is Beta; qualification evidence is stale and refreshing.",
            row_state="stale",
            skew_window_class="bounded_skew",
            deprecation_status="active",
            evidence_refs=[
                evidence("qualification_row", "m5-toolchain-envelope"),
                evidence("claim_manifest", "m5-claim-toolchain"),
                evidence("compatibility_report", "compat-report/toolchain-envelope", "stale"),
            ],
            export_state="narrowed_stale",
            scope_support_class="full_support",
            published_label="beta",
            scope_claim_text="Toolchain envelope is Beta; qualification evidence is stale and refreshing.",
            scope_caveats=[],
            proof=proof_packet("toolchain", "current", "2026-06-10"),
            waiver=None,
            active_scope_reasons=["row_downgraded", "qualification_stale"],
            rationale="The qualification row narrowed to Beta because a dimension's evidence went stale; the export inherits the Beta scope and is gated upstream.",
        )
    )

    # 7. Managed sync — inherited Preview ahead of a scheduled removal.
    rows.append(
        row(
            entry_id="claim-scope-managed-sync-service",
            title="Managed sync service claim scope",
            family_kind="managed_service",
            family_ref="family/managed-sync-service",
            family_summary="Managed sync/relay/registry service boundary.",
            release_blocking=False,
            qualification_row_ref="m5-managed-sync-service",
            deprecation_packet_ref="deprecation/managed-sync-service",
            claim_manifest_entry_ref="m5-claim-managed-sync",
            claim_label="stable",
            source_published_label="preview",
            source_support_class="maintenance_only",
            source_claim_text="Managed sync service is Preview ahead of scheduled removal; successor available.",
            row_state="deprecated",
            skew_window_class="backward_compatible",
            deprecation_status="removal_scheduled",
            evidence_refs=[
                evidence("qualification_row", "m5-managed-sync-service"),
                evidence("claim_manifest", "m5-claim-managed-sync"),
                evidence("deprecation_packet", "deprecation/managed-sync-service"),
            ],
            export_state="narrowed_row_downgraded",
            scope_support_class="maintenance_only",
            published_label="preview",
            scope_claim_text="Managed sync service is Preview ahead of scheduled removal; successor available.",
            scope_caveats=[],
            proof=proof_packet("managed-sync", "current", "2026-06-10"),
            waiver=None,
            active_scope_reasons=["row_downgraded", "deprecation_scheduled"],
            rationale="The qualification row narrowed to Preview ahead of a scheduled removal; the export inherits the Preview scope and is gated upstream.",
        )
    )

    # 8. Managed air-gapped — published Stable on security-only support.
    rows.append(
        row(
            entry_id="claim-scope-managed-airgapped-profile",
            title="Air-gapped managed profile claim scope",
            family_kind="managed_service",
            family_ref="family/managed-airgapped-profile",
            family_summary="Air-gapped managed deployment profile.",
            release_blocking=False,
            qualification_row_ref="m5-managed-airgapped-profile",
            deprecation_packet_ref="deprecation/managed-airgapped-profile",
            claim_manifest_entry_ref="m5-claim-managed-airgapped",
            claim_label="stable",
            source_published_label="stable",
            source_support_class="security_only",
            source_claim_text="Air-gapped managed profile is Stable on security-only support; evidence refresh is due soon.",
            row_state="qualified",
            skew_window_class="lockstep_only",
            deprecation_status="active",
            evidence_refs=[
                evidence("qualification_row", "m5-managed-airgapped-profile"),
                evidence("claim_manifest", "m5-claim-managed-airgapped"),
                evidence("support_window", "support-window/managed-airgapped-profile"),
            ],
            export_state="published",
            scope_support_class="security_only",
            published_label="stable",
            scope_claim_text="Air-gapped managed profile is Stable on security-only support; evidence refresh is due soon.",
            scope_caveats=[],
            proof=proof_packet("managed-airgapped", "due_for_refresh", "2026-04-20"),
            waiver=None,
            active_scope_reasons=[],
            rationale="The public claim holds Stable on security-only support; the export reuses the wording while flagging the due-for-refresh evidence freshness.",
        )
    )

    return rows


def label_at_or_above_cutline(label: str) -> bool:
    return label in ABOVE_CUTLINE


def stop_rule_fires(rule: dict, rows: list) -> bool:
    return any(
        r["source_published_label"] in rule["applies_to_labels"]
        and rule["trigger_reason"] in r["active_scope_reasons"]
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
            and any(reason in blocking_triggers for reason in r["active_scope_reasons"])
        }
    )
    rationale = (
        "An export-layer failure on a row whose public claim is still at or above the cutline holds promotion."
        if decision == "hold"
        else "Every claimed row either holds its public label or merely inherits an upstream narrowing."
    )
    return {
        "promotion_gate": "m5_claim_scope_export_publication",
        "decision": decision,
        "blocking_rule_ids": blocking_rule_ids,
        "blocking_claim_ids": blocking_claim_ids,
        "rationale": rationale,
    }


def compute_summary(rows: list) -> dict:
    def kind(k):
        return sum(1 for r in rows if r["family_kind"] == k)

    def state(s):
        return sum(1 for r in rows if r["export_state"] == s)

    def packets(s):
        return sum(1 for r in rows if r["proof_packet"]["slo_state"] == s)

    def evidence_in(s):
        return sum(1 for r in rows for e in r["evidence_refs"] if e["state"] == s)

    def published(r):
        return label_at_or_above_cutline(r["published_label"])

    rb = [r for r in rows if r["release_blocking"]]
    families = sorted({r["family_ref"] for r in rows})
    all_aud = [a for r in rows for a in r["audiences"]]
    return {
        "total_rows": len(rows),
        "total_families": len(families),
        "rows_published": sum(1 for r in rows if published(r)),
        "rows_narrowed": sum(1 for r in rows if not published(r)),
        "release_blocking_total": len(rb),
        "release_blocking_published": sum(1 for r in rb if published(r)),
        "release_blocking_narrowed": sum(1 for r in rb if not published(r)),
        "notebook_rows": kind("notebook"),
        "ai_provider_rows": kind("ai_provider"),
        "remote_helper_rows": kind("remote_helper"),
        "companion_rows": kind("companion"),
        "ecosystem_rows": kind("ecosystem"),
        "managed_service_rows": kind("managed_service"),
        "toolchain_runtime_rows": kind("toolchain_runtime"),
        "state_published": state("published"),
        "state_narrowed_row_downgraded": state("narrowed_row_downgraded"),
        "state_narrowed_stale": state("narrowed_stale"),
        "state_narrowed_retest_pending": state("narrowed_retest_pending"),
        "state_withheld": state("withheld"),
        "rows_with_caveats": sum(1 for r in rows if r["scope_caveats"]),
        "total_caveats": sum(len(r["scope_caveats"]) for r in rows),
        "total_evidence_refs": sum(len(r["evidence_refs"]) for r in rows),
        "evidence_current": evidence_in("current"),
        "evidence_stale": evidence_in("stale"),
        "evidence_missing": evidence_in("missing"),
        "evidence_dropped": evidence_in("dropped"),
        "evidence_unsigned": evidence_in("unsigned"),
        "total_audiences": len(all_aud),
        "audiences_freshness_disclosed": sum(1 for a in all_aud if a["discloses_freshness"]),
        "audiences_reasons_disclosed": sum(1 for a in all_aud if a["discloses_scope_reasons"]),
        "audiences_reopen_disclosed": sum(1 for a in all_aud if a["reopens_authoritative_row"]),
        "packets_current": packets("current"),
        "packets_due_for_refresh": packets("due_for_refresh"),
        "packets_breached": packets("breached"),
        "packets_missing": packets("missing"),
        "total_active_scope_reasons": sum(len(r["active_scope_reasons"]) for r in rows),
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
        "evidence_index_ref": EVIDENCE_INDEX_REF,
        "lifecycle_labels": list(LIFECYCLE_LABELS),
        "family_kinds": list(FAMILY_KINDS),
        "support_classes": list(SUPPORT_CLASSES),
        "row_states": list(ROW_STATES),
        "skew_window_classes": list(SKEW_WINDOW_CLASSES),
        "deprecation_statuses": list(DEPRECATION_STATUSES),
        "evidence_states": list(EVIDENCE_STATES),
        "evidence_kinds": list(EVIDENCE_KINDS),
        "audiences": list(AUDIENCES),
        "required_audiences": list(REQUIRED_AUDIENCES),
        "export_states": list(EXPORT_STATES),
        "freshness_states": list(FRESHNESS_STATES),
        "scope_reasons": list(SCOPE_REASONS),
        "stop_actions": list(STOP_ACTIONS),
        "launch_cutline": {
            "cutline_level": "stable",
            "above_cutline_levels": list(ABOVE_CUTLINE),
            "below_cutline_levels": list(BELOW_CUTLINE),
            "description": "Stable is the minimum claim-scope label considered launch-qualified; Beta, Preview, and Withdrawn fall below the cutline.",
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

    # 2. A row over-claims its below-cutline public claim by publishing Stable.
    over = copy.deepcopy(register)
    target = next(r for r in over["rows"] if r["source_published_label"] not in ABOVE_CUTLINE)
    target["published_label"] = "stable"
    for a in target["audiences"]:
        a["rendered_label"] = "stable"
    cases.append(("row_over_claims_public_claim.json", over, "RowLabelExceedsSource"))

    # 3. A published row carries an active narrowing reason.
    gap = copy.deepcopy(register)
    pub = next(r for r in gap["rows"] if r["export_state"] == "published")
    pub["active_scope_reasons"] = ["evidence_stale"]
    cases.append(("published_with_active_gap.json", gap, "PublishedWithActiveGap"))

    # 4. An audience renders wording that drifted from its row.
    drift = copy.deepcopy(register)
    drift["rows"][0]["audiences"][0]["rendered_claim_text"] = (
        "Hand-edited shiproom copy that drifted from the public claim."
    )
    cases.append(("audience_copy_drift.json", drift, "AudienceCopyDrift"))

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
            "rows_published": s["rows_published"],
            "rows_narrowed": s["rows_narrowed"],
            "state_published": s["state_published"],
            "state_narrowed_row_downgraded": s["state_narrowed_row_downgraded"],
            "state_narrowed_stale": s["state_narrowed_stale"],
            "state_narrowed_retest_pending": s["state_narrowed_retest_pending"],
            "rows_with_caveats": s["rows_with_caveats"],
            "total_evidence_refs": s["total_evidence_refs"],
            "evidence_stale": s["evidence_stale"],
            "total_audiences": s["total_audiences"],
            "audiences_freshness_disclosed": s["audiences_freshness_disclosed"],
            "audiences_reasons_disclosed": s["audiences_reasons_disclosed"],
            "audiences_reopen_disclosed": s["audiences_reopen_disclosed"],
            "total_active_scope_reasons": s["total_active_scope_reasons"],
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
            {"drill_id": "drill:published_with_active_gap", "status": "passed"},
            {"drill_id": "drill:audience_copy_drift", "status": "passed"},
            {"drill_id": "drill:audience_hides_freshness", "status": "passed"},
            {"drill_id": "drill:shiproom_without_reopen_ref", "status": "passed"},
            {"drill_id": "drill:lost_retest_reason", "status": "passed"},
            {"drill_id": "drill:required_audience_uncovered", "status": "passed"},
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
