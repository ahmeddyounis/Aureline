#!/usr/bin/env python3
"""Unattended onboarding task-success and first-useful-work lane.

Walks every measurement row under
``fixtures/ux/onboarding_task_success_rows/*.yaml`` and verifies that
the five protected onboarding paths (Open folder, Open workspace,
Clone repository, Restore last session, Import from…) emit
structured task-success / first-useful-work signals in one shared
vocabulary instead of a per-surface private notion of success.

For each row the runner:

- loads the schema-frozen closed vocabularies
  (``schemas/telemetry/m1_onboarding_metrics.schema.json``);
- joins the row against the canonical sources — the onboarding
  measurement plan (``docs/product/onboarding_measurement_plan.md``)
  for entry-route / failure-category / primary-event vocab, the
  first-useful-work qualification corpus
  (``artifacts/ux/first_useful_work_corpus/*.yaml``) for the
  ``corpus_row_ref``, and the worked fixtures
  (``fixtures/ux/first_useful_work_cases/*.yaml``) for the
  ``supporting_fixture_ref``;
- asserts that every ``expected_failure_category_subset`` value is
  a subset of the closed per-surface vocabulary in the measurement
  plan §3, and every ``expected_primary_events`` value is in the
  surface's closed primary-events list;
- asserts that ``expected_first_useful_work_target_surface``
  matches both the corpus row and the worked fixture, that
  ``entry_route_id`` and ``measurement_surface`` agree across the
  three sources, and that each ``expected_protected_metric_refs``
  value is also listed in the corpus row's
  ``protected_metric_refs`` (so the lane never claims a metric the
  corpus row does not also qualify);
- asserts that the row distinguishes first useful work from mere
  app launch: a ``completion_checkpoint_class = first_useful_edit``
  paired with ``expected_completion_class =
  completed_first_useful_navigation_only`` is non-conforming;
- asserts the privacy / telemetry-default posture is one of the
  closed values, with ``telemetry_default_posture = opt_in_only``
  the canonical posture for M1;
- asserts the lane covers every required onboarding path class
  (the five protected entry verbs) and every required measurement
  surface that the lane's required-path coverage implies;
- exercises a named failure drill that mutates one of the row's
  inputs so the runner reproduces a precise ``check_id`` rather
  than silently passing, proving the lane fails loudly.

The runner emits a durable JSON capture (``--report``) and exits
non-zero on any regression. ``--force-drill <drill_id>`` replays
the named drill and exits 0 only when the runner reproduced
exactly the expected ``check_id``.

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


DEFAULT_ROWS_DIR_REL = "fixtures/ux/onboarding_task_success_rows"
DEFAULT_REPORT_REL = (
    "artifacts/milestones/m1/captures/"
    "onboarding_task_success_validation_capture.json"
)
DEFAULT_SCHEMA_REL = (
    "schemas/telemetry/m1_onboarding_metrics.schema.json"
)
DEFAULT_BUILD_IDENTITY_REL = "artifacts/build/build_identity.json"
DEFAULT_CORPUS_DIR_REL = "artifacts/ux/first_useful_work_corpus"


# Closed measurement-plan vocabularies (frozen, mirroring
# docs/product/onboarding_measurement_plan.md §3.x). Adding a value
# is additive-minor; this runner refuses to admit a row whose
# vocab is outside these sets.

SURFACE_FAILURE_CATEGORIES: dict[str, set[str]] = {
    "surface_first_run": {
        "forced_sign_in_before_useful_local_work",
        "forced_marketplace_detour",
        "forced_tour_blocking_useful_work",
        "start_center_unreachable",
        "entry_verb_collapsed_into_get_started",
        "network_required_for_local_entry",
        "aborted_before_admission",
    },
    "surface_first_open": {
        "target_kind_unresolved",
        "admission_denied_policy",
        "admission_denied_trust",
        "admission_denied_needs_repair",
        "admission_denied_needs_reconnect",
        "admission_denied_needs_reauth",
        "resulting_mode_silently_downgraded",
        "destination_disposition_mismatch",
        "collision_unreviewed_before_commit",
    },
    "surface_first_useful_edit": {
        "editor_blocked_on_index_warmup",
        "save_blocked_on_service",
        "recovery_journal_unavailable",
        "edit_lost_before_journal",
        "trust_gate_blocks_edit_without_explanation",
        "buffer_read_only_unexplained",
        "first_edit_required_sign_in",
    },
    "surface_migration_review": {
        "dry_run_skipped",
        "outcome_aggregated_not_per_item",
        "unsupported_items_hidden",
        "rollback_checkpoint_missing",
        "rollback_failed",
        "rollback_side_effect_leak",
        "parity_score_aggregate_hid_weak_category",
        "needs_review_not_flagged",
        "blocked_reason_free_form",
    },
    "surface_restore_success": {
        "restore_level_promised_higher_than_delivered",
        "missing_target_state_silently_dropped",
        "corrupt_restorable_state_silently_discarded",
        "silent_mutating_command_replay",
        "dirty_buffer_lost_without_journal",
        "recovery_class_free_form",
        "live_session_inferred_from_absence",
    },
    "surface_opt_in_boundary": {
        "opt_in_forced_to_reach_useful_work",
        "decline_degraded_prior_local_flow",
        "continue_local_hidden_or_subordinate",
        "absence_narrowing_undeclared",
        "service_error_collapsed_into_needs_account",
        "retroactive_lockout_after_decline",
    },
}

SURFACE_PRIMARY_EVENTS: dict[str, set[str]] = {
    "surface_first_run": {
        "first_run_reached",
        "first_run_entry_route_selected",
        "first_run_admitted",
        "first_run_abandoned",
        "first_run_failure_classified",
    },
    "surface_first_open": {
        "entry_verb_resolved",
        "target_kind_classified",
        "resulting_mode_committed",
        "admission_decided",
        "first_open_completed",
        "first_open_denied",
    },
    "surface_first_useful_edit": {
        "first_useful_navigation_reached",
        "first_useful_edit_started",
        "first_useful_edit_durable",
        "first_useful_edit_blocked",
    },
    "surface_migration_review": {
        "migration_dry_run_produced",
        "migration_parity_scored",
        "migration_applied",
        "migration_rolled_back",
        "migration_outcome_recorded",
        "migration_rollback_checkpoint_written",
        "migration_rollback_checkpoint_restored",
    },
    "surface_restore_success": {
        "restore_prompt_presented",
        "restore_level_advertised",
        "restore_level_delivered",
        "restore_missing_target_classified",
        "restore_recovery_class_selected",
        "restore_completed",
        "restore_abandoned",
    },
    "surface_opt_in_boundary": {
        "opt_in_prompt_presented",
        "opt_in_accepted",
        "opt_in_declined",
        "continue_local_selected",
        "narrowed_capability_advertised",
        "local_flow_continued_after_decline",
        "local_flow_degraded_after_decline",
    },
}

ENTRY_ROUTE_VOCAB: set[str] = {
    "er.start_center",
    "er.recent_work_reopen",
    "er.restore_prompt",
    "er.protocol_handler_reentry",
    "er.clone_or_import",
    "er.plain_open",
    "er.workspace_switch",
    "er.warm_start",
}

COMPLETION_CLASS_VOCAB: set[str] = {
    "completed_first_useful_edit",
    "completed_first_useful_navigation_only",
    "completed_with_advertised_narrowing",
    "completed_migration_committed_per_item",
    "completed_restore_level_delivered",
    "completed_decline_without_degradation",
    "aborted_before_admission",
    "abandoned_after_admission",
    "failed_with_typed_blocker",
}

COMPLETION_CHECKPOINT_VOCAB: set[str] = {
    "first_useful_navigation",
    "first_useful_edit",
    "migration_committed_with_per_item_outcomes",
    "restore_level_delivered",
    "decline_continued_without_degradation",
}

PRIVACY_CLASS_VOCAB: set[str] = {
    "privacy_local_only_no_emission",
    "privacy_opt_in_aggregate_only",
    "privacy_opt_in_attributable",
}

TELEMETRY_POSTURE_VOCAB: set[str] = {
    "opt_in_only",
    "off_by_default_no_emission_until_consent",
}

REQUIRED_ONBOARDING_PATH_COVERAGE: set[str] = {
    "open_folder",
    "open_workspace",
    "clone_repository",
    "restore_last_session",
    "import_from_external",
}


# Check ids the runner emits. Stable strings — failure-drill
# expected_check_id values resolve against these.
CHECK_ID_SCHEMA_VIOLATION = "onboarding_task_success.schema.violation"
CHECK_ID_ENTRY_ROUTE_UNKNOWN = (
    "onboarding_task_success.entry_route.unknown"
)
CHECK_ID_MEASUREMENT_SURFACE_UNKNOWN = (
    "onboarding_task_success.measurement_surface.unknown"
)
CHECK_ID_FAILURE_CATEGORY_OUTSIDE_VOCAB = (
    "onboarding_task_success.failure_category.outside_surface_vocab"
)
CHECK_ID_PRIMARY_EVENT_OUTSIDE_VOCAB = (
    "onboarding_task_success.primary_event.outside_surface_vocab"
)
CHECK_ID_COMPLETION_CLASS_UNKNOWN = (
    "onboarding_task_success.completion_class.unknown"
)
CHECK_ID_CHECKPOINT_UNKNOWN = (
    "onboarding_task_success.completion_checkpoint.unknown"
)
CHECK_ID_PRIVACY_CLASS_UNKNOWN = (
    "onboarding_task_success.privacy_class.unknown"
)
CHECK_ID_TELEMETRY_POSTURE_UNKNOWN = (
    "onboarding_task_success.telemetry_default_posture.unknown"
)
CHECK_ID_CORPUS_ROW_MISSING = (
    "onboarding_task_success.corpus_row.missing"
)
CHECK_ID_CORPUS_ROW_MISMATCH = (
    "onboarding_task_success.corpus_row.mismatch"
)
CHECK_ID_FIXTURE_MISSING = (
    "onboarding_task_success.supporting_fixture.missing"
)
CHECK_ID_FIXTURE_MISMATCH = (
    "onboarding_task_success.supporting_fixture.mismatch"
)
CHECK_ID_TARGET_SURFACE_MISMATCH = (
    "onboarding_task_success.first_useful_work_target_surface.mismatch"
)
CHECK_ID_PROTECTED_METRIC_UNBOUND = (
    "onboarding_task_success.protected_metric.unbound_in_corpus_row"
)
CHECK_ID_CHECKPOINT_COLLAPSED = (
    "onboarding_task_success.first_useful_work_checkpoint.collapsed_into_app_launch"
)
CHECK_ID_PATH_COVERAGE_MISSING = (
    "onboarding_task_success.coverage.onboarding_path.missing"
)
CHECK_ID_DUPLICATE_ROW = (
    "onboarding_task_success.row.duplicate_id"
)
CHECK_ID_FAILURE_DRILL_MISSING_EXPECTED = (
    "onboarding_task_success.failure_drill.expected_finding_missing"
)


@dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    remediation: str
    row_id: str | None = None
    onboarding_path_class: str | None = None
    ref: str | None = None
    details: dict[str, Any] = field(default_factory=dict)

    def as_report(self) -> dict[str, Any]:
        payload = asdict(self)
        for key in ("ref", "row_id", "onboarding_path_class"):
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
        "--rows-dir",
        default=DEFAULT_ROWS_DIR_REL,
        help="Directory of onboarding measurement row YAML files.",
    )
    parser.add_argument(
        "--corpus-dir",
        default=DEFAULT_CORPUS_DIR_REL,
        help=(
            "Directory holding the first-useful-work qualification "
            "corpus YAML files."
        ),
    )
    parser.add_argument(
        "--report",
        default=DEFAULT_REPORT_REL,
        help="Where to write the durable JSON capture (repo-relative).",
    )
    parser.add_argument(
        "--schema",
        default=DEFAULT_SCHEMA_REL,
        help="Path to the measurement-row schema (sanity-checked).",
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
            "Replay the named failure drill (drill_id) from one of "
            "the row files. The runner injects the forced input and "
            "exits 0 only if the row's declared expected_check_id "
            "is reproduced exactly."
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
class MeasurementRow:
    row_path: str
    measurement_row_id: str
    onboarding_path_class: str
    entry_route_id: str
    measurement_surface: str
    completion_checkpoint_class: str
    corpus_row_ref: str
    supporting_fixture_ref: str
    task_success_corpus_scenario_refs: list[str]
    expected_first_useful_work_target_surface: str
    expected_admission_class: str
    expected_completion_class: str
    expected_failure_category_subset: list[str]
    expected_primary_events: list[str]
    expected_protected_metric_refs: list[str]
    telemetry_default_posture: str
    privacy_class: str
    deployment_profile_id: str
    owner_dri: str
    failure_drill: dict[str, Any]


def load_measurement_row(path: Path, repo_root: Path) -> MeasurementRow:
    raw = ensure_dict(
        render_yaml_as_json(path), str(path.relative_to(repo_root))
    )
    record_kind = raw.get("record_kind")
    if record_kind != "onboarding_task_success_measurement_row":
        raise SystemExit(
            f"{path}: record_kind must be "
            f"onboarding_task_success_measurement_row, got "
            f"{record_kind!r}"
        )
    schema_version = raw.get("schema_version")
    if schema_version != 1:
        raise SystemExit(
            f"{path}: schema_version must be 1, got {schema_version!r}"
        )

    def _str(key: str) -> str:
        return ensure_str(raw.get(key), f"{path}.{key}")

    def _list_str(key: str) -> list[str]:
        items = ensure_list(raw.get(key), f"{path}.{key}")
        return [ensure_str(item, f"{path}.{key}[]") for item in items]

    failure_drill = ensure_dict(
        raw.get("failure_drill"), f"{path}.failure_drill"
    )
    ensure_str(
        failure_drill.get("drill_id"), f"{path}.failure_drill.drill_id"
    )
    ensure_str(
        failure_drill.get("expected_check_id"),
        f"{path}.failure_drill.expected_check_id",
    )
    ensure_dict(
        failure_drill.get("forced_input"),
        f"{path}.failure_drill.forced_input",
    )
    ensure_str(
        failure_drill.get("actionable_next_action"),
        f"{path}.failure_drill.actionable_next_action",
    )
    ensure_str(
        failure_drill.get("description"),
        f"{path}.failure_drill.description",
    )

    return MeasurementRow(
        row_path=str(path.relative_to(repo_root)),
        measurement_row_id=_str("measurement_row_id"),
        onboarding_path_class=_str("onboarding_path_class"),
        entry_route_id=_str("entry_route_id"),
        measurement_surface=_str("measurement_surface"),
        completion_checkpoint_class=_str("completion_checkpoint_class"),
        corpus_row_ref=_str("corpus_row_ref"),
        supporting_fixture_ref=_str("supporting_fixture_ref"),
        task_success_corpus_scenario_refs=_list_str(
            "task_success_corpus_scenario_refs"
        ),
        expected_first_useful_work_target_surface=_str(
            "expected_first_useful_work_target_surface"
        ),
        expected_admission_class=_str("expected_admission_class"),
        expected_completion_class=_str("expected_completion_class"),
        expected_failure_category_subset=_list_str(
            "expected_failure_category_subset"
        ),
        expected_primary_events=_list_str("expected_primary_events"),
        expected_protected_metric_refs=_list_str(
            "expected_protected_metric_refs"
        ),
        telemetry_default_posture=_str("telemetry_default_posture"),
        privacy_class=_str("privacy_class"),
        deployment_profile_id=_str("deployment_profile_id"),
        owner_dri=_str("owner_dri"),
        failure_drill=failure_drill,
    )


@dataclass
class CorpusRowSummary:
    row_id: str
    entry_route_id: str
    measurement_surface: str
    first_useful_work_target_surface: str
    protected_metric_refs: list[str]


def load_corpus_rows(
    corpus_dir: Path,
) -> dict[str, CorpusRowSummary]:
    """Read every fuw corpus YAML and index rows by row_id."""
    rows: dict[str, CorpusRowSummary] = {}
    for yaml_path in sorted(corpus_dir.glob("*.yaml")):
        payload = ensure_dict(
            render_yaml_as_json(yaml_path), str(yaml_path)
        )
        raw_rows = payload.get("rows")
        if not isinstance(raw_rows, list):
            continue
        for idx, raw_row in enumerate(raw_rows):
            row = ensure_dict(
                raw_row, f"{yaml_path}.rows[{idx}]"
            )
            row_id = ensure_str(
                row.get("row_id"), f"{yaml_path}.rows[{idx}].row_id"
            )
            entry_route_id = ensure_str(
                row.get("entry_route_id"),
                f"{yaml_path}.rows[{idx}].entry_route_id",
            )
            measurement_surface = ensure_str(
                row.get("measurement_surface"),
                f"{yaml_path}.rows[{idx}].measurement_surface",
            )
            target_surface = ensure_str(
                row.get("first_useful_work_target_surface"),
                f"{yaml_path}.rows[{idx}]."
                "first_useful_work_target_surface",
            )
            metric_refs = [
                str(v)
                for v in (row.get("protected_metric_refs") or [])
                if isinstance(v, str)
            ]
            rows[row_id] = CorpusRowSummary(
                row_id=row_id,
                entry_route_id=entry_route_id,
                measurement_surface=measurement_surface,
                first_useful_work_target_surface=target_surface,
                protected_metric_refs=metric_refs,
            )
    return rows


@dataclass
class WorkedFixtureSummary:
    fixture_path: str
    entry_route_id: str
    measurement_surface: str
    first_useful_work_target_surface: str
    protected_metric_refs: list[str]
    forbidden_failure_categories: list[str]


def load_worked_fixture(path: Path) -> WorkedFixtureSummary:
    raw = ensure_dict(render_yaml_as_json(path), str(path))
    entry_route_id = ensure_str(
        raw.get("entry_route_id"), f"{path}.entry_route_id"
    )
    measurement_surface = ensure_str(
        raw.get("measurement_surface"),
        f"{path}.measurement_surface",
    )
    first_useful_work_target_surface = ensure_str(
        raw.get("first_useful_work_target_surface"),
        f"{path}.first_useful_work_target_surface",
    )
    protected_metric_refs = [
        str(v)
        for v in (raw.get("protected_metric_refs") or [])
        if isinstance(v, str)
    ]
    forbidden_failure_categories = [
        str(v)
        for v in (raw.get("forbidden_failure_categories") or [])
        if isinstance(v, str)
    ]
    return WorkedFixtureSummary(
        fixture_path=str(path),
        entry_route_id=entry_route_id,
        measurement_surface=measurement_surface,
        first_useful_work_target_surface=(
            first_useful_work_target_surface
        ),
        protected_metric_refs=protected_metric_refs,
        forbidden_failure_categories=forbidden_failure_categories,
    )


@dataclass
class EvaluationInputs:
    row: MeasurementRow
    expected_completion_class: str
    expected_failure_category_subset: list[str]
    expected_primary_events: list[str]
    privacy_class: str
    telemetry_default_posture: str


def apply_drill(
    row: MeasurementRow,
) -> EvaluationInputs:
    """Apply the row's named drill, returning mutated inputs."""
    drill = row.failure_drill
    forced_input = drill.get("forced_input") or {}
    if not isinstance(forced_input, dict) or not forced_input:
        raise SystemExit(
            f"{row.measurement_row_id}: failure_drill.forced_input "
            "must declare a directive"
        )

    expected_completion_class = row.expected_completion_class
    expected_failure_category_subset = list(
        row.expected_failure_category_subset
    )
    expected_primary_events = list(row.expected_primary_events)
    privacy_class = row.privacy_class
    telemetry_default_posture = row.telemetry_default_posture

    for directive, payload in forced_input.items():
        if directive == "rewrite_expected_completion_class":
            if isinstance(payload, str):
                expected_completion_class = payload
        elif directive == "inject_failure_category":
            if isinstance(payload, str):
                expected_failure_category_subset.append(payload)
        elif directive == "inject_primary_event":
            if isinstance(payload, str):
                expected_primary_events.append(payload)
        elif directive == "rewrite_privacy_class":
            if isinstance(payload, str):
                privacy_class = payload
        elif directive == "rewrite_telemetry_default_posture":
            if isinstance(payload, str):
                telemetry_default_posture = payload
        else:
            raise SystemExit(
                f"{row.measurement_row_id}: unknown failure_drill "
                f"directive: {directive}"
            )

    return EvaluationInputs(
        row=row,
        expected_completion_class=expected_completion_class,
        expected_failure_category_subset=(
            expected_failure_category_subset
        ),
        expected_primary_events=expected_primary_events,
        privacy_class=privacy_class,
        telemetry_default_posture=telemetry_default_posture,
    )


