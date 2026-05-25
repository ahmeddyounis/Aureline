#!/usr/bin/env python3
"""Validate the stable publication pack for the release line's known-limits, public
benchmark, compatibility, and migration publications.

The stable claim manifest decides the single canonical lifecycle label each *subject*
publishes; the stable proof index decides whether each launch-blocking *requirement* is
proven; the stable version-window freeze freezes each public interface surface's version
window; the maintenance-control packet governs each post-release maintenance lane. This
pack answers the publication-level question: for each outward-facing publication the
release line ships about its own limits and behavior — a known-limits publication, a
public benchmark publication, a compatibility publication, or a migration publication —
is that publication actually backed by a fresh proof packet, within its published
p50/p95 budget where it makes a performance claim, and an owner sign-off? It reads the
checked-in pack at ``artifacts/release/stable_publication_pack.json`` and:

  - asserts the closed vocabularies (lifecycle labels, publication kinds, publication
    states, gap reasons, publication actions) and the launch cutline are canonical;
  - asserts every row that is narrowed (for an absent/incomplete surface, a missing
    corpus/trace, a regressed benchmark budget, a breached/missing proof packet, an
    expired waiver, a missing owner sign-off, or a public claim whose canonical label is
    itself below the cutline) drops below the cutline rather than carrying a label wider
    than the public claim, and that a backed row carries the public claim's canonical
    label cleanly with a captured, within-SLO proof packet, a within-budget benchmark,
    and an owner sign-off;
  - protects published p50/p95 budgets: a benchmark row must carry a budget with corpus
    metadata and a benchmark-lab trace, must be ordered p50 <= p95, must name
    ``budget_regressed`` when measured exceeds published, and may only hold its label over
    budget when an active waiver covers an intentionally tightened threshold;
  - asserts known-limit/benchmark/compatibility/migration coverage: every publication
    kind is represented, every declared release-blocking surface ref has exactly one
    covering release-blocking row, every release-blocking row is declared, and no surface
    ref repeats;
  - performs the ceiling cross-check the typed model cannot — it reads the stable claim
    manifest named by ``claim_manifest_ref`` and fails when a row's ``claim_label`` is not
    the label the claim manifest publishes for the entry named by ``claim_ref``, or names
    an entry the claim manifest does not carry;
  - performs the packet-freshness SLO automation against the pack ``as_of`` date and the
    waiver-expiry date arithmetic the typed model cannot;
  - recomputes the publication verdict and the summary block, and fails on any drift;
  - runs negative drills proving the narrowing, ceiling, freshness, budget, waiver, and
    publication rejections all fire; and
  - runs the checked-in fixture cases under ``fixtures/release/stable_publication_pack``
    and fails when a case the pack marks as rejected validates clean.

With ``--require-proceed`` the gate additionally fails (exit code 2) when the recomputed
publication verdict is ``hold``, so shiproom and release tooling can block publication
directly from this artifact.

The typed Rust consumer
(``aureline_release::stable_publication_pack::current_stable_publication_pack``) reads the
same pack and runs the same structural cross-check, so this gate and
``cargo test -p aureline-release`` agree without a cargo build in CI.
"""

from __future__ import annotations

import argparse
import copy
import dataclasses
import datetime as dt
import json
import sys
from pathlib import Path
from typing import Any


DEFAULT_PACK_REL = "artifacts/release/stable_publication_pack.json"
DEFAULT_REPORT_REL = (
    "artifacts/release/captures/stable_publication_pack_validation_capture.json"
)
DEFAULT_FIXTURES_REL = "fixtures/release/stable_publication_pack"

EXPECTED_SCHEMA_VERSION = 1
PACK_RECORD_KIND = "stable_publication_pack"

LIFECYCLE_LABELS = ("lts", "stable", "beta", "preview", "withdrawn")
ABOVE_CUTLINE = ("lts", "stable")
BELOW_CUTLINE = ("beta", "preview", "withdrawn")
PUBLICATION_KINDS = ("known_limit", "benchmark", "compatibility", "migration")
FRESHNESS_SLO_STATES = ("current", "due_for_refresh", "breached", "missing")
PUBLICATION_STATES = (
    "published",
    "published_on_waiver",
    "narrowed_unbacked",
    "narrowed_claim_narrowed",
    "narrowed_stale",
    "narrowed_waiver_expired",
    "narrowed_budget_regressed",
)
GAP_REASONS = (
    "claim_label_narrowed",
    "surface_capability_absent",
    "evidence_incomplete",
    "corpus_metadata_missing",
    "budget_regressed",
    "proof_packet_freshness_breached",
    "proof_packet_missing",
    "waiver_expired",
    "owner_signoff_missing",
)
PUBLICATION_ACTIONS = (
    "hold_publication",
    "narrow_publication_label",
    "refresh_proof_packet",
    "recapture_evidence",
    "retune_or_waive_budget",
    "request_owner_signoff",
)
HOLDING_STATES = ("published", "published_on_waiver")
NARROWING_STATES = (
    "narrowed_unbacked",
    "narrowed_claim_narrowed",
    "narrowed_stale",
    "narrowed_waiver_expired",
    "narrowed_budget_regressed",
)

LEVEL_RANK = {"lts": 4, "stable": 3, "beta": 2, "preview": 1, "withdrawn": 0}
CUTLINE_RANK = LEVEL_RANK["stable"]
FRESHNESS_RANK = {"current": 3, "due_for_refresh": 2, "breached": 1, "missing": 0}


@dataclasses.dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    ref: str

    def as_report(self) -> dict[str, str]:
        return dataclasses.asdict(self)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--pack", default=DEFAULT_PACK_REL)
    parser.add_argument("--fixtures-dir", default=DEFAULT_FIXTURES_REL)
    parser.add_argument("--report", default=None, help="Optional JSON validation capture path.")
    parser.add_argument(
        "--check",
        action="store_true",
        help="CI mode: validate and write the validation capture (default report path).",
    )
    parser.add_argument(
        "--no-capture",
        action="store_true",
        help="Validate without writing the validation capture.",
    )
    parser.add_argument(
        "--require-proceed",
        action="store_true",
        help="Publication gate: also fail (exit 2) when the recomputed verdict is hold.",
    )
    return parser.parse_args()


