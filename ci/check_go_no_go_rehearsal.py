#!/usr/bin/env python3
"""Validate the final go/no-go rehearsal with explicit cutline, exception packets, and rollback checkpoints.

The stable claim manifest decides the single canonical lifecycle label each *subject*
publishes; the stable proof index and stable version windows ground the requirements and
interface surfaces that are meant to ship at the cutline. This rehearsal is the
launch-rehearsal layer beside those gates: for every rehearsal stage — the explicit
launch-cutline signoff, the promotion publish-step dry run, a rollback-checkpoint drill, or
the open-exception-packet review — it asks whether the stage was actually exercised behind a
fresh, owner-signed rehearsal packet with its required rollback checkpoints verified, or
whether the go/no-go is riding on optimism and an adjacent rehearsed stage. It reads the
checked-in rehearsal at ``artifacts/release/go_no_go_rehearsal.json`` and:

  - asserts the closed vocabularies (lifecycle labels, stage kinds, states, gap reasons,
    actions) and the launch cutline are canonical;
  - asserts every Go stage renders the public claim's canonical label cleanly with a
    captured within-SLO packet, verified rollback checkpoints, an owner sign-off, and no
    expired exception packet, and that every No-Go stage drops below the cutline rather than
    rendering a label wider than the public claim;
  - asserts stage-kind coverage (every kind has at least one row), the release-blocking set
    is mutually covered, and no entry id or subject ref repeats;
  - performs the ceiling cross-check the typed model cannot — it reads the stable claim
    manifest named by ``claim_manifest_ref`` and fails when a row's ``claim_label`` is not
    the label the claim manifest publishes for the entry named by ``claim_ref``, or names an
    entry the claim manifest does not carry;
  - performs the packet-freshness SLO automation and the exception-expiry date arithmetic
    against the rehearsal ``as_of`` date the typed model cannot;
  - recomputes the go/no-go verdict and the summary block, and fails on any drift;
  - runs negative drills proving the narrowing, ceiling, freshness, checkpoint, and
    publication rejections all fire; and
  - runs the checked-in fixture cases under ``fixtures/release/go_no_go_rehearsal`` and fails
    when a case the rehearsal marks as rejected validates clean.

With ``--require-proceed`` the gate additionally fails (exit code 2) when the recomputed
go/no-go verdict is ``hold``, so shiproom and release tooling can fail promotion directly
from this artifact.

The typed Rust consumer
(``aureline_release::go_no_go_rehearsal::current_go_no_go_rehearsal``) reads the same
rehearsal and runs the same structural cross-check, so this gate and
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


DEFAULT_REHEARSAL_REL = "artifacts/release/go_no_go_rehearsal.json"
DEFAULT_REPORT_REL = "artifacts/release/captures/go_no_go_rehearsal_validation_capture.json"
DEFAULT_FIXTURES_REL = "fixtures/release/go_no_go_rehearsal"

EXPECTED_SCHEMA_VERSION = 1
REHEARSAL_RECORD_KIND = "go_no_go_rehearsal"

LIFECYCLE_LABELS = ("lts", "stable", "beta", "preview", "withdrawn")
ABOVE_CUTLINE = ("lts", "stable")
BELOW_CUTLINE = ("beta", "preview", "withdrawn")
STAGE_KINDS = ("cutline_review", "promotion_step", "rollback_checkpoint", "exception_review")
REHEARSAL_STATES = (
    "go_rehearsed",
    "go_on_exception",
    "no_go_unrehearsed",
    "no_go_claim_narrowed",
    "no_go_stale",
    "no_go_exception_expired",
)
GAP_REASONS = (
    "claim_label_narrowed",
    "rehearsal_evidence_incomplete",
    "rehearsal_packet_freshness_breached",
    "rehearsal_packet_missing",
    "exception_expired",
    "owner_signoff_missing",
    "rollback_checkpoint_unverified",
)
REHEARSAL_ACTIONS = (
    "hold_go_no_go",
    "narrow_rehearsal_label",
    "refresh_rehearsal_packet",
    "verify_rollback_checkpoint",
    "recapture_rehearsal_evidence",
    "request_owner_signoff",
)
FRESHNESS_SLO_STATES = ("current", "due_for_refresh", "breached", "missing")
HOLDING_STATES = ("go_rehearsed", "go_on_exception")
NARROWING_STATES = (
    "no_go_unrehearsed",
    "no_go_claim_narrowed",
    "no_go_stale",
    "no_go_exception_expired",
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
    parser.add_argument("--rehearsal", default=DEFAULT_REHEARSAL_REL)
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
    packet = row.get("rehearsal_packet")
    return packet if isinstance(packet, dict) else {}


def has_capture(packet: dict[str, Any]) -> bool:
    return bool(packet.get("captured_at")) and bool(packet.get("evidence_refs"))


def checkpoints_of(row: dict[str, Any]) -> list[dict[str, Any]]:
    checkpoints = row.get("rollback_checkpoints", [])
    return [c for c in checkpoints if isinstance(c, dict)] if isinstance(checkpoints, list) else []


def unverified_checkpoint_count(row: dict[str, Any]) -> int:
    return sum(1 for c in checkpoints_of(row) if c.get("verified") is not True)


def all_checkpoints_verified(row: dict[str, Any]) -> bool:
    checkpoints = checkpoints_of(row)
    return bool(checkpoints) and all(c.get("verified") is True for c in checkpoints)


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


def computed_publication(rehearsal: dict[str, Any]) -> tuple[str, list[str], list[str]]:
    rows = rehearsal.get("rows", [])
    rules = rehearsal.get("rules", [])
    blocking_rules = [
        rule
        for rule in rules
        if rule.get("blocks_go_no_go") is True and rule_fires(rule, rows)
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


def computed_summary(rehearsal: dict[str, Any]) -> dict[str, Any]:
    rows = rehearsal.get("rows", [])
    rules = rehearsal.get("rules", [])

    def packet_count(state: str) -> int:
        return sum(1 for r in rows if packet_of(r).get("slo_state") == state)

    holding = [r for r in rows if is_above_cutline(r.get("effective_label"))]
    release_blocking = [r for r in rows if r.get("release_blocking") is True]
    rb_holding = [r for r in release_blocking if is_above_cutline(r.get("effective_label"))]
    claims = {str(r.get("claim_ref")) for r in rows}
    total_checkpoints = sum(len(checkpoints_of(r)) for r in rows)
    checkpoints_unverified = sum(unverified_checkpoint_count(r) for r in rows)
    return {
        "total_rows": len(rows),
        "total_claims": len(claims),
        "rows_go": len(holding),
        "rows_no_go_below_cutline": len(rows) - len(holding),
        "release_blocking_total": len(release_blocking),
        "release_blocking_go": len(rb_holding),
        "release_blocking_no_go": len(release_blocking) - len(rb_holding),
        "rows_on_active_exception": sum(
            1 for r in rows if r.get("rehearsal_state") == "go_on_exception"
        ),
        "packets_current": packet_count("current"),
        "packets_due_for_refresh": packet_count("due_for_refresh"),
        "packets_breached": packet_count("breached"),
        "packets_missing": packet_count("missing"),
        "total_checkpoints": total_checkpoints,
        "checkpoints_verified": total_checkpoints - checkpoints_unverified,
        "checkpoints_unverified": checkpoints_unverified,
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


def validate_envelope(rehearsal: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings: list[Finding] = []
    rehearsal_id = str(rehearsal.get("rehearsal_id", "<rehearsal>"))

    if rehearsal.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(Finding("error", "rehearsal.schema_version", "schema_version must be 1", rehearsal_id))
    if rehearsal.get("record_kind") != REHEARSAL_RECORD_KIND:
        findings.append(Finding("error", "rehearsal.record_kind", "record_kind is not supported", rehearsal_id))
    for field in ("rehearsal_id", "status", "overview_page", "as_of", "claim_manifest_ref", "stable_proof_index_ref"):
        if not is_str(rehearsal.get(field)):
            findings.append(Finding("error", "rehearsal.empty_field", f"{field} must be a non-empty string", rehearsal_id))
    if parse_date(rehearsal.get("as_of")) is None:
        findings.append(Finding("error", "rehearsal.as_of_invalid", "as_of must be an ISO date", rehearsal_id))

    for field in ("claim_manifest_ref", "stable_proof_index_ref"):
        ref = rehearsal.get(field)
        if is_str(ref) and not (repo_root / ref.split("#", 1)[0]).exists():
            findings.append(Finding("error", "rehearsal.ref_missing", f"{field} does not exist: {ref}", rehearsal_id))

    for key, expected in (
        ("lifecycle_labels", list(LIFECYCLE_LABELS)),
        ("stage_kinds", list(STAGE_KINDS)),
        ("rehearsal_states", list(REHEARSAL_STATES)),
        ("gap_reasons", list(GAP_REASONS)),
        ("rehearsal_actions", list(REHEARSAL_ACTIONS)),
    ):
        if list(rehearsal.get(key, [])) != expected:
            findings.append(Finding("error", "rehearsal.vocabulary", f"rehearsal.{key} is not the closed vocabulary", key))

    cutline = rehearsal.get("launch_cutline")
    if not isinstance(cutline, dict):
        findings.append(Finding("error", "cutline.missing", "rehearsal must carry a launch_cutline", rehearsal_id))
    else:
        if cutline.get("cutline_level") != "stable":
            findings.append(Finding("error", "cutline.level", "cutline_level must be stable", rehearsal_id))
        if list(cutline.get("above_cutline_levels", [])) != list(ABOVE_CUTLINE):
            findings.append(Finding("error", "cutline.above", "above_cutline_levels is not canonical", rehearsal_id))
        if list(cutline.get("below_cutline_levels", [])) != list(BELOW_CUTLINE):
            findings.append(Finding("error", "cutline.below", "below_cutline_levels is not canonical", rehearsal_id))
        if not is_str(cutline.get("description")):
            findings.append(Finding("error", "cutline.description", "cutline description must be non-empty", rehearsal_id))
    return findings


def validate_rules(rehearsal: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    rules = rehearsal.get("rules")
    if not isinstance(rules, list) or not rules:
        findings.append(Finding("error", "rules.empty", "rehearsal must enumerate at least one rule", "<rehearsal>"))
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
        if rule.get("default_action") not in REHEARSAL_ACTIONS:
            findings.append(Finding("error", "rule.action_invalid", "default_action is outside the vocabulary", rule_id))
        labels = rule.get("applies_to_labels")
        if not isinstance(labels, list) or not labels:
            findings.append(Finding("error", "rule.labels_empty", "rule must watch at least one label", rule_id))
        elif any(label not in LIFECYCLE_LABELS for label in labels):
            findings.append(Finding("error", "rule.labels_invalid", "applies_to_labels has an unknown label", rule_id))
        if not isinstance(rule.get("blocks_go_no_go"), bool):
            findings.append(Finding("error", "rule.blocks_invalid", "blocks_go_no_go must be a boolean", rule_id))

    for reason in GAP_REASONS:
        if reason not in covered:
            findings.append(Finding("error", "rule.reason_uncovered", "gap reason has no rule watching for it", reason))
    return findings


def validate_packet_block(entry_id: str, packet: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    for field in ("packet_id", "packet_ref", "proof_index_ref"):
        if not is_str(packet.get(field)):
            findings.append(Finding("error", "packet.empty_field", f"rehearsal_packet.{field} must be non-empty", entry_id))
    if packet.get("slo_state") not in FRESHNESS_SLO_STATES:
        findings.append(Finding("error", "packet.state_invalid", "slo_state is outside the vocabulary", entry_id))
    slo = packet.get("freshness_slo")
    if not isinstance(slo, dict):
        findings.append(Finding("error", "packet.slo_missing", "rehearsal_packet must carry a freshness_slo block", entry_id))
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


def validate_checkpoints(entry_id: str, row: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    checkpoints = checkpoints_of(row)
    if not checkpoints:
        findings.append(Finding("error", "checkpoint.empty", "a row must carry at least one rollback checkpoint", entry_id))
    seen: set[str] = set()
    for checkpoint in checkpoints:
        checkpoint_id = str(checkpoint.get("checkpoint_id", "<checkpoint>"))
        if checkpoint_id in seen:
            findings.append(Finding("error", "checkpoint.duplicate_id", "checkpoint ids must be unique within a row", entry_id))
        seen.add(checkpoint_id)
        for field in ("checkpoint_id", "title", "restore_point_ref"):
            if not is_str(checkpoint.get(field)):
                findings.append(Finding("error", "checkpoint.empty_field", f"checkpoint {field} must be non-empty", entry_id))
        if not isinstance(checkpoint.get("verified"), bool):
            findings.append(Finding("error", "checkpoint.verified_invalid", "checkpoint verified must be a boolean", entry_id))
    if unverified_checkpoint_count(row) > 0 and "rollback_checkpoint_unverified" not in reasons_of(row):
        findings.append(Finding("error", "checkpoint.unverified_without_reason", "an unverified checkpoint must name the rollback_checkpoint_unverified reason", entry_id))
    return findings


def validate_row(row: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings: list[Finding] = []
    entry_id = str(row.get("entry_id", "<row>"))

    for field in ("entry_id", "title", "subject_ref", "subject_summary", "claim_ref", "rationale"):
        if not is_str(row.get(field)):
            findings.append(Finding("error", "row.empty_field", f"row {field} must be non-empty", entry_id))

    if row.get("stage_kind") not in STAGE_KINDS:
        findings.append(Finding("error", "row.stage_kind_invalid", "stage_kind is outside the vocabulary", entry_id))
    if not isinstance(row.get("release_blocking"), bool):
        findings.append(Finding("error", "row.release_blocking_invalid", "release_blocking must be a boolean", entry_id))

    claim_label = row.get("claim_label")
    effective = row.get("effective_label")
    state = row.get("rehearsal_state")
    reasons = reasons_of(row)

    if claim_label not in LIFECYCLE_LABELS:
        findings.append(Finding("error", "row.claim_label_invalid", "claim_label is invalid", entry_id))
    if effective not in LIFECYCLE_LABELS:
        findings.append(Finding("error", "row.effective_invalid", "effective_label is invalid", entry_id))
    if state not in REHEARSAL_STATES:
        findings.append(Finding("error", "row.state_invalid", "rehearsal_state is invalid", entry_id))
    if any(reason not in GAP_REASONS for reason in reasons):
        findings.append(Finding("error", "row.reason_invalid", "active_gap_reasons has an unknown reason", entry_id))

    # The ceiling: no row may render a label wider than the public claim's label.
    if claim_label in LEVEL_RANK and effective in LEVEL_RANK and LEVEL_RANK[effective] > LEVEL_RANK[claim_label]:
        findings.append(Finding("error", "row.effective_wider_than_claim", "effective_label is wider than the claim ceiling", entry_id))

    packet = packet_of(row)
    findings.extend(validate_packet_block(entry_id, packet))
    findings.extend(validate_checkpoints(entry_id, row))
    slo_state = packet.get("slo_state")

    holding = state in HOLDING_STATES
    claim_holds = is_above_cutline(claim_label)

    # A claim canonically below the cutline forces the row to inherit the ceiling and narrow.
    if claim_label in LEVEL_RANK and not claim_holds:
        if holding:
            findings.append(Finding("error", "row.go_on_narrowed_claim", "a row returns Go while the public claim label is below the cutline", entry_id))
        if "claim_label_narrowed" not in reasons:
            findings.append(Finding("error", "row.claim_narrowed_without_reason", "a row whose claim is narrowed must name claim_label_narrowed", entry_id))

    if holding:
        if claim_label != effective:
            findings.append(Finding("error", "row.go_label_not_equal_claim", "a Go row must render the public claim's canonical label", entry_id))
        if reasons:
            findings.append(Finding("error", "row.go_with_active_gap", "a Go row must carry no active gap reason", entry_id))
        if not has_capture(packet):
            findings.append(Finding("error", "row.go_without_fresh_packet", "a Go row must ride a captured, evidence-backed packet", entry_id))
        if slo_state in ("breached", "missing"):
            findings.append(Finding("error", "row.go_on_stale_packet", "a Go row must ride a packet within its freshness SLO", entry_id))
        if not all_checkpoints_verified(row):
            findings.append(Finding("error", "row.go_with_unverified_checkpoint", "a Go row must verify every required rollback checkpoint", entry_id))
        if not owner_signed(row):
            findings.append(Finding("error", "row.go_without_owner_signoff", "a Go row must carry an owner sign-off with a date", entry_id))
    else:
        if is_above_cutline(effective):
            findings.append(Finding("error", "row.effective_not_narrowed", "a row that is not a Go must narrow below the cutline", entry_id))
        if not reasons:
            findings.append(Finding("error", "row.narrowing_without_reason", "a narrowing row must name an active gap reason", entry_id))
        if slo_state == "breached" and "rehearsal_packet_freshness_breached" not in reasons:
            findings.append(Finding("error", "row.breached_without_reason", "a breached packet must name rehearsal_packet_freshness_breached", entry_id))
        if slo_state == "missing" and "rehearsal_packet_missing" not in reasons:
            findings.append(Finding("error", "row.missing_without_reason", "a missing packet must name rehearsal_packet_missing", entry_id))

    signoff = row.get("owner_signoff", {})
    if not (isinstance(signoff, dict) and is_str(signoff.get("owner_ref"))):
        findings.append(Finding("error", "row.signoff_owner_missing", "owner_signoff.owner_ref must be non-empty", entry_id))

    findings.extend(validate_state_reason_coherence(entry_id, state, reasons, row))
    findings.extend(validate_row_refs(entry_id, row, repo_root))
    return findings


def validate_state_reason_coherence(entry_id: str, state: Any, reasons: list[str], row: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    if state == "no_go_unrehearsed":
        allowed = {"rehearsal_evidence_incomplete", "owner_signoff_missing", "rollback_checkpoint_unverified"}
        if not allowed.intersection(reasons):
            findings.append(Finding("error", "row.state_reason_incoherent", "no_go_unrehearsed requires an evidence/signoff/checkpoint reason", entry_id))
    if state == "no_go_claim_narrowed" and "claim_label_narrowed" not in reasons:
        findings.append(Finding("error", "row.state_reason_incoherent", "no_go_claim_narrowed requires the claim_label_narrowed reason", entry_id))
    if state == "no_go_stale" and not ({"rehearsal_packet_freshness_breached", "rehearsal_packet_missing"} & set(reasons)):
        findings.append(Finding("error", "row.state_reason_incoherent", "no_go_stale requires a packet freshness/missing reason", entry_id))
    if state == "no_go_exception_expired":
        if "exception_expired" not in reasons:
            findings.append(Finding("error", "row.state_reason_incoherent", "no_go_exception_expired requires the exception_expired reason", entry_id))
        if not isinstance(row.get("exception_packet"), dict):
            findings.append(Finding("error", "row.exception_state_without_packet", "no_go_exception_expired state must name an exception packet", entry_id))
    if state == "go_on_exception":
        packet = row.get("exception_packet")
        if not isinstance(packet, dict) or not is_str(packet.get("waiver_ref")) or not is_str(packet.get("expires_at")):
            findings.append(Finding("error", "row.exception_state_without_packet", "go_on_exception state must name an exception packet", entry_id))
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


def validate_coverage(rehearsal: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    rows = rehearsal.get("rows", [])

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

    declared = rehearsal.get("release_blocking_stage_refs", [])
    declared_set = {r for r in declared if is_str(r)} if isinstance(declared, list) else set()
    covered = {
        str(row.get("entry_id"))
        for row in rows
        if row.get("release_blocking") is True and is_str(row.get("entry_id"))
    }
    for ref in sorted(declared_set):
        if ref not in covered:
            findings.append(Finding("error", "coverage.release_blocking_ref_without_row", f"declared release-blocking stage has no row: {ref}", ref))
    for row in rows:
        if row.get("release_blocking") is True and is_str(row.get("entry_id")) and row["entry_id"] not in declared_set:
            findings.append(Finding("error", "coverage.release_blocking_not_declared", "a release-blocking row is not in release_blocking_stage_refs", str(row.get("entry_id"))))

    present_kinds = {row.get("stage_kind") for row in rows}
    for kind in STAGE_KINDS:
        if kind not in present_kinds:
            findings.append(Finding("error", "coverage.stage_kind_absent", f"stage kind has no row: {kind}", kind))
    return findings


def validate_freshness(rehearsal: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    as_of = parse_date(rehearsal.get("as_of"))
    if as_of is None:
        return findings
    for row in rehearsal.get("rows", []):
        entry_id = str(row.get("entry_id", "<row>"))
        pkt = packet_of(row)
        declared = pkt.get("slo_state")
        computed = computed_slo_state(pkt, as_of)
        if computed is None or declared not in FRESHNESS_RANK or computed not in FRESHNESS_RANK:
            continue
        if FRESHNESS_RANK[declared] > FRESHNESS_RANK[computed]:
            findings.append(Finding("error", "row.packet_freshness_overstated", f"declared slo_state {declared!r} is fresher than the {computed!r} the as_of date allows", entry_id))
        if row.get("rehearsal_state") in HOLDING_STATES and computed in ("breached", "missing"):
            findings.append(Finding("error", "row.held_on_breached_packet", "a Go row rides a packet past its SLO against as_of", entry_id))
    return findings


def validate_exceptions(rehearsal: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    as_of = parse_date(rehearsal.get("as_of"))
    if as_of is None:
        return findings
    for row in rehearsal.get("rows", []):
        entry_id = str(row.get("entry_id", "<row>"))
        state = row.get("rehearsal_state")
        packet = row.get("exception_packet")
        if not isinstance(packet, dict):
            continue
        expires = parse_date(packet.get("expires_at"))
        if expires is None:
            findings.append(Finding("error", "row.exception_expiry_invalid", "exception_packet expires_at must be an ISO date", entry_id))
            continue
        expired = as_of >= expires
        if expired and state == "go_on_exception":
            findings.append(Finding("error", "row.held_on_expired_exception", "a row returns Go on an exception packet that has expired against as_of", entry_id))
        if not expired and state == "no_go_exception_expired":
            findings.append(Finding("error", "row.exception_expired_but_active", "a row is marked exception-expired but the exception packet is still active against as_of", entry_id))
    return findings


def validate_ceiling(rehearsal: dict[str, Any], repo_root: Path) -> list[Finding]:
    """Cross-artifact ingestion: read the stable claim manifest and fail when a row's
    claim_label is not the label the claim manifest publishes for the entry named by
    claim_ref, or names an entry the claim manifest does not carry."""
    findings: list[Finding] = []
    ref = rehearsal.get("claim_manifest_ref")
    if not is_str(ref):
        return findings
    path = repo_root / ref.split("#", 1)[0]
    if not path.exists():
        findings.append(Finding("error", "ceiling.artifact_missing", f"claim_manifest_ref does not exist: {ref}", "<rehearsal>"))
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

    for row in rehearsal.get("rows", []):
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


def validate_publication(rehearsal: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    publication = rehearsal.get("publication")
    if not isinstance(publication, dict):
        findings.append(Finding("error", "publication.missing", "rehearsal must carry a publication block", "<rehearsal>"))
        return findings
    for field in ("publication_gate", "rationale"):
        if not is_str(publication.get(field)):
            findings.append(Finding("error", "publication.empty_field", f"publication.{field} must be non-empty", "<rehearsal>"))
    decision, rule_ids, entry_ids = computed_publication(rehearsal)
    if publication.get("decision") != decision:
        findings.append(Finding("error", "publication.decision_inconsistent", f"declared decision {publication.get('decision')!r} disagrees with computed {decision!r}", "<rehearsal>"))
    if list(publication.get("blocking_rule_ids", [])) != rule_ids:
        findings.append(Finding("error", "publication.blocking_rules_mismatch", "blocking_rule_ids disagrees with the firing rules", "<rehearsal>"))
    if list(publication.get("blocking_entry_ids", [])) != entry_ids:
        findings.append(Finding("error", "publication.blocking_entries_mismatch", "blocking_entry_ids disagrees with the firing rules", "<rehearsal>"))
    return findings


def validate_summary(rehearsal: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    summary = rehearsal.get("summary")
    if not isinstance(summary, dict):
        findings.append(Finding("error", "summary.missing", "rehearsal must carry a summary block", "<rehearsal>"))
        return findings
    expected = computed_summary(rehearsal)
    for key, value in expected.items():
        if summary.get(key) != value:
            findings.append(Finding("error", "summary.count_mismatch", f"summary.{key} must equal the recomputed value", key))
    return findings


def validate_rehearsal(rehearsal: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings = validate_envelope(rehearsal, repo_root)
    findings.extend(validate_rules(rehearsal))

    rows = rehearsal.get("rows")
    if not isinstance(rows, list) or not rows:
        findings.append(Finding("error", "rehearsal.rows_empty", "rehearsal must enumerate at least one row", "<rehearsal>"))
        return findings

    for raw in rows:
        row = ensure_dict(raw, "rehearsal.rows[]")
        findings.extend(validate_row(row, repo_root))

    findings.extend(validate_coverage(rehearsal))
    findings.extend(validate_freshness(rehearsal))
    findings.extend(validate_exceptions(rehearsal))
    findings.extend(validate_ceiling(rehearsal, repo_root))
    findings.extend(validate_publication(rehearsal))
    findings.extend(validate_summary(rehearsal))
    return findings


def recompute_derived(rehearsal: dict[str, Any]) -> None:
    rehearsal["summary"] = computed_summary(rehearsal)
    decision, rule_ids, entry_ids = computed_publication(rehearsal)
    if isinstance(rehearsal.get("publication"), dict):
        rehearsal["publication"].update(
            {"decision": decision, "blocking_rule_ids": rule_ids, "blocking_entry_ids": entry_ids}
        )


def run_negative_drills(rehearsal: dict[str, Any], repo_root: Path) -> tuple[list[dict[str, str]], list[Finding]]:
    results: list[dict[str, str]] = []
    findings: list[Finding] = []

    def record(drill_id: str, expected_check_id: str, fired: bool) -> None:
        results.append({"drill_id": drill_id, "expected_check_id": expected_check_id, "status": "passed" if fired else "failed"})
        if not fired:
            findings.append(Finding("error", "negative_drill.not_rejected", f"negative drill {drill_id} did not fire", drill_id))

    def check_ids(candidate: dict[str, Any]) -> set[str]:
        return {f.check_id for f in validate_rehearsal(candidate, repo_root)}

    # A Go row rendered with a breached packet against as_of must be flagged.
    mutated = copy.deepcopy(rehearsal)
    target = next((r for r in mutated["rows"] if r.get("rehearsal_state") in HOLDING_STATES), None)
    if target is not None:
        target["rehearsal_packet"]["captured_at"] = "2000-01-01"
        record("held_on_breached_packet_rejected", "row.held_on_breached_packet", "row.held_on_breached_packet" in check_ids(mutated))

    # A narrowing row that still renders a stable label must be flagged.
    mutated = copy.deepcopy(rehearsal)
    target = next((r for r in mutated["rows"] if r.get("rehearsal_state") in NARROWING_STATES), None)
    if target is not None:
        target["effective_label"] = "stable"
        recompute_derived(mutated)
        record("effective_not_narrowed_rejected", "row.effective_not_narrowed", "row.effective_not_narrowed" in check_ids(mutated))

    # A Go row with an unverified rollback checkpoint must be flagged.
    mutated = copy.deepcopy(rehearsal)
    target = next((r for r in mutated["rows"] if r.get("rehearsal_state") in HOLDING_STATES), None)
    if target is not None and target.get("rollback_checkpoints"):
        target["rollback_checkpoints"][0]["verified"] = False
        record("go_with_unverified_checkpoint_rejected", "row.go_with_unverified_checkpoint", "row.go_with_unverified_checkpoint" in check_ids(mutated))

    # A Go row with a missing owner sign-off must be flagged.
    mutated = copy.deepcopy(rehearsal)
    target = next((r for r in mutated["rows"] if r.get("rehearsal_state") in HOLDING_STATES), None)
    if target is not None:
        target["owner_signoff"]["signed_off"] = False
        target["owner_signoff"]["signed_at"] = None
        record("go_without_owner_signoff_rejected", "row.go_without_owner_signoff", "row.go_without_owner_signoff" in check_ids(mutated))

    # A row whose claim_label disagrees with the claim manifest must be flagged.
    mutated = copy.deepcopy(rehearsal)
    target = next((r for r in mutated["rows"] if r.get("claim_label") == "beta"), None)
    if target is not None:
        target["claim_label"] = "stable"
        record("claim_label_mismatch_rejected", "ceiling.claim_label_mismatch", "ceiling.claim_label_mismatch" in check_ids(mutated))

    # Removing every row of a stage kind must be flagged.
    mutated = copy.deepcopy(rehearsal)
    mutated["rows"] = [r for r in mutated["rows"] if r.get("stage_kind") != "cutline_review"]
    recompute_derived(mutated)
    record("stage_kind_absent_rejected", "coverage.stage_kind_absent", "coverage.stage_kind_absent" in check_ids(mutated))

    # Dropping a declared release-blocking row must be flagged.
    mutated = copy.deepcopy(rehearsal)
    declared = mutated.get("release_blocking_stage_refs", [])
    if declared:
        dropped = declared[0]
        mutated["rows"] = [r for r in mutated["rows"] if r.get("entry_id") != dropped]
        recompute_derived(mutated)
        record("release_blocking_uncovered_rejected", "coverage.release_blocking_ref_without_row", "coverage.release_blocking_ref_without_row" in check_ids(mutated))

    # A go/no-go decision that disagrees with the firing rules must be flagged.
    mutated = copy.deepcopy(rehearsal)
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
        ids = {f.check_id for f in validate_rehearsal(ensure_dict(candidate, "fixture"), repo_root)}
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
    rehearsal: dict[str, Any],
    findings: list[Finding],
    drill_results: list[dict[str, str]],
    fixture_results: list[dict[str, str]],
) -> None:
    decision, rule_ids, entry_ids = computed_publication(rehearsal)
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "record_kind": "go_no_go_rehearsal_validation_capture",
        "status": "pass" if not any(f.severity == "error" for f in findings) else "fail",
        "generated_at": generated_at_now(),
        "rehearsal_id": rehearsal.get("rehearsal_id"),
        "as_of": rehearsal.get("as_of"),
        "summary": rehearsal.get("summary"),
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

    rehearsal = ensure_dict(
        load_json(repo_root / args.rehearsal, "go/no-go rehearsal"),
        "go/no-go rehearsal",
    )

    findings = validate_rehearsal(rehearsal, repo_root)
    drill_results, drill_findings = run_negative_drills(rehearsal, repo_root)
    findings.extend(drill_findings)
    fixture_results, fixture_findings = run_fixture_cases(repo_root, args.fixtures_dir)
    findings.extend(fixture_findings)

    report_rel = args.report
    if report_rel is None and not args.no_capture:
        report_rel = DEFAULT_REPORT_REL
    if report_rel:
        write_report(repo_root / report_rel, rehearsal, findings, drill_results, fixture_results)

    errors = [f for f in findings if f.severity == "error"]
    if errors:
        for item in errors:
            print(f"ERROR [{item.check_id}] ({item.ref}): {item.message}", file=sys.stderr)
        return 1

    summary = rehearsal.get("summary", {})
    decision, rule_ids, _entry_ids = computed_publication(rehearsal)
    print(
        "go/no-go rehearsal validated "
        f"({summary.get('total_rows')} rows across {summary.get('total_claims')} claims, "
        f"{summary.get('rows_go')} go, "
        f"{summary.get('rows_no_go_below_cutline')} no-go; "
        f"release-blocking {summary.get('release_blocking_go')}/{summary.get('release_blocking_total')} go; "
        f"packets {summary.get('packets_current')} current / "
        f"{summary.get('packets_due_for_refresh')} due / "
        f"{summary.get('packets_breached')} breached / "
        f"{summary.get('packets_missing')} missing; "
        f"checkpoints {summary.get('checkpoints_verified')}/{summary.get('total_checkpoints')} verified; "
        f"{summary.get('rules_firing')} rules firing; "
        f"go_no_go={decision}; {len(drill_results)} negative drills, {len(fixture_results)} fixture cases)"
    )

    if args.require_proceed and decision == "hold":
        print(
            f"GO/NO-GO HELD: go/no-go rehearsal promotion blocked by {len(rule_ids)} rule(s): {', '.join(rule_ids)}",
            file=sys.stderr,
        )
        return 2

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