def evaluate_row(
    inputs: EvaluationInputs,
    corpus_rows: dict[str, CorpusRowSummary],
    repo_root: Path,
) -> tuple[list[Finding], dict[str, Any]]:
    row = inputs.row
    findings: list[Finding] = []

    # 1) Closed-vocab checks on the row's own fields.
    if row.entry_route_id not in ENTRY_ROUTE_VOCAB:
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_ENTRY_ROUTE_UNKNOWN,
                message=(
                    f"row '{row.measurement_row_id}' uses entry_route_id "
                    f"'{row.entry_route_id}' which is not in the frozen "
                    "vocabulary"
                ),
                remediation=(
                    "Quote one of the er.* values from "
                    "docs/product/onboarding_measurement_plan.md §4."
                ),
                row_id=row.measurement_row_id,
                onboarding_path_class=row.onboarding_path_class,
                ref=row.entry_route_id,
            )
        )

    if row.measurement_surface not in SURFACE_FAILURE_CATEGORIES:
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_MEASUREMENT_SURFACE_UNKNOWN,
                message=(
                    f"row '{row.measurement_row_id}' uses "
                    f"measurement_surface '{row.measurement_surface}' "
                    "which is not in the frozen §3 vocab"
                ),
                remediation=(
                    "Quote one of the surface_* values from "
                    "docs/product/onboarding_measurement_plan.md §3."
                ),
                row_id=row.measurement_row_id,
                onboarding_path_class=row.onboarding_path_class,
                ref=row.measurement_surface,
            )
        )

    if inputs.expected_completion_class not in COMPLETION_CLASS_VOCAB:
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_COMPLETION_CLASS_UNKNOWN,
                message=(
                    f"row '{row.measurement_row_id}' uses "
                    f"expected_completion_class "
                    f"'{inputs.expected_completion_class}' which is "
                    "not in the schema's closed enum"
                ),
                remediation=(
                    "Restore expected_completion_class to one of the "
                    "closed values in "
                    "schemas/telemetry/m1_onboarding_metrics.schema.json."
                ),
                row_id=row.measurement_row_id,
                onboarding_path_class=row.onboarding_path_class,
                ref=inputs.expected_completion_class,
            )
        )

    if row.completion_checkpoint_class not in COMPLETION_CHECKPOINT_VOCAB:
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_CHECKPOINT_UNKNOWN,
                message=(
                    f"row '{row.measurement_row_id}' uses "
                    f"completion_checkpoint_class "
                    f"'{row.completion_checkpoint_class}' which is "
                    "not in the schema's closed enum"
                ),
                remediation=(
                    "Restore completion_checkpoint_class to one of the "
                    "closed values in "
                    "schemas/telemetry/m1_onboarding_metrics.schema.json."
                ),
                row_id=row.measurement_row_id,
                onboarding_path_class=row.onboarding_path_class,
                ref=row.completion_checkpoint_class,
            )
        )

    if inputs.privacy_class not in PRIVACY_CLASS_VOCAB:
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_PRIVACY_CLASS_UNKNOWN,
                message=(
                    f"row '{row.measurement_row_id}' uses privacy_class "
                    f"'{inputs.privacy_class}' which is not in the "
                    "schema's closed enum"
                ),
                remediation=(
                    "Restore privacy_class to one of the closed "
                    "values; default-on / always-send postures are "
                    "non-conforming for the M1 measurement lane."
                ),
                row_id=row.measurement_row_id,
                onboarding_path_class=row.onboarding_path_class,
                ref=inputs.privacy_class,
            )
        )

    if (
        inputs.telemetry_default_posture
        not in TELEMETRY_POSTURE_VOCAB
    ):
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_TELEMETRY_POSTURE_UNKNOWN,
                message=(
                    f"row '{row.measurement_row_id}' uses "
                    f"telemetry_default_posture "
                    f"'{inputs.telemetry_default_posture}' which is "
                    "not in the schema's closed enum"
                ),
                remediation=(
                    "Restore telemetry_default_posture to opt_in_only "
                    "(or off_by_default_no_emission_until_consent)."
                ),
                row_id=row.measurement_row_id,
                onboarding_path_class=row.onboarding_path_class,
                ref=inputs.telemetry_default_posture,
            )
        )

    # 2) Subset checks against the per-surface vocab.
    surface_categories = SURFACE_FAILURE_CATEGORIES.get(
        row.measurement_surface, set()
    )
    outside_categories = [
        cat
        for cat in inputs.expected_failure_category_subset
        if cat not in surface_categories
    ]
    for cat in outside_categories:
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_FAILURE_CATEGORY_OUTSIDE_VOCAB,
                message=(
                    f"row '{row.measurement_row_id}' lists failure "
                    f"category '{cat}' which is not in the closed "
                    f"vocab for {row.measurement_surface} "
                    "(docs/product/onboarding_measurement_plan.md §3)"
                ),
                remediation=(
                    "Drop the free-form category, or — if the "
                    "regression class is real — promote it through "
                    "the measurement plan in the same change."
                ),
                row_id=row.measurement_row_id,
                onboarding_path_class=row.onboarding_path_class,
                ref=cat,
            )
        )

    surface_events = SURFACE_PRIMARY_EVENTS.get(
        row.measurement_surface, set()
    )
    outside_events = [
        ev
        for ev in inputs.expected_primary_events
        if ev not in surface_events
    ]
    for ev in outside_events:
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_PRIMARY_EVENT_OUTSIDE_VOCAB,
                message=(
                    f"row '{row.measurement_row_id}' lists primary "
                    f"event '{ev}' which is not in the closed vocab "
                    f"for {row.measurement_surface} "
                    "(docs/product/onboarding_measurement_plan.md §3)"
                ),
                remediation=(
                    "Drop the free-form event name, or promote it "
                    "through the measurement plan in the same change."
                ),
                row_id=row.measurement_row_id,
                onboarding_path_class=row.onboarding_path_class,
                ref=ev,
            )
        )

    # 3) First-useful-work-vs-app-launch guard.
    #
    # If the row declares completion_checkpoint_class =
    # first_useful_edit, the expected_completion_class MUST also
    # be a first-useful-edit-class outcome
    # (completed_first_useful_edit). Any path that asserts a
    # first-useful-edit checkpoint while classifying the outcome
    # as navigation-only is silently equating mere app launch with
    # first useful work.
    if (
        row.completion_checkpoint_class == "first_useful_edit"
        and inputs.expected_completion_class
        == "completed_first_useful_navigation_only"
    ):
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_CHECKPOINT_COLLAPSED,
                message=(
                    f"row '{row.measurement_row_id}' claims "
                    "completion_checkpoint_class = first_useful_edit "
                    "but expected_completion_class = "
                    "completed_first_useful_navigation_only; the row "
                    "is silently classifying mere app launch as "
                    "first-useful-work success"
                ),
                remediation=(
                    "Either deliver a first-useful-edit checkpoint on "
                    "this path (editor + buffer + save + recovery "
                    "journal reachable before semantic warm-up) or "
                    "downgrade completion_checkpoint_class to "
                    "first_useful_navigation. Do not silently classify "
                    "navigation-only outcomes as first-useful-edit "
                    "success."
                ),
                row_id=row.measurement_row_id,
                onboarding_path_class=row.onboarding_path_class,
            )
        )

    # 4) Corpus / fixture cross-references.
    corpus_row = corpus_rows.get(row.corpus_row_ref)
    if corpus_row is None:
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_CORPUS_ROW_MISSING,
                message=(
                    f"row '{row.measurement_row_id}' references "
                    f"corpus_row_ref '{row.corpus_row_ref}' which is "
                    "not present in artifacts/ux/first_useful_work_corpus/"
                ),
                remediation=(
                    "Restore the corpus row or pick an existing fuw_row:* "
                    "id from the qualification corpus."
                ),
                row_id=row.measurement_row_id,
                onboarding_path_class=row.onboarding_path_class,
                ref=row.corpus_row_ref,
            )
        )
    else:
        if corpus_row.entry_route_id != row.entry_route_id:
            findings.append(
                Finding(
                    severity="error",
                    check_id=CHECK_ID_CORPUS_ROW_MISMATCH,
                    message=(
                        f"row '{row.measurement_row_id}' declares "
                        f"entry_route_id '{row.entry_route_id}' but "
                        f"corpus row '{corpus_row.row_id}' declares "
                        f"'{corpus_row.entry_route_id}'"
                    ),
                    remediation=(
                        "Align the measurement row with the corpus "
                        "row, or pick a corpus row whose entry_route_id "
                        "matches this onboarding path."
                    ),
                    row_id=row.measurement_row_id,
                    onboarding_path_class=row.onboarding_path_class,
                    ref=row.corpus_row_ref,
                )
            )
        if corpus_row.measurement_surface != row.measurement_surface:
            findings.append(
                Finding(
                    severity="error",
                    check_id=CHECK_ID_CORPUS_ROW_MISMATCH,
                    message=(
                        f"row '{row.measurement_row_id}' declares "
                        f"measurement_surface "
                        f"'{row.measurement_surface}' but corpus row "
                        f"'{corpus_row.row_id}' declares "
                        f"'{corpus_row.measurement_surface}'"
                    ),
                    remediation=(
                        "Align the measurement row with the corpus "
                        "row's measurement_surface."
                    ),
                    row_id=row.measurement_row_id,
                    onboarding_path_class=row.onboarding_path_class,
                    ref=row.corpus_row_ref,
                )
            )
        if (
            corpus_row.first_useful_work_target_surface
            != row.expected_first_useful_work_target_surface
        ):
            findings.append(
                Finding(
                    severity="error",
                    check_id=CHECK_ID_TARGET_SURFACE_MISMATCH,
                    message=(
                        f"row '{row.measurement_row_id}' declares "
                        "expected_first_useful_work_target_surface "
                        f"'{row.expected_first_useful_work_target_surface}'"
                        f" but corpus row '{corpus_row.row_id}' "
                        "declares "
                        f"'{corpus_row.first_useful_work_target_surface}'"
                    ),
                    remediation=(
                        "Either align the target surface with the "
                        "corpus row or pick a different corpus row."
                    ),
                    row_id=row.measurement_row_id,
                    onboarding_path_class=row.onboarding_path_class,
                    ref=row.corpus_row_ref,
                )
            )
        unbound_metrics = [
            metric
            for metric in row.expected_protected_metric_refs
            if metric not in corpus_row.protected_metric_refs
        ]
        for metric in unbound_metrics:
            findings.append(
                Finding(
                    severity="error",
                    check_id=CHECK_ID_PROTECTED_METRIC_UNBOUND,
                    message=(
                        f"row '{row.measurement_row_id}' lists "
                        f"protected metric '{metric}' which is not in "
                        f"the corpus row '{corpus_row.row_id}' "
                        "protected_metric_refs"
                    ),
                    remediation=(
                        "Add the metric to the corpus row's "
                        "protected_metric_refs in the same change, or "
                        "drop it from the measurement row."
                    ),
                    row_id=row.measurement_row_id,
                    onboarding_path_class=row.onboarding_path_class,
                    ref=metric,
                )
            )

    fixture_path = repo_root / row.supporting_fixture_ref
    fixture_summary: WorkedFixtureSummary | None = None
    if not fixture_path.exists():
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_FIXTURE_MISSING,
                message=(
                    f"row '{row.measurement_row_id}' references "
                    f"supporting_fixture_ref '{row.supporting_fixture_ref}' "
                    "which is not on disk"
                ),
                remediation=(
                    "Restore the worked fixture under "
                    "fixtures/ux/first_useful_work_cases/ or update the "
                    "measurement row to the new path."
                ),
                row_id=row.measurement_row_id,
                onboarding_path_class=row.onboarding_path_class,
                ref=row.supporting_fixture_ref,
            )
        )
    else:
        fixture_summary = load_worked_fixture(fixture_path)
        if fixture_summary.entry_route_id != row.entry_route_id:
            findings.append(
                Finding(
                    severity="error",
                    check_id=CHECK_ID_FIXTURE_MISMATCH,
                    message=(
                        f"row '{row.measurement_row_id}' declares "
                        f"entry_route_id '{row.entry_route_id}' but "
                        f"fixture '{row.supporting_fixture_ref}' "
                        f"declares '{fixture_summary.entry_route_id}'"
                    ),
                    remediation=(
                        "Align the measurement row with the worked "
                        "fixture's entry_route_id."
                    ),
                    row_id=row.measurement_row_id,
                    onboarding_path_class=row.onboarding_path_class,
                    ref=row.supporting_fixture_ref,
                )
            )
        if (
            fixture_summary.measurement_surface
            != row.measurement_surface
        ):
            findings.append(
                Finding(
                    severity="error",
                    check_id=CHECK_ID_FIXTURE_MISMATCH,
                    message=(
                        f"row '{row.measurement_row_id}' declares "
                        f"measurement_surface "
                        f"'{row.measurement_surface}' but fixture "
                        f"'{row.supporting_fixture_ref}' declares "
                        f"'{fixture_summary.measurement_surface}'"
                    ),
                    remediation=(
                        "Align the measurement row with the worked "
                        "fixture's measurement_surface."
                    ),
                    row_id=row.measurement_row_id,
                    onboarding_path_class=row.onboarding_path_class,
                    ref=row.supporting_fixture_ref,
                )
            )
        if (
            fixture_summary.first_useful_work_target_surface
            != row.expected_first_useful_work_target_surface
        ):
            findings.append(
                Finding(
                    severity="error",
                    check_id=CHECK_ID_TARGET_SURFACE_MISMATCH,
                    message=(
                        f"row '{row.measurement_row_id}' declares "
                        "expected_first_useful_work_target_surface "
                        f"'{row.expected_first_useful_work_target_surface}'"
                        f" but fixture "
                        f"'{row.supporting_fixture_ref}' declares "
                        f"'{fixture_summary.first_useful_work_target_surface}'"
                    ),
                    remediation=(
                        "Align the target surface with the worked "
                        "fixture."
                    ),
                    row_id=row.measurement_row_id,
                    onboarding_path_class=row.onboarding_path_class,
                    ref=row.supporting_fixture_ref,
                )
            )

    diagnostics: dict[str, Any] = {
        "measurement_row_id": row.measurement_row_id,
        "onboarding_path_class": row.onboarding_path_class,
        "entry_route_id": row.entry_route_id,
        "measurement_surface": row.measurement_surface,
        "completion_checkpoint_class": row.completion_checkpoint_class,
        "expected_completion_class": (
            inputs.expected_completion_class
        ),
        "expected_first_useful_work_target_surface": (
            row.expected_first_useful_work_target_surface
        ),
        "expected_failure_category_subset": (
            list(inputs.expected_failure_category_subset)
        ),
        "expected_primary_events": list(
            inputs.expected_primary_events
        ),
        "expected_protected_metric_refs": list(
            row.expected_protected_metric_refs
        ),
        "telemetry_default_posture": (
            inputs.telemetry_default_posture
        ),
        "privacy_class": inputs.privacy_class,
        "corpus_row_resolved": corpus_row is not None,
        "fixture_resolved": fixture_summary is not None,
    }
    if corpus_row is not None:
        diagnostics["corpus_row_summary"] = {
            "entry_route_id": corpus_row.entry_route_id,
            "measurement_surface": corpus_row.measurement_surface,
            "first_useful_work_target_surface": (
                corpus_row.first_useful_work_target_surface
            ),
            "protected_metric_refs": list(
                corpus_row.protected_metric_refs
            ),
        }
    if fixture_summary is not None:
        diagnostics["fixture_summary"] = {
            "entry_route_id": fixture_summary.entry_route_id,
            "measurement_surface": (
                fixture_summary.measurement_surface
            ),
            "first_useful_work_target_surface": (
                fixture_summary.first_useful_work_target_surface
            ),
            "protected_metric_refs": list(
                fixture_summary.protected_metric_refs
            ),
            "forbidden_failure_categories": list(
                fixture_summary.forbidden_failure_categories
            ),
        }
    return findings, diagnostics


