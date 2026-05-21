#!/usr/bin/env python3
"""Validate the stable shiproom dashboard that wires each shiproom panel to its upstream
source, a packet-freshness SLO, measurable fitness functions, and the qualification-row
stop rules that hold promotion.

The stable claim manifest decides the single canonical lifecycle label each *subject*
publishes; the stable qualification matrix decides whether each *qualification row* holds
its claimed level; the stable proof index decides whether each launch-blocking
*requirement* is proven; the maintenance-control packet decides whether each maintenance
*lane* is governed. This dashboard is the consuming layer over all of them: for each panel
— claim truth, qualification, public proof, or maintenance — is the panel green, backed by
a fresh freshness packet, by fitness functions that all clear their thresholds, by the
qualification rows it watches still holding the cutline, and by an owner sign-off? It reads
the checked-in dashboard at ``artifacts/release/shiproom_dashboard.json`` and:

  - asserts the closed vocabularies (lifecycle labels, panel kinds, comparators, fitness
    statuses, panel states, stop reasons, stop actions) and the launch cutline are
    canonical;
  - asserts every panel that is narrowed (for an absent/incomplete panel, a failing or
    unmeasured fitness function, a regressed watched qualification row, a breached/missing
    freshness packet, an expired waiver, a missing owner sign-off, or a public claim whose
    canonical label is itself below the cutline) drops below the cutline rather than
    rendering a label wider than the public claim, and that a green panel renders the public
    claim's canonical label cleanly with a captured within-SLO freshness packet, fitness
    functions that all clear, and an owner sign-off;
  - recomputes each fitness function's pass/warn/fail/unmeasured status from its comparator,
    threshold, warn band, and measurement, and fails on any drift or inconsistent warn band;
  - asserts claim_truth/qualification/public_proof/maintenance coverage: every panel kind is
    represented, every declared release-blocking panel ref has exactly one covering
    release-blocking row, every release-blocking row is declared, and no panel ref repeats;
  - performs the ceiling cross-check the typed model cannot — it reads the stable claim
    manifest named by ``claim_manifest_ref`` and fails when a panel's ``claim_label`` is not
    the label the claim manifest publishes for the entry named by ``claim_ref``, or names an
    entry the claim manifest does not carry;
  - performs the qualification-row stop-rule cross-check — it reads the stable qualification
    matrix named by ``qualification_matrix_ref`` and fails when a watched row is not in the
    matrix, when a watched row has regressed below the cutline but the panel does not name
    ``qualification_row_regressed`` and narrow, or when a panel names that reason while every
    watched row still holds the cutline;
  - performs the packet-freshness SLO automation and the waiver-expiry date arithmetic
    against the dashboard ``as_of`` date the typed model cannot;
  - recomputes the publication verdict and the summary block, and fails on any drift;
  - runs negative drills proving the narrowing, ceiling, fitness, freshness, qualification,
    and publication rejections all fire; and
  - runs the checked-in fixture cases under ``fixtures/release/shiproom_dashboard`` and fails
    when a case the dashboard marks as rejected validates clean.

With ``--require-proceed`` the gate additionally fails (exit code 2) when the recomputed
publication verdict is ``hold``, so shiproom and release tooling can fail promotion directly
from this artifact.

The typed Rust consumer
(``aureline_release::shiproom_dashboard::current_shiproom_dashboard``) reads the same
dashboard and runs the same structural cross-check, so this gate and
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


DEFAULT_DASHBOARD_REL = "artifacts/release/shiproom_dashboard.json"
DEFAULT_REPORT_REL = "artifacts/release/captures/shiproom_dashboard_validation_capture.json"
DEFAULT_FIXTURES_REL = "fixtures/release/shiproom_dashboard"

EXPECTED_SCHEMA_VERSION = 1
DASHBOARD_RECORD_KIND = "shiproom_dashboard"

LIFECYCLE_LABELS = ("lts", "stable", "beta", "preview", "withdrawn")
ABOVE_CUTLINE = ("lts", "stable")
BELOW_CUTLINE = ("beta", "preview", "withdrawn")
PANEL_KINDS = ("claim_truth", "qualification", "public_proof", "maintenance")
COMPARATORS = ("at_least", "at_most", "equals")
FITNESS_STATUSES = ("pass", "warn", "fail", "unmeasured")
FRESHNESS_SLO_STATES = ("current", "due_for_refresh", "breached", "missing")
PANEL_STATES = (
    "green",
    "green_on_waiver",
    "narrowed_unbacked",
    "narrowed_regressed",
    "narrowed_claim_narrowed",
    "narrowed_stale",
    "narrowed_waiver_expired",
)
STOP_REASONS = (
    "claim_label_narrowed",
    "panel_capability_absent",
    "qualification_row_regressed",
    "fitness_function_failing",
    "panel_evidence_incomplete",
    "freshness_packet_breached",
    "freshness_packet_missing",
    "waiver_expired",
    "owner_signoff_missing",
)
STOP_ACTIONS = (
    "hold_promotion",
    "narrow_panel_label",
    "refresh_freshness_packet",
    "remediate_fitness_function",
    "recapture_panel_evidence",
    "request_owner_signoff",
)
GREEN_STATES = ("green", "green_on_waiver")
NARROWING_STATES = (
    "narrowed_unbacked",
    "narrowed_regressed",
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
    parser.add_argument("--dashboard", default=DEFAULT_DASHBOARD_REL)
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


def reasons_of(panel: dict[str, Any]) -> list[str]:
    reasons = panel.get("active_stop_reasons", [])
    return reasons if isinstance(reasons, list) else []


def fitness_functions_of(panel: dict[str, Any]) -> list[dict[str, Any]]:
    functions = panel.get("fitness_functions", [])
    return [f for f in functions if isinstance(f, dict)] if isinstance(functions, list) else []


def computed_fitness_status(function: dict[str, Any]) -> str | None:
    """Recompute pass/warn/fail/unmeasured from the comparator, threshold, warn band, and
    measurement. Returns None when the inputs are not numerically comparable (handled by the
    structural checks)."""
    measured = function.get("measured")
    if measured is None:
        return "unmeasured"
    if not is_int(measured):
        return None
    comparator = function.get("comparator")
    threshold = function.get("threshold")
    if comparator not in COMPARATORS or not is_int(threshold):
        return None
    if comparator == "at_least":
        passes = measured >= threshold
    elif comparator == "at_most":
        passes = measured <= threshold
    else:
        passes = measured == threshold
    if not passes:
        return "fail"
    warn = function.get("warn_threshold")
    if warn is None:
        return "pass"
    if not is_int(warn):
        return None
    if comparator == "at_least":
        within = measured >= warn
    elif comparator == "at_most":
        within = measured <= warn
    else:
        within = True
    return "pass" if within else "warn"


def warn_band_consistent(function: dict[str, Any]) -> bool:
    warn = function.get("warn_threshold")
    if warn is None:
        return True
    comparator = function.get("comparator")
    threshold = function.get("threshold")
    if comparator == "equals":
        return False
    if not (is_int(warn) and is_int(threshold)):
        return True
    if comparator == "at_least":
        return warn >= threshold
    if comparator == "at_most":
        return warn <= threshold
    return True


def fitness_satisfied(function: dict[str, Any]) -> bool:
    return function.get("status") in ("pass", "warn")


def stop_rule_fires(rule: dict[str, Any], panels: list[dict[str, Any]]) -> bool:
    trigger = rule.get("trigger_reason")
    labels = rule.get("applies_to_labels", [])
    if not isinstance(labels, list):
        return False
    for panel in panels:
        if panel.get("claim_label") in labels and trigger in reasons_of(panel):
            return True
    return False


def computed_publication(dashboard: dict[str, Any]) -> tuple[str, list[str], list[str]]:
    panels = dashboard.get("panels", [])
    rules = dashboard.get("stop_rules", [])
    blocking_rules = [
        rule
        for rule in rules
        if rule.get("blocks_promotion") is True and stop_rule_fires(rule, panels)
    ]
    rule_ids = sorted(str(rule.get("rule_id")) for rule in blocking_rules)
    blocking_triggers = {rule.get("trigger_reason") for rule in blocking_rules}
    panel_ids: set[str] = set()
    for panel in panels:
        if is_above_cutline(panel.get("claim_label")) and blocking_triggers.intersection(
            reasons_of(panel)
        ):
            panel_ids.add(str(panel.get("panel_id")))
    decision = "hold" if blocking_rules else "proceed"
    return decision, rule_ids, sorted(panel_ids)


def computed_summary(dashboard: dict[str, Any]) -> dict[str, Any]:
    panels = dashboard.get("panels", [])
    rules = dashboard.get("stop_rules", [])

    def packet_state(panel: dict[str, Any]) -> Any:
        packet = panel.get("freshness_packet")
        return packet.get("slo_state") if isinstance(packet, dict) else None

    def kind_count(kind: str) -> int:
        return sum(1 for p in panels if p.get("panel_kind") == kind)

    def fitness_count(status: str) -> int:
        return sum(
            1
            for p in panels
            for f in fitness_functions_of(p)
            if f.get("status") == status
        )

    green = [p for p in panels if is_above_cutline(p.get("displayed_label"))]
    claims = {str(p.get("claim_ref")) for p in panels}
    release_blocking = [p for p in panels if p.get("release_blocking") is True]
    rb_green = [p for p in release_blocking if is_above_cutline(p.get("displayed_label"))]
    return {
        "total_panels": len(panels),
        "total_claims": len(claims),
        "panels_green_stable": len(green),
        "panels_narrowed_below_cutline": len(panels) - len(green),
        "panels_on_active_waiver": sum(
            1 for p in panels if p.get("panel_state") == "green_on_waiver"
        ),
        "release_blocking_total": len(release_blocking),
        "release_blocking_green_stable": len(rb_green),
        "release_blocking_narrowed": len(release_blocking) - len(rb_green),
        "claim_truth_panels": kind_count("claim_truth"),
        "qualification_panels": kind_count("qualification"),
        "public_proof_panels": kind_count("public_proof"),
        "maintenance_panels": kind_count("maintenance"),
        "packets_current": sum(1 for p in panels if packet_state(p) == "current"),
        "packets_due_for_refresh": sum(1 for p in panels if packet_state(p) == "due_for_refresh"),
        "packets_breached": sum(1 for p in panels if packet_state(p) == "breached"),
        "packets_missing": sum(1 for p in panels if packet_state(p) == "missing"),
        "total_fitness_functions": sum(len(fitness_functions_of(p)) for p in panels),
        "fitness_pass": fitness_count("pass"),
        "fitness_warn": fitness_count("warn"),
        "fitness_fail": fitness_count("fail"),
        "fitness_unmeasured": fitness_count("unmeasured"),
        "total_active_stop_reasons": sum(len(reasons_of(p)) for p in panels),
        "stop_rules_firing": sum(1 for rule in rules if stop_rule_fires(rule, panels)),
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


def validate_envelope(dashboard: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings: list[Finding] = []
    dashboard_id = str(dashboard.get("dashboard_id", "<dashboard>"))

    if dashboard.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(Finding("error", "dashboard.schema_version", "schema_version must be 1", dashboard_id))
    if dashboard.get("record_kind") != DASHBOARD_RECORD_KIND:
        findings.append(Finding("error", "dashboard.record_kind", "record_kind is not supported", dashboard_id))
    for field in (
        "dashboard_id",
        "status",
        "overview_page",
        "as_of",
        "claim_manifest_ref",
        "qualification_matrix_ref",
        "freshness_slo_register_ref",
    ):
        if not is_str(dashboard.get(field)):
            findings.append(
                Finding("error", "dashboard.empty_field", f"{field} must be a non-empty string", dashboard_id)
            )
    if parse_date(dashboard.get("as_of")) is None:
        findings.append(Finding("error", "dashboard.as_of_invalid", "as_of must be an ISO date", dashboard_id))

    for field in ("claim_manifest_ref", "qualification_matrix_ref"):
        ref = dashboard.get(field)
        if is_str(ref) and not (repo_root / ref.split("#", 1)[0]).exists():
            findings.append(
                Finding("error", "dashboard.ref_missing", f"{field} does not exist: {ref}", dashboard_id)
            )

    for key, expected in (
        ("lifecycle_labels", list(LIFECYCLE_LABELS)),
        ("panel_kinds", list(PANEL_KINDS)),
        ("comparators", list(COMPARATORS)),
        ("fitness_statuses", list(FITNESS_STATUSES)),
        ("panel_states", list(PANEL_STATES)),
        ("stop_reasons", list(STOP_REASONS)),
        ("stop_actions", list(STOP_ACTIONS)),
    ):
        if list(dashboard.get(key, [])) != expected:
            findings.append(
                Finding("error", "dashboard.vocabulary", f"dashboard.{key} is not the closed vocabulary", key)
            )

    refs = dashboard.get("release_blocking_panel_refs")
    if not isinstance(refs, list) or not refs or any(not is_str(r) for r in refs):
        findings.append(
            Finding(
                "error",
                "dashboard.release_blocking_refs",
                "release_blocking_panel_refs must be a non-empty list of non-empty strings",
                dashboard_id,
            )
        )

    cutline = dashboard.get("launch_cutline")
    if not isinstance(cutline, dict):
        findings.append(Finding("error", "cutline.missing", "dashboard must carry a launch_cutline", dashboard_id))
    else:
        if cutline.get("cutline_level") != "stable":
            findings.append(Finding("error", "cutline.level", "cutline_level must be stable", dashboard_id))
        if list(cutline.get("above_cutline_levels", [])) != list(ABOVE_CUTLINE):
            findings.append(Finding("error", "cutline.above", "above_cutline_levels is not canonical", dashboard_id))
        if list(cutline.get("below_cutline_levels", [])) != list(BELOW_CUTLINE):
            findings.append(Finding("error", "cutline.below", "below_cutline_levels is not canonical", dashboard_id))
        if not is_str(cutline.get("description")):
            findings.append(Finding("error", "cutline.description", "cutline description must be non-empty", dashboard_id))
    return findings


def validate_rules(dashboard: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    rules = dashboard.get("stop_rules")
    if not isinstance(rules, list) or not rules:
        findings.append(Finding("error", "rules.empty", "dashboard must enumerate at least one stop rule", "<dashboard>"))
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
        if rule.get("trigger_reason") not in STOP_REASONS:
            findings.append(Finding("error", "rule.trigger_invalid", "trigger_reason is outside the vocabulary", rule_id))
        else:
            covered.add(rule["trigger_reason"])
        if rule.get("default_action") not in STOP_ACTIONS:
            findings.append(Finding("error", "rule.action_invalid", "default_action is outside the vocabulary", rule_id))
        labels = rule.get("applies_to_labels")
        if not isinstance(labels, list) or not labels:
            findings.append(Finding("error", "rule.labels_empty", "rule must watch at least one label", rule_id))
        elif any(label not in LIFECYCLE_LABELS for label in labels):
            findings.append(Finding("error", "rule.labels_invalid", "applies_to_labels has an unknown label", rule_id))
        if not isinstance(rule.get("blocks_promotion"), bool):
            findings.append(Finding("error", "rule.blocks_invalid", "blocks_promotion must be a boolean", rule_id))

    for reason in STOP_REASONS:
        if reason not in covered:
            findings.append(
                Finding("error", "rule.reason_uncovered", "stop reason has no rule watching for it", reason)
            )
    return findings


def validate_packet_block(panel_id: str, packet: Any) -> list[Finding]:
    findings: list[Finding] = []
    if not isinstance(packet, dict):
        findings.append(Finding("error", "panel.packet_missing", "panel must carry a freshness_packet block", panel_id))
        return findings
    for field in ("packet_id", "packet_ref", "proof_index_ref"):
        if not is_str(packet.get(field)):
            findings.append(Finding("error", "packet.empty_field", f"freshness_packet.{field} must be non-empty", panel_id))
    if packet.get("slo_state") not in FRESHNESS_SLO_STATES:
        findings.append(Finding("error", "packet.state_invalid", "slo_state is outside the vocabulary", panel_id))
    slo = packet.get("freshness_slo")
    if not isinstance(slo, dict):
        findings.append(Finding("error", "packet.slo_missing", "freshness_packet must carry a freshness_slo block", panel_id))
        return findings
    if not is_str(slo.get("slo_register_ref")):
        findings.append(Finding("error", "packet.slo_register_missing", "freshness_slo.slo_register_ref must be non-empty", panel_id))
    target = slo.get("target_max_age_days")
    warn = slo.get("warn_within_days")
    if not (is_int(target) and target >= 1):
        findings.append(Finding("error", "packet.target_invalid", "target_max_age_days must be an integer >= 1", panel_id))
    if not (is_int(warn) and warn >= 0):
        findings.append(Finding("error", "packet.warn_invalid", "warn_within_days must be an integer >= 0", panel_id))
    if is_int(target) and is_int(warn) and warn > target:
        findings.append(Finding("error", "packet.slo_inconsistent", "warn_within_days may not exceed target_max_age_days", panel_id))
    return findings


def validate_fitness(panel_id: str, panel: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    functions = panel.get("fitness_functions", [])
    if functions and not isinstance(functions, list):
        findings.append(Finding("error", "panel.fitness_invalid", "fitness_functions must be a list", panel_id))
        return findings

    reasons = reasons_of(panel)
    seen: set[str] = set()
    any_fail = False
    any_unmeasured = False
    for function in fitness_functions_of(panel):
        function_id = str(function.get("function_id", "<fitness>"))
        for field in ("function_id", "title", "metric", "unit", "measurement_ref"):
            if not is_str(function.get(field)):
                findings.append(Finding("error", "fitness.empty_field", f"fitness function {field} must be non-empty", function_id))
        if function_id in seen:
            findings.append(Finding("error", "fitness.duplicate_id", "fitness function ids must be unique on a panel", function_id))
        seen.add(function_id)
        if function.get("comparator") not in COMPARATORS:
            findings.append(Finding("error", "fitness.comparator_invalid", "comparator is outside the vocabulary", function_id))
        if function.get("status") not in FITNESS_STATUSES:
            findings.append(Finding("error", "fitness.status_invalid", "status is outside the vocabulary", function_id))
        if not is_int(function.get("threshold")):
            findings.append(Finding("error", "fitness.threshold_invalid", "threshold must be an integer", function_id))
        warn = function.get("warn_threshold")
        if warn is not None and not is_int(warn):
            findings.append(Finding("error", "fitness.warn_invalid", "warn_threshold must be an integer or null", function_id))
        measured = function.get("measured")
        if measured is not None and not is_int(measured):
            findings.append(Finding("error", "fitness.measured_invalid", "measured must be an integer or null", function_id))
        computed = computed_fitness_status(function)
        if computed is not None and function.get("status") != computed:
            findings.append(
                Finding("error", "fitness.status_inconsistent", f"declared status {function.get('status')!r} disagrees with computed {computed!r}", function_id)
            )
        if not warn_band_consistent(function):
            findings.append(Finding("error", "fitness.warn_band_inconsistent", "warn_threshold is inconsistent with the comparator", function_id))
        if function.get("status") == "fail":
            any_fail = True
        if function.get("status") == "unmeasured":
            any_unmeasured = True

    if any_fail and "fitness_function_failing" not in reasons:
        findings.append(Finding("error", "panel.failing_fitness_without_reason", "a panel with a failing fitness function must name fitness_function_failing", panel_id))
    if any_unmeasured and "panel_evidence_incomplete" not in reasons:
        findings.append(Finding("error", "panel.unmeasured_fitness_without_reason", "a panel with an unmeasured fitness function must name panel_evidence_incomplete", panel_id))
    return findings


def validate_panel(panel: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings: list[Finding] = []
    panel_id = str(panel.get("panel_id", "<panel>"))

    for field in (
        "panel_id",
        "title",
        "panel_ref",
        "panel_summary",
        "claim_ref",
        "source_ref",
        "rationale",
    ):
        if not is_str(panel.get(field)):
            findings.append(Finding("error", "panel.empty_field", f"panel {field} must be non-empty", panel_id))

    if panel.get("panel_kind") not in PANEL_KINDS:
        findings.append(Finding("error", "panel.kind_invalid", "panel_kind is outside the vocabulary", panel_id))
    if not isinstance(panel.get("release_blocking"), bool):
        findings.append(Finding("error", "panel.release_blocking_invalid", "release_blocking must be a boolean", panel_id))

    claim_label = panel.get("claim_label")
    displayed = panel.get("displayed_label")
    state = panel.get("panel_state")
    reasons = reasons_of(panel)

    if claim_label not in LIFECYCLE_LABELS:
        findings.append(Finding("error", "panel.claim_label_invalid", "claim_label is invalid", panel_id))
    if displayed not in LIFECYCLE_LABELS:
        findings.append(Finding("error", "panel.displayed_invalid", "displayed_label is invalid", panel_id))
    if state not in PANEL_STATES:
        findings.append(Finding("error", "panel.state_invalid", "panel_state is invalid", panel_id))
    if any(reason not in STOP_REASONS for reason in reasons):
        findings.append(Finding("error", "panel.reason_invalid", "active_stop_reasons has an unknown reason", panel_id))

    # The ceiling: no panel may render a label wider than the public claim's label.
    if claim_label in LEVEL_RANK and displayed in LEVEL_RANK and LEVEL_RANK[displayed] > LEVEL_RANK[claim_label]:
        findings.append(Finding("error", "panel.displayed_wider_than_claim", "displayed_label is wider than the claim ceiling", panel_id))

    packet = panel.get("freshness_packet")
    findings.extend(validate_packet_block(panel_id, packet))
    findings.extend(validate_fitness(panel_id, panel))
    slo_state = packet.get("slo_state") if isinstance(packet, dict) else None

    green = state in GREEN_STATES
    claim_holds = is_above_cutline(claim_label)

    # A claim canonically below the cutline forces the panel to inherit the ceiling and
    # narrow.
    if claim_label in LEVEL_RANK and not claim_holds:
        if green:
            findings.append(Finding("error", "panel.green_on_narrowed_claim", "a panel renders green while the public claim label is below the cutline", panel_id))
        if "claim_label_narrowed" not in reasons:
            findings.append(Finding("error", "panel.claim_narrowed_without_reason", "a panel whose claim is narrowed must name claim_label_narrowed", panel_id))

    if state in NARROWING_STATES:
        if is_above_cutline(displayed):
            findings.append(Finding("error", "panel.displayed_not_narrowed", "a panel that is not green must narrow below the cutline", panel_id))
        if not reasons:
            findings.append(Finding("error", "panel.narrowing_without_reason", "a narrowing panel must name an active stop reason", panel_id))
        if slo_state == "breached" and "freshness_packet_breached" not in reasons:
            findings.append(Finding("error", "panel.breached_without_reason", "a breached packet must name freshness_packet_breached", panel_id))
        if slo_state == "missing" and "freshness_packet_missing" not in reasons:
            findings.append(Finding("error", "panel.missing_without_reason", "a missing packet must name freshness_packet_missing", panel_id))
    if green:
        if claim_label != displayed:
            findings.append(Finding("error", "panel.green_label_not_equal_claim", "a green panel must render the public claim's canonical label", panel_id))
        if reasons:
            findings.append(Finding("error", "panel.green_with_active_stop", "a green panel must carry no active stop reason", panel_id))
        if isinstance(packet, dict) and not has_capture(packet):
            findings.append(Finding("error", "panel.green_without_fresh_packet", "a green panel must ride a captured, evidence-backed freshness packet", panel_id))
        if slo_state in ("breached", "missing"):
            findings.append(Finding("error", "panel.green_on_stale_packet", "a green panel must ride a packet within its freshness SLO", panel_id))
        if not all(fitness_satisfied(f) for f in fitness_functions_of(panel)):
            findings.append(Finding("error", "panel.green_with_failing_fitness", "a green panel must carry only fitness functions that clear their threshold", panel_id))
        signoff = panel.get("owner_signoff", {})
        if not (isinstance(signoff, dict) and signoff.get("signed_off") is True and is_str(signoff.get("signed_at"))):
            findings.append(Finding("error", "panel.green_without_signoff", "a green panel must carry an owner sign-off with a date", panel_id))
    signoff = panel.get("owner_signoff", {})
    if not (isinstance(signoff, dict) and is_str(signoff.get("owner_ref"))):
        findings.append(Finding("error", "panel.signoff_owner_missing", "owner_signoff.owner_ref must be non-empty", panel_id))

    findings.extend(validate_state_reason_coherence(panel_id, state, reasons, panel))
    findings.extend(validate_panel_refs(panel_id, panel, repo_root))
    return findings


def validate_state_reason_coherence(panel_id: str, state: Any, reasons: list[str], panel: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    if state == "narrowed_unbacked":
        allowed = {"panel_capability_absent", "panel_evidence_incomplete", "owner_signoff_missing"}
        if not allowed.intersection(reasons):
            findings.append(Finding("error", "panel.state_reason_incoherent", "narrowed_unbacked requires a capability/evidence/signoff reason", panel_id))
    if state == "narrowed_regressed" and not (
        "qualification_row_regressed" in reasons or "fitness_function_failing" in reasons
    ):
        findings.append(Finding("error", "panel.state_reason_incoherent", "narrowed_regressed requires a qualification or fitness reason", panel_id))
    if state == "narrowed_claim_narrowed" and "claim_label_narrowed" not in reasons:
        findings.append(Finding("error", "panel.state_reason_incoherent", "narrowed_claim_narrowed requires the claim_label_narrowed reason", panel_id))
    if state == "narrowed_stale" and not (
        "freshness_packet_breached" in reasons or "freshness_packet_missing" in reasons
    ):
        findings.append(Finding("error", "panel.state_reason_incoherent", "narrowed_stale requires a packet-freshness reason", panel_id))
    if state == "narrowed_waiver_expired":
        if "waiver_expired" not in reasons:
            findings.append(Finding("error", "panel.state_reason_incoherent", "narrowed_waiver_expired requires the waiver_expired reason", panel_id))
        if not isinstance(panel.get("waiver"), dict):
            findings.append(Finding("error", "panel.waiver_state_without_waiver", "narrowed_waiver_expired state must name a waiver", panel_id))
    if state == "green_on_waiver":
        waiver = panel.get("waiver")
        if not isinstance(waiver, dict) or not is_str(waiver.get("waiver_ref")) or not is_str(waiver.get("expires_at")):
            findings.append(Finding("error", "panel.waiver_state_without_waiver", "green_on_waiver state must name a waiver", panel_id))
    return findings


def validate_panel_refs(panel_id: str, panel: dict[str, Any], repo_root: Path) -> list[Finding]:
    """Path-existence checks for the source ref and the freshness-packet refs (not the
    cross-artifact ids, which resolve against the claim manifest and qualification matrix)."""
    findings: list[Finding] = []
    refs: list[str] = []
    source = panel.get("source_ref")
    if is_str(source):
        refs.append(source.split("#", 1)[0])
    packet = panel.get("freshness_packet")
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
    for ref in refs:
        if not (repo_root / ref).exists():
            findings.append(Finding("error", "panel.ref_missing", f"referenced artifact does not exist: {ref}", panel_id))
    return findings


def validate_coverage(dashboard: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    panels = dashboard.get("panels", [])

    seen: set[str] = set()
    for panel in panels:
        ref = panel.get("panel_ref")
        if not is_str(ref):
            continue
        if ref in seen:
            findings.append(Finding("error", "coverage.panel_duplicate", f"panel ref appears more than once: {ref}", ref))
        seen.add(ref)

    declared = dashboard.get("release_blocking_panel_refs", [])
    declared_set = {r for r in declared if is_str(r)} if isinstance(declared, list) else set()
    covered = {
        str(panel.get("panel_ref"))
        for panel in panels
        if panel.get("release_blocking") is True and is_str(panel.get("panel_ref"))
    }
    for ref in sorted(declared_set):
        if ref not in covered:
            findings.append(Finding("error", "coverage.release_blocking_uncovered", f"declared release-blocking panel has no covering row: {ref}", ref))
    for panel in panels:
        if panel.get("release_blocking") is True and is_str(panel.get("panel_ref")) and panel["panel_ref"] not in declared_set:
            findings.append(Finding("error", "coverage.release_blocking_not_declared", "a release-blocking panel is not in release_blocking_panel_refs", str(panel.get("panel_id"))))

    present_kinds = {panel.get("panel_kind") for panel in panels}
    for kind in PANEL_KINDS:
        if kind not in present_kinds:
            findings.append(Finding("error", "coverage.panel_kind_absent", f"panel kind has no row: {kind}", kind))
    return findings


def validate_freshness(dashboard: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    as_of = parse_date(dashboard.get("as_of"))
    if as_of is None:
        return findings
    for panel in dashboard.get("panels", []):
        panel_id = str(panel.get("panel_id", "<panel>"))
        packet = panel.get("freshness_packet")
        if not isinstance(packet, dict):
            continue
        declared = packet.get("slo_state")
        computed = computed_slo_state(packet, as_of)
        if computed is None or declared not in FRESHNESS_RANK or computed not in FRESHNESS_RANK:
            continue
        if FRESHNESS_RANK[declared] > FRESHNESS_RANK[computed]:
            findings.append(
                Finding("error", "panel.packet_freshness_overstated", f"declared slo_state {declared!r} is fresher than the {computed!r} the as_of date allows", panel_id)
            )
        if panel.get("panel_state") in GREEN_STATES and computed in ("breached", "missing"):
            findings.append(
                Finding("error", "panel.green_on_breached_packet", "a green panel rides a freshness packet past its SLO against as_of", panel_id)
            )
    return findings


def validate_waivers(dashboard: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    as_of = parse_date(dashboard.get("as_of"))
    if as_of is None:
        return findings
    for panel in dashboard.get("panels", []):
        panel_id = str(panel.get("panel_id", "<panel>"))
        state = panel.get("panel_state")
        waiver = panel.get("waiver")
        if not isinstance(waiver, dict):
            continue
        expires = parse_date(waiver.get("expires_at"))
        if expires is None:
            findings.append(Finding("error", "panel.waiver_expiry_invalid", "waiver expires_at must be an ISO date", panel_id))
            continue
        expired = as_of >= expires
        if expired and state == "green_on_waiver":
            findings.append(Finding("error", "panel.green_on_expired_waiver", "a panel renders green on a waiver that has expired against as_of", panel_id))
        if not expired and state == "narrowed_waiver_expired":
            findings.append(Finding("error", "panel.waiver_expired_but_active", "a panel is marked waiver-expired but the waiver is still active against as_of", panel_id))
    return findings


def validate_ceiling(dashboard: dict[str, Any], repo_root: Path) -> list[Finding]:
    """Cross-artifact ingestion: read the stable claim manifest and fail when a panel's
    claim_label is not the label the claim manifest publishes for the entry named by
    claim_ref, or names an entry the claim manifest does not carry."""
    findings: list[Finding] = []
    ref = dashboard.get("claim_manifest_ref")
    if not is_str(ref):
        return findings
    path = repo_root / ref.split("#", 1)[0]
    if not path.exists():
        findings.append(Finding("error", "ceiling.artifact_missing", f"claim_manifest_ref does not exist: {ref}", "<dashboard>"))
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

    for panel in dashboard.get("panels", []):
        panel_id = str(panel.get("panel_id", "<panel>"))
        claim_ref = panel.get("claim_ref")
        if not is_str(claim_ref):
            continue
        if claim_ref not in published_by_entry:
            findings.append(Finding("error", "ceiling.claim_entry_unknown", f"claim_ref is not an entry in the stable claim manifest: {claim_ref}", panel_id))
            continue
        canonical = published_by_entry[claim_ref]
        if panel.get("claim_label") != canonical:
            findings.append(Finding("error", "ceiling.claim_label_mismatch", f"claim_label {panel.get('claim_label')!r} does not match the claim manifest published_label {canonical!r}", panel_id))
    return findings


def validate_qualification(dashboard: dict[str, Any], repo_root: Path) -> list[Finding]:
    """Qualification-row stop-rule cross-check: read the stable qualification matrix and fail
    when a watched row is not in the matrix, when a watched row has regressed below the
    cutline but the panel does not name qualification_row_regressed, or when a panel names
    that reason while every watched row still holds the cutline."""
    findings: list[Finding] = []
    ref = dashboard.get("qualification_matrix_ref")
    if not is_str(ref):
        return findings
    path = repo_root / ref.split("#", 1)[0]
    if not path.exists():
        findings.append(Finding("error", "qualification.artifact_missing", f"qualification_matrix_ref does not exist: {ref}", "<dashboard>"))
        return findings
    try:
        matrix = json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        findings.append(Finding("error", "qualification.artifact_invalid", f"qualification_matrix_ref is not valid JSON: {exc}", ref))
        return findings

    effective_by_row = {}
    if isinstance(matrix, dict):
        effective_by_row = {
            str(r.get("row_id")): r.get("effective_level")
            for r in matrix.get("rows", [])
            if isinstance(r, dict)
        }

    for panel in dashboard.get("panels", []):
        panel_id = str(panel.get("panel_id", "<panel>"))
        reasons = reasons_of(panel)
        watched = panel.get("qualification_row_refs", [])
        watched = watched if isinstance(watched, list) else []
        any_regressed = False
        for row_ref in watched:
            if not is_str(row_ref):
                continue
            if row_ref not in effective_by_row:
                findings.append(Finding("error", "qualification.row_unknown", f"watched qualification row is not in the matrix: {row_ref}", panel_id))
                continue
            if not is_above_cutline(effective_by_row[row_ref]):
                any_regressed = True
        if any_regressed and "qualification_row_regressed" not in reasons:
            findings.append(Finding("error", "qualification.regressed_without_reason", "a panel watching a regressed qualification row must name qualification_row_regressed", panel_id))
        if "qualification_row_regressed" in reasons and not any_regressed:
            findings.append(Finding("error", "qualification.reason_without_regression", "a panel names qualification_row_regressed but every watched row holds the cutline", panel_id))
    return findings


def validate_publication(dashboard: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    publication = dashboard.get("publication")
    if not isinstance(publication, dict):
        findings.append(Finding("error", "publication.missing", "dashboard must carry a publication block", "<dashboard>"))
        return findings
    for field in ("publication_gate", "rationale"):
        if not is_str(publication.get(field)):
            findings.append(Finding("error", "publication.empty_field", f"publication.{field} must be non-empty", "<dashboard>"))
    decision, rule_ids, panel_ids = computed_publication(dashboard)
    if publication.get("decision") != decision:
        findings.append(
            Finding("error", "publication.decision_inconsistent", f"declared decision {publication.get('decision')!r} disagrees with computed {decision!r}", "<dashboard>")
        )
    if list(publication.get("blocking_rule_ids", [])) != rule_ids:
        findings.append(Finding("error", "publication.blocking_rules_mismatch", "blocking_rule_ids disagrees with the firing stop rules", "<dashboard>"))
    if list(publication.get("blocking_panel_ids", [])) != panel_ids:
        findings.append(Finding("error", "publication.blocking_panels_mismatch", "blocking_panel_ids disagrees with the firing stop rules", "<dashboard>"))
    return findings


def validate_summary(dashboard: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    summary = dashboard.get("summary")
    if not isinstance(summary, dict):
        findings.append(Finding("error", "summary.missing", "dashboard must carry a summary block", "<dashboard>"))
        return findings
    expected = computed_summary(dashboard)
    for key, value in expected.items():
        if summary.get(key) != value:
            findings.append(Finding("error", "summary.count_mismatch", f"summary.{key} must equal the recomputed value", key))
    return findings


def validate_dashboard(dashboard: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings = validate_envelope(dashboard, repo_root)
    findings.extend(validate_rules(dashboard))

    panels = dashboard.get("panels")
    if not isinstance(panels, list) or not panels:
        findings.append(Finding("error", "dashboard.panels_empty", "dashboard must enumerate at least one panel", "<dashboard>"))
        return findings

    seen: set[str] = set()
    for raw in panels:
        panel = ensure_dict(raw, "dashboard.panels[]")
        findings.extend(validate_panel(panel, repo_root))
        panel_id = str(panel.get("panel_id", "<panel>"))
        if panel_id in seen:
            findings.append(Finding("error", "panel.duplicate_id", "panel ids must be unique", panel_id))
        seen.add(panel_id)

    findings.extend(validate_coverage(dashboard))
    findings.extend(validate_freshness(dashboard))
    findings.extend(validate_waivers(dashboard))
    findings.extend(validate_ceiling(dashboard, repo_root))
    findings.extend(validate_qualification(dashboard, repo_root))
    findings.extend(validate_publication(dashboard))
    findings.extend(validate_summary(dashboard))
    return findings


def recompute_derived(dashboard: dict[str, Any]) -> None:
    dashboard["summary"] = computed_summary(dashboard)
    decision, rule_ids, panel_ids = computed_publication(dashboard)
    if isinstance(dashboard.get("publication"), dict):
        dashboard["publication"].update(
            {"decision": decision, "blocking_rule_ids": rule_ids, "blocking_panel_ids": panel_ids}
        )


def run_negative_drills(dashboard: dict[str, Any], repo_root: Path) -> tuple[list[dict[str, str]], list[Finding]]:
    results: list[dict[str, str]] = []
    findings: list[Finding] = []

    def record(drill_id: str, expected_check_id: str, fired: bool) -> None:
        results.append({"drill_id": drill_id, "expected_check_id": expected_check_id, "status": "passed" if fired else "failed"})
        if not fired:
            findings.append(Finding("error", "negative_drill.not_rejected", f"negative drill {drill_id} did not fire", drill_id))

    def check_ids(candidate: dict[str, Any]) -> set[str]:
        return {f.check_id for f in validate_dashboard(candidate, repo_root)}

    # A narrowing panel that still renders a stable label must be flagged.
    mutated = copy.deepcopy(dashboard)
    target = next((p for p in mutated["panels"] if p.get("panel_state") in NARROWING_STATES), None)
    if target is not None:
        target["displayed_label"] = "stable"
        recompute_derived(mutated)
        record("displayed_not_narrowed_rejected", "panel.displayed_not_narrowed", "panel.displayed_not_narrowed" in check_ids(mutated))

    # A green panel whose packet is breached against as_of must be flagged.
    mutated = copy.deepcopy(dashboard)
    target = next((p for p in mutated["panels"] if p.get("panel_state") in GREEN_STATES), None)
    if target is not None:
        target["freshness_packet"]["captured_at"] = "2000-01-01"
        record("green_on_breached_packet_rejected", "panel.green_on_breached_packet", "panel.green_on_breached_packet" in check_ids(mutated))

    # A fitness function whose declared status disagrees with its measurement must be flagged.
    mutated = copy.deepcopy(dashboard)
    target = next(
        (
            p
            for p in mutated["panels"]
            for f in fitness_functions_of(p)
            if f.get("status") == "pass" and f.get("comparator") == "at_least"
        ),
        None,
    )
    if target is not None:
        function = next(f for f in target["fitness_functions"] if f.get("status") == "pass" and f.get("comparator") == "at_least")
        function["measured"] = function["threshold"] - 1
        record("fitness_status_inconsistent_rejected", "fitness.status_inconsistent", "fitness.status_inconsistent" in check_ids(mutated))

    # A panel whose claim_label disagrees with the claim manifest must be flagged.
    mutated = copy.deepcopy(dashboard)
    target = next((p for p in mutated["panels"] if p.get("claim_label") == "beta"), None)
    if target is not None:
        target["claim_label"] = "stable"
        record("claim_label_mismatch_rejected", "ceiling.claim_label_mismatch", "ceiling.claim_label_mismatch" in check_ids(mutated))

    # A panel rendered wider than its public claim's ceiling must be flagged.
    mutated = copy.deepcopy(dashboard)
    target = next(
        (p for p in mutated["panels"] if p.get("claim_label") == "beta" and not is_above_cutline(p.get("displayed_label"))),
        None,
    )
    if target is not None:
        target["displayed_label"] = "lts"
        recompute_derived(mutated)
        record("displayed_wider_than_claim_rejected", "panel.displayed_wider_than_claim", "panel.displayed_wider_than_claim" in check_ids(mutated))

    # A green panel watching a regressed qualification row without naming it must be flagged.
    mutated = copy.deepcopy(dashboard)
    target = next((p for p in mutated["panels"] if p.get("panel_state") == "green" and isinstance(p.get("qualification_row_refs"), list)), None)
    if target is not None:
        target["qualification_row_refs"] = list(target["qualification_row_refs"]) + ["qual_row:state_schema_readers_writers"]
        record("qualification_regressed_without_reason_rejected", "qualification.regressed_without_reason", "qualification.regressed_without_reason" in check_ids(mutated))

    # Dropping a declared release-blocking panel's row must be flagged.
    mutated = copy.deepcopy(dashboard)
    declared = mutated.get("release_blocking_panel_refs", [])
    if declared:
        dropped = declared[0]
        mutated["panels"] = [p for p in mutated["panels"] if p.get("panel_ref") != dropped]
        recompute_derived(mutated)
        record("release_blocking_uncovered_rejected", "coverage.release_blocking_uncovered", "coverage.release_blocking_uncovered" in check_ids(mutated))

    # Removing every panel of a panel kind must be flagged.
    mutated = copy.deepcopy(dashboard)
    mutated["panels"] = [p for p in mutated["panels"] if p.get("panel_kind") != "maintenance"]
    recompute_derived(mutated)
    record("panel_kind_absent_rejected", "coverage.panel_kind_absent", "coverage.panel_kind_absent" in check_ids(mutated))

    # A publication decision that disagrees with the firing rules must be flagged.
    mutated = copy.deepcopy(dashboard)
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
        ids = {f.check_id for f in validate_dashboard(ensure_dict(candidate, "fixture"), repo_root)}
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
    dashboard: dict[str, Any],
    findings: list[Finding],
    drill_results: list[dict[str, str]],
    fixture_results: list[dict[str, str]],
) -> None:
    decision, rule_ids, panel_ids = computed_publication(dashboard)
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "record_kind": "shiproom_dashboard_validation_capture",
        "status": "pass" if not any(f.severity == "error" for f in findings) else "fail",
        "generated_at": generated_at_now(),
        "dashboard_id": dashboard.get("dashboard_id"),
        "as_of": dashboard.get("as_of"),
        "summary": dashboard.get("summary"),
        "publication": {
            "decision": decision,
            "blocking_rule_ids": rule_ids,
            "blocking_panel_ids": panel_ids,
        },
        "negative_drills": drill_results,
        "fixture_cases": fixture_results,
        "findings": [f.as_report() for f in findings],
    }
    path.write_text(json_text(payload), encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    dashboard = ensure_dict(
        load_json(repo_root / args.dashboard, "shiproom dashboard"),
        "shiproom dashboard",
    )

    findings = validate_dashboard(dashboard, repo_root)
    drill_results, drill_findings = run_negative_drills(dashboard, repo_root)
    findings.extend(drill_findings)
    fixture_results, fixture_findings = run_fixture_cases(repo_root, args.fixtures_dir)
    findings.extend(fixture_findings)

    report_rel = args.report
    if report_rel is None and not args.no_capture:
        report_rel = DEFAULT_REPORT_REL
    if report_rel:
        write_report(repo_root / report_rel, dashboard, findings, drill_results, fixture_results)

    errors = [f for f in findings if f.severity == "error"]
    if errors:
        for item in errors:
            print(f"ERROR [{item.check_id}] ({item.ref}): {item.message}", file=sys.stderr)
        return 1

    summary = dashboard.get("summary", {})
    decision, rule_ids, _panel_ids = computed_publication(dashboard)
    print(
        "shiproom dashboard validated "
        f"({summary.get('total_panels')} panels across {summary.get('total_claims')} claims, "
        f"{summary.get('panels_green_stable')} green stable, "
        f"{summary.get('panels_narrowed_below_cutline')} narrowed; "
        f"kinds claim_truth={summary.get('claim_truth_panels')} qualification={summary.get('qualification_panels')} "
        f"public_proof={summary.get('public_proof_panels')} maintenance={summary.get('maintenance_panels')}; "
        f"release-blocking {summary.get('release_blocking_green_stable')}/{summary.get('release_blocking_total')} green; "
        f"packets {summary.get('packets_current')} current / "
        f"{summary.get('packets_due_for_refresh')} due / "
        f"{summary.get('packets_breached')} breached / "
        f"{summary.get('packets_missing')} missing; "
        f"fitness {summary.get('fitness_pass')} pass / {summary.get('fitness_warn')} warn / "
        f"{summary.get('fitness_fail')} fail / {summary.get('fitness_unmeasured')} unmeasured; "
        f"{summary.get('stop_rules_firing')} rules firing; "
        f"publication={decision}; {len(drill_results)} negative drills, {len(fixture_results)} fixture cases)"
    )

    if args.require_proceed and decision == "hold":
        print(
            f"PUBLICATION HELD: shiproom promotion blocked by {len(rule_ids)} rule(s): {', '.join(rule_ids)}",
            file=sys.stderr,
        )
        return 2

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
