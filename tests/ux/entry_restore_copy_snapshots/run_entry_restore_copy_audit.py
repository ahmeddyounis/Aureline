#!/usr/bin/env python3
"""Unattended entry / restore placeholder audit and truth-state copy review.

Walks every case fixture under
``fixtures/ux/entry_restore_placeholder_cases/*.yaml`` and verifies that
the protected first-run / open / import / restore dogfood paths still
quote the agreed truth vocabulary instead of free-form synonyms and do
not hide missing roots, stale restores, partial recovery, or crash
conditions behind ready-sounding placeholders.

For each case the audit:

- resolves the case's ``startup_state_token`` against the audit
  vocabulary frozen in ``artifacts/ux/startup_state_copy_review.yaml``
  and ``docs/ux/entry_restore_truth_audit.md`` (no private state tokens);
- loads the upstream seed fixture under
  ``fixtures/ux/entry_restore_states/<state>.yaml`` and confirms it
  pins ``overclaims_readiness: false``, names every required
  next-safe-action hook, blocked-capability token, recovery-ladder
  rung, support-packet family, and measurement hook the case promises;
- looks up the matching copy-review row in
  ``artifacts/ux/startup_state_copy_review.yaml`` and confirms it
  also pins ``overclaims_readiness: false`` and enumerates at least
  one forbidden_label string;
- scans the case's ``dogfood_copy_snapshots`` (the rendered start-
  centre row, restore prompt, placeholder card, or state-copy example
  JSON / MD fixtures the dogfood shell paints for this state) and
  flags any field that contains one of the case's
  ``forbidden_label_patterns`` so a misleading row is caught before
  broader dogfood rollout;
- exercises a named failure drill that mutates the case state so the
  audit reports a precise ``check_id`` rather than silently passing,
  proving the lane fails loudly.

The runner emits a durable JSON capture (``--report``) and exits
non-zero on any regression. ``--force-drill <drill_id>`` replays the
named drill and exits 0 only if the audit reproduced exactly the
expected ``check_id``.

YAML decoding goes through Ruby/Psych, matching the repo-wide
convention used by adjacent audit runners.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_CASES_DIR_REL = "fixtures/ux/entry_restore_placeholder_cases"
DEFAULT_REPORT_REL = (
    "artifacts/milestones/m1/captures/"
    "entry_restore_truth_audit_validation_capture.json"
)
DEFAULT_BUILD_IDENTITY_REL = "artifacts/build/build_identity.json"
DEFAULT_COPY_REVIEW_REL = "artifacts/ux/startup_state_copy_review.yaml"
DEFAULT_SEED_FIXTURES_DIR_REL = "fixtures/ux/entry_restore_states"

REQUIRED_DOGFOOD_PATH_COVERAGE = {
    "first_run",
    "open",
    "restore",
    "placeholder_transition",
}

REQUIRED_ENTRY_VERB_COVERAGE = {
    "open",
    "clone",
    "import",
    "restore",
}

CHECK_ID_UNKNOWN_STARTUP_STATE_TOKEN = (
    "entry_restore_copy_audit.startup_state.unknown_token"
)
CHECK_ID_SOURCE_FIXTURE_MISSING = (
    "entry_restore_copy_audit.source_fixture.missing"
)
CHECK_ID_COPY_REVIEW_ROW_MISSING = (
    "entry_restore_copy_audit.copy_review_row.missing"
)
CHECK_ID_OVERCLAIM_DETECTED = "entry_restore_copy_audit.overclaim.detected"
CHECK_ID_NEXT_SAFE_ACTION_MISSING = (
    "entry_restore_copy_audit.next_safe_action_hook.missing"
)
CHECK_ID_BLOCKED_CAPABILITY_MISSING = (
    "entry_restore_copy_audit.blocked_capability_token.missing"
)
CHECK_ID_RECOVERY_OR_SUPPORT_MISSING = (
    "entry_restore_copy_audit.recovery_or_support_ref.missing"
)
CHECK_ID_MEASUREMENT_HOOK_MISSING = (
    "entry_restore_copy_audit.measurement_hook.missing"
)
CHECK_ID_FORBIDDEN_LABEL_MATCHED = (
    "entry_restore_copy_audit.forbidden_label.matched"
)
CHECK_ID_COPY_SNAPSHOT_MISSING = (
    "entry_restore_copy_audit.copy_snapshot_target.missing"
)
CHECK_ID_FORBIDDEN_LABEL_ENUMERATION_MISSING = (
    "entry_restore_copy_audit.forbidden_label_enumeration.missing"
)
CHECK_ID_DOGFOOD_PATH_COVERAGE = (
    "entry_restore_copy_audit.coverage.dogfood_path.missing"
)
CHECK_ID_ENTRY_VERB_COVERAGE = (
    "entry_restore_copy_audit.coverage.entry_verb.missing"
)
CHECK_ID_FAILURE_DRILL_EXPECTED_FINDING_MISSING = (
    "entry_restore_copy_audit.failure_drill.expected_finding_missing"
)


@dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    remediation: str
    case_id: str | None = None
    startup_state_token: str | None = None
    ref: str | None = None
    details: dict[str, Any] = field(default_factory=dict)

    def as_report(self) -> dict[str, Any]:
        payload = asdict(self)
        for key in ("ref", "case_id", "startup_state_token"):
            if payload[key] is None:
                payload.pop(key)
        if not payload["details"]:
            payload.pop("details")
        return payload


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--repo-root",
        default=".",
        help="Repository root (must contain a .git directory).",
    )
    parser.add_argument(
        "--cases-dir",
        default=DEFAULT_CASES_DIR_REL,
        help=(
            "Directory holding entry_restore_copy_audit_case_record YAML "
            "files."
        ),
    )
    parser.add_argument(
        "--report",
        default=DEFAULT_REPORT_REL,
        help="Where to write the durable JSON capture (repo-relative).",
    )
    parser.add_argument(
        "--copy-review",
        default=DEFAULT_COPY_REVIEW_REL,
        help="Path to the startup-state copy-review YAML registry.",
    )
    parser.add_argument(
        "--build-identity",
        default=DEFAULT_BUILD_IDENTITY_REL,
        help="Path to the build identity record to embed in the capture.",
    )
    parser.add_argument(
        "--force-drill",
        default=None,
        help=(
            "Replay the named failure drill (drill_id) from one of the case "
            "files. The runner injects the forced input and verifies the "
            "expected check_id is reported, then exits 0 only if the drill "
            "reproduced exactly as declared."
        ),
    )
    return parser.parse_args()


def render_yaml_as_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing YAML file: {path}")
    ruby = subprocess.run(
        [
            "ruby",
            "-rjson",
            "-ryaml",
            "-e",
            (
                "payload = YAML.safe_load(File.read(ARGV[0]), "
                "permitted_classes: [], aliases: false); "
                "STDOUT.write(JSON.generate(payload))"
            ),
            str(path),
        ],
        capture_output=True,
        text=True,
    )
    if ruby.returncode != 0:
        stderr = ruby.stderr.strip() or "unknown Ruby/Psych failure"
        raise SystemExit(
            f"failed to parse YAML at {path} via Ruby/Psych: {stderr}"
        )
    try:
        return json.loads(ruby.stdout)
    except json.JSONDecodeError as exc:
        raise SystemExit(
            f"Ruby/Psych emitted invalid JSON for {path}: {exc}"
        ) from exc


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be a YAML mapping/object")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a YAML list/array")
    return value


def ensure_str(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value.strip():
        raise SystemExit(f"{label} must be a non-empty string")
    return value.strip()


def now_iso_z() -> str:
    return (
        dt.datetime.now(dt.timezone.utc)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )


@dataclass
class CopySnapshot:
    ref: str
    fields: list[str]


@dataclass
class CaseRecord:
    case_id: str
    case_path: str
    dogfood_path_class: str
    entry_verbs_covered: list[str]
    startup_state_token: str
    source_fixture_ref: str
    copy_review_row_id: str
    required_next_safe_action_hooks: list[str]
    required_blocked_capability_tokens: list[str]
    required_recovery_ladder_rung_refs: list[str]
    required_support_packet_family_refs: list[str]
    required_journey_classes: list[str]
    required_protected_metric_refs: list[str]
    forbidden_label_patterns: list[str]
    dogfood_copy_snapshots: list[CopySnapshot]
    failure_drill: dict[str, Any]


def load_case(path: Path, repo_root: Path) -> CaseRecord:
    raw = ensure_dict(
        render_yaml_as_json(path), str(path.relative_to(repo_root))
    )
    if raw.get("record_kind") != "entry_restore_copy_audit_case_record":
        raise SystemExit(
            f"{path}: record_kind must be "
            "entry_restore_copy_audit_case_record"
        )
    schema_version = raw.get("schema_version")
    if schema_version != 1:
        raise SystemExit(
            f"{path}: schema_version must be 1, got {schema_version!r}"
        )
    case_id = ensure_str(raw.get("case_id"), f"{path}.case_id")
    dogfood_path_class = ensure_str(
        raw.get("dogfood_path_class"), f"{path}.dogfood_path_class"
    )
    entry_verbs_raw = raw.get("entry_verbs_covered", [])
    entry_verbs_covered = [
        ensure_str(v, f"{path}.entry_verbs_covered[]")
        for v in ensure_list(entry_verbs_raw, f"{path}.entry_verbs_covered")
    ]
    startup_state_token = ensure_str(
        raw.get("startup_state_token"), f"{path}.startup_state_token"
    )
    source_fixture_ref = ensure_str(
        raw.get("source_fixture_ref"), f"{path}.source_fixture_ref"
    )
    copy_review_row_id = ensure_str(
        raw.get("copy_review_row_id"), f"{path}.copy_review_row_id"
    )

    def _list_of_str(key: str, optional: bool = False) -> list[str]:
        value = raw.get(key)
        if value is None and optional:
            return []
        items = ensure_list(value, f"{path}.{key}")
        return [ensure_str(item, f"{path}.{key}[]") for item in items]

    required_hooks = _list_of_str("required_next_safe_action_hooks")
    required_blocked = _list_of_str(
        "required_blocked_capability_tokens", optional=True
    )
    required_rungs = _list_of_str("required_recovery_ladder_rung_refs")
    required_support = _list_of_str("required_support_packet_family_refs")
    required_journeys = _list_of_str("required_journey_classes")
    required_metrics = _list_of_str("required_protected_metric_refs")
    forbidden_labels = _list_of_str("forbidden_label_patterns")
    if not forbidden_labels:
        raise SystemExit(
            f"{path}.forbidden_label_patterns must list at least one "
            "regression-class string the dogfood snapshot must NOT contain"
        )

    snapshots_raw = ensure_list(
        raw.get("dogfood_copy_snapshots", []),
        f"{path}.dogfood_copy_snapshots",
    )
    dogfood_copy_snapshots: list[CopySnapshot] = []
    for idx, item in enumerate(snapshots_raw):
        row = ensure_dict(item, f"{path}.dogfood_copy_snapshots[{idx}]")
        ref = ensure_str(
            row.get("ref"), f"{path}.dogfood_copy_snapshots[{idx}].ref"
        )
        fields_raw = ensure_list(
            row.get("inspect_fields", []),
            f"{path}.dogfood_copy_snapshots[{idx}].inspect_fields",
        )
        fields = [
            ensure_str(
                v, f"{path}.dogfood_copy_snapshots[{idx}].inspect_fields[]"
            )
            for v in fields_raw
        ]
        dogfood_copy_snapshots.append(CopySnapshot(ref=ref, fields=fields))

    failure_drill = ensure_dict(
        raw.get("failure_drill"), f"{path}.failure_drill"
    )
    ensure_str(
        failure_drill.get("drill_id"), f"{path}.failure_drill.drill_id"
    )
    ensure_dict(
        failure_drill.get("forced_input"),
        f"{path}.failure_drill.forced_input",
    )
    ensure_list(
        failure_drill.get("expected_findings"),
        f"{path}.failure_drill.expected_findings",
    )

    return CaseRecord(
        case_id=case_id,
        case_path=str(path.relative_to(repo_root)),
        dogfood_path_class=dogfood_path_class,
        entry_verbs_covered=entry_verbs_covered,
        startup_state_token=startup_state_token,
        source_fixture_ref=source_fixture_ref,
        copy_review_row_id=copy_review_row_id,
        required_next_safe_action_hooks=required_hooks,
        required_blocked_capability_tokens=required_blocked,
        required_recovery_ladder_rung_refs=required_rungs,
        required_support_packet_family_refs=required_support,
        required_journey_classes=required_journeys,
        required_protected_metric_refs=required_metrics,
        forbidden_label_patterns=forbidden_labels,
        dogfood_copy_snapshots=dogfood_copy_snapshots,
        failure_drill=failure_drill,
    )


@dataclass
class CopyReviewRow:
    row_id: str
    startup_state_token: str
    overclaims_readiness: bool
    next_safe_action_hooks: list[str]
    blocked_capability_tokens: list[str]
    recovery_ladder_rung_refs: list[str]
    support_packet_family_refs: list[str]
    journey_classes: list[str]
    protected_metric_refs: list[str]
    forbidden_labels: list[str]


def load_copy_review_registry(
    path: Path,
) -> tuple[set[str], dict[str, CopyReviewRow]]:
    """Return (allowed_startup_state_tokens, rows_by_id)."""
    raw = ensure_dict(render_yaml_as_json(path), str(path))
    vocabulary = ensure_list(
        raw.get("startup_state_token_vocabulary"),
        f"{path}.startup_state_token_vocabulary",
    )
    allowed = {ensure_str(v, f"{path}.startup_state_token_vocabulary[]") for v in vocabulary}

    rows: dict[str, CopyReviewRow] = {}
    for idx, row in enumerate(
        ensure_list(raw.get("copy_review_rows"), f"{path}.copy_review_rows")
    ):
        row = ensure_dict(row, f"{path}.copy_review_rows[{idx}]")
        row_id = ensure_str(
            row.get("startup_state_row_id"),
            f"{path}.copy_review_rows[{idx}].startup_state_row_id",
        )
        token = ensure_str(
            row.get("startup_state_token"),
            f"{path}.copy_review_rows[{idx}].startup_state_token",
        )
        overclaims = bool(row.get("overclaims_readiness", False))
        hooks = [
            str(v)
            for v in (row.get("next_safe_action_hooks") or [])
            if isinstance(v, str)
        ]
        blocked = [
            str(v)
            for v in (row.get("blocked_capability_tokens") or [])
            if isinstance(v, str)
        ]
        rungs = [
            str(v)
            for v in (row.get("recovery_ladder_rung_refs") or [])
            if isinstance(v, str)
        ]
        support = [
            str(v)
            for v in (row.get("support_packet_family_refs") or [])
            if isinstance(v, str)
        ]
        measurement = row.get("measurement_hooks") or {}
        journeys = [
            str(v)
            for v in (measurement.get("journey_classes") or [])
            if isinstance(v, str)
        ]
        metrics = [
            str(v)
            for v in (measurement.get("protected_metric_refs") or [])
            if isinstance(v, str)
        ]
        forbidden = [
            str(v)
            for v in (row.get("forbidden_labels") or [])
            if isinstance(v, str)
        ]
        rows[row_id] = CopyReviewRow(
            row_id=row_id,
            startup_state_token=token,
            overclaims_readiness=overclaims,
            next_safe_action_hooks=hooks,
            blocked_capability_tokens=blocked,
            recovery_ladder_rung_refs=rungs,
            support_packet_family_refs=support,
            journey_classes=journeys,
            protected_metric_refs=metrics,
            forbidden_labels=forbidden,
        )
    return allowed, rows


@dataclass
class SourceFixtureSummary:
    overclaims_readiness: bool
    startup_state_token: str
    next_step_decision_hooks: list[str]
    blocked_capability_tokens: list[str]
    recovery_ladder_rung_refs: list[str]
    support_packet_family_refs: list[str]
    journey_classes: list[str]
    protected_metric_refs: list[str]


def summarize_source_fixture(path: Path) -> SourceFixtureSummary:
    raw = ensure_dict(render_yaml_as_json(path), str(path))
    fixture_meta = raw.get("__fixture__") or {}
    if not isinstance(fixture_meta, dict):
        fixture_meta = {}
    exercised_axes = fixture_meta.get("exercised_axes") or {}
    if not isinstance(exercised_axes, dict):
        exercised_axes = {}
    measurement_hooks = fixture_meta.get("measurement_hooks") or {}
    if not isinstance(measurement_hooks, dict):
        measurement_hooks = {}

    overclaims = bool(exercised_axes.get("overclaims_readiness", False))
    startup_state_token = str(
        raw.get("startup_state_token")
        or fixture_meta.get("startup_state")
        or ""
    )

    # Pull hooks both from top-level and from exercised_axes for robustness.
    hooks_top = raw.get("next_step_decision_hooks") or []
    hooks_axes = exercised_axes.get("next_step_decision_hooks_exercised") or []
    hooks = [str(v) for v in (list(hooks_top) + list(hooks_axes)) if isinstance(v, str)]

    blocked = [
        str(v)
        for v in (exercised_axes.get("blocked_capability_tokens") or [])
        if isinstance(v, str)
    ]
    rungs = [
        str(v)
        for v in (exercised_axes.get("recovery_ladder_rung_refs") or [])
        if isinstance(v, str)
    ]
    support = [
        str(v)
        for v in (exercised_axes.get("support_packet_family_refs") or [])
        if isinstance(v, str)
    ]
    journeys = [
        str(v)
        for v in (measurement_hooks.get("journey_classes") or [])
        if isinstance(v, str)
    ]
    metrics = [
        str(v)
        for v in (measurement_hooks.get("protected_metric_refs") or [])
        if isinstance(v, str)
    ]
    return SourceFixtureSummary(
        overclaims_readiness=overclaims,
        startup_state_token=startup_state_token,
        next_step_decision_hooks=hooks,
        blocked_capability_tokens=blocked,
        recovery_ladder_rung_refs=rungs,
        support_packet_family_refs=support,
        journey_classes=journeys,
        protected_metric_refs=metrics,
    )


def extract_field_value(payload: Any, dotted_path: str) -> tuple[bool, Any]:
    """Look up a dotted path through nested dicts; lists fan out to strings."""
    if not isinstance(payload, dict):
        return False, None
    parts = dotted_path.split(".")
    cursor: Any = payload
    for part in parts:
        if not isinstance(cursor, dict) or part not in cursor:
            return False, None
        cursor = cursor[part]
    return True, cursor


def flatten_to_strings(value: Any) -> list[str]:
    if isinstance(value, str):
        return [value]
    if isinstance(value, list):
        result: list[str] = []
        for item in value:
            result.extend(flatten_to_strings(item))
        return result
    if isinstance(value, dict):
        result = []
        for inner in value.values():
            result.extend(flatten_to_strings(inner))
        return result
    return []


def load_snapshot_payload(path: Path) -> Any:
    suffix = path.suffix.lower()
    if suffix == ".json":
        with path.open("r", encoding="utf-8") as fh:
            return json.load(fh)
    if suffix in {".yaml", ".yml"}:
        return render_yaml_as_json(path)
    # Markdown / text: return whole file content under a sentinel key so
    # callers can scan it with the same interface.
    with path.open("r", encoding="utf-8") as fh:
        return {"__raw_text__": fh.read()}


def scan_snapshot_for_forbidden_labels(
    snapshot_payload: Any,
    snapshot: CopySnapshot,
    forbidden_label_patterns: list[str],
    injected_label_hits: list[dict[str, str]],
) -> list[dict[str, Any]]:
    """Return a list of matches; each match is one (pattern, field, value)."""
    matches: list[dict[str, Any]] = []
    # 1) Injected (drill) hits — applied verbatim, always reported.
    for hit in injected_label_hits:
        matches.append(
            {
                "pattern": hit["pattern"],
                "field_path": hit["field_path"],
                "value": hit["value"],
                "injected": True,
            }
        )
    # 2) Real scan over the requested fields (or whole payload if none given).
    field_paths = snapshot.fields or ["__whole_payload__"]
    for field_path in field_paths:
        if field_path == "__whole_payload__":
            candidates = flatten_to_strings(snapshot_payload)
            display_field = "<whole payload>"
        else:
            present, value = extract_field_value(snapshot_payload, field_path)
            if not present:
                # Allow missing fields silently — different fixtures expose
                # different sub-fields. The case fixture's required fields
                # are tracked under inspect_fields; absence isn't a fault.
                continue
            candidates = flatten_to_strings(value)
            display_field = field_path
        for candidate in candidates:
            for pattern in forbidden_label_patterns:
                if pattern.lower() in candidate.lower():
                    matches.append(
                        {
                            "pattern": pattern,
                            "field_path": display_field,
                            "value": candidate,
                            "injected": False,
                        }
                    )
    return matches


def apply_drill_to_case(
    case: CaseRecord,
    source_summary: SourceFixtureSummary,
    copy_review_row: CopyReviewRow | None,
    snapshot_injections: dict[str, list[dict[str, str]]],
    findings_observed: list[Finding],
) -> tuple[SourceFixtureSummary, CopyReviewRow | None, list[str], dict[str, list[dict[str, str]]]]:
    """Mutate the inputs per the case's failure drill before evaluation."""
    drill = case.failure_drill
    forced_input = drill.get("forced_input") or {}
    if not isinstance(forced_input, dict) or not forced_input:
        raise SystemExit(
            f"{case.case_id}: failure_drill.forced_input must declare a "
            "drop_* / inject_* directive"
        )
    new_hooks = list(source_summary.next_step_decision_hooks)
    new_blocked = list(source_summary.blocked_capability_tokens)
    new_rungs = list(source_summary.recovery_ladder_rung_refs)
    new_support = list(source_summary.support_packet_family_refs)
    new_journeys = list(source_summary.journey_classes)
    new_metrics = list(source_summary.protected_metric_refs)
    new_overclaims = source_summary.overclaims_readiness
    forbidden_labels = list(case.forbidden_label_patterns)
    new_injections = {
        ref: list(items) for ref, items in snapshot_injections.items()
    }

    if copy_review_row is not None:
        review_hooks = list(copy_review_row.next_safe_action_hooks)
        review_blocked = list(copy_review_row.blocked_capability_tokens)
        review_rungs = list(copy_review_row.recovery_ladder_rung_refs)
        review_support = list(copy_review_row.support_packet_family_refs)
        review_journeys = list(copy_review_row.journey_classes)
        review_metrics = list(copy_review_row.protected_metric_refs)
        review_forbidden = list(copy_review_row.forbidden_labels)
        review_overclaims = copy_review_row.overclaims_readiness
    else:
        review_hooks = []
        review_blocked = []
        review_rungs = []
        review_support = []
        review_journeys = []
        review_metrics = []
        review_forbidden = []
        review_overclaims = False

    def _drop_all(items: list[str], value: str) -> list[str]:
        return [item for item in items if item != value]

    for directive, payload in forced_input.items():
        if directive == "drop_required_next_safe_action_hook":
            value = payload.get("hook") if isinstance(payload, dict) else None
            if isinstance(value, str):
                new_hooks = _drop_all(new_hooks, value)
                review_hooks = _drop_all(review_hooks, value)
        elif directive == "drop_required_blocked_capability_token":
            value = (
                payload.get("token") if isinstance(payload, dict) else None
            )
            if isinstance(value, str):
                new_blocked = _drop_all(new_blocked, value)
                review_blocked = _drop_all(review_blocked, value)
        elif directive == "drop_required_recovery_ladder_rung_ref":
            value = payload.get("rung") if isinstance(payload, dict) else None
            if isinstance(value, str):
                new_rungs = _drop_all(new_rungs, value)
                review_rungs = _drop_all(review_rungs, value)
        elif directive == "drop_required_support_packet_family_ref":
            value = (
                payload.get("family") if isinstance(payload, dict) else None
            )
            if isinstance(value, str):
                new_support = _drop_all(new_support, value)
                review_support = _drop_all(review_support, value)
        elif directive == "drop_required_measurement_hook":
            if not isinstance(payload, dict):
                continue
            jc = payload.get("journey_class")
            if isinstance(jc, str):
                new_journeys = _drop_all(new_journeys, jc)
                review_journeys = _drop_all(review_journeys, jc)
            metric = payload.get("protected_metric_ref")
            if isinstance(metric, str):
                new_metrics = _drop_all(new_metrics, metric)
                review_metrics = _drop_all(review_metrics, metric)
        elif directive == "force_overclaims_readiness":
            new_overclaims = True
            review_overclaims = True
        elif directive == "drop_forbidden_label_enumeration":
            review_forbidden = []
        elif directive == "inject_forbidden_label_match":
            if not isinstance(payload, dict):
                continue
            ref = payload.get("snapshot_ref")
            field_path = payload.get("field_path")
            forbidden_substring = payload.get("forbidden_label_substring")
            if (
                isinstance(ref, str)
                and isinstance(field_path, str)
                and isinstance(forbidden_substring, str)
            ):
                new_injections.setdefault(ref, []).append(
                    {
                        "pattern": forbidden_substring,
                        "field_path": field_path,
                        "value": forbidden_substring,
                    }
                )
        else:
            raise SystemExit(
                f"{case.case_id}: unknown failure_drill directive: {directive}"
            )

    new_summary = SourceFixtureSummary(
        overclaims_readiness=new_overclaims,
        startup_state_token=source_summary.startup_state_token,
        next_step_decision_hooks=new_hooks,
        blocked_capability_tokens=new_blocked,
        recovery_ladder_rung_refs=new_rungs,
        support_packet_family_refs=new_support,
        journey_classes=new_journeys,
        protected_metric_refs=new_metrics,
    )
    new_review = (
        None
        if copy_review_row is None
        else CopyReviewRow(
            row_id=copy_review_row.row_id,
            startup_state_token=copy_review_row.startup_state_token,
            overclaims_readiness=review_overclaims,
            next_safe_action_hooks=review_hooks,
            blocked_capability_tokens=review_blocked,
            recovery_ladder_rung_refs=review_rungs,
            support_packet_family_refs=review_support,
            journey_classes=review_journeys,
            protected_metric_refs=review_metrics,
            forbidden_labels=review_forbidden,
        )
    )
    return new_summary, new_review, forbidden_labels, new_injections


