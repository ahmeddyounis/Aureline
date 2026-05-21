#!/usr/bin/env python3
"""Validate the maintenance-control packet for the release line's hotfix, backport,
correction-train, and support-window lanes.

The stable claim manifest decides the single canonical lifecycle label each *subject*
publishes; the stable proof index decides whether each launch-blocking *requirement* is
proven; the stable version-window freeze freezes each public interface surface's version
window. This packet answers the maintenance-level question: for each post-release
maintenance lane — an emergency hotfix lane, a supported-line backport lane, a planned
correction-train lane, or a support-window commitment — is that lane actually governed,
backed by a fresh control packet, a complete and unexpired support window, and an owner
sign-off? It reads the checked-in packet at
``artifacts/release/maintenance_control_packet.json`` and:

  - asserts the closed vocabularies (lifecycle labels, lane kinds, support postures,
    control states, gap reasons, control actions) and the launch cutline are canonical;
  - asserts every row that is narrowed (for an absent/incomplete lane, an incomplete
    support window, an expired support window, a breached/missing control packet, an
    expired waiver, a missing owner sign-off, or a public claim whose canonical label is
    itself below the cutline) drops below the cutline rather than governing a label wider
    than the public claim, and that a governed row backs the public claim's canonical
    label cleanly with a captured, within-SLO control packet, a complete support window,
    and an owner sign-off;
  - asserts each support window is ordered opened <= review_through <= end_of_support;
  - asserts hotfix/backport/correction-train/support-window coverage: every lane kind is
    represented, every declared release-blocking lane ref has exactly one covering
    release-blocking row, every release-blocking row is declared, and no lane ref repeats;
  - performs the ceiling cross-check the typed model cannot — it reads the stable claim
    manifest named by ``claim_manifest_ref`` and fails when a row's ``claim_label`` is not
    the label the claim manifest publishes for the entry named by ``claim_ref``, or names
    an entry the claim manifest does not carry;
  - performs the packet-freshness SLO automation against the packet ``as_of`` date, the
    waiver-expiry date arithmetic, and the support-window-expiry date arithmetic the typed
    model cannot;
  - recomputes the publication verdict and the summary block, and fails on any drift;
  - runs negative drills proving the narrowing, ceiling, freshness, support-window,
    waiver, and publication rejections all fire; and
  - runs the checked-in fixture cases under
    ``fixtures/release/maintenance_control_packet`` and fails when a case the packet
    marks as rejected validates clean.

With ``--require-proceed`` the gate additionally fails (exit code 2) when the recomputed
publication verdict is ``hold``, so shiproom and release tooling can block maintenance
publication directly from this artifact.

The typed Rust consumer
(``aureline_release::maintenance_control_packet::current_maintenance_control_packet``)
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


DEFAULT_PACKET_REL = "artifacts/release/maintenance_control_packet.json"
DEFAULT_REPORT_REL = (
    "artifacts/release/captures/maintenance_control_packet_validation_capture.json"
)
DEFAULT_FIXTURES_REL = "fixtures/release/maintenance_control_packet"

EXPECTED_SCHEMA_VERSION = 1
PACKET_RECORD_KIND = "maintenance_control_packet"

LIFECYCLE_LABELS = ("lts", "stable", "beta", "preview", "withdrawn")
ABOVE_CUTLINE = ("lts", "stable")
BELOW_CUTLINE = ("beta", "preview", "withdrawn")
LANE_KINDS = ("hotfix", "backport", "correction_train", "support_window")
SUPPORT_POSTURES = (
    "full_support",
    "security_and_critical_only",
    "security_only",
    "end_of_life_scheduled",
)
FRESHNESS_SLO_STATES = ("current", "due_for_refresh", "breached", "missing")
CONTROL_STATES = (
    "governed",
    "governed_on_waiver",
    "ungoverned_unbacked",
    "ungoverned_claim_narrowed",
    "ungoverned_stale",
    "ungoverned_waiver_expired",
    "ungoverned_support_expired",
)
GAP_REASONS = (
    "claim_label_narrowed",
    "lane_capability_absent",
    "control_evidence_incomplete",
    "support_window_incomplete",
    "support_window_expired",
    "control_packet_freshness_breached",
    "control_packet_missing",
    "waiver_expired",
    "owner_signoff_missing",
)
CONTROL_ACTIONS = (
    "hold_publication",
    "narrow_control_label",
    "refresh_control_packet",
    "complete_support_window",
    "recapture_control_evidence",
    "request_owner_signoff",
)
HOLDING_STATES = ("governed", "governed_on_waiver")
NARROWING_STATES = (
    "ungoverned_unbacked",
    "ungoverned_claim_narrowed",
    "ungoverned_stale",
    "ungoverned_waiver_expired",
    "ungoverned_support_expired",
)
ACTIVE_SUPPORT_POSTURES = (
    "full_support",
    "security_and_critical_only",
    "security_only",
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


def support_window_of(row: dict[str, Any]) -> dict[str, Any]:
    window = row.get("support_window")
    return window if isinstance(window, dict) else {}


def support_window_is_complete(row: dict[str, Any]) -> bool:
    window = support_window_of(row)
    return all(
        is_str(window.get(field))
        for field in ("opened_at", "review_through_date", "end_of_support_date")
    )


def control_rule_fires(rule: dict[str, Any], rows: list[dict[str, Any]]) -> bool:
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
    rules = packet.get("control_rules", [])
    blocking_rules = [
        rule
        for rule in rules
        if rule.get("blocks_publication") is True and control_rule_fires(rule, rows)
    ]
    rule_ids = sorted(str(rule.get("rule_id")) for rule in blocking_rules)
    blocking_triggers = {rule.get("trigger_reason") for rule in blocking_rules}
    lane_ids: set[str] = set()
    for row in rows:
        if is_above_cutline(row.get("claim_label")) and blocking_triggers.intersection(
            reasons_of(row)
        ):
            lane_ids.add(str(row.get("control_id")))
    decision = "hold" if blocking_rules else "proceed"
    return decision, rule_ids, sorted(lane_ids)


def computed_summary(packet: dict[str, Any]) -> dict[str, Any]:
    rows = packet.get("rows", [])
    rules = packet.get("control_rules", [])

    def packet_state(row: dict[str, Any]) -> Any:
        control = row.get("control_packet")
        return control.get("slo_state") if isinstance(control, dict) else None

    def kind_count(kind: str) -> int:
        return sum(1 for r in rows if r.get("lane_kind") == kind)

    governed = [r for r in rows if is_above_cutline(r.get("controlled_label"))]
    claims = {str(r.get("claim_ref")) for r in rows}
    release_blocking = [r for r in rows if r.get("release_blocking") is True]
    rb_governed = [r for r in release_blocking if is_above_cutline(r.get("controlled_label"))]
    return {
        "total_lanes": len(rows),
        "total_claims": len(claims),
        "lanes_governed_stable": len(governed),
        "lanes_narrowed_below_cutline": len(rows) - len(governed),
        "lanes_on_active_waiver": sum(
            1 for r in rows if r.get("control_state") == "governed_on_waiver"
        ),
        "release_blocking_total": len(release_blocking),
        "release_blocking_governed_stable": len(rb_governed),
        "release_blocking_ungoverned": len(release_blocking) - len(rb_governed),
        "hotfix_lanes": kind_count("hotfix"),
        "backport_lanes": kind_count("backport"),
        "correction_train_lanes": kind_count("correction_train"),
        "support_window_lanes": kind_count("support_window"),
        "packets_current": sum(1 for r in rows if packet_state(r) == "current"),
        "packets_due_for_refresh": sum(1 for r in rows if packet_state(r) == "due_for_refresh"),
        "packets_breached": sum(1 for r in rows if packet_state(r) == "breached"),
        "packets_missing": sum(1 for r in rows if packet_state(r) == "missing"),
        "lanes_support_expired": sum(
            1 for r in rows if r.get("control_state") == "ungoverned_support_expired"
        ),
        "total_active_gap_reasons": sum(len(reasons_of(r)) for r in rows),
        "control_rules_firing": sum(1 for rule in rules if control_rule_fires(rule, rows)),
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
    if not (isinstance(target, int) and not isinstance(target, bool) and target >= 1):
        return None
    if not (isinstance(warn, int) and not isinstance(warn, bool) and warn >= 0):
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
        findings.append(Finding("error", "packet.schema_version", "schema_version must be 1", packet_id))
    if packet.get("record_kind") != PACKET_RECORD_KIND:
        findings.append(Finding("error", "packet.record_kind", "record_kind is not supported", packet_id))
    for field in (
        "packet_id",
        "status",
        "overview_page",
        "as_of",
        "claim_manifest_ref",
        "correction_train_template_ref",
        "support_window_contract_ref",
    ):
        if not is_str(packet.get(field)):
            findings.append(
                Finding("error", "packet.empty_field", f"{field} must be a non-empty string", packet_id)
            )
    if parse_date(packet.get("as_of")) is None:
        findings.append(Finding("error", "packet.as_of_invalid", "as_of must be an ISO date", packet_id))

    for field in ("correction_train_template_ref", "support_window_contract_ref"):
        ref = packet.get(field)
        if is_str(ref) and not (repo_root / ref.split("#", 1)[0]).exists():
            findings.append(
                Finding("error", "packet.ref_missing", f"{field} does not exist: {ref}", packet_id)
            )

    for key, expected in (
        ("lifecycle_labels", list(LIFECYCLE_LABELS)),
        ("lane_kinds", list(LANE_KINDS)),
        ("support_postures", list(SUPPORT_POSTURES)),
        ("control_states", list(CONTROL_STATES)),
        ("gap_reasons", list(GAP_REASONS)),
        ("control_actions", list(CONTROL_ACTIONS)),
    ):
        if list(packet.get(key, [])) != expected:
            findings.append(
                Finding("error", "packet.vocabulary", f"packet.{key} is not the closed vocabulary", key)
            )

    refs = packet.get("release_blocking_lane_refs")
    if not isinstance(refs, list) or not refs or any(not is_str(r) for r in refs):
        findings.append(
            Finding(
                "error",
                "packet.release_blocking_refs",
                "release_blocking_lane_refs must be a non-empty list of non-empty strings",
                packet_id,
            )
        )

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
    rules = packet.get("control_rules")
    if not isinstance(rules, list) or not rules:
        findings.append(Finding("error", "rules.empty", "packet must enumerate at least one control rule", "<packet>"))
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
        if rule.get("default_action") not in CONTROL_ACTIONS:
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


def validate_packet_block(control_id: str, control: Any) -> list[Finding]:
    findings: list[Finding] = []
    if not isinstance(control, dict):
        findings.append(Finding("error", "row.packet_missing", "row must carry a control_packet block", control_id))
        return findings
    for field in ("packet_id", "packet_ref", "proof_index_ref"):
        if not is_str(control.get(field)):
            findings.append(Finding("error", "packet.empty_field", f"control_packet.{field} must be non-empty", control_id))
    if control.get("slo_state") not in FRESHNESS_SLO_STATES:
        findings.append(Finding("error", "packet.state_invalid", "slo_state is outside the vocabulary", control_id))
    slo = control.get("freshness_slo")
    if not isinstance(slo, dict):
        findings.append(Finding("error", "packet.slo_missing", "control_packet must carry a freshness_slo block", control_id))
        return findings
    if not is_str(slo.get("slo_register_ref")):
        findings.append(Finding("error", "packet.slo_register_missing", "freshness_slo.slo_register_ref must be non-empty", control_id))
    target = slo.get("target_max_age_days")
    warn = slo.get("warn_within_days")
    if not (isinstance(target, int) and not isinstance(target, bool) and target >= 1):
        findings.append(Finding("error", "packet.target_invalid", "target_max_age_days must be an integer >= 1", control_id))
    if not (isinstance(warn, int) and not isinstance(warn, bool) and warn >= 0):
        findings.append(Finding("error", "packet.warn_invalid", "warn_within_days must be an integer >= 0", control_id))
    if (
        isinstance(target, int)
        and not isinstance(target, bool)
        and isinstance(warn, int)
        and not isinstance(warn, bool)
        and warn > target
    ):
        findings.append(Finding("error", "packet.slo_inconsistent", "warn_within_days may not exceed target_max_age_days", control_id))
    return findings


def validate_support_window(control_id: str, row: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    window = row.get("support_window")
    if not isinstance(window, dict):
        findings.append(Finding("error", "row.support_window_missing", "row must carry a support_window block", control_id))
        return findings
    for field in ("opened_at", "end_of_support_date"):
        if not is_str(window.get(field)):
            findings.append(Finding("error", "window.empty_field", f"support_window.{field} must be non-empty", control_id))
    if window.get("support_posture") not in SUPPORT_POSTURES:
        findings.append(Finding("error", "window.posture_invalid", "support_posture is outside the vocabulary", control_id))
    opened = parse_date(window.get("opened_at"))
    review = parse_date(window.get("review_through_date"))
    end = parse_date(window.get("end_of_support_date"))
    if opened is not None and review is not None and end is not None:
        if not (opened <= review <= end):
            findings.append(Finding("error", "row.support_window_disordered", "support window is not ordered opened <= review_through <= end_of_support", control_id))

    reasons = reasons_of(row)
    incomplete = not support_window_is_complete(row)
    if "support_window_incomplete" in reasons and not incomplete:
        findings.append(Finding("error", "row.support_window_reason_without_incomplete", "names support_window_incomplete but the window is complete", control_id))
    if incomplete and "support_window_incomplete" not in reasons:
        findings.append(Finding("error", "row.incomplete_support_window_without_reason", "has an incomplete support window but does not name support_window_incomplete", control_id))
    return findings


def validate_row(row: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings: list[Finding] = []
    control_id = str(row.get("control_id", "<row>"))

    for field in (
        "control_id",
        "title",
        "lane_ref",
        "lane_summary",
        "claim_ref",
        "correction_packet_ref",
        "rationale",
    ):
        if not is_str(row.get(field)):
            findings.append(Finding("error", "row.empty_field", f"row {field} must be non-empty", control_id))

    if row.get("lane_kind") not in LANE_KINDS:
        findings.append(Finding("error", "row.lane_kind_invalid", "lane_kind is outside the vocabulary", control_id))
    if not isinstance(row.get("release_blocking"), bool):
        findings.append(Finding("error", "row.release_blocking_invalid", "release_blocking must be a boolean", control_id))

    claim_label = row.get("claim_label")
    controlled = row.get("controlled_label")
    state = row.get("control_state")
    reasons = reasons_of(row)

    if claim_label not in LIFECYCLE_LABELS:
        findings.append(Finding("error", "row.claim_label_invalid", "claim_label is invalid", control_id))
    if controlled not in LIFECYCLE_LABELS:
        findings.append(Finding("error", "row.controlled_invalid", "controlled_label is invalid", control_id))
    if state not in CONTROL_STATES:
        findings.append(Finding("error", "row.state_invalid", "control_state is invalid", control_id))
    if any(reason not in GAP_REASONS for reason in reasons):
        findings.append(Finding("error", "row.reason_invalid", "active_gap_reasons has an unknown reason", control_id))

    # The ceiling: no control may back a label wider than the public claim's label.
    if claim_label in LEVEL_RANK and controlled in LEVEL_RANK and LEVEL_RANK[controlled] > LEVEL_RANK[claim_label]:
        findings.append(Finding("error", "row.controlled_wider_than_claim", "controlled_label is wider than the claim ceiling", control_id))

    findings.extend(validate_support_window(control_id, row))

    control = row.get("control_packet")
    findings.extend(validate_packet_block(control_id, control))
    slo_state = control.get("slo_state") if isinstance(control, dict) else None

    holds = state in HOLDING_STATES
    claim_holds = is_above_cutline(claim_label)

    # A claim canonically below the cutline forces the control to inherit the ceiling
    # and narrow.
    if claim_label in LEVEL_RANK and not claim_holds:
        if holds:
            findings.append(Finding("error", "row.held_on_narrowed_claim", "a row holds control while the public claim label is below the cutline", control_id))
        if "claim_label_narrowed" not in reasons:
            findings.append(Finding("error", "row.claim_narrowed_without_reason", "a row whose claim is narrowed must name claim_label_narrowed", control_id))

    if state in NARROWING_STATES:
        if is_above_cutline(controlled):
            findings.append(Finding("error", "row.controlled_not_narrowed", "a lane that is not governed must narrow below the cutline", control_id))
        if not reasons:
            findings.append(Finding("error", "row.narrowing_without_reason", "a narrowing row must name an active gap reason", control_id))
        if slo_state == "breached" and "control_packet_freshness_breached" not in reasons:
            findings.append(Finding("error", "row.breached_without_reason", "a breached packet must name control_packet_freshness_breached", control_id))
        if slo_state == "missing" and "control_packet_missing" not in reasons:
            findings.append(Finding("error", "row.missing_without_reason", "a missing packet must name control_packet_missing", control_id))
    if holds:
        if claim_label != controlled:
            findings.append(Finding("error", "row.held_label_not_equal_claim", "a governed row must back the public claim's canonical label", control_id))
        if reasons:
            findings.append(Finding("error", "row.held_with_active_gap", "a governed row must carry no active gap reason", control_id))
        if isinstance(control, dict) and not has_capture(control):
            findings.append(Finding("error", "row.held_without_fresh_packet", "a governed row must ride a captured, evidence-backed control packet", control_id))
        if slo_state in ("breached", "missing"):
            findings.append(Finding("error", "row.held_on_stale_packet", "a governed row must ride a packet within its freshness SLO", control_id))
        if not support_window_is_complete(row):
            findings.append(Finding("error", "row.held_with_incomplete_support_window", "a governed row must carry a complete support window", control_id))
        signoff = row.get("owner_signoff", {})
        if not (isinstance(signoff, dict) and signoff.get("signed_off") is True and is_str(signoff.get("signed_at"))):
            findings.append(Finding("error", "row.held_without_signoff", "a governed row must carry an owner sign-off with a date", control_id))
    signoff = row.get("owner_signoff", {})
    if not (isinstance(signoff, dict) and is_str(signoff.get("owner_ref"))):
        findings.append(Finding("error", "row.signoff_owner_missing", "owner_signoff.owner_ref must be non-empty", control_id))

    findings.extend(validate_state_reason_coherence(control_id, state, reasons, row))
    findings.extend(validate_row_refs(control_id, row, repo_root))
    return findings


def validate_state_reason_coherence(control_id: str, state: Any, reasons: list[str], row: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    if state == "ungoverned_unbacked":
        allowed = {
            "lane_capability_absent",
            "control_evidence_incomplete",
            "support_window_incomplete",
            "owner_signoff_missing",
        }
        if not allowed.intersection(reasons):
            findings.append(Finding("error", "row.state_reason_incoherent", "ungoverned_unbacked requires a capability/evidence/support-window/signoff reason", control_id))
    if state == "ungoverned_claim_narrowed" and "claim_label_narrowed" not in reasons:
        findings.append(Finding("error", "row.state_reason_incoherent", "ungoverned_claim_narrowed requires the claim_label_narrowed reason", control_id))
    if state == "ungoverned_stale" and not (
        "control_packet_freshness_breached" in reasons or "control_packet_missing" in reasons
    ):
        findings.append(Finding("error", "row.state_reason_incoherent", "ungoverned_stale requires a packet-freshness reason", control_id))
    if state == "ungoverned_waiver_expired":
        if "waiver_expired" not in reasons:
            findings.append(Finding("error", "row.state_reason_incoherent", "ungoverned_waiver_expired requires the waiver_expired reason", control_id))
        if not isinstance(row.get("waiver"), dict):
            findings.append(Finding("error", "row.waiver_state_without_waiver", "ungoverned_waiver_expired state must name a waiver", control_id))
    if state == "ungoverned_support_expired":
        if "support_window_expired" not in reasons:
            findings.append(Finding("error", "row.state_reason_incoherent", "ungoverned_support_expired requires the support_window_expired reason", control_id))
        if support_window_of(row).get("support_posture") not in ACTIVE_SUPPORT_POSTURES:
            findings.append(Finding("error", "row.expired_state_on_end_of_life_window", "ungoverned_support_expired state names a window that is formally end-of-life", control_id))
    if state == "governed_on_waiver":
        waiver = row.get("waiver")
        if not isinstance(waiver, dict) or not is_str(waiver.get("waiver_ref")) or not is_str(waiver.get("expires_at")):
            findings.append(Finding("error", "row.waiver_state_without_waiver", "governed_on_waiver state must name a waiver", control_id))
    return findings


def validate_row_refs(control_id: str, row: dict[str, Any], repo_root: Path) -> list[Finding]:
    """Path-existence checks for the control-packet and correction-packet refs (not the
    cross-artifact ids, which resolve against the claim manifest)."""
    findings: list[Finding] = []
    refs: list[str] = []
    control = row.get("control_packet")
    if isinstance(control, dict):
        for key in ("packet_ref", "proof_index_ref"):
            ref = control.get(key)
            if is_str(ref):
                refs.append(ref.split("#", 1)[0])
        slo = control.get("freshness_slo")
        if isinstance(slo, dict) and is_str(slo.get("slo_register_ref")):
            refs.append(slo["slo_register_ref"].split("#", 1)[0])
        for ref in control.get("evidence_refs", []) or []:
            if is_str(ref):
                refs.append(ref.split("#", 1)[0])
    correction_ref = row.get("correction_packet_ref")
    if is_str(correction_ref):
        refs.append(correction_ref.split("#", 1)[0])
    for ref in refs:
        if not (repo_root / ref).exists():
            findings.append(Finding("error", "row.ref_missing", f"referenced artifact does not exist: {ref}", control_id))
    return findings


def validate_coverage(packet: dict[str, Any]) -> list[Finding]:
    """Every lane kind is represented, every declared release-blocking lane is covered by
    exactly one release-blocking row, every release-blocking row is declared, and no lane
    ref repeats."""
    findings: list[Finding] = []
    rows = packet.get("rows", [])

    seen: set[str] = set()
    for row in rows:
        ref = row.get("lane_ref")
        if not is_str(ref):
            continue
        if ref in seen:
            findings.append(Finding("error", "coverage.lane_duplicate", f"lane ref appears more than once: {ref}", ref))
        seen.add(ref)

    declared = packet.get("release_blocking_lane_refs", [])
    declared_set = {r for r in declared if is_str(r)} if isinstance(declared, list) else set()
    covered = {
        str(row.get("lane_ref"))
        for row in rows
        if row.get("release_blocking") is True and is_str(row.get("lane_ref"))
    }
    for ref in sorted(declared_set):
        if ref not in covered:
            findings.append(Finding("error", "coverage.release_blocking_uncovered", f"declared release-blocking lane has no covering row: {ref}", ref))
    for row in rows:
        if row.get("release_blocking") is True and is_str(row.get("lane_ref")) and row["lane_ref"] not in declared_set:
            findings.append(Finding("error", "coverage.release_blocking_not_declared", "a release-blocking row's lane is not in release_blocking_lane_refs", str(row.get("control_id"))))

    present_kinds = {row.get("lane_kind") for row in rows}
    for kind in LANE_KINDS:
        if kind not in present_kinds:
            findings.append(Finding("error", "coverage.lane_kind_absent", f"lane kind has no control row: {kind}", kind))
    return findings


def validate_freshness(packet: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    as_of = parse_date(packet.get("as_of"))
    if as_of is None:
        return findings
    for row in packet.get("rows", []):
        control_id = str(row.get("control_id", "<row>"))
        control = row.get("control_packet")
        if not isinstance(control, dict):
            continue
        declared = control.get("slo_state")
        computed = computed_slo_state(control, as_of)
        if computed is None or declared not in FRESHNESS_RANK or computed not in FRESHNESS_RANK:
            continue
        if FRESHNESS_RANK[declared] > FRESHNESS_RANK[computed]:
            findings.append(
                Finding("error", "row.packet_freshness_overstated", f"declared slo_state {declared!r} is fresher than the {computed!r} the as_of date allows", control_id)
            )
        if row.get("control_state") in HOLDING_STATES and computed in ("breached", "missing"):
            findings.append(
                Finding("error", "row.governed_on_breached_packet", "a governed lane rides a control packet past its freshness SLO against as_of", control_id)
            )
    return findings


def validate_waivers(packet: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    as_of = parse_date(packet.get("as_of"))
    if as_of is None:
        return findings
    for row in packet.get("rows", []):
        control_id = str(row.get("control_id", "<row>"))
        state = row.get("control_state")
        waiver = row.get("waiver")
        if not isinstance(waiver, dict):
            continue
        expires = parse_date(waiver.get("expires_at"))
        if expires is None:
            findings.append(Finding("error", "row.waiver_expiry_invalid", "waiver expires_at must be an ISO date", control_id))
            continue
        expired = as_of >= expires
        if expired and state == "governed_on_waiver":
            findings.append(Finding("error", "row.governed_on_expired_waiver", "a lane governs on a waiver that has expired against as_of", control_id))
        if not expired and state == "ungoverned_waiver_expired":
            findings.append(Finding("error", "row.waiver_expired_but_active", "a row is marked waiver-expired but the waiver is still active against as_of", control_id))
    return findings


def validate_support_dates(packet: dict[str, Any]) -> list[Finding]:
    """Support-window-expiry date arithmetic: a support window whose end-of-support date
    has passed against as_of while it still claims active support makes the lane
    expired."""
    findings: list[Finding] = []
    as_of = parse_date(packet.get("as_of"))
    if as_of is None:
        return findings
    for row in packet.get("rows", []):
        control_id = str(row.get("control_id", "<row>"))
        reasons = reasons_of(row)
        state = row.get("control_state")
        window = support_window_of(row)
        for field in ("opened_at", "review_through_date", "end_of_support_date"):
            value = window.get(field)
            if is_str(value) and parse_date(value) is None:
                findings.append(Finding("error", "row.support_date_invalid", f"support_window.{field} must be an ISO date", control_id))
        end = parse_date(window.get("end_of_support_date"))
        active = window.get("support_posture") in ACTIVE_SUPPORT_POSTURES
        expired = end is not None and as_of >= end and active
        if expired:
            if "support_window_expired" not in reasons:
                findings.append(Finding("error", "row.support_expired_undeclared", "a lane has a support window past its end-of-support date but does not name support_window_expired", control_id))
            if state in HOLDING_STATES:
                findings.append(Finding("error", "row.governed_on_expired_window", "a governed lane carries a support window whose end-of-support date is overdue against as_of", control_id))
        elif "support_window_expired" in reasons:
            findings.append(Finding("error", "row.support_declared_expired_but_active", "a row names support_window_expired but the window is not past its end-of-support date against as_of", control_id))
    return findings


def validate_ceiling(packet: dict[str, Any], repo_root: Path) -> list[Finding]:
    """Cross-artifact ingestion: read the stable claim manifest and fail when a row's
    claim_label is not the label the claim manifest publishes for the entry named by
    claim_ref, or names an entry the claim manifest does not carry."""
    findings: list[Finding] = []
    ref = packet.get("claim_manifest_ref")
    if not is_str(ref):
        return findings
    path = repo_root / ref
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
        control_id = str(row.get("control_id", "<row>"))
        claim_ref = row.get("claim_ref")
        if not is_str(claim_ref):
            continue
        if claim_ref not in published_by_entry:
            findings.append(Finding("error", "ceiling.claim_entry_unknown", f"claim_ref is not an entry in the stable claim manifest: {claim_ref}", control_id))
            continue
        canonical = published_by_entry[claim_ref]
        if row.get("claim_label") != canonical:
            findings.append(Finding("error", "ceiling.claim_label_mismatch", f"claim_label {row.get('claim_label')!r} does not match the claim manifest published_label {canonical!r}", control_id))
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
    decision, rule_ids, lane_ids = computed_publication(packet)
    if publication.get("decision") != decision:
        findings.append(
            Finding("error", "publication.decision_inconsistent", f"declared decision {publication.get('decision')!r} disagrees with computed {decision!r}", "<packet>")
        )
    if list(publication.get("blocking_rule_ids", [])) != rule_ids:
        findings.append(Finding("error", "publication.blocking_rules_mismatch", "blocking_rule_ids disagrees with the firing control rules", "<packet>"))
    if list(publication.get("blocking_lane_ids", [])) != lane_ids:
        findings.append(Finding("error", "publication.blocking_lanes_mismatch", "blocking_lane_ids disagrees with the firing control rules", "<packet>"))
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
        findings.append(Finding("error", "packet.rows_empty", "packet must enumerate at least one row", "<packet>"))
        return findings

    seen: set[str] = set()
    for raw in rows:
        row = ensure_dict(raw, "packet.rows[]")
        findings.extend(validate_row(row, repo_root))
        control_id = str(row.get("control_id", "<row>"))
        if control_id in seen:
            findings.append(Finding("error", "row.duplicate_id", "control ids must be unique", control_id))
        seen.add(control_id)

    findings.extend(validate_coverage(packet))
    findings.extend(validate_freshness(packet))
    findings.extend(validate_waivers(packet))
    findings.extend(validate_support_dates(packet))
    findings.extend(validate_ceiling(packet, repo_root))
    findings.extend(validate_publication(packet))
    findings.extend(validate_summary(packet))
    return findings


def recompute_derived(packet: dict[str, Any]) -> None:
    packet["summary"] = computed_summary(packet)
    decision, rule_ids, lane_ids = computed_publication(packet)
    if isinstance(packet.get("publication"), dict):
        packet["publication"].update(
            {"decision": decision, "blocking_rule_ids": rule_ids, "blocking_lane_ids": lane_ids}
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

    # A narrowing row that still governs a stable label must be flagged.
    mutated = copy.deepcopy(packet)
    target = next((r for r in mutated["rows"] if r.get("control_state") in NARROWING_STATES), None)
    if target is not None:
        target["controlled_label"] = "stable"
        recompute_derived(mutated)
        record("controlled_not_narrowed_rejected", "row.controlled_not_narrowed", "row.controlled_not_narrowed" in check_ids(mutated))

    # A governed row carrying an active gap reason must be flagged.
    mutated = copy.deepcopy(packet)
    target = next((r for r in mutated["rows"] if r.get("control_state") in HOLDING_STATES), None)
    if target is not None:
        target["active_gap_reasons"] = ["control_evidence_incomplete"]
        recompute_derived(mutated)
        record("held_with_active_gap_rejected", "row.held_with_active_gap", "row.held_with_active_gap" in check_ids(mutated))

    # A governed row whose packet is breached against as_of must be flagged.
    mutated = copy.deepcopy(packet)
    target = next((r for r in mutated["rows"] if r.get("control_state") in HOLDING_STATES), None)
    if target is not None:
        target["control_packet"]["captured_at"] = "2000-01-01"
        record("governed_on_breached_packet_rejected", "row.governed_on_breached_packet", "row.governed_on_breached_packet" in check_ids(mutated))

    # A packet whose declared state overstates its freshness must be flagged.
    mutated = copy.deepcopy(packet)
    target = next(
        (r for r in mutated["rows"] if r.get("control_packet", {}).get("slo_state") == "current"),
        None,
    )
    if target is not None:
        target["control_packet"]["captured_at"] = "2000-01-01"
        record("packet_freshness_overstated_rejected", "row.packet_freshness_overstated", "row.packet_freshness_overstated" in check_ids(mutated))

    # A row whose claim_label disagrees with the claim manifest must be flagged.
    mutated = copy.deepcopy(packet)
    target = next((r for r in mutated["rows"] if r.get("claim_label") == "beta"), None)
    if target is not None:
        target["claim_label"] = "stable"
        record("claim_label_mismatch_rejected", "ceiling.claim_label_mismatch", "ceiling.claim_label_mismatch" in check_ids(mutated))

    # A control backed wider than its public claim's ceiling must be flagged.
    mutated = copy.deepcopy(packet)
    target = next(
        (r for r in mutated["rows"] if r.get("claim_label") == "beta" and not is_above_cutline(r.get("controlled_label"))),
        None,
    )
    if target is not None:
        target["controlled_label"] = "lts"
        recompute_derived(mutated)
        record("controlled_wider_than_claim_rejected", "row.controlled_wider_than_claim", "row.controlled_wider_than_claim" in check_ids(mutated))

    # Renewing past the end-of-support date without declaring expiry must be flagged.
    mutated = copy.deepcopy(packet)
    target = next((r for r in mutated["rows"] if r.get("control_state") == "governed"), None)
    if target is not None:
        target["support_window"] = {
            "opened_at": "2024-01-01",
            "review_through_date": "2024-06-01",
            "end_of_support_date": "2025-01-01",
            "support_posture": "full_support",
        }
        record("support_expired_undeclared_rejected", "row.support_expired_undeclared", "row.support_expired_undeclared" in check_ids(mutated))

    # Dropping a declared release-blocking lane's row must be flagged.
    mutated = copy.deepcopy(packet)
    declared = mutated.get("release_blocking_lane_refs", [])
    if declared:
        dropped = declared[0]
        mutated["rows"] = [r for r in mutated["rows"] if r.get("lane_ref") != dropped]
        recompute_derived(mutated)
        record("release_blocking_uncovered_rejected", "coverage.release_blocking_uncovered", "coverage.release_blocking_uncovered" in check_ids(mutated))

    # Removing every row of a lane kind must be flagged.
    mutated = copy.deepcopy(packet)
    mutated["rows"] = [r for r in mutated["rows"] if r.get("lane_kind") != "support_window"]
    recompute_derived(mutated)
    record("lane_kind_absent_rejected", "coverage.lane_kind_absent", "coverage.lane_kind_absent" in check_ids(mutated))

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
    decision, rule_ids, lane_ids = computed_publication(packet)
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "record_kind": "maintenance_control_packet_validation_capture",
        "status": "pass" if not any(f.severity == "error" for f in findings) else "fail",
        "generated_at": generated_at_now(),
        "packet_id": packet.get("packet_id"),
        "as_of": packet.get("as_of"),
        "summary": packet.get("summary"),
        "publication": {
            "decision": decision,
            "blocking_rule_ids": rule_ids,
            "blocking_lane_ids": lane_ids,
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
        load_json(repo_root / args.packet, "maintenance-control packet"),
        "maintenance-control packet",
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
    decision, rule_ids, _lane_ids = computed_publication(packet)
    print(
        "maintenance-control packet validated "
        f"({summary.get('total_lanes')} lanes across {summary.get('total_claims')} claims, "
        f"{summary.get('lanes_governed_stable')} governed stable, "
        f"{summary.get('lanes_narrowed_below_cutline')} narrowed; "
        f"kinds hotfix={summary.get('hotfix_lanes')} backport={summary.get('backport_lanes')} "
        f"correction_train={summary.get('correction_train_lanes')} support_window={summary.get('support_window_lanes')}; "
        f"release-blocking {summary.get('release_blocking_governed_stable')}/{summary.get('release_blocking_total')} governed; "
        f"packets {summary.get('packets_current')} current / "
        f"{summary.get('packets_due_for_refresh')} due / "
        f"{summary.get('packets_breached')} breached / "
        f"{summary.get('packets_missing')} missing; "
        f"{summary.get('lanes_support_expired')} support-expired; "
        f"{summary.get('control_rules_firing')} rules firing; "
        f"publication={decision}; {len(drill_results)} negative drills, {len(fixture_results)} fixture cases)"
    )

    if args.require_proceed and decision == "hold":
        print(
            f"PUBLICATION HELD: maintenance-control publication blocked by {len(rule_ids)} rule(s): {', '.join(rule_ids)}",
            file=sys.stderr,
        )
        return 2

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