def load_json(path: Path, label: str) -> Any:
    if not path.exists():
        raise SystemExit(f"missing {label}: {path}")
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise SystemExit(f"{label} is not valid JSON: {exc}") from exc


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be a mapping/object")
    return value


def generated_at_now() -> str:
    return dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def json_text(payload: dict[str, Any]) -> str:
    return json.dumps(payload, indent=2, sort_keys=True) + "\n"


def is_str(value: Any) -> bool:
    return isinstance(value, str) and bool(value.strip())


def is_int(value: Any) -> bool:
    return isinstance(value, int) and not isinstance(value, bool)


def parse_date(value: Any) -> dt.date | None:
    if not isinstance(value, str):
        return None
    try:
        return dt.date.fromisoformat(value)
    except ValueError:
        return None


def is_above_cutline(level: Any) -> bool:
    return isinstance(level, str) and LEVEL_RANK.get(level, -1) >= CUTLINE_RANK


def has_capture(packet: dict[str, Any]) -> bool:
    return bool(packet.get("captured_at")) and bool(packet.get("evidence_refs"))


def reasons_of(row: dict[str, Any]) -> list[str]:
    reasons = row.get("active_gap_reasons", [])
    return reasons if isinstance(reasons, list) else []


def budget_of(row: dict[str, Any]) -> dict[str, Any] | None:
    budget = row.get("benchmark_budget")
    return budget if isinstance(budget, dict) else None


def budget_within(budget: dict[str, Any]) -> bool:
    p50_pub = budget.get("published_p50_ms")
    p95_pub = budget.get("published_p95_ms")
    p50_m = budget.get("measured_p50_ms")
    p95_m = budget.get("measured_p95_ms")
    if not all(is_int(v) for v in (p50_pub, p95_pub, p50_m, p95_m)):
        return False
    return p50_m <= p50_pub and p95_m <= p95_pub


def budget_complete(budget: dict[str, Any]) -> bool:
    return is_str(budget.get("corpus_ref")) and is_str(budget.get("trace_ref"))


def waiver_covers_tightened(row: dict[str, Any]) -> bool:
    budget = budget_of(row)
    return (
        row.get("publication_state") == "published_on_waiver"
        and budget is not None
        and budget.get("tightened") is True
    )


def publication_rule_fires(rule: dict[str, Any], rows: list[dict[str, Any]]) -> bool:
    trigger = rule.get("trigger_reason")
    labels = rule.get("applies_to_labels", [])
    if not isinstance(labels, list):
        return False
    for row in rows:
        if row.get("claim_label") in labels and trigger in reasons_of(row):
            return True
    return False


def computed_publication(pack: dict[str, Any]) -> tuple[str, list[str], list[str]]:
    rows = pack.get("rows", [])
    rules = pack.get("publication_rules", [])
    blocking_rules = [
        rule
        for rule in rules
        if rule.get("blocks_publication") is True and publication_rule_fires(rule, rows)
    ]
    rule_ids = sorted(str(rule.get("rule_id")) for rule in blocking_rules)
    blocking_triggers = {rule.get("trigger_reason") for rule in blocking_rules}
    entry_ids: set[str] = set()
    for row in rows:
        if is_above_cutline(row.get("claim_label")) and blocking_triggers.intersection(
            reasons_of(row)
        ):
            entry_ids.add(str(row.get("entry_id")))
    decision = "hold" if blocking_rules else "proceed"
    return decision, rule_ids, sorted(entry_ids)


def computed_summary(pack: dict[str, Any]) -> dict[str, Any]:
    rows = pack.get("rows", [])
    rules = pack.get("publication_rules", [])

    def packet_state(row: dict[str, Any]) -> Any:
        packet = row.get("proof_packet")
        return packet.get("slo_state") if isinstance(packet, dict) else None

    def kind_count(kind: str) -> int:
        return sum(1 for r in rows if r.get("publication_kind") == kind)

    published = [r for r in rows if is_above_cutline(r.get("published_label"))]
    claims = {str(r.get("claim_ref")) for r in rows}
    release_blocking = [r for r in rows if r.get("release_blocking") is True]
    rb_published = [r for r in release_blocking if is_above_cutline(r.get("published_label"))]
    budgets = [budget_of(r) for r in rows]
    budgets = [b for b in budgets if b is not None]
    return {
        "total_entries": len(rows),
        "total_claims": len(claims),
        "entries_published_stable": len(published),
        "entries_narrowed_below_cutline": len(rows) - len(published),
        "entries_on_active_waiver": sum(
            1 for r in rows if r.get("publication_state") == "published_on_waiver"
        ),
        "release_blocking_total": len(release_blocking),
        "release_blocking_published_stable": len(rb_published),
        "release_blocking_narrowed": len(release_blocking) - len(rb_published),
        "known_limit_entries": kind_count("known_limit"),
        "benchmark_entries": kind_count("benchmark"),
        "compatibility_entries": kind_count("compatibility"),
        "migration_entries": kind_count("migration"),
        "benchmark_budgets_within": sum(1 for b in budgets if budget_within(b)),
        "benchmark_budgets_regressed": sum(1 for b in budgets if not budget_within(b)),
        "packets_current": sum(1 for r in rows if packet_state(r) == "current"),
        "packets_due_for_refresh": sum(1 for r in rows if packet_state(r) == "due_for_refresh"),
        "packets_breached": sum(1 for r in rows if packet_state(r) == "breached"),
        "packets_missing": sum(1 for r in rows if packet_state(r) == "missing"),
        "total_active_gap_reasons": sum(len(reasons_of(r)) for r in rows),
        "publication_rules_firing": sum(1 for rule in rules if publication_rule_fires(rule, rows)),
    }


def computed_slo_state(packet: dict[str, Any], as_of: dt.date | None) -> str | None:
    """Recompute the freshness-SLO state from captured_at + target against as_of.
    Returns None when the date arithmetic cannot be performed."""
    captured = parse_date(packet.get("captured_at"))
    if captured is None:
        return "missing" if packet.get("captured_at") is None else None
    slo = packet.get("freshness_slo")
    if not isinstance(slo, dict):
        return None
    target = slo.get("target_max_age_days")
    warn = slo.get("warn_within_days")
    if not (is_int(target) and target >= 1):
        return None
    if not (is_int(warn) and warn >= 0):
        return None
    if as_of is None:
        return None
    age = (as_of - captured).days
    if age > target:
        return "breached"
    if (target - age) <= warn:
        return "due_for_refresh"
    return "current"


