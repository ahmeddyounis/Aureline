#!/usr/bin/env python3
"""Validate the stable version-window freeze for the release line's CLI, schema,
API, and manifest surfaces, with deprecation packets.

The stable claim manifest decides the single canonical lifecycle label each
*subject* publishes, and the stable proof index decides whether each launch-blocking
*requirement* is proven. This freeze answers the interface-level question: which
version window does each public surface commit to for the release line, and is that
window actually frozen — backed by a fresh freeze packet, a complete deprecation
packet with no overdue removal, and an owner sign-off? It reads the checked-in
freeze at ``artifacts/release/stable_version_windows.json`` and:

  - asserts the closed vocabularies (lifecycle labels, surface kinds, compatibility
    postures, deprecation statuses, window states, gap reasons, window actions) and
    the launch cutline are canonical;
  - asserts every row that is narrowed (for an absent/incomplete surface, an
    incomplete deprecation packet, an overdue deprecation removal, a breached/missing
    freeze packet, an expired waiver, a missing owner sign-off, or a public claim
    whose canonical label is itself below the cutline) drops below the cutline rather
    than freezing a label wider than the public claim, and that a frozen row freezes
    the public claim's canonical label cleanly with a captured, within-SLO freeze
    packet, a complete deprecation packet, and an owner sign-off;
  - asserts each version window is ordered floor <= current <= ceiling;
  - asserts CLI/schema/API/manifest coverage: every surface kind is represented,
    every declared release-blocking surface ref has exactly one covering
    release-blocking row, every release-blocking row is declared, and no surface ref
    repeats;
  - performs the ceiling cross-check the typed model cannot — it reads the stable
    claim manifest named by ``claim_manifest_ref`` and fails when a row's
    ``claim_label`` is not the label the claim manifest publishes for the entry named
    by ``claim_ref``, or names an entry the claim manifest does not carry;
  - performs the packet-freshness SLO automation against the freeze ``as_of`` date,
    the waiver-expiry date arithmetic, and the deprecation-removal-overdue date
    arithmetic the typed model cannot;
  - recomputes the publication verdict and the summary block, and fails on any drift;
  - runs negative drills proving the narrowing, ceiling, freshness, deprecation,
    waiver, and publication rejections all fire; and
  - runs the checked-in fixture cases under
    ``fixtures/release/stable_version_windows`` and fails when a case the freeze
    marks as rejected validates clean.

With ``--require-proceed`` the gate additionally fails (exit code 2) when the
recomputed publication verdict is ``hold``, so shiproom and release tooling can block
stable version-window publication directly from this artifact.

The typed Rust consumer
(``aureline_release::stable_version_windows::current_stable_version_windows``) reads
the same freeze and runs the same structural cross-check, so this gate and
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


DEFAULT_FREEZE_REL = "artifacts/release/stable_version_windows.json"
DEFAULT_REPORT_REL = (
    "artifacts/release/captures/stable_version_windows_validation_capture.json"
)
DEFAULT_FIXTURES_REL = "fixtures/release/stable_version_windows"

EXPECTED_SCHEMA_VERSION = 1
FREEZE_RECORD_KIND = "stable_version_windows"

LIFECYCLE_LABELS = ("lts", "stable", "beta", "preview", "withdrawn")
ABOVE_CUTLINE = ("lts", "stable")
BELOW_CUTLINE = ("beta", "preview", "withdrawn")
SURFACE_KINDS = ("cli", "schema", "api", "manifest")
COMPATIBILITY_POSTURES = (
    "backward_compatible",
    "additive_only",
    "frozen_no_change",
    "breaking_major_only",
)
DEPRECATION_STATUSES = ("announced", "migration_available", "removed")
FRESHNESS_SLO_STATES = ("current", "due_for_refresh", "breached", "missing")
WINDOW_STATES = (
    "frozen",
    "frozen_on_waiver",
    "unfrozen_unbacked",
    "unfrozen_claim_narrowed",
    "unfrozen_stale",
    "unfrozen_waiver_expired",
    "unfrozen_deprecation_overdue",
)
GAP_REASONS = (
    "claim_label_narrowed",
    "surface_capability_absent",
    "freeze_evidence_incomplete",
    "deprecation_packet_incomplete",
    "deprecation_removal_overdue",
    "freeze_packet_freshness_breached",
    "freeze_packet_missing",
    "waiver_expired",
    "owner_signoff_missing",
)
WINDOW_ACTIONS = (
    "hold_publication",
    "narrow_window_label",
    "refresh_freeze_packet",
    "complete_deprecation_packet",
    "recapture_surface_evidence",
    "request_owner_signoff",
)
HOLDING_STATES = ("frozen", "frozen_on_waiver")
NARROWING_STATES = (
    "unfrozen_unbacked",
    "unfrozen_claim_narrowed",
    "unfrozen_stale",
    "unfrozen_waiver_expired",
    "unfrozen_deprecation_overdue",
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
    parser.add_argument("--freeze", default=DEFAULT_FREEZE_REL)
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


def parse_version(value: Any) -> tuple[int, ...] | None:
    if not is_str(value):
        return None
    parts: list[int] = []
    for part in value.split("."):
        if not part.isdigit():
            return None
        parts.append(int(part))
    return tuple(parts)


def is_above_cutline(level: Any) -> bool:
    return isinstance(level, str) and LEVEL_RANK.get(level, -1) >= CUTLINE_RANK


def has_capture(packet: dict[str, Any]) -> bool:
    return bool(packet.get("captured_at")) and bool(packet.get("evidence_refs"))


def reasons_of(row: dict[str, Any]) -> list[str]:
    reasons = row.get("active_gap_reasons", [])
    return reasons if isinstance(reasons, list) else []


def deprecations_of(row: dict[str, Any]) -> list[dict[str, Any]]:
    packet = row.get("deprecation_packet")
    if not isinstance(packet, dict):
        return []
    items = packet.get("deprecations", [])
    return [d for d in items if isinstance(d, dict)] if isinstance(items, list) else []


def notice_is_complete(notice: dict[str, Any]) -> bool:
    return all(
        is_str(notice.get(field))
        for field in (
            "deprecated_version",
            "superseded_by",
            "announced_at",
            "removal_target_version",
            "removal_target_date",
            "migration_ref",
        )
    )


def packet_has_incomplete_notice(row: dict[str, Any]) -> bool:
    return any(not notice_is_complete(n) for n in deprecations_of(row))


def freeze_rule_fires(rule: dict[str, Any], rows: list[dict[str, Any]]) -> bool:
    trigger = rule.get("trigger_reason")
    labels = rule.get("applies_to_labels", [])
    if not isinstance(labels, list):
        return False
    for row in rows:
        if row.get("claim_label") in labels and trigger in reasons_of(row):
            return True
    return False


def computed_publication(freeze: dict[str, Any]) -> tuple[str, list[str], list[str]]:
    rows = freeze.get("rows", [])
    rules = freeze.get("freeze_rules", [])
    blocking_rules = [
        rule
        for rule in rules
        if rule.get("blocks_publication") is True and freeze_rule_fires(rule, rows)
    ]
    rule_ids = sorted(str(rule.get("rule_id")) for rule in blocking_rules)
    blocking_triggers = {rule.get("trigger_reason") for rule in blocking_rules}
    window_ids: set[str] = set()
    for row in rows:
        if is_above_cutline(row.get("claim_label")) and blocking_triggers.intersection(
            reasons_of(row)
        ):
            window_ids.add(str(row.get("window_id")))
    decision = "hold" if blocking_rules else "proceed"
    return decision, rule_ids, sorted(window_ids)


def computed_summary(freeze: dict[str, Any]) -> dict[str, Any]:
    rows = freeze.get("rows", [])
    rules = freeze.get("freeze_rules", [])

    def packet_state(row: dict[str, Any]) -> Any:
        packet = row.get("freeze_packet")
        return packet.get("slo_state") if isinstance(packet, dict) else None

    def kind_count(kind: str) -> int:
        return sum(1 for r in rows if r.get("surface_kind") == kind)

    frozen = [r for r in rows if is_above_cutline(r.get("frozen_label"))]
    claims = {str(r.get("claim_ref")) for r in rows}
    release_blocking = [r for r in rows if r.get("release_blocking") is True]
    rb_frozen = [r for r in release_blocking if is_above_cutline(r.get("frozen_label"))]
    return {
        "total_surfaces": len(rows),
        "total_claims": len(claims),
        "surfaces_frozen_stable": len(frozen),
        "surfaces_narrowed_below_cutline": len(rows) - len(frozen),
        "surfaces_on_active_waiver": sum(
            1 for r in rows if r.get("window_state") == "frozen_on_waiver"
        ),
        "release_blocking_total": len(release_blocking),
        "release_blocking_frozen_stable": len(rb_frozen),
        "release_blocking_unfrozen": len(release_blocking) - len(rb_frozen),
        "cli_surfaces": kind_count("cli"),
        "schema_surfaces": kind_count("schema"),
        "api_surfaces": kind_count("api"),
        "manifest_surfaces": kind_count("manifest"),
        "packets_current": sum(1 for r in rows if packet_state(r) == "current"),
        "packets_due_for_refresh": sum(1 for r in rows if packet_state(r) == "due_for_refresh"),
        "packets_breached": sum(1 for r in rows if packet_state(r) == "breached"),
        "packets_missing": sum(1 for r in rows if packet_state(r) == "missing"),
        "total_deprecations": sum(len(deprecations_of(r)) for r in rows),
        "surfaces_deprecation_overdue": sum(
            1 for r in rows if r.get("window_state") == "unfrozen_deprecation_overdue"
        ),
        "total_active_gap_reasons": sum(len(reasons_of(r)) for r in rows),
        "freeze_rules_firing": sum(1 for rule in rules if freeze_rule_fires(rule, rows)),
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


def validate_envelope(freeze: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    freeze_id = str(freeze.get("freeze_id", "<freeze>"))

    if freeze.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(Finding("error", "freeze.schema_version", "schema_version must be 1", freeze_id))
    if freeze.get("record_kind") != FREEZE_RECORD_KIND:
        findings.append(Finding("error", "freeze.record_kind", "record_kind is not supported", freeze_id))
    for field in ("freeze_id", "status", "overview_page", "as_of", "claim_manifest_ref"):
        if not is_str(freeze.get(field)):
            findings.append(
                Finding("error", "freeze.empty_field", f"{field} must be a non-empty string", freeze_id)
            )
    if parse_date(freeze.get("as_of")) is None:
        findings.append(Finding("error", "freeze.as_of_invalid", "as_of must be an ISO date", freeze_id))

    for key, expected in (
        ("lifecycle_labels", list(LIFECYCLE_LABELS)),
        ("surface_kinds", list(SURFACE_KINDS)),
        ("compatibility_postures", list(COMPATIBILITY_POSTURES)),
        ("deprecation_statuses", list(DEPRECATION_STATUSES)),
        ("window_states", list(WINDOW_STATES)),
        ("gap_reasons", list(GAP_REASONS)),
        ("window_actions", list(WINDOW_ACTIONS)),
    ):
        if list(freeze.get(key, [])) != expected:
            findings.append(
                Finding("error", "freeze.vocabulary", f"freeze.{key} is not the closed vocabulary", key)
            )

    refs = freeze.get("release_blocking_surface_refs")
    if not isinstance(refs, list) or not refs or any(not is_str(r) for r in refs):
        findings.append(
            Finding(
                "error",
                "freeze.release_blocking_refs",
                "release_blocking_surface_refs must be a non-empty list of non-empty strings",
                freeze_id,
            )
        )

    cutline = freeze.get("launch_cutline")
    if not isinstance(cutline, dict):
        findings.append(Finding("error", "cutline.missing", "freeze must carry a launch_cutline", freeze_id))
    else:
        if cutline.get("cutline_level") != "stable":
            findings.append(Finding("error", "cutline.level", "cutline_level must be stable", freeze_id))
        if list(cutline.get("above_cutline_levels", [])) != list(ABOVE_CUTLINE):
            findings.append(Finding("error", "cutline.above", "above_cutline_levels is not canonical", freeze_id))
        if list(cutline.get("below_cutline_levels", [])) != list(BELOW_CUTLINE):
            findings.append(Finding("error", "cutline.below", "below_cutline_levels is not canonical", freeze_id))
        if not is_str(cutline.get("description")):
            findings.append(Finding("error", "cutline.description", "cutline description must be non-empty", freeze_id))
    return findings


def validate_rules(freeze: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    rules = freeze.get("freeze_rules")
    if not isinstance(rules, list) or not rules:
        findings.append(Finding("error", "rules.empty", "freeze must enumerate at least one freeze rule", "<freeze>"))
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
        if rule.get("default_action") not in WINDOW_ACTIONS:
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


def validate_packet(window_id: str, packet: Any) -> list[Finding]:
    findings: list[Finding] = []
    if not isinstance(packet, dict):
        findings.append(Finding("error", "row.packet_missing", "row must carry a freeze_packet block", window_id))
        return findings
    for field in ("packet_id", "packet_ref", "proof_index_ref"):
        if not is_str(packet.get(field)):
            findings.append(Finding("error", "packet.empty_field", f"freeze_packet.{field} must be non-empty", window_id))
    if packet.get("slo_state") not in FRESHNESS_SLO_STATES:
        findings.append(Finding("error", "packet.state_invalid", "slo_state is outside the vocabulary", window_id))
    slo = packet.get("freshness_slo")
    if not isinstance(slo, dict):
        findings.append(Finding("error", "packet.slo_missing", "freeze_packet must carry a freshness_slo block", window_id))
        return findings
    if not is_str(slo.get("slo_register_ref")):
        findings.append(Finding("error", "packet.slo_register_missing", "freshness_slo.slo_register_ref must be non-empty", window_id))
    target = slo.get("target_max_age_days")
    warn = slo.get("warn_within_days")
    if not (isinstance(target, int) and not isinstance(target, bool) and target >= 1):
        findings.append(Finding("error", "packet.target_invalid", "target_max_age_days must be an integer >= 1", window_id))
    if not (isinstance(warn, int) and not isinstance(warn, bool) and warn >= 0):
        findings.append(Finding("error", "packet.warn_invalid", "warn_within_days must be an integer >= 0", window_id))
    if (
        isinstance(target, int)
        and not isinstance(target, bool)
        and isinstance(warn, int)
        and not isinstance(warn, bool)
        and warn > target
    ):
        findings.append(Finding("error", "packet.slo_inconsistent", "warn_within_days may not exceed target_max_age_days", window_id))
    return findings


def validate_version_window(window_id: str, window: Any) -> list[Finding]:
    findings: list[Finding] = []
    if not isinstance(window, dict):
        findings.append(Finding("error", "row.version_window_missing", "row must carry a version_window block", window_id))
        return findings
    for field in ("floor_version", "current_version", "ceiling_version"):
        if not is_str(window.get(field)):
            findings.append(Finding("error", "window.empty_field", f"version_window.{field} must be non-empty", window_id))
    if window.get("compatibility_posture") not in COMPATIBILITY_POSTURES:
        findings.append(Finding("error", "window.posture_invalid", "compatibility_posture is outside the vocabulary", window_id))
    floor = parse_version(window.get("floor_version"))
    current = parse_version(window.get("current_version"))
    ceiling = parse_version(window.get("ceiling_version"))
    if floor is not None and current is not None and ceiling is not None:
        if not (floor <= current <= ceiling):
            findings.append(Finding("error", "row.version_window_disordered", "version window is not ordered floor <= current <= ceiling", window_id))
    return findings


def validate_deprecation_packet(window_id: str, row: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    packet = row.get("deprecation_packet")
    if not isinstance(packet, dict):
        findings.append(Finding("error", "row.deprecation_packet_missing", "row must carry a deprecation_packet block", window_id))
        return findings
    if not is_str(packet.get("packet_id")):
        findings.append(Finding("error", "deprecation.packet_id_missing", "deprecation_packet.packet_id must be non-empty", window_id))

    reasons = reasons_of(row)
    notices = deprecations_of(row)
    for idx, notice in enumerate(notices):
        if not is_str(notice.get("deprecated_version")):
            findings.append(Finding("error", "deprecation.missing_version", f"deprecation #{idx} must name the deprecated version", window_id))
        if notice.get("status") not in DEPRECATION_STATUSES:
            findings.append(Finding("error", "deprecation.status_invalid", f"deprecation #{idx} status is outside the vocabulary", window_id))

    has_incomplete = packet_has_incomplete_notice(row)
    if "deprecation_packet_incomplete" in reasons and not has_incomplete:
        findings.append(Finding("error", "row.deprecation_reason_without_incomplete", "names deprecation_packet_incomplete but every notice is complete", window_id))
    if has_incomplete and "deprecation_packet_incomplete" not in reasons:
        findings.append(Finding("error", "row.incomplete_deprecation_without_reason", "has an incomplete deprecation notice but does not name deprecation_packet_incomplete", window_id))
    return findings


def validate_row(row: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings: list[Finding] = []
    window_id = str(row.get("window_id", "<row>"))

    for field in (
        "window_id",
        "title",
        "surface_ref",
        "surface_summary",
        "claim_ref",
        "rationale",
    ):
        if not is_str(row.get(field)):
            findings.append(Finding("error", "row.empty_field", f"row {field} must be non-empty", window_id))

    if row.get("surface_kind") not in SURFACE_KINDS:
        findings.append(Finding("error", "row.surface_kind_invalid", "surface_kind is outside the vocabulary", window_id))
    if not isinstance(row.get("release_blocking"), bool):
        findings.append(Finding("error", "row.release_blocking_invalid", "release_blocking must be a boolean", window_id))

    claim_label = row.get("claim_label")
    frozen = row.get("frozen_label")
    state = row.get("window_state")
    reasons = reasons_of(row)

    if claim_label not in LIFECYCLE_LABELS:
        findings.append(Finding("error", "row.claim_label_invalid", "claim_label is invalid", window_id))
    if frozen not in LIFECYCLE_LABELS:
        findings.append(Finding("error", "row.frozen_invalid", "frozen_label is invalid", window_id))
    if state not in WINDOW_STATES:
        findings.append(Finding("error", "row.state_invalid", "window_state is invalid", window_id))
    if any(reason not in GAP_REASONS for reason in reasons):
        findings.append(Finding("error", "row.reason_invalid", "active_gap_reasons has an unknown reason", window_id))

    # The ceiling: no freeze may back a label wider than the public claim's label.
    if claim_label in LEVEL_RANK and frozen in LEVEL_RANK and LEVEL_RANK[frozen] > LEVEL_RANK[claim_label]:
        findings.append(Finding("error", "row.frozen_wider_than_claim", "frozen_label is wider than the claim ceiling", window_id))

    findings.extend(validate_version_window(window_id, row.get("version_window")))
    findings.extend(validate_deprecation_packet(window_id, row))

    packet = row.get("freeze_packet")
    findings.extend(validate_packet(window_id, packet))
    slo_state = packet.get("slo_state") if isinstance(packet, dict) else None

    holds = state in HOLDING_STATES
    claim_holds = is_above_cutline(claim_label)

    # A claim canonically below the cutline forces the freeze to inherit the ceiling
    # and narrow.
    if claim_label in LEVEL_RANK and not claim_holds:
        if holds:
            findings.append(Finding("error", "row.held_on_narrowed_claim", "a row holds a freeze while the public claim label is below the cutline", window_id))
        if "claim_label_narrowed" not in reasons:
            findings.append(Finding("error", "row.claim_narrowed_without_reason", "a row whose claim is narrowed must name claim_label_narrowed", window_id))

    if state in NARROWING_STATES:
        if is_above_cutline(frozen):
            findings.append(Finding("error", "row.frozen_not_narrowed", "a surface that is not frozen must narrow below the cutline", window_id))
        if not reasons:
            findings.append(Finding("error", "row.narrowing_without_reason", "a narrowing row must name an active gap reason", window_id))
        if slo_state == "breached" and "freeze_packet_freshness_breached" not in reasons:
            findings.append(Finding("error", "row.breached_without_reason", "a breached packet must name freeze_packet_freshness_breached", window_id))
        if slo_state == "missing" and "freeze_packet_missing" not in reasons:
            findings.append(Finding("error", "row.missing_without_reason", "a missing packet must name freeze_packet_missing", window_id))
    if holds:
        if claim_label != frozen:
            findings.append(Finding("error", "row.held_label_not_equal_claim", "a frozen row must back the public claim's canonical label", window_id))
        if reasons:
            findings.append(Finding("error", "row.held_with_active_gap", "a frozen row must carry no active gap reason", window_id))
        if isinstance(packet, dict) and not has_capture(packet):
            findings.append(Finding("error", "row.held_without_fresh_packet", "a frozen row must ride a captured, evidence-backed freeze packet", window_id))
        if slo_state in ("breached", "missing"):
            findings.append(Finding("error", "row.held_on_stale_packet", "a frozen row must ride a packet within its freshness SLO", window_id))
        if packet_has_incomplete_notice(row):
            findings.append(Finding("error", "row.held_with_incomplete_deprecation", "a frozen row must carry a complete deprecation packet", window_id))
        signoff = row.get("owner_signoff", {})
        if not (isinstance(signoff, dict) and signoff.get("signed_off") is True and is_str(signoff.get("signed_at"))):
            findings.append(Finding("error", "row.held_without_signoff", "a frozen row must carry an owner sign-off with a date", window_id))
    signoff = row.get("owner_signoff", {})
    if not (isinstance(signoff, dict) and is_str(signoff.get("owner_ref"))):
        findings.append(Finding("error", "row.signoff_owner_missing", "owner_signoff.owner_ref must be non-empty", window_id))

    findings.extend(validate_state_reason_coherence(window_id, state, reasons, row))
    findings.extend(validate_row_refs(window_id, row, repo_root))
    return findings


def validate_state_reason_coherence(window_id: str, state: Any, reasons: list[str], row: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    if state == "unfrozen_unbacked":
        allowed = {
            "surface_capability_absent",
            "freeze_evidence_incomplete",
            "deprecation_packet_incomplete",
            "owner_signoff_missing",
        }
        if not allowed.intersection(reasons):
            findings.append(Finding("error", "row.state_reason_incoherent", "unfrozen_unbacked requires a capability/evidence/deprecation/signoff reason", window_id))
    if state == "unfrozen_claim_narrowed" and "claim_label_narrowed" not in reasons:
        findings.append(Finding("error", "row.state_reason_incoherent", "unfrozen_claim_narrowed requires the claim_label_narrowed reason", window_id))
    if state == "unfrozen_stale" and not (
        "freeze_packet_freshness_breached" in reasons or "freeze_packet_missing" in reasons
    ):
        findings.append(Finding("error", "row.state_reason_incoherent", "unfrozen_stale requires a packet-freshness reason", window_id))
    if state == "unfrozen_waiver_expired":
        if "waiver_expired" not in reasons:
            findings.append(Finding("error", "row.state_reason_incoherent", "unfrozen_waiver_expired requires the waiver_expired reason", window_id))
        if not isinstance(row.get("waiver"), dict):
            findings.append(Finding("error", "row.waiver_state_without_waiver", "unfrozen_waiver_expired state must name a waiver", window_id))
    if state == "unfrozen_deprecation_overdue":
        if "deprecation_removal_overdue" not in reasons:
            findings.append(Finding("error", "row.state_reason_incoherent", "unfrozen_deprecation_overdue requires the deprecation_removal_overdue reason", window_id))
        if not any(n.get("status") != "removed" for n in deprecations_of(row)):
            findings.append(Finding("error", "row.overdue_state_without_notice", "unfrozen_deprecation_overdue state names no still-present deprecation notice", window_id))
    if state == "frozen_on_waiver":
        waiver = row.get("waiver")
        if not isinstance(waiver, dict) or not is_str(waiver.get("waiver_ref")) or not is_str(waiver.get("expires_at")):
            findings.append(Finding("error", "row.waiver_state_without_waiver", "frozen_on_waiver state must name a waiver", window_id))
    return findings


def validate_row_refs(window_id: str, row: dict[str, Any], repo_root: Path) -> list[Finding]:
    """Path-existence checks for the freeze-packet and migration refs (not the
    cross-artifact ids, which resolve against the claim manifest)."""
    findings: list[Finding] = []
    refs: list[str] = []
    packet = row.get("freeze_packet")
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
    for notice in deprecations_of(row):
        ref = notice.get("migration_ref")
        if is_str(ref):
            refs.append(ref.split("#", 1)[0])
    for ref in refs:
        if not (repo_root / ref).exists():
            findings.append(Finding("error", "row.ref_missing", f"referenced artifact does not exist: {ref}", window_id))
    return findings


def validate_coverage(freeze: dict[str, Any]) -> list[Finding]:
    """Every surface kind is represented, every declared release-blocking surface is
    covered by exactly one release-blocking row, every release-blocking row is
    declared, and no surface ref repeats."""
    findings: list[Finding] = []
    rows = freeze.get("rows", [])

    seen: set[str] = set()
    for row in rows:
        ref = row.get("surface_ref")
        if not is_str(ref):
            continue
        if ref in seen:
            findings.append(Finding("error", "coverage.surface_duplicate", f"surface ref appears more than once: {ref}", ref))
        seen.add(ref)

    declared = freeze.get("release_blocking_surface_refs", [])
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
            findings.append(Finding("error", "coverage.release_blocking_not_declared", "a release-blocking row's surface is not in release_blocking_surface_refs", str(row.get("window_id"))))

    present_kinds = {row.get("surface_kind") for row in rows}
    for kind in SURFACE_KINDS:
        if kind not in present_kinds:
            findings.append(Finding("error", "coverage.surface_kind_absent", f"surface kind has no window row: {kind}", kind))
    return findings


def validate_freshness(freeze: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    as_of = parse_date(freeze.get("as_of"))
    if as_of is None:
        return findings
    for row in freeze.get("rows", []):
        window_id = str(row.get("window_id", "<row>"))
        packet = row.get("freeze_packet")
        if not isinstance(packet, dict):
            continue
        declared = packet.get("slo_state")
        computed = computed_slo_state(packet, as_of)
        if computed is None or declared not in FRESHNESS_RANK or computed not in FRESHNESS_RANK:
            continue
        if FRESHNESS_RANK[declared] > FRESHNESS_RANK[computed]:
            findings.append(
                Finding("error", "row.packet_freshness_overstated", f"declared slo_state {declared!r} is fresher than the {computed!r} the as_of date allows", window_id)
            )
        if row.get("window_state") in HOLDING_STATES and computed in ("breached", "missing"):
            findings.append(
                Finding("error", "row.frozen_on_breached_packet", "a frozen surface rides a freeze packet past its freshness SLO against as_of", window_id)
            )
    return findings


def validate_waivers(freeze: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    as_of = parse_date(freeze.get("as_of"))
    if as_of is None:
        return findings
    for row in freeze.get("rows", []):
        window_id = str(row.get("window_id", "<row>"))
        state = row.get("window_state")
        waiver = row.get("waiver")
        if not isinstance(waiver, dict):
            continue
        expires = parse_date(waiver.get("expires_at"))
        if expires is None:
            findings.append(Finding("error", "row.waiver_expiry_invalid", "waiver expires_at must be an ISO date", window_id))
            continue
        expired = as_of >= expires
        if expired and state == "frozen_on_waiver":
            findings.append(Finding("error", "row.frozen_on_expired_waiver", "a surface freezes on a waiver that has expired against as_of", window_id))
        if not expired and state == "unfrozen_waiver_expired":
            findings.append(Finding("error", "row.waiver_expired_but_active", "a row is marked waiver-expired but the waiver is still active against as_of", window_id))
    return findings


def validate_deprecation_dates(freeze: dict[str, Any]) -> list[Finding]:
    """Deprecation-removal-overdue date arithmetic: a deprecation whose removal
    target date has passed against as_of without removal makes the surface overdue."""
    findings: list[Finding] = []
    as_of = parse_date(freeze.get("as_of"))
    if as_of is None:
        return findings
    for row in freeze.get("rows", []):
        window_id = str(row.get("window_id", "<row>"))
        reasons = reasons_of(row)
        state = row.get("window_state")
        any_overdue = False
        for idx, notice in enumerate(deprecations_of(row)):
            if is_str(notice.get("announced_at")) and parse_date(notice.get("announced_at")) is None:
                findings.append(Finding("error", "row.deprecation_date_invalid", f"deprecation #{idx} announced_at must be an ISO date", window_id))
            removal = notice.get("removal_target_date")
            if is_str(removal):
                removal_date = parse_date(removal)
                if removal_date is None:
                    findings.append(Finding("error", "row.deprecation_date_invalid", f"deprecation #{idx} removal_target_date must be an ISO date", window_id))
                    continue
                if as_of >= removal_date and notice.get("status") != "removed":
                    any_overdue = True
        if any_overdue:
            if "deprecation_removal_overdue" not in reasons:
                findings.append(Finding("error", "row.deprecation_overdue_undeclared", "a surface has a deprecation past its removal target but does not name deprecation_removal_overdue", window_id))
            if state in HOLDING_STATES:
                findings.append(Finding("error", "row.frozen_on_overdue_removal", "a frozen surface carries a deprecation whose removal is overdue against as_of", window_id))
        elif "deprecation_removal_overdue" in reasons:
            findings.append(Finding("error", "row.deprecation_declared_overdue_but_current", "a row names deprecation_removal_overdue but no removal is overdue against as_of", window_id))
    return findings


def validate_ceiling(freeze: dict[str, Any], repo_root: Path) -> list[Finding]:
    """Cross-artifact ingestion: read the stable claim manifest and fail when a
    row's claim_label is not the label the claim manifest publishes for the entry
    named by claim_ref, or names an entry the claim manifest does not carry."""
    findings: list[Finding] = []
    ref = freeze.get("claim_manifest_ref")
    if not is_str(ref):
        return findings
    path = repo_root / ref
    if not path.exists():
        findings.append(Finding("error", "ceiling.artifact_missing", f"claim_manifest_ref does not exist: {ref}", "<freeze>"))
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

    for row in freeze.get("rows", []):
        window_id = str(row.get("window_id", "<row>"))
        claim_ref = row.get("claim_ref")
        if not is_str(claim_ref):
            continue
        if claim_ref not in published_by_entry:
            findings.append(Finding("error", "ceiling.claim_entry_unknown", f"claim_ref is not an entry in the stable claim manifest: {claim_ref}", window_id))
            continue
        canonical = published_by_entry[claim_ref]
        if row.get("claim_label") != canonical:
            findings.append(Finding("error", "ceiling.claim_label_mismatch", f"claim_label {row.get('claim_label')!r} does not match the claim manifest published_label {canonical!r}", window_id))
    return findings


def validate_publication(freeze: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    publication = freeze.get("publication")
    if not isinstance(publication, dict):
        findings.append(Finding("error", "publication.missing", "freeze must carry a publication block", "<freeze>"))
        return findings
    for field in ("publication_gate", "rationale"):
        if not is_str(publication.get(field)):
            findings.append(Finding("error", "publication.empty_field", f"publication.{field} must be non-empty", "<freeze>"))
    decision, rule_ids, window_ids = computed_publication(freeze)
    if publication.get("decision") != decision:
        findings.append(
            Finding("error", "publication.decision_inconsistent", f"declared decision {publication.get('decision')!r} disagrees with computed {decision!r}", "<freeze>")
        )
    if list(publication.get("blocking_rule_ids", [])) != rule_ids:
        findings.append(Finding("error", "publication.blocking_rules_mismatch", "blocking_rule_ids disagrees with the firing freeze rules", "<freeze>"))
    if list(publication.get("blocking_window_ids", [])) != window_ids:
        findings.append(Finding("error", "publication.blocking_windows_mismatch", "blocking_window_ids disagrees with the firing freeze rules", "<freeze>"))
    return findings


def validate_summary(freeze: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    summary = freeze.get("summary")
    if not isinstance(summary, dict):
        findings.append(Finding("error", "summary.missing", "freeze must carry a summary block", "<freeze>"))
        return findings
    expected = computed_summary(freeze)
    for key, value in expected.items():
        if summary.get(key) != value:
            findings.append(Finding("error", "summary.count_mismatch", f"summary.{key} must equal the recomputed value", key))
    return findings


def validate_freeze(freeze: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings = validate_envelope(freeze)
    findings.extend(validate_rules(freeze))

    rows = freeze.get("rows")
    if not isinstance(rows, list) or not rows:
        findings.append(Finding("error", "freeze.rows_empty", "freeze must enumerate at least one row", "<freeze>"))
        return findings

    seen: set[str] = set()
    for raw in rows:
        row = ensure_dict(raw, "freeze.rows[]")
        findings.extend(validate_row(row, repo_root))
        window_id = str(row.get("window_id", "<row>"))
        if window_id in seen:
            findings.append(Finding("error", "row.duplicate_id", "window ids must be unique", window_id))
        seen.add(window_id)

    findings.extend(validate_coverage(freeze))
    findings.extend(validate_freshness(freeze))
    findings.extend(validate_waivers(freeze))
    findings.extend(validate_deprecation_dates(freeze))
    findings.extend(validate_ceiling(freeze, repo_root))
    findings.extend(validate_publication(freeze))
    findings.extend(validate_summary(freeze))
    return findings


def recompute_derived(freeze: dict[str, Any]) -> None:
    freeze["summary"] = computed_summary(freeze)
    decision, rule_ids, window_ids = computed_publication(freeze)
    if isinstance(freeze.get("publication"), dict):
        freeze["publication"].update(
            {"decision": decision, "blocking_rule_ids": rule_ids, "blocking_window_ids": window_ids}
        )


def run_negative_drills(freeze: dict[str, Any], repo_root: Path) -> tuple[list[dict[str, str]], list[Finding]]:
    results: list[dict[str, str]] = []
    findings: list[Finding] = []

    def record(drill_id: str, expected_check_id: str, fired: bool) -> None:
        results.append({"drill_id": drill_id, "expected_check_id": expected_check_id, "status": "passed" if fired else "failed"})
        if not fired:
            findings.append(Finding("error", "negative_drill.not_rejected", f"negative drill {drill_id} did not fire", drill_id))

    def check_ids(candidate: dict[str, Any]) -> set[str]:
        return {f.check_id for f in validate_freeze(candidate, repo_root)}

    # A narrowing row that still freezes a stable label must be flagged.
    mutated = copy.deepcopy(freeze)
    target = next((r for r in mutated["rows"] if r.get("window_state") in NARROWING_STATES), None)
    if target is not None:
        target["frozen_label"] = "stable"
        recompute_derived(mutated)
        record("frozen_not_narrowed_rejected", "row.frozen_not_narrowed", "row.frozen_not_narrowed" in check_ids(mutated))

    # A frozen row carrying an active gap reason must be flagged.
    mutated = copy.deepcopy(freeze)
    target = next((r for r in mutated["rows"] if r.get("window_state") in HOLDING_STATES), None)
    if target is not None:
        target["active_gap_reasons"] = ["freeze_evidence_incomplete"]
        recompute_derived(mutated)
        record("held_with_active_gap_rejected", "row.held_with_active_gap", "row.held_with_active_gap" in check_ids(mutated))

    # A frozen row whose packet is breached against as_of must be flagged.
    mutated = copy.deepcopy(freeze)
    target = next((r for r in mutated["rows"] if r.get("window_state") in HOLDING_STATES), None)
    if target is not None:
        target["freeze_packet"]["captured_at"] = "2000-01-01"
        record("frozen_on_breached_packet_rejected", "row.frozen_on_breached_packet", "row.frozen_on_breached_packet" in check_ids(mutated))

    # A packet whose declared state overstates its freshness must be flagged.
    mutated = copy.deepcopy(freeze)
    target = next(
        (r for r in mutated["rows"] if r.get("freeze_packet", {}).get("slo_state") == "current"),
        None,
    )
    if target is not None:
        target["freeze_packet"]["captured_at"] = "2000-01-01"
        record("packet_freshness_overstated_rejected", "row.packet_freshness_overstated", "row.packet_freshness_overstated" in check_ids(mutated))

    # A row whose claim_label disagrees with the claim manifest must be flagged.
    mutated = copy.deepcopy(freeze)
    target = next((r for r in mutated["rows"] if r.get("claim_label") == "beta"), None)
    if target is not None:
        target["claim_label"] = "stable"
        record("claim_label_mismatch_rejected", "ceiling.claim_label_mismatch", "ceiling.claim_label_mismatch" in check_ids(mutated))

    # A freeze backed wider than its public claim's ceiling must be flagged.
    mutated = copy.deepcopy(freeze)
    target = next(
        (r for r in mutated["rows"] if r.get("claim_label") == "beta" and not is_above_cutline(r.get("frozen_label"))),
        None,
    )
    if target is not None:
        target["frozen_label"] = "lts"
        recompute_derived(mutated)
        record("frozen_wider_than_claim_rejected", "row.frozen_wider_than_claim", "row.frozen_wider_than_claim" in check_ids(mutated))

    # Introducing an overdue deprecation removal without declaring it must be flagged.
    mutated = copy.deepcopy(freeze)
    target = next((r for r in mutated["rows"] if r.get("window_state") == "frozen"), None)
    if target is not None:
        target["deprecation_packet"]["deprecations"] = [
            {
                "deprecated_version": "0.1.0",
                "superseded_by": "1.0.0",
                "announced_at": "2025-01-01",
                "removal_target_version": "1.0.0",
                "removal_target_date": "2025-06-01",
                "migration_ref": "docs/release/deprecation_packet_template.md",
                "status": "announced",
            }
        ]
        record("deprecation_overdue_undeclared_rejected", "row.deprecation_overdue_undeclared", "row.deprecation_overdue_undeclared" in check_ids(mutated))

    # Dropping a declared release-blocking surface's row must be flagged.
    mutated = copy.deepcopy(freeze)
    declared = mutated.get("release_blocking_surface_refs", [])
    if declared:
        dropped = declared[0]
        mutated["rows"] = [r for r in mutated["rows"] if r.get("surface_ref") != dropped]
        recompute_derived(mutated)
        record("release_blocking_uncovered_rejected", "coverage.release_blocking_uncovered", "coverage.release_blocking_uncovered" in check_ids(mutated))

    # Removing every row of a surface kind must be flagged.
    mutated = copy.deepcopy(freeze)
    mutated["rows"] = [r for r in mutated["rows"] if r.get("surface_kind") != "manifest"]
    recompute_derived(mutated)
    record("surface_kind_absent_rejected", "coverage.surface_kind_absent", "coverage.surface_kind_absent" in check_ids(mutated))

    # A publication decision that disagrees with the firing rules must be flagged.
    mutated = copy.deepcopy(freeze)
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
        ids = {f.check_id for f in validate_freeze(ensure_dict(candidate, "fixture"), repo_root)}
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
    freeze: dict[str, Any],
    findings: list[Finding],
    drill_results: list[dict[str, str]],
    fixture_results: list[dict[str, str]],
) -> None:
    decision, rule_ids, window_ids = computed_publication(freeze)
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "record_kind": "stable_version_windows_validation_capture",
        "status": "pass" if not any(f.severity == "error" for f in findings) else "fail",
        "generated_at": generated_at_now(),
        "freeze_id": freeze.get("freeze_id"),
        "as_of": freeze.get("as_of"),
        "summary": freeze.get("summary"),
        "publication": {
            "decision": decision,
            "blocking_rule_ids": rule_ids,
            "blocking_window_ids": window_ids,
        },
        "negative_drills": drill_results,
        "fixture_cases": fixture_results,
        "findings": [f.as_report() for f in findings],
    }
    path.write_text(json_text(payload), encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    freeze = ensure_dict(
        load_json(repo_root / args.freeze, "stable version-window freeze"),
        "stable version-window freeze",
    )

    findings = validate_freeze(freeze, repo_root)
    drill_results, drill_findings = run_negative_drills(freeze, repo_root)
    findings.extend(drill_findings)
    fixture_results, fixture_findings = run_fixture_cases(repo_root, args.fixtures_dir)
    findings.extend(fixture_findings)

    report_rel = args.report
    if report_rel is None and not args.no_capture:
        report_rel = DEFAULT_REPORT_REL
    if report_rel:
        write_report(repo_root / report_rel, freeze, findings, drill_results, fixture_results)

    errors = [f for f in findings if f.severity == "error"]
    if errors:
        for item in errors:
            print(f"ERROR [{item.check_id}] ({item.ref}): {item.message}", file=sys.stderr)
        return 1

    summary = freeze.get("summary", {})
    decision, rule_ids, _window_ids = computed_publication(freeze)
    print(
        "stable version-window freeze validated "
        f"({summary.get('total_surfaces')} surfaces across {summary.get('total_claims')} claims, "
        f"{summary.get('surfaces_frozen_stable')} frozen stable, "
        f"{summary.get('surfaces_narrowed_below_cutline')} narrowed; "
        f"kinds cli={summary.get('cli_surfaces')} schema={summary.get('schema_surfaces')} "
        f"api={summary.get('api_surfaces')} manifest={summary.get('manifest_surfaces')}; "
        f"release-blocking {summary.get('release_blocking_frozen_stable')}/{summary.get('release_blocking_total')} frozen; "
        f"packets {summary.get('packets_current')} current / "
        f"{summary.get('packets_due_for_refresh')} due / "
        f"{summary.get('packets_breached')} breached / "
        f"{summary.get('packets_missing')} missing; "
        f"{summary.get('total_deprecations')} deprecations, "
        f"{summary.get('surfaces_deprecation_overdue')} overdue; "
        f"{summary.get('freeze_rules_firing')} rules firing; "
        f"publication={decision}; {len(drill_results)} negative drills, {len(fixture_results)} fixture cases)"
    )

    if args.require_proceed and decision == "hold":
        print(
            f"PUBLICATION HELD: stable version-window publication blocked by {len(rule_ids)} rule(s): {', '.join(rule_ids)}",
            file=sys.stderr,
        )
        return 2

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
