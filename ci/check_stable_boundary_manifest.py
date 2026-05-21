#!/usr/bin/env python3
"""Validate the stable boundary manifest across the local-OSS, self-hosted,
managed, and air-gapped value lines.

A single canonical lifecycle label per subject (the stable claim manifest) does
not survive contact with how the product is deployed: a subject that is Stable
when a hosted gateway is reachable can be unsupported air-gapped. This gate makes
the per-value-line boundary a single typed record that *ingests* the stable claim
manifest as a hard ceiling and narrows automatically when a value line cannot
match the subject's canonical label. It reads the checked-in manifest at
``artifacts/release/stable_boundary_manifest.json`` and:

  - asserts the closed vocabularies (lifecycle labels, value lines, boundary
    states, narrowing reasons, boundary actions) and the launch cutline are
    canonical;
  - asserts every row that is narrowed (for an absent line capability, incomplete
    line evidence, a breached/missing line proof packet, an expired waiver, or a
    subject whose canonical manifest label is itself below the cutline) drops
    below the cutline rather than publishing wider than the subject's label, and
    that a published row publishes the subject's canonical label cleanly with a
    captured, within-SLO proof packet and an owner sign-off;
  - asserts the manifest is a full subject x value-line matrix: every subject
    carries exactly one row per value line and the same ceiling label everywhere;
  - performs the ceiling cross-check the typed model cannot — it reads the stable
    claim manifest named by ``claim_manifest_ref`` and fails when a row's
    ``manifest_label`` is not the label the claim manifest publishes for that
    entry, or names an entry the claim manifest does not carry;
  - performs the packet-freshness SLO automation the typed model cannot — against
    the manifest ``as_of`` date it recomputes each packet's SLO state from its
    ``captured_at`` and ``freshness_slo`` and fails when a declared state is
    fresher than the clock allows or when a published row rides a packet whose
    recomputed state is outside its SLO;
  - performs the waiver-expiry date arithmetic;
  - recomputes the publication verdict, the per-value-line rollups, and the
    summary block, and fails on any drift;
  - runs negative drills proving the narrowing, ceiling, freshness, waiver, and
    publication rejections all fire; and
  - runs the checked-in fixture cases under
    ``fixtures/release/stable_boundary_manifest`` and fails when a case the
    manifest marks as rejected validates clean.

With ``--require-proceed`` the gate additionally fails (exit code 2) when the
recomputed publication verdict is ``hold``, so shiproom and release tooling can
block stable boundary publication directly from this artifact.

The typed Rust consumer
(``aureline_release::stable_boundary_manifest::current_stable_boundary_manifest``)
reads the same manifest and runs the same structural cross-check, so this gate and
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


DEFAULT_MANIFEST_REL = "artifacts/release/stable_boundary_manifest.json"
DEFAULT_REPORT_REL = (
    "artifacts/release/captures/stable_boundary_manifest_validation_capture.json"
)
DEFAULT_FIXTURES_REL = "fixtures/release/stable_boundary_manifest"

EXPECTED_SCHEMA_VERSION = 1
MANIFEST_RECORD_KIND = "stable_boundary_manifest"

LIFECYCLE_LABELS = ("lts", "stable", "beta", "preview", "withdrawn")
ABOVE_CUTLINE = ("lts", "stable")
BELOW_CUTLINE = ("beta", "preview", "withdrawn")
VALUE_LINES = ("local_oss", "self_hosted", "managed", "air_gapped")
FRESHNESS_SLO_STATES = ("current", "due_for_refresh", "breached", "missing")
BOUNDARY_STATES = (
    "available",
    "available_on_waiver",
    "narrowed_unsupported",
    "narrowed_by_manifest",
    "narrowed_stale",
    "narrowed_waiver_expired",
)
NARROWING_REASONS = (
    "manifest_label_narrowed",
    "line_capability_absent",
    "line_evidence_incomplete",
    "boundary_packet_freshness_breached",
    "boundary_packet_missing",
    "waiver_expired",
    "owner_signoff_missing",
)
BOUNDARY_ACTIONS = (
    "hold_publication",
    "narrow_line_label",
    "refresh_boundary_packet",
    "revalidate_line_support",
    "request_owner_signoff",
)
HOLDING_STATES = ("available", "available_on_waiver")
NARROWING_STATES = (
    "narrowed_unsupported",
    "narrowed_by_manifest",
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


def reasons_of(row: dict[str, Any]) -> list[str]:
    reasons = row.get("active_narrowing_reasons", [])
    return reasons if isinstance(reasons, list) else []


def boundary_rule_fires(rule: dict[str, Any], rows: list[dict[str, Any]]) -> bool:
    trigger = rule.get("trigger_reason")
    labels = rule.get("applies_to_labels", [])
    if not isinstance(labels, list):
        return False
    for row in rows:
        if row.get("manifest_label") in labels and trigger in reasons_of(row):
            return True
    return False


def computed_publication(manifest: dict[str, Any]) -> tuple[str, list[str], list[str]]:
    rows = manifest.get("rows", [])
    rules = manifest.get("boundary_rules", [])
    blocking_rules = [
        rule
        for rule in rules
        if rule.get("blocks_publication") is True and boundary_rule_fires(rule, rows)
    ]
    rule_ids = sorted(str(rule.get("rule_id")) for rule in blocking_rules)
    blocking_triggers = {rule.get("trigger_reason") for rule in blocking_rules}
    boundary_ids: set[str] = set()
    for row in rows:
        if is_above_cutline(row.get("manifest_label")) and blocking_triggers.intersection(
            reasons_of(row)
        ):
            boundary_ids.add(str(row.get("boundary_id")))
    decision = "hold" if blocking_rules else "proceed"
    return decision, rule_ids, sorted(boundary_ids)


def computed_line_rollups(manifest: dict[str, Any]) -> list[dict[str, Any]]:
    rows = manifest.get("rows", [])
    rollups: list[dict[str, Any]] = []
    for line in VALUE_LINES:
        line_rows = [r for r in rows if r.get("value_line") == line]
        published = [r for r in line_rows if is_above_cutline(r.get("published_label"))]
        rollups.append(
            {
                "value_line": line,
                "total": len(line_rows),
                "published_stable": len(published),
                "narrowed_below_cutline": len(line_rows) - len(published),
            }
        )
    return rollups


def computed_summary(manifest: dict[str, Any]) -> dict[str, Any]:
    rows = manifest.get("rows", [])
    rules = manifest.get("boundary_rules", [])

    def packet_state(row: dict[str, Any]) -> Any:
        packet = row.get("boundary_packet")
        return packet.get("slo_state") if isinstance(packet, dict) else None

    published = [r for r in rows if is_above_cutline(r.get("published_label"))]
    subjects = {str(r.get("manifest_entry_ref")) for r in rows}
    return {
        "total_boundaries": len(rows),
        "total_subjects": len(subjects),
        "boundaries_published_stable": len(published),
        "boundaries_narrowed_below_cutline": len(rows) - len(published),
        "boundaries_on_active_waiver": sum(
            1 for r in rows if r.get("boundary_state") == "available_on_waiver"
        ),
        "packets_current": sum(1 for r in rows if packet_state(r) == "current"),
        "packets_due_for_refresh": sum(1 for r in rows if packet_state(r) == "due_for_refresh"),
        "packets_breached": sum(1 for r in rows if packet_state(r) == "breached"),
        "packets_missing": sum(1 for r in rows if packet_state(r) == "missing"),
        "total_active_narrowing_reasons": sum(len(reasons_of(r)) for r in rows),
        "boundary_rules_firing": sum(
            1 for rule in rules if boundary_rule_fires(rule, rows)
        ),
        "line_rollups": computed_line_rollups(manifest),
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
    for field in ("manifest_id", "status", "overview_page", "as_of", "claim_manifest_ref"):
        if not is_str(manifest.get(field)):
            findings.append(
                Finding("error", "manifest.empty_field", f"{field} must be a non-empty string", manifest_id)
            )
    if parse_date(manifest.get("as_of")) is None:
        findings.append(Finding("error", "manifest.as_of_invalid", "as_of must be an ISO date", manifest_id))

    for key, expected in (
        ("lifecycle_labels", list(LIFECYCLE_LABELS)),
        ("boundary_states", list(BOUNDARY_STATES)),
        ("narrowing_reasons", list(NARROWING_REASONS)),
        ("boundary_actions", list(BOUNDARY_ACTIONS)),
    ):
        if list(manifest.get(key, [])) != expected:
            findings.append(
                Finding("error", "manifest.vocabulary", f"manifest.{key} is not the closed vocabulary", key)
            )

    profiles = manifest.get("value_lines")
    if not isinstance(profiles, list) or [p.get("value_line") for p in profiles] != list(VALUE_LINES):
        findings.append(
            Finding("error", "manifest.value_lines", "value_lines is not the closed value-line vocabulary", manifest_id)
        )
    else:
        for profile in profiles:
            line = str(profile.get("value_line", "<line>"))
            for field in ("title", "connectivity_posture", "description"):
                if not is_str(profile.get(field)):
                    findings.append(
                        Finding("error", "value_line.empty_field", f"value line {field} must be non-empty", line)
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
    rules = manifest.get("boundary_rules")
    if not isinstance(rules, list) or not rules:
        findings.append(Finding("error", "rules.empty", "manifest must enumerate at least one boundary rule", "<manifest>"))
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
        if rule.get("default_action") not in BOUNDARY_ACTIONS:
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


def validate_packet(boundary_id: str, packet: Any) -> list[Finding]:
    findings: list[Finding] = []
    if not isinstance(packet, dict):
        findings.append(Finding("error", "row.packet_missing", "row must carry a boundary_packet block", boundary_id))
        return findings
    for field in ("packet_id", "packet_ref", "proof_index_ref"):
        if not is_str(packet.get(field)):
            findings.append(Finding("error", "packet.empty_field", f"boundary_packet.{field} must be non-empty", boundary_id))
    if packet.get("slo_state") not in FRESHNESS_SLO_STATES:
        findings.append(Finding("error", "packet.state_invalid", "slo_state is outside the vocabulary", boundary_id))
    slo = packet.get("freshness_slo")
    if not isinstance(slo, dict):
        findings.append(Finding("error", "packet.slo_missing", "boundary_packet must carry a freshness_slo block", boundary_id))
        return findings
    if not is_str(slo.get("slo_register_ref")):
        findings.append(Finding("error", "packet.slo_register_missing", "freshness_slo.slo_register_ref must be non-empty", boundary_id))
    target = slo.get("target_max_age_days")
    warn = slo.get("warn_within_days")
    if not (isinstance(target, int) and not isinstance(target, bool) and target >= 1):
        findings.append(Finding("error", "packet.target_invalid", "target_max_age_days must be an integer >= 1", boundary_id))
    if not (isinstance(warn, int) and not isinstance(warn, bool) and warn >= 0):
        findings.append(Finding("error", "packet.warn_invalid", "warn_within_days must be an integer >= 0", boundary_id))
    if (
        isinstance(target, int)
        and not isinstance(target, bool)
        and isinstance(warn, int)
        and not isinstance(warn, bool)
        and warn > target
    ):
        findings.append(Finding("error", "packet.slo_inconsistent", "warn_within_days may not exceed target_max_age_days", boundary_id))
    return findings


def validate_row(row: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings: list[Finding] = []
    boundary_id = str(row.get("boundary_id", "<row>"))

    for field in (
        "boundary_id",
        "title",
        "subject_family",
        "manifest_entry_ref",
        "line_capability_ref",
        "rationale",
    ):
        if not is_str(row.get(field)):
            findings.append(Finding("error", "row.empty_field", f"row {field} must be non-empty", boundary_id))

    if row.get("value_line") not in VALUE_LINES:
        findings.append(Finding("error", "row.value_line_invalid", "value_line is outside the vocabulary", boundary_id))

    manifest_label = row.get("manifest_label")
    published = row.get("published_label")
    state = row.get("boundary_state")
    reasons = reasons_of(row)

    if manifest_label not in LIFECYCLE_LABELS:
        findings.append(Finding("error", "row.manifest_label_invalid", "manifest_label is invalid", boundary_id))
    if published not in LIFECYCLE_LABELS:
        findings.append(Finding("error", "row.published_invalid", "published_label is invalid", boundary_id))
    if state not in BOUNDARY_STATES:
        findings.append(Finding("error", "row.state_invalid", "boundary_state is invalid", boundary_id))
    if any(reason not in NARROWING_REASONS for reason in reasons):
        findings.append(Finding("error", "row.reason_invalid", "active_narrowing_reasons has an unknown reason", boundary_id))

    # The ceiling: no value line may publish wider than the subject's manifest label.
    if manifest_label in LEVEL_RANK and published in LEVEL_RANK and LEVEL_RANK[published] > LEVEL_RANK[manifest_label]:
        findings.append(Finding("error", "row.published_wider_than_manifest", "published_label is wider than the manifest ceiling", boundary_id))

    packet = row.get("boundary_packet")
    findings.extend(validate_packet(boundary_id, packet))
    slo_state = packet.get("slo_state") if isinstance(packet, dict) else None

    holds = state in HOLDING_STATES
    manifest_holds = is_above_cutline(manifest_label)

    # A subject canonically below the cutline forces every line to inherit the
    # ceiling and narrow.
    if manifest_label in LEVEL_RANK and not manifest_holds:
        if holds:
            findings.append(Finding("error", "row.held_on_narrowed_manifest", "a row holds a label while the subject's manifest label is below the cutline", boundary_id))
        if "manifest_label_narrowed" not in reasons:
            findings.append(Finding("error", "row.manifest_narrowed_without_reason", "a row whose subject is narrowed by the manifest must name manifest_label_narrowed", boundary_id))

    if state in NARROWING_STATES:
        if is_above_cutline(published):
            findings.append(Finding("error", "row.published_not_narrowed", "a value line that is not supported must narrow below the cutline", boundary_id))
        if not reasons:
            findings.append(Finding("error", "row.narrowing_without_reason", "a narrowing row must name an active narrowing reason", boundary_id))
        if slo_state == "breached" and "boundary_packet_freshness_breached" not in reasons:
            findings.append(Finding("error", "row.breached_without_reason", "a breached packet must name boundary_packet_freshness_breached", boundary_id))
        if slo_state == "missing" and "boundary_packet_missing" not in reasons:
            findings.append(Finding("error", "row.missing_without_reason", "a missing packet must name boundary_packet_missing", boundary_id))
    if holds:
        if manifest_label != published:
            findings.append(Finding("error", "row.held_label_not_equal_manifest", "a published row must publish the subject's canonical manifest label", boundary_id))
        if reasons:
            findings.append(Finding("error", "row.held_with_active_narrowing", "a published row must carry no active narrowing reason", boundary_id))
        if isinstance(packet, dict) and not has_capture(packet):
            findings.append(Finding("error", "row.held_without_fresh_packet", "a published row must ride a captured, evidence-backed proof packet", boundary_id))
        if slo_state in ("breached", "missing"):
            findings.append(Finding("error", "row.held_on_stale_packet", "a published row must ride a packet within its freshness SLO", boundary_id))
        signoff = row.get("owner_signoff", {})
        if not (isinstance(signoff, dict) and signoff.get("signed_off") is True and is_str(signoff.get("signed_at"))):
            findings.append(Finding("error", "row.held_without_signoff", "a published row must carry an owner sign-off with a date", boundary_id))
    signoff = row.get("owner_signoff", {})
    if not (isinstance(signoff, dict) and is_str(signoff.get("owner_ref"))):
        findings.append(Finding("error", "row.signoff_owner_missing", "owner_signoff.owner_ref must be non-empty", boundary_id))

    findings.extend(validate_state_reason_coherence(boundary_id, state, reasons, row))
    findings.extend(validate_row_refs(boundary_id, row, repo_root))
    return findings


def validate_state_reason_coherence(boundary_id: str, state: Any, reasons: list[str], row: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    if state == "narrowed_unsupported":
        allowed = {"line_capability_absent", "line_evidence_incomplete", "owner_signoff_missing"}
        if not allowed.intersection(reasons):
            findings.append(Finding("error", "row.state_reason_incoherent", "narrowed_unsupported requires a capability/evidence/signoff reason", boundary_id))
    if state == "narrowed_by_manifest" and "manifest_label_narrowed" not in reasons:
        findings.append(Finding("error", "row.state_reason_incoherent", "narrowed_by_manifest requires the manifest_label_narrowed reason", boundary_id))
    if state == "narrowed_stale" and not (
        "boundary_packet_freshness_breached" in reasons or "boundary_packet_missing" in reasons
    ):
        findings.append(Finding("error", "row.state_reason_incoherent", "narrowed_stale requires a packet-freshness reason", boundary_id))
    if state == "narrowed_waiver_expired":
        if "waiver_expired" not in reasons:
            findings.append(Finding("error", "row.state_reason_incoherent", "narrowed_waiver_expired requires the waiver_expired reason", boundary_id))
        if not isinstance(row.get("waiver"), dict):
            findings.append(Finding("error", "row.waiver_state_without_waiver", "narrowed_waiver_expired state must name a waiver", boundary_id))
    if state == "available_on_waiver":
        waiver = row.get("waiver")
        if not isinstance(waiver, dict) or not is_str(waiver.get("waiver_ref")) or not is_str(waiver.get("expires_at")):
            findings.append(Finding("error", "row.waiver_state_without_waiver", "available_on_waiver state must name a waiver", boundary_id))
    return findings


def validate_row_refs(boundary_id: str, row: dict[str, Any], repo_root: Path) -> list[Finding]:
    """Path-existence checks for the proof-packet refs (not the cross-artifact
    ids, which resolve against the claim manifest)."""
    findings: list[Finding] = []
    packet = row.get("boundary_packet")
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
    if is_str(row.get("line_capability_ref")):
        refs.append(row["line_capability_ref"].split("#", 1)[0])
    for ref in refs:
        if not (repo_root / ref).exists():
            findings.append(Finding("error", "row.ref_missing", f"referenced artifact does not exist: {ref}", boundary_id))
    return findings


def validate_coverage(manifest: dict[str, Any]) -> list[Finding]:
    """Every subject must carry exactly one row per value line, with a single
    consistent ceiling label."""
    findings: list[Finding] = []
    by_subject: dict[str, list[dict[str, Any]]] = {}
    for row in manifest.get("rows", []):
        by_subject.setdefault(str(row.get("manifest_entry_ref")), []).append(row)
    for subject, rows in by_subject.items():
        lines = [r.get("value_line") for r in rows]
        for line in VALUE_LINES:
            if lines.count(line) == 0:
                findings.append(Finding("error", "coverage.line_missing", f"subject has no row for value line {line}", subject))
            elif lines.count(line) > 1:
                findings.append(Finding("error", "coverage.line_duplicate", f"subject has more than one row for value line {line}", subject))
        labels = {r.get("manifest_label") for r in rows}
        if len(labels) > 1:
            findings.append(Finding("error", "coverage.label_inconsistent", "subject carries different manifest ceiling labels across its rows", subject))
    return findings


def validate_freshness(manifest: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    as_of = parse_date(manifest.get("as_of"))
    if as_of is None:
        return findings
    for row in manifest.get("rows", []):
        boundary_id = str(row.get("boundary_id", "<row>"))
        packet = row.get("boundary_packet")
        if not isinstance(packet, dict):
            continue
        declared = packet.get("slo_state")
        computed = computed_slo_state(packet, as_of)
        if computed is None or declared not in FRESHNESS_RANK or computed not in FRESHNESS_RANK:
            continue
        if FRESHNESS_RANK[declared] > FRESHNESS_RANK[computed]:
            findings.append(
                Finding("error", "row.packet_freshness_overstated", f"declared slo_state {declared!r} is fresher than the {computed!r} the as_of date allows", boundary_id)
            )
        if row.get("boundary_state") in HOLDING_STATES and computed in ("breached", "missing"):
            findings.append(
                Finding("error", "row.published_on_breached_packet", "a published line rides a proof packet past its freshness SLO against as_of", boundary_id)
            )
    return findings


def validate_waivers(manifest: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    as_of = parse_date(manifest.get("as_of"))
    if as_of is None:
        return findings
    for row in manifest.get("rows", []):
        boundary_id = str(row.get("boundary_id", "<row>"))
        state = row.get("boundary_state")
        waiver = row.get("waiver")
        if not isinstance(waiver, dict):
            continue
        expires = parse_date(waiver.get("expires_at"))
        if expires is None:
            findings.append(Finding("error", "row.waiver_expiry_invalid", "waiver expires_at must be an ISO date", boundary_id))
            continue
        expired = as_of >= expires
        if expired and state == "available_on_waiver":
            findings.append(Finding("error", "row.available_on_expired_waiver", "a line holds on a waiver that has expired against as_of", boundary_id))
        if not expired and state == "narrowed_waiver_expired":
            findings.append(Finding("error", "row.waiver_expired_but_active", "a row is marked waiver-expired but the waiver is still active against as_of", boundary_id))
    return findings


def validate_ceiling(manifest: dict[str, Any], repo_root: Path) -> list[Finding]:
    """Cross-artifact ingestion: read the stable claim manifest and fail when a
    row's manifest_label is not the label the claim manifest publishes for its
    entry, or names an entry the claim manifest does not carry."""
    findings: list[Finding] = []
    ref = manifest.get("claim_manifest_ref")
    if not is_str(ref):
        return findings
    path = repo_root / ref
    if not path.exists():
        findings.append(Finding("error", "ceiling.artifact_missing", f"claim_manifest_ref does not exist: {ref}", "<manifest>"))
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

    for row in manifest.get("rows", []):
        boundary_id = str(row.get("boundary_id", "<row>"))
        entry_ref = row.get("manifest_entry_ref")
        if not is_str(entry_ref):
            continue
        if entry_ref not in published_by_entry:
            findings.append(Finding("error", "ceiling.manifest_entry_unknown", f"manifest_entry_ref is not an entry in the stable claim manifest: {entry_ref}", boundary_id))
            continue
        canonical = published_by_entry[entry_ref]
        if row.get("manifest_label") != canonical:
            findings.append(Finding("error", "ceiling.manifest_label_mismatch", f"manifest_label {row.get('manifest_label')!r} does not match the claim manifest published_label {canonical!r}", boundary_id))
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
    decision, rule_ids, boundary_ids = computed_publication(manifest)
    if publication.get("decision") != decision:
        findings.append(
            Finding("error", "publication.decision_inconsistent", f"declared decision {publication.get('decision')!r} disagrees with computed {decision!r}", "<manifest>")
        )
    if list(publication.get("blocking_rule_ids", [])) != rule_ids:
        findings.append(Finding("error", "publication.blocking_rules_mismatch", "blocking_rule_ids disagrees with the firing boundary rules", "<manifest>"))
    if list(publication.get("blocking_boundary_ids", [])) != boundary_ids:
        findings.append(Finding("error", "publication.blocking_boundaries_mismatch", "blocking_boundary_ids disagrees with the firing boundary rules", "<manifest>"))
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
            findings.append(Finding("error", "summary.count_mismatch", f"summary.{key} must equal the recomputed value", key))
    return findings


def validate_manifest(manifest: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings = validate_envelope(manifest)
    findings.extend(validate_rules(manifest))

    rows = manifest.get("rows")
    if not isinstance(rows, list) or not rows:
        findings.append(Finding("error", "manifest.rows_empty", "manifest must enumerate at least one row", "<manifest>"))
        return findings

    seen: set[str] = set()
    for raw in rows:
        row = ensure_dict(raw, "manifest.rows[]")
        findings.extend(validate_row(row, repo_root))
        boundary_id = str(row.get("boundary_id", "<row>"))
        if boundary_id in seen:
            findings.append(Finding("error", "row.duplicate_id", "boundary ids must be unique", boundary_id))
        seen.add(boundary_id)

    findings.extend(validate_coverage(manifest))
    findings.extend(validate_freshness(manifest))
    findings.extend(validate_waivers(manifest))
    findings.extend(validate_ceiling(manifest, repo_root))
    findings.extend(validate_publication(manifest))
    findings.extend(validate_summary(manifest))
    return findings


def recompute_derived(manifest: dict[str, Any]) -> None:
    manifest["summary"] = computed_summary(manifest)
    decision, rule_ids, boundary_ids = computed_publication(manifest)
    if isinstance(manifest.get("publication"), dict):
        manifest["publication"].update(
            {"decision": decision, "blocking_rule_ids": rule_ids, "blocking_boundary_ids": boundary_ids}
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

    # A narrowing row that still publishes a stable label must be flagged.
    mutated = copy.deepcopy(manifest)
    target = next((r for r in mutated["rows"] if r.get("boundary_state") in NARROWING_STATES), None)
    if target is not None:
        target["published_label"] = "stable"
        recompute_derived(mutated)
        record("published_not_narrowed_rejected", "row.published_not_narrowed", "row.published_not_narrowed" in check_ids(mutated))

    # A published row carrying an active narrowing reason must be flagged.
    mutated = copy.deepcopy(manifest)
    target = next((r for r in mutated["rows"] if r.get("boundary_state") in HOLDING_STATES), None)
    if target is not None:
        target["active_narrowing_reasons"] = ["line_capability_absent"]
        recompute_derived(mutated)
        record("held_with_active_narrowing_rejected", "row.held_with_active_narrowing", "row.held_with_active_narrowing" in check_ids(mutated))

    # A published row whose packet is breached against as_of must be flagged.
    mutated = copy.deepcopy(manifest)
    target = next((r for r in mutated["rows"] if r.get("boundary_state") in HOLDING_STATES), None)
    if target is not None:
        target["boundary_packet"]["captured_at"] = "2000-01-01"
        record("published_on_breached_packet_rejected", "row.published_on_breached_packet", "row.published_on_breached_packet" in check_ids(mutated))

    # A packet whose declared state overstates its freshness must be flagged.
    mutated = copy.deepcopy(manifest)
    target = next(
        (r for r in mutated["rows"] if r.get("boundary_packet", {}).get("slo_state") == "current"),
        None,
    )
    if target is not None:
        target["boundary_packet"]["captured_at"] = "2000-01-01"
        record("packet_freshness_overstated_rejected", "row.packet_freshness_overstated", "row.packet_freshness_overstated" in check_ids(mutated))

    # A row whose manifest_label disagrees with the claim manifest must be flagged.
    mutated = copy.deepcopy(manifest)
    target = next((r for r in mutated["rows"] if r.get("manifest_label") == "beta"), None)
    if target is not None:
        target["manifest_label"] = "stable"
        record("manifest_label_mismatch_rejected", "ceiling.manifest_label_mismatch", "ceiling.manifest_label_mismatch" in check_ids(mutated))

    # A line published wider than its subject's ceiling must be flagged.
    mutated = copy.deepcopy(manifest)
    target = next(
        (r for r in mutated["rows"] if r.get("manifest_label") == "beta" and not is_above_cutline(r.get("published_label"))),
        None,
    )
    if target is not None:
        target["published_label"] = "lts"
        recompute_derived(mutated)
        record("published_wider_than_manifest_rejected", "row.published_wider_than_manifest", "row.published_wider_than_manifest" in check_ids(mutated))

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
    decision, rule_ids, boundary_ids = computed_publication(manifest)
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "record_kind": "stable_boundary_manifest_validation_capture",
        "status": "pass" if not any(f.severity == "error" for f in findings) else "fail",
        "generated_at": generated_at_now(),
        "manifest_id": manifest.get("manifest_id"),
        "as_of": manifest.get("as_of"),
        "summary": manifest.get("summary"),
        "publication": {
            "decision": decision,
            "blocking_rule_ids": rule_ids,
            "blocking_boundary_ids": boundary_ids,
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
        load_json(repo_root / args.manifest, "stable boundary manifest"),
        "stable boundary manifest",
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
    decision, rule_ids, _boundary_ids = computed_publication(manifest)
    rollups = summary.get("line_rollups", [])
    line_brief = ", ".join(
        f"{r.get('value_line')}={r.get('published_stable')}/{r.get('total')}" for r in rollups
    )
    print(
        "stable boundary manifest validated "
        f"({summary.get('total_boundaries')} boundaries across {summary.get('total_subjects')} subjects, "
        f"{summary.get('boundaries_published_stable')} published stable, "
        f"{summary.get('boundaries_narrowed_below_cutline')} narrowed; "
        f"per line stable/total: {line_brief}; "
        f"packets {summary.get('packets_current')} current / "
        f"{summary.get('packets_due_for_refresh')} due / "
        f"{summary.get('packets_breached')} breached / "
        f"{summary.get('packets_missing')} missing; "
        f"{summary.get('boundary_rules_firing')} rules firing; "
        f"publication={decision}; {len(drill_results)} negative drills, {len(fixture_results)} fixture cases)"
    )

    if args.require_proceed and decision == "hold":
        print(
            f"PUBLICATION HELD: stable boundary publication blocked by {len(rule_ids)} rule(s): {', '.join(rule_ids)}",
            file=sys.stderr,
        )
        return 2

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