def validate_envelope(pack: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings: list[Finding] = []
    pack_id = str(pack.get("pack_id", "<pack>"))

    if pack.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(Finding("error", "packet.schema_version", "schema_version must be 1", pack_id))
    if pack.get("record_kind") != PACK_RECORD_KIND:
        findings.append(Finding("error", "packet.record_kind", "record_kind is not supported", pack_id))
    for field in (
        "pack_id",
        "status",
        "overview_page",
        "as_of",
        "claim_manifest_ref",
        "known_limits_register_ref",
        "benchmark_pack_template_ref",
        "compatibility_report_template_ref",
        "migration_contract_ref",
    ):
        if not is_str(pack.get(field)):
            findings.append(
                Finding("error", "packet.empty_field", f"{field} must be a non-empty string", pack_id)
            )
    if parse_date(pack.get("as_of")) is None:
        findings.append(Finding("error", "packet.as_of_invalid", "as_of must be an ISO date", pack_id))

    for field in (
        "known_limits_register_ref",
        "benchmark_pack_template_ref",
        "compatibility_report_template_ref",
        "migration_contract_ref",
    ):
        ref = pack.get(field)
        if is_str(ref) and not (repo_root / ref.split("#", 1)[0]).exists():
            findings.append(
                Finding("error", "packet.ref_missing", f"{field} does not exist: {ref}", pack_id)
            )

    for key, expected in (
        ("lifecycle_labels", list(LIFECYCLE_LABELS)),
        ("publication_kinds", list(PUBLICATION_KINDS)),
        ("publication_states", list(PUBLICATION_STATES)),
        ("gap_reasons", list(GAP_REASONS)),
        ("publication_actions", list(PUBLICATION_ACTIONS)),
    ):
        if list(pack.get(key, [])) != expected:
            findings.append(
                Finding("error", "packet.vocabulary", f"packet.{key} is not the closed vocabulary", key)
            )

    refs = pack.get("release_blocking_surface_refs")
    if not isinstance(refs, list) or not refs or any(not is_str(r) for r in refs):
        findings.append(
            Finding(
                "error",
                "packet.release_blocking_refs",
                "release_blocking_surface_refs must be a non-empty list of non-empty strings",
                pack_id,
            )
        )

    cutline = pack.get("launch_cutline")
    if not isinstance(cutline, dict):
        findings.append(Finding("error", "cutline.missing", "pack must carry a launch_cutline", pack_id))
    else:
        if cutline.get("cutline_level") != "stable":
            findings.append(Finding("error", "cutline.level", "cutline_level must be stable", pack_id))
        if list(cutline.get("above_cutline_levels", [])) != list(ABOVE_CUTLINE):
            findings.append(Finding("error", "cutline.above", "above_cutline_levels is not canonical", pack_id))
        if list(cutline.get("below_cutline_levels", [])) != list(BELOW_CUTLINE):
            findings.append(Finding("error", "cutline.below", "below_cutline_levels is not canonical", pack_id))
        if not is_str(cutline.get("description")):
            findings.append(Finding("error", "cutline.description", "cutline description must be non-empty", pack_id))
    return findings


def validate_rules(pack: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    rules = pack.get("publication_rules")
    if not isinstance(rules, list) or not rules:
        findings.append(Finding("error", "rules.empty", "pack must enumerate at least one publication rule", "<pack>"))
        return findings

    seen: set[str] = set()
    covered: set[str] = set()
    for rule in rules:
        rule_id = str(rule.get("rule_id", "<rule>"))
        for field in ("rule_id", "title", "rationale"):
            if not is_str(rule.get(field)):
                findings.append(Finding("error", "rule.empty_field", f"rule {field} must be non-empty", rule_id))
        if rule_id in seen:
            findings.append(Finding("error", "rule.duplicate_id", "rule ids must be unique", rule_id))
        seen.add(rule_id)
        if rule.get("trigger_reason") not in GAP_REASONS:
            findings.append(Finding("error", "rule.trigger_invalid", "trigger_reason is outside the vocabulary", rule_id))
        else:
            covered.add(rule["trigger_reason"])
        if rule.get("default_action") not in PUBLICATION_ACTIONS:
            findings.append(Finding("error", "rule.action_invalid", "default_action is outside the vocabulary", rule_id))
        labels = rule.get("applies_to_labels")
        if not isinstance(labels, list) or not labels:
            findings.append(Finding("error", "rule.labels_empty", "rule must watch at least one label", rule_id))
        elif any(label not in LIFECYCLE_LABELS for label in labels):
            findings.append(Finding("error", "rule.labels_invalid", "applies_to_labels has an unknown label", rule_id))
        if not isinstance(rule.get("blocks_publication"), bool):
            findings.append(Finding("error", "rule.blocks_invalid", "blocks_publication must be a boolean", rule_id))

    for reason in GAP_REASONS:
        if reason not in covered:
            findings.append(
                Finding("error", "rule.reason_uncovered", "gap reason has no rule watching for it", reason)
            )
    return findings


def validate_packet_block(entry_id: str, packet: Any) -> list[Finding]:
    findings: list[Finding] = []
    if not isinstance(packet, dict):
        findings.append(Finding("error", "row.packet_missing", "row must carry a proof_packet block", entry_id))
        return findings
    for field in ("packet_id", "packet_ref", "proof_index_ref"):
        if not is_str(packet.get(field)):
            findings.append(Finding("error", "packet.empty_field", f"proof_packet.{field} must be non-empty", entry_id))
    if packet.get("slo_state") not in FRESHNESS_SLO_STATES:
        findings.append(Finding("error", "packet.state_invalid", "slo_state is outside the vocabulary", entry_id))
    slo = packet.get("freshness_slo")
    if not isinstance(slo, dict):
        findings.append(Finding("error", "packet.slo_missing", "proof_packet must carry a freshness_slo block", entry_id))
        return findings
    if not is_str(slo.get("slo_register_ref")):
        findings.append(Finding("error", "packet.slo_register_missing", "freshness_slo.slo_register_ref must be non-empty", entry_id))
    target = slo.get("target_max_age_days")
    warn = slo.get("warn_within_days")
    if not (is_int(target) and target >= 1):
        findings.append(Finding("error", "packet.target_invalid", "target_max_age_days must be an integer >= 1", entry_id))
    if not (is_int(warn) and warn >= 0):
        findings.append(Finding("error", "packet.warn_invalid", "warn_within_days must be an integer >= 0", entry_id))
    if is_int(target) and is_int(warn) and warn > target:
        findings.append(Finding("error", "packet.slo_inconsistent", "warn_within_days may not exceed target_max_age_days", entry_id))
    return findings


def validate_benchmark(entry_id: str, row: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    kind = row.get("publication_kind")
    budget = budget_of(row)
    reasons = reasons_of(row)

    if kind == "benchmark" and budget is None:
        findings.append(Finding("error", "row.benchmark_without_budget", "benchmark publication carries no benchmark_budget", entry_id))
        return findings
    if kind != "benchmark" and budget is not None:
        findings.append(Finding("error", "row.non_benchmark_with_budget", "non-benchmark publication carries a benchmark_budget", entry_id))
        return findings
    if budget is None:
        return findings

    if not is_str(budget.get("metric_ref")):
        findings.append(Finding("error", "budget.metric_missing", "benchmark_budget.metric_ref must be non-empty", entry_id))
    for field in ("published_p50_ms", "published_p95_ms", "measured_p50_ms", "measured_p95_ms"):
        if not (is_int(budget.get(field)) and budget.get(field) >= 0):
            findings.append(Finding("error", "budget.number_invalid", f"benchmark_budget.{field} must be a non-negative integer", entry_id))
    if not isinstance(budget.get("tightened"), bool):
        findings.append(Finding("error", "budget.tightened_invalid", "benchmark_budget.tightened must be a boolean", entry_id))

    p50_pub = budget.get("published_p50_ms")
    p95_pub = budget.get("published_p95_ms")
    p50_m = budget.get("measured_p50_ms")
    p95_m = budget.get("measured_p95_ms")
    if all(is_int(v) for v in (p50_pub, p95_pub, p50_m, p95_m)):
        if not (p50_pub >= 1 and p95_pub >= p50_pub and p95_m >= p50_m):
            findings.append(Finding("error", "row.budget_disordered", "benchmark budget is not ordered p50 <= p95 with a positive published floor", entry_id))

    incomplete = not budget_complete(budget)
    if "corpus_metadata_missing" in reasons and not incomplete:
        findings.append(Finding("error", "row.corpus_reason_without_incomplete", "names corpus_metadata_missing but the budget carries corpus and trace refs", entry_id))
    if incomplete and "corpus_metadata_missing" not in reasons:
        findings.append(Finding("error", "row.incomplete_budget_without_reason", "benchmark budget is missing corpus or trace refs but does not name corpus_metadata_missing", entry_id))
    if "budget_regressed" in reasons and budget_within(budget):
        findings.append(Finding("error", "row.budget_regressed_reason_without_regression", "names budget_regressed but the measured numbers are within the published budget", entry_id))
    return findings


def validate_row(row: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings: list[Finding] = []
    entry_id = str(row.get("entry_id", "<row>"))

    for field in (
        "entry_id",
        "title",
        "surface_ref",
        "surface_summary",
        "claim_ref",
        "rationale",
    ):
        if not is_str(row.get(field)):
            findings.append(Finding("error", "row.empty_field", f"row {field} must be non-empty", entry_id))

    if row.get("publication_kind") not in PUBLICATION_KINDS:
        findings.append(Finding("error", "row.kind_invalid", "publication_kind is outside the vocabulary", entry_id))
    if not isinstance(row.get("release_blocking"), bool):
        findings.append(Finding("error", "row.release_blocking_invalid", "release_blocking must be a boolean", entry_id))

    claim_label = row.get("claim_label")
    published = row.get("published_label")
    state = row.get("publication_state")
    reasons = reasons_of(row)

    if claim_label not in LIFECYCLE_LABELS:
        findings.append(Finding("error", "row.claim_label_invalid", "claim_label is invalid", entry_id))
    if published not in LIFECYCLE_LABELS:
        findings.append(Finding("error", "row.published_invalid", "published_label is invalid", entry_id))
    if state not in PUBLICATION_STATES:
        findings.append(Finding("error", "row.state_invalid", "publication_state is invalid", entry_id))
    if any(reason not in GAP_REASONS for reason in reasons):
        findings.append(Finding("error", "row.reason_invalid", "active_gap_reasons has an unknown reason", entry_id))

    # The ceiling: no publication may carry a label wider than the public claim's label.
    if claim_label in LEVEL_RANK and published in LEVEL_RANK and LEVEL_RANK[published] > LEVEL_RANK[claim_label]:
        findings.append(Finding("error", "row.published_wider_than_claim", "published_label is wider than the claim ceiling", entry_id))

    findings.extend(validate_benchmark(entry_id, row))

    packet = row.get("proof_packet")
    findings.extend(validate_packet_block(entry_id, packet))
    slo_state = packet.get("slo_state") if isinstance(packet, dict) else None
    budget = budget_of(row)

    holds = state in HOLDING_STATES
    claim_holds = is_above_cutline(claim_label)

    # A claim canonically below the cutline forces the publication to inherit the ceiling
    # and narrow.
    if claim_label in LEVEL_RANK and not claim_holds:
        if holds:
            findings.append(Finding("error", "row.held_on_narrowed_claim", "a row holds its label while the public claim label is below the cutline", entry_id))
        if "claim_label_narrowed" not in reasons:
            findings.append(Finding("error", "row.claim_narrowed_without_reason", "a row whose claim is narrowed must name claim_label_narrowed", entry_id))

    if state in NARROWING_STATES:
        if is_above_cutline(published):
            findings.append(Finding("error", "row.published_not_narrowed", "a publication that is not backed must narrow below the cutline", entry_id))
        if not reasons:
            findings.append(Finding("error", "row.narrowing_without_reason", "a narrowing row must name an active gap reason", entry_id))
        if slo_state == "breached" and "proof_packet_freshness_breached" not in reasons:
            findings.append(Finding("error", "row.breached_without_reason", "a breached packet must name proof_packet_freshness_breached", entry_id))
        if slo_state == "missing" and "proof_packet_missing" not in reasons:
            findings.append(Finding("error", "row.missing_without_reason", "a missing packet must name proof_packet_missing", entry_id))
        if budget is not None and not budget_within(budget) and "budget_regressed" not in reasons:
            findings.append(Finding("error", "row.budget_regressed_without_reason", "a benchmark over its budget must name budget_regressed", entry_id))
    if holds:
        if claim_label != published:
            findings.append(Finding("error", "row.held_label_not_equal_claim", "a backed row must carry the public claim's canonical label", entry_id))
        if reasons:
            findings.append(Finding("error", "row.held_with_active_gap", "a backed row must carry no active gap reason", entry_id))
        if isinstance(packet, dict) and not has_capture(packet):
            findings.append(Finding("error", "row.held_without_fresh_packet", "a backed row must ride a captured, evidence-backed proof packet", entry_id))
        if slo_state in ("breached", "missing"):
            findings.append(Finding("error", "row.held_on_stale_packet", "a backed row must ride a packet within its freshness SLO", entry_id))
        if budget is not None:
            if not budget_complete(budget):
                findings.append(Finding("error", "row.held_with_incomplete_budget", "a backed benchmark row must carry corpus and trace refs", entry_id))
            if not budget_within(budget) and not waiver_covers_tightened(row):
                findings.append(Finding("error", "row.held_over_budget", "a backed benchmark row must be within its published budget unless a tightened-budget waiver covers it", entry_id))
        signoff = row.get("owner_signoff", {})
        if not (isinstance(signoff, dict) and signoff.get("signed_off") is True and is_str(signoff.get("signed_at"))):
            findings.append(Finding("error", "row.held_without_signoff", "a backed row must carry an owner sign-off with a date", entry_id))
    signoff = row.get("owner_signoff", {})
    if not (isinstance(signoff, dict) and is_str(signoff.get("owner_ref"))):
        findings.append(Finding("error", "row.signoff_owner_missing", "owner_signoff.owner_ref must be non-empty", entry_id))

    findings.extend(validate_state_reason_coherence(entry_id, state, reasons, row))
    findings.extend(validate_row_refs(entry_id, row, repo_root))
    return findings


def validate_state_reason_coherence(entry_id: str, state: Any, reasons: list[str], row: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    if state == "narrowed_unbacked":
        allowed = {
            "surface_capability_absent",
            "evidence_incomplete",
            "corpus_metadata_missing",
            "owner_signoff_missing",
        }
        if not allowed.intersection(reasons):
            findings.append(Finding("error", "row.state_reason_incoherent", "narrowed_unbacked requires a capability/evidence/corpus/signoff reason", entry_id))
    if state == "narrowed_claim_narrowed" and "claim_label_narrowed" not in reasons:
        findings.append(Finding("error", "row.state_reason_incoherent", "narrowed_claim_narrowed requires the claim_label_narrowed reason", entry_id))
    if state == "narrowed_stale" and not (
        "proof_packet_freshness_breached" in reasons or "proof_packet_missing" in reasons
    ):
        findings.append(Finding("error", "row.state_reason_incoherent", "narrowed_stale requires a packet-freshness reason", entry_id))
    if state == "narrowed_waiver_expired":
        if "waiver_expired" not in reasons:
            findings.append(Finding("error", "row.state_reason_incoherent", "narrowed_waiver_expired requires the waiver_expired reason", entry_id))
        if not isinstance(row.get("waiver"), dict):
            findings.append(Finding("error", "row.waiver_state_without_waiver", "narrowed_waiver_expired state must name a waiver", entry_id))
    if state == "narrowed_budget_regressed":
        if "budget_regressed" not in reasons:
            findings.append(Finding("error", "row.state_reason_incoherent", "narrowed_budget_regressed requires the budget_regressed reason", entry_id))
        if row.get("publication_kind") != "benchmark":
            findings.append(Finding("error", "row.budget_state_on_non_benchmark", "narrowed_budget_regressed state names a non-benchmark publication", entry_id))
    if state == "published_on_waiver":
        waiver = row.get("waiver")
        if not isinstance(waiver, dict) or not is_str(waiver.get("waiver_ref")) or not is_str(waiver.get("expires_at")):
            findings.append(Finding("error", "row.waiver_state_without_waiver", "published_on_waiver state must name a waiver", entry_id))
    return findings


def validate_row_refs(entry_id: str, row: dict[str, Any], repo_root: Path) -> list[Finding]:
    """Path-existence checks for the proof-packet and benchmark refs (not the
    cross-artifact ids, which resolve against the claim manifest)."""
    findings: list[Finding] = []
    refs: list[str] = []
    packet = row.get("proof_packet")
    if isinstance(packet, dict):
        for key in ("packet_ref", "proof_index_ref"):
            ref = packet.get(key)
            if is_str(ref):
                refs.append(ref.split("#", 1)[0])
        slo = packet.get("freshness_slo")
        if isinstance(slo, dict) and is_str(slo.get("slo_register_ref")):
            refs.append(slo["slo_register_ref"].split("#", 1)[0])
        for ref in packet.get("evidence_refs", []) or []:
            if is_str(ref):
                refs.append(ref.split("#", 1)[0])
    budget = budget_of(row)
    if budget is not None:
        for key in ("corpus_ref", "trace_ref"):
            ref = budget.get(key)
            if is_str(ref):
                refs.append(ref.split("#", 1)[0])
    for ref in refs:
        if not (repo_root / ref).exists():
            findings.append(Finding("error", "row.ref_missing", f"referenced artifact does not exist: {ref}", entry_id))
    return findings


def validate_coverage(pack: dict[str, Any]) -> list[Finding]:
    """Every publication kind is represented, every declared release-blocking surface is
    covered by exactly one release-blocking row, every release-blocking row is declared,
    and no surface ref repeats."""
    findings: list[Finding] = []
    rows = pack.get("rows", [])

    seen: set[str] = set()
    for row in rows:
        ref = row.get("surface_ref")
        if not is_str(ref):
            continue
        if ref in seen:
            findings.append(Finding("error", "coverage.surface_duplicate", f"surface ref appears more than once: {ref}", ref))
        seen.add(ref)

    declared = pack.get("release_blocking_surface_refs", [])
    declared_set = {r for r in declared if is_str(r)} if isinstance(declared, list) else set()
    covered = {
        str(row.get("surface_ref"))
        for row in rows
        if row.get("release_blocking") is True and is_str(row.get("surface_ref"))
    }
    for ref in sorted(declared_set):
        if ref not in covered:
            findings.append(Finding("error", "coverage.release_blocking_uncovered", f"declared release-blocking surface has no covering row: {ref}", ref))
    for row in rows:
        if row.get("release_blocking") is True and is_str(row.get("surface_ref")) and row["surface_ref"] not in declared_set:
            findings.append(Finding("error", "coverage.release_blocking_not_declared", "a release-blocking row's surface is not in release_blocking_surface_refs", str(row.get("entry_id"))))

    present_kinds = {row.get("publication_kind") for row in rows}
    for kind in PUBLICATION_KINDS:
        if kind not in present_kinds:
            findings.append(Finding("error", "coverage.publication_kind_absent", f"publication kind has no row: {kind}", kind))
    return findings


def validate_freshness(pack: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    as_of = parse_date(pack.get("as_of"))
    if as_of is None:
        return findings
    for row in pack.get("rows", []):
        entry_id = str(row.get("entry_id", "<row>"))
        packet = row.get("proof_packet")
        if not isinstance(packet, dict):
            continue
        declared = packet.get("slo_state")
        computed = computed_slo_state(packet, as_of)
        if computed is None or declared not in FRESHNESS_RANK or computed not in FRESHNESS_RANK:
            continue
        if FRESHNESS_RANK[declared] > FRESHNESS_RANK[computed]:
            findings.append(
                Finding("error", "row.packet_freshness_overstated", f"declared slo_state {declared!r} is fresher than the {computed!r} the as_of date allows", entry_id)
            )
        if row.get("publication_state") in HOLDING_STATES and computed in ("breached", "missing"):
            findings.append(
                Finding("error", "row.published_on_breached_packet", "a backed publication rides a proof packet past its freshness SLO against as_of", entry_id)
            )
    return findings


def validate_waivers(pack: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    as_of = parse_date(pack.get("as_of"))
    if as_of is None:
        return findings
    for row in pack.get("rows", []):
        entry_id = str(row.get("entry_id", "<row>"))
        state = row.get("publication_state")
        waiver = row.get("waiver")
        if not isinstance(waiver, dict):
            continue
        expires = parse_date(waiver.get("expires_at"))
        if expires is None:
            findings.append(Finding("error", "row.waiver_expiry_invalid", "waiver expires_at must be an ISO date", entry_id))
            continue
        expired = as_of >= expires
        if expired and state == "published_on_waiver":
            findings.append(Finding("error", "row.published_on_expired_waiver", "a publication holds its label on a waiver that has expired against as_of", entry_id))
        if not expired and state == "narrowed_waiver_expired":
            findings.append(Finding("error", "row.waiver_expired_but_active", "a row is marked waiver-expired but the waiver is still active against as_of", entry_id))
    return findings


def validate_ceiling(pack: dict[str, Any], repo_root: Path) -> list[Finding]:
    """Cross-artifact ingestion: read the stable claim manifest and fail when a row's
    claim_label is not the label the claim manifest publishes for the entry named by
    claim_ref, or names an entry the claim manifest does not carry."""
    findings: list[Finding] = []
    ref = pack.get("claim_manifest_ref")
    if not is_str(ref):
        return findings
    path = repo_root / ref
    if not path.exists():
        findings.append(Finding("error", "ceiling.artifact_missing", f"claim_manifest_ref does not exist: {ref}", "<pack>"))
        return findings
    try:
        claim_manifest = json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        findings.append(Finding("error", "ceiling.artifact_invalid", f"claim_manifest_ref is not valid JSON: {exc}", ref))
        return findings

    published_by_entry = {}
    if isinstance(claim_manifest, dict):
        published_by_entry = {
            str(e.get("entry_id")): e.get("published_label")
            for e in claim_manifest.get("entries", [])
            if isinstance(e, dict)
        }

    for row in pack.get("rows", []):
        entry_id = str(row.get("entry_id", "<row>"))
        claim_ref = row.get("claim_ref")
        if not is_str(claim_ref):
            continue
        if claim_ref not in published_by_entry:
            findings.append(Finding("error", "ceiling.claim_entry_unknown", f"claim_ref is not an entry in the stable claim manifest: {claim_ref}", entry_id))
            continue
        canonical = published_by_entry[claim_ref]
        if row.get("claim_label") != canonical:
            findings.append(Finding("error", "ceiling.claim_label_mismatch", f"claim_label {row.get('claim_label')!r} does not match the claim manifest published_label {canonical!r}", entry_id))
    return findings


def validate_publication(pack: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    publication = pack.get("publication")
    if not isinstance(publication, dict):
        findings.append(Finding("error", "publication.missing", "pack must carry a publication block", "<pack>"))
        return findings
    for field in ("publication_gate", "rationale"):
        if not is_str(publication.get(field)):
            findings.append(Finding("error", "publication.empty_field", f"publication.{field} must be non-empty", "<pack>"))
    decision, rule_ids, entry_ids = computed_publication(pack)
    if publication.get("decision") != decision:
        findings.append(
            Finding("error", "publication.decision_inconsistent", f"declared decision {publication.get('decision')!r} disagrees with computed {decision!r}", "<pack>")
        )
    if list(publication.get("blocking_rule_ids", [])) != rule_ids:
        findings.append(Finding("error", "publication.blocking_rules_mismatch", "blocking_rule_ids disagrees with the firing publication rules", "<pack>"))
    if list(publication.get("blocking_entry_ids", [])) != entry_ids:
        findings.append(Finding("error", "publication.blocking_entries_mismatch", "blocking_entry_ids disagrees with the firing publication rules", "<pack>"))
    return findings


def validate_summary(pack: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    summary = pack.get("summary")
    if not isinstance(summary, dict):
        findings.append(Finding("error", "summary.missing", "pack must carry a summary block", "<pack>"))
        return findings
    expected = computed_summary(pack)
    for key, value in expected.items():
        if summary.get(key) != value:
            findings.append(Finding("error", "summary.count_mismatch", f"summary.{key} must equal the recomputed value", key))
    return findings


def validate_pack(pack: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings = validate_envelope(pack, repo_root)
    findings.extend(validate_rules(pack))

    rows = pack.get("rows")
    if not isinstance(rows, list) or not rows:
        findings.append(Finding("error", "packet.rows_empty", "pack must enumerate at least one row", "<pack>"))
        return findings

    seen: set[str] = set()
    for raw in rows:
        row = ensure_dict(raw, "packet.rows[]")
        findings.extend(validate_row(row, repo_root))
        entry_id = str(row.get("entry_id", "<row>"))
        if entry_id in seen:
            findings.append(Finding("error", "row.duplicate_id", "entry ids must be unique", entry_id))
        seen.add(entry_id)

    findings.extend(validate_coverage(pack))
    findings.extend(validate_freshness(pack))
    findings.extend(validate_waivers(pack))
    findings.extend(validate_ceiling(pack, repo_root))
    findings.extend(validate_publication(pack))
    findings.extend(validate_summary(pack))
    return findings


def recompute_derived(pack: dict[str, Any]) -> None:
    pack["summary"] = computed_summary(pack)
    decision, rule_ids, entry_ids = computed_publication(pack)
    if isinstance(pack.get("publication"), dict):
        pack["publication"].update(
            {"decision": decision, "blocking_rule_ids": rule_ids, "blocking_entry_ids": entry_ids}
        )


def run_negative_drills(pack: dict[str, Any], repo_root: Path) -> tuple[list[dict[str, str]], list[Finding]]:
    results: list[dict[str, str]] = []
    findings: list[Finding] = []

    def record(drill_id: str, expected_check_id: str, fired: bool) -> None:
        results.append({"drill_id": drill_id, "expected_check_id": expected_check_id, "status": "passed" if fired else "failed"})
        if not fired:
            findings.append(Finding("error", "negative_drill.not_rejected", f"negative drill {drill_id} did not fire", drill_id))

    def check_ids(candidate: dict[str, Any]) -> set[str]:
        return {f.check_id for f in validate_pack(candidate, repo_root)}

    # A narrowing row that still publishes a stable label must be flagged.
    mutated = copy.deepcopy(pack)
    target = next((r for r in mutated["rows"] if r.get("publication_state") in NARROWING_STATES), None)
    if target is not None:
        target["published_label"] = "stable"
        recompute_derived(mutated)
        record("published_not_narrowed_rejected", "row.published_not_narrowed", "row.published_not_narrowed" in check_ids(mutated))

    # A backed row carrying an active gap reason must be flagged.
    mutated = copy.deepcopy(pack)
    target = next((r for r in mutated["rows"] if r.get("publication_state") in HOLDING_STATES), None)
    if target is not None:
        target["active_gap_reasons"] = ["evidence_incomplete"]
        recompute_derived(mutated)
        record("held_with_active_gap_rejected", "row.held_with_active_gap", "row.held_with_active_gap" in check_ids(mutated))

    # A backed row whose packet is breached against as_of must be flagged.
    mutated = copy.deepcopy(pack)
    target = next((r for r in mutated["rows"] if r.get("publication_state") in HOLDING_STATES), None)
    if target is not None:
        target["proof_packet"]["captured_at"] = "2000-01-01"
        record("published_on_breached_packet_rejected", "row.published_on_breached_packet", "row.published_on_breached_packet" in check_ids(mutated))

    # A packet whose declared state overstates its freshness must be flagged.
    mutated = copy.deepcopy(pack)
    target = next(
        (r for r in mutated["rows"] if r.get("proof_packet", {}).get("slo_state") == "current"),
        None,
    )
    if target is not None:
        target["proof_packet"]["captured_at"] = "2000-01-01"
        record("packet_freshness_overstated_rejected", "row.packet_freshness_overstated", "row.packet_freshness_overstated" in check_ids(mutated))

    # A row whose claim_label disagrees with the claim manifest must be flagged.
    mutated = copy.deepcopy(pack)
    target = next((r for r in mutated["rows"] if r.get("claim_label") == "beta"), None)
    if target is not None:
        target["claim_label"] = "stable"
        record("claim_label_mismatch_rejected", "ceiling.claim_label_mismatch", "ceiling.claim_label_mismatch" in check_ids(mutated))

    # A publication carried wider than its public claim's ceiling must be flagged.
    mutated = copy.deepcopy(pack)
    target = next(
        (r for r in mutated["rows"] if r.get("claim_label") == "beta" and not is_above_cutline(r.get("published_label"))),
        None,
    )
    if target is not None:
        target["published_label"] = "lts"
        recompute_derived(mutated)
        record("published_wider_than_claim_rejected", "row.published_wider_than_claim", "row.published_wider_than_claim" in check_ids(mutated))

    # A backed benchmark row pushed over its published budget must be flagged.
    mutated = copy.deepcopy(pack)
    target = next(
        (
            r
            for r in mutated["rows"]
            if r.get("publication_state") in HOLDING_STATES and budget_of(r) is not None and budget_within(budget_of(r))
        ),
        None,
    )
    if target is not None:
        target["benchmark_budget"]["measured_p95_ms"] = target["benchmark_budget"]["published_p95_ms"] + 1000
        recompute_derived(mutated)
        record("held_over_budget_rejected", "row.held_over_budget", "row.held_over_budget" in check_ids(mutated))

    # A benchmark stripped of its corpus metadata must be flagged.
    mutated = copy.deepcopy(pack)
    target = next((r for r in mutated["rows"] if budget_of(r) is not None), None)
    if target is not None:
        target["benchmark_budget"]["corpus_ref"] = ""
        recompute_derived(mutated)
        record("incomplete_budget_without_reason_rejected", "row.incomplete_budget_without_reason", "row.incomplete_budget_without_reason" in check_ids(mutated))

    # Dropping a declared release-blocking surface's row must be flagged.
    mutated = copy.deepcopy(pack)
    declared = mutated.get("release_blocking_surface_refs", [])
    if declared:
        dropped = declared[0]
        mutated["rows"] = [r for r in mutated["rows"] if r.get("surface_ref") != dropped]
        recompute_derived(mutated)
        record("release_blocking_uncovered_rejected", "coverage.release_blocking_uncovered", "coverage.release_blocking_uncovered" in check_ids(mutated))

    # Removing every row of a publication kind must be flagged.
    mutated = copy.deepcopy(pack)
    mutated["rows"] = [r for r in mutated["rows"] if r.get("publication_kind") != "migration"]
    recompute_derived(mutated)
    record("publication_kind_absent_rejected", "coverage.publication_kind_absent", "coverage.publication_kind_absent" in check_ids(mutated))

    # A publication decision that disagrees with the firing rules must be flagged.
    mutated = copy.deepcopy(pack)
    mutated["publication"]["decision"] = "proceed" if mutated["publication"].get("decision") == "hold" else "hold"
    record("publication_decision_inconsistent_rejected", "publication.decision_inconsistent", "publication.decision_inconsistent" in check_ids(mutated))

    return results, findings


def run_fixture_cases(repo_root: Path, fixtures_dir: str) -> tuple[list[dict[str, str]], list[Finding]]:
    results: list[dict[str, str]] = []
    findings: list[Finding] = []
    manifest_path = repo_root / fixtures_dir / "cases.json"
    if not manifest_path.exists():
        return results, findings
    try:
        manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        findings.append(Finding("error", "fixture.manifest_invalid", f"fixture manifest is not valid JSON: {exc}", str(manifest_path)))
        return results, findings

    for case in manifest.get("cases", []):
        case_id = str(case.get("case_id", "<case>"))
        rel = case.get("file")
        expected = case.get("expected_check_id")
        outcome = case.get("expected_outcome", "rejected")
        case_path = repo_root / fixtures_dir / rel if is_str(rel) else None
        if case_path is None or not case_path.exists():
            findings.append(Finding("error", "fixture.missing", f"fixture file does not exist: {rel}", case_id))
            continue
        try:
            candidate = json.loads(case_path.read_text(encoding="utf-8"))
        except json.JSONDecodeError as exc:
            findings.append(Finding("error", "fixture.invalid_json", f"fixture is not valid JSON: {exc}", case_id))
            continue
        ids = {f.check_id for f in validate_pack(ensure_dict(candidate, "fixture"), repo_root)}
        if outcome == "rejected":
            fired = (expected in ids) if is_str(expected) else bool(ids)
            results.append({"case_id": case_id, "expected_check_id": expected or "<any>", "status": "passed" if fired else "failed"})
            if not fired:
                findings.append(Finding("error", "fixture.not_rejected", f"fixture case {case_id} should be rejected with {expected!r} but was not", case_id))
        else:
            results.append({"case_id": case_id, "expected_check_id": "<none>", "status": "passed" if not ids else "failed"})
            if ids:
                findings.append(Finding("error", "fixture.unexpected_rejection", f"fixture case {case_id} should validate clean but was rejected", case_id))
    return results, findings


def write_report(
    path: Path,
    pack: dict[str, Any],
    findings: list[Finding],
    drill_results: list[dict[str, str]],
    fixture_results: list[dict[str, str]],
) -> None:
    decision, rule_ids, entry_ids = computed_publication(pack)
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "record_kind": "stable_publication_pack_validation_capture",
        "status": "pass" if not any(f.severity == "error" for f in findings) else "fail",
        "generated_at": generated_at_now(),
        "pack_id": pack.get("pack_id"),
        "as_of": pack.get("as_of"),
        "summary": pack.get("summary"),
        "publication": {
            "decision": decision,
            "blocking_rule_ids": rule_ids,
            "blocking_entry_ids": entry_ids,
        },
        "negative_drills": drill_results,
        "fixture_cases": fixture_results,
        "findings": [f.as_report() for f in findings],
    }
    path.write_text(json_text(payload), encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    pack = ensure_dict(
        load_json(repo_root / args.pack, "stable publication pack"),
        "stable publication pack",
    )

    findings = validate_pack(pack, repo_root)
    drill_results, drill_findings = run_negative_drills(pack, repo_root)
    findings.extend(drill_findings)
    fixture_results, fixture_findings = run_fixture_cases(repo_root, args.fixtures_dir)
    findings.extend(fixture_findings)

    report_rel = args.report
    if report_rel is None and not args.no_capture:
        report_rel = DEFAULT_REPORT_REL
    if report_rel:
        write_report(repo_root / report_rel, pack, findings, drill_results, fixture_results)

    errors = [f for f in findings if f.severity == "error"]
    if errors:
        for item in errors:
            print(f"ERROR [{item.check_id}] ({item.ref}): {item.message}", file=sys.stderr)
        return 1

    summary = pack.get("summary", {})
    decision, rule_ids, _entry_ids = computed_publication(pack)
    print(
        "stable publication pack validated "
        f"({summary.get('total_entries')} publications across {summary.get('total_claims')} claims, "
        f"{summary.get('entries_published_stable')} published stable, "
        f"{summary.get('entries_narrowed_below_cutline')} narrowed; "
        f"kinds known_limit={summary.get('known_limit_entries')} benchmark={summary.get('benchmark_entries')} "
        f"compatibility={summary.get('compatibility_entries')} migration={summary.get('migration_entries')}; "
        f"benchmark budgets {summary.get('benchmark_budgets_within')} within / "
        f"{summary.get('benchmark_budgets_regressed')} regressed; "
        f"release-blocking {summary.get('release_blocking_published_stable')}/{summary.get('release_blocking_total')} published; "
        f"packets {summary.get('packets_current')} current / "
        f"{summary.get('packets_due_for_refresh')} due / "
        f"{summary.get('packets_breached')} breached / "
        f"{summary.get('packets_missing')} missing; "
        f"{summary.get('publication_rules_firing')} rules firing; "
        f"publication={decision}; {len(drill_results)} negative drills, {len(fixture_results)} fixture cases)"
    )

    if args.require_proceed and decision == "hold":
        print(
            f"PUBLICATION HELD: stable publication blocked by {len(rule_ids)} rule(s): {', '.join(rule_ids)}",
            file=sys.stderr,
        )
        return 2

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
