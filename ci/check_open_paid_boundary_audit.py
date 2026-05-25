#!/usr/bin/env python3
"""Validate the open-versus-paid boundary, licensing, provenance, and contribution-policy audit.

The stable claim manifest decides the single canonical lifecycle label each *subject*
publishes; the stable proof index and stable version windows ground the requirements and
interface surfaces that are meant to ship at the cutline. This audit is the governance-fact
layer beside those gates: for every audited subject — where the open-source core ends and the
paid/managed tier begins, the licensing posture, the build provenance, or the contribution
policy — it asks whether the subject carries a fresh, owner-signed attestation packet with
its required audit controls satisfied, or whether it is riding on optimism and an adjacent
attested row. It reads the checked-in audit at ``artifacts/release/open_paid_boundary_audit.json``
and:

  - asserts the closed vocabularies (lifecycle labels, domains, states, gap reasons, actions)
    and the launch cutline are canonical;
  - asserts every attested row renders the public claim's canonical label cleanly with a
    captured within-SLO packet, satisfied audit controls, an owner sign-off, and no expired
    waiver, and that every narrowing row drops below the cutline rather than rendering a
    label wider than the public claim;
  - asserts domain coverage (every domain has at least one row), the release-blocking set is
    mutually covered, and no entry id or subject ref repeats;
  - performs the ceiling cross-check the typed model cannot — it reads the stable claim
    manifest named by ``claim_manifest_ref`` and fails when a row's ``claim_label`` is not the
    label the claim manifest publishes for the entry named by ``claim_ref``, or names an entry
    the claim manifest does not carry;
  - performs the packet-freshness SLO automation and the waiver-expiry date arithmetic against
    the audit ``as_of`` date the typed model cannot;
  - recomputes the publication verdict and the summary block, and fails on any drift;
  - runs negative drills proving the narrowing, ceiling, freshness, control, and publication
    rejections all fire; and
  - runs the checked-in fixture cases under ``fixtures/release/open_paid_boundary_audit`` and
    fails when a case the audit marks as rejected validates clean.

With ``--require-proceed`` the gate additionally fails (exit code 2) when the recomputed
publication verdict is ``hold``, so shiproom and release tooling can fail promotion directly
from this artifact.

The typed Rust consumer
(``aureline_release::open_paid_boundary_audit::current_open_paid_boundary_audit``)
reads the same audit and runs the same structural cross-check, so this gate and
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


DEFAULT_AUDIT_REL = "artifacts/release/open_paid_boundary_audit.json"
DEFAULT_REPORT_REL = "artifacts/release/captures/open_paid_boundary_audit_validation_capture.json"
DEFAULT_FIXTURES_REL = "fixtures/release/open_paid_boundary_audit"

EXPECTED_SCHEMA_VERSION = 1
AUDIT_RECORD_KIND = "open_paid_boundary_audit"

LIFECYCLE_LABELS = ("lts", "stable", "beta", "preview", "withdrawn")
ABOVE_CUTLINE = ("lts", "stable")
BELOW_CUTLINE = ("beta", "preview", "withdrawn")
AUDIT_DOMAINS = ("open_paid_boundary", "licensing", "provenance", "contribution_policy")
AUDIT_STATES = (
    "attested",
    "attested_on_waiver",
    "narrowed_unbacked",
    "narrowed_claim_narrowed",
    "narrowed_stale",
    "narrowed_waiver_expired",
)
GAP_REASONS = (
    "claim_label_narrowed",
    "audit_evidence_incomplete",
    "attestation_packet_freshness_breached",
    "attestation_packet_missing",
    "waiver_expired",
    "owner_signoff_missing",
    "audit_control_unsatisfied",
)
AUDIT_ACTIONS = (
    "hold_publication",
    "narrow_audit_label",
    "refresh_attestation_packet",
    "satisfy_audit_control",
    "recapture_audit_evidence",
    "request_owner_signoff",
)
FRESHNESS_SLO_STATES = ("current", "due_for_refresh", "breached", "missing")
HOLDING_STATES = ("attested", "attested_on_waiver")
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
    parser.add_argument("--audit", default=DEFAULT_AUDIT_REL)
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
    packet = row.get("attestation_packet")
    return packet if isinstance(packet, dict) else {}


def has_capture(packet: dict[str, Any]) -> bool:
    return bool(packet.get("captured_at")) and bool(packet.get("evidence_refs"))


def controls_of(row: dict[str, Any]) -> list[dict[str, Any]]:
    controls = row.get("audit_controls", [])
    return [c for c in controls if isinstance(c, dict)] if isinstance(controls, list) else []


def unsatisfied_control_count(row: dict[str, Any]) -> int:
    return sum(1 for c in controls_of(row) if c.get("satisfied") is not True)


def all_controls_satisfied(row: dict[str, Any]) -> bool:
    controls = controls_of(row)
    return bool(controls) and all(c.get("satisfied") is True for c in controls)


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


def computed_publication(audit: dict[str, Any]) -> tuple[str, list[str], list[str]]:
    rows = audit.get("rows", [])
    rules = audit.get("rules", [])
    blocking_rules = [
        rule
        for rule in rules
        if rule.get("blocks_publication") is True and rule_fires(rule, rows)
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


def computed_summary(audit: dict[str, Any]) -> dict[str, Any]:
    rows = audit.get("rows", [])
    rules = audit.get("rules", [])

    def packet_count(state: str) -> int:
        return sum(1 for r in rows if packet_of(r).get("slo_state") == state)

    holding = [r for r in rows if is_above_cutline(r.get("effective_label"))]
    release_blocking = [r for r in rows if r.get("release_blocking") is True]
    rb_holding = [r for r in release_blocking if is_above_cutline(r.get("effective_label"))]
    claims = {str(r.get("claim_ref")) for r in rows}
    total_controls = sum(len(controls_of(r)) for r in rows)
    controls_unsatisfied = sum(unsatisfied_control_count(r) for r in rows)
    return {
        "total_rows": len(rows),
        "total_claims": len(claims),
        "rows_attested": len(holding),
        "rows_narrowed_below_cutline": len(rows) - len(holding),
        "release_blocking_total": len(release_blocking),
        "release_blocking_attested": len(rb_holding),
        "release_blocking_narrowed": len(release_blocking) - len(rb_holding),
        "rows_on_active_waiver": sum(
            1 for r in rows if r.get("audit_state") == "attested_on_waiver"
        ),
        "packets_current": packet_count("current"),
        "packets_due_for_refresh": packet_count("due_for_refresh"),
        "packets_breached": packet_count("breached"),
        "packets_missing": packet_count("missing"),
        "total_controls": total_controls,
        "controls_satisfied": total_controls - controls_unsatisfied,
        "controls_unsatisfied": controls_unsatisfied,
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


def validate_envelope(audit: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings: list[Finding] = []
    audit_id = str(audit.get("audit_id", "<audit>"))

    if audit.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(Finding("error", "audit.schema_version", "schema_version must be 1", audit_id))
    if audit.get("record_kind") != AUDIT_RECORD_KIND:
        findings.append(Finding("error", "audit.record_kind", "record_kind is not supported", audit_id))
    for field in ("audit_id", "status", "overview_page", "as_of", "claim_manifest_ref", "stable_proof_index_ref"):
        if not is_str(audit.get(field)):
            findings.append(Finding("error", "audit.empty_field", f"{field} must be a non-empty string", audit_id))
    if parse_date(audit.get("as_of")) is None:
        findings.append(Finding("error", "audit.as_of_invalid", "as_of must be an ISO date", audit_id))

    for field in ("claim_manifest_ref", "stable_proof_index_ref"):
        ref = audit.get(field)
        if is_str(ref) and not (repo_root / ref.split("#", 1)[0]).exists():
            findings.append(Finding("error", "audit.ref_missing", f"{field} does not exist: {ref}", audit_id))

    for key, expected in (
        ("lifecycle_labels", list(LIFECYCLE_LABELS)),
        ("audit_domains", list(AUDIT_DOMAINS)),
        ("audit_states", list(AUDIT_STATES)),
        ("gap_reasons", list(GAP_REASONS)),
        ("audit_actions", list(AUDIT_ACTIONS)),
    ):
        if list(audit.get(key, [])) != expected:
            findings.append(Finding("error", "audit.vocabulary", f"audit.{key} is not the closed vocabulary", key))

    cutline = audit.get("launch_cutline")
    if not isinstance(cutline, dict):
        findings.append(Finding("error", "cutline.missing", "audit must carry a launch_cutline", audit_id))
    else:
        if cutline.get("cutline_level") != "stable":
            findings.append(Finding("error", "cutline.level", "cutline_level must be stable", audit_id))
        if list(cutline.get("above_cutline_levels", [])) != list(ABOVE_CUTLINE):
            findings.append(Finding("error", "cutline.above", "above_cutline_levels is not canonical", audit_id))
        if list(cutline.get("below_cutline_levels", [])) != list(BELOW_CUTLINE):
            findings.append(Finding("error", "cutline.below", "below_cutline_levels is not canonical", audit_id))
        if not is_str(cutline.get("description")):
            findings.append(Finding("error", "cutline.description", "cutline description must be non-empty", audit_id))
    return findings


def validate_rules(audit: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    rules = audit.get("rules")
    if not isinstance(rules, list) or not rules:
        findings.append(Finding("error", "rules.empty", "audit must enumerate at least one rule", "<audit>"))
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
        if rule.get("default_action") not in AUDIT_ACTIONS:
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


def validate_packet_block(entry_id: str, packet: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    for field in ("packet_id", "packet_ref", "proof_index_ref"):
        if not is_str(packet.get(field)):
            findings.append(Finding("error", "packet.empty_field", f"attestation_packet.{field} must be non-empty", entry_id))
    if packet.get("slo_state") not in FRESHNESS_SLO_STATES:
        findings.append(Finding("error", "packet.state_invalid", "slo_state is outside the vocabulary", entry_id))
    slo = packet.get("freshness_slo")
    if not isinstance(slo, dict):
        findings.append(Finding("error", "packet.slo_missing", "attestation_packet must carry a freshness_slo block", entry_id))
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


def validate_controls(entry_id: str, row: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    controls = controls_of(row)
    if not controls:
        findings.append(Finding("error", "control.empty", "a row must carry at least one audit control", entry_id))
    seen: set[str] = set()
    for control in controls:
        control_id = str(control.get("control_id", "<control>"))
        if control_id in seen:
            findings.append(Finding("error", "control.duplicate_id", "control ids must be unique within a row", entry_id))
        seen.add(control_id)
        for field in ("control_id", "title", "control_ref"):
            if not is_str(control.get(field)):
                findings.append(Finding("error", "control.empty_field", f"control {field} must be non-empty", entry_id))
        if not isinstance(control.get("satisfied"), bool):
            findings.append(Finding("error", "control.satisfied_invalid", "control satisfied must be a boolean", entry_id))
    if unsatisfied_control_count(row) > 0 and "audit_control_unsatisfied" not in reasons_of(row):
        findings.append(Finding("error", "control.unsatisfied_without_reason", "an unsatisfied control must name the audit_control_unsatisfied reason", entry_id))
    return findings


def validate_row(row: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings: list[Finding] = []
    entry_id = str(row.get("entry_id", "<row>"))

    for field in ("entry_id", "title", "subject_ref", "subject_summary", "claim_ref", "rationale"):
        if not is_str(row.get(field)):
            findings.append(Finding("error", "row.empty_field", f"row {field} must be non-empty", entry_id))

    if row.get("domain") not in AUDIT_DOMAINS:
        findings.append(Finding("error", "row.domain_invalid", "domain is outside the vocabulary", entry_id))
    if not isinstance(row.get("release_blocking"), bool):
        findings.append(Finding("error", "row.release_blocking_invalid", "release_blocking must be a boolean", entry_id))

    claim_label = row.get("claim_label")
    effective = row.get("effective_label")
    state = row.get("audit_state")
    reasons = reasons_of(row)

    if claim_label not in LIFECYCLE_LABELS:
        findings.append(Finding("error", "row.claim_label_invalid", "claim_label is invalid", entry_id))
    if effective not in LIFECYCLE_LABELS:
        findings.append(Finding("error", "row.effective_invalid", "effective_label is invalid", entry_id))
    if state not in AUDIT_STATES:
        findings.append(Finding("error", "row.state_invalid", "audit_state is invalid", entry_id))
    if any(reason not in GAP_REASONS for reason in reasons):
        findings.append(Finding("error", "row.reason_invalid", "active_gap_reasons has an unknown reason", entry_id))

    # The ceiling: no row may render a label wider than the public claim's label.
    if claim_label in LEVEL_RANK and effective in LEVEL_RANK and LEVEL_RANK[effective] > LEVEL_RANK[claim_label]:
        findings.append(Finding("error", "row.effective_wider_than_claim", "effective_label is wider than the claim ceiling", entry_id))

    packet = packet_of(row)
    findings.extend(validate_packet_block(entry_id, packet))
    findings.extend(validate_controls(entry_id, row))
    slo_state = packet.get("slo_state")

    holding = state in HOLDING_STATES
    claim_holds = is_above_cutline(claim_label)

    # A claim canonically below the cutline forces the row to inherit the ceiling and narrow.
    if claim_label in LEVEL_RANK and not claim_holds:
        if holding:
            findings.append(Finding("error", "row.held_on_narrowed_claim", "a row attests while the public claim label is below the cutline", entry_id))
        if "claim_label_narrowed" not in reasons:
            findings.append(Finding("error", "row.claim_narrowed_without_reason", "a row whose claim is narrowed must name claim_label_narrowed", entry_id))

    if holding:
        if claim_label != effective:
            findings.append(Finding("error", "row.held_label_not_equal_claim", "an attested row must render the public claim's canonical label", entry_id))
        if reasons:
            findings.append(Finding("error", "row.held_with_active_gap", "an attested row must carry no active gap reason", entry_id))
        if not has_capture(packet):
            findings.append(Finding("error", "row.held_without_fresh_packet", "an attested row must ride a captured, evidence-backed packet", entry_id))
        if slo_state in ("breached", "missing"):
            findings.append(Finding("error", "row.held_on_stale_packet", "an attested row must ride a packet within its freshness SLO", entry_id))
        if not all_controls_satisfied(row):
            findings.append(Finding("error", "row.held_with_unsatisfied_control", "an attested row must satisfy every required audit control", entry_id))
        if not owner_signed(row):
            findings.append(Finding("error", "row.held_without_owner_signoff", "an attested row must carry an owner sign-off with a date", entry_id))
    else:
        if is_above_cutline(effective):
            findings.append(Finding("error", "row.effective_not_narrowed", "a row that is not attested must narrow below the cutline", entry_id))
        if not reasons:
            findings.append(Finding("error", "row.narrowing_without_reason", "a narrowing row must name an active gap reason", entry_id))
        if slo_state == "breached" and "attestation_packet_freshness_breached" not in reasons:
            findings.append(Finding("error", "row.breached_without_reason", "a breached packet must name attestation_packet_freshness_breached", entry_id))
        if slo_state == "missing" and "attestation_packet_missing" not in reasons:
            findings.append(Finding("error", "row.missing_without_reason", "a missing packet must name attestation_packet_missing", entry_id))

    signoff = row.get("owner_signoff", {})
    if not (isinstance(signoff, dict) and is_str(signoff.get("owner_ref"))):
        findings.append(Finding("error", "row.signoff_owner_missing", "owner_signoff.owner_ref must be non-empty", entry_id))

    findings.extend(validate_state_reason_coherence(entry_id, state, reasons, row))
    findings.extend(validate_row_refs(entry_id, row, repo_root))
    return findings


def validate_state_reason_coherence(entry_id: str, state: Any, reasons: list[str], row: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    if state == "narrowed_unbacked":
        allowed = {"audit_evidence_incomplete", "owner_signoff_missing", "audit_control_unsatisfied"}
        if not allowed.intersection(reasons):
            findings.append(Finding("error", "row.state_reason_incoherent", "narrowed_unbacked requires an evidence/signoff/control reason", entry_id))
    if state == "narrowed_claim_narrowed" and "claim_label_narrowed" not in reasons:
        findings.append(Finding("error", "row.state_reason_incoherent", "narrowed_claim_narrowed requires the claim_label_narrowed reason", entry_id))
    if state == "narrowed_stale" and not ({"attestation_packet_freshness_breached", "attestation_packet_missing"} & set(reasons)):
        findings.append(Finding("error", "row.state_reason_incoherent", "narrowed_stale requires a packet freshness/missing reason", entry_id))
    if state == "narrowed_waiver_expired":
        if "waiver_expired" not in reasons:
            findings.append(Finding("error", "row.state_reason_incoherent", "narrowed_waiver_expired requires the waiver_expired reason", entry_id))
        if not isinstance(row.get("waiver"), dict):
            findings.append(Finding("error", "row.waiver_state_without_waiver", "narrowed_waiver_expired state must name a waiver", entry_id))
    if state == "attested_on_waiver":
        waiver = row.get("waiver")
        if not isinstance(waiver, dict) or not is_str(waiver.get("waiver_ref")) or not is_str(waiver.get("expires_at")):
            findings.append(Finding("error", "row.waiver_state_without_waiver", "attested_on_waiver state must name a waiver", entry_id))
    return findings


def validate_row_refs(entry_id: str, row: dict[str, Any], repo_root: Path) -> list[Finding]:
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
    for ref in refs:
        if not (repo_root / ref).exists():
            findings.append(Finding("error", "row.ref_missing", f"referenced artifact does not exist: {ref}", entry_id))
    return findings


def validate_coverage(audit: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    rows = audit.get("rows", [])

    seen: set[str] = set()
    subjects: set[str] = set()
    for row in rows:
        rid = row.get("entry_id")
        if is_str(rid):
            if rid in seen:
                findings.append(Finding("error", "row.duplicate_id", f"entry id appears more than once: {rid}", rid))
            seen.add(rid)
        subject = row.get("subject_ref")
        if is_str(subject):
            if subject in subjects:
                findings.append(Finding("error", "row.duplicate_subject_ref", f"subject ref appears more than once: {subject}", str(rid)))
            subjects.add(subject)

    declared = audit.get("release_blocking_audit_refs", [])
    declared_set = {r for r in declared if is_str(r)} if isinstance(declared, list) else set()
    covered = {
        str(row.get("entry_id"))
        for row in rows
        if row.get("release_blocking") is True and is_str(row.get("entry_id"))
    }
    for ref in sorted(declared_set):
        if ref not in covered:
            findings.append(Finding("error", "coverage.release_blocking_ref_without_row", f"declared release-blocking audit has no row: {ref}", ref))
    for row in rows:
        if row.get("release_blocking") is True and is_str(row.get("entry_id")) and row["entry_id"] not in declared_set:
            findings.append(Finding("error", "coverage.release_blocking_not_declared", "a release-blocking row is not in release_blocking_audit_refs", str(row.get("entry_id"))))

    present_domains = {row.get("domain") for row in rows}
    for domain in AUDIT_DOMAINS:
        if domain not in present_domains:
            findings.append(Finding("error", "coverage.domain_absent", f"audit domain has no row: {domain}", domain))
    return findings


def validate_freshness(audit: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    as_of = parse_date(audit.get("as_of"))
    if as_of is None:
        return findings
    for row in audit.get("rows", []):
        entry_id = str(row.get("entry_id", "<row>"))
        pkt = packet_of(row)
        declared = pkt.get("slo_state")
        computed = computed_slo_state(pkt, as_of)
        if computed is None or declared not in FRESHNESS_RANK or computed not in FRESHNESS_RANK:
            continue
        if FRESHNESS_RANK[declared] > FRESHNESS_RANK[computed]:
            findings.append(Finding("error", "row.packet_freshness_overstated", f"declared slo_state {declared!r} is fresher than the {computed!r} the as_of date allows", entry_id))
        if row.get("audit_state") in HOLDING_STATES and computed in ("breached", "missing"):
            findings.append(Finding("error", "row.held_on_breached_packet", "an attested row rides a packet past its SLO against as_of", entry_id))
    return findings


def validate_waivers(audit: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    as_of = parse_date(audit.get("as_of"))
    if as_of is None:
        return findings
    for row in audit.get("rows", []):
        entry_id = str(row.get("entry_id", "<row>"))
        state = row.get("audit_state")
        waiver = row.get("waiver")
        if not isinstance(waiver, dict):
            continue
        expires = parse_date(waiver.get("expires_at"))
        if expires is None:
            findings.append(Finding("error", "row.waiver_expiry_invalid", "waiver expires_at must be an ISO date", entry_id))
            continue
        expired = as_of >= expires
        if expired and state == "attested_on_waiver":
            findings.append(Finding("error", "row.held_on_expired_waiver", "a row attests on a waiver that has expired against as_of", entry_id))
        if not expired and state == "narrowed_waiver_expired":
            findings.append(Finding("error", "row.waiver_expired_but_active", "a row is marked waiver-expired but the waiver is still active against as_of", entry_id))
    return findings


def validate_ceiling(audit: dict[str, Any], repo_root: Path) -> list[Finding]:
    """Cross-artifact ingestion: read the stable claim manifest and fail when a row's
    claim_label is not the label the claim manifest publishes for the entry named by
    claim_ref, or names an entry the claim manifest does not carry."""
    findings: list[Finding] = []
    ref = audit.get("claim_manifest_ref")
    if not is_str(ref):
        return findings
    path = repo_root / ref.split("#", 1)[0]
    if not path.exists():
        findings.append(Finding("error", "ceiling.artifact_missing", f"claim_manifest_ref does not exist: {ref}", "<audit>"))
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

    for row in audit.get("rows", []):
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


def validate_publication(audit: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    publication = audit.get("publication")
    if not isinstance(publication, dict):
        findings.append(Finding("error", "publication.missing", "audit must carry a publication block", "<audit>"))
        return findings
    for field in ("publication_gate", "rationale"):
        if not is_str(publication.get(field)):
            findings.append(Finding("error", "publication.empty_field", f"publication.{field} must be non-empty", "<audit>"))
    decision, rule_ids, entry_ids = computed_publication(audit)
    if publication.get("decision") != decision:
        findings.append(Finding("error", "publication.decision_inconsistent", f"declared decision {publication.get('decision')!r} disagrees with computed {decision!r}", "<audit>"))
    if list(publication.get("blocking_rule_ids", [])) != rule_ids:
        findings.append(Finding("error", "publication.blocking_rules_mismatch", "blocking_rule_ids disagrees with the firing rules", "<audit>"))
    if list(publication.get("blocking_entry_ids", [])) != entry_ids:
        findings.append(Finding("error", "publication.blocking_entries_mismatch", "blocking_entry_ids disagrees with the firing rules", "<audit>"))
    return findings


def validate_summary(audit: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    summary = audit.get("summary")
    if not isinstance(summary, dict):
        findings.append(Finding("error", "summary.missing", "audit must carry a summary block", "<audit>"))
        return findings
    expected = computed_summary(audit)
    for key, value in expected.items():
        if summary.get(key) != value:
            findings.append(Finding("error", "summary.count_mismatch", f"summary.{key} must equal the recomputed value", key))
    return findings


def validate_audit(audit: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings = validate_envelope(audit, repo_root)
    findings.extend(validate_rules(audit))

    rows = audit.get("rows")
    if not isinstance(rows, list) or not rows:
        findings.append(Finding("error", "audit.rows_empty", "audit must enumerate at least one row", "<audit>"))
        return findings

    for raw in rows:
        row = ensure_dict(raw, "audit.rows[]")
        findings.extend(validate_row(row, repo_root))

    findings.extend(validate_coverage(audit))
    findings.extend(validate_freshness(audit))
    findings.extend(validate_waivers(audit))
    findings.extend(validate_ceiling(audit, repo_root))
    findings.extend(validate_publication(audit))
    findings.extend(validate_summary(audit))
    return findings


def recompute_derived(audit: dict[str, Any]) -> None:
    audit["summary"] = computed_summary(audit)
    decision, rule_ids, entry_ids = computed_publication(audit)
    if isinstance(audit.get("publication"), dict):
        audit["publication"].update(
            {"decision": decision, "blocking_rule_ids": rule_ids, "blocking_entry_ids": entry_ids}
        )


def run_negative_drills(audit: dict[str, Any], repo_root: Path) -> tuple[list[dict[str, str]], list[Finding]]:
    results: list[dict[str, str]] = []
    findings: list[Finding] = []

    def record(drill_id: str, expected_check_id: str, fired: bool) -> None:
        results.append({"drill_id": drill_id, "expected_check_id": expected_check_id, "status": "passed" if fired else "failed"})
        if not fired:
            findings.append(Finding("error", "negative_drill.not_rejected", f"negative drill {drill_id} did not fire", drill_id))

    def check_ids(candidate: dict[str, Any]) -> set[str]:
        return {f.check_id for f in validate_audit(candidate, repo_root)}

    # An attested row rendered with a breached packet against as_of must be flagged.
    mutated = copy.deepcopy(audit)
    target = next((r for r in mutated["rows"] if r.get("audit_state") in HOLDING_STATES), None)
    if target is not None:
        target["attestation_packet"]["captured_at"] = "2000-01-01"
        record("held_on_breached_packet_rejected", "row.held_on_breached_packet", "row.held_on_breached_packet" in check_ids(mutated))

    # A narrowing row that still renders a stable label must be flagged.
    mutated = copy.deepcopy(audit)
    target = next((r for r in mutated["rows"] if r.get("audit_state") in NARROWING_STATES), None)
    if target is not None:
        target["effective_label"] = "stable"
        recompute_derived(mutated)
        record("effective_not_narrowed_rejected", "row.effective_not_narrowed", "row.effective_not_narrowed" in check_ids(mutated))

    # An attested row with an unsatisfied control must be flagged.
    mutated = copy.deepcopy(audit)
    target = next((r for r in mutated["rows"] if r.get("audit_state") in HOLDING_STATES), None)
    if target is not None and target.get("audit_controls"):
        target["audit_controls"][0]["satisfied"] = False
        record("held_with_unsatisfied_control_rejected", "row.held_with_unsatisfied_control", "row.held_with_unsatisfied_control" in check_ids(mutated))

    # An attested row with a missing owner sign-off must be flagged.
    mutated = copy.deepcopy(audit)
    target = next((r for r in mutated["rows"] if r.get("audit_state") in HOLDING_STATES), None)
    if target is not None:
        target["owner_signoff"]["signed_off"] = False
        target["owner_signoff"]["signed_at"] = None
        record("held_without_owner_signoff_rejected", "row.held_without_owner_signoff", "row.held_without_owner_signoff" in check_ids(mutated))

    # A row whose claim_label disagrees with the claim manifest must be flagged.
    mutated = copy.deepcopy(audit)
    target = next((r for r in mutated["rows"] if r.get("claim_label") == "beta"), None)
    if target is not None:
        target["claim_label"] = "stable"
        record("claim_label_mismatch_rejected", "ceiling.claim_label_mismatch", "ceiling.claim_label_mismatch" in check_ids(mutated))

    # Removing every row of a domain must be flagged.
    mutated = copy.deepcopy(audit)
    mutated["rows"] = [r for r in mutated["rows"] if r.get("domain") != "open_paid_boundary"]
    recompute_derived(mutated)
    record("domain_absent_rejected", "coverage.domain_absent", "coverage.domain_absent" in check_ids(mutated))

    # Dropping a declared release-blocking row must be flagged.
    mutated = copy.deepcopy(audit)
    declared = mutated.get("release_blocking_audit_refs", [])
    if declared:
        dropped = declared[0]
        mutated["rows"] = [r for r in mutated["rows"] if r.get("entry_id") != dropped]
        recompute_derived(mutated)
        record("release_blocking_uncovered_rejected", "coverage.release_blocking_ref_without_row", "coverage.release_blocking_ref_without_row" in check_ids(mutated))

    # A publication decision that disagrees with the firing rules must be flagged.
    mutated = copy.deepcopy(audit)
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
        ids = {f.check_id for f in validate_audit(ensure_dict(candidate, "fixture"), repo_root)}
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
    audit: dict[str, Any],
    findings: list[Finding],
    drill_results: list[dict[str, str]],
    fixture_results: list[dict[str, str]],
) -> None:
    decision, rule_ids, entry_ids = computed_publication(audit)
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "record_kind": "open_paid_boundary_audit_validation_capture",
        "status": "pass" if not any(f.severity == "error" for f in findings) else "fail",
        "generated_at": generated_at_now(),
        "audit_id": audit.get("audit_id"),
        "as_of": audit.get("as_of"),
        "summary": audit.get("summary"),
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

    audit = ensure_dict(
        load_json(repo_root / args.audit, "open/paid boundary audit"),
        "open/paid boundary audit",
    )

    findings = validate_audit(audit, repo_root)
    drill_results, drill_findings = run_negative_drills(audit, repo_root)
    findings.extend(drill_findings)
    fixture_results, fixture_findings = run_fixture_cases(repo_root, args.fixtures_dir)
    findings.extend(fixture_findings)

    report_rel = args.report
    if report_rel is None and not args.no_capture:
        report_rel = DEFAULT_REPORT_REL
    if report_rel:
        write_report(repo_root / report_rel, audit, findings, drill_results, fixture_results)

    errors = [f for f in findings if f.severity == "error"]
    if errors:
        for item in errors:
            print(f"ERROR [{item.check_id}] ({item.ref}): {item.message}", file=sys.stderr)
        return 1

    summary = audit.get("summary", {})
    decision, rule_ids, _entry_ids = computed_publication(audit)
    print(
        "open/paid boundary audit validated "
        f"({summary.get('total_rows')} rows across {summary.get('total_claims')} claims, "
        f"{summary.get('rows_attested')} attested, "
        f"{summary.get('rows_narrowed_below_cutline')} narrowed; "
        f"release-blocking {summary.get('release_blocking_attested')}/{summary.get('release_blocking_total')} attested; "
        f"packets {summary.get('packets_current')} current / "
        f"{summary.get('packets_due_for_refresh')} due / "
        f"{summary.get('packets_breached')} breached / "
        f"{summary.get('packets_missing')} missing; "
        f"controls {summary.get('controls_satisfied')}/{summary.get('total_controls')} satisfied; "
        f"{summary.get('rules_firing')} rules firing; "
        f"publication={decision}; {len(drill_results)} negative drills, {len(fixture_results)} fixture cases)"
    )

    if args.require_proceed and decision == "hold":
        print(
            f"PUBLICATION HELD: open/paid boundary audit promotion blocked by {len(rule_ids)} rule(s): {', '.join(rule_ids)}",
            file=sys.stderr,
        )
        return 2

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
