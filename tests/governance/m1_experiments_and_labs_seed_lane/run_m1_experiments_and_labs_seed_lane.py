#!/usr/bin/env python3
"""Unattended M1 experiments / flags / Labs registry seed validation lane.

Replays every row in ``artifacts/governance/experiments_registry.yaml``
against:

- ``schemas/governance/experiment_registry.schema.json`` — the seed
  envelope schema (vocabularies, required coverage, named consumers);
- ``schemas/governance/experiment_registry_entry.schema.json`` — the
  row vocabulary; and
- the canonical landing page at
  ``docs/governance/m1_experiments_and_labs_seed.md`` so the seed
  cannot quietly point at a missing reviewer entry.

Per-row assertions:

- ``record_kind`` is ``experiment_registry_entry_record`` and
  ``experiment_registry_entry_schema_version`` is ``1``;
- ``register_entry_id`` is unique, non-empty, matches the row schema's
  pattern (``^ereg\\.``), and is not duplicated across rows;
- ``source_register`` is ``experiments_register`` and ``source_id``
  matches its row-kind prefix (``exp.`` / ``mode.`` / ``rollout.`` /
  ``flag.``) AND resolves on disk in
  ``artifacts/governance/experiments_register.yaml``;
- ``row_kind_class``, ``audience_class``, ``lifecycle_state_class``,
  ``cohort_class``, ``default_posture_class``, ``public_label_class``,
  ``telemetry_posture_class``, ``graduation_retirement_path_class``,
  and ``kill_switch.source_class`` are members of their closed
  vocabularies in the row schema;
- ``owner_dri`` is a non-empty ``@handle`` so an ownerless experiment
  trips the registry's named drill;
- ``review_by`` and ``expires_on`` are either null or ISO-8601 dates,
  AND at least one of them is non-null. Whichever is non-null is
  compared against the envelope's ``as_of`` value and trips the
  ``row_review_by_in_past_breaches_expiry`` /
  ``row_expires_on_in_past_breaches_expiry`` check if it has drifted
  into the past;
- ``kill_switch.source_ref`` and ``rollback_path.rollback_ref`` are
  non-empty so a row cannot claim a kill switch / rollback path and
  point at nothing;
- ``audience_class = contributor_visible_labs`` rows MUST publish a
  non-empty ``labs_projection_ref`` of the form
  ``artifacts/governance/labs_register.yaml#<labs_id>`` whose
  ``labs_id`` resolves in the labs register on disk and whose
  ``experiment_ref`` agrees with the seed row's ``source_id``;
- ``audience_class`` in {``hidden_developer_toggle``,
  ``control_stack_reserved``, ``ci_rollout_only``} rows MUST NOT
  publish a ``labs_projection_ref``;
- ``public_label_class`` MUST match ``audience_class`` per the row
  schema's allOf invariants;
- ``row_kind_class = benchmark_mode`` rows MUST publish
  ``default_shift_change_log_posture`` in
  {``docs_update_required``, ``release_note_required``};
- ``row_kind_class = rollout`` rows MUST publish a non-empty
  ``rollout_guardrails`` list;
- ``default_posture_class = reserved_until_runtime_control_stack_lands``
  rows MUST publish ``graduation_retirement_path_class =
  hold_pending_runtime_control_stack`` AND ``telemetry_posture_class
  = not_applicable_reserved_binding``;
- ``failure_drill`` is a non-null object with a closed ``drill_id``
  drawn from the envelope's ``failure_drill_id_vocabulary``, a
  non-empty ``forced_input``, ``expected_check_id``, and
  ``actionable_next_action``.

Envelope assertions:

- ``schema_version = 1``, ``matrix_id = m1_experiments_and_labs_seed``,
  ``status`` non-empty, ``as_of`` is an ISO date,
  ``owner_dri`` is a ``@handle``;
- ``overview_page``, ``row_schema_ref``, ``build_identity_ref``,
  ``validation_lane_ref`` resolve on disk;
- closed envelope vocabularies AGREE with the row schema ``$defs``
  of the same names;
- ``required_row_kind_class_coverage``,
  ``required_audience_class_coverage``, and
  ``required_lifecycle_state_class_coverage`` are each satisfied;
- every ``named_runtime_consumer.consumer_ref`` resolves on disk and
  ``consumed_fields`` is non-empty.

``--force-drill <register_entry_id>:<drill_id>`` replays the named
drill on the named row and exits 0 only when the runner reproduces
the declared ``expected_check_id``. Drift in the unforced rows still
fails the lane.

YAML decoding follows the repository convention: matrix and fixture
files are parsed via Ruby/Psych so this script does not require a
third-party Python YAML dependency.
"""

from __future__ import annotations

import argparse
import copy
import datetime as dt
import json
import re
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_MATRIX_REL = "artifacts/governance/experiments_registry.yaml"
DEFAULT_ENVELOPE_SCHEMA_REL = (
    "schemas/governance/experiment_registry.schema.json"
)
DEFAULT_ROW_SCHEMA_REL = (
    "schemas/governance/experiment_registry_entry.schema.json"
)
DEFAULT_BUILD_IDENTITY_REL = "artifacts/build/build_identity.json"
DEFAULT_REPORT_REL = (
    "artifacts/milestones/m1/captures/"
    "experiments_and_labs_seed_validation_capture.json"
)

EXPECTED_RECORD_KIND = "experiment_registry_entry_record"
EXPECTED_ROW_SCHEMA_VERSION = 1
EXPECTED_MATRIX_ID = "m1_experiments_and_labs_seed"

REGISTER_ENTRY_ID_PATTERN = re.compile(r"^ereg\.[a-z0-9]+(?:[._-][a-z0-9]+)*$")
OWNER_DRI_PATTERN = re.compile(r"^@[a-zA-Z0-9_-]+$")
SOURCE_ID_PATTERN = re.compile(
    r"^(exp|mode|rollout|flag)\.[a-z0-9]+(?:[._-][a-z0-9]+)*$"
)
ISO_DATE_PATTERN = re.compile(r"^\d{4}-\d{2}-\d{2}$")

ROW_KIND_TO_SOURCE_PREFIX = {
    "experiment": "exp.",
    "benchmark_mode": "mode.",
    "rollout": "rollout.",
    "feature_flag": "flag.",
}

AUDIENCE_TO_PUBLIC_LABEL_CLASS = {
    "contributor_visible_labs": "public_labs_visible_label",
    "hidden_developer_toggle": "hidden_developer_toggle_label",
    "control_stack_reserved": "control_stack_reserved_label",
    "ci_rollout_only": "ci_rollout_only_label",
}


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


