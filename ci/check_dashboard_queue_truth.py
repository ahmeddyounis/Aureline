#!/usr/bin/env python3
"""Validate the operator dashboard & queue truth audit corpus.

This is the deterministic audit lane for the marketed beta operator dashboards
and queues. It consumes the freshness/order/evidence truth records that the
desktop shell, CLI / headless inspect, diagnostics, and support exports all read
(`dashboard_truth_view_record`, see
crates/aureline-shell/src/dashboard_truth/model.rs), and proves the quiet ways a
dashboard or queue can lie are caught before they ship:

- **Stale green.** A row keeps an all-clear headline after its freshness or its
  last-successful evidence expired.
- **Unexplained order / silent narrowing.** A queue sorts rows with no stated
  reason, or hides rows without disclosing how many or why.
- **Broken evidence routing.** An open-details / inspect-evidence / reveal path
  points at a generic landing page instead of the durable object behind the row.
- **Export drift.** A support bundle or CLI / headless projection loses the
  freshness / order / hidden semantics the product UI shows.

It validates two corpora that together cover every marketed surface plus the two
audit drills this lane adds:

1. The Rust-pinned runtime fixtures under
   `fixtures/ops/m3/dashboard_and_queue_truth/` (minted by the shell emitter and
   replay-pinned bit-for-bit in
   `crates/aureline-shell/tests/dashboard_truth_fixtures.rs`).
2. The audit drills under `fixtures/ops/m3/dashboard_queue_truth_corpus/`:
   `review_inbox_order_ambiguity` (risk vs time vs owner ordering is
   disambiguated per row) and `support_queue_restart_evidence_break` (a row
   whose evidence link broke after restart / reconnect cannot stay green and
   keeps a canonical reopen path).

For **every** view fixture the validator independently re-derives the
no-silent-green rule, the evidence-age buckets, the downgrade reasons, the
summary roll-ups, and the queue order / hidden-scope projection from the raw
inputs, and asserts the stored record matches. The derivation is a second
implementation of the model, so a regression in either the model or a fixture
fails the lane instead of shipping silently.

The audit corpus ships two governed companions, both regenerated and
drift-checked here:

- `corpus_matrix.json` — the enum-only matrix joining every scenario to its
  surface, roll-ups, order / narrowing vocabulary, and the lane properties it
  proves.
- `export_parity_packet.json` — per scenario, the support-export plaintext and
  CLI index projections plus the semantic digest each one must preserve, so a
  support bundle and a headless inspect cannot drift from the product UI.

Run via scripts/ci/run_dashboard_queue_truth.sh. Use --write to regenerate the
audit fixtures, the matrix, and the parity packet.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any

from jsonschema import Draft202012Validator, FormatChecker

try:
    from referencing import Registry, Resource

    _HAVE_REFERENCING = True
except ImportError:  # pragma: no cover - referencing ships with jsonschema 4.18+
    _HAVE_REFERENCING = False


# --------------------------------------------------------------------------- #
# Paths
# --------------------------------------------------------------------------- #

CARD_SCHEMA_REL = "schemas/ops/dashboard_freshness_card.schema.json"
QUEUE_SCHEMA_REL = "schemas/ops/queue_order_reason.schema.json"
RUNTIME_FIXTURE_DIR_REL = "fixtures/ops/m3/dashboard_and_queue_truth"
AUDIT_FIXTURE_DIR_REL = "fixtures/ops/m3/dashboard_queue_truth_corpus"
CORPUS_MATRIX_REL = "fixtures/ops/m3/dashboard_queue_truth_corpus/corpus_matrix.json"
PARITY_PACKET_REL = (
    "fixtures/ops/m3/dashboard_queue_truth_corpus/export_parity_packet.json"
)
README_REL = "fixtures/ops/m3/dashboard_queue_truth_corpus/README.md"
REPORT_REL = "artifacts/ops/m3/dashboard_queue_truth_report.md"
VALIDATOR_REL = "ci/check_dashboard_queue_truth.py"
SCRIPT_REL = "scripts/ci/run_dashboard_queue_truth.sh"

CORPUS_MATRIX_RECORD_KIND = "dashboard_queue_truth_corpus_matrix"
PARITY_PACKET_RECORD_KIND = "dashboard_queue_truth_export_parity_packet"
AUDIT_CONTRACT_REF = "ops:dashboard_queue_truth_corpus:v1"

# Files in either fixture dir that are companions, not view records.
NON_VIEW_FILES = {"corpus_matrix.json", "export_parity_packet.json", "README.md"}


# --------------------------------------------------------------------------- #
# Model vocabulary — a deliberate second implementation of
# crates/aureline-shell/src/dashboard_truth/model.rs. Keep these in lockstep
# with the Rust enums; the cargo cross-check in the shell script ties them to the
# product when a toolchain is present.
# --------------------------------------------------------------------------- #

VIEW_RECORD_KIND = "dashboard_truth_view_record"
CARD_RECORD_KIND = "dashboard_freshness_card_record"
QUEUE_RECORD_KIND = "queue_order_reason_record"
VIEW_SCHEMA_VERSION = 1
CARD_SCHEMA_VERSION = 1
QUEUE_SCHEMA_VERSION = 1

NOTICE = (
    "Dashboard & queue truth: every row carries a freshness class, the age of "
    "the last-successful evidence, and the canonical object its open-details "
    "path routes to; a row may render its all-clear headline only while it is "
    "fresh and evidence-current — otherwise it downgrades and names why. Queue "
    "rows carry an order reason and the hidden-by-scope counts. Shell, CLI / "
    "headless inspect, diagnostics, and support exports read this record "
    "verbatim — surface-local status copy and silent green are not admitted."
)

CANONICAL_SCHEME = "aureline://"
GENERIC_LANDING_CLASSES = {
    "home",
    "dashboard",
    "landing",
    "index",
    "overview",
    "start",
    "root",
}
MAX_REF_CHARS = 200
MAX_TITLE_CHARS = 120
MAX_EXPLANATION_CHARS = 240

SURFACE_LABEL = {
    "service_health": "Service health",
    "review_inbox": "Review inbox",
    "incident_queue": "Incident queue",
    "support_queue": "Support queue",
    "admin_queue": "Admin queue",
}
QUEUE_SURFACES = {"review_inbox", "incident_queue", "support_queue", "admin_queue"}

DISPLAYED_LABEL = {"clear": "Clear", "attention": "Needs attention", "blocked": "Blocked"}
EFFECTIVE_LABEL = {
    "clear": "Clear",
    "unconfirmed": "Unconfirmed — evidence not current",
    "attention": "Needs attention",
    "blocked": "Blocked",
}
EFFECTIVE_SEVERITY = {"clear": 0, "attention": 1, "unconfirmed": 2, "blocked": 3}

FRESHNESS_LABEL = {
    "fresh": "Fresh",
    "cached": "Cached",
    "stale": "Stale",
    "partial": "Partial",
    "policy_blocked": "Policy blocked",
    "unavailable": "Unavailable",
}
FRESHNESS_SEVERITY = {
    "fresh": 0,
    "cached": 1,
    "stale": 2,
    "partial": 3,
    "policy_blocked": 4,
    "unavailable": 5,
}

EVIDENCE_AGE_LABEL = {
    "fresh": "Just now",
    "recent": "Within the hour",
    "stale": "Hours ago",
    "very_stale": "More than a day ago",
    "never": "No evidence yet",
}
EVIDENCE_AGE_CURRENT = {"fresh", "recent"}
EVIDENCE_AGE_WARNING = {"stale", "very_stale", "never"}

EVIDENCE_KIND_LABEL = {
    "service_health_card": "Service-health card",
    "change_review": "Change review",
    "incident_record": "Incident record",
    "support_case": "Support case",
    "audit_entry": "Audit entry",
    "runbook_packet": "Runbook packet",
    "policy_decision": "Policy decision",
}

DOWNGRADE_LABEL = {
    "cached_fallback": "Serving cached data",
    "freshness_expired": "Freshness expired",
    "evidence_aged_out": "Evidence aged out",
    "source_partial": "Partial data",
    "policy_blocked": "Policy blocked",
    "source_offline": "Source offline",
}
# Declaration order of DowngradeReasonClass — Rust `.sort()` follows this, not
# alphabetical order.
DOWNGRADE_RANK = {
    "cached_fallback": 0,
    "freshness_expired": 1,
    "evidence_aged_out": 2,
    "source_partial": 3,
    "policy_blocked": 4,
    "source_offline": 5,
}
FRESHNESS_DOWNGRADE_REASON = {
    "cached": "cached_fallback",
    "stale": "freshness_expired",
    "partial": "source_partial",
    "policy_blocked": "policy_blocked",
    "unavailable": "source_offline",
}

ORDER_REASON_LABEL = {
    "severity_descending": "Most severe first",
    "sla_deadline": "Deadline soonest first",
    "oldest_unresolved_first": "Oldest unresolved first",
    "recently_updated": "Recently updated",
    "assigned_to_you": "Assigned to you",
    "blocking_dependency": "Blocking other work",
    "manual_pin": "Pinned",
    "default_recency": "Default order",
}
# Which ordering principle each order reason expresses. The order-ambiguity drill
# proves a queue that could be sorted by risk, by time, or by owner discloses
# which principle placed each row.
ORDER_PRINCIPLE = {
    "severity_descending": "risk",
    "blocking_dependency": "risk",
    "sla_deadline": "time",
    "oldest_unresolved_first": "time",
    "recently_updated": "time",
    "assigned_to_you": "owner",
    "manual_pin": "manual",
    "default_recency": "default",
}

NARROWING_LABEL = {
    "scope_filter": "Hidden by scope filter",
    "policy_scope": "Hidden by policy scope",
    "assignee_filter": "Hidden by assignee filter",
    "resolved_hidden": "Resolved items hidden",
    "archived_hidden": "Archived items hidden",
    "severity_filter": "Below severity filter",
    "offline_partial_list": "List incomplete (offline)",
}
INCOMPLETE_KNOWLEDGE_REASONS = {"policy_scope", "offline_partial_list"}


# --------------------------------------------------------------------------- #
# Findings
# --------------------------------------------------------------------------- #


@dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    remediation: str
    ref: str | None = None
    details: dict[str, Any] = field(default_factory=dict)

    def as_report(self) -> dict[str, Any]:
        payload = asdict(self)
        if payload["ref"] is None:
            payload.pop("ref")
        if not payload["details"]:
            payload.pop("details")
        return payload


def err(check_id: str, message: str, remediation: str, ref: str | None = None,
        **details: Any) -> Finding:
    return Finding("error", check_id, message, remediation, ref, dict(details))


# --------------------------------------------------------------------------- #
# Timestamp / evidence-age derivation (port of model.rs derive_age)
# --------------------------------------------------------------------------- #


def days_from_civil(y: int, m: int, d: int) -> int:
    y = y - 1 if m <= 2 else y
    era = (y if y >= 0 else y - 399) // 400
    yoe = y - era * 400
    doy = (153 * (m - 3 if m > 2 else m + 9) + 2) // 5 + d - 1
    doe = yoe * 365 + yoe // 4 - yoe // 100 + doy
    return era * 146_097 + doe - 719_468


def parse_timestamp_minutes(value: str) -> int | None:
    if value is None or len(value) < 16:
        return None
    if value[4] != "-" or value[7] != "-":
        return None
    if value[10] not in ("T", " "):
        return None
    if value[13] != ":":
        return None
    try:
        year = int(value[0:4])
        month = int(value[5:7])
        day = int(value[8:10])
        hour = int(value[11:13])
        minute = int(value[14:16])
    except ValueError:
        return None
    if not (1 <= month <= 12) or not (1 <= day <= 31):
        return None
    if hour > 23 or minute > 59:
        return None
    return days_from_civil(year, month, day) * 24 * 60 + hour * 60 + minute


def derive_age(evidence_at: str | None, as_of: str) -> str:
    if evidence_at is None:
        return "never"
    last = parse_timestamp_minutes(evidence_at)
    now = parse_timestamp_minutes(as_of)
    if last is None or now is None or now < last:
        return "never"
    delta = now - last
    if delta <= 5:
        return "fresh"
    if delta <= 60:
        return "recent"
    if delta <= 60 * 24:
        return "stale"
    return "very_stale"


def effective_state(displayed: str, freshness: str, evidence_age: str) -> str:
    if displayed == "clear":
        if freshness == "fresh" and evidence_age in EVIDENCE_AGE_CURRENT:
            return "clear"
        return "unconfirmed"
    return displayed


def downgrade_reasons(freshness: str, evidence_age: str) -> list[str]:
    reasons: list[str] = []
    if freshness != "fresh":
        reasons.append(FRESHNESS_DOWNGRADE_REASON[freshness])
    if evidence_age in EVIDENCE_AGE_WARNING:
        reasons.append("evidence_aged_out")
    reasons = sorted(set(reasons), key=lambda r: DOWNGRADE_RANK[r])
    return reasons


def is_canonical_ref(ref: str) -> bool:
    ref = (ref or "").strip()
    if not ref or len(ref) > MAX_REF_CHARS:
        return False
    if not ref.startswith(CANONICAL_SCHEME):
        return False
    rest = ref[len(CANONICAL_SCHEME):]
    if "/" not in rest:
        return False
    cls, ident = rest.split("/", 1)
    if not cls or not ident:
        return False
    return cls not in GENERIC_LANDING_CLASSES


# --------------------------------------------------------------------------- #
# Builder (port of model.rs DashboardTruthView::build) — produces a record from
# raw inputs. Used to (a) rebuild a stored fixture from its extracted inputs and
# assert equality, and (b) mint the audit drill fixtures in --write mode.
# --------------------------------------------------------------------------- #


def project_card(surface: str, c: dict[str, Any], as_of: str) -> dict[str, Any]:
    displayed = c["displayed_state"]
    freshness = c["freshness"]
    evidence_at = c.get("last_successful_evidence_at")
    age = derive_age(evidence_at, as_of)
    reasons = downgrade_reasons(freshness, age)
    eff = effective_state(displayed, freshness, age)
    green_downgraded = displayed == "clear" and eff == "unconfirmed"
    honesty = eff != "clear" or freshness != "fresh" or age in EVIDENCE_AGE_WARNING
    return {
        "record_kind": CARD_RECORD_KIND,
        "schema_version": CARD_SCHEMA_VERSION,
        "card_id": c["card_id"],
        "surface": surface,
        "surface_token": surface,
        "surface_label": SURFACE_LABEL[surface],
        "title": c["title"].strip(),
        "displayed_state": displayed,
        "displayed_state_token": displayed,
        "displayed_state_label": DISPLAYED_LABEL[displayed],
        "effective_state": eff,
        "effective_state_token": eff,
        "effective_state_label": EFFECTIVE_LABEL[eff],
        "freshness": freshness,
        "freshness_token": freshness,
        "freshness_label": FRESHNESS_LABEL[freshness],
        "last_successful_evidence_at": evidence_at,
        "evidence_age": age,
        "evidence_age_token": age,
        "evidence_age_label": EVIDENCE_AGE_LABEL[age],
        "evidence_kind": c["evidence_kind"],
        "evidence_kind_token": c["evidence_kind"],
        "evidence_kind_label": EVIDENCE_KIND_LABEL[c["evidence_kind"]],
        "evidence_ref": c["evidence_ref"].strip(),
        "downgrade_reasons": reasons,
        "downgrade_reason_tokens": list(reasons),
        "state_explanation": c["state_explanation"].strip(),
        "green_downgraded": green_downgraded,
        "honesty_marker_present": honesty,
    }


def card_sort_key(card: dict[str, Any]) -> tuple[int, int, str]:
    return (
        -EFFECTIVE_SEVERITY[card["effective_state"]],
        -FRESHNESS_SEVERITY[card["freshness"]],
        card["card_id"],
    )


def build_queue_order(
    surface: str, queue: dict[str, Any]
) -> dict[str, Any]:
    rows = []
    for idx, r in enumerate(queue["rows"]):
        rows.append(
            {
                "row_id": r["row_id"],
                "order_rank": idx + 1,
                "order_reason": r["order_reason"],
                "order_reason_token": r["order_reason"],
                "order_reason_label": ORDER_REASON_LABEL[r["order_reason"]],
                "order_explanation": r["order_explanation"].strip(),
                "open_details_ref": r["open_details_ref"].strip(),
            }
        )
    hidden = []
    hidden_total = 0
    for h in queue.get("hidden_scope", []):
        reason = h["narrowing_reason"]
        hidden_total += h["hidden_count"]
        hidden.append(
            {
                "narrowing_reason": reason,
                "narrowing_reason_token": reason,
                "narrowing_reason_label": NARROWING_LABEL[reason],
                "hidden_count": h["hidden_count"],
                "narrowing_explanation": h["narrowing_explanation"].strip(),
                "reveal_ref": h["reveal_ref"].strip(),
                "incomplete_knowledge": reason in INCOMPLETE_KNOWLEDGE_REASONS,
            }
        )
    hidden.sort(key=lambda x: x["narrowing_reason_token"])
    order_reasons_present = sorted({r["order_reason_token"] for r in rows})
    visible = len(rows)
    return {
        "record_kind": QUEUE_RECORD_KIND,
        "schema_version": QUEUE_SCHEMA_VERSION,
        "queue_id": queue["queue_id"].strip(),
        "surface": surface,
        "surface_token": surface,
        "surface_label": SURFACE_LABEL[surface],
        "visible_row_count": visible,
        "hidden_total": hidden_total,
        "total_in_scope_count": visible + hidden_total,
        "rows": rows,
        "order_reasons_present": order_reasons_present,
        "hidden_scope": hidden,
        "narrowing_present": hidden_total > 0,
        "incomplete_knowledge_present": any(h["incomplete_knowledge"] for h in hidden),
    }


def compute_summary(cards: list[dict[str, Any]]) -> dict[str, int]:
    summary = {
        "total_card_count": len(cards),
        "clear_card_count": 0,
        "unconfirmed_card_count": 0,
        "attention_card_count": 0,
        "blocked_card_count": 0,
        "green_downgrade_count": 0,
        "stale_evidence_count": 0,
        "freshness_downgrade_count": 0,
    }
    for c in cards:
        summary[f"{c['effective_state']}_card_count"] += 1
        if c["green_downgraded"]:
            summary["green_downgrade_count"] += 1
        if c["evidence_age"] in EVIDENCE_AGE_WARNING:
            summary["stale_evidence_count"] += 1
        if c["freshness"] != "fresh":
            summary["freshness_downgrade_count"] += 1
    return summary


def build_view(
    view_id: str,
    surface: str,
    as_of: str,
    cards: list[dict[str, Any]],
    queue: dict[str, Any] | None,
) -> dict[str, Any]:
    projected = [project_card(surface, c, as_of) for c in cards]
    projected.sort(key=card_sort_key)
    queue_order = build_queue_order(surface, queue) if queue is not None else None
    summary = compute_summary(projected)
    overall_eff = max(
        (c["effective_state"] for c in projected),
        key=lambda s: EFFECTIVE_SEVERITY[s],
        default="clear",
    )
    overall_fresh = max(
        (c["freshness"] for c in projected),
        key=lambda f: FRESHNESS_SEVERITY[f],
        default="fresh",
    )
    honesty = any(c["honesty_marker_present"] for c in projected) or (
        queue_order is not None and queue_order["narrowing_present"]
    )
    return {
        "record_kind": VIEW_RECORD_KIND,
        "schema_version": VIEW_SCHEMA_VERSION,
        "notice": NOTICE,
        "view_id": view_id,
        "surface": surface,
        "surface_token": surface,
        "surface_label": SURFACE_LABEL[surface],
        "as_of": as_of,
        "cards": projected,
        "queue_order": queue_order,
        "summary": summary,
        "overall_effective_state": overall_eff,
        "overall_effective_state_token": overall_eff,
        "overall_effective_state_label": EFFECTIVE_LABEL[overall_eff],
        "overall_freshness": overall_fresh,
        "overall_freshness_token": overall_fresh,
        "overall_freshness_label": FRESHNESS_LABEL[overall_fresh],
        "honesty_marker_present": honesty,
    }


def extract_inputs(view: dict[str, Any]) -> tuple[list[dict[str, Any]], dict[str, Any] | None]:
    """Recover the raw builder inputs from a stored view record."""
    cards = [
        {
            "card_id": c["card_id"],
            "title": c["title"],
            "displayed_state": c["displayed_state"],
            "freshness": c["freshness"],
            "last_successful_evidence_at": c.get("last_successful_evidence_at"),
            "evidence_kind": c["evidence_kind"],
            "evidence_ref": c["evidence_ref"],
            "state_explanation": c["state_explanation"],
        }
        for c in view["cards"]
    ]
    queue = None
    q = view.get("queue_order")
    if q is not None:
        queue = {
            "queue_id": q["queue_id"],
            "rows": [
                {
                    "row_id": r["row_id"],
                    "order_reason": r["order_reason"],
                    "order_explanation": r["order_explanation"],
                    "open_details_ref": r["open_details_ref"],
                }
                for r in q["rows"]
            ],
            "hidden_scope": [
                {
                    "narrowing_reason": h["narrowing_reason"],
                    "hidden_count": h["hidden_count"],
                    "narrowing_explanation": h["narrowing_explanation"],
                    "reveal_ref": h["reveal_ref"],
                }
                for h in q.get("hidden_scope", [])
            ],
        }
    return cards, queue


# --------------------------------------------------------------------------- #
# Audit drill definitions — the two cases this lane adds on top of the runtime
# corpus. Inputs only; the builder derives the honest record.
# --------------------------------------------------------------------------- #

CORPUS_AS_OF = "2026-05-20T12:00"
FRESH = "2026-05-20T11:58"   # 2 min  -> fresh
RECENT = "2026-05-20T11:20"  # 40 min -> recent
HOURS_AGO = "2026-05-20T06:00"  # 6 h  -> stale
DAY_AGO = "2026-05-19T06:00"    # >24 h -> very_stale


@dataclass
class AuditDrill:
    scenario_id: str
    surface: str
    fixture_filename: str
    narrative: str
    cards: list[dict[str, Any]]
    queue: dict[str, Any] | None

    def view(self) -> dict[str, Any]:
        return build_view(
            f"dashboard_truth_view:m3.beta.corpus.{self.scenario_id}",
            self.surface,
            CORPUS_AS_OF,
            self.cards,
            self.queue,
        )


def _card(card_id, title, displayed, freshness, evidence_at, kind, ref, explanation):
    return {
        "card_id": card_id,
        "title": title,
        "displayed_state": displayed,
        "freshness": freshness,
        "last_successful_evidence_at": evidence_at,
        "evidence_kind": kind,
        "evidence_ref": ref,
        "state_explanation": explanation,
    }


def _row(row_id, reason, explanation, ref):
    return {
        "row_id": row_id,
        "order_reason": reason,
        "order_explanation": explanation,
        "open_details_ref": ref,
    }


def _hidden(reason, count, explanation, ref):
    return {
        "narrowing_reason": reason,
        "hidden_count": count,
        "narrowing_explanation": explanation,
        "reveal_ref": ref,
    }


def audit_drills() -> list[AuditDrill]:
    order_ambiguity = AuditDrill(
        scenario_id="review_inbox_order_ambiguity",
        surface="review_inbox",
        fixture_filename="review_inbox_order_ambiguity.json",
        narrative=(
            "The same inbox could be sorted by risk, by time, or by owner; each "
            "visible row names which principle placed it, so the operator never "
            "has to guess why a blocking review outranks an older one or why an "
            "assigned review jumps the queue."
        ),
        cards=[
            _card(
                "card:cr-risk",
                "Blocking review holds the merge train",
                "blocked",
                "fresh",
                FRESH,
                "change_review",
                "aureline://change_review/cr-7001",
                "Blocks dependent merges; sorted by risk ahead of time and owner order.",
            ),
            _card(
                "card:cr-owner",
                "Review assigned to you",
                "attention",
                "fresh",
                RECENT,
                "change_review",
                "aureline://change_review/cr-7002",
                "Assigned to you; raised above plain time order so owner work is not buried.",
            ),
            _card(
                "card:cr-time",
                "Oldest unresolved review",
                "attention",
                "fresh",
                RECENT,
                "change_review",
                "aureline://change_review/cr-7003",
                "Oldest unresolved review; placed by time once risk and owner rows are shown.",
            ),
            _card(
                "card:cr-default",
                "Recently opened review",
                "clear",
                "fresh",
                FRESH,
                "change_review",
                "aureline://change_review/cr-7004",
                "No stronger reason than recency; current and clear.",
            ),
        ],
        queue={
            "queue_id": "queue:review_inbox_ambiguity",
            "rows": [
                _row(
                    "card:cr-risk",
                    "blocking_dependency",
                    "Sorted first by risk: it blocks dependent merges.",
                    "aureline://change_review/cr-7001",
                ),
                _row(
                    "card:cr-owner",
                    "assigned_to_you",
                    "Sorted by owner: assigned to you, above plain time order.",
                    "aureline://change_review/cr-7002",
                ),
                _row(
                    "card:cr-time",
                    "oldest_unresolved_first",
                    "Sorted by time: oldest unresolved once risk and owner are placed.",
                    "aureline://change_review/cr-7003",
                ),
                _row(
                    "card:cr-default",
                    "default_recency",
                    "Default recency order; no risk, owner, or deadline reason applied.",
                    "aureline://change_review/cr-7004",
                ),
            ],
            "hidden_scope": [
                _hidden(
                    "scope_filter",
                    4,
                    "4 reviews are outside the active workspace scope.",
                    "aureline://change_review_query/all_scopes",
                ),
                _hidden(
                    "assignee_filter",
                    2,
                    "2 reviews are hidden by the assigned-to-me filter.",
                    "aureline://change_review_query/all_assignees",
                ),
            ],
        },
    )

    restart_break = AuditDrill(
        scenario_id="support_queue_restart_evidence_break",
        surface="support_queue",
        fixture_filename="support_queue_restart_evidence_break.json",
        narrative=(
            "After a restart and a reconnect, a case restored from a snapshot with "
            "a broken evidence link and a case whose last evidence aged out both "
            "lose their green headline, while a re-confirmed case goes clear again; "
            "every row keeps a canonical reopen path even when its evidence is not "
            "current."
        ),
        cards=[
            _card(
                "card:sc-restart-never",
                "Case restored from snapshot, evidence link lost",
                "clear",
                "unavailable",
                None,
                "support_case",
                "aureline://support_case/sc-9001",
                "Restored from a snapshot after restart; the evidence link broke and no re-fetch has confirmed it.",
            ),
            _card(
                "card:sc-reconnect-stale",
                "Case reconnecting, last evidence aged out",
                "clear",
                "stale",
                DAY_AGO,
                "support_case",
                "aureline://support_case/sc-9002",
                "Reconnecting after a dropped session; the last successful evidence aged out and is not current.",
            ),
            _card(
                "card:sc-reconnect-ok",
                "Case re-confirmed after reconnect",
                "clear",
                "fresh",
                FRESH,
                "support_case",
                "aureline://support_case/sc-9003",
                "Reconnected and re-confirmed within the review window.",
            ),
        ],
        queue={
            "queue_id": "queue:support_restart",
            "rows": [
                _row(
                    "card:sc-restart-never",
                    "default_recency",
                    "Default order; restored after restart and awaiting an evidence re-fetch.",
                    "aureline://support_case/sc-9001",
                ),
                _row(
                    "card:sc-reconnect-stale",
                    "oldest_unresolved_first",
                    "Oldest unresolved case while the reconnect re-confirms evidence.",
                    "aureline://support_case/sc-9002",
                ),
                _row(
                    "card:sc-reconnect-ok",
                    "default_recency",
                    "Default recency order; evidence re-confirmed after reconnect.",
                    "aureline://support_case/sc-9003",
                ),
            ],
            "hidden_scope": [
                _hidden(
                    "offline_partial_list",
                    4,
                    "4 cases could not be re-listed after reconnect; the remainder is unknown.",
                    "aureline://support_case_query/retry_full_load",
                ),
            ],
        },
    )

    return [order_ambiguity, restart_break]


# --------------------------------------------------------------------------- #
# Per-view invariant checks (the lane-failing rules)
# --------------------------------------------------------------------------- #


def check_view_invariants(label: str, view: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    surface = view.get("surface")
    if surface not in SURFACE_LABEL:
        return [
            err(
                "dashboard_queue_truth.surface.unknown",
                f"{label}: unknown surface {surface!r}",
                "Use one of the closed dashboard/queue surfaces.",
                label,
            )
        ]
    as_of = view.get("as_of", "")

    # 1. Independent rebuild from extracted inputs must match the stored record.
    #    This is the structural no-silent-green + roll-up + ordering guard: if a
    #    fixture claims a row is green while its freshness/evidence cannot back
    #    it, the rebuilt record disagrees and the lane fails.
    try:
        cards, queue = extract_inputs(view)
        rebuilt = build_view(view["view_id"], surface, as_of, cards, queue)
    except (KeyError, TypeError) as exc:
        return [
            err(
                "dashboard_queue_truth.rebuild.failed",
                f"{label}: could not rebuild view from inputs: {exc}",
                "Ensure the record carries every field the model emits.",
                label,
            )
        ]
    if rebuilt != view:
        findings.append(
            err(
                "dashboard_queue_truth.rebuild.mismatch",
                f"{label}: stored record does not match the model projection of its "
                "inputs (freshness/evidence/order/roll-up drift)",
                "Re-emit the fixture from the shell corpus, or fix the derived "
                "fields so freshness, evidence age, downgrade reasons, roll-ups, "
                "and queue order all follow the no-silent-green model.",
                label,
                diff=_first_diff(view, rebuilt),
            )
        )

    # 2. No silent green — explicit, independent of the rebuild.
    for card in view["cards"]:
        cid = card.get("card_id")
        eff = card.get("effective_state")
        fresh = card.get("freshness")
        age = card.get("evidence_age")
        displayed = card.get("displayed_state")
        if eff == "clear":
            if fresh != "fresh" or age not in EVIDENCE_AGE_CURRENT:
                findings.append(
                    err(
                        "dashboard_queue_truth.green.stale",
                        f"{label} card {cid}: effective_state is clear but "
                        f"freshness={fresh} evidence_age={age} — a green that is "
                        "not fresh and evidence-current",
                        "A row may stay green only while fresh and "
                        "evidence-current; otherwise downgrade to unconfirmed.",
                        label,
                    )
                )
            if card.get("honesty_marker_present") or card.get("green_downgraded"):
                findings.append(
                    err(
                        "dashboard_queue_truth.green.marked",
                        f"{label} card {cid}: effective-clear yet flags a honesty "
                        "marker or green downgrade",
                        "A confirmed-clear row carries no honesty marker.",
                        label,
                    )
                )
        if displayed == "clear" and (fresh != "fresh" or age not in EVIDENCE_AGE_CURRENT):
            if not card.get("green_downgraded") or eff != "unconfirmed":
                findings.append(
                    err(
                        "dashboard_queue_truth.green.not_downgraded",
                        f"{label} card {cid}: declared clear with "
                        f"freshness={fresh}/evidence_age={age} but was not "
                        "withdrawn to unconfirmed",
                        "Withdraw a declared-clear row to unconfirmed when its "
                        "freshness or evidence is not current, and say why.",
                        label,
                    )
                )
            if not card.get("honesty_marker_present"):
                findings.append(
                    err(
                        "dashboard_queue_truth.green.no_marker",
                        f"{label} card {cid}: downgraded green without lighting the "
                        "honesty marker",
                        "Light the honesty marker on every downgraded row.",
                        label,
                    )
                )

    # 3. Canonical routing on every ref.
    for card in view["cards"]:
        if not is_canonical_ref(card.get("evidence_ref", "")):
            findings.append(
                err(
                    "dashboard_queue_truth.route.evidence_not_canonical",
                    f"{label} card {card.get('card_id')}: evidence_ref "
                    f"{card.get('evidence_ref')!r} is not a canonical durable object",
                    "Route open-details to aureline://<class>/<id>, never a "
                    "generic landing page.",
                    label,
                )
            )

    # 4. Queue presence + order/narrowing explainability.
    queue_order = view.get("queue_order")
    is_queue = surface in QUEUE_SURFACES
    if is_queue and queue_order is None:
        findings.append(
            err(
                "dashboard_queue_truth.queue.missing",
                f"{label}: queue surface carries no queue_order record",
                "Queue surfaces must carry order + narrowing truth.",
                label,
            )
        )
    if not is_queue and queue_order is not None:
        findings.append(
            err(
                "dashboard_queue_truth.queue.unexpected",
                f"{label}: service-health dashboard must not carry queue_order",
                "Only queue surfaces carry a queue_order record.",
                label,
            )
        )
    if queue_order is not None:
        findings.extend(check_queue(label, view, queue_order))

    return findings


def check_queue(label: str, view: dict[str, Any], queue: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    card_ids = {c["card_id"] for c in view["cards"]}
    row_ids = [r.get("row_id") for r in queue.get("rows", [])]

    if set(row_ids) != card_ids or len(row_ids) != len(card_ids):
        findings.append(
            err(
                "dashboard_queue_truth.queue.rows_cover_cards",
                f"{label}: queue rows {sorted(set(row_ids))} do not bijectively "
                f"cover cards {sorted(card_ids)}",
                "Every visible card needs exactly one order row.",
                label,
            )
        )

    for idx, row in enumerate(queue.get("rows", [])):
        rid = row.get("row_id")
        if row.get("order_rank") != idx + 1:
            findings.append(
                err(
                    "dashboard_queue_truth.order.rank",
                    f"{label} row {rid}: order_rank {row.get('order_rank')} != "
                    f"render position {idx + 1}",
                    "Rank visible rows 1..N in render order.",
                    label,
                )
            )
        if row.get("order_reason") not in ORDER_REASON_LABEL:
            findings.append(
                err(
                    "dashboard_queue_truth.order.reason_unknown",
                    f"{label} row {rid}: order_reason {row.get('order_reason')!r} "
                    "is outside the closed vocabulary",
                    "Quote a closed order-reason token so the order is explainable.",
                    label,
                )
            )
        expl = (row.get("order_explanation") or "").strip()
        if not expl or len(expl) > MAX_EXPLANATION_CHARS:
            findings.append(
                err(
                    "dashboard_queue_truth.order.explanation",
                    f"{label} row {rid}: order_explanation is empty or too long",
                    "Give each row a short, user-facing reason it is sorted here.",
                    label,
                )
            )
        if not is_canonical_ref(row.get("open_details_ref", "")):
            findings.append(
                err(
                    "dashboard_queue_truth.order.open_details_not_canonical",
                    f"{label} row {rid}: open_details_ref is not canonical",
                    "Route open-details to the durable object behind the row.",
                    label,
                )
            )

    seen_reasons: set[str] = set()
    for hidden in queue.get("hidden_scope", []):
        reason = hidden.get("narrowing_reason")
        if reason not in NARROWING_LABEL:
            findings.append(
                err(
                    "dashboard_queue_truth.narrow.reason_unknown",
                    f"{label}: narrowing reason {reason!r} is outside the vocabulary",
                    "Quote a closed narrowing-reason token.",
                    label,
                )
            )
            continue
        if reason in seen_reasons:
            findings.append(
                err(
                    "dashboard_queue_truth.narrow.duplicate",
                    f"{label}: narrowing reason {reason} appears more than once",
                    "Collapse each narrowing reason into one counter.",
                    label,
                )
            )
        seen_reasons.add(reason)
        if hidden.get("hidden_count", 0) < 1:
            findings.append(
                err(
                    "dashboard_queue_truth.narrow.count",
                    f"{label}: narrowing reason {reason} has a non-positive count",
                    "A bucket with zero hidden rows must not be emitted.",
                    label,
                )
            )
        expl = (hidden.get("narrowing_explanation") or "").strip()
        if not expl or len(expl) > MAX_EXPLANATION_CHARS:
            findings.append(
                err(
                    "dashboard_queue_truth.narrow.explanation",
                    f"{label}: narrowing reason {reason} has an empty/too-long "
                    "explanation",
                    "Explain how many rows are hidden and why.",
                    label,
                )
            )
        if not is_canonical_ref(hidden.get("reveal_ref", "")):
            findings.append(
                err(
                    "dashboard_queue_truth.narrow.reveal_not_canonical",
                    f"{label}: narrowing reason {reason} reveal_ref is not canonical",
                    "Route reveal to a scoped query/object, never a landing page.",
                    label,
                )
            )
        expected_incomplete = reason in INCOMPLETE_KNOWLEDGE_REASONS
        if bool(hidden.get("incomplete_knowledge")) != expected_incomplete:
            findings.append(
                err(
                    "dashboard_queue_truth.narrow.incomplete_flag",
                    f"{label}: narrowing reason {reason} incomplete_knowledge flag "
                    "is wrong",
                    "policy_scope and offline_partial_list are unknown rows "
                    "(incomplete knowledge); other reasons are deliberate filters.",
                    label,
                )
            )

    # narrowing_present / incomplete_knowledge_present consistency.
    hidden_total = queue.get("hidden_total", 0)
    if queue.get("narrowing_present") != (hidden_total > 0):
        findings.append(
            err(
                "dashboard_queue_truth.narrow.present_flag",
                f"{label}: narrowing_present disagrees with hidden_total",
                "Set narrowing_present whenever any row is hidden.",
                label,
            )
        )
    return findings


def _first_diff(a: Any, b: Any, path: str = "") -> str:
    """Locate the first differing leaf between two records, for diagnostics."""
    if isinstance(a, dict) and isinstance(b, dict):
        for key in sorted(set(a) | set(b)):
            if key not in a:
                return f"{path}.{key}: missing in stored"
            if key not in b:
                return f"{path}.{key}: missing in rebuilt"
            sub = _first_diff(a[key], b[key], f"{path}.{key}")
            if sub:
                return sub
        return ""
    if isinstance(a, list) and isinstance(b, list):
        if len(a) != len(b):
            return f"{path}: length {len(a)} != {len(b)}"
        for idx, (x, y) in enumerate(zip(a, b)):
            sub = _first_diff(x, y, f"{path}[{idx}]")
            if sub:
                return sub
        return ""
    if a != b:
        return f"{path}: stored={a!r} rebuilt={b!r}"
    return ""


# --------------------------------------------------------------------------- #
# Coverage — the corpus must exercise every class and prove the two audit drills
# --------------------------------------------------------------------------- #


def validate_coverage(views: dict[str, dict[str, Any]]) -> list[Finding]:
    findings: list[Finding] = []
    seen_surfaces: set[str] = set()
    seen_freshness: set[str] = set()
    seen_age: set[str] = set()
    seen_order: set[str] = set()
    seen_narrow: set[str] = set()
    seen_kind: set[str] = set()
    seen_downgrade: set[str] = set()

    for view in views.values():
        seen_surfaces.add(view["surface"])
        for c in view["cards"]:
            seen_freshness.add(c["freshness"])
            seen_age.add(c["evidence_age"])
            seen_kind.add(c["evidence_kind"])
            seen_downgrade.update(c["downgrade_reason_tokens"])
        q = view.get("queue_order")
        if q:
            for r in q["rows"]:
                seen_order.add(r["order_reason"])
            for h in q["hidden_scope"]:
                seen_narrow.add(h["narrowing_reason"])

    def missing(name: str, expected: set[str], seen: set[str]) -> None:
        gap = expected - seen
        if gap:
            findings.append(
                err(
                    f"dashboard_queue_truth.coverage.{name}",
                    f"corpus does not exercise {name}: {sorted(gap)}",
                    f"Add a drill that exercises every {name} token.",
                )
            )

    missing("surface", set(SURFACE_LABEL), seen_surfaces)
    missing("freshness", set(FRESHNESS_LABEL), seen_freshness)
    missing("evidence_age", set(EVIDENCE_AGE_LABEL), seen_age)
    missing("order_reason", set(ORDER_REASON_LABEL), seen_order)
    missing("narrowing_reason", set(NARROWING_LABEL), seen_narrow)
    missing("evidence_kind", set(EVIDENCE_KIND_LABEL), seen_kind)
    missing("downgrade_reason", set(DOWNGRADE_LABEL), seen_downgrade)

    findings.extend(validate_audit_properties(views))
    return findings


def order_principles(view: dict[str, Any]) -> set[str]:
    q = view.get("queue_order")
    if not q:
        return set()
    return {ORDER_PRINCIPLE.get(r["order_reason"], "unknown") for r in q["rows"]}


def validate_audit_properties(views: dict[str, dict[str, Any]]) -> list[Finding]:
    findings: list[Finding] = []

    # Order ambiguity: at least one queue must disambiguate risk vs time vs owner
    # — three distinct ordering principles, each with a distinct explanation.
    ambiguity_ok = False
    for view in views.values():
        principles = order_principles(view)
        if {"risk", "time", "owner"}.issubset(principles):
            q = view["queue_order"]
            expls = [r["order_explanation"] for r in q["rows"]]
            if len(set(expls)) == len(expls):
                ambiguity_ok = True
                break
    if not ambiguity_ok:
        findings.append(
            err(
                "dashboard_queue_truth.audit.order_ambiguity",
                "no drill disambiguates risk vs time vs owner ordering with a "
                "distinct, explained reason per row",
                "Add a queue drill whose rows span risk, time, and owner order "
                "principles, each with its own explanation.",
            )
        )

    # Restart / reconnect evidence break: a declared-clear row whose evidence
    # link broke (evidence_age never, or freshness unavailable/stale with aged
    # evidence) must be downgraded and keep a canonical reopen path.
    break_ok = False
    for view in views.values():
        for c in view["cards"]:
            broke = c["evidence_age"] == "never" or (
                c["freshness"] in {"unavailable", "stale"}
                and c["evidence_age"] in EVIDENCE_AGE_WARNING
            )
            if (
                c["displayed_state"] == "clear"
                and broke
                and c["green_downgraded"]
                and c["effective_state"] == "unconfirmed"
                and is_canonical_ref(c["evidence_ref"])
            ):
                break_ok = True
                break
        if break_ok:
            break
    if not break_ok:
        findings.append(
            err(
                "dashboard_queue_truth.audit.restart_evidence_break",
                "no drill proves a row with a broken evidence link after "
                "restart/reconnect is downgraded while keeping a canonical reopen "
                "path",
                "Add a drill with a declared-clear row whose evidence link broke "
                "(never / aged out) that downgrades to unconfirmed.",
            )
        )

    # The 'never' evidence-age bucket must be exercised (the broken-link case).
    if not any(
        c["evidence_age"] == "never" for v in views.values() for c in v["cards"]
    ):
        findings.append(
            err(
                "dashboard_queue_truth.audit.never_evidence",
                "no card exercises the 'never' evidence-age bucket",
                "Cover a row whose evidence link never re-confirmed.",
            )
        )
    return findings


# --------------------------------------------------------------------------- #
# Export parity — support bundle (plaintext) and CLI index preserve the same
# freshness / order / hidden semantics as the product UI record.
# --------------------------------------------------------------------------- #


def render_support_export(view: dict[str, Any]) -> str:
    """Port of DashboardTruthView::render_plaintext (the support-export block)."""
    out: list[str] = []
    out.append("Dashboard & queue truth")
    out.append(f"View: {view['view_id']}")
    out.append(f"Surface: {view['surface_label']} ({view['surface_token']})")
    out.append(f"As of: {view['as_of']}")
    overall_eff = view["overall_effective_state"]
    overall_fresh = view["overall_freshness"]
    out.append(
        f"Overall: {EFFECTIVE_LABEL.get(overall_eff, overall_eff)} "
        f"({view['overall_effective_state_token']}) | Freshness: "
        f"{FRESHNESS_LABEL.get(overall_fresh, overall_fresh)} "
        f"({view['overall_freshness_token']})"
    )
    out.append(
        "Honesty marker: "
        + ("present" if view["honesty_marker_present"] else "none")
    )
    s = view["summary"]
    out.append(
        "Summary: total={}, clear={}, unconfirmed={}, attention={}, blocked={}, "
        "green_downgrades={}, stale_evidence={}, freshness_downgrades={}".format(
            s["total_card_count"],
            s["clear_card_count"],
            s["unconfirmed_card_count"],
            s["attention_card_count"],
            s["blocked_card_count"],
            s["green_downgrade_count"],
            s["stale_evidence_count"],
            s["freshness_downgrade_count"],
        )
    )
    out.append("")
    for card in view["cards"]:
        tag = "warn" if card["honesty_marker_present"] else "ok"
        out.append(
            f"- {card['card_id']} [{tag}] state={card['displayed_state_token']}/"
            f"{card['effective_state_token']} freshness={card['freshness_token']} "
            f"evidence_age={card['evidence_age_token']}"
        )
        if card["downgrade_reasons"]:
            out.append(
                "    downgrade reasons: " + ", ".join(card["downgrade_reason_tokens"])
            )
        out.append(f"    explain: {card['state_explanation']}")
        out.append(
            f"    evidence: {card['evidence_kind_token']} -> {card['evidence_ref']}"
        )
    q = view.get("queue_order")
    if q:
        out.append("")
        out.append(
            f"Queue order ({q['queue_id']}): visible={q['visible_row_count']}, "
            f"hidden={q['hidden_total']}, in_scope={q['total_in_scope_count']}"
        )
        for row in q["rows"]:
            out.append(
                f"  #{row['order_rank']} {row['row_id']} "
                f"order={row['order_reason_token']} -> {row['open_details_ref']}"
            )
        for hidden in q["hidden_scope"]:
            marker = "unknown" if hidden["incomplete_knowledge"] else "filtered"
            out.append(
                f"  hidden {hidden['narrowing_reason_token']} "
                f"x{hidden['hidden_count']} ({marker}) -> {hidden['reveal_ref']}"
            )
    return "\n".join(out) + "\n"


def render_cli_index(view: dict[str, Any], fixture_filename: str) -> str:
    """Port of the emitter `index` subcommand line (CLI / headless summary)."""
    scenario_id = view["view_id"].rsplit(".", 1)[-1]
    honesty = "honesty=present" if view["honesty_marker_present"] else "honesty=none"
    hidden_total = (view["queue_order"] or {}).get("hidden_total", 0) if view.get(
        "queue_order"
    ) else 0
    return "\t".join(
        [
            scenario_id,
            view["surface_token"],
            fixture_filename,
            view["overall_effective_state_token"],
            view["overall_freshness_token"],
            honesty,
            f"hidden={hidden_total}",
        ]
    )


def digest_from_view(view: dict[str, Any]) -> dict[str, Any]:
    """The full semantic digest the support export must preserve."""
    cards = [
        {
            "card_id": c["card_id"],
            "displayed_state": c["displayed_state_token"],
            "effective_state": c["effective_state_token"],
            "freshness": c["freshness_token"],
            "evidence_age": c["evidence_age_token"],
            "green_downgraded": c["green_downgraded"],
            "honesty_marker_present": c["honesty_marker_present"],
            "downgrade_reasons": list(c["downgrade_reason_tokens"]),
            "evidence_kind": c["evidence_kind_token"],
            "evidence_ref": c["evidence_ref"],
        }
        for c in view["cards"]
    ]
    queue = None
    q = view.get("queue_order")
    if q:
        queue = {
            "queue_id": q["queue_id"],
            "visible_row_count": q["visible_row_count"],
            "hidden_total": q["hidden_total"],
            "rows": [
                {
                    "row_id": r["row_id"],
                    "order_rank": r["order_rank"],
                    "order_reason": r["order_reason_token"],
                    "open_details_ref": r["open_details_ref"],
                }
                for r in q["rows"]
            ],
            "hidden": [
                {
                    "narrowing_reason": h["narrowing_reason_token"],
                    "hidden_count": h["hidden_count"],
                    "incomplete_knowledge": h["incomplete_knowledge"],
                    "reveal_ref": h["reveal_ref"],
                }
                for h in q["hidden_scope"]
            ],
        }
    return {
        "overall_effective_state": view["overall_effective_state_token"],
        "overall_freshness": view["overall_freshness_token"],
        "honesty_marker_present": view["honesty_marker_present"],
        "cards": cards,
        "queue": queue,
    }


_CARD_RE = re.compile(
    r"^- (?P<card_id>\S+) \[(?P<tag>warn|ok)\] state=(?P<displayed>\w+)/"
    r"(?P<effective>\w+) freshness=(?P<freshness>\w+) evidence_age=(?P<age>\w+)$"
)
_ROW_RE = re.compile(
    r"^  #(?P<rank>\d+) (?P<row_id>\S+) order=(?P<reason>\w+) -> (?P<ref>\S+)$"
)
_HIDDEN_RE = re.compile(
    r"^  hidden (?P<reason>\w+) x(?P<count>\d+) \((?P<marker>unknown|filtered)\) "
    r"-> (?P<ref>\S+)$"
)
_QUEUE_RE = re.compile(
    r"^Queue order \((?P<queue_id>[^)]+)\): visible=(?P<visible>\d+), "
    r"hidden=(?P<hidden>\d+), in_scope=(?P<in_scope>\d+)$"
)


def digest_from_support_export(text: str) -> dict[str, Any]:
    """Recover the digest by parsing the support-export plaintext back."""
    lines = text.split("\n")
    overall_eff = overall_fresh = None
    honesty = None
    cards: list[dict[str, Any]] = []
    queue: dict[str, Any] | None = None
    i = 0
    while i < len(lines):
        line = lines[i]
        if line.startswith("Overall: "):
            m = re.search(
                r"\((?P<eff>\w+)\) \| Freshness: .*\((?P<fresh>\w+)\)$", line
            )
            overall_eff, overall_fresh = m.group("eff"), m.group("fresh")
        elif line.startswith("Honesty marker: "):
            honesty = line.split(": ", 1)[1].strip() == "present"
        elif _CARD_RE.match(line):
            m = _CARD_RE.match(line)
            reasons: list[str] = []
            evidence_kind = evidence_ref = None
            j = i + 1
            while j < len(lines) and lines[j].startswith("    "):
                sub = lines[j]
                if sub.startswith("    downgrade reasons: "):
                    reasons = [
                        r.strip()
                        for r in sub.split(": ", 1)[1].split(",")
                        if r.strip()
                    ]
                elif sub.startswith("    evidence: "):
                    body = sub.split(": ", 1)[1]
                    evidence_kind, evidence_ref = body.split(" -> ", 1)
                j += 1
            displayed, effective = m.group("displayed"), m.group("effective")
            cards.append(
                {
                    "card_id": m.group("card_id"),
                    "displayed_state": displayed,
                    "effective_state": effective,
                    "freshness": m.group("freshness"),
                    "evidence_age": m.group("age"),
                    "green_downgraded": displayed == "clear" and effective == "unconfirmed",
                    "honesty_marker_present": m.group("tag") == "warn",
                    "downgrade_reasons": reasons,
                    "evidence_kind": evidence_kind,
                    "evidence_ref": evidence_ref,
                }
            )
            i = j
            continue
        elif _QUEUE_RE.match(line):
            m = _QUEUE_RE.match(line)
            queue = {
                "queue_id": m.group("queue_id"),
                "visible_row_count": int(m.group("visible")),
                "hidden_total": int(m.group("hidden")),
                "rows": [],
                "hidden": [],
            }
        elif _ROW_RE.match(line):
            m = _ROW_RE.match(line)
            queue["rows"].append(
                {
                    "row_id": m.group("row_id"),
                    "order_rank": int(m.group("rank")),
                    "order_reason": m.group("reason"),
                    "open_details_ref": m.group("ref"),
                }
            )
        elif _HIDDEN_RE.match(line):
            m = _HIDDEN_RE.match(line)
            queue["hidden"].append(
                {
                    "narrowing_reason": m.group("reason"),
                    "hidden_count": int(m.group("count")),
                    "incomplete_knowledge": m.group("marker") == "unknown",
                    "reveal_ref": m.group("ref"),
                }
            )
        i += 1
    return {
        "overall_effective_state": overall_eff,
        "overall_freshness": overall_fresh,
        "honesty_marker_present": honesty,
        "cards": cards,
        "queue": queue,
    }


def coarse_digest(full: dict[str, Any]) -> dict[str, Any]:
    return {
        "overall_effective_state": full["overall_effective_state"],
        "overall_freshness": full["overall_freshness"],
        "honesty_marker_present": full["honesty_marker_present"],
        "hidden_total": (full["queue"] or {}).get("hidden_total", 0)
        if full["queue"]
        else 0,
    }


def digest_from_cli_index(line: str) -> dict[str, Any]:
    parts = line.split("\t")
    return {
        "overall_effective_state": parts[3],
        "overall_freshness": parts[4],
        "honesty_marker_present": parts[5] == "honesty=present",
        "hidden_total": int(parts[6].split("=", 1)[1]),
    }


def build_parity_entry(
    scenario_id: str, view: dict[str, Any], source_fixture: str
) -> dict[str, Any]:
    support_text = render_support_export(view)
    cli_line = render_cli_index(view, Path(source_fixture).name)
    ui_full = digest_from_view(view)
    return {
        "scenario_id": scenario_id,
        "surface": view["surface_token"],
        "source_fixture": source_fixture,
        "semantic_digest": ui_full,
        "support_export_text": support_text,
        "cli_index_line": cli_line,
        "parity": {
            "ui_record_full": True,
            "support_export_full": digest_from_support_export(support_text) == ui_full,
            "cli_index_coarse": digest_from_cli_index(cli_line) == coarse_digest(ui_full),
        },
    }


def check_parity(views_by_scenario: dict[str, tuple[str, dict[str, Any]]]) -> list[Finding]:
    findings: list[Finding] = []
    for scenario_id, (source_fixture, view) in sorted(views_by_scenario.items()):
        ui_full = digest_from_view(view)
        support_text = render_support_export(view)
        cli_line = render_cli_index(view, Path(source_fixture).name)
        if digest_from_support_export(support_text) != ui_full:
            findings.append(
                err(
                    "dashboard_queue_truth.parity.support_export",
                    f"{scenario_id}: support-export plaintext loses freshness/order/"
                    "hidden semantics from the UI record",
                    "Keep the support-export projection faithful to the record.",
                    source_fixture,
                )
            )
        if digest_from_cli_index(cli_line) != coarse_digest(ui_full):
            findings.append(
                err(
                    "dashboard_queue_truth.parity.cli_index",
                    f"{scenario_id}: CLI index line loses overall state/freshness/"
                    "hidden semantics from the UI record",
                    "Keep the CLI / headless index faithful to the record.",
                    source_fixture,
                )
            )
    return findings


# --------------------------------------------------------------------------- #
# Matrix + parity packet (build / drift-check)
# --------------------------------------------------------------------------- #


def lane_properties(scenario_id: str, view: dict[str, Any]) -> list[str]:
    props = ["no_silent_green", "canonical_routing", "export_parity"]
    if view.get("queue_order"):
        props.append("order_explainable")
        if view["queue_order"]["narrowing_present"]:
            props.append("narrowing_explainable")
        if {"risk", "time", "owner"}.issubset(order_principles(view)):
            props.append("order_ambiguity")
    if any(c["green_downgraded"] for c in view["cards"]):
        props.append("stale_green_downgrade")
    if any(c["evidence_age"] == "never" for c in view["cards"]):
        props.append("restart_evidence_break")
    return sorted(set(props))


def build_matrix(views_by_scenario: dict[str, tuple[str, dict[str, Any]]]) -> dict[str, Any]:
    cases = []
    for scenario_id, (source_fixture, view) in sorted(views_by_scenario.items()):
        q = view.get("queue_order")
        cases.append(
            {
                "scenario_id": scenario_id,
                "surface": view["surface_token"],
                "source_fixture": source_fixture,
                "overall_effective_state": view["overall_effective_state_token"],
                "overall_freshness": view["overall_freshness_token"],
                "honesty_marker_present": view["honesty_marker_present"],
                "green_downgrade_count": view["summary"]["green_downgrade_count"],
                "hidden_total": q["hidden_total"] if q else 0,
                "order_reasons_present": q["order_reasons_present"] if q else [],
                "narrowing_reasons_present": (
                    sorted(h["narrowing_reason_token"] for h in q["hidden_scope"])
                    if q
                    else []
                ),
                "lane_properties": lane_properties(scenario_id, view),
            }
        )
    return {
        "record_kind": CORPUS_MATRIX_RECORD_KIND,
        "schema_version": 1,
        "shared_contract_ref": AUDIT_CONTRACT_REF,
        "runtime_fixture_dir": RUNTIME_FIXTURE_DIR_REL,
        "audit_fixture_dir": AUDIT_FIXTURE_DIR_REL,
        "card_schema_ref": CARD_SCHEMA_REL,
        "queue_schema_ref": QUEUE_SCHEMA_REL,
        "validator_ref": VALIDATOR_REL,
        "drill_cases": cases,
    }


def build_parity_packet(
    views_by_scenario: dict[str, tuple[str, dict[str, Any]]]
) -> dict[str, Any]:
    entries = [
        build_parity_entry(scenario_id, view, source_fixture)
        for scenario_id, (source_fixture, view) in sorted(views_by_scenario.items())
    ]
    return {
        "record_kind": PARITY_PACKET_RECORD_KIND,
        "schema_version": 1,
        "shared_contract_ref": AUDIT_CONTRACT_REF,
        "note": (
            "For every scenario the support-export plaintext and the CLI / "
            "headless index preserve the same freshness, order, and hidden-scope "
            "semantics as the product UI record (semantic_digest). Regenerate "
            "with `ci/check_dashboard_queue_truth.py --write`."
        ),
        "scenarios": entries,
    }


def dump_json(path: Path, payload: Any) -> None:
    path.write_text(
        json.dumps(payload, indent=2, ensure_ascii=False) + "\n", encoding="utf-8"
    )


def validate_companion(path: Path, kind: str, expected: Any) -> list[Finding]:
    if not path.exists():
        return [
            err(
                f"dashboard_queue_truth.{kind}.missing",
                f"{kind} is missing: {path}",
                f"Generate it with --write.",
                str(path),
            )
        ]
    actual = load_json(path)
    if actual != expected:
        return [
            err(
                f"dashboard_queue_truth.{kind}.drift",
                f"{kind} has drifted from the corpus; regenerate with --write",
                "Run ci/check_dashboard_queue_truth.py --write and commit the result.",
                str(path),
                diff=_first_diff(actual, expected),
            )
        ]
    return []


def validate_report_and_readme(
    report_path: Path, readme_path: Path, scenario_ids: list[str]
) -> list[Finding]:
    findings: list[Finding] = []
    required_refs = [
        CARD_SCHEMA_REL,
        QUEUE_SCHEMA_REL,
        RUNTIME_FIXTURE_DIR_REL,
        AUDIT_FIXTURE_DIR_REL,
        VALIDATOR_REL,
        SCRIPT_REL,
    ]
    for kind, path, refs in (
        ("report", report_path, required_refs),
        ("readme", readme_path, [VALIDATOR_REL, RUNTIME_FIXTURE_DIR_REL]),
    ):
        if not path.exists():
            findings.append(
                err(
                    f"dashboard_queue_truth.{kind}.missing",
                    f"{kind} is missing: {path}",
                    f"Land the {kind} for the audit lane.",
                    str(path),
                )
            )
            continue
        text = path.read_text(encoding="utf-8")
        for ref in refs:
            if ref not in text:
                findings.append(
                    err(
                        f"dashboard_queue_truth.{kind}.missing_ref",
                        f"{kind} does not mention {ref}",
                        "Keep the report/README pointing at the schemas, fixtures, "
                        "validator, and script.",
                        str(path),
                    )
                )
        for scenario_id in scenario_ids:
            if scenario_id not in text:
                findings.append(
                    err(
                        f"dashboard_queue_truth.{kind}.missing_scenario",
                        f"{kind} does not mention scenario {scenario_id}",
                        "Document every drill scenario.",
                        str(path),
                    )
                )
    return findings


# --------------------------------------------------------------------------- #
# Loading + schema validation
# --------------------------------------------------------------------------- #


def load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError as exc:
        raise SystemExit(f"missing JSON file: {path}") from exc
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON at {path}: {exc}") from exc


def make_validator(repo_root: Path) -> Draft202012Validator:
    card_schema = load_json(repo_root / CARD_SCHEMA_REL)
    queue_schema = load_json(repo_root / QUEUE_SCHEMA_REL)
    Draft202012Validator.check_schema(card_schema)
    if _HAVE_REFERENCING:
        registry = Registry().with_resources(
            [
                (card_schema["$id"], Resource.from_contents(card_schema)),
                (queue_schema["$id"], Resource.from_contents(queue_schema)),
            ]
        )
        return Draft202012Validator(
            card_schema, registry=registry, format_checker=FormatChecker()
        )
    # Fallback for ancient jsonschema: resolve via a classic RefResolver.
    from jsonschema import RefResolver  # type: ignore

    store = {card_schema["$id"]: card_schema, queue_schema["$id"]: queue_schema}
    resolver = RefResolver.from_schema(card_schema, store=store)
    return Draft202012Validator(
        card_schema, resolver=resolver, format_checker=FormatChecker()
    )


def schema_validate(validator: Draft202012Validator, label: str, view: Any) -> list[Finding]:
    findings: list[Finding] = []
    for error in sorted(validator.iter_errors(view), key=lambda e: list(e.path)):
        path = ".".join(str(p) for p in error.path) or "<root>"
        findings.append(
            err(
                "dashboard_queue_truth.schema.validation_failed",
                f"{label}: {path}: {error.message}",
                f"Fix the view so it validates against {CARD_SCHEMA_REL}.",
                label,
                schema_path=list(error.schema_path),
            )
        )
    return findings


def collect_views(fixture_dir: Path) -> dict[str, dict[str, Any]]:
    views: dict[str, dict[str, Any]] = {}
    if not fixture_dir.exists():
        return views
    for path in sorted(fixture_dir.glob("*.json")):
        if path.name in NON_VIEW_FILES:
            continue
        record = load_json(path)
        if record.get("record_kind") != VIEW_RECORD_KIND:
            continue
        views[path.name] = record
    return views


def scenario_id_of(view: dict[str, Any]) -> str:
    return view["view_id"].rsplit(".", 1)[-1]


# --------------------------------------------------------------------------- #
# Main
# --------------------------------------------------------------------------- #


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument(
        "--write",
        action="store_true",
        help="Regenerate the audit drill fixtures, corpus matrix, and parity packet.",
    )
    parser.add_argument("--report-json", default=None)
    return parser.parse_args()


def write_corpus(repo_root: Path) -> None:
    audit_dir = repo_root / AUDIT_FIXTURE_DIR_REL
    audit_dir.mkdir(parents=True, exist_ok=True)
    for drill in audit_drills():
        dump_json(audit_dir / drill.fixture_filename, drill.view())
        print(f"wrote {AUDIT_FIXTURE_DIR_REL}/{drill.fixture_filename}")

    views_by_scenario = gather_views(repo_root)
    dump_json(repo_root / CORPUS_MATRIX_REL, build_matrix(views_by_scenario))
    print(f"wrote {CORPUS_MATRIX_REL}")
    dump_json(repo_root / PARITY_PACKET_REL, build_parity_packet(views_by_scenario))
    print(f"wrote {PARITY_PACKET_REL}")


def gather_views(repo_root: Path) -> dict[str, tuple[str, dict[str, Any]]]:
    """scenario_id -> (repo-relative source fixture path, view record)."""
    result: dict[str, tuple[str, dict[str, Any]]] = {}
    for rel_dir in (RUNTIME_FIXTURE_DIR_REL, AUDIT_FIXTURE_DIR_REL):
        for name, view in collect_views(repo_root / rel_dir).items():
            result[scenario_id_of(view)] = (f"{rel_dir}/{name}", view)
    return result


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    if args.write:
        write_corpus(repo_root)
        return 0

    findings: list[Finding] = []
    validator = make_validator(repo_root)

    views_by_scenario = gather_views(repo_root)
    if not views_by_scenario:
        raise SystemExit("no dashboard-truth view fixtures found")

    # Structural (schema) validation gates the semantic and builder stages: a
    # malformed record cannot be re-derived or projected, so we report its schema
    # errors and skip the stages that assume a well-formed corpus.
    flat_views: dict[str, dict[str, Any]] = {}
    schema_findings: list[Finding] = []
    for scenario_id, (source_fixture, view) in views_by_scenario.items():
        flat_views[source_fixture] = view
        these = schema_validate(validator, source_fixture, view)
        schema_findings.extend(these)
        findings.extend(these)
        findings.extend(check_view_invariants(source_fixture, view))

    if not schema_findings:
        findings.extend(validate_coverage(flat_views))
        findings.extend(check_parity(views_by_scenario))

        expected_matrix = build_matrix(views_by_scenario)
        findings.extend(
            validate_companion(
                repo_root / CORPUS_MATRIX_REL, "corpus_matrix", expected_matrix
            )
        )
        expected_packet = build_parity_packet(views_by_scenario)
        findings.extend(
            validate_companion(
                repo_root / PARITY_PACKET_REL, "parity_packet", expected_packet
            )
        )

    scenario_ids = sorted(views_by_scenario)
    findings.extend(
        validate_report_and_readme(
            repo_root / REPORT_REL, repo_root / README_REL, scenario_ids
        )
    )

    report = {
        "status": "pass" if not findings else "fail",
        "runtime_fixture_dir": RUNTIME_FIXTURE_DIR_REL,
        "audit_fixture_dir": AUDIT_FIXTURE_DIR_REL,
        "scenario_count": len(views_by_scenario),
        "findings": [f.as_report() for f in findings],
    }
    if args.report_json:
        Path(args.report_json).write_text(
            json.dumps(report, indent=2) + "\n", encoding="utf-8"
        )
    if findings:
        print(json.dumps(report, indent=2), file=sys.stderr)
        return 1
    print(
        f"[dashboard-queue-truth] PASS ({len(views_by_scenario)} scenarios)",
        file=sys.stderr,
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