def evaluate_case(
    case: CaseRecord,
    repo_root: Path,
    allowed_state_tokens: set[str],
    copy_review_rows: dict[str, CopyReviewRow],
    drill_applies: bool,
) -> tuple[list[Finding], dict[str, Any]]:
    findings: list[Finding] = []

    # 1) startup_state_token must be in the closed vocabulary.
    if case.startup_state_token not in allowed_state_tokens:
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_UNKNOWN_STARTUP_STATE_TOKEN,
                message=(
                    f"case '{case.case_id}' names startup_state_token "
                    f"'{case.startup_state_token}' which is not in the "
                    "frozen copy-review vocabulary"
                ),
                remediation=(
                    "Quote a startup_state_token from "
                    "artifacts/ux/startup_state_copy_review.yaml#"
                    "startup_state_token_vocabulary or open a decision row."
                ),
                case_id=case.case_id,
                startup_state_token=case.startup_state_token,
            )
        )

    # 2) Source seed fixture must exist and pin overclaims_readiness=false.
    source_path = repo_root / case.source_fixture_ref
    if not source_path.exists():
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_SOURCE_FIXTURE_MISSING,
                message=(
                    f"case '{case.case_id}': source fixture not found at "
                    f"{case.source_fixture_ref}"
                ),
                remediation=(
                    "Add or relocate the seed fixture under "
                    "fixtures/ux/entry_restore_states/ and re-run the lane."
                ),
                case_id=case.case_id,
                startup_state_token=case.startup_state_token,
                ref=case.source_fixture_ref,
            )
        )
        source_summary = SourceFixtureSummary(
            overclaims_readiness=False,
            startup_state_token="",
            next_step_decision_hooks=[],
            blocked_capability_tokens=[],
            recovery_ladder_rung_refs=[],
            support_packet_family_refs=[],
            journey_classes=[],
            protected_metric_refs=[],
        )
    else:
        source_summary = summarize_source_fixture(source_path)

    # 3) Copy-review row must exist for the case.
    copy_review_row = copy_review_rows.get(case.copy_review_row_id)
    if copy_review_row is None:
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_COPY_REVIEW_ROW_MISSING,
                message=(
                    f"case '{case.case_id}': copy-review row "
                    f"'{case.copy_review_row_id}' not present in "
                    f"{DEFAULT_COPY_REVIEW_REL}"
                ),
                remediation=(
                    "Add the matching startup_state_copy_review_row in "
                    "artifacts/ux/startup_state_copy_review.yaml so all "
                    "three audiences (designers, docs, implementation) "
                    "resolve to the same audited state name."
                ),
                case_id=case.case_id,
                startup_state_token=case.startup_state_token,
                ref=case.copy_review_row_id,
            )
        )

    # Pre-drill: collect snapshot injections (drill mode only).
    snapshot_injections: dict[str, list[dict[str, str]]] = {}
    effective_summary = source_summary
    effective_forbidden = list(case.forbidden_label_patterns)
    if drill_applies:
        (
            effective_summary,
            copy_review_row,
            effective_forbidden,
            snapshot_injections,
        ) = apply_drill_to_case(
            case,
            source_summary,
            copy_review_row,
            snapshot_injections,
            findings,
        )

    # 4) overclaims_readiness must remain false on both source + review row.
    if effective_summary.overclaims_readiness:
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_OVERCLAIM_DETECTED,
                message=(
                    f"case '{case.case_id}': source fixture "
                    f"{case.source_fixture_ref} sets "
                    "overclaims_readiness = true; placeholder must not "
                    "imply readiness for this state"
                ),
                remediation=(
                    "Reset overclaims_readiness to false. A placeholder "
                    "that implies readiness is non-conforming per the "
                    "audit's truthfulness posture (no-Ready-overclaim)."
                ),
                case_id=case.case_id,
                startup_state_token=case.startup_state_token,
                ref=case.source_fixture_ref,
            )
        )
    if copy_review_row is not None and copy_review_row.overclaims_readiness:
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_OVERCLAIM_DETECTED,
                message=(
                    f"case '{case.case_id}': copy-review row "
                    f"{case.copy_review_row_id} sets "
                    "overclaims_readiness = true"
                ),
                remediation=(
                    "Reset overclaims_readiness to false on the "
                    "matching copy-review row."
                ),
                case_id=case.case_id,
                startup_state_token=case.startup_state_token,
                ref=case.copy_review_row_id,
            )
        )

    # 5) Required next-safe action hooks must be cited by source + review.
    source_hooks = set(effective_summary.next_step_decision_hooks)
    review_hooks = (
        set(copy_review_row.next_safe_action_hooks)
        if copy_review_row is not None
        else set()
    )
    for hook in case.required_next_safe_action_hooks:
        if hook not in source_hooks and hook not in review_hooks:
            findings.append(
                Finding(
                    severity="error",
                    check_id=CHECK_ID_NEXT_SAFE_ACTION_MISSING,
                    message=(
                        f"case '{case.case_id}': required next-safe action "
                        f"hook '{hook}' is missing from both the source "
                        "fixture and the copy-review row"
                    ),
                    remediation=(
                        "Restore the hook on the source fixture's "
                        "next_step_decision_hooks (or in exercised_axes) "
                        "and on the copy-review row's "
                        "next_safe_action_hooks. Free-form action labels "
                        "are non-conforming."
                    ),
                    case_id=case.case_id,
                    startup_state_token=case.startup_state_token,
                    ref=hook,
                )
            )

    # 6) Required blocked-capability tokens (optional for first-run).
    source_blocked = set(effective_summary.blocked_capability_tokens)
    review_blocked = (
        set(copy_review_row.blocked_capability_tokens)
        if copy_review_row is not None
        else set()
    )
    for token in case.required_blocked_capability_tokens:
        if token not in source_blocked and token not in review_blocked:
            findings.append(
                Finding(
                    severity="error",
                    check_id=CHECK_ID_BLOCKED_CAPABILITY_MISSING,
                    message=(
                        f"case '{case.case_id}': required blocked-"
                        f"capability token '{token}' is missing from both "
                        "the source fixture and the copy-review row"
                    ),
                    remediation=(
                        "Restore the token on the source fixture's "
                        "blocked_capability_tokens (exercised_axes) and "
                        "on the copy-review row's "
                        "blocked_capability_tokens."
                    ),
                    case_id=case.case_id,
                    startup_state_token=case.startup_state_token,
                    ref=token,
                )
            )

    # 7) Required recovery-ladder rungs and support-packet families.
    source_rungs = set(effective_summary.recovery_ladder_rung_refs)
    review_rungs = (
        set(copy_review_row.recovery_ladder_rung_refs)
        if copy_review_row is not None
        else set()
    )
    source_support = set(effective_summary.support_packet_family_refs)
    review_support = (
        set(copy_review_row.support_packet_family_refs)
        if copy_review_row is not None
        else set()
    )
    for rung in case.required_recovery_ladder_rung_refs:
        if rung not in source_rungs and rung not in review_rungs:
            findings.append(
                Finding(
                    severity="error",
                    check_id=CHECK_ID_RECOVERY_OR_SUPPORT_MISSING,
                    message=(
                        f"case '{case.case_id}': required recovery-ladder "
                        f"rung '{rung}' is missing"
                    ),
                    remediation=(
                        "Restore the rung on the source fixture and the "
                        "copy-review row so support / recovery surfaces "
                        "resolve by id."
                    ),
                    case_id=case.case_id,
                    startup_state_token=case.startup_state_token,
                    ref=rung,
                )
            )
    for family in case.required_support_packet_family_refs:
        if family not in source_support and family not in review_support:
            findings.append(
                Finding(
                    severity="error",
                    check_id=CHECK_ID_RECOVERY_OR_SUPPORT_MISSING,
                    message=(
                        f"case '{case.case_id}': required support-packet "
                        f"family '{family}' is missing"
                    ),
                    remediation=(
                        "Restore the support_packet_family_ref on the "
                        "source fixture and the copy-review row."
                    ),
                    case_id=case.case_id,
                    startup_state_token=case.startup_state_token,
                    ref=family,
                )
            )

    # 8) Required measurement hooks.
    source_journeys = set(effective_summary.journey_classes)
    review_journeys = (
        set(copy_review_row.journey_classes)
        if copy_review_row is not None
        else set()
    )
    source_metrics = set(effective_summary.protected_metric_refs)
    review_metrics = (
        set(copy_review_row.protected_metric_refs)
        if copy_review_row is not None
        else set()
    )
    for jc in case.required_journey_classes:
        if jc not in source_journeys and jc not in review_journeys:
            findings.append(
                Finding(
                    severity="error",
                    check_id=CHECK_ID_MEASUREMENT_HOOK_MISSING,
                    message=(
                        f"case '{case.case_id}': required journey class "
                        f"'{jc}' is missing"
                    ),
                    remediation=(
                        "Restore the journey_class on the source "
                        "fixture's measurement_hooks and on the "
                        "copy-review row so later telemetry can fire "
                        "without inventing names."
                    ),
                    case_id=case.case_id,
                    startup_state_token=case.startup_state_token,
                    ref=jc,
                )
            )
    for metric in case.required_protected_metric_refs:
        if metric not in source_metrics and metric not in review_metrics:
            findings.append(
                Finding(
                    severity="error",
                    check_id=CHECK_ID_MEASUREMENT_HOOK_MISSING,
                    message=(
                        f"case '{case.case_id}': required protected metric "
                        f"'{metric}' is missing"
                    ),
                    remediation=(
                        "Restore the protected metric on the source "
                        "fixture's measurement_hooks and on the "
                        "copy-review row."
                    ),
                    case_id=case.case_id,
                    startup_state_token=case.startup_state_token,
                    ref=metric,
                )
            )

    # 9) Copy-review row must enumerate at least one forbidden label.
    if (
        copy_review_row is not None
        and not copy_review_row.forbidden_labels
    ):
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_FORBIDDEN_LABEL_ENUMERATION_MISSING,
                message=(
                    f"case '{case.case_id}': copy-review row "
                    f"{case.copy_review_row_id} does not enumerate any "
                    "forbidden_labels"
                ),
                remediation=(
                    "Add at least one forbidden_label string (e.g. "
                    "'Workspace ready', 'Session restored') to the row so "
                    "reviewers can spot overclaim regressions at copy "
                    "review."
                ),
                case_id=case.case_id,
                startup_state_token=case.startup_state_token,
                ref=case.copy_review_row_id,
            )
        )

    # 10) Dogfood copy snapshots: each snapshot file must exist and must
    #     not contain any forbidden label string in the inspected fields.
    snapshot_diagnostics: list[dict[str, Any]] = []
    for snapshot in case.dogfood_copy_snapshots:
        snapshot_path = repo_root / snapshot.ref
        if not snapshot_path.exists():
            findings.append(
                Finding(
                    severity="error",
                    check_id=CHECK_ID_COPY_SNAPSHOT_MISSING,
                    message=(
                        f"case '{case.case_id}': dogfood copy snapshot "
                        f"{snapshot.ref} does not exist"
                    ),
                    remediation=(
                        "Restore the rendered surface fixture or update "
                        "the case's dogfood_copy_snapshots list to the "
                        "new path."
                    ),
                    case_id=case.case_id,
                    startup_state_token=case.startup_state_token,
                    ref=snapshot.ref,
                )
            )
            snapshot_diagnostics.append(
                {
                    "ref": snapshot.ref,
                    "present": False,
                    "inspected_fields": snapshot.fields,
                    "matches": [],
                }
            )
            continue
        payload = load_snapshot_payload(snapshot_path)
        injected_hits = snapshot_injections.get(snapshot.ref, [])
        matches = scan_snapshot_for_forbidden_labels(
            payload,
            snapshot,
            effective_forbidden,
            injected_hits,
        )
        snapshot_diagnostics.append(
            {
                "ref": snapshot.ref,
                "present": True,
                "inspected_fields": snapshot.fields,
                "matches": matches,
            }
        )
        for match in matches:
            findings.append(
                Finding(
                    severity="error",
                    check_id=CHECK_ID_FORBIDDEN_LABEL_MATCHED,
                    message=(
                        f"case '{case.case_id}': dogfood snapshot "
                        f"{snapshot.ref} field {match['field_path']!r} "
                        f"contains forbidden label fragment "
                        f"{match['pattern']!r}"
                    ),
                    remediation=(
                        "Rewrite the surface copy to quote the "
                        "controlled truth vocabulary (e.g. the advertised "
                        "restore_level, 'Compatible restore' / 'Layout "
                        "only' / 'Recovered drafts' / 'Evidence only', "
                        "or a typed degraded-state label) instead of a "
                        "ready-sounding synonym."
                    ),
                    case_id=case.case_id,
                    startup_state_token=case.startup_state_token,
                    ref=snapshot.ref,
                    details={
                        "field_path": match["field_path"],
                        "matched_pattern": match["pattern"],
                        "value": match["value"],
                        "injected": match["injected"],
                    },
                )
            )

    diagnostics = {
        "startup_state_token": case.startup_state_token,
        "source_fixture_ref": case.source_fixture_ref,
        "copy_review_row_id": case.copy_review_row_id,
        "dogfood_path_class": case.dogfood_path_class,
        "entry_verbs_covered": case.entry_verbs_covered,
        "source_summary": {
            "overclaims_readiness": effective_summary.overclaims_readiness,
            "next_step_decision_hooks": sorted(source_hooks),
            "blocked_capability_tokens": sorted(source_blocked),
            "recovery_ladder_rung_refs": sorted(source_rungs),
            "support_packet_family_refs": sorted(source_support),
            "journey_classes": sorted(source_journeys),
            "protected_metric_refs": sorted(source_metrics),
        },
        "copy_review_summary": (
            None
            if copy_review_row is None
            else {
                "overclaims_readiness": copy_review_row.overclaims_readiness,
                "next_safe_action_hooks": sorted(review_hooks),
                "forbidden_labels": list(copy_review_row.forbidden_labels),
            }
        ),
        "snapshots": snapshot_diagnostics,
        "drill_applied": drill_applies,
    }
    return findings, diagnostics


