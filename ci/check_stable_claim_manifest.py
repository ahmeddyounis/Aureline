#!/usr/bin/env python3
"""Validate the stable claim manifest, its canonical lifecycle labels, and the
packet-freshness SLO automation.

Stable launch-facing wording regresses when the maturity label a surface
publishes lives in prose, side spreadsheets, or optimistic badges, or when a
proof packet quietly ages out from under a Stable claim. This gate makes the
published lifecycle label a single typed record that *ingests* the stable claim
matrix, the stable qualification matrix, and the v1.0 support-class ledger, and
narrows automatically when any of them — or the proof packet's freshness SLO —
no longer holds. It reads the checked-in manifest at
``artifacts/release/stable_claim_manifest.json`` and:

  - asserts the closed vocabularies (lifecycle labels, freshness-SLO states,
    narrowing reasons, publication actions) and the launch cutline are canonical;
  - asserts every entry that is narrowed (for a narrowed backing claim, a
    narrowed qualification lane, a thinned support class, a breached/missing proof
    packet, or an expired waiver) drops below the cutline rather than inheriting
    an adjacent published label, and that a published label publishes its claim
    cleanly with a captured, within-SLO proof packet and an owner sign-off;
  - performs the packet-freshness SLO automation the typed model cannot — against
    the manifest ``as_of`` date it recomputes each packet's SLO state from its
    ``captured_at`` and ``freshness_slo`` and fails when a declared state is
    fresher than the clock allows or when a published label rides a packet whose
    recomputed state is outside its SLO;
  - performs the cross-artifact ingestion the typed model cannot — it reads the
    stable claim matrix, the stable qualification matrix, and the support-class
    ledger and fails when an entry's posture disagrees with a backing row
    (publishes while its backing narrowed, narrows on a backing that still holds,
    or names a backing the neighbouring artifact does not carry);
  - performs the waiver-expiry date arithmetic and fails when a provisional label
    rides an expired waiver or a waiver-expired entry's waiver is still active;
  - recomputes the publication verdict from the entries and publication rules and
    fails when the declared decision or blocking sets drift;
  - recomputes the summary block;
  - runs negative drills proving the narrowing, freshness, backing, waiver, and
    publication rejections all fire; and
  - runs the checked-in fixture cases under ``fixtures/release/stable_claim_manifest``
    and fails when a case the manifest marks as rejected validates clean.

With ``--require-proceed`` the gate additionally fails (exit code 2) when the
recomputed publication verdict is ``hold``, so shiproom and release tooling can
block stable claim publication directly from this artifact.

The typed Rust consumer
(``aureline_release::stable_claim_manifest::current_stable_claim_manifest``) reads
the same manifest and runs the same structural cross-check, so this gate and
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


DEFAULT_MANIFEST_REL = "artifacts/release/stable_claim_manifest.json"
DEFAULT_REPORT_REL = "artifacts/release/captures/stable_claim_manifest_validation_capture.json"
DEFAULT_FIXTURES_REL = "fixtures/release/stable_claim_manifest"

EXPECTED_SCHEMA_VERSION = 1
MANIFEST_RECORD_KIND = "stable_claim_manifest"

LIFECYCLE_LABELS = ("lts", "stable", "beta", "preview", "withdrawn")
ABOVE_CUTLINE = ("lts", "stable")
BELOW_CUTLINE = ("beta", "preview", "withdrawn")
FRESHNESS_SLO_STATES = ("current", "due_for_refresh", "breached", "missing")
NARROWING_REASONS = (
    "backing_claim_narrowed",
    "qualification_incomplete",
    "support_class_thinned",
    "proof_packet_freshness_breached",
    "proof_packet_missing",
    "waiver_expired",
    "owner_signoff_missing",
)
PUBLICATION_ACTIONS = (
    "hold_publication",
    "narrow_label",
    "refresh_proof_packet",
    "revalidate_backing_claim",
    "request_owner_signoff",
)
HOLDING_STATES = ("published", "provisional_on_waiver")
NARROWING_STATES = ("narrowed_unqualified", "narrowed_stale", "narrowed_waiver_expired")
MANIFEST_STATES = ("published", "provisional_on_waiver", *NARROWING_STATES)

LEVEL_RANK = {"lts": 4, "stable": 3, "beta": 2, "preview": 1, "withdrawn": 0}
CUTLINE_RANK = LEVEL_RANK["stable"]
# Freshness-SLO rank: a fresher state ranks higher.
FRESHNESS_RANK = {"current": 3, "due_for_refresh": 2, "breached": 1, "missing": 0}

# Levels at or above the stable cutline in the neighbouring artifacts.
STABLE_HOLDING_LEVELS = ("lts", "stable")


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
    parser.add_argument("--manifest", default=DEFAULT_MANIFEST_REL)
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


def reasons_of(entry: dict[str, Any]) -> list[str]:
    reasons = entry.get("active_narrowing_reasons", [])
    return reasons if isinstance(reasons, list) else []


def publication_rule_fires(rule: dict[str, Any], entries: list[dict[str, Any]]) -> bool:
    trigger = rule.get("trigger_reason")
    labels = rule.get("applies_to_labels", [])
    if not isinstance(labels, list):
        return False
    for entry in entries:
        if entry.get("claimed_label") in labels and trigger in reasons_of(entry):
            return True
    return False


def computed_publication(manifest: dict[str, Any]) -> tuple[str, list[str], list[str]]:
    entries = manifest.get("entries", [])
    rules = manifest.get("publication_rules", [])
    blocking_rules = [
        rule
        for rule in rules
        if rule.get("blocks_publication") is True and publication_rule_fires(rule, entries)
    ]
    rule_ids = sorted(str(rule.get("rule_id")) for rule in blocking_rules)
    blocking_triggers = {rule.get("trigger_reason") for rule in blocking_rules}
    entry_ids: set[str] = set()
    for entry in entries:
        if is_above_cutline(entry.get("claimed_label")) and blocking_triggers.intersection(
            reasons_of(entry)
        ):
            entry_ids.add(str(entry.get("entry_id")))
    decision = "hold" if blocking_rules else "proceed"
    return decision, rule_ids, sorted(entry_ids)


def computed_summary(manifest: dict[str, Any]) -> dict[str, int]:
    entries = manifest.get("entries", [])
    rules = manifest.get("publication_rules", [])

    def packet_state(entry: dict[str, Any]) -> Any:
        packet = entry.get("proof_packet")
        return packet.get("slo_state") if isinstance(packet, dict) else None

    published = [e for e in entries if is_above_cutline(e.get("published_label"))]
    return {
        "total_entries": len(entries),
        "entries_published_stable": len(published),
        "entries_narrowed_below_cutline": len(entries) - len(published),
        "entries_on_active_waiver": sum(
            1 for e in entries if e.get("manifest_state") == "provisional_on_waiver"
        ),
        "packets_current": sum(1 for e in entries if packet_state(e) == "current"),
        "packets_due_for_refresh": sum(1 for e in entries if packet_state(e) == "due_for_refresh"),
        "packets_breached": sum(1 for e in entries if packet_state(e) == "breached"),
        "packets_missing": sum(1 for e in entries if packet_state(e) == "missing"),
        "total_active_narrowing_reasons": sum(len(reasons_of(e)) for e in entries),
        "publication_rules_firing": sum(
            1 for rule in rules if publication_rule_fires(rule, entries)
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


def validate_envelope(manifest: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    manifest_id = str(manifest.get("manifest_id", "<manifest>"))

    if manifest.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(Finding("error", "manifest.schema_version", "schema_version must be 1", manifest_id))
    if manifest.get("record_kind") != MANIFEST_RECORD_KIND:
        findings.append(Finding("error", "manifest.record_kind", "record_kind is not supported", manifest_id))
    for field in (
        "manifest_id",
        "status",
        "overview_page",
        "as_of",
        "claim_matrix_ref",
        "qualification_matrix_ref",
        "support_class_ledger_ref",
    ):
        if not is_str(manifest.get(field)):
            findings.append(
                Finding("error", "manifest.empty_field", f"{field} must be a non-empty string", manifest_id)
            )
    if parse_date(manifest.get("as_of")) is None:
        findings.append(Finding("error", "manifest.as_of_invalid", "as_of must be an ISO date", manifest_id))

    for key, expected in (
        ("lifecycle_labels", list(LIFECYCLE_LABELS)),
        ("freshness_slo_states", list(FRESHNESS_SLO_STATES)),
        ("narrowing_reasons", list(NARROWING_REASONS)),
        ("publication_actions", list(PUBLICATION_ACTIONS)),
    ):
        if list(manifest.get(key, [])) != expected:
            findings.append(
                Finding("error", "manifest.vocabulary", f"manifest.{key} is not the closed vocabulary", key)
            )

    cutline = manifest.get("launch_cutline")
    if not isinstance(cutline, dict):
        findings.append(Finding("error", "cutline.missing", "manifest must carry a launch_cutline", manifest_id))
    else:
        if cutline.get("cutline_level") != "stable":
            findings.append(Finding("error", "cutline.level", "cutline_level must be stable", manifest_id))
        if list(cutline.get("above_cutline_levels", [])) != list(ABOVE_CUTLINE):
            findings.append(Finding("error", "cutline.above", "above_cutline_levels is not canonical", manifest_id))
        if list(cutline.get("below_cutline_levels", [])) != list(BELOW_CUTLINE):
            findings.append(Finding("error", "cutline.below", "below_cutline_levels is not canonical", manifest_id))
        if not is_str(cutline.get("description")):
            findings.append(Finding("error", "cutline.description", "cutline description must be non-empty", manifest_id))
    return findings


def validate_rules(manifest: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    rules = manifest.get("publication_rules")
    if not isinstance(rules, list) or not rules:
        findings.append(Finding("error", "rules.empty", "manifest must enumerate at least one publication rule", "<manifest>"))
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
        if rule.get("trigger_reason") not in NARROWING_REASONS:
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

    for reason in NARROWING_REASONS:
        if reason not in covered:
            findings.append(
                Finding("error", "rule.reason_uncovered", "narrowing reason has no rule watching for it", reason)
            )
    return findings


def validate_packet(entry_id: str, packet: Any) -> list[Finding]:
    findings: list[Finding] = []
    if not isinstance(packet, dict):
        findings.append(Finding("error", "entry.packet_missing", "entry must carry a proof_packet block", entry_id))
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
    if not (isinstance(target, int) and not isinstance(target, bool) and target >= 1):
        findings.append(Finding("error", "packet.target_invalid", "target_max_age_days must be an integer >= 1", entry_id))
    if not (isinstance(warn, int) and not isinstance(warn, bool) and warn >= 0):
        findings.append(Finding("error", "packet.warn_invalid", "warn_within_days must be an integer >= 0", entry_id))
    if (
        isinstance(target, int)
        and not isinstance(target, bool)
        and isinstance(warn, int)
        and not isinstance(warn, bool)
        and warn > target
    ):
        findings.append(Finding("error", "packet.slo_inconsistent", "warn_within_days may not exceed target_max_age_days", entry_id))
    return findings


def validate_entry(entry: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings: list[Finding] = []
    entry_id = str(entry.get("entry_id", "<entry>"))

    for field in ("entry_id", "title", "subject_family", "backing_claim_ref", "support_class_ref", "rationale"):
        if not is_str(entry.get(field)):
            findings.append(Finding("error", "entry.empty_field", f"entry {field} must be non-empty", entry_id))

    claimed = entry.get("claimed_label")
    published = entry.get("published_label")
    state = entry.get("manifest_state")
    reasons = reasons_of(entry)

    if claimed not in LIFECYCLE_LABELS:
        findings.append(Finding("error", "entry.claimed_invalid", "claimed_label is invalid", entry_id))
    elif not is_above_cutline(claimed):
        findings.append(Finding("error", "entry.claimed_below_cutline", "claimed_label must be at or above the cutline", entry_id))
    if published not in LIFECYCLE_LABELS:
        findings.append(Finding("error", "entry.published_invalid", "published_label is invalid", entry_id))
    if state not in MANIFEST_STATES:
        findings.append(Finding("error", "entry.state_invalid", "manifest_state is invalid", entry_id))
    if any(reason not in NARROWING_REASONS for reason in reasons):
        findings.append(Finding("error", "entry.reason_invalid", "active_narrowing_reasons has an unknown reason", entry_id))

    if claimed in LEVEL_RANK and published in LEVEL_RANK and LEVEL_RANK[published] > LEVEL_RANK[claimed]:
        findings.append(Finding("error", "entry.published_wider_than_claimed", "published_label is wider than claimed_label", entry_id))

    packet = entry.get("proof_packet")
    findings.extend(validate_packet(entry_id, packet))
    slo_state = packet.get("slo_state") if isinstance(packet, dict) else None

    holds = state in HOLDING_STATES

    if state in NARROWING_STATES:
        if is_above_cutline(published):
            findings.append(Finding("error", "entry.published_not_narrowed", "an entry that is not qualified must narrow below the cutline", entry_id))
        if not reasons:
            findings.append(Finding("error", "entry.narrowing_without_reason", "a narrowing entry must name an active narrowing reason", entry_id))
        if slo_state == "breached" and "proof_packet_freshness_breached" not in reasons:
            findings.append(Finding("error", "entry.breached_without_reason", "a breached packet must name proof_packet_freshness_breached", entry_id))
        if slo_state == "missing" and "proof_packet_missing" not in reasons:
            findings.append(Finding("error", "entry.missing_without_reason", "a missing packet must name proof_packet_missing", entry_id))
    if holds:
        if claimed != published:
            findings.append(Finding("error", "entry.held_label_not_equal_claimed", "a published entry must publish its claimed label", entry_id))
        if reasons:
            findings.append(Finding("error", "entry.held_with_active_narrowing", "a published entry must carry no active narrowing reason", entry_id))
        if isinstance(packet, dict) and not has_capture(packet):
            findings.append(Finding("error", "entry.held_without_fresh_packet", "a published entry must ride a captured, evidence-backed proof packet", entry_id))
        if slo_state in ("breached", "missing"):
            findings.append(Finding("error", "entry.held_on_stale_packet", "a published entry must ride a packet within its freshness SLO", entry_id))
        signoff = entry.get("owner_signoff", {})
        if not (isinstance(signoff, dict) and signoff.get("signed_off") is True and is_str(signoff.get("signed_at"))):
            findings.append(Finding("error", "entry.held_without_signoff", "a published entry must carry an owner sign-off with a date", entry_id))

    findings.extend(validate_state_reason_coherence(entry_id, state, reasons, entry))
    findings.extend(validate_entry_refs(entry_id, entry, repo_root))
    return findings


def validate_state_reason_coherence(entry_id: str, state: Any, reasons: list[str], entry: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    if state == "narrowed_unqualified":
        allowed = {
            "backing_claim_narrowed",
            "qualification_incomplete",
            "support_class_thinned",
            "owner_signoff_missing",
        }
        if not allowed.intersection(reasons):
            findings.append(Finding("error", "entry.state_reason_incoherent", "narrowed_unqualified requires a backing/qualification/support/signoff reason", entry_id))
    if state == "narrowed_stale" and not (
        "proof_packet_freshness_breached" in reasons or "proof_packet_missing" in reasons
    ):
        findings.append(Finding("error", "entry.state_reason_incoherent", "narrowed_stale requires a packet-freshness reason", entry_id))
    if state == "narrowed_waiver_expired":
        if "waiver_expired" not in reasons:
            findings.append(Finding("error", "entry.state_reason_incoherent", "narrowed_waiver_expired requires the waiver_expired reason", entry_id))
        if not isinstance(entry.get("waiver"), dict):
            findings.append(Finding("error", "entry.waiver_state_without_waiver", "narrowed_waiver_expired state must name a waiver", entry_id))
    if state == "provisional_on_waiver":
        waiver = entry.get("waiver")
        if not isinstance(waiver, dict) or not is_str(waiver.get("waiver_ref")) or not is_str(waiver.get("expires_at")):
            findings.append(Finding("error", "entry.waiver_state_without_waiver", "provisional_on_waiver state must name a waiver", entry_id))
    return findings


def validate_entry_refs(entry_id: str, entry: dict[str, Any], repo_root: Path) -> list[Finding]:
    """Path-existence checks for the proof-packet refs (not the cross-artifact
    ids, which resolve against the neighbouring artifacts)."""
    findings: list[Finding] = []
    packet = entry.get("proof_packet")
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
            findings.append(Finding("error", "entry.ref_missing", f"referenced artifact does not exist: {ref}", entry_id))
    return findings


def validate_freshness(manifest: dict[str, Any]) -> list[Finding]:
    """Packet-freshness SLO automation: recompute each packet's SLO state against
    as_of and fail when the declared state is fresher than the clock allows, or
    when a published label rides a packet whose recomputed state is outside its
    SLO."""
    findings: list[Finding] = []
    as_of = parse_date(manifest.get("as_of"))
    if as_of is None:
        return findings
    for entry in manifest.get("entries", []):
        entry_id = str(entry.get("entry_id", "<entry>"))
        packet = entry.get("proof_packet")
        if not isinstance(packet, dict):
            continue
        declared = packet.get("slo_state")
        computed = computed_slo_state(packet, as_of)
        if computed is None or declared not in FRESHNESS_RANK or computed not in FRESHNESS_RANK:
            continue
        if FRESHNESS_RANK[declared] > FRESHNESS_RANK[computed]:
            findings.append(
                Finding("error", "entry.packet_freshness_overstated", f"declared slo_state {declared!r} is fresher than the {computed!r} the as_of date allows", entry_id)
            )
        if entry.get("manifest_state") in HOLDING_STATES and computed in ("breached", "missing"):
            findings.append(
                Finding("error", "entry.published_on_breached_packet", "a published label rides a proof packet past its freshness SLO against as_of", entry_id)
            )
    return findings


def validate_waivers(manifest: dict[str, Any]) -> list[Finding]:
    """Waiver-expiry date arithmetic against as_of."""
    findings: list[Finding] = []
    as_of = parse_date(manifest.get("as_of"))
    if as_of is None:
        return findings
    for entry in manifest.get("entries", []):
        entry_id = str(entry.get("entry_id", "<entry>"))
        state = entry.get("manifest_state")
        waiver = entry.get("waiver")
        if not isinstance(waiver, dict):
            continue
        expires = parse_date(waiver.get("expires_at"))
        if expires is None:
            findings.append(Finding("error", "entry.waiver_expiry_invalid", "waiver expires_at must be an ISO date", entry_id))
            continue
        expired = as_of >= expires
        if expired and state == "provisional_on_waiver":
            findings.append(Finding("error", "entry.provisional_on_expired_waiver", "a provisional label relies on a waiver that has expired against as_of", entry_id))
        if not expired and state == "narrowed_waiver_expired":
            findings.append(Finding("error", "entry.waiver_expired_but_active", "an entry is marked waiver-expired but the waiver is still active against as_of", entry_id))
    return findings


def _load_neighbour(repo_root: Path, ref: Any, label: str, findings: list[Finding]) -> dict[str, Any] | None:
    if not is_str(ref):
        return None
    path = repo_root / ref
    if not path.exists():
        findings.append(Finding("error", "backing.artifact_missing", f"{label} does not exist: {ref}", "<manifest>"))
        return None
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        findings.append(Finding("error", "backing.artifact_invalid", f"{label} is not valid JSON: {exc}", ref))
        return None


def validate_backing(manifest: dict[str, Any], repo_root: Path) -> list[Finding]:
    """Cross-artifact ingestion: read the stable claim matrix, the stable
    qualification matrix, and the support-class ledger, and fail when an entry's
    posture disagrees with a backing row."""
    findings: list[Finding] = []

    matrix = _load_neighbour(repo_root, manifest.get("claim_matrix_ref"), "claim_matrix_ref", findings)
    qual = _load_neighbour(repo_root, manifest.get("qualification_matrix_ref"), "qualification_matrix_ref", findings)
    ledger = _load_neighbour(repo_root, manifest.get("support_class_ledger_ref"), "support_class_ledger_ref", findings)

    claim_effective = {}
    if isinstance(matrix, dict):
        claim_effective = {
            str(r.get("claim_id")): r.get("effective_level")
            for r in matrix.get("rows", [])
            if isinstance(r, dict)
        }
    qual_effective = {}
    if isinstance(qual, dict):
        qual_effective = {
            str(r.get("row_id")): r.get("effective_level")
            for r in qual.get("rows", [])
            if isinstance(r, dict)
        }
    support_state = {}
    if isinstance(ledger, dict):
        support_state = {
            str(e.get("entry_id")): (e.get("claimed_class"), e.get("effective_class"))
            for e in ledger.get("entries", [])
            if isinstance(e, dict)
        }

    for entry in manifest.get("entries", []):
        entry_id = str(entry.get("entry_id", "<entry>"))
        holds = entry.get("manifest_state") in HOLDING_STATES
        reasons = reasons_of(entry)

        # Backing stable claim.
        backing_ref = entry.get("backing_claim_ref")
        if is_str(backing_ref) and isinstance(matrix, dict):
            if backing_ref not in claim_effective:
                findings.append(Finding("error", "backing.claim_unknown", f"backing_claim_ref is not a row in the stable claim matrix: {backing_ref}", entry_id))
            else:
                backing_holds = claim_effective[backing_ref] in STABLE_HOLDING_LEVELS
                if not backing_holds:
                    if holds:
                        findings.append(Finding("error", "backing.holds_on_narrowed_backing", "an entry publishes its label while its backing stable claim is narrowed below the cutline", entry_id))
                    if "backing_claim_narrowed" not in reasons:
                        findings.append(Finding("error", "backing.narrowing_reason_missing", "an entry whose backing stable claim is narrowed must carry the backing_claim_narrowed reason", entry_id))
                elif "backing_claim_narrowed" in reasons:
                    findings.append(Finding("error", "backing.reason_without_narrowed_claim", "an entry names backing_claim_narrowed but its backing stable claim still holds", entry_id))

        # Backing qualification lanes.
        if isinstance(qual, dict):
            qual_refs = entry.get("qualification_row_refs", []) or []
            any_narrowed = False
            for q_ref in qual_refs:
                if not is_str(q_ref):
                    continue
                if q_ref not in qual_effective:
                    findings.append(Finding("error", "backing.qual_unknown", f"qualification_row_ref is not a row in the stable qualification matrix: {q_ref}", entry_id))
                elif qual_effective[q_ref] not in STABLE_HOLDING_LEVELS:
                    any_narrowed = True
            if any_narrowed:
                if holds:
                    findings.append(Finding("error", "backing.holds_on_narrowed_qual", "an entry publishes its label while a backing qualification lane is narrowed below the cutline", entry_id))
                if "qualification_incomplete" not in reasons:
                    findings.append(Finding("error", "backing.qual_reason_missing", "an entry with a narrowed qualification lane must carry the qualification_incomplete reason", entry_id))
            elif qual_refs and "qualification_incomplete" in reasons:
                findings.append(Finding("error", "backing.qual_reason_without_narrowing", "an entry names qualification_incomplete but every backing qualification lane holds", entry_id))

        # Backing support class.
        support_ref = entry.get("support_class_ref")
        if is_str(support_ref) and isinstance(ledger, dict):
            if support_ref not in support_state:
                findings.append(Finding("error", "backing.support_unknown", f"support_class_ref is not an entry in the support-class ledger: {support_ref}", entry_id))
            else:
                claimed_class, effective_class = support_state[support_ref]
                thinned = claimed_class != effective_class
                if thinned:
                    if holds:
                        findings.append(Finding("error", "backing.holds_on_thinned_support", "an entry publishes its label while its backing support class is thinned below the class it was put forward as", entry_id))
                    if "support_class_thinned" not in reasons:
                        findings.append(Finding("error", "backing.support_reason_missing", "an entry whose support class is thinned must carry the support_class_thinned reason", entry_id))
                elif "support_class_thinned" in reasons:
                    findings.append(Finding("error", "backing.support_reason_without_thinning", "an entry names support_class_thinned but its support class still holds", entry_id))
    return findings


def validate_publication(manifest: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    publication = manifest.get("publication")
    if not isinstance(publication, dict):
        findings.append(Finding("error", "publication.missing", "manifest must carry a publication block", "<manifest>"))
        return findings
    for field in ("publication_gate", "rationale"):
        if not is_str(publication.get(field)):
            findings.append(Finding("error", "publication.empty_field", f"publication.{field} must be non-empty", "<manifest>"))
    decision, rule_ids, entry_ids = computed_publication(manifest)
    if publication.get("decision") != decision:
        findings.append(
            Finding("error", "publication.decision_inconsistent", f"declared decision {publication.get('decision')!r} disagrees with computed {decision!r}", "<manifest>")
        )
    if list(publication.get("blocking_rule_ids", [])) != rule_ids:
        findings.append(Finding("error", "publication.blocking_rules_mismatch", "blocking_rule_ids disagrees with the firing publication rules", "<manifest>"))
    if list(publication.get("blocking_entry_ids", [])) != entry_ids:
        findings.append(Finding("error", "publication.blocking_entries_mismatch", "blocking_entry_ids disagrees with the firing publication rules", "<manifest>"))
    return findings


def validate_summary(manifest: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    summary = manifest.get("summary")
    if not isinstance(summary, dict):
        findings.append(Finding("error", "summary.missing", "manifest must carry a summary block", "<manifest>"))
        return findings
    expected = computed_summary(manifest)
    for key, value in expected.items():
        if summary.get(key) != value:
            findings.append(Finding("error", "summary.count_mismatch", f"summary.{key} must equal {value}", key))
    return findings


def validate_manifest(manifest: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings = validate_envelope(manifest)
    findings.extend(validate_rules(manifest))

    entries = manifest.get("entries")
    if not isinstance(entries, list) or not entries:
        findings.append(Finding("error", "manifest.entries_empty", "manifest must enumerate at least one entry", "<manifest>"))
        return findings

    seen: set[str] = set()
    for raw in entries:
        entry = ensure_dict(raw, "manifest.entries[]")
        findings.extend(validate_entry(entry, repo_root))
        entry_id = str(entry.get("entry_id", "<entry>"))
        if entry_id in seen:
            findings.append(Finding("error", "entry.duplicate_id", "entry ids must be unique", entry_id))
        seen.add(entry_id)

    findings.extend(validate_freshness(manifest))
    findings.extend(validate_waivers(manifest))
    findings.extend(validate_backing(manifest, repo_root))
    findings.extend(validate_publication(manifest))
    findings.extend(validate_summary(manifest))
    return findings


def recompute_derived(manifest: dict[str, Any]) -> None:
    manifest["summary"] = computed_summary(manifest)
    decision, rule_ids, entry_ids = computed_publication(manifest)
    if isinstance(manifest.get("publication"), dict):
        manifest["publication"].update(
            {"decision": decision, "blocking_rule_ids": rule_ids, "blocking_entry_ids": entry_ids}
        )


def run_negative_drills(manifest: dict[str, Any], repo_root: Path) -> tuple[list[dict[str, str]], list[Finding]]:
    results: list[dict[str, str]] = []
    findings: list[Finding] = []

    def record(drill_id: str, expected_check_id: str, fired: bool) -> None:
        results.append({"drill_id": drill_id, "expected_check_id": expected_check_id, "status": "passed" if fired else "failed"})
        if not fired:
            findings.append(Finding("error", "negative_drill.not_rejected", f"negative drill {drill_id} did not fire", drill_id))

    def check_ids(candidate: dict[str, Any]) -> set[str]:
        return {f.check_id for f in validate_manifest(candidate, repo_root)}

    # A narrowing entry that still publishes a stable label must be flagged.
    mutated = copy.deepcopy(manifest)
    target = next((e for e in mutated["entries"] if e.get("manifest_state") in NARROWING_STATES), None)
    if target is not None:
        target["published_label"] = target["claimed_label"]
        recompute_derived(mutated)
        record("published_not_narrowed_rejected", "entry.published_not_narrowed", "entry.published_not_narrowed" in check_ids(mutated))

    # A published entry carrying an active narrowing reason must be flagged.
    mutated = copy.deepcopy(manifest)
    target = next((e for e in mutated["entries"] if e.get("manifest_state") in HOLDING_STATES), None)
    if target is not None:
        target["active_narrowing_reasons"] = ["backing_claim_narrowed"]
        recompute_derived(mutated)
        record("held_with_active_narrowing_rejected", "entry.held_with_active_narrowing", "entry.held_with_active_narrowing" in check_ids(mutated))

    # A published entry whose packet is breached against as_of must be flagged.
    mutated = copy.deepcopy(manifest)
    target = next((e for e in mutated["entries"] if e.get("manifest_state") in HOLDING_STATES), None)
    if target is not None:
        target["proof_packet"]["captured_at"] = "2000-01-01"
        record("published_on_breached_packet_rejected", "entry.published_on_breached_packet", "entry.published_on_breached_packet" in check_ids(mutated))

    # A packet whose declared state overstates its freshness must be flagged.
    mutated = copy.deepcopy(manifest)
    target = next(
        (e for e in mutated["entries"] if e.get("proof_packet", {}).get("slo_state") == "current"),
        None,
    )
    if target is not None:
        target["proof_packet"]["captured_at"] = "2000-01-01"
        record("packet_freshness_overstated_rejected", "entry.packet_freshness_overstated", "entry.packet_freshness_overstated" in check_ids(mutated))

    # An entry that holds while its backing stable claim is narrowed must be flagged.
    mutated = copy.deepcopy(manifest)
    target = next(
        (e for e in mutated["entries"] if e.get("manifest_state") in HOLDING_STATES and is_str(e.get("backing_claim_ref"))),
        None,
    )
    if target is not None:
        mutated["claim_matrix_ref"] = "fixtures/release/stable_claim_manifest/narrowed_claim_matrix.json"
        target["backing_claim_ref"] = "stable_claim:export_and_offboarding_support"
        record("holds_on_narrowed_backing_rejected", "backing.holds_on_narrowed_backing", "backing.holds_on_narrowed_backing" in check_ids(mutated))

    # A provisional entry whose waiver expired against as_of must be flagged.
    mutated = copy.deepcopy(manifest)
    target = next((e for e in mutated["entries"] if e.get("manifest_state") == "provisional_on_waiver"), None)
    if target is not None and isinstance(target.get("waiver"), dict):
        target["waiver"]["expires_at"] = "2000-01-01"
        record("provisional_on_expired_waiver_rejected", "entry.provisional_on_expired_waiver", "entry.provisional_on_expired_waiver" in check_ids(mutated))

    # A publication decision that disagrees with the firing rules must be flagged.
    mutated = copy.deepcopy(manifest)
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
        ids = {f.check_id for f in validate_manifest(ensure_dict(candidate, "fixture"), repo_root)}
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
    manifest: dict[str, Any],
    findings: list[Finding],
    drill_results: list[dict[str, str]],
    fixture_results: list[dict[str, str]],
) -> None:
    decision, rule_ids, entry_ids = computed_publication(manifest)
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "record_kind": "stable_claim_manifest_validation_capture",
        "status": "pass" if not any(f.severity == "error" for f in findings) else "fail",
        "generated_at": generated_at_now(),
        "manifest_id": manifest.get("manifest_id"),
        "as_of": manifest.get("as_of"),
        "summary": manifest.get("summary"),
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

    manifest = ensure_dict(
        load_json(repo_root / args.manifest, "stable claim manifest"),
        "stable claim manifest",
    )

    findings = validate_manifest(manifest, repo_root)
    drill_results, drill_findings = run_negative_drills(manifest, repo_root)
    findings.extend(drill_findings)
    fixture_results, fixture_findings = run_fixture_cases(repo_root, args.fixtures_dir)
    findings.extend(fixture_findings)

    report_rel = args.report
    if report_rel is None and not args.no_capture:
        report_rel = DEFAULT_REPORT_REL
    if report_rel:
        write_report(repo_root / report_rel, manifest, findings, drill_results, fixture_results)

    errors = [f for f in findings if f.severity == "error"]
    if errors:
        for item in errors:
            print(f"ERROR [{item.check_id}] ({item.ref}): {item.message}", file=sys.stderr)
        return 1

    summary = manifest.get("summary", {})
    decision, rule_ids, _entry_ids = computed_publication(manifest)
    print(
        "stable claim manifest validated "
        f"({summary.get('total_entries')} entries, "
        f"{summary.get('entries_published_stable')} published stable, "
        f"{summary.get('entries_narrowed_below_cutline')} narrowed; "
        f"packets {summary.get('packets_current')} current / "
        f"{summary.get('packets_due_for_refresh')} due / "
        f"{summary.get('packets_breached')} breached / "
        f"{summary.get('packets_missing')} missing; "
        f"{summary.get('publication_rules_firing')} rules firing; "
        f"publication={decision}; {len(drill_results)} negative drills, {len(fixture_results)} fixture cases)"
    )

    if args.require_proceed and decision == "hold":
        print(
            f"PUBLICATION HELD: stable claim publication blocked by {len(rule_ids)} rule(s): {', '.join(rule_ids)}",
            file=sys.stderr,
        )
        return 2

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
