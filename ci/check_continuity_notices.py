#!/usr/bin/env python3
"""Validate the maintenance & failover continuity-notice audit corpus.

This is the deterministic audit lane for the marketed beta continuity-
communication surfaces: scheduled maintenance, read-only / drain windows,
regional and control-plane failover, and tenant / region migration. It consumes
the `continuity_notice_view_record` views the desktop shell, the activity center
/ durable history, CLI / headless inspect, diagnostics, and support exports all
read (see crates/aureline-shell/src/continuity_notices/model.rs), and proves the
quiet ways a continuity notice can lie are caught before they ship:

- **Stale-as-current.** A superseded, completed, imported, or refresh-aged-out
  notice keeps reading as current operational truth.
- **Lost queued work.** A queued publish-later or local-draft intent is not
  preserved with a durable ref, or is collapsed into successful hosted
  mutations.
- **Hidden boundary change.** A failover / migration changed a tenant, region,
  residency, key-ownership, or endpoint boundary, but the recovered state hides
  it.
- **Generic-degraded collapse.** A precise window is flattened into a generic
  "something is offline" banner with incident language on a planned event.

For **every** view fixture the validator schema-validates the record against
`schemas/ops/continuity_notice_view.schema.json`, then independently re-derives
the refresh-age bucket, the no-silent-current downgrade, the boundary-unresolved
flag, the honesty marker, and the summary counts from the raw fields and asserts
the stored record matches. The derivation is a second implementation of the
model, so a regression in either the model or a fixture fails the lane instead
of shipping silently. It also proves the corpus exercises every notice kind,
category, effective freshness, write-continuity posture, downgrade reason, and
boundary-axis state.

Run via scripts/ci/run_continuity_notices.sh.
"""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any

from jsonschema import Draft202012Validator, FormatChecker

# --------------------------------------------------------------------------- #
# Paths
# --------------------------------------------------------------------------- #

SCHEMA_REL = "schemas/ops/continuity_notice_view.schema.json"
FIXTURE_DIR_REL = "fixtures/ops/m3/maintenance_and_failover_notices"
DOC_REL = "docs/ops/m3/maintenance_failover_truth.md"
VALIDATOR_REL = "ci/check_continuity_notices.py"
SCRIPT_REL = "scripts/ci/run_continuity_notices.sh"
REPORT_REL = "artifacts/ops/m3/maintenance_failover_truth_report.md"

VIEW_RECORD_KIND = "continuity_notice_view_record"
VIEW_SCHEMA_VERSION = 1

NON_VIEW_FILES = {"README.md"}

# --------------------------------------------------------------------------- #
# Closed vocabularies — a deliberate second copy of the Rust enums. Keep these
# in lockstep with crates/aureline-shell/src/continuity_notices/model.rs.
# --------------------------------------------------------------------------- #

NOTICE_KINDS = {
    "scheduled_maintenance_window",
    "read_only_window",
    "drain_window",
    "scheduled_export_freeze",
    "tenant_migration",
    "region_migration",
    "regional_failover",
    "control_plane_failover",
    "post_event_reconciliation",
}
CATEGORIES = {"maintenance", "drain", "failover", "tenant_migration"}
CATEGORY_FOR_KIND = {
    "scheduled_maintenance_window": "maintenance",
    "scheduled_export_freeze": "maintenance",
    "post_event_reconciliation": "maintenance",
    "read_only_window": "drain",
    "drain_window": "drain",
    "regional_failover": "failover",
    "control_plane_failover": "failover",
    "tenant_migration": "tenant_migration",
    "region_migration": "tenant_migration",
}
PLAN_FOR_KIND = {k: ("emergency" if k in {"regional_failover", "control_plane_failover"} else "planned") for k in NOTICE_KINDS}

FRESHNESS = {"active_current", "superseded_stale", "completed_historical", "imported_historical"}
EFFECTIVE = {"current", "refresh_stale", "superseded_stale", "completed_historical", "imported_historical"}
REFRESH_AGES = {"fresh", "recent", "stale", "very_stale", "never"}
DOWNGRADES = {"refresh_expired", "notice_superseded", "window_completed", "imported_offline"}
PRESERVED_POSTURES = {"queued_publish_later", "local_draft_preserved"}
POSTURES = {
    "queued_publish_later",
    "local_draft_preserved",
    "retryable_when_connected",
    "draining_existing_only",
    "blocked_pending_reconnect",
    "blocked_pending_boundary_recheck",
    "blocked_no_safe_retry",
    "requires_manual_rerun",
}
AXIS_STATES = {"unchanged", "changed", "unknown_recheck_required", "not_applicable"}
MEANINGFUL_AXIS_STATES = {"changed", "unknown_recheck_required"}