def expected_findings_match(
    findings: list[Finding], expected_rows: list[Any]
) -> tuple[bool, list[str]]:
    missing: list[str] = []
    for expected in expected_rows:
        check_id = (
            expected.get("check_id") if isinstance(expected, dict) else None
        )
        message_contains = (
            expected.get("message_contains")
            if isinstance(expected, dict)
            else None
        )
        matched = False
        for finding in findings:
            if finding.check_id != check_id:
                continue
            if isinstance(message_contains, str) and message_contains:
                if message_contains not in finding.message:
                    continue
            matched = True
            break
        if not matched:
            missing.append(
                f"check_id={check_id!r}, message_contains={message_contains!r}"
            )
    return len(missing) == 0, missing


def find_drill_case(
    cases: list[CaseRecord], drill_id: str
) -> CaseRecord | None:
    for case in cases:
        if case.failure_drill.get("drill_id") == drill_id:
            return case
    return None


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(
            f"--repo-root does not look like a repository root: {repo_root}"
        )

    cases_dir = repo_root / args.cases_dir
    if not cases_dir.is_dir():
        raise SystemExit(f"cases dir not found: {args.cases_dir}")

    case_paths = sorted(p for p in cases_dir.glob("*.yaml"))
    if not case_paths:
        raise SystemExit(
            f"no entry_restore_copy_audit cases found in {args.cases_dir}"
        )

    cases = [load_case(p, repo_root) for p in case_paths]

    seen_ids: set[str] = set()
    seen_path_classes: set[str] = set()
    seen_entry_verbs: set[str] = set()
    for case in cases:
        if case.case_id in seen_ids:
            raise SystemExit(f"duplicate case_id: {case.case_id}")
        seen_ids.add(case.case_id)
        seen_path_classes.add(case.dogfood_path_class)
        seen_entry_verbs.update(case.entry_verbs_covered)

    copy_review_path = repo_root / args.copy_review
    allowed_state_tokens, copy_review_rows = load_copy_review_registry(
        copy_review_path
    )

    drill_mode = args.force_drill is not None
    drill_case: CaseRecord | None = None
    if drill_mode:
        drill_case = find_drill_case(cases, args.force_drill)
        if drill_case is None:
            raise SystemExit(
                f"--force-drill: no case declares drill_id "
                f"'{args.force_drill}'"
            )

    # Coverage findings.
    coverage_findings: list[Finding] = []
    missing_paths = REQUIRED_DOGFOOD_PATH_COVERAGE - seen_path_classes
    if missing_paths:
        coverage_findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_DOGFOOD_PATH_COVERAGE,
                message=(
                    "cases must seed at least one row each for dogfood "
                    f"path classes {sorted(REQUIRED_DOGFOOD_PATH_COVERAGE)};"
                    f" missing: {sorted(missing_paths)}"
                ),
                remediation=(
                    "Add the missing dogfood-path-class row so the "
                    "protected walk covers first-run, open, restore, and "
                    "placeholder-transition paths."
                ),
            )
        )
    missing_verbs = REQUIRED_ENTRY_VERB_COVERAGE - seen_entry_verbs
    if missing_verbs:
        coverage_findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_ENTRY_VERB_COVERAGE,
                message=(
                    "cases must collectively cover entry verbs "
                    f"{sorted(REQUIRED_ENTRY_VERB_COVERAGE)}; missing: "
                    f"{sorted(missing_verbs)}"
                ),
                remediation=(
                    "Extend entry_verbs_covered on one of the cases so the "
                    "open / clone / import / restore verbs all show up "
                    "across the protected walk."
                ),
            )
        )

    case_results: list[dict[str, Any]] = []
    all_findings: list[Finding] = list(coverage_findings)

    for case in cases:
        is_drill_case = (
            drill_mode
            and drill_case is not None
            and case.case_id == drill_case.case_id
        )
        findings, diagnostics = evaluate_case(
            case,
            repo_root,
            allowed_state_tokens,
            copy_review_rows,
            drill_applies=is_drill_case,
        )
        case_record: dict[str, Any] = {
            "case_id": case.case_id,
            "case_path": case.case_path,
            "dogfood_path_class": case.dogfood_path_class,
            "entry_verbs_covered": case.entry_verbs_covered,
            "startup_state_token": case.startup_state_token,
            "diagnostics": diagnostics,
            "finding_count": len(findings),
            "findings": [f.as_report() for f in findings],
        }
        if is_drill_case:
            expected_rows = case.failure_drill.get("expected_findings", [])
            ok, missing = expected_findings_match(findings, expected_rows)
            case_record["failure_drill"] = {
                "drill_id": case.failure_drill.get("drill_id"),
                "expected_findings": expected_rows,
                "expected_findings_observed": ok,
                "missing_expected_findings": missing,
            }
            if not ok:
                all_findings.append(
                    Finding(
                        severity="error",
                        check_id=CHECK_ID_FAILURE_DRILL_EXPECTED_FINDING_MISSING,
                        message=(
                            f"failure drill "
                            f"{case.failure_drill.get('drill_id')!r} did "
                            "not surface the expected findings: "
                            f"{missing}"
                        ),
                        remediation=(
                            "Either restore the audit logic to detect "
                            "this regression class, or update the case "
                            "fixture if the regression class genuinely "
                            "changed."
                        ),
                        case_id=case.case_id,
                        startup_state_token=case.startup_state_token,
                        ref=case.failure_drill.get("drill_id"),
                    )
                )
        else:
            all_findings.extend(findings)
        case_results.append(case_record)

    errors = [f for f in all_findings if f.severity == "error"]
    if drill_mode:
        non_drill_failures = [
            f
            for f in errors
            if f.check_id == CHECK_ID_FAILURE_DRILL_EXPECTED_FINDING_MISSING
        ]
        status = "PASS" if not non_drill_failures else "FAIL"
    else:
        status = "PASS" if not errors else "FAIL"

    capture: dict[str, Any] = {
        "schema_version": 1,
        "capture_kind": "entry_restore_truth_audit_validation_capture",
        "captured_at": now_iso_z(),
        "owner_dri": "@ahmeddyounis",
        "cases_dir_ref": args.cases_dir,
        "copy_review_ref": args.copy_review,
        "case_count": len(cases),
        "drill_mode": drill_mode,
        "drill_id": args.force_drill,
        "exact_build_identity_ref": args.build_identity,
        "command": (
            "python3 tests/ux/entry_restore_copy_snapshots/"
            "run_entry_restore_copy_audit.py --repo-root ."
            + (f" --force-drill {args.force_drill}" if drill_mode else "")
        ),
        "required_dogfood_path_coverage": sorted(
            REQUIRED_DOGFOOD_PATH_COVERAGE
        ),
        "observed_dogfood_path_classes": sorted(seen_path_classes),
        "required_entry_verb_coverage": sorted(REQUIRED_ENTRY_VERB_COVERAGE),
        "observed_entry_verbs": sorted(seen_entry_verbs),
        "status": status,
        "cases": case_results,
        "finding_counts": {
            "error": sum(1 for f in all_findings if f.severity == "error"),
            "warning": sum(1 for f in all_findings if f.severity == "warning"),
        },
        "findings": [f.as_report() for f in all_findings],
    }

    report_path = repo_root / args.report
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text(
        json.dumps(capture, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )

    print(
        f"[entry-restore-copy-audit] {status} "
        f"({len(cases)} cases, {len(errors)} errors, "
        f"{sum(1 for f in all_findings if f.severity == 'warning')} "
        f"warnings) — capture: {args.report}"
    )
    for finding in all_findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        case_suffix = (
            f" {{{finding.case_id}}}" if finding.case_id else ""
        )
        print(
            f"[entry-restore-copy-audit] {prefix}{case_suffix} "
            f"{finding.check_id}: {finding.message}{ref_suffix}"
        )
        print(
            f"[entry-restore-copy-audit]   remediation: "
            f"{finding.remediation}"
        )

    return 0 if status == "PASS" else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[entry-restore-copy-audit] interrupted", file=sys.stderr)
        sys.exit(130)