@dataclass
class RowResult:
    register_entry_id: str
    row_kind_class: str
    audience_class: str
    passed_checks: list[str] = field(default_factory=list)
    failed_checks: list[dict[str, str]] = field(default_factory=list)
    diagnostics: dict[str, Any] = field(default_factory=dict)


def fail(result: RowResult, check_id: str, message: str) -> None:
    result.failed_checks.append({"check_id": check_id, "message": message})


def pass_(result: RowResult, message: str) -> None:
    result.passed_checks.append(message)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--repo-root",
        default=".",
        help="Repository root (must contain a .git directory).",
    )
    parser.add_argument(
        "--matrix",
        default=DEFAULT_MATRIX_REL,
        help="Seed YAML path, repo-relative.",
    )
    parser.add_argument(
        "--envelope-schema",
        default=DEFAULT_ENVELOPE_SCHEMA_REL,
        help="Envelope schema JSON path, repo-relative.",
    )
    parser.add_argument(
        "--row-schema",
        default=DEFAULT_ROW_SCHEMA_REL,
        help="Row schema JSON path, repo-relative.",
    )
    parser.add_argument(
        "--build-identity",
        default=DEFAULT_BUILD_IDENTITY_REL,
        help="Build-identity record path the capture embeds.",
    )
    parser.add_argument(
        "--report",
        default=DEFAULT_REPORT_REL,
        help="Where to write the durable JSON capture.",
    )
    parser.add_argument(
        "--force-drill",
        default=None,
        help=(
            "Replay a named failure drill on a named row in the form "
            "'<register_entry_id>:<drill_id>'. The runner exits 0 only "
            "when the row's failure drill reproduces the exact "
            "expected_check_id."
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
            "-rdate",
            "-rtime",
            "-e",
            (
                "payload = YAML.safe_load(File.read(ARGV[0]), "
                "permitted_classes: [Time, Date, DateTime], aliases: false); "
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


def artifact_ref_exists(repo_root: Path, ref: str) -> bool:
    ref = ref.strip()
    path = ref.split("#", 1)[0].strip()
    if not path:
        return False
    return (repo_root / path).exists()


def now_iso_z() -> str:
    return (
        dt.datetime.now(dt.timezone.utc)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )


def load_schema_enum(repo_root: Path, ref: str, defs_key: str) -> list[str]:
    schema_path = repo_root / ref
    if not schema_path.exists():
        return []
    schema = json.loads(schema_path.read_text(encoding="utf-8"))
    defs = schema.get("$defs", {})
    entry = defs.get(defs_key, {})
    if "enum" in entry and isinstance(entry["enum"], list):
        return [str(v) for v in entry["enum"]]
    return []


def apply_forced_overrides(
    row: dict[str, Any], forced_overrides: dict[str, Any]
) -> dict[str, Any]:
    row = copy.deepcopy(row)
    if not forced_overrides:
        return row

    if forced_overrides.get("clear_owner_dri"):
        row["owner_dri"] = ""

    if "rewrite_owner_dri" in forced_overrides:
        row["owner_dri"] = forced_overrides["rewrite_owner_dri"]

    if forced_overrides.get("clear_review_by"):
        row["review_by"] = None

    if forced_overrides.get("clear_expires_on"):
        row["expires_on"] = None

    if "rewrite_review_by" in forced_overrides:
        row["review_by"] = forced_overrides["rewrite_review_by"]

    if "rewrite_expires_on" in forced_overrides:
        row["expires_on"] = forced_overrides["rewrite_expires_on"]

    if forced_overrides.get("clear_labs_projection_ref"):
        row.pop("labs_projection_ref", None)

    if forced_overrides.get("clear_graduation_retirement_path_class"):
        row["graduation_retirement_path_class"] = ""

    if "rewrite_graduation_retirement_path_class" in forced_overrides:
        row["graduation_retirement_path_class"] = forced_overrides[
            "rewrite_graduation_retirement_path_class"
        ]

    if forced_overrides.get("clear_kill_switch_source_ref"):
        ks = row.get("kill_switch")
        if isinstance(ks, dict):
            ks = dict(ks)
            ks["source_ref"] = ""
            row["kill_switch"] = ks

    if "rewrite_default_shift_change_log_posture" in forced_overrides:
        row["default_shift_change_log_posture"] = forced_overrides[
            "rewrite_default_shift_change_log_posture"
        ]

    if forced_overrides.get("clear_rollout_guardrails"):
        row["rollout_guardrails"] = []

    if "rewrite_audience_class" in forced_overrides:
        row["audience_class"] = forced_overrides["rewrite_audience_class"]

    if forced_overrides.get("clear_rollback_path_rollback_ref"):
        rp = row.get("rollback_path")
        if isinstance(rp, dict):
            rp = dict(rp)
            rp["rollback_ref"] = ""
            row["rollback_path"] = rp

    return row


def parse_iso_date(value: Any) -> dt.date | None:
    if not isinstance(value, str):
        return None
    if not ISO_DATE_PATTERN.match(value):
        return None
    try:
        return dt.date.fromisoformat(value)
    except ValueError:
        return None


def validate_row(
    row: dict[str, Any],
    *,
    capability_label: str,
    as_of: dt.date,
    row_kind_class_vocab: set[str],
    audience_class_vocab: set[str],
    lifecycle_state_class_vocab: set[str],
    cohort_class_vocab: set[str],
    default_posture_class_vocab: set[str],
    public_label_class_vocab: set[str],
    telemetry_posture_class_vocab: set[str],
    graduation_retirement_path_class_vocab: set[str],
    kill_switch_source_class_vocab: set[str],
    failure_drill_id_vocab: set[str],
) -> RowResult:
    entry_id = ensure_str(
        row.get("register_entry_id"),
        f"{capability_label}.register_entry_id",
    )
    row_kind_class = (
        row.get("row_kind_class")
        if isinstance(row.get("row_kind_class"), str)
        else ""
    )
    audience_class = (
        row.get("audience_class")
        if isinstance(row.get("audience_class"), str)
        else ""
    )

    result = RowResult(
        register_entry_id=entry_id,
        row_kind_class=row_kind_class,
        audience_class=audience_class,
    )

    # --- discriminator and version pins ---------------------------------
    if row.get("record_kind") != EXPECTED_RECORD_KIND:
        fail(
            result,
            "experiment_registry.record_kind_wrong",
            (
                f"record_kind must be {EXPECTED_RECORD_KIND!r}; got "
                f"{row.get('record_kind')!r}"
            ),
        )
    if (
        row.get("experiment_registry_entry_schema_version")
        != EXPECTED_ROW_SCHEMA_VERSION
    ):
        fail(
            result,
            "experiment_registry.schema_version_wrong",
            (
                "experiment_registry_entry_schema_version must be "
                f"{EXPECTED_ROW_SCHEMA_VERSION}; got "
                f"{row.get('experiment_registry_entry_schema_version')!r}"
            ),
        )

    # --- register_entry_id pattern --------------------------------------
    if not REGISTER_ENTRY_ID_PATTERN.match(entry_id):
        fail(
            result,
            "experiment_registry.register_entry_id_pattern_invalid",
            (
                f"register_entry_id {entry_id!r} does not match "
                f"{REGISTER_ENTRY_ID_PATTERN.pattern!r}"
            ),
        )

    # --- source_register / source_id ------------------------------------
    source_register = row.get("source_register")
    if source_register != "experiments_register":
        fail(
            result,
            "experiment_registry.source_register_class_unknown",
            (
                f"source_register {source_register!r} must be "
                "'experiments_register'"
            ),
        )

    source_id = row.get("source_id")
    if not isinstance(source_id, str) or not SOURCE_ID_PATTERN.match(source_id):
        fail(
            result,
            "experiment_registry.source_id_pattern_mismatch",
            (
                f"source_id {source_id!r} does not match "
                f"{SOURCE_ID_PATTERN.pattern!r}"
            ),
        )
    elif row_kind_class in ROW_KIND_TO_SOURCE_PREFIX:
        expected_prefix = ROW_KIND_TO_SOURCE_PREFIX[row_kind_class]
        if not source_id.startswith(expected_prefix):
            fail(
                result,
                "experiment_registry.source_id_prefix_must_match_row_kind",
                (
                    f"row_kind_class = {row_kind_class!r} forces source_id "
                    f"to start with {expected_prefix!r}; got {source_id!r}"
                ),
            )

    # --- public_label ---------------------------------------------------
    public_label = row.get("public_label")
    if not isinstance(public_label, str) or not public_label.strip():
        fail(
            result,
            "experiment_registry.public_label_required",
            "public_label must be a non-empty string",
        )

    # --- row_kind_class -------------------------------------------------
    if row_kind_class not in row_kind_class_vocab:
        fail(
            result,
            "experiment_registry.row_kind_class_unknown",
            (
                f"row_kind_class {row_kind_class!r} is not in the row "
                "schema's row_kind_class enum"
            ),
        )

    # --- audience_class -------------------------------------------------
    if audience_class not in audience_class_vocab:
        fail(
            result,
            "experiment_registry.audience_class_unknown",
            (
                f"audience_class {audience_class!r} is not in the row "
                "schema's audience_class enum"
            ),
        )

    # --- lifecycle_state_class -----------------------------------------
    lifecycle_state_class = row.get("lifecycle_state_class")
    if lifecycle_state_class not in lifecycle_state_class_vocab:
        fail(
            result,
            "experiment_registry.lifecycle_state_class_unknown",
            (
                f"lifecycle_state_class {lifecycle_state_class!r} is not "
                "in the row schema's lifecycle_state_class enum"
            ),
        )

    # --- cohort_class ---------------------------------------------------
    cohort_class = row.get("cohort_class")
    if cohort_class not in cohort_class_vocab:
        fail(
            result,
            "experiment_registry.cohort_class_unknown",
            (
                f"cohort_class {cohort_class!r} is not in the row "
                "schema's cohort_class enum"
            ),
        )

    # --- default_posture_class -----------------------------------------
    default_posture_class = row.get("default_posture_class")
    if default_posture_class not in default_posture_class_vocab:
        fail(
            result,
            "experiment_registry.default_posture_class_unknown",
            (
                f"default_posture_class {default_posture_class!r} is not "
                "in the row schema's default_posture_class enum"
            ),
        )

    # --- public_label_class --------------------------------------------
    public_label_class = row.get("public_label_class")
    if public_label_class not in public_label_class_vocab:
        fail(
            result,
            "experiment_registry.public_label_class_unknown",
            (
                f"public_label_class {public_label_class!r} is not in the "
                "row schema's public_label_class enum"
            ),
        )

    expected_label_class = AUDIENCE_TO_PUBLIC_LABEL_CLASS.get(audience_class)
    if (
        expected_label_class is not None
        and public_label_class != expected_label_class
    ):
        fail(
            result,
            "experiment_registry.public_label_class_must_match_audience_class",
            (
                f"audience_class = {audience_class!r} forces "
                f"public_label_class = {expected_label_class!r}; got "
                f"{public_label_class!r}"
            ),
        )

    # --- telemetry_posture_class ---------------------------------------
    telemetry_posture_class = row.get("telemetry_posture_class")
    if telemetry_posture_class not in telemetry_posture_class_vocab:
        fail(
            result,
            "experiment_registry.telemetry_posture_class_unknown",
            (
                f"telemetry_posture_class {telemetry_posture_class!r} is "
                "not in the row schema's telemetry_posture_class enum"
            ),
        )

    # --- graduation_retirement_path_class -------------------------------
    graduation_path = row.get("graduation_retirement_path_class")
    if not isinstance(graduation_path, str) or not graduation_path.strip():
        fail(
            result,
            "experiment_registry.graduation_retirement_path_class_required",
            (
                "graduation_retirement_path_class must be a non-empty "
                "string drawn from the closed vocabulary"
            ),
        )
    elif graduation_path not in graduation_retirement_path_class_vocab:
        fail(
            result,
            "experiment_registry.graduation_retirement_path_class_unknown",
            (
                f"graduation_retirement_path_class {graduation_path!r} is "
                "not in the row schema's graduation_retirement_path_class enum"
            ),
        )

    # --- reserved-default-posture invariants ---------------------------
    if (
        default_posture_class
        == "reserved_until_runtime_control_stack_lands"
    ):
        if graduation_path != "hold_pending_runtime_control_stack":
            fail(
                result,
                "experiment_registry.reserved_default_posture_forces_hold_pending_runtime_control_stack",
                (
                    "default_posture_class = "
                    "reserved_until_runtime_control_stack_lands forces "
                    "graduation_retirement_path_class = "
                    "hold_pending_runtime_control_stack; got "
                    f"{graduation_path!r}"
                ),
            )
        if telemetry_posture_class != "not_applicable_reserved_binding":
            fail(
                result,
                "experiment_registry.reserved_default_posture_forces_not_applicable_reserved_binding_telemetry",
                (
                    "default_posture_class = "
                    "reserved_until_runtime_control_stack_lands forces "
                    "telemetry_posture_class = "
                    "not_applicable_reserved_binding; got "
                    f"{telemetry_posture_class!r}"
                ),
            )

    # --- owner_dri ------------------------------------------------------
    owner_dri = row.get("owner_dri")
    if not isinstance(owner_dri, str) or not owner_dri.strip():
        fail(
            result,
            "experiment_registry.owner_dri_required",
            "owner_dri must be a non-empty @handle",
        )
    elif not OWNER_DRI_PATTERN.match(owner_dri):
        fail(
            result,
            "experiment_registry.owner_dri_pattern_invalid",
            (
                f"owner_dri {owner_dri!r} must match "
                f"{OWNER_DRI_PATTERN.pattern!r}"
            ),
        )

    # --- review_by / expires_on -----------------------------------------
    review_by_raw = row.get("review_by")
    expires_on_raw = row.get("expires_on")
    review_by_date = parse_iso_date(review_by_raw)
    expires_on_date = parse_iso_date(expires_on_raw)

    if review_by_raw is not None and review_by_date is None:
        fail(
            result,
            "experiment_registry.review_by_iso_date_invalid",
            (
                f"review_by {review_by_raw!r} must be null or an ISO-8601 "
                "date (YYYY-MM-DD)"
            ),
        )
    if expires_on_raw is not None and expires_on_date is None:
        fail(
            result,
            "experiment_registry.expires_on_iso_date_invalid",
            (
                f"expires_on {expires_on_raw!r} must be null or an "
                "ISO-8601 date (YYYY-MM-DD)"
            ),
        )

    if review_by_raw is None and expires_on_raw is None:
        fail(
            result,
            "experiment_registry.review_by_or_expires_on_required",
            (
                "at least one of review_by or expires_on MUST be a "
                "non-null ISO date so the row cannot live forever"
            ),
        )

    if review_by_date is not None and review_by_date < as_of:
        fail(
            result,
            "experiment_registry.row_review_by_in_past_breaches_expiry",
            (
                f"review_by {review_by_date.isoformat()} is earlier than "
                f"as_of {as_of.isoformat()}; renew the review window or "
                "retire the row"
            ),
        )

    if expires_on_date is not None and expires_on_date < as_of:
        fail(
            result,
            "experiment_registry.row_expires_on_in_past_breaches_expiry",
            (
                f"expires_on {expires_on_date.isoformat()} is earlier "
                f"than as_of {as_of.isoformat()}; renew the expiry or "
                "retire the row"
            ),
        )

    # --- kill_switch ----------------------------------------------------
    kill_switch = row.get("kill_switch")
    if not isinstance(kill_switch, dict):
        fail(
            result,
            "experiment_registry.kill_switch_required",
            "kill_switch must be an object",
        )
    else:
        ks_source = kill_switch.get("source_class")
        if ks_source not in kill_switch_source_class_vocab:
            fail(
                result,
                "experiment_registry.kill_switch_source_class_unknown",
                (
                    f"kill_switch.source_class {ks_source!r} is not in the "
                    "row schema's kill_switch_source_class enum"
                ),
            )
        ks_ref = kill_switch.get("source_ref")
        if not isinstance(ks_ref, str) or not ks_ref.strip():
            fail(
                result,
                "experiment_registry.kill_switch_source_ref_required",
                "kill_switch.source_ref must be a non-empty string",
            )

    # --- rollback_path --------------------------------------------------
    rollback_path = row.get("rollback_path")
    if not isinstance(rollback_path, dict):
        fail(
            result,
            "experiment_registry.rollback_path_required",
            "rollback_path must be an object",
        )
    else:
        rp_ref = rollback_path.get("rollback_ref")
        if not isinstance(rp_ref, str) or not rp_ref.strip():
            fail(
                result,
                "experiment_registry.rollback_path_rollback_ref_required",
                "rollback_path.rollback_ref must be a non-empty string",
            )

    # --- labs_projection_ref --------------------------------------------
    labs_ref = row.get("labs_projection_ref")
    if audience_class == "contributor_visible_labs":
        if not isinstance(labs_ref, str) or not labs_ref.strip():
            fail(
                result,
                "experiment_registry.labs_projection_required_for_contributor_visible",
                (
                    "audience_class = contributor_visible_labs forces a "
                    "non-empty labs_projection_ref pointing at the "
                    "labs_register row"
                ),
            )
    elif audience_class in {
        "hidden_developer_toggle",
        "control_stack_reserved",
        "ci_rollout_only",
    }:
        if labs_ref is not None:
            fail(
                result,
                "experiment_registry.labs_projection_forbidden_for_non_contributor_visible",
                (
                    f"audience_class = {audience_class!r} forbids a "
                    "labs_projection_ref; only contributor_visible_labs "
                    "rows may resolve in the Labs inventory"
                ),
            )

    # --- benchmark_mode / rollout invariants ----------------------------
    if row_kind_class == "benchmark_mode":
        change_log = row.get("default_shift_change_log_posture")
        if change_log not in {"docs_update_required", "release_note_required"}:
            fail(
                result,
                "experiment_registry.benchmark_mode_default_shift_change_log_posture_must_be_logged",
                (
                    "benchmark_mode rows MUST publish "
                    "default_shift_change_log_posture in "
                    "{docs_update_required, release_note_required}; got "
                    f"{change_log!r}"
                ),
            )

    if row_kind_class == "rollout":
        guardrails = row.get("rollout_guardrails")
        if not isinstance(guardrails, list) or not guardrails:
            fail(
                result,
                "experiment_registry.rollout_row_must_publish_rollout_guardrails",
                (
                    "rollout rows MUST publish a non-empty "
                    "rollout_guardrails list"
                ),
            )

    # --- evidence_refs --------------------------------------------------
    if not isinstance(row.get("evidence_refs"), list):
        fail(
            result,
            "experiment_registry.evidence_refs_must_be_list",
            "evidence_refs must be a list (may be empty)",
        )

    # --- failure_drill --------------------------------------------------
    drill = row.get("failure_drill")
    if not isinstance(drill, dict):
        fail(
            result,
            "experiment_registry.failure_drill_required",
            "failure_drill must be a non-null object on every row",
        )
    else:
        drill_id = drill.get("drill_id")
        if not isinstance(drill_id, str) or not drill_id.strip():
            fail(
                result,
                "experiment_registry.failure_drill_drill_id_required",
                "failure_drill.drill_id must be a non-empty string",
            )
        elif drill_id not in failure_drill_id_vocab:
            fail(
                result,
                "experiment_registry.failure_drill_drill_id_unknown",
                (
                    f"failure_drill.drill_id {drill_id!r} is not in "
                    "failure_drill_id_vocabulary"
                ),
            )
        forced_input = drill.get("forced_input")
        if not isinstance(forced_input, dict) or not forced_input:
            fail(
                result,
                "experiment_registry.failure_drill_forced_input_empty",
                "failure_drill.forced_input must declare at least one drift",
            )
        expected_check = drill.get("expected_check_id")
        if not isinstance(expected_check, str) or not expected_check.strip():
            fail(
                result,
                "experiment_registry.failure_drill_expected_check_id_required",
                "failure_drill.expected_check_id must be non-empty",
            )
        actionable = drill.get("actionable_next_action")
        if not isinstance(actionable, str) or not actionable.strip():
            fail(
                result,
                "experiment_registry.failure_drill_actionable_next_action_required",
                "failure_drill.actionable_next_action must be non-empty",
            )

    result.diagnostics.update(
        {
            "register_entry_id": entry_id,
            "row_kind_class": row_kind_class,
            "audience_class": audience_class,
            "lifecycle_state_class": lifecycle_state_class,
            "public_label_class": public_label_class,
            "default_posture_class": default_posture_class,
            "graduation_retirement_path_class": graduation_path,
            "review_by": review_by_raw,
            "expires_on": expires_on_raw,
        }
    )

    if not result.failed_checks:
        pass_(result, f"row {entry_id} passes")

    return result


def load_upstream_register(repo_root: Path, rel: str) -> dict[str, dict[str, Any]]:
    payload = ensure_dict(render_yaml_as_json(repo_root / rel), rel)
    rows = ensure_list(payload.get("rows"), f"{rel}.rows")
    out: dict[str, dict[str, Any]] = {}
    for row in rows:
        if not isinstance(row, dict):
            continue
        row_id = row.get("id")
        if isinstance(row_id, str) and row_id.strip():
            out[row_id] = row
    return out


def load_labs_register(repo_root: Path, rel: str) -> dict[str, dict[str, Any]]:
    payload = ensure_dict(render_yaml_as_json(repo_root / rel), rel)
    rows = ensure_list(payload.get("rows"), f"{rel}.rows")
    out: dict[str, dict[str, Any]] = {}
    for row in rows:
        if not isinstance(row, dict):
            continue
        labs_id = row.get("labs_id")
        if isinstance(labs_id, str) and labs_id.strip():
            out[labs_id] = row
    return out


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(
            f"--repo-root does not look like a repository root: {repo_root}"
        )

    matrix_rel = args.matrix
    matrix = ensure_dict(
        render_yaml_as_json(repo_root / matrix_rel), matrix_rel
    )

    findings: list[Finding] = []

    schema_version = matrix.get("schema_version")
    if schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="experiment_registry.envelope_schema_version_wrong",
                message=(
                    f"matrix schema_version must be 1; got {schema_version!r}"
                ),
                remediation="Bump runner together with the envelope schema.",
            )
        )

    matrix_id = ensure_str(matrix.get("matrix_id"), "matrix.matrix_id")
    if matrix_id != EXPECTED_MATRIX_ID:
        findings.append(
            Finding(
                severity="error",
                check_id="experiment_registry.envelope_matrix_id_wrong",
                message=(
                    f"matrix_id must be {EXPECTED_MATRIX_ID!r}; got "
                    f"{matrix_id!r}"
                ),
                remediation="Restore the canonical envelope matrix id.",
            )
        )

    ensure_str(matrix.get("status"), "matrix.status")

    as_of_raw = ensure_str(matrix.get("as_of"), "matrix.as_of")
    as_of = parse_iso_date(as_of_raw)
    if as_of is None:
        raise SystemExit(
            f"matrix.as_of {as_of_raw!r} must be an ISO-8601 date (YYYY-MM-DD)"
        )

    owner_dri = ensure_str(matrix.get("owner_dri"), "matrix.owner_dri")
    if not OWNER_DRI_PATTERN.match(owner_dri):
        findings.append(
            Finding(
                severity="error",
                check_id="experiment_registry.envelope_owner_dri_pattern_invalid",
                message=(
                    f"owner_dri {owner_dri!r} must match "
                    f"{OWNER_DRI_PATTERN.pattern!r}"
                ),
                remediation="Use an @handle for the owner DRI.",
            )
        )

    overview_page = ensure_str(
        matrix.get("overview_page"), "matrix.overview_page"
    )
    if not artifact_ref_exists(repo_root, overview_page):
        findings.append(
            Finding(
                severity="error",
                check_id="experiment_registry.envelope_overview_page_missing",
                message=f"overview_page does not exist: {overview_page}",
                remediation="Create the reviewer landing page or fix the path.",
                ref=overview_page,
            )
        )

    for key in ("row_schema_ref", "build_identity_ref"):
        ref = ensure_str(matrix.get(key), f"matrix.{key}")
        if not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"experiment_registry.envelope_{key}_missing",
                    message=f"{key} does not resolve: {ref}",
                    remediation="Fix the path or land the referenced artifact.",
                    ref=ref,
                )
            )

    validation_lane_ref = ensure_str(
        matrix.get("validation_lane_ref"), "matrix.validation_lane_ref"
    )
    if not artifact_ref_exists(repo_root, validation_lane_ref):
        findings.append(
            Finding(
                severity="error",
                check_id="experiment_registry.envelope_validation_lane_ref_missing",
                message=(
                    f"validation_lane_ref base does not exist: "
                    f"{validation_lane_ref}"
                ),
                remediation="Point at a seeded QE lane registry entry.",
                ref=validation_lane_ref,
            )
        )

    def load_vocab(key: str) -> set[str]:
        return {
            ensure_str(item, f"matrix.{key}[]")
            for item in ensure_list(matrix.get(key), f"matrix.{key}")
        }

    row_kind_class_vocab = load_vocab("row_kind_class_vocabulary")
    audience_class_vocab = load_vocab("audience_class_vocabulary")
    lifecycle_state_class_vocab = load_vocab("lifecycle_state_class_vocabulary")
    cohort_class_vocab = load_vocab("cohort_class_vocabulary")
    default_posture_class_vocab = load_vocab("default_posture_class_vocabulary")
    public_label_class_vocab = load_vocab("public_label_class_vocabulary")
    telemetry_posture_class_vocab = load_vocab(
        "telemetry_posture_class_vocabulary"
    )
    graduation_retirement_path_class_vocab = load_vocab(
        "graduation_retirement_path_class_vocabulary"
    )
    kill_switch_source_class_vocab = load_vocab(
        "kill_switch_source_class_vocabulary"
    )
    failure_drill_id_vocab = load_vocab("failure_drill_id_vocabulary")

    required_row_kind_class_coverage = load_vocab(
        "required_row_kind_class_coverage"
    )
    required_audience_class_coverage = load_vocab(
        "required_audience_class_coverage"
    )
    required_lifecycle_state_class_coverage = load_vocab(
        "required_lifecycle_state_class_coverage"
    )

    row_schema_ref = ensure_str(
        matrix.get("row_schema_ref"), "matrix.row_schema_ref"
    )

    # Closed-vocabulary agreement with the row schema.
    schema_vocab_pairs = [
        ("row_kind_class_vocabulary", "row_kind_class", row_kind_class_vocab),
        ("audience_class_vocabulary", "audience_class", audience_class_vocab),
        (
            "lifecycle_state_class_vocabulary",
            "lifecycle_state_class",
            lifecycle_state_class_vocab,
        ),
        ("cohort_class_vocabulary", "cohort_class", cohort_class_vocab),
        (
            "default_posture_class_vocabulary",
            "default_posture_class",
            default_posture_class_vocab,
        ),
        (
            "public_label_class_vocabulary",
            "public_label_class",
            public_label_class_vocab,
        ),
        (
            "telemetry_posture_class_vocabulary",
            "telemetry_posture_class",
            telemetry_posture_class_vocab,
        ),
        (
            "graduation_retirement_path_class_vocabulary",
            "graduation_retirement_path_class",
            graduation_retirement_path_class_vocab,
        ),
        (
            "kill_switch_source_class_vocabulary",
            "kill_switch_source_class",
            kill_switch_source_class_vocab,
        ),
    ]
    for envelope_key, defs_key, envelope_set in schema_vocab_pairs:
        schema_enum = set(load_schema_enum(repo_root, row_schema_ref, defs_key))
        if not schema_enum:
            findings.append(
                Finding(
                    severity="error",
                    check_id="experiment_registry.envelope_row_schema_defs_missing",
                    message=(
                        f"row schema {row_schema_ref} is missing $defs.{defs_key}.enum"
                    ),
                    remediation="Restore the row schema $defs entry.",
                    ref=row_schema_ref,
                )
            )
            continue
        diff = envelope_set.symmetric_difference(schema_enum)
        if diff:
            findings.append(
                Finding(
                    severity="error",
                    check_id=(
                        "experiment_registry.envelope_"
                        f"{envelope_key}_disagrees_with_row_schema"
                    ),
                    message=(
                        f"matrix.{envelope_key} disagrees with "
                        f"{row_schema_ref}#$defs.{defs_key}; "
                        f"matrix-only: {sorted(envelope_set - schema_enum)}; "
                        f"schema-only: {sorted(schema_enum - envelope_set)}"
                    ),
                    remediation=(
                        "Keep the matrix vocabulary in lock-step with the "
                        "row schema; the schema is canonical."
                    ),
                )
            )

    # --- named runtime consumers --------------------------------------
    consumers = ensure_list(
        matrix.get("named_runtime_consumers"),
        "matrix.named_runtime_consumers",
    )
    if not consumers:
        findings.append(
            Finding(
                severity="error",
                check_id="experiment_registry.envelope_named_runtime_consumers_empty",
                message=(
                    "named_runtime_consumers must declare at least one consumer"
                ),
                remediation=(
                    "Add at least one named runtime consumer that reads the seed."
                ),
            )
        )
    for idx, consumer in enumerate(consumers):
        consumer = ensure_dict(
            consumer, f"matrix.named_runtime_consumers[{idx}]"
        )
        ensure_str(
            consumer.get("consumer_id"),
            f"matrix.named_runtime_consumers[{idx}].consumer_id",
        )
        consumer_ref = ensure_str(
            consumer.get("consumer_ref"),
            f"matrix.named_runtime_consumers[{idx}].consumer_ref",
        )
        if not artifact_ref_exists(repo_root, consumer_ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="experiment_registry.named_runtime_consumer_ref_missing",
                    message=(
                        f"named_runtime_consumers[{idx}].consumer_ref does "
                        f"not exist: {consumer_ref}"
                    ),
                    remediation=(
                        "Fix the path or land the referenced consumer "
                        "before claiming it as live."
                    ),
                    ref=consumer_ref,
                )
            )
        consumed_fields = consumer.get("consumed_fields")
        if not isinstance(consumed_fields, list) or not consumed_fields:
            findings.append(
                Finding(
                    severity="error",
                    check_id="experiment_registry.named_runtime_consumer_consumed_fields_empty",
                    message=(
                        f"named_runtime_consumers[{idx}].consumed_fields "
                        "must be a non-empty list"
                    ),
                    remediation=(
                        "Name at least one field the consumer reads so "
                        "the consumer cannot regress to mentioned-but-"
                        "unread."
                    ),
                )
            )

    # --- companion-register paths -------------------------------------
    companion = ensure_dict(
        matrix.get("companion_registers"), "matrix.companion_registers"
    )
    exp_rel = ensure_str(
        companion.get("experiments_register"),
        "matrix.companion_registers.experiments_register",
    )
    labs_rel = ensure_str(
        companion.get("labs_register"),
        "matrix.companion_registers.labs_register",
    )
    for label, rel in (
        ("experiments_register", exp_rel),
        ("labs_register", labs_rel),
    ):
        if not artifact_ref_exists(repo_root, rel):
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"experiment_registry.companion_register_missing.{label}",
                    message=f"companion register {label} does not exist: {rel}",
                    remediation="Fix the path or land the companion register.",
                    ref=rel,
                )
            )

    exp_rows = load_upstream_register(repo_root, exp_rel)
    labs_rows = load_labs_register(repo_root, labs_rel)

    # --force-drill plumbing ---------------------------------------------
    forced_entry_id: str | None = None
    forced_drill_id: str | None = None
    if args.force_drill:
        if ":" not in args.force_drill:
            raise SystemExit(
                "--force-drill must be of the form "
                "'<register_entry_id>:<drill_id>'"
            )
        forced_entry_id, forced_drill_id = args.force_drill.rsplit(":", 1)
        forced_entry_id = forced_entry_id.strip()
        forced_drill_id = forced_drill_id.strip()

    rows = ensure_list(matrix.get("entries"), "matrix.entries")
    if not rows:
        findings.append(
            Finding(
                severity="error",
                check_id="experiment_registry.envelope_entries_empty",
                message="matrix.entries must declare at least one row",
                remediation="Seed at least one register row.",
            )
        )

    row_results: list[RowResult] = []
    seen_ids: set[str] = set()
    seen_row_kinds: set[str] = set()
    seen_audiences: set[str] = set()
    seen_lifecycle_states: set[str] = set()
    forced_replay_record: dict[str, Any] | None = None

    for idx, raw_row in enumerate(rows):
        raw_row = ensure_dict(raw_row, f"matrix.entries[{idx}]")
        entry_id_local = ensure_str(
            raw_row.get("register_entry_id"),
            f"matrix.entries[{idx}].register_entry_id",
        )

        applied_overrides: dict[str, Any] = {}
        replay_row_payload = raw_row
        drill_local: dict[str, Any] | None = None
        if (
            forced_entry_id is not None
            and entry_id_local == forced_entry_id
        ):
            drill_local = raw_row.get("failure_drill")
            if not isinstance(drill_local, dict):
                raise SystemExit(
                    f"--force-drill targeted register_entry_id "
                    f"{forced_entry_id!r} but the row has no failure_drill"
                )
            drill_id_local = drill_local.get("drill_id")
            if drill_id_local != forced_drill_id:
                raise SystemExit(
                    f"--force-drill drill_id {forced_drill_id!r} does not "
                    f"match the row's failure_drill.drill_id "
                    f"{drill_id_local!r}"
                )
            forced_input_local = drill_local.get("forced_input")
            if not isinstance(forced_input_local, dict):
                raise SystemExit(
                    f"failure_drill.forced_input must be an object on row "
                    f"{forced_entry_id!r}"
                )
            applied_overrides = forced_input_local
            replay_row_payload = apply_forced_overrides(
                raw_row, applied_overrides
            )

        result = validate_row(
            replay_row_payload,
            capability_label=entry_id_local,
            as_of=as_of,
            row_kind_class_vocab=row_kind_class_vocab,
            audience_class_vocab=audience_class_vocab,
            lifecycle_state_class_vocab=lifecycle_state_class_vocab,
            cohort_class_vocab=cohort_class_vocab,
            default_posture_class_vocab=default_posture_class_vocab,
            public_label_class_vocab=public_label_class_vocab,
            telemetry_posture_class_vocab=telemetry_posture_class_vocab,
            graduation_retirement_path_class_vocab=(
                graduation_retirement_path_class_vocab
            ),
            kill_switch_source_class_vocab=kill_switch_source_class_vocab,
            failure_drill_id_vocab=failure_drill_id_vocab,
        )
        if applied_overrides:
            result.diagnostics["forced_overrides_applied"] = applied_overrides
        row_results.append(result)

        if result.register_entry_id in seen_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="experiment_registry.entries_duplicate_register_entry_id",
                    message=(
                        f"duplicate register_entry_id: {result.register_entry_id}"
                    ),
                    remediation="register_entry_ids must be unique.",
                    ref=result.register_entry_id,
                )
            )
        seen_ids.add(result.register_entry_id)

        # Source-id resolution against the upstream register. The seed
        # cites the row regardless of --force-drill; the drill exercises
        # an unrelated drift in the row's other fields.
        src_reg_raw = raw_row.get("source_register")
        src_id_raw = raw_row.get("source_id")
        if (
            src_reg_raw == "experiments_register"
            and isinstance(src_id_raw, str)
            and src_id_raw not in exp_rows
        ):
            findings.append(
                Finding(
                    severity="error",
                    check_id="experiment_registry.source_id_not_found_in_companion_register",
                    message=(
                        f"source_id {src_id_raw!r} does not resolve in "
                        f"{exp_rel}"
                    ),
                    remediation=(
                        "Fix source_id or add the missing upstream row in "
                        "the companion experiments register."
                    ),
                    ref=f"{matrix_rel}#{result.register_entry_id}",
                )
            )

        # Labs-projection resolution per-row, skipped under --force-drill
        # so the drill verdict stays clean.
        if not (
            forced_entry_id is not None
            and result.register_entry_id == forced_entry_id
            and applied_overrides
        ):
            audience_local = replay_row_payload.get("audience_class")
            labs_ref_local = replay_row_payload.get("labs_projection_ref")
            if (
                audience_local == "contributor_visible_labs"
                and isinstance(labs_ref_local, str)
                and labs_ref_local.strip()
            ):
                path_part, _, fragment = labs_ref_local.partition("#")
                if path_part.strip() != labs_rel:
                    findings.append(
                        Finding(
                            severity="error",
                            check_id="experiment_registry.labs_projection_ref_must_target_labs_register",
                            message=(
                                f"labs_projection_ref {labs_ref_local!r} "
                                f"must target {labs_rel!r}"
                            ),
                            remediation=(
                                "Anchor the labs_projection_ref in the "
                                "canonical labs register."
                            ),
                            ref=f"{matrix_rel}#{result.register_entry_id}",
                        )
                    )
                elif not fragment:
                    findings.append(
                        Finding(
                            severity="error",
                            check_id="experiment_registry.labs_projection_ref_must_carry_labs_id",
                            message=(
                                f"labs_projection_ref {labs_ref_local!r} "
                                "must include a '#<labs_id>' fragment"
                            ),
                            remediation="Append the labs_register row id.",
                            ref=f"{matrix_rel}#{result.register_entry_id}",
                        )
                    )
                else:
                    labs_row = labs_rows.get(fragment)
                    if labs_row is None:
                        findings.append(
                            Finding(
                                severity="error",
                                check_id="experiment_registry.labs_projection_ref_labs_id_unresolved",
                                message=(
                                    f"labs_projection_ref labs_id "
                                    f"{fragment!r} does not resolve in "
                                    f"{labs_rel}"
                                ),
                                remediation=(
                                    "Fix the labs_id or land the missing "
                                    "labs_register row."
                                ),
                                ref=f"{matrix_rel}#{result.register_entry_id}",
                            )
                        )
                    elif isinstance(src_id_raw, str) and labs_row.get(
                        "experiment_ref"
                    ) != src_id_raw:
                        findings.append(
                            Finding(
                                severity="error",
                                check_id="experiment_registry.labs_projection_experiment_ref_disagrees_with_source_id",
                                message=(
                                    f"labs_register row {fragment!r} "
                                    f"experiment_ref "
                                    f"{labs_row.get('experiment_ref')!r} "
                                    f"disagrees with seed source_id "
                                    f"{src_id_raw!r}"
                                ),
                                remediation=(
                                    "Re-anchor the labs_register row on "
                                    "the matching experiment_ref."
                                ),
                                ref=f"{matrix_rel}#{result.register_entry_id}",
                            )
                        )

        if isinstance(raw_row.get("row_kind_class"), str):
            seen_row_kinds.add(raw_row["row_kind_class"])
        if isinstance(raw_row.get("audience_class"), str):
            seen_audiences.add(raw_row["audience_class"])
        if isinstance(raw_row.get("lifecycle_state_class"), str):
            seen_lifecycle_states.add(raw_row["lifecycle_state_class"])

        if (
            forced_entry_id is not None
            and result.register_entry_id == forced_entry_id
            and applied_overrides
            and isinstance(drill_local, dict)
        ):
            expected_check = ensure_str(
                drill_local.get("expected_check_id"),
                f"{forced_entry_id}.failure_drill.expected_check_id",
            )
            observed = [
                fc.get("check_id")
                for fc in result.failed_checks
                if isinstance(fc, dict)
            ]
            forced_replay_record = {
                "register_entry_id": forced_entry_id,
                "drill_id": forced_drill_id,
                "expected_check_id": expected_check,
                "observed_failed_check_ids": observed,
                "reproduced": expected_check in observed,
            }

    # --- required coverage ---------------------------------------------
    missing_kinds = required_row_kind_class_coverage - seen_row_kinds
    if missing_kinds:
        findings.append(
            Finding(
                severity="error",
                check_id="experiment_registry.coverage_missing_required_row_kind_classes",
                message=(
                    "matrix must seed at least one row for each required "
                    f"row_kind_class: {sorted(required_row_kind_class_coverage)};"
                    f" missing: {sorted(missing_kinds)}"
                ),
                remediation=(
                    "Add the missing rows so the canonical experiment / "
                    "feature_flag / benchmark_mode / rollout set is covered."
                ),
            )
        )

    missing_audiences = required_audience_class_coverage - seen_audiences
    if missing_audiences:
        findings.append(
            Finding(
                severity="error",
                check_id="experiment_registry.coverage_missing_required_audience_classes",
                message=(
                    "matrix must exercise each required audience_class "
                    f"{sorted(required_audience_class_coverage)};"
                    f" missing: {sorted(missing_audiences)}"
                ),
                remediation=(
                    "Add a row whose audience_class is each missing class."
                ),
            )
        )

    missing_lifecycles = (
        required_lifecycle_state_class_coverage - seen_lifecycle_states
    )
    if missing_lifecycles:
        findings.append(
            Finding(
                severity="error",
                check_id="experiment_registry.coverage_missing_required_lifecycle_state_classes",
                message=(
                    "matrix must exercise each required lifecycle_state_class "
                    f"{sorted(required_lifecycle_state_class_coverage)};"
                    f" missing: {sorted(missing_lifecycles)}"
                ),
                remediation=(
                    "Add a row whose lifecycle_state_class is each missing "
                    "state."
                ),
            )
        )

    # Promote per-row failures into findings, skipping the targeted row
    # under --force-drill so the runner's exit can reflect the drill verdict.
    for result in row_results:
        if (
            forced_entry_id is not None
            and result.register_entry_id == forced_entry_id
            and forced_replay_record is not None
        ):
            continue
        for failure in result.failed_checks:
            findings.append(
                Finding(
                    severity="error",
                    check_id=failure.get(
                        "check_id",
                        "experiment_registry.row_failed_check",
                    ),
                    message=(
                        f"{result.register_entry_id}: "
                        f"{failure.get('message', '')}"
                    ),
                    remediation=(
                        "Re-align the row with the experiment-registry "
                        "contract or fix the drift in the seed; failures "
                        "are reported with the precise actionable check_id."
                    ),
                    ref=result.register_entry_id,
                )
            )

    errors = [f for f in findings if f.severity == "error"]
    if forced_replay_record is not None and forced_replay_record["reproduced"]:
        status = "FORCE_DRILL_REPRODUCED"
    elif forced_replay_record is not None:
        status = "FORCE_DRILL_FAILED_TO_REPRODUCE"
    else:
        status = "PASS" if not errors else "FAIL"

    capture: dict[str, Any] = {
        "schema_version": 1,
        "capture_kind": "experiments_and_labs_seed_validation_capture",
        "captured_at": now_iso_z(),
        "owner_dri": owner_dri,
        "matrix_ref": matrix_rel,
        "envelope_schema_ref": args.envelope_schema,
        "row_schema_ref": args.row_schema,
        "exact_build_identity_ref": args.build_identity,
        "command": (
            "python3 tests/governance/m1_experiments_and_labs_seed_lane/"
            "run_m1_experiments_and_labs_seed_lane.py --repo-root ."
        ),
        "status": status,
        "as_of": as_of.isoformat(),
        "required_row_kind_class_coverage": sorted(
            required_row_kind_class_coverage
        ),
        "observed_row_kind_classes": sorted(seen_row_kinds),
        "required_audience_class_coverage": sorted(
            required_audience_class_coverage
        ),
        "observed_audience_classes": sorted(seen_audiences),
        "required_lifecycle_state_class_coverage": sorted(
            required_lifecycle_state_class_coverage
        ),
        "observed_lifecycle_state_classes": sorted(seen_lifecycle_states),
        "rows": [
            {
                "register_entry_id": r.register_entry_id,
                "row_kind_class": r.row_kind_class,
                "audience_class": r.audience_class,
                "passed_checks": r.passed_checks,
                "failed_checks": r.failed_checks,
                "diagnostics": r.diagnostics,
            }
            for r in row_results
        ],
        "finding_counts": {
            "error": sum(1 for f in findings if f.severity == "error"),
            "warning": sum(1 for f in findings if f.severity == "warning"),
        },
        "findings": [f.as_report() for f in findings],
    }

    if forced_replay_record is not None:
        capture["forced_drill_replay"] = forced_replay_record

    report_path = repo_root / args.report
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text(
        json.dumps(capture, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )

    label = "experiments-and-labs-seed"
    print(
        f"[{label}] {status} ({len(errors)} errors, "
        f"{len(findings) - len(errors)} warnings) — capture: {args.report}"
    )
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        print(
            f"[{label}] {prefix} {finding.check_id}: {finding.message}"
            f"{ref_suffix}"
        )
        print(f"[{label}]   remediation: {finding.remediation}")

    if forced_replay_record is not None:
        if forced_replay_record["reproduced"]:
            print(
                f"[{label}] forced drill {forced_replay_record['drill_id']}"
                f" on {forced_replay_record['register_entry_id']} reproduced"
                f" {forced_replay_record['expected_check_id']}"
            )
            return 0
        print(
            f"[{label}] forced drill {forced_replay_record['drill_id']} on"
            f" {forced_replay_record['register_entry_id']} did NOT reproduce"
            f" {forced_replay_record['expected_check_id']};"
            f" observed: {forced_replay_record['observed_failed_check_ids']}"
        )
        return 2

    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[experiments-and-labs-seed] interrupted", file=sys.stderr)
        sys.exit(130)