def expected_findings_match(
    findings: list[Finding], expected_check_id: str
) -> tuple[bool, str | None]:
    for finding in findings:
        if finding.check_id == expected_check_id:
            return True, finding.check_id
    return False, None


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(
            f"--repo-root does not look like a repository root: {repo_root}"
        )

    schema_path = repo_root / args.schema
    if not schema_path.exists():
        raise SystemExit(
            f"missing schema: {args.schema}; reserved closed vocab "
            "must live on disk"
        )

    rows_dir = repo_root / args.rows_dir
    if not rows_dir.is_dir():
        raise SystemExit(f"rows dir not found: {args.rows_dir}")
    row_paths = sorted(p for p in rows_dir.glob("*.yaml"))
    if not row_paths:
        raise SystemExit(
            f"no measurement rows found in {args.rows_dir}"
        )

    corpus_dir = repo_root / args.corpus_dir
    if not corpus_dir.is_dir():
        raise SystemExit(
            f"corpus dir not found: {args.corpus_dir}"
        )

    rows = [load_measurement_row(p, repo_root) for p in row_paths]

    seen_ids: set[str] = set()
    coverage_findings: list[Finding] = []
    for row in rows:
        if row.measurement_row_id in seen_ids:
            coverage_findings.append(
                Finding(
                    severity="error",
                    check_id=CHECK_ID_DUPLICATE_ROW,
                    message=(
                        f"duplicate measurement_row_id: "
                        f"{row.measurement_row_id}"
                    ),
                    remediation=(
                        "Make every row id unique; row ids reach "
                        "release evidence and dashboards."
                    ),
                    row_id=row.measurement_row_id,
                    onboarding_path_class=row.onboarding_path_class,
                )
            )
        seen_ids.add(row.measurement_row_id)

    observed_paths = {row.onboarding_path_class for row in rows}
    missing_paths = REQUIRED_ONBOARDING_PATH_COVERAGE - observed_paths
    if missing_paths:
        coverage_findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_PATH_COVERAGE_MISSING,
                message=(
                    "onboarding path coverage missing; required: "
                    f"{sorted(REQUIRED_ONBOARDING_PATH_COVERAGE)}, "
                    f"missing: {sorted(missing_paths)}"
                ),
                remediation=(
                    "Add at least one measurement row for each "
                    "missing onboarding_path_class so the lane covers "
                    "Open folder, Open workspace, Clone repository, "
                    "Restore last session, and Import from… distinctly."
                ),
            )
        )

    corpus_rows = load_corpus_rows(corpus_dir)

    drill_mode = args.force_drill is not None
    drill_row: MeasurementRow | None = None
    if drill_mode:
        for row in rows:
            if row.failure_drill.get("drill_id") == args.force_drill:
                drill_row = row
                break
        if drill_row is None:
            raise SystemExit(
                f"--force-drill: no measurement row declares drill_id "
                f"'{args.force_drill}'"
            )

    row_results: list[dict[str, Any]] = []
    all_findings: list[Finding] = list(coverage_findings)

    for row in rows:
        is_drill_row = (
            drill_mode
            and drill_row is not None
            and row.measurement_row_id == drill_row.measurement_row_id
        )
        if is_drill_row:
            inputs = apply_drill(row)
        else:
            inputs = EvaluationInputs(
                row=row,
                expected_completion_class=row.expected_completion_class,
                expected_failure_category_subset=list(
                    row.expected_failure_category_subset
                ),
                expected_primary_events=list(
                    row.expected_primary_events
                ),
                privacy_class=row.privacy_class,
                telemetry_default_posture=(
                    row.telemetry_default_posture
                ),
            )

        findings, diagnostics = evaluate_row(
            inputs, corpus_rows, repo_root
        )

        row_record: dict[str, Any] = {
            "measurement_row_id": row.measurement_row_id,
            "row_path": row.row_path,
            "onboarding_path_class": row.onboarding_path_class,
            "diagnostics": diagnostics,
            "finding_count": len(findings),
            "findings": [f.as_report() for f in findings],
        }

        if is_drill_row:
            expected = row.failure_drill.get("expected_check_id")
            assert isinstance(expected, str)
            ok, matched = expected_findings_match(findings, expected)
            row_record["failure_drill"] = {
                "drill_id": row.failure_drill.get("drill_id"),
                "expected_check_id": expected,
                "expected_finding_observed": ok,
                "matched_check_id": matched,
            }
            if not ok:
                all_findings.append(
                    Finding(
                        severity="error",
                        check_id=CHECK_ID_FAILURE_DRILL_MISSING_EXPECTED,
                        message=(
                            f"failure drill "
                            f"{row.failure_drill.get('drill_id')!r} on "
                            f"row '{row.measurement_row_id}' did not "
                            f"reproduce expected check_id "
                            f"{expected!r}"
                        ),
                        remediation=(
                            "Either restore the audit logic to detect "
                            "this regression class, or update the row "
                            "if the regression class genuinely changed."
                        ),
                        row_id=row.measurement_row_id,
                        onboarding_path_class=(
                            row.onboarding_path_class
                        ),
                        ref=row.failure_drill.get("drill_id"),
                    )
                )
        else:
            all_findings.extend(findings)

        row_results.append(row_record)

    errors = [f for f in all_findings if f.severity == "error"]
    if drill_mode:
        non_drill_failures = [
            f
            for f in errors
            if f.check_id
            == CHECK_ID_FAILURE_DRILL_MISSING_EXPECTED
        ]
        status = "PASS" if not non_drill_failures else "FAIL"
    else:
        status = "PASS" if not errors else "FAIL"

    capture: dict[str, Any] = {
        "schema_version": 1,
        "capture_kind": "onboarding_task_success_validation_capture",
        "captured_at": now_iso_z(),
        "owner_dri": "@ahmeddyounis",
        "rows_dir_ref": args.rows_dir,
        "corpus_dir_ref": args.corpus_dir,
        "schema_ref": args.schema,
        "row_count": len(rows),
        "drill_mode": drill_mode,
        "drill_id": args.force_drill,
        "exact_build_identity_ref": args.build_identity,
        "command": (
            "python3 tests/ux/onboarding_task_success/"
            "run_onboarding_task_success_lane.py --repo-root ."
            + (
                f" --force-drill {args.force_drill}"
                if drill_mode
                else ""
            )
        ),
        "required_onboarding_path_coverage": sorted(
            REQUIRED_ONBOARDING_PATH_COVERAGE
        ),
        "observed_onboarding_path_classes": sorted(observed_paths),
        "status": status,
        "rows": row_results,
        "finding_counts": {
            "error": sum(
                1 for f in all_findings if f.severity == "error"
            ),
            "warning": sum(
                1 for f in all_findings if f.severity == "warning"
            ),
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
        f"[onboarding-task-success] {status} ({len(rows)} rows, "
        f"{len(errors)} errors, "
        f"{sum(1 for f in all_findings if f.severity == 'warning')} "
        f"warnings) — capture: {args.report}"
    )
    for finding in all_findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        row_suffix = (
            f" {{{finding.row_id}}}" if finding.row_id else ""
        )
        print(
            f"[onboarding-task-success] {prefix}{row_suffix} "
            f"{finding.check_id}: {finding.message}{ref_suffix}"
        )
        print(
            f"[onboarding-task-success]   remediation: "
            f"{finding.remediation}"
        )

    return 0 if status == "PASS" else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print(
            "[onboarding-task-success] interrupted",
            file=sys.stderr,
        )
        sys.exit(130)