CANONICAL_SCHEME = "aureline://"
GENERIC_LANDING_CLASSES = {"home", "dashboard", "landing", "index", "overview", "start", "root"}


# --------------------------------------------------------------------------- #
# Findings
# --------------------------------------------------------------------------- #


@dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    remediation: str
    ref: str | None = None
    details: dict[str, Any] = field(default_factory=dict)

    def as_report(self) -> dict[str, Any]:
        payload = asdict(self)
        if payload["ref"] is None:
            payload.pop("ref")
        if not payload["details"]:
            payload.pop("details")
        return payload


def err(check_id: str, message: str, remediation: str, ref: str | None = None, **details: Any) -> Finding:
    return Finding("error", check_id, message, remediation, ref, dict(details))


# --------------------------------------------------------------------------- #
# Timestamp / refresh-age derivation (port of model.rs derive_refresh_age)
# --------------------------------------------------------------------------- #


def days_from_civil(y: int, m: int, d: int) -> int:
    y = y - 1 if m <= 2 else y
    era = (y if y >= 0 else y - 399) // 400
    yoe = y - era * 400
    doy = (153 * (m - 3 if m > 2 else m + 9) + 2) // 5 + d - 1
    doe = yoe * 365 + yoe // 4 - yoe // 100 + doy
    return era * 146_097 + doe - 719_468


def parse_timestamp_minutes(value: str | None) -> int | None:
    if value is None or len(value) < 16:
        return None
    if value[4] != "-" or value[7] != "-":
        return None
    if value[10] not in ("T", " "):
        return None
    if value[13] != ":":
        return None
    try:
        year = int(value[0:4])
        month = int(value[5:7])
        day = int(value[8:10])
        hour = int(value[11:13])
        minute = int(value[14:16])
    except ValueError:
        return None
    if not (1 <= month <= 12) or not (1 <= day <= 31):
        return None
    if hour > 23 or minute > 59:
        return None
    return days_from_civil(year, month, day) * 24 * 60 + hour * 60 + minute


def derive_refresh_age(refresh_at: str | None, as_of: str) -> str:
    last = parse_timestamp_minutes(refresh_at)
    now = parse_timestamp_minutes(as_of)
    if last is None or now is None or now < last:
        return "never"
    delta = now - last
    if delta <= 5:
        return "fresh"
    if delta <= 60:
        return "recent"
    if delta <= 60 * 24:
        return "stale"
    return "very_stale"


def derive_effective_freshness(declared: str, refresh_age: str) -> tuple[str, list[str]]:
    if declared == "active_current":
        if refresh_age in ("fresh", "recent"):
            return "current", []
        return "refresh_stale", ["refresh_expired"]
    if declared == "superseded_stale":
        return "superseded_stale", ["notice_superseded"]
    if declared == "completed_historical":
        return "completed_historical", ["window_completed"]
    if declared == "imported_historical":
        return "imported_historical", ["imported_offline"]
    return declared, []


def is_canonical_ref(ref: str | None) -> bool:
    ref = (ref or "").strip()
    if not ref or len(ref) > 200 or not ref.startswith(CANONICAL_SCHEME):
        return False
    rest = ref[len(CANONICAL_SCHEME):]
    if "/" not in rest:
        return False
    cls, ident = rest.split("/", 1)
    if not cls or not ident:
        return False
    return cls not in GENERIC_LANDING_CLASSES


# --------------------------------------------------------------------------- #
# Per-view independent re-derivation (the lane-failing rules)
# --------------------------------------------------------------------------- #


