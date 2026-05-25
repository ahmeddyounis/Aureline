#!/usr/bin/env python3
"""Validate the design-partner, certified-archetype, and stable-cohort scoreboards packet.

The stable claim manifest decides the single canonical lifecycle label each *subject*
publishes; the stable qualification matrix and the stable proof index ground the surfaces
that are meant to ship at the cutline. This packet is the signoff-loop layer beside those
gates: for every scoreboard row — in the design-partner, certified-archetype, or
stable-cohort lane — it asks whether the row carries a fresh, complete, owner-signed
scoreboard packet, metrics that clear their thresholds, and a complete required signoff loop,
or whether it is riding on optimism and an adjacent signed-off row. It reads the checked-in
packet at ``artifacts/release/cohort_scoreboards.json`` and:

  - asserts the closed vocabularies (lifecycle labels, lanes, states, gap reasons, actions)
    and the launch cutline are canonical;
  - asserts every signed-off row renders the public claim's canonical label cleanly with a
    captured within-SLO packet, passing metrics, an owner sign-off, a complete signoff loop,
    and no expired waiver, and that every narrowing row drops below the cutline rather than
    rendering a label wider than the public claim;
  - asserts lane coverage (every lane has at least one row), the release-blocking set is
    mutually covered, and no scoreboard id repeats;
  - performs the ceiling cross-check the typed model cannot — it reads the stable claim
    manifest named by ``claim_manifest_ref`` and fails when a row's ``claim_label`` is not the
    label the claim manifest publishes for the entry named by ``claim_ref``, or names an entry
    the claim manifest does not carry;
  - performs the packet-freshness SLO automation and the waiver-expiry date arithmetic against
    the packet ``as_of`` date the typed model cannot;
  - recomputes the publication verdict and the summary block, and fails on any drift;
  - runs negative drills proving the narrowing, ceiling, freshness, signoff, metric, and
    publication rejections all fire; and
  - runs the checked-in fixture cases under ``fixtures/release/cohort_scoreboards`` and fails
    when a case the packet marks as rejected validates clean.

With ``--require-proceed`` the gate additionally fails (exit code 2) when the recomputed
publication verdict is ``hold``, so shiproom and release tooling can fail promotion directly
from this artifact.

The typed Rust consumer
(``aureline_release::finalize_design_partner_certified_archetype_and_stable_cohort::current_cohort_scoreboards``)
reads the same packet and runs the same structural cross-check, so this gate and
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


DEFAULT_PACKET_REL = "artifacts/release/cohort_scoreboards.json"
DEFAULT_REPORT_REL = "artifacts/release/captures/cohort_scoreboards_validation_capture.json"
DEFAULT_FIXTURES_REL = "fixtures/release/cohort_scoreboards"

EXPECTED_SCHEMA_VERSION = 1
PACKET_RECORD_KIND = "cohort_scoreboards"

LIFECYCLE_LABELS = ("lts", "stable", "beta", "preview", "withdrawn")
ABOVE_CUTLINE = ("lts", "stable")
BELOW_CUTLINE = ("beta", "preview", "withdrawn")
SCOREBOARD_LANES = ("design_partner", "certified_archetype", "stable_cohort")
SCOREBOARD_STATES = (
    "signed_off",
    "signed_off_on_waiver",
    "narrowed_unbacked",
    "narrowed_claim_narrowed",
    "narrowed_stale",
    "narrowed_waiver_expired",
)
GAP_REASONS = (
    "claim_label_narrowed",
    "scoreboard_evidence_incomplete",
    "scoreboard_packet_freshness_breached",
    "scoreboard_packet_missing",
    "waiver_expired",
    "owner_signoff_missing",
    "required_signoff_missing",
    "score_below_threshold",
)
SCOREBOARD_ACTIONS = (
    "hold_publication",
    "narrow_scoreboard_label",
    "refresh_scoreboard_packet",
    "complete_signoff_loop",
    "recapture_scoreboard_evidence",
)
FRESHNESS_SLO_STATES = ("current", "due_for_refresh", "breached", "missing")
HOLDING_STATES = ("signed_off", "signed_off_on_waiver")
NARROWING_STATES = (
    "narrowed_unbacked",
    "narrowed_claim_narrowed",
    "narrowed_stale",
    "narrowed_waiver_expired",
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
    parser.add_argument("--packet", default=DEFAULT_PACKET_REL)
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


def reasons_of(row: dict[str, Any]) -> list[str]:
    reasons = row.get("active_gap_reasons", [])
    return reasons if isinstance(reasons, list) else []


def packet_of(row: dict[str, Any]) -> dict[str, Any]:
    packet = row.get("scoreboard_packet")
    return packet if isinstance(packet, dict) else {}


def has_capture(packet: dict[str, Any]) -> bool:
    return bool(packet.get("captured_at")) and bool(packet.get("evidence_refs"))


def metrics_of(row: dict[str, Any]) -> list[dict[str, Any]]:
    metrics = row.get("metrics", [])
    return [m for m in metrics if isinstance(m, dict)] if isinstance(metrics, list) else []


def metric_passes(metric: dict[str, Any]) -> bool:
    measured = metric.get("measured")
    threshold = metric.get("threshold")
    return is_int(measured) and is_int(threshold) and measured >= threshold


def failing_metric_count(row: dict[str, Any]) -> int:
    return sum(1 for m in metrics_of(row) if not metric_passes(m))


def all_metrics_pass(row: dict[str, Any]) -> bool:
    metrics = metrics_of(row)
    return bool(metrics) and all(metric_passes(m) for m in metrics)


def required_signoffs_of(row: dict[str, Any]) -> list[dict[str, Any]]:
    loop = row.get("signoff_loop")
    if not isinstance(loop, dict):
        return []
    signoffs = loop.get("required_signoffs", [])
    return [s for s in signoffs if isinstance(s, dict)] if isinstance(signoffs, list) else []


def required_signoff_complete(signoff: dict[str, Any]) -> bool:
    return (
        signoff.get("signed_off") is True
        and is_str(signoff.get("signed_at"))
        and is_str(signoff.get("signer_ref"))
    )


def signoff_loop_complete(row: dict[str, Any]) -> bool:
    signoffs = required_signoffs_of(row)
    return bool(signoffs) and all(required_signoff_complete(s) for s in signoffs)


def owner_signed(row: dict[str, Any]) -> bool:
    signoff = row.get("owner_signoff")
    return (
        isinstance(signoff, dict)
        and signoff.get("signed_off") is True
        and is_str(signoff.get("signed_at"))
    )


def rule_fires(rule: dict[str, Any], rows: list[dict[str, Any]]) -> bool:
    trigger = rule.get("trigger_reason")
    labels = rule.get("applies_to_labels", [])
    if not isinstance(labels, list):
        return False
    for row in rows:
        if row.get("claim_label") in labels and trigger in reasons_of(row):
            return True
    return False


def computed_publication(packet: dict[str, Any]) -> tuple[str, list[str], list[str]]:
    rows = packet.get("rows", [])
    rules = packet.get("rules", [])
    blocking_rules = [
        rule
        for rule in rules
        if rule.get("blocks_publication") is True and rule_fires(rule, rows)
    ]
    rule_ids = sorted(str(rule.get("rule_id")) for rule in blocking_rules)
    blocking_triggers = {rule.get("trigger_reason") for rule in blocking_rules}
    scoreboard_ids: set[str] = set()
    for row in rows:
        if is_above_cutline(row.get("claim_label")) and blocking_triggers.intersection(
            reasons_of(row)
        ):
            scoreboard_ids.add(str(row.get("scoreboard_id")))
    decision = "hold" if blocking_rules else "proceed"
    return decision, rule_ids, sorted(scoreboard_ids)


def computed_summary(packet: dict[str, Any]) -> dict[str, Any]:
    rows = packet.get("rows", [])
    rules = packet.get("rules", [])

    def packet_count(state: str) -> int:
        return sum(1 for r in rows if packet_of(r).get("slo_state") == state)

    holding = [r for r in rows if is_above_cutline(r.get("effective_label"))]
    release_blocking = [r for r in rows if r.get("release_blocking") is True]
    rb_holding = [r for r in release_blocking if is_above_cutline(r.get("effective_label"))]
    claims = {str(r.get("claim_ref")) for r in rows}
    total_metrics = sum(len(metrics_of(r)) for r in rows)
    metrics_failing = sum(failing_metric_count(r) for r in rows)
    return {
        "total_rows": len(rows),
        "total_claims": len(claims),
        "rows_holding_stable": len(holding),
        "rows_narrowed_below_cutline": len(rows) - len(holding),
        "release_blocking_total": len(release_blocking),
        "release_blocking_holding_stable": len(rb_holding),
        "release_blocking_narrowed": len(release_blocking) - len(rb_holding),
        "rows_on_active_waiver": sum(
            1 for r in rows if r.get("scoreboard_state") == "signed_off_on_waiver"
        ),
        "packets_current": packet_count("current"),
        "packets_due_for_refresh": packet_count("due_for_refresh"),
        "packets_breached": packet_count("breached"),
        "packets_missing": packet_count("missing"),
        "complete_signoff_loops": sum(1 for r in rows if signoff_loop_complete(r)),
        "incomplete_signoff_loops": sum(1 for r in rows if not signoff_loop_complete(r)),
        "total_metrics": total_metrics,
        "metrics_passing": total_metrics - metrics_failing,
        "metrics_failing": metrics_failing,
        "total_active_gap_reasons": sum(len(reasons_of(r)) for r in rows),
        "rules_firing": sum(1 for rule in rules if rule_fires(rule, rows)),
    }


def computed_slo_state(packet: dict[str, Any], as_of: dt.date | None) -> str | None:
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


def validate_envelope(packet: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings: list[Finding] = []
    packet_id = str(packet.get("packet_id", "<packet>"))

    if packet.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(Finding("error", "scoreboards.schema_version", "schema_version must be 1", packet_id))
    if packet.get("record_kind") != PACKET_RECORD_KIND:
        findings.append(Finding("error", "scoreboards.record_kind", "record_kind is not supported", packet_id))
    for field in ("packet_id", "status", "overview_page", "as_of", "claim_manifest_ref", "stable_proof_index_ref"):
        if not is_str(packet.get(field)):
            findings.append(Finding("error", "scoreboards.empty_field", f"{field} must be a non-empty string", packet_id))
    if parse_date(packet.get("as_of")) is None:
        findings.append(Finding("error", "scoreboards.as_of_invalid", "as_of must be an ISO date", packet_id))

    for field in ("claim_manifest_ref", "stable_proof_index_ref"):
        ref = packet.get(field)
        if is_str(ref) and not (repo_root / ref.split("#", 1)[0]).exists():
            findings.append(Finding("error", "scoreboards.ref_missing", f"{field} does not exist: {ref}", packet_id))

    for key, expected in (
        ("lifecycle_labels", list(LIFECYCLE_LABELS)),
        ("scoreboard_lanes", list(SCOREBOARD_LANES)),
        ("scoreboard_states", list(SCOREBOARD_STATES)),
        ("gap_reasons", list(GAP_REASONS)),
        ("scoreboard_actions", list(SCOREBOARD_ACTIONS)),
    ):
        if list(packet.get(key, [])) != expected:
            findings.append(Finding("error", "scoreboards.vocabulary", f"scoreboards.{key} is not the closed vocabulary", key))

    cutline = packet.get("launch_cutline")
    if not isinstance(cutline, dict):
        findings.append(Finding("error", "cutline.missing", "packet must carry a launch_cutline", packet_id))
    else:
        if cutline.get("cutline_level") != "stable":
            findings.append(Finding("error", "cutline.level", "cutline_level must be stable", packet_id))
        if list(cutline.get("above_cutline_levels", [])) != list(ABOVE_CUTLINE):
            findings.append(Finding("error", "cutline.above", "above_cutline_levels is not canonical", packet_id))
        if list(cutline.get("below_cutline_levels", [])) != list(BELOW_CUTLINE):
            findings.append(Finding("error", "cutline.below", "below_cutline_levels is not canonical", packet_id))
        if not is_str(cutline.get("description")):
            findings.append(Finding("error", "cutline.description", "cutline description must be non-empty", packet_id))
    return findings


def validate_rules(packet: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    rules = packet.get("rules")
    if not isinstance(rules, list) or not rules:
        findings.append(Finding("error", "rules.empty", "packet must enumerate at least one rule", "<packet>"))
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
        if rule.get("default_action") not in SCOREBOARD_ACTIONS:
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
            findings.append(Finding("error", "rule.reason_uncovered", "gap reason has no rule watching for it", reason))
    return findings


def validate_packet_block(scoreboard_id: str, packet: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    for field in ("packet_id", "packet_ref", "proof_index_ref"):
        if not is_str(packet.get(field)):
            findings.append(Finding("error", "packet.empty_field", f"scoreboard_packet.{field} must be non-empty", scoreboard_id))
    if packet.get("slo_state") not in FRESHNESS_SLO_STATES:
        findings.append(Finding("error", "packet.state_invalid", "slo_state is outside the vocabulary", scoreboard_id))
    slo = packet.get("freshness_slo")
    if not isinstance(slo, dict):
        findings.append(Finding("error", "packet.slo_missing", "scoreboard_packet must carry a freshness_slo block", scoreboard_id))
        return findings
    if not is_str(slo.get("slo_register_ref")):
        findings.append(Finding("error", "packet.slo_register_missing", "freshness_slo.slo_register_ref must be non-empty", scoreboard_id))
    target = slo.get("target_max_age_days")
    warn = slo.get("warn_within_days")
    if not (is_int(target) and target >= 1):
        findings.append(Finding("error", "packet.target_invalid", "target_max_age_days must be an integer >= 1", scoreboard_id))
    if not (is_int(warn) and warn >= 0):
        findings.append(Finding("error", "packet.warn_invalid", "warn_within_days must be an integer >= 0", scoreboard_id))
    if is_int(target) and is_int(warn) and warn > target:
        findings.append(Finding("error", "packet.slo_inconsistent", "warn_within_days may not exceed target_max_age_days", scoreboard_id))
    return findings


def validate_metrics(scoreboard_id: str, row: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    metrics = metrics_of(row)
    if not metrics:
        findings.append(Finding("error", "metric.empty", "a row must carry at least one metric", scoreboard_id))
    seen: set[str] = set()
    for metric in metrics:
        metric_id = str(metric.get("metric_id", "<metric>"))
        if metric_id in seen:
            findings.append(Finding("error", "metric.duplicate_id", "metric ids must be unique within a row", scoreboard_id))
        seen.add(metric_id)
        for field in ("metric_id", "title", "unit", "measurement_ref"):
            if not is_str(metric.get(field)):
                findings.append(Finding("error", "metric.empty_field", f"metric {field} must be non-empty", scoreboard_id))
        if not is_int(metric.get("threshold")):
            findings.append(Finding("error", "metric.threshold_invalid", "metric threshold must be an integer", scoreboard_id))
        measured = metric.get("measured")
        if measured is not None and not is_int(measured):
            findings.append(Finding("error", "metric.measured_invalid", "metric measured must be an integer or null", scoreboard_id))
    if failing_metric_count(row) > 0 and "score_below_threshold" not in reasons_of(row):
        findings.append(Finding("error", "metric.failing_without_reason", "a failing metric must name the score_below_threshold reason", scoreboard_id))
    return findings


def validate_signoffs(scoreboard_id: str, row: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    loop = row.get("signoff_loop")
    if not isinstance(loop, dict):
        findings.append(Finding("error", "signoff.loop_missing", "a row must carry a signoff_loop", scoreboard_id))
        return findings
    for field in ("loop_id", "cadence", "packet_ref"):
        if not is_str(loop.get(field)):
            findings.append(Finding("error", "signoff.empty_field", f"signoff_loop.{field} must be non-empty", scoreboard_id))
    signoffs = required_signoffs_of(row)
    if not signoffs:
        findings.append(Finding("error", "signoff.required_empty", "signoff_loop must carry at least one required signoff", scoreboard_id))
    seen: set[str] = set()
    for signoff in signoffs:
        role = signoff.get("role_ref")
        if not is_str(role):
            findings.append(Finding("error", "signoff.role_empty", "required_signoff.role_ref must be non-empty", scoreboard_id))
        elif role in seen:
            findings.append(Finding("error", "signoff.duplicate_role", f"required signoff role appears more than once: {role}", scoreboard_id))
        seen.add(str(role))
    if not signoff_loop_complete(row) and "required_signoff_missing" not in reasons_of(row):
        findings.append(Finding("error", "signoff.incomplete_without_reason", "an incomplete signoff loop must name required_signoff_missing", scoreboard_id))
    return findings


def validate_row(row: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings: list[Finding] = []
    scoreboard_id = str(row.get("scoreboard_id", "<row>"))

    for field in ("scoreboard_id", "title", "subject_family", "claim_ref", "rationale"):
        if not is_str(row.get(field)):
            findings.append(Finding("error", "row.empty_field", f"row {field} must be non-empty", scoreboard_id))

    if row.get("lane") not in SCOREBOARD_LANES:
        findings.append(Finding("error", "row.lane_invalid", "lane is outside the vocabulary", scoreboard_id))
    if not isinstance(row.get("release_blocking"), bool):
        findings.append(Finding("error", "row.release_blocking_invalid", "release_blocking must be a boolean", scoreboard_id))

    claim_label = row.get("claim_label")
    effective = row.get("effective_label")
    state = row.get("scoreboard_state")
    reasons = reasons_of(row)

    if claim_label not in LIFECYCLE_LABELS:
        findings.append(Finding("error", "row.claim_label_invalid", "claim_label is invalid", scoreboard_id))
    if effective not in LIFECYCLE_LABELS:
        findings.append(Finding("error", "row.effective_invalid", "effective_label is invalid", scoreboard_id))
    if state not in SCOREBOARD_STATES:
        findings.append(Finding("error", "row.state_invalid", "scoreboard_state is invalid", scoreboard_id))
    if any(reason not in GAP_REASONS for reason in reasons):
        findings.append(Finding("error", "row.reason_invalid", "active_gap_reasons has an unknown reason", scoreboard_id))

    # The ceiling: no row may render a label wider than the public claim's label.
    if claim_label in LEVEL_RANK and effective in LEVEL_RANK and LEVEL_RANK[effective] > LEVEL_RANK[claim_label]:
        findings.append(Finding("error", "row.effective_wider_than_claim", "effective_label is wider than the claim ceiling", scoreboard_id))

    packet = packet_of(row)
    findings.extend(validate_packet_block(scoreboard_id, packet))
    findings.extend(validate_metrics(scoreboard_id, row))
    findings.extend(validate_signoffs(scoreboard_id, row))
    slo_state = packet.get("slo_state")

    holding = state in HOLDING_STATES
    claim_holds = is_above_cutline(claim_label)

    # A claim canonically below the cutline forces the row to inherit the ceiling and narrow.
    if claim_label in LEVEL_RANK and not claim_holds:
        if holding:
            findings.append(Finding("error", "row.held_on_narrowed_claim", "a row signs off while the public claim label is below the cutline", scoreboard_id))
        if "claim_label_narrowed" not in reasons:
            findings.append(Finding("error", "row.claim_narrowed_without_reason", "a row whose claim is narrowed must name claim_label_narrowed", scoreboard_id))

    if holding:
        if claim_label != effective:
            findings.append(Finding("error", "row.held_label_not_equal_claim", "a signed-off row must render the public claim's canonical label", scoreboard_id))
        if reasons:
            findings.append(Finding("error", "row.held_with_active_gap", "a signed-off row must carry no active gap reason", scoreboard_id))
        if not has_capture(packet):
            findings.append(Finding("error", "row.held_without_fresh_packet", "a signed-off row must ride a captured, evidence-backed packet", scoreboard_id))
        if slo_state in ("breached", "missing"):
            findings.append(Finding("error", "row.held_on_stale_packet", "a signed-off row must ride a packet within its freshness SLO", scoreboard_id))
        if not all_metrics_pass(row):
            findings.append(Finding("error", "row.held_with_failing_metric", "a signed-off row must clear every metric threshold", scoreboard_id))
        if not owner_signed(row):
            findings.append(Finding("error", "row.held_without_owner_signoff", "a signed-off row must carry an owner sign-off with a date", scoreboard_id))
        if not signoff_loop_complete(row):
            findings.append(Finding("error", "row.held_without_required_signoffs", "a signed-off row must complete every required signoff", scoreboard_id))
    else:
        if is_above_cutline(effective):
            findings.append(Finding("error", "row.effective_not_narrowed", "a row that is not signed off must narrow below the cutline", scoreboard_id))
        if not reasons:
            findings.append(Finding("error", "row.narrowing_without_reason", "a narrowing row must name an active gap reason", scoreboard_id))
        if slo_state == "breached" and "scoreboard_packet_freshness_breached" not in reasons:
            findings.append(Finding("error", "row.breached_without_reason", "a breached packet must name scoreboard_packet_freshness_breached", scoreboard_id))
        if slo_state == "missing" and "scoreboard_packet_missing" not in reasons:
            findings.append(Finding("error", "row.missing_without_reason", "a missing packet must name scoreboard_packet_missing", scoreboard_id))

    signoff = row.get("owner_signoff", {})
    if not (isinstance(signoff, dict) and is_str(signoff.get("owner_ref"))):
        findings.append(Finding("error", "row.signoff_owner_missing", "owner_signoff.owner_ref must be non-empty", scoreboard_id))

    findings.extend(validate_state_reason_coherence(scoreboard_id, state, reasons, row))
    findings.extend(validate_row_refs(scoreboard_id, row, repo_root))
    return findings


def validate_state_reason_coherence(scoreboard_id: str, state: Any, reasons: list[str], row: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    if state == "narrowed_unbacked":
        allowed = {"scoreboard_evidence_incomplete", "owner_signoff_missing", "required_signoff_missing", "score_below_threshold"}
        if not allowed.intersection(reasons):
            findings.append(Finding("error", "row.state_reason_incoherent", "narrowed_unbacked requires an evidence/signoff/metric reason", scoreboard_id))
    if state == "narrowed_claim_narrowed" and "claim_label_narrowed" not in reasons:
        findings.append(Finding("error", "row.state_reason_incoherent", "narrowed_claim_narrowed requires the claim_label_narrowed reason", scoreboard_id))
    if state == "narrowed_stale" and not ({"scoreboard_packet_freshness_breached", "scoreboard_packet_missing"} & set(reasons)):
        findings.append(Finding("error", "row.state_reason_incoherent", "narrowed_stale requires a packet freshness/missing reason", scoreboard_id))
    if state == "narrowed_waiver_expired":
        if "waiver_expired" not in reasons:
            findings.append(Finding("error", "row.state_reason_incoherent", "narrowed_waiver_expired requires the waiver_expired reason", scoreboard_id))
        if not isinstance(row.get("waiver"), dict):
            findings.append(Finding("error", "row.waiver_state_without_waiver", "narrowed_waiver_expired state must name a waiver", scoreboard_id))
    if state == "signed_off_on_waiver":
        waiver = row.get("waiver")
        if not isinstance(waiver, dict) or not is_str(waiver.get("waiver_ref")) or not is_str(waiver.get("expires_at")):
            findings.append(Finding("error", "row.waiver_state_without_waiver", "signed_off_on_waiver state must name a waiver", scoreboard_id))
    return findings


def validate_row_refs(scoreboard_id: str, row: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings: list[Finding] = []
    refs: list[str] = []
    packet = packet_of(row)
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
    loop = row.get("signoff_loop")
    if isinstance(loop, dict) and is_str(loop.get("packet_ref")):
        refs.append(loop["packet_ref"].split("#", 1)[0])
    for ref in refs:
        if not (repo_root / ref).exists():
            findings.append(Finding("error", "row.ref_missing", f"referenced artifact does not exist: {ref}", scoreboard_id))
    return findings


def validate_coverage(packet: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    rows = packet.get("rows", [])

    seen: set[str] = set()
    for row in rows:
        rid = row.get("scoreboard_id")
        if not is_str(rid):
            continue
        if rid in seen:
            findings.append(Finding("error", "row.duplicate_id", f"scoreboard id appears more than once: {rid}", rid))
        seen.add(rid)

    declared = packet.get("release_blocking_scoreboard_refs", [])
    declared_set = {r for r in declared if is_str(r)} if isinstance(declared, list) else set()
    covered = {
        str(row.get("scoreboard_id"))
        for row in rows
        if row.get("release_blocking") is True and is_str(row.get("scoreboard_id"))
    }
    for ref in sorted(declared_set):
        if ref not in covered:
            findings.append(Finding("error", "coverage.release_blocking_ref_without_row", f"declared release-blocking scoreboard has no row: {ref}", ref))
    for row in rows:
        if row.get("release_blocking") is True and is_str(row.get("scoreboard_id")) and row["scoreboard_id"] not in declared_set:
            findings.append(Finding("error", "coverage.release_blocking_not_declared", "a release-blocking row is not in release_blocking_scoreboard_refs", str(row.get("scoreboard_id"))))

    present_lanes = {row.get("lane") for row in rows}
    for lane in SCOREBOARD_LANES:
        if lane not in present_lanes:
            findings.append(Finding("error", "coverage.lane_absent", f"scoreboard lane has no row: {lane}", lane))
    return findings


def validate_freshness(packet: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    as_of = parse_date(packet.get("as_of"))
    if as_of is None:
        return findings
    for row in packet.get("rows", []):
        scoreboard_id = str(row.get("scoreboard_id", "<row>"))
        pkt = packet_of(row)
        declared = pkt.get("slo_state")
        computed = computed_slo_state(pkt, as_of)
        if computed is None or declared not in FRESHNESS_RANK or computed not in FRESHNESS_RANK:
            continue
        if FRESHNESS_RANK[declared] > FRESHNESS_RANK[computed]:
            findings.append(Finding("error", "row.packet_freshness_overstated", f"declared slo_state {declared!r} is fresher than the {computed!r} the as_of date allows", scoreboard_id))
        if row.get("scoreboard_state") in HOLDING_STATES and computed in ("breached", "missing"):
            findings.append(Finding("error", "row.held_on_breached_packet", "a signed-off row rides a packet past its SLO against as_of", scoreboard_id))
    return findings


def validate_waivers(packet: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    as_of = parse_date(packet.get("as_of"))
    if as_of is None:
        return findings
    for row in packet.get("rows", []):
        scoreboard_id = str(row.get("scoreboard_id", "<row>"))
        state = row.get("scoreboard_state")
        waiver = row.get("waiver")
        if not isinstance(waiver, dict):
            continue
        expires = parse_date(waiver.get("expires_at"))
        if expires is None:
            findings.append(Finding("error", "row.waiver_expiry_invalid", "waiver expires_at must be an ISO date", scoreboard_id))
            continue
        expired = as_of >= expires
        if expired and state == "signed_off_on_waiver":
            findings.append(Finding("error", "row.held_on_expired_waiver", "a row signs off on a waiver that has expired against as_of", scoreboard_id))
        if not expired and state == "narrowed_waiver_expired":
            findings.append(Finding("error", "row.waiver_expired_but_active", "a row is marked waiver-expired but the waiver is still active against as_of", scoreboard_id))
    return findings


def validate_ceiling(packet: dict[str, Any], repo_root: Path) -> list[Finding]:
    """Cross-artifact ingestion: read the stable claim manifest and fail when a row's
    claim_label is not the label the claim manifest publishes for the entry named by
    claim_ref, or names an entry the claim manifest does not carry."""
    findings: list[Finding] = []
    ref = packet.get("claim_manifest_ref")
    if not is_str(ref):
        return findings
    path = repo_root / ref.split("#", 1)[0]
    if not path.exists():
        findings.append(Finding("error", "ceiling.artifact_missing", f"claim_manifest_ref does not exist: {ref}", "<packet>"))
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

    for row in packet.get("rows", []):
        scoreboard_id = str(row.get("scoreboard_id", "<row>"))
        claim_ref = row.get("claim_ref")
        if not is_str(claim_ref):
            continue
        if claim_ref not in published_by_entry:
            findings.append(Finding("error", "ceiling.claim_entry_unknown", f"claim_ref is not an entry in the stable claim manifest: {claim_ref}", scoreboard_id))
            continue
        canonical = published_by_entry[claim_ref]
        if row.get("claim_label") != canonical:
            findings.append(Finding("error", "ceiling.claim_label_mismatch", f"claim_label {row.get('claim_label')!r} does not match the claim manifest published_label {canonical!r}", scoreboard_id))
    return findings


def validate_publication(packet: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    publication = packet.get("publication")
    if not isinstance(publication, dict):
        findings.append(Finding("error", "publication.missing", "packet must carry a publication block", "<packet>"))
        return findings
    for field in ("publication_gate", "rationale"):
        if not is_str(publication.get(field)):
            findings.append(Finding("error", "publication.empty_field", f"publication.{field} must be non-empty", "<packet>"))
    decision, rule_ids, scoreboard_ids = computed_publication(packet)
    if publication.get("decision") != decision:
        findings.append(Finding("error", "publication.decision_inconsistent", f"declared decision {publication.get('decision')!r} disagrees with computed {decision!r}", "<packet>"))
    if list(publication.get("blocking_rule_ids", [])) != rule_ids:
        findings.append(Finding("error", "publication.blocking_rules_mismatch", "blocking_rule_ids disagrees with the firing rules", "<packet>"))
    if list(publication.get("blocking_scoreboard_ids", [])) != scoreboard_ids:
        findings.append(Finding("error", "publication.blocking_scoreboards_mismatch", "blocking_scoreboard_ids disagrees with the firing rules", "<packet>"))
    return findings


def validate_summary(packet: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    summary = packet.get("summary")
    if not isinstance(summary, dict):
        findings.append(Finding("error", "summary.missing", "packet must carry a summary block", "<packet>"))
        return findings
    expected = computed_summary(packet)
    for key, value in expected.items():
        if summary.get(key) != value:
            findings.append(Finding("error", "summary.count_mismatch", f"summary.{key} must equal the recomputed value", key))
    return findings


def validate_packet(packet: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings = validate_envelope(packet, repo_root)
    findings.extend(validate_rules(packet))

    rows = packet.get("rows")
    if not isinstance(rows, list) or not rows:
        findings.append(Finding("error", "scoreboards.rows_empty", "packet must enumerate at least one row", "<packet>"))
        return findings

    for raw in rows:
        row = ensure_dict(raw, "packet.rows[]")
        findings.extend(validate_row(row, repo_root))

    findings.extend(validate_coverage(packet))
    findings.extend(validate_freshness(packet))
    findings.extend(validate_waivers(packet))
    findings.extend(validate_ceiling(packet, repo_root))
    findings.extend(validate_publication(packet))
    findings.extend(validate_summary(packet))
    return findings


def recompute_derived(packet: dict[str, Any]) -> None:
    packet["summary"] = computed_summary(packet)
    decision, rule_ids, scoreboard_ids = computed_publication(packet)
    if isinstance(packet.get("publication"), dict):
        packet["publication"].update(
            {"decision": decision, "blocking_rule_ids": rule_ids, "blocking_scoreboard_ids": scoreboard_ids}
        )


def run_negative_drills(packet: dict[str, Any], repo_root: Path) -> tuple[list[dict[str, str]], list[Finding]]:
    results: list[dict[str, str]] = []
    findings: list[Finding] = []

    def record(drill_id: str, expected_check_id: str, fired: bool) -> None:
        results.append({"drill_id": drill_id, "expected_check_id": expected_check_id, "status": "passed" if fired else "failed"})
        if not fired:
            findings.append(Finding("error", "negative_drill.not_rejected", f"negative drill {drill_id} did not fire", drill_id))

    def check_ids(candidate: dict[str, Any]) -> set[str]:
        return {f.check_id for f in validate_packet(candidate, repo_root)}

    # A signed-off row rendered with a breached packet against as_of must be flagged.
    mutated = copy.deepcopy(packet)
    target = next((r for r in mutated["rows"] if r.get("scoreboard_state") in HOLDING_STATES), None)
    if target is not None:
        target["scoreboard_packet"]["captured_at"] = "2000-01-01"
        record("held_on_breached_packet_rejected", "row.held_on_breached_packet", "row.held_on_breached_packet" in check_ids(mutated))

    # A narrowing row that still renders a stable label must be flagged.
    mutated = copy.deepcopy(packet)
    target = next((r for r in mutated["rows"] if r.get("scoreboard_state") in NARROWING_STATES), None)
    if target is not None:
        target["effective_label"] = "stable"
        recompute_derived(mutated)
        record("effective_not_narrowed_rejected", "row.effective_not_narrowed", "row.effective_not_narrowed" in check_ids(mutated))

    # A signed-off row with a failing metric must be flagged.
    mutated = copy.deepcopy(packet)
    target = next((r for r in mutated["rows"] if r.get("scoreboard_state") in HOLDING_STATES), None)
    if target is not None and target.get("metrics"):
        target["metrics"][0]["measured"] = -1
        record("held_with_failing_metric_rejected", "row.held_with_failing_metric", "row.held_with_failing_metric" in check_ids(mutated))

    # A signed-off row with an incomplete required signoff must be flagged.
    mutated = copy.deepcopy(packet)
    target = next((r for r in mutated["rows"] if r.get("scoreboard_state") in HOLDING_STATES), None)
    if target is not None and required_signoffs_of(target):
        target["signoff_loop"]["required_signoffs"][0]["signed_off"] = False
        target["signoff_loop"]["required_signoffs"][0]["signed_at"] = None
        target["signoff_loop"]["required_signoffs"][0]["signer_ref"] = None
        record("held_without_required_signoffs_rejected", "row.held_without_required_signoffs", "row.held_without_required_signoffs" in check_ids(mutated))

    # A row whose claim_label disagrees with the claim manifest must be flagged.
    mutated = copy.deepcopy(packet)
    target = next((r for r in mutated["rows"] if r.get("claim_label") == "beta"), None)
    if target is not None:
        target["claim_label"] = "stable"
        record("claim_label_mismatch_rejected", "ceiling.claim_label_mismatch", "ceiling.claim_label_mismatch" in check_ids(mutated))

    # Removing every row of a lane must be flagged.
    mutated = copy.deepcopy(packet)
    mutated["rows"] = [r for r in mutated["rows"] if r.get("lane") != "design_partner"]
    recompute_derived(mutated)
    record("lane_absent_rejected", "coverage.lane_absent", "coverage.lane_absent" in check_ids(mutated))

    # Dropping a declared release-blocking row must be flagged.
    mutated = copy.deepcopy(packet)
    declared = mutated.get("release_blocking_scoreboard_refs", [])
    if declared:
        dropped = declared[0]
        mutated["rows"] = [r for r in mutated["rows"] if r.get("scoreboard_id") != dropped]
        recompute_derived(mutated)
        record("release_blocking_uncovered_rejected", "coverage.release_blocking_ref_without_row", "coverage.release_blocking_ref_without_row" in check_ids(mutated))

    # A publication decision that disagrees with the firing rules must be flagged.
    mutated = copy.deepcopy(packet)
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
        ids = {f.check_id for f in validate_packet(ensure_dict(candidate, "fixture"), repo_root)}
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
    packet: dict[str, Any],
    findings: list[Finding],
    drill_results: list[dict[str, str]],
    fixture_results: list[dict[str, str]],
) -> None:
    decision, rule_ids, scoreboard_ids = computed_publication(packet)
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "record_kind": "cohort_scoreboards_validation_capture",
        "status": "pass" if not any(f.severity == "error" for f in findings) else "fail",
        "generated_at": generated_at_now(),
        "packet_id": packet.get("packet_id"),
        "as_of": packet.get("as_of"),
        "summary": packet.get("summary"),
        "publication": {
            "decision": decision,
            "blocking_rule_ids": rule_ids,
            "blocking_scoreboard_ids": scoreboard_ids,
        },
        "negative_drills": drill_results,
        "fixture_cases": fixture_results,
        "findings": [f.as_report() for f in findings],
    }
    path.write_text(json_text(payload), encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    packet = ensure_dict(
        load_json(repo_root / args.packet, "cohort scoreboards packet"),
        "cohort scoreboards packet",
    )

    findings = validate_packet(packet, repo_root)
    drill_results, drill_findings = run_negative_drills(packet, repo_root)
    findings.extend(drill_findings)
    fixture_results, fixture_findings = run_fixture_cases(repo_root, args.fixtures_dir)
    findings.extend(fixture_findings)

    report_rel = args.report
    if report_rel is None and not args.no_capture:
        report_rel = DEFAULT_REPORT_REL
    if report_rel:
        write_report(repo_root / report_rel, packet, findings, drill_results, fixture_results)

    errors = [f for f in findings if f.severity == "error"]
    if errors:
        for item in errors:
            print(f"ERROR [{item.check_id}] ({item.ref}): {item.message}", file=sys.stderr)
        return 1

    summary = packet.get("summary", {})
    decision, rule_ids, _scoreboard_ids = computed_publication(packet)
    print(
        "cohort scoreboards packet validated "
        f"({summary.get('total_rows')} rows across {summary.get('total_claims')} claims, "
        f"{summary.get('rows_holding_stable')} signed off, "
        f"{summary.get('rows_narrowed_below_cutline')} narrowed; "
        f"release-blocking {summary.get('release_blocking_holding_stable')}/{summary.get('release_blocking_total')} holding; "
        f"packets {summary.get('packets_current')} current / "
        f"{summary.get('packets_due_for_refresh')} due / "
        f"{summary.get('packets_breached')} breached / "
        f"{summary.get('packets_missing')} missing; "
        f"metrics {summary.get('metrics_passing')}/{summary.get('total_metrics')} passing; "
        f"signoff loops {summary.get('complete_signoff_loops')} complete; "
        f"{summary.get('rules_firing')} rules firing; "
        f"publication={decision}; {len(drill_results)} negative drills, {len(fixture_results)} fixture cases)"
    )

    if args.require_proceed and decision == "hold":
        print(
            f"PUBLICATION HELD: cohort scoreboards promotion blocked by {len(rule_ids)} rule(s): {', '.join(rule_ids)}",
            file=sys.stderr,
        )
        return 2

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
