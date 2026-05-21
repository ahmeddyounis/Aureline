#!/usr/bin/env python3
"""Validate the stable proof index linking launch-blocking requirements, proof
packets, waivers, and public claims.

The stable claim manifest decides the single canonical lifecycle label each
*subject* publishes; this index answers the requirement-level question the launch
shiproom asks: is each launch-blocking requirement proven, and which public claim
does that proof back? It reads the checked-in index at
``artifacts/release/stable_proof_index.json`` and:

  - asserts the closed vocabularies (lifecycle labels, proof states, gap reasons,
    index actions) and the launch cutline are canonical;
  - asserts every row that is narrowed (for an absent/incomplete requirement, a
    breached/missing proof packet, an expired waiver, a missing owner sign-off, or
    a public claim whose canonical label is itself below the cutline) drops below
    the cutline rather than backing a label wider than the public claim, and that a
    proven row backs the public claim's canonical label cleanly with a captured,
    within-SLO proof packet and an owner sign-off;
  - asserts the launch-blocking requirement set is fully covered: every declared
    launch-blocking requirement ref has exactly one covering launch-blocking row,
    every launch-blocking row is declared, and no requirement ref repeats;
  - performs the ceiling cross-check the typed model cannot — it reads the stable
    claim manifest named by ``claim_manifest_ref`` and fails when a row's
    ``claim_label`` is not the label the claim manifest publishes for the entry
    named by ``claim_ref``, or names an entry the claim manifest does not carry;
  - performs the packet-freshness SLO automation the typed model cannot — against
    the index ``as_of`` date it recomputes each packet's SLO state from its
    ``captured_at`` and ``freshness_slo`` and fails when a declared state is fresher
    than the clock allows or when a proven row rides a packet whose recomputed state
    is outside its SLO;
  - performs the waiver-expiry date arithmetic;
  - recomputes the publication verdict and the summary block, and fails on any
    drift;
  - runs negative drills proving the narrowing, ceiling, freshness, waiver, and
    publication rejections all fire; and
  - runs the checked-in fixture cases under
    ``fixtures/release/stable_proof_index`` and fails when a case the index marks as
    rejected validates clean.

With ``--require-proceed`` the gate additionally fails (exit code 2) when the
recomputed publication verdict is ``hold``, so shiproom and release tooling can
block stable proof-index publication directly from this artifact.

The typed Rust consumer
(``aureline_release::stable_proof_index::current_stable_proof_index``) reads the
same index and runs the same structural cross-check, so this gate and
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


DEFAULT_INDEX_REL = "artifacts/release/stable_proof_index.json"
DEFAULT_REPORT_REL = (
    "artifacts/release/captures/stable_proof_index_validation_capture.json"
)
DEFAULT_FIXTURES_REL = "fixtures/release/stable_proof_index"

EXPECTED_SCHEMA_VERSION = 1
INDEX_RECORD_KIND = "stable_proof_index"

LIFECYCLE_LABELS = ("lts", "stable", "beta", "preview", "withdrawn")
ABOVE_CUTLINE = ("lts", "stable")
BELOW_CUTLINE = ("beta", "preview", "withdrawn")
FRESHNESS_SLO_STATES = ("current", "due_for_refresh", "breached", "missing")
PROOF_STATES = (
    "proven",
    "proven_on_waiver",
    "unproven_unbacked",
    "unproven_claim_narrowed",
    "unproven_stale",
    "unproven_waiver_expired",
)
GAP_REASONS = (
    "claim_label_narrowed",
    "requirement_capability_absent",
    "requirement_evidence_incomplete",
    "proof_packet_freshness_breached",
    "proof_packet_missing",
    "waiver_expired",
    "owner_signoff_missing",
)
INDEX_ACTIONS = (
    "hold_publication",
    "narrow_claim_label",
    "refresh_proof_packet",
    "recapture_requirement_evidence",
    "request_owner_signoff",
)
HOLDING_STATES = ("proven", "proven_on_waiver")
NARROWING_STATES = (
    "unproven_unbacked",
    "unproven_claim_narrowed",
    "unproven_stale",
    "unproven_waiver_expired",
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
    parser.add_argument("--index", default=DEFAULT_INDEX_REL)
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


def proof_rule_fires(rule: dict[str, Any], rows: list[dict[str, Any]]) -> bool:
    trigger = rule.get("trigger_reason")
    labels = rule.get("applies_to_labels", [])
    if not isinstance(labels, list):
        return False
    for row in rows:
        if row.get("claim_label") in labels and trigger in reasons_of(row):
            return True
    return False


def computed_publication(index: dict[str, Any]) -> tuple[str, list[str], list[str]]:
    rows = index.get("rows", [])
    rules = index.get("proof_rules", [])
    blocking_rules = [
        rule
        for rule in rules
        if rule.get("blocks_publication") is True and proof_rule_fires(rule, rows)
    ]
    rule_ids = sorted(str(rule.get("rule_id")) for rule in blocking_rules)
    blocking_triggers = {rule.get("trigger_reason") for rule in blocking_rules}
    proof_ids: set[str] = set()
    for row in rows:
        if is_above_cutline(row.get("claim_label")) and blocking_triggers.intersection(
            reasons_of(row)
        ):
            proof_ids.add(str(row.get("proof_id")))
    decision = "hold" if blocking_rules else "proceed"
    return decision, rule_ids, sorted(proof_ids)


def computed_summary(index: dict[str, Any]) -> dict[str, Any]:
    rows = index.get("rows", [])
    rules = index.get("proof_rules", [])

    def packet_state(row: dict[str, Any]) -> Any:
        packet = row.get("proof_packet")
        return packet.get("slo_state") if isinstance(packet, dict) else None

    proven = [r for r in rows if is_above_cutline(r.get("proven_label"))]
    claims = {str(r.get("claim_ref")) for r in rows}
    launch_blocking = [r for r in rows if r.get("launch_blocking") is True]
    lb_proven = [r for r in launch_blocking if is_above_cutline(r.get("proven_label"))]
    return {
        "total_requirements": len(rows),
        "total_claims": len(claims),
        "requirements_proven_stable": len(proven),
        "requirements_narrowed_below_cutline": len(rows) - len(proven),
        "requirements_on_active_waiver": sum(
            1 for r in rows if r.get("index_state") == "proven_on_waiver"
        ),
        "launch_blocking_total": len(launch_blocking),
        "launch_blocking_proven_stable": len(lb_proven),
        "launch_blocking_unproven": len(launch_blocking) - len(lb_proven),
        "packets_current": sum(1 for r in rows if packet_state(r) == "current"),
        "packets_due_for_refresh": sum(1 for r in rows if packet_state(r) == "due_for_refresh"),
        "packets_breached": sum(1 for r in rows if packet_state(r) == "breached"),
        "packets_missing": sum(1 for r in rows if packet_state(r) == "missing"),
        "total_active_gap_reasons": sum(len(reasons_of(r)) for r in rows),
        "proof_rules_firing": sum(
            1 for rule in rules if proof_rule_fires(rule, rows)
        ),
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


def validate_envelope(index: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    index_id = str(index.get("index_id", "<index>"))

    if index.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(Finding("error", "index.schema_version", "schema_version must be 1", index_id))
    if index.get("record_kind") != INDEX_RECORD_KIND:
        findings.append(Finding("error", "index.record_kind", "record_kind is not supported", index_id))
    for field in ("index_id", "status", "overview_page", "as_of", "claim_manifest_ref"):
        if not is_str(index.get(field)):
            findings.append(
                Finding("error", "index.empty_field", f"{field} must be a non-empty string", index_id)
            )
    if parse_date(index.get("as_of")) is None:
        findings.append(Finding("error", "index.as_of_invalid", "as_of must be an ISO date", index_id))

    for key, expected in (
        ("lifecycle_labels", list(LIFECYCLE_LABELS)),
        ("proof_states", list(PROOF_STATES)),
        ("gap_reasons", list(GAP_REASONS)),
        ("index_actions", list(INDEX_ACTIONS)),
    ):
        if list(index.get(key, [])) != expected:
            findings.append(
                Finding("error", "index.vocabulary", f"index.{key} is not the closed vocabulary", key)
            )

    refs = index.get("launch_blocking_requirement_refs")
    if not isinstance(refs, list) or not refs or any(not is_str(r) for r in refs):
        findings.append(
            Finding(
                "error",
                "index.launch_blocking_refs",
                "launch_blocking_requirement_refs must be a non-empty list of non-empty strings",
                index_id,
            )
        )

    cutline = index.get("launch_cutline")
    if not isinstance(cutline, dict):
        findings.append(Finding("error", "cutline.missing", "index must carry a launch_cutline", index_id))
    else:
        if cutline.get("cutline_level") != "stable":
            findings.append(Finding("error", "cutline.level", "cutline_level must be stable", index_id))
        if list(cutline.get("above_cutline_levels", [])) != list(ABOVE_CUTLINE):
            findings.append(Finding("error", "cutline.above", "above_cutline_levels is not canonical", index_id))
        if list(cutline.get("below_cutline_levels", [])) != list(BELOW_CUTLINE):
            findings.append(Finding("error", "cutline.below", "below_cutline_levels is not canonical", index_id))
        if not is_str(cutline.get("description")):
            findings.append(Finding("error", "cutline.description", "cutline description must be non-empty", index_id))
    return findings


def validate_rules(index: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    rules = index.get("proof_rules")
    if not isinstance(rules, list) or not rules:
        findings.append(Finding("error", "rules.empty", "index must enumerate at least one proof rule", "<index>"))
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
        if rule.get("default_action") not in INDEX_ACTIONS:
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


def validate_packet(proof_id: str, packet: Any) -> list[Finding]:
    findings: list[Finding] = []
    if not isinstance(packet, dict):
        findings.append(Finding("error", "row.packet_missing", "row must carry a proof_packet block", proof_id))
        return findings
    for field in ("packet_id", "packet_ref", "proof_index_ref"):
        if not is_str(packet.get(field)):
            findings.append(Finding("error", "packet.empty_field", f"proof_packet.{field} must be non-empty", proof_id))
    if packet.get("slo_state") not in FRESHNESS_SLO_STATES:
        findings.append(Finding("error", "packet.state_invalid", "slo_state is outside the vocabulary", proof_id))
    slo = packet.get("freshness_slo")
    if not isinstance(slo, dict):
        findings.append(Finding("error", "packet.slo_missing", "proof_packet must carry a freshness_slo block", proof_id))
        return findings
    if not is_str(slo.get("slo_register_ref")):
        findings.append(Finding("error", "packet.slo_register_missing", "freshness_slo.slo_register_ref must be non-empty", proof_id))
    target = slo.get("target_max_age_days")
    warn = slo.get("warn_within_days")
    if not (isinstance(target, int) and not isinstance(target, bool) and target >= 1):
        findings.append(Finding("error", "packet.target_invalid", "target_max_age_days must be an integer >= 1", proof_id))
    if not (isinstance(warn, int) and not isinstance(warn, bool) and warn >= 0):
        findings.append(Finding("error", "packet.warn_invalid", "warn_within_days must be an integer >= 0", proof_id))
    if (
        isinstance(target, int)
        and not isinstance(target, bool)
        and isinstance(warn, int)
        and not isinstance(warn, bool)
        and warn > target
    ):
        findings.append(Finding("error", "packet.slo_inconsistent", "warn_within_days may not exceed target_max_age_days", proof_id))
    return findings


def validate_row(row: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings: list[Finding] = []
    proof_id = str(row.get("proof_id", "<row>"))

    for field in (
        "proof_id",
        "title",
        "requirement_ref",
        "requirement_class",
        "requirement_summary",
        "claim_ref",
        "rationale",
    ):
        if not is_str(row.get(field)):
            findings.append(Finding("error", "row.empty_field", f"row {field} must be non-empty", proof_id))

    if not isinstance(row.get("launch_blocking"), bool):
        findings.append(Finding("error", "row.launch_blocking_invalid", "launch_blocking must be a boolean", proof_id))

    claim_label = row.get("claim_label")
    proven = row.get("proven_label")
    state = row.get("index_state")
    reasons = reasons_of(row)

    if claim_label not in LIFECYCLE_LABELS:
        findings.append(Finding("error", "row.claim_label_invalid", "claim_label is invalid", proof_id))
    if proven not in LIFECYCLE_LABELS:
        findings.append(Finding("error", "row.proven_invalid", "proven_label is invalid", proof_id))
    if state not in PROOF_STATES:
        findings.append(Finding("error", "row.state_invalid", "index_state is invalid", proof_id))
    if any(reason not in GAP_REASONS for reason in reasons):
        findings.append(Finding("error", "row.reason_invalid", "active_gap_reasons has an unknown reason", proof_id))

    # The ceiling: no proof may back a label wider than the public claim's label.
    if claim_label in LEVEL_RANK and proven in LEVEL_RANK and LEVEL_RANK[proven] > LEVEL_RANK[claim_label]:
        findings.append(Finding("error", "row.proven_wider_than_claim", "proven_label is wider than the claim ceiling", proof_id))

    packet = row.get("proof_packet")
    findings.extend(validate_packet(proof_id, packet))
    slo_state = packet.get("slo_state") if isinstance(packet, dict) else None

    holds = state in HOLDING_STATES
    claim_holds = is_above_cutline(claim_label)

    # A claim canonically below the cutline forces the proof to inherit the ceiling
    # and narrow.
    if claim_label in LEVEL_RANK and not claim_holds:
        if holds:
            findings.append(Finding("error", "row.held_on_narrowed_claim", "a row holds a proof while the public claim label is below the cutline", proof_id))
        if "claim_label_narrowed" not in reasons:
            findings.append(Finding("error", "row.claim_narrowed_without_reason", "a row whose claim is narrowed must name claim_label_narrowed", proof_id))

    if state in NARROWING_STATES:
        if is_above_cutline(proven):
            findings.append(Finding("error", "row.proven_not_narrowed", "a requirement that is not proven must narrow below the cutline", proof_id))
        if not reasons:
            findings.append(Finding("error", "row.narrowing_without_reason", "a narrowing row must name an active gap reason", proof_id))
        if slo_state == "breached" and "proof_packet_freshness_breached" not in reasons:
            findings.append(Finding("error", "row.breached_without_reason", "a breached packet must name proof_packet_freshness_breached", proof_id))
        if slo_state == "missing" and "proof_packet_missing" not in reasons:
            findings.append(Finding("error", "row.missing_without_reason", "a missing packet must name proof_packet_missing", proof_id))
    if holds:
        if claim_label != proven:
            findings.append(Finding("error", "row.held_label_not_equal_claim", "a proven row must back the public claim's canonical label", proof_id))
        if reasons:
            findings.append(Finding("error", "row.held_with_active_gap", "a proven row must carry no active gap reason", proof_id))
        if isinstance(packet, dict) and not has_capture(packet):
            findings.append(Finding("error", "row.held_without_fresh_packet", "a proven row must ride a captured, evidence-backed proof packet", proof_id))
        if slo_state in ("breached", "missing"):
            findings.append(Finding("error", "row.held_on_stale_packet", "a proven row must ride a packet within its freshness SLO", proof_id))
        signoff = row.get("owner_signoff", {})
        if not (isinstance(signoff, dict) and signoff.get("signed_off") is True and is_str(signoff.get("signed_at"))):
            findings.append(Finding("error", "row.held_without_signoff", "a proven row must carry an owner sign-off with a date", proof_id))
    signoff = row.get("owner_signoff", {})
    if not (isinstance(signoff, dict) and is_str(signoff.get("owner_ref"))):
        findings.append(Finding("error", "row.signoff_owner_missing", "owner_signoff.owner_ref must be non-empty", proof_id))

    findings.extend(validate_state_reason_coherence(proof_id, state, reasons, row))
    findings.extend(validate_row_refs(proof_id, row, repo_root))
    return findings


def validate_state_reason_coherence(proof_id: str, state: Any, reasons: list[str], row: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    if state == "unproven_unbacked":
        allowed = {"requirement_capability_absent", "requirement_evidence_incomplete", "owner_signoff_missing"}
        if not allowed.intersection(reasons):
            findings.append(Finding("error", "row.state_reason_incoherent", "unproven_unbacked requires a capability/evidence/signoff reason", proof_id))
    if state == "unproven_claim_narrowed" and "claim_label_narrowed" not in reasons:
        findings.append(Finding("error", "row.state_reason_incoherent", "unproven_claim_narrowed requires the claim_label_narrowed reason", proof_id))
    if state == "unproven_stale" and not (
        "proof_packet_freshness_breached" in reasons or "proof_packet_missing" in reasons
    ):
        findings.append(Finding("error", "row.state_reason_incoherent", "unproven_stale requires a packet-freshness reason", proof_id))
    if state == "unproven_waiver_expired":
        if "waiver_expired" not in reasons:
            findings.append(Finding("error", "row.state_reason_incoherent", "unproven_waiver_expired requires the waiver_expired reason", proof_id))
        if not isinstance(row.get("waiver"), dict):
            findings.append(Finding("error", "row.waiver_state_without_waiver", "unproven_waiver_expired state must name a waiver", proof_id))
    if state == "proven_on_waiver":
        waiver = row.get("waiver")
        if not isinstance(waiver, dict) or not is_str(waiver.get("waiver_ref")) or not is_str(waiver.get("expires_at")):
            findings.append(Finding("error", "row.waiver_state_without_waiver", "proven_on_waiver state must name a waiver", proof_id))
    return findings


def validate_row_refs(proof_id: str, row: dict[str, Any], repo_root: Path) -> list[Finding]:
    """Path-existence checks for the proof-packet refs (not the cross-artifact ids,
    which resolve against the claim manifest)."""
    findings: list[Finding] = []
    packet = row.get("proof_packet")
    if not isinstance(packet, dict):
        return findings
    refs: list[str] = []
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
            findings.append(Finding("error", "row.ref_missing", f"referenced artifact does not exist: {ref}", proof_id))
    return findings


def validate_coverage(index: dict[str, Any]) -> list[Finding]:
    """Every declared launch-blocking requirement is covered by exactly one
    launch-blocking row, every launch-blocking row is declared, and no requirement
    ref repeats."""
    findings: list[Finding] = []
    rows = index.get("rows", [])

    seen: set[str] = set()
    for row in rows:
        ref = row.get("requirement_ref")
        if not is_str(ref):
            continue
        if ref in seen:
            findings.append(Finding("error", "coverage.requirement_duplicate", f"requirement ref appears more than once: {ref}", ref))
        seen.add(ref)

    declared = index.get("launch_blocking_requirement_refs", [])
    declared_set = {r for r in declared if is_str(r)} if isinstance(declared, list) else set()
    covered = {
        str(row.get("requirement_ref"))
        for row in rows
        if row.get("launch_blocking") is True and is_str(row.get("requirement_ref"))
    }
    for ref in sorted(declared_set):
        if ref not in covered:
            findings.append(Finding("error", "coverage.launch_blocking_uncovered", f"declared launch-blocking requirement has no covering row: {ref}", ref))
    for row in rows:
        if row.get("launch_blocking") is True and is_str(row.get("requirement_ref")) and row["requirement_ref"] not in declared_set:
            findings.append(Finding("error", "coverage.launch_blocking_not_declared", "a launch-blocking row's requirement is not in launch_blocking_requirement_refs", str(row.get("proof_id"))))
    return findings


def validate_freshness(index: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    as_of = parse_date(index.get("as_of"))
    if as_of is None:
        return findings
    for row in index.get("rows", []):
        proof_id = str(row.get("proof_id", "<row>"))
        packet = row.get("proof_packet")
        if not isinstance(packet, dict):
            continue
        declared = packet.get("slo_state")
        computed = computed_slo_state(packet, as_of)
        if computed is None or declared not in FRESHNESS_RANK or computed not in FRESHNESS_RANK:
            continue
        if FRESHNESS_RANK[declared] > FRESHNESS_RANK[computed]:
            findings.append(
                Finding("error", "row.packet_freshness_overstated", f"declared slo_state {declared!r} is fresher than the {computed!r} the as_of date allows", proof_id)
            )
        if row.get("index_state") in HOLDING_STATES and computed in ("breached", "missing"):
            findings.append(
                Finding("error", "row.proven_on_breached_packet", "a proven requirement rides a proof packet past its freshness SLO against as_of", proof_id)
            )
    return findings


def validate_waivers(index: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    as_of = parse_date(index.get("as_of"))
    if as_of is None:
        return findings
    for row in index.get("rows", []):
        proof_id = str(row.get("proof_id", "<row>"))
        state = row.get("index_state")
        waiver = row.get("waiver")
        if not isinstance(waiver, dict):
            continue
        expires = parse_date(waiver.get("expires_at"))
        if expires is None:
            findings.append(Finding("error", "row.waiver_expiry_invalid", "waiver expires_at must be an ISO date", proof_id))
            continue
        expired = as_of >= expires
        if expired and state == "proven_on_waiver":
            findings.append(Finding("error", "row.proven_on_expired_waiver", "a requirement holds on a waiver that has expired against as_of", proof_id))
        if not expired and state == "unproven_waiver_expired":
            findings.append(Finding("error", "row.waiver_expired_but_active", "a row is marked waiver-expired but the waiver is still active against as_of", proof_id))
    return findings


def validate_ceiling(index: dict[str, Any], repo_root: Path) -> list[Finding]:
    """Cross-artifact ingestion: read the stable claim manifest and fail when a
    row's claim_label is not the label the claim manifest publishes for the entry
    named by claim_ref, or names an entry the claim manifest does not carry."""
    findings: list[Finding] = []
    ref = index.get("claim_manifest_ref")
    if not is_str(ref):
        return findings
    path = repo_root / ref
    if not path.exists():
        findings.append(Finding("error", "ceiling.artifact_missing", f"claim_manifest_ref does not exist: {ref}", "<index>"))
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

    for row in index.get("rows", []):
        proof_id = str(row.get("proof_id", "<row>"))
        claim_ref = row.get("claim_ref")
        if not is_str(claim_ref):
            continue
        if claim_ref not in published_by_entry:
            findings.append(Finding("error", "ceiling.claim_entry_unknown", f"claim_ref is not an entry in the stable claim manifest: {claim_ref}", proof_id))
            continue
        canonical = published_by_entry[claim_ref]
        if row.get("claim_label") != canonical:
            findings.append(Finding("error", "ceiling.claim_label_mismatch", f"claim_label {row.get('claim_label')!r} does not match the claim manifest published_label {canonical!r}", proof_id))
    return findings


def validate_publication(index: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    publication = index.get("publication")
    if not isinstance(publication, dict):
        findings.append(Finding("error", "publication.missing", "index must carry a publication block", "<index>"))
        return findings
    for field in ("publication_gate", "rationale"):
        if not is_str(publication.get(field)):
            findings.append(Finding("error", "publication.empty_field", f"publication.{field} must be non-empty", "<index>"))
    decision, rule_ids, proof_ids = computed_publication(index)
    if publication.get("decision") != decision:
        findings.append(
            Finding("error", "publication.decision_inconsistent", f"declared decision {publication.get('decision')!r} disagrees with computed {decision!r}", "<index>")
        )
    if list(publication.get("blocking_rule_ids", [])) != rule_ids:
        findings.append(Finding("error", "publication.blocking_rules_mismatch", "blocking_rule_ids disagrees with the firing proof rules", "<index>"))
    if list(publication.get("blocking_proof_ids", [])) != proof_ids:
        findings.append(Finding("error", "publication.blocking_proofs_mismatch", "blocking_proof_ids disagrees with the firing proof rules", "<index>"))
    return findings


def validate_summary(index: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    summary = index.get("summary")
    if not isinstance(summary, dict):
        findings.append(Finding("error", "summary.missing", "index must carry a summary block", "<index>"))
        return findings
    expected = computed_summary(index)
    for key, value in expected.items():
        if summary.get(key) != value:
            findings.append(Finding("error", "summary.count_mismatch", f"summary.{key} must equal the recomputed value", key))
    return findings


def validate_index(index: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings = validate_envelope(index)
    findings.extend(validate_rules(index))

    rows = index.get("rows")
    if not isinstance(rows, list) or not rows:
        findings.append(Finding("error", "index.rows_empty", "index must enumerate at least one row", "<index>"))
        return findings

    seen: set[str] = set()
    for raw in rows:
        row = ensure_dict(raw, "index.rows[]")
        findings.extend(validate_row(row, repo_root))
        proof_id = str(row.get("proof_id", "<row>"))
        if proof_id in seen:
            findings.append(Finding("error", "row.duplicate_id", "proof ids must be unique", proof_id))
        seen.add(proof_id)

    findings.extend(validate_coverage(index))
    findings.extend(validate_freshness(index))
    findings.extend(validate_waivers(index))
    findings.extend(validate_ceiling(index, repo_root))
    findings.extend(validate_publication(index))
    findings.extend(validate_summary(index))
    return findings


def recompute_derived(index: dict[str, Any]) -> None:
    index["summary"] = computed_summary(index)
    decision, rule_ids, proof_ids = computed_publication(index)
    if isinstance(index.get("publication"), dict):
        index["publication"].update(
            {"decision": decision, "blocking_rule_ids": rule_ids, "blocking_proof_ids": proof_ids}
        )


def run_negative_drills(index: dict[str, Any], repo_root: Path) -> tuple[list[dict[str, str]], list[Finding]]:
    results: list[dict[str, str]] = []
    findings: list[Finding] = []

    def record(drill_id: str, expected_check_id: str, fired: bool) -> None:
        results.append({"drill_id": drill_id, "expected_check_id": expected_check_id, "status": "passed" if fired else "failed"})
        if not fired:
            findings.append(Finding("error", "negative_drill.not_rejected", f"negative drill {drill_id} did not fire", drill_id))

    def check_ids(candidate: dict[str, Any]) -> set[str]:
        return {f.check_id for f in validate_index(candidate, repo_root)}

    # A narrowing row that still backs a stable label must be flagged.
    mutated = copy.deepcopy(index)
    target = next((r for r in mutated["rows"] if r.get("index_state") in NARROWING_STATES), None)
    if target is not None:
        target["proven_label"] = "stable"
        recompute_derived(mutated)
        record("proven_not_narrowed_rejected", "row.proven_not_narrowed", "row.proven_not_narrowed" in check_ids(mutated))

    # A proven row carrying an active gap reason must be flagged.
    mutated = copy.deepcopy(index)
    target = next((r for r in mutated["rows"] if r.get("index_state") in HOLDING_STATES), None)
    if target is not None:
        target["active_gap_reasons"] = ["requirement_evidence_incomplete"]
        recompute_derived(mutated)
        record("held_with_active_gap_rejected", "row.held_with_active_gap", "row.held_with_active_gap" in check_ids(mutated))

    # A proven row whose packet is breached against as_of must be flagged.
    mutated = copy.deepcopy(index)
    target = next((r for r in mutated["rows"] if r.get("index_state") in HOLDING_STATES), None)
    if target is not None:
        target["proof_packet"]["captured_at"] = "2000-01-01"
        record("proven_on_breached_packet_rejected", "row.proven_on_breached_packet", "row.proven_on_breached_packet" in check_ids(mutated))

    # A packet whose declared state overstates its freshness must be flagged.
    mutated = copy.deepcopy(index)
    target = next(
        (r for r in mutated["rows"] if r.get("proof_packet", {}).get("slo_state") == "current"),
        None,
    )
    if target is not None:
        target["proof_packet"]["captured_at"] = "2000-01-01"
        record("packet_freshness_overstated_rejected", "row.packet_freshness_overstated", "row.packet_freshness_overstated" in check_ids(mutated))

    # A row whose claim_label disagrees with the claim manifest must be flagged.
    mutated = copy.deepcopy(index)
    target = next((r for r in mutated["rows"] if r.get("claim_label") == "beta"), None)
    if target is not None:
        target["claim_label"] = "stable"
        record("claim_label_mismatch_rejected", "ceiling.claim_label_mismatch", "ceiling.claim_label_mismatch" in check_ids(mutated))

    # A proof backed wider than its public claim's ceiling must be flagged.
    mutated = copy.deepcopy(index)
    target = next(
        (r for r in mutated["rows"] if r.get("claim_label") == "beta" and not is_above_cutline(r.get("proven_label"))),
        None,
    )
    if target is not None:
        target["proven_label"] = "lts"
        recompute_derived(mutated)
        record("proven_wider_than_claim_rejected", "row.proven_wider_than_claim", "row.proven_wider_than_claim" in check_ids(mutated))

    # Dropping a declared launch-blocking requirement's row must be flagged.
    mutated = copy.deepcopy(index)
    declared = mutated.get("launch_blocking_requirement_refs", [])
    if declared:
        dropped = declared[0]
        mutated["rows"] = [r for r in mutated["rows"] if r.get("requirement_ref") != dropped]
        recompute_derived(mutated)
        record("launch_blocking_uncovered_rejected", "coverage.launch_blocking_uncovered", "coverage.launch_blocking_uncovered" in check_ids(mutated))

    # A publication decision that disagrees with the firing rules must be flagged.
    mutated = copy.deepcopy(index)
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
        ids = {f.check_id for f in validate_index(ensure_dict(candidate, "fixture"), repo_root)}
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
    index: dict[str, Any],
    findings: list[Finding],
    drill_results: list[dict[str, str]],
    fixture_results: list[dict[str, str]],
) -> None:
    decision, rule_ids, proof_ids = computed_publication(index)
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "record_kind": "stable_proof_index_validation_capture",
        "status": "pass" if not any(f.severity == "error" for f in findings) else "fail",
        "generated_at": generated_at_now(),
        "index_id": index.get("index_id"),
        "as_of": index.get("as_of"),
        "summary": index.get("summary"),
        "publication": {
            "decision": decision,
            "blocking_rule_ids": rule_ids,
            "blocking_proof_ids": proof_ids,
        },
        "negative_drills": drill_results,
        "fixture_cases": fixture_results,
        "findings": [f.as_report() for f in findings],
    }
    path.write_text(json_text(payload), encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    index = ensure_dict(
        load_json(repo_root / args.index, "stable proof index"),
        "stable proof index",
    )

    findings = validate_index(index, repo_root)
    drill_results, drill_findings = run_negative_drills(index, repo_root)
    findings.extend(drill_findings)
    fixture_results, fixture_findings = run_fixture_cases(repo_root, args.fixtures_dir)
    findings.extend(fixture_findings)

    report_rel = args.report
    if report_rel is None and not args.no_capture:
        report_rel = DEFAULT_REPORT_REL
    if report_rel:
        write_report(repo_root / report_rel, index, findings, drill_results, fixture_results)

    errors = [f for f in findings if f.severity == "error"]
    if errors:
        for item in errors:
            print(f"ERROR [{item.check_id}] ({item.ref}): {item.message}", file=sys.stderr)
        return 1

    summary = index.get("summary", {})
    decision, rule_ids, _proof_ids = computed_publication(index)
    print(
        "stable proof index validated "
        f"({summary.get('total_requirements')} requirements across {summary.get('total_claims')} claims, "
        f"{summary.get('requirements_proven_stable')} proven stable, "
        f"{summary.get('requirements_narrowed_below_cutline')} narrowed; "
        f"launch-blocking {summary.get('launch_blocking_proven_stable')}/{summary.get('launch_blocking_total')} proven; "
        f"packets {summary.get('packets_current')} current / "
        f"{summary.get('packets_due_for_refresh')} due / "
        f"{summary.get('packets_breached')} breached / "
        f"{summary.get('packets_missing')} missing; "
        f"{summary.get('proof_rules_firing')} rules firing; "
        f"publication={decision}; {len(drill_results)} negative drills, {len(fixture_results)} fixture cases)"
    )

    if args.require_proceed and decision == "hold":
        print(
            f"PUBLICATION HELD: stable proof-index publication blocked by {len(rule_ids)} rule(s): {', '.join(rule_ids)}",
            file=sys.stderr,
        )
        return 2

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