def check_view(label: str, view: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []

    if view.get("record_kind") != VIEW_RECORD_KIND:
        findings.append(err("continuity.record_kind", f"{label}: wrong record_kind",
                            "Emit continuity_notice_view_record.", label))
    if view.get("schema_version") != VIEW_SCHEMA_VERSION:
        findings.append(err("continuity.schema_version", f"{label}: wrong schema_version",
                            "Set schema_version to 1.", label))

    kind = view.get("notice_kind")
    as_of = view.get("as_of", "")
    if kind not in NOTICE_KINDS:
        return findings + [err("continuity.kind.unknown", f"{label}: unknown notice_kind {kind!r}",
                               "Use a closed notice kind.", label)]

    # Category + plan derivation.
    if view.get("category") != CATEGORY_FOR_KIND[kind]:
        findings.append(err("continuity.category", f"{label}: category {view.get('category')!r} != "
                            f"derived {CATEGORY_FOR_KIND[kind]!r}", "Derive category from notice kind.", label))
    if view.get("plan_class") != PLAN_FOR_KIND[kind]:
        findings.append(err("continuity.plan", f"{label}: plan_class drift", "Derive plan from notice kind.", label))

    # Refresh-age + no-silent-current.
    sched = view.get("schedule", {})
    refresh_age = derive_refresh_age(sched.get("latest_refresh_at"), as_of)
    if sched.get("refresh_age") != refresh_age:
        findings.append(err("continuity.refresh_age", f"{label}: stored refresh_age "
                            f"{sched.get('refresh_age')!r} != derived {refresh_age!r}",
                            "Re-derive refresh_age from latest_refresh_at vs as_of.", label))
    declared = view.get("lifecycle", {}).get("freshness_class")
    eff, reasons = derive_effective_freshness(declared, refresh_age)
    if view.get("effective_freshness") != eff:
        findings.append(err("continuity.effective_freshness", f"{label}: effective_freshness "
                            f"{view.get('effective_freshness')!r} != derived {eff!r}",
                            "Re-derive effective_freshness; a notice is current only when active and refreshed.", label))
    if sorted(view.get("downgrade_reasons", [])) != sorted(reasons):
        findings.append(err("continuity.downgrade_reasons", f"{label}: downgrade_reasons drift",
                            "Re-derive downgrade reasons from the lifecycle and refresh age.", label))
    downgraded = eff != "current"
    if bool(view.get("freshness_downgraded")) != downgraded:
        findings.append(err("continuity.downgraded_flag", f"{label}: freshness_downgraded drift",
                            "freshness_downgraded must be true iff effective freshness is not current.", label))
    stale_label = view.get("display_copy", {}).get("stale_label")
    if downgraded and not stale_label:
        findings.append(err("continuity.stale_label.missing", f"{label}: downgraded notice has no stale label",
                            "A downgraded notice must carry a stale label.", label))
    if not downgraded and stale_label is not None:
        findings.append(err("continuity.stale_label.unexpected", f"{label}: current notice carries a stale label",
                            "A current notice must not carry a stale label.", label))

    # Boundary derivation.
    bc = view.get("boundary_change", {})
    axes = bc.get("axes", [])
    changed = sum(1 for a in axes if a.get("axis_state_class") == "changed")
    unknown = sum(1 for a in axes if a.get("axis_state_class") == "unknown_recheck_required")
    if bc.get("changed_axis_count") != changed:
        findings.append(err("continuity.boundary.changed_count", f"{label}: changed_axis_count drift",
                            "Count changed axes.", label))
    if bc.get("unknown_axis_count") != unknown:
        findings.append(err("continuity.boundary.unknown_count", f"{label}: unknown_axis_count drift",
                            "Count unknown-recheck axes.", label))
    unresolved = bool(bc.get("boundary_change_required")) and not bool(bc.get("review_completed")) and (changed + unknown > 0)
    if bool(bc.get("boundary_change_unresolved")) != unresolved:
        findings.append(err("continuity.boundary.unresolved", f"{label}: boundary_change_unresolved drift",
                            "Unresolved iff required, not reviewed, and a changed/unknown axis exists.", label))
    if bool(view.get("boundary_change_unresolved")) != unresolved:
        findings.append(err("continuity.boundary.top_unresolved", f"{label}: top-level boundary_change_unresolved drift",
                            "Mirror boundary_change.boundary_change_unresolved.", label))
    for a in axes:
        if a.get("axis_state_class") in MEANINGFUL_AXIS_STATES and not is_canonical_ref(a.get("current_ref")):
            findings.append(err("continuity.boundary.current_ref", f"{label}: changed axis "
                                f"{a.get('axis_class')} has a non-canonical current_ref",
                                "A changed/unknown axis must carry a canonical current_ref.", label))
    if bc.get("boundary_change_required") and view.get("display_copy", {}).get("boundary_change_hidden"):
        findings.append(err("continuity.boundary.hidden", f"{label}: a required boundary change is hidden",
                            "Never hide a changed boundary on a recovered state.", label))

    # Honesty marker.
    honesty = downgraded or unresolved
    if bool(view.get("honesty_marker_present")) != honesty:
        findings.append(err("continuity.honesty", f"{label}: honesty_marker_present drift",
                            "Light the marker when downgraded or boundary-unresolved.", label))

    # Write classes + preservation + separation.
    blocked = view.get("blocked_writes", [])
    succeeded = view.get("succeeded_hosted_mutations", [])
    queued = sum(1 for w in blocked if w.get("continuity_posture") == "queued_publish_later")
    drafts = sum(1 for w in blocked if w.get("continuity_posture") == "local_draft_preserved")
    no_retry = sum(1 for w in blocked if w.get("continuity_posture") == "blocked_no_safe_retry")
    manual = sum(1 for w in blocked if w.get("continuity_posture") == "requires_manual_rerun")
    succeeded_actions = {m.get("action_class") for m in succeeded}
    for w in blocked:
        posture = w.get("continuity_posture")
        if posture in PRESERVED_POSTURES:
            if not w.get("intent_preserved"):
                findings.append(err("continuity.write.preserved_flag", f"{label}: preserved write "
                                    f"{w.get('action_class')} not marked intent_preserved",
                                    "Mark preserved writes intent_preserved.", label))
            if not is_canonical_ref(w.get("queue_or_intent_ref")):
                findings.append(err("continuity.write.queue_ref", f"{label}: preserved write "
                                    f"{w.get('action_class')} lacks a canonical queue/intent ref",
                                    "Preserved writes must carry a canonical queue/intent ref.", label))
        if w.get("action_class") in succeeded_actions:
            findings.append(err("continuity.write.collapsed", f"{label}: action {w.get('action_class')} "
                                "appears both queued/blocked and succeeded",
                                "Keep queued work separate from successful hosted mutations.", label))
    for m in succeeded:
        if not is_canonical_ref(m.get("result_ref")):
            findings.append(err("continuity.mutation.result_ref", f"{label}: hosted mutation "
                                f"{m.get('action_class')} has a non-canonical result_ref",
                                "Route hosted mutations to a canonical durable object.", label))

    sc = view.get("summary_counts", {})
    expected_counts = {
        "blocked_write_count": len(blocked),
        "queued_publish_later_count": queued,
        "local_draft_preserved_count": drafts,
        "preserved_intent_count": queued + drafts,
        "blocked_no_safe_retry_count": no_retry,
        "requires_manual_rerun_count": manual,
        "succeeded_hosted_mutation_count": len(succeeded),
        "changed_boundary_axis_count": changed,
        "unknown_boundary_axis_count": unknown,
    }
    for key, want in expected_counts.items():
        if sc.get(key) != want:
            findings.append(err("continuity.summary." + key, f"{label}: summary_counts.{key} "
                                f"{sc.get(key)!r} != derived {want!r}", "Re-derive the summary counts.", label))

    # Routing.
    if not is_canonical_ref(view.get("history_ref")):
        findings.append(err("continuity.route.history", f"{label}: history_ref is not canonical",
                            "Route durable history to a canonical object.", label))
    if not is_canonical_ref(view.get("support_export_ref")):
        findings.append(err("continuity.route.support_export", f"{label}: support_export_ref is not canonical",
                            "Route support export to a canonical object.", label))

    # No-lie display invariants.
    dc = view.get("display_copy", {})
    for flag in ("all_work_broken_implied", "incident_language_for_planned_used",
                 "generic_degraded_banner_used", "queued_and_succeeded_collapsed",
                 "stale_presented_as_current", "boundary_change_hidden"):
        if dc.get(flag) is not False:
            findings.append(err("continuity.display." + flag, f"{label}: display_copy.{flag} must be false",
                                "Keep the continuity-notice no-lie invariants false.", label))

    return findings


def validate_coverage(views: dict[str, dict[str, Any]]) -> list[Finding]:
    findings: list[Finding] = []
    seen_kind: set[str] = set()
    seen_cat: set[str] = set()
    seen_eff: set[str] = set()
    seen_posture: set[str] = set()
    seen_downgrade: set[str] = set()
    seen_axis: set[str] = set()
    for view in views.values():
        seen_kind.add(view["notice_kind"])
        seen_cat.add(view["category"])
        seen_eff.add(view["effective_freshness"])
        for w in view.get("blocked_writes", []):
            seen_posture.add(w["continuity_posture"])
        for r in view.get("downgrade_reasons", []):
            seen_downgrade.add(r)
        for a in view.get("boundary_change", {}).get("axes", []):
            seen_axis.add(a["axis_state_class"])

    def missing(name: str, expected: set[str], seen: set[str]) -> None:
        gap = expected - seen
        if gap:
            findings.append(err(f"continuity.coverage.{name}",
                                f"corpus does not exercise {name}: {sorted(gap)}",
                                f"Add a drill that exercises every {name} token."))

    missing("notice_kind", NOTICE_KINDS, seen_kind)
    missing("category", CATEGORIES, seen_cat)
    missing("effective_freshness", EFFECTIVE, seen_eff)
    missing("write_continuity_posture", POSTURES, seen_posture)
    missing("downgrade_reason", DOWNGRADES, seen_downgrade)
    missing("boundary_axis_state", AXIS_STATES, seen_axis)
    return findings


# --------------------------------------------------------------------------- #
# Driver
# --------------------------------------------------------------------------- #


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".", type=Path)
    parser.add_argument("--report-json", default=None, type=Path)
    args = parser.parse_args()

    root: Path = args.repo_root
    schema_path = root / SCHEMA_REL
    fixture_dir = root / FIXTURE_DIR_REL

    if not schema_path.exists():
        print(f"[continuity-notices] missing schema: {schema_path}", file=sys.stderr)
        return 2
    if not fixture_dir.is_dir():
        print(f"[continuity-notices] missing fixture dir: {fixture_dir}", file=sys.stderr)
        return 2

    schema = json.loads(schema_path.read_text())
    validator = Draft202012Validator(schema, format_checker=FormatChecker())

    findings: list[Finding] = []
    views: dict[str, dict[str, Any]] = {}

    for path in sorted(fixture_dir.glob("*.json")):
        if path.name in NON_VIEW_FILES:
            continue
        label = path.name
        try:
            view = json.loads(path.read_text())
        except json.JSONDecodeError as exc:
            findings.append(err("continuity.json.parse", f"{label}: invalid JSON: {exc}",
                                "Emit valid JSON from the corpus.", label))
            continue
        schema_errors = sorted(validator.iter_errors(view), key=lambda e: list(e.path))
        if schema_errors:
            for e in schema_errors[:8]:
                findings.append(err("continuity.schema", f"{label}: {'/'.join(str(p) for p in e.path)}: {e.message}",
                                    "Fix the record to satisfy continuity_notice_view.schema.json.", label))
            continue
        views[label] = view
        findings.extend(check_view(label, view))

    if not views:
        findings.append(err("continuity.corpus.empty", "no continuity-notice view fixtures found",
                            "Emit fixtures with the shell corpus emitter."))
    else:
        findings.extend(validate_coverage(views))

    errors = [f for f in findings if f.severity == "error"]
    report = {
        "lane": "maintenance_and_failover_continuity_notices",
        "schema": SCHEMA_REL,
        "fixture_dir": FIXTURE_DIR_REL,
        "fixture_count": len(views),
        "ok": not errors,
        "error_count": len(errors),
        "findings": [f.as_report() for f in findings],
    }
    if args.report_json:
        args.report_json.parent.mkdir(parents=True, exist_ok=True)
        args.report_json.write_text(json.dumps(report, indent=2) + "\n")

    if errors:
        print(f"[continuity-notices] FAIL: {len(errors)} error(s) across {len(views)} fixture(s)", file=sys.stderr)
        for f in errors:
            print(f"  - {f.check_id}: {f.message}", file=sys.stderr)
        return 1

    print(f"[continuity-notices] PASS: {len(views)} fixture(s) validated and re-derived")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
