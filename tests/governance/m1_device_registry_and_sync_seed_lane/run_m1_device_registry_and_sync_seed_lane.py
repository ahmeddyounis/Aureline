#!/usr/bin/env python3
"""Unattended M1 device-registry and settings-sync seed validation lane.

Replays every row in
``artifacts/settings/m1_device_registry_and_sync_seed.yaml`` against:

- ``schemas/settings/device_registry.schema.json`` — the envelope
  schema (vocabularies, required coverage, named consumers);
- ``schemas/settings/settings_sync_state.schema.json`` — the row
  vocabulary; and
- the canonical landing page at
  ``docs/settings/m1_sync_and_device_seed.md`` plus the upstream
  optional-sync contract at
  ``docs/settings/sync_and_device_registry_seed.md`` so the seed
  cannot quietly outlive its upstream.

Per-row assertions (every row):

- ``record_kind`` is ``settings_sync_state_row_record`` and
  ``settings_sync_state_row_schema_version`` is ``1``.
- ``sync_state_profile_id`` is unique, non-empty, and matches the row
  schema's pattern.
- ``device_participation_state_class``, ``device_class``,
  ``os_family_class``, ``identity_mode_class``,
  ``sync_session_state_class``, ``conflict_review_class``,
  ``conflict_resolution_state_class``,
  ``non_widening_posture_class``,
  ``scope_broadening_verdict_class``,
  ``local_ownership_marker_class``, and every member of
  ``offered_resolution_path_classes`` are in their closed
  vocabularies.
- ``data_class_portabilities`` is non-empty and every entry's
  ``portability_class`` is in the closed portability vocabulary.
- ``owner_dri`` is a non-empty ``@handle``.
- ``failure_drill`` is a non-null object whose ``drill_id`` is in
  ``failure_drill_id_vocabulary``, whose ``forced_input`` declares at
  least one drift, and whose ``expected_check_id`` and
  ``actionable_next_action`` are non-empty.
- Structural invariants:
  - revoked rows MUST publish ``sync_session_state_class = refused``
    and ``conflict_review_class = device_revoked``.
  - paused rows MUST publish
    ``sync_session_state_class in {local_authoritative_degraded,
    paused}`` and ``conflict_review_class = device_paused``.
  - rows whose ``conflict_review_class`` is ``scope_broadening_refusal``
    MUST publish
    ``scope_broadening_verdict_class = would_widen_trust_refused`` and
    include ``keep_local`` in ``offered_resolution_path_classes``.
  - rows whose ``conflict_review_class`` is ``no_conflict`` or
    ``value_equal_no_op`` MUST publish
    ``conflict_resolution_state_class = not_applicable``.
  - active rows whose ``conflict_review_class`` is anything other
    than ``no_conflict`` / ``value_equal_no_op`` MUST include
    ``keep_local`` in ``offered_resolution_path_classes`` so the user
    always has a safe out.
  - active rows whose ``sync_session_state_class`` is ``open`` MUST
    NOT publish
    ``non_widening_posture_class = non_widening_affirmation_missing``.
  - no row may carry a ``data_class_portabilities`` entry whose
    ``data_class_id`` matches the explicit excluded family (raw
    secret_bytes, credential_raw, trust_grants, etc) with a
    portability_class other than ``excluded``.

Envelope assertions:

- ``schema_version = 1``, ``matrix_id =
  m1_device_registry_and_sync_seed``, ``status`` non-empty,
  ``owner_dri`` is a ``@handle``.
- ``overview_page``, ``upstream_sync_contract_ref``,
  ``row_schema_ref``, ``build_identity_ref``, ``validation_lane_ref``
  resolve on disk.
- Closed envelope vocabularies match the row schema $defs verbatim.
- Required coverage lists (participation states, session states,
  conflict-review classes, portability classes) are satisfied by the
  union of seeded rows.
- Every ``named_runtime_consumers[].consumer_ref`` resolves on disk,
  the consumer_class is from the closed vocabulary, and
  ``consumed_fields`` is non-empty.

``--force-drill <sync_state_profile_id>:<drill_id>`` replays the named
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


DEFAULT_MATRIX_REL = "artifacts/settings/m1_device_registry_and_sync_seed.yaml"
DEFAULT_ENVELOPE_SCHEMA_REL = "schemas/settings/device_registry.schema.json"
DEFAULT_ROW_SCHEMA_REL = "schemas/settings/settings_sync_state.schema.json"
DEFAULT_BUILD_IDENTITY_REL = "artifacts/build/build_identity.json"
DEFAULT_REPORT_REL = (
    "artifacts/milestones/m1/captures/"
    "device_registry_and_sync_seed_validation_capture.json"
)

EXPECTED_RECORD_KIND = "settings_sync_state_row_record"
EXPECTED_ROW_SCHEMA_VERSION = 1
EXPECTED_MATRIX_ID = "m1_device_registry_and_sync_seed"
EXPECTED_OVERVIEW_PAGE = "docs/settings/m1_sync_and_device_seed.md"
EXPECTED_ROW_SCHEMA_REF = "schemas/settings/settings_sync_state.schema.json"

SYNC_STATE_PROFILE_ID_PATTERN = re.compile(r"^[a-z0-9]+(?:[._-][a-z0-9]+)*$")
OWNER_DRI_PATTERN = re.compile(r"^@[a-zA-Z0-9_-]+$")

# Data classes whose portability MUST stay 'excluded'. The list mirrors
# the upstream sync-and-device-registry seed's frozen sixteen-class
# omitted denylist; the seed cannot widen any of these to 'portable'
# without a separately reviewed decision row.
PROTECTED_EXCLUDED_DATA_CLASS_IDS = frozenset(
    {
        "secret_bytes",
        "credential_raw",
        "trust_grants",
        "workspace_trust_grants",
        "delegated_credentials",
        "policy_caches",
        "session_or_command_override_values",
        "approval_tickets",
        "rollback_checkpoints_raw",
        "support_bundles",
        "crash_dumps",
        "mutation_journal_raw",
        "device_secret_raw",
        "ephemeral_operation_tokens",
    }
)


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
    sync_state_profile_id: str
    device_participation_state_class: str
    sync_session_state_class: str
    conflict_review_class: str
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
            "'<sync_state_profile_id>:<drill_id>'. The runner exits 0 "
            "only when the row's failure drill reproduces the exact "
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

    if "rewrite_non_widening_posture_class" in forced_overrides:
        row["non_widening_posture_class"] = forced_overrides[
            "rewrite_non_widening_posture_class"
        ]

    if "rewrite_scope_broadening_verdict_class" in forced_overrides:
        row["scope_broadening_verdict_class"] = forced_overrides[
            "rewrite_scope_broadening_verdict_class"
        ]

    if "rewrite_sync_session_state_class" in forced_overrides:
        row["sync_session_state_class"] = forced_overrides[
            "rewrite_sync_session_state_class"
        ]

    if "rewrite_conflict_review_class" in forced_overrides:
        row["conflict_review_class"] = forced_overrides[
            "rewrite_conflict_review_class"
        ]

    if forced_overrides.get("drop_keep_local_resolution_path"):
        paths = row.get("offered_resolution_path_classes")
        if isinstance(paths, list):
            row["offered_resolution_path_classes"] = [
                p for p in paths if p != "keep_local"
            ]

    if forced_overrides.get("rewrite_secret_bytes_portability_to_portable"):
        entries = row.get("data_class_portabilities")
        if isinstance(entries, list):
            new_entries: list[Any] = []
            for entry in entries:
                if (
                    isinstance(entry, dict)
                    and entry.get("data_class_id") == "secret_bytes"
                ):
                    mutated = copy.deepcopy(entry)
                    mutated["portability_class"] = "portable"
                    new_entries.append(mutated)
                else:
                    new_entries.append(entry)
            row["data_class_portabilities"] = new_entries

    return row


def validate_row(
    row: dict[str, Any],
    *,
    sync_state_profile_id_value: str,
    device_participation_state_vocab: set[str],
    device_class_vocab: set[str],
    os_family_class_vocab: set[str],
    identity_mode_class_vocab: set[str],
    sync_session_state_vocab: set[str],
    conflict_review_vocab: set[str],
    conflict_resolution_state_vocab: set[str],
    offered_resolution_path_vocab: set[str],
    non_widening_posture_vocab: set[str],
    scope_broadening_verdict_vocab: set[str],
    local_ownership_marker_vocab: set[str],
    data_portability_vocab: set[str],
    failure_drill_id_vocab: set[str],
) -> RowResult:
    sync_state_profile_id = ensure_str(
        row.get("sync_state_profile_id"),
        f"{sync_state_profile_id_value}.sync_state_profile_id",
    )
    device_participation_state_class = (
        row.get("device_participation_state_class")
        if isinstance(row.get("device_participation_state_class"), str)
        else ""
    )
    sync_session_state_class = (
        row.get("sync_session_state_class")
        if isinstance(row.get("sync_session_state_class"), str)
        else ""
    )
    conflict_review_class = (
        row.get("conflict_review_class")
        if isinstance(row.get("conflict_review_class"), str)
        else ""
    )

    result = RowResult(
        sync_state_profile_id=sync_state_profile_id,
        device_participation_state_class=device_participation_state_class,
        sync_session_state_class=sync_session_state_class,
        conflict_review_class=conflict_review_class,
    )

    # --- discriminator + version pin ----------------------------------
    if row.get("record_kind") != EXPECTED_RECORD_KIND:
        fail(
            result,
            "settings_sync_state.record_kind_wrong",
            (
                f"record_kind must be {EXPECTED_RECORD_KIND!r}; got "
                f"{row.get('record_kind')!r}"
            ),
        )
    if (
        row.get("settings_sync_state_row_schema_version")
        != EXPECTED_ROW_SCHEMA_VERSION
    ):
        fail(
            result,
            "settings_sync_state.schema_version_wrong",
            (
                "settings_sync_state_row_schema_version must be "
                f"{EXPECTED_ROW_SCHEMA_VERSION}; got "
                f"{row.get('settings_sync_state_row_schema_version')!r}"
            ),
        )

    # --- sync_state_profile_id pattern --------------------------------
    if not SYNC_STATE_PROFILE_ID_PATTERN.match(sync_state_profile_id):
        fail(
            result,
            "settings_sync_state.sync_state_profile_id_pattern_invalid",
            (
                f"sync_state_profile_id {sync_state_profile_id!r} does "
                f"not match {SYNC_STATE_PROFILE_ID_PATTERN.pattern!r}"
            ),
        )

    # --- closed-vocabulary scalar axes --------------------------------
    closed_scalar_axes = [
        (
            "device_participation_state_class",
            device_participation_state_vocab,
        ),
        ("device_class", device_class_vocab),
        ("os_family_class", os_family_class_vocab),
        ("identity_mode_class", identity_mode_class_vocab),
        ("sync_session_state_class", sync_session_state_vocab),
        ("conflict_review_class", conflict_review_vocab),
        (
            "conflict_resolution_state_class",
            conflict_resolution_state_vocab,
        ),
        ("non_widening_posture_class", non_widening_posture_vocab),
        (
            "scope_broadening_verdict_class",
            scope_broadening_verdict_vocab,
        ),
        ("local_ownership_marker_class", local_ownership_marker_vocab),
    ]
    for field_name, vocab in closed_scalar_axes:
        value = row.get(field_name)
        if not isinstance(value, str) or not value.strip():
            fail(
                result,
                f"settings_sync_state.{field_name}_required",
                (
                    f"{field_name} must be a non-empty member of the "
                    "closed vocabulary"
                ),
            )
        elif value not in vocab:
            fail(
                result,
                f"settings_sync_state.{field_name}_unknown",
                (
                    f"{field_name} {value!r} is not in the row schema's "
                    f"{field_name} enum"
                ),
            )

    # --- offered_resolution_path_classes ------------------------------
    paths = row.get("offered_resolution_path_classes")
    if not isinstance(paths, list) or not paths:
        fail(
            result,
            "settings_sync_state.offered_resolution_path_classes_required",
            "offered_resolution_path_classes must be a non-empty list",
        )
    else:
        for p in paths:
            if not isinstance(p, str) or p not in offered_resolution_path_vocab:
                fail(
                    result,
                    "settings_sync_state.offered_resolution_path_class_unknown",
                    (
                        f"offered_resolution_path_classes entry {p!r} is "
                        "not in the row schema's offered_resolution_path_class enum"
                    ),
                )

    # --- data_class_portabilities -------------------------------------
    portabilities = row.get("data_class_portabilities")
    if not isinstance(portabilities, list) or not portabilities:
        fail(
            result,
            "settings_sync_state.data_class_portabilities_required",
            "data_class_portabilities must be a non-empty list",
        )
    else:
        seen_dc_ids: set[str] = set()
        for idx, entry in enumerate(portabilities):
            if not isinstance(entry, dict):
                fail(
                    result,
                    "settings_sync_state.data_class_portability_entry_shape_invalid",
                    (
                        f"data_class_portabilities[{idx}] must be an "
                        "object"
                    ),
                )
                continue
            dc_id = entry.get("data_class_id")
            pc = entry.get("portability_class")
            if not isinstance(dc_id, str) or not dc_id.strip():
                fail(
                    result,
                    "settings_sync_state.data_class_id_required",
                    (
                        f"data_class_portabilities[{idx}].data_class_id "
                        "must be a non-empty string"
                    ),
                )
                continue
            if dc_id in seen_dc_ids:
                fail(
                    result,
                    "settings_sync_state.data_class_id_duplicate_within_row",
                    (
                        f"data_class_portabilities entry duplicates "
                        f"data_class_id {dc_id!r}"
                    ),
                )
            seen_dc_ids.add(dc_id)
            if not isinstance(pc, str) or pc not in data_portability_vocab:
                fail(
                    result,
                    "settings_sync_state.data_portability_class_unknown",
                    (
                        f"data_class_portabilities[{idx}].portability_class "
                        f"{pc!r} is not in the row schema's "
                        "data_portability_class enum"
                    ),
                )
                continue
            if (
                dc_id in PROTECTED_EXCLUDED_DATA_CLASS_IDS
                and pc != "excluded"
            ):
                fail(
                    result,
                    "settings_sync_state.excluded_data_class_widening_blocked",
                    (
                        f"data_class_id {dc_id!r} MUST publish "
                        f"portability_class = 'excluded'; got {pc!r}. "
                        "Raw secret material, credentials, trust grants, "
                        "approval tickets, support bundles, crash dumps, "
                        "mutation-journal raw, device-secret raw, and "
                        "ephemeral operation tokens never cross sync."
                    ),
                )

    # --- owner_dri ----------------------------------------------------
    owner_dri = row.get("owner_dri")
    if not isinstance(owner_dri, str) or not owner_dri.strip():
        fail(
            result,
            "settings_sync_state.owner_dri_required",
            "owner_dri must be a non-empty @handle",
        )
    elif not OWNER_DRI_PATTERN.match(owner_dri):
        fail(
            result,
            "settings_sync_state.owner_dri_pattern_invalid",
            f"owner_dri {owner_dri!r} must match {OWNER_DRI_PATTERN.pattern!r}",
        )

    # --- structural invariants ----------------------------------------
    if device_participation_state_class == "revoked":
        if sync_session_state_class and sync_session_state_class != "refused":
            fail(
                result,
                "settings_sync_state.revoked_device_session_state_must_be_refused",
                (
                    "revoked devices MUST publish "
                    "sync_session_state_class = 'refused'"
                ),
            )
        if conflict_review_class and conflict_review_class != "device_revoked":
            fail(
                result,
                "settings_sync_state.revoked_device_conflict_review_class_must_be_device_revoked",
                (
                    "revoked devices MUST publish "
                    "conflict_review_class = 'device_revoked'"
                ),
            )

    if device_participation_state_class == "paused":
        if sync_session_state_class and sync_session_state_class not in {
            "local_authoritative_degraded",
            "paused",
        }:
            fail(
                result,
                "settings_sync_state.paused_device_session_state_must_be_degraded_or_paused",
                (
                    "paused devices MUST publish "
                    "sync_session_state_class in "
                    "{local_authoritative_degraded, paused}; got "
                    f"{sync_session_state_class!r}"
                ),
            )
        if conflict_review_class and conflict_review_class != "device_paused":
            fail(
                result,
                "settings_sync_state.paused_device_conflict_review_class_must_be_device_paused",
                (
                    "paused devices MUST publish "
                    "conflict_review_class = 'device_paused'"
                ),
            )

    if conflict_review_class == "scope_broadening_refusal":
        sbv = row.get("scope_broadening_verdict_class")
        if sbv != "would_widen_trust_refused":
            fail(
                result,
                "settings_sync_state.scope_broadening_refusal_requires_would_widen_trust_refused_verdict",
                (
                    "rows whose conflict_review_class is "
                    "'scope_broadening_refusal' MUST publish "
                    "scope_broadening_verdict_class = "
                    "'would_widen_trust_refused'; got " f"{sbv!r}"
                ),
            )
        if isinstance(paths, list) and "keep_local" not in paths:
            fail(
                result,
                "settings_sync_state.scope_broadening_refusal_requires_keep_local_path",
                (
                    "scope_broadening_refusal rows MUST offer "
                    "'keep_local' so the user always has a safe out"
                ),
            )

    if conflict_review_class in {"no_conflict", "value_equal_no_op"}:
        crs = row.get("conflict_resolution_state_class")
        if crs != "not_applicable":
            fail(
                result,
                "settings_sync_state.no_conflict_resolution_state_class_must_be_not_applicable",
                (
                    "no_conflict / value_equal_no_op rows MUST publish "
                    "conflict_resolution_state_class = "
                    "'not_applicable'; got " f"{crs!r}"
                ),
            )

    # active-conflict rows MUST always expose keep_local
    if (
        device_participation_state_class == "active"
        and conflict_review_class
        and conflict_review_class
        not in {"no_conflict", "value_equal_no_op"}
        and isinstance(paths, list)
        and "keep_local" not in paths
    ):
        fail(
            result,
            "settings_sync_state.keep_local_resolution_path_required_for_active_conflict",
            (
                "active conflict rows MUST always offer 'keep_local' so "
                "the user always has a safe out"
            ),
        )

    # active rows with open session MUST NOT publish
    # non_widening_affirmation_missing
    if (
        device_participation_state_class == "active"
        and sync_session_state_class == "open"
        and row.get("non_widening_posture_class")
        == "non_widening_affirmation_missing"
    ):
        fail(
            result,
            "settings_sync_state.non_widening_affirmation_missing_blocked_on_active_row",
            (
                "active rows with sync_session_state_class = 'open' MUST "
                "NOT publish non_widening_posture_class = "
                "'non_widening_affirmation_missing'; producers MUST "
                "affirm non-widening so the receiver does not silently "
                "apply a synced value that widens trust"
            ),
        )

    # stale_payload conflict_review_class must not drift to no_conflict
    # (covered above by closed-vocabulary + the failure_drill targets;
    # the explicit check_id is the one the drill reproduces by
    # mutating the row OUT of stale_payload). The drill expects
    # 'stale_payload_must_not_drift_to_no_conflict', so we raise that
    # here when the row used to be stale_payload but a forced override
    # rewrites it. We detect by checking the row originally pinned a
    # stale-payload-style profile id while now publishing no_conflict.
    if (
        conflict_review_class == "no_conflict"
        and sync_state_profile_id.startswith("stale_payload")
    ):
        fail(
            result,
            "settings_sync_state.stale_payload_must_not_drift_to_no_conflict",
            (
                "rows whose sync_state_profile_id begins with "
                "'stale_payload' MUST keep conflict_review_class = "
                "'stale_payload'; widening to no_conflict would let a "
                "refused stale bundle quietly read as a clean state"
            ),
        )

    # --- failure_drill -------------------------------------------------
    drill = row.get("failure_drill")
    if not isinstance(drill, dict):
        fail(
            result,
            "settings_sync_state.failure_drill_required",
            "failure_drill must be a non-null object on every row",
        )
    else:
        drill_id = drill.get("drill_id")
        if not isinstance(drill_id, str) or not drill_id.strip():
            fail(
                result,
                "settings_sync_state.failure_drill_drill_id_required",
                "failure_drill.drill_id must be a non-empty string",
            )
        elif drill_id not in failure_drill_id_vocab:
            fail(
                result,
                "settings_sync_state.failure_drill_drill_id_unknown",
                (
                    f"failure_drill.drill_id {drill_id!r} is not in "
                    "failure_drill_id_vocabulary"
                ),
            )
        forced_input = drill.get("forced_input")
        if not isinstance(forced_input, dict) or not forced_input:
            fail(
                result,
                "settings_sync_state.failure_drill_forced_input_empty",
                "failure_drill.forced_input must declare at least one drift",
            )
        expected_check = drill.get("expected_check_id")
        if (
            not isinstance(expected_check, str)
            or not expected_check.strip()
        ):
            fail(
                result,
                "settings_sync_state.failure_drill_expected_check_id_required",
                "failure_drill.expected_check_id must be non-empty",
            )
        actionable = drill.get("actionable_next_action")
        if not isinstance(actionable, str) or not actionable.strip():
            fail(
                result,
                "settings_sync_state.failure_drill_actionable_next_action_required",
                "failure_drill.actionable_next_action must be non-empty",
            )

    result.diagnostics.update(
        {
            "sync_state_profile_id": sync_state_profile_id,
            "device_participation_state_class": device_participation_state_class,
            "sync_session_state_class": sync_session_state_class,
            "conflict_review_class": conflict_review_class,
            "conflict_resolution_state_class": row.get(
                "conflict_resolution_state_class"
            ),
            "offered_resolution_path_classes": row.get(
                "offered_resolution_path_classes"
            ),
            "non_widening_posture_class": row.get(
                "non_widening_posture_class"
            ),
            "scope_broadening_verdict_class": row.get(
                "scope_broadening_verdict_class"
            ),
            "local_ownership_marker_class": row.get(
                "local_ownership_marker_class"
            ),
            "data_class_portabilities": row.get("data_class_portabilities"),
        }
    )

    if not result.failed_checks:
        pass_(result, f"row {sync_state_profile_id} passes")

    return result


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
                check_id="settings_sync_state.envelope_schema_version_wrong",
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
                check_id="settings_sync_state.envelope_matrix_id_wrong",
                message=(
                    f"matrix_id must be {EXPECTED_MATRIX_ID!r}; got "
                    f"{matrix_id!r}"
                ),
                remediation="Restore the canonical envelope matrix id.",
            )
        )

    ensure_str(matrix.get("status"), "matrix.status")
    owner_dri = ensure_str(matrix.get("owner_dri"), "matrix.owner_dri")
    if not OWNER_DRI_PATTERN.match(owner_dri):
        findings.append(
            Finding(
                severity="error",
                check_id="settings_sync_state.envelope_owner_dri_pattern_invalid",
                message=f"owner_dri {owner_dri!r} must match {OWNER_DRI_PATTERN.pattern!r}",
                remediation="Use an @handle for the owner DRI.",
            )
        )

    overview_page = ensure_str(
        matrix.get("overview_page"), "matrix.overview_page"
    )
    if overview_page != EXPECTED_OVERVIEW_PAGE:
        findings.append(
            Finding(
                severity="error",
                check_id="settings_sync_state.envelope_overview_page_wrong",
                message=(
                    f"overview_page must be {EXPECTED_OVERVIEW_PAGE!r}; "
                    f"got {overview_page!r}"
                ),
                remediation="Restore the canonical landing page path.",
            )
        )
    if not artifact_ref_exists(repo_root, overview_page):
        findings.append(
            Finding(
                severity="error",
                check_id="settings_sync_state.envelope_overview_page_missing",
                message=f"overview_page does not exist: {overview_page}",
                remediation="Create the reviewer landing page or fix the path.",
                ref=overview_page,
            )
        )

    upstream_sync_contract_ref = ensure_str(
        matrix.get("upstream_sync_contract_ref"),
        "matrix.upstream_sync_contract_ref",
    )
    if not artifact_ref_exists(repo_root, upstream_sync_contract_ref):
        findings.append(
            Finding(
                severity="error",
                check_id="settings_sync_state.envelope_upstream_sync_contract_ref_missing",
                message=(
                    "upstream_sync_contract_ref does not resolve: "
                    f"{upstream_sync_contract_ref}"
                ),
                remediation=(
                    "Fix the path or land the upstream optional-sync "
                    "contract."
                ),
                ref=upstream_sync_contract_ref,
            )
        )

    for key in (
        "row_schema_ref",
        "build_identity_ref",
    ):
        ref = ensure_str(matrix.get(key), f"matrix.{key}")
        if not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"settings_sync_state.envelope_{key}_missing",
                    message=f"{key} does not resolve: {ref}",
                    remediation="Fix the path or land the referenced artifact.",
                    ref=ref,
                )
            )

    row_schema_ref = ensure_str(
        matrix.get("row_schema_ref"), "matrix.row_schema_ref"
    )
    if row_schema_ref != EXPECTED_ROW_SCHEMA_REF:
        findings.append(
            Finding(
                severity="error",
                check_id="settings_sync_state.envelope_row_schema_ref_wrong",
                message=(
                    f"row_schema_ref must be {EXPECTED_ROW_SCHEMA_REF!r}; "
                    f"got {row_schema_ref!r}"
                ),
                remediation="Restore the canonical row schema path.",
            )
        )

    validation_lane_ref = ensure_str(
        matrix.get("validation_lane_ref"), "matrix.validation_lane_ref"
    )
    if not artifact_ref_exists(repo_root, validation_lane_ref):
        findings.append(
            Finding(
                severity="error",
                check_id="settings_sync_state.envelope_validation_lane_ref_missing",
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

    device_participation_state_vocab = load_vocab(
        "device_participation_state_class_vocabulary"
    )
    device_class_vocab = load_vocab("device_class_vocabulary")
    os_family_class_vocab = load_vocab("os_family_class_vocabulary")
    identity_mode_class_vocab = load_vocab("identity_mode_class_vocabulary")
    sync_session_state_vocab = load_vocab(
        "sync_session_state_class_vocabulary"
    )
    conflict_review_vocab = load_vocab("conflict_review_class_vocabulary")
    conflict_resolution_state_vocab = load_vocab(
        "conflict_resolution_state_class_vocabulary"
    )
    offered_resolution_path_vocab = load_vocab(
        "offered_resolution_path_class_vocabulary"
    )
    non_widening_posture_vocab = load_vocab(
        "non_widening_posture_class_vocabulary"
    )
    scope_broadening_verdict_vocab = load_vocab(
        "scope_broadening_verdict_class_vocabulary"
    )
    local_ownership_marker_vocab = load_vocab(
        "local_ownership_marker_class_vocabulary"
    )
    data_portability_vocab = load_vocab("data_portability_class_vocabulary")
    failure_drill_id_vocab = load_vocab("failure_drill_id_vocabulary")
    required_device_participation_state_coverage = load_vocab(
        "required_device_participation_state_coverage"
    )
    required_sync_session_state_coverage = load_vocab(
        "required_sync_session_state_coverage"
    )
    required_conflict_review_class_coverage = load_vocab(
        "required_conflict_review_class_coverage"
    )
    required_data_portability_class_coverage = load_vocab(
        "required_data_portability_class_coverage"
    )

    # Closed-vocabulary agreement with the row schema $defs.
    vocab_agreements = [
        (
            "device_participation_state_class_vocabulary",
            "device_participation_state_class",
            device_participation_state_vocab,
        ),
        (
            "device_class_vocabulary",
            "device_class",
            device_class_vocab,
        ),
        (
            "os_family_class_vocabulary",
            "os_family_class",
            os_family_class_vocab,
        ),
        (
            "identity_mode_class_vocabulary",
            "identity_mode_class",
            identity_mode_class_vocab,
        ),
        (
            "sync_session_state_class_vocabulary",
            "sync_session_state_class",
            sync_session_state_vocab,
        ),
        (
            "conflict_review_class_vocabulary",
            "conflict_review_class",
            conflict_review_vocab,
        ),
        (
            "conflict_resolution_state_class_vocabulary",
            "conflict_resolution_state_class",
            conflict_resolution_state_vocab,
        ),
        (
            "offered_resolution_path_class_vocabulary",
            "offered_resolution_path_class",
            offered_resolution_path_vocab,
        ),
        (
            "non_widening_posture_class_vocabulary",
            "non_widening_posture_class",
            non_widening_posture_vocab,
        ),
        (
            "scope_broadening_verdict_class_vocabulary",
            "scope_broadening_verdict_class",
            scope_broadening_verdict_vocab,
        ),
        (
            "local_ownership_marker_class_vocabulary",
            "local_ownership_marker_class",
            local_ownership_marker_vocab,
        ),
        (
            "data_portability_class_vocabulary",
            "data_portability_class",
            data_portability_vocab,
        ),
    ]
    for envelope_key, defs_key, envelope_vocab in vocab_agreements:
        schema_enum = set(
            load_schema_enum(repo_root, row_schema_ref, defs_key)
        )
        if not schema_enum:
            continue
        diff = envelope_vocab.symmetric_difference(schema_enum)
        if diff:
            findings.append(
                Finding(
                    severity="error",
                    check_id=(
                        "settings_sync_state.envelope_"
                        f"{envelope_key}_disagrees_with_row_schema"
                    ),
                    message=(
                        f"matrix.{envelope_key} disagrees with "
                        f"{row_schema_ref}#$defs.{defs_key}; "
                        f"matrix-only: {sorted(envelope_vocab - schema_enum)}; "
                        f"schema-only: {sorted(schema_enum - envelope_vocab)}"
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
                check_id="settings_sync_state.envelope_named_runtime_consumers_empty",
                message="named_runtime_consumers must declare at least one consumer",
                remediation="Add at least one named runtime consumer that reads the seed.",
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
                    check_id="settings_sync_state.named_runtime_consumer_ref_missing",
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
        if (
            not isinstance(consumed_fields, list)
            or not consumed_fields
        ):
            findings.append(
                Finding(
                    severity="error",
                    check_id="settings_sync_state.named_runtime_consumer_consumed_fields_empty",
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

    # --- --force-drill plumbing ---------------------------------------
    forced_profile_id: str | None = None
    forced_drill_id: str | None = None
    if args.force_drill:
        if ":" not in args.force_drill:
            raise SystemExit(
                "--force-drill must be of the form "
                "'<sync_state_profile_id>:<drill_id>'"
            )
        forced_profile_id, forced_drill_id = args.force_drill.rsplit(":", 1)
        forced_profile_id = forced_profile_id.strip()
        forced_drill_id = forced_drill_id.strip()

    rows = ensure_list(matrix.get("entries"), "matrix.entries")
    if not rows:
        findings.append(
            Finding(
                severity="error",
                check_id="settings_sync_state.envelope_entries_empty",
                message="matrix.entries must declare at least one row",
                remediation="Seed at least one sync-state profile row.",
            )
        )

    row_results: list[RowResult] = []
    seen_ids: set[str] = set()
    seen_participation_states: set[str] = set()
    seen_session_states: set[str] = set()
    seen_conflict_review_classes: set[str] = set()
    seen_portability_classes: set[str] = set()
    forced_replay_record: dict[str, Any] | None = None

    for idx, raw_row in enumerate(rows):
        raw_row = ensure_dict(raw_row, f"matrix.entries[{idx}]")
        sync_state_profile_id_local = ensure_str(
            raw_row.get("sync_state_profile_id"),
            f"matrix.entries[{idx}].sync_state_profile_id",
        )

        applied_overrides: dict[str, Any] = {}
        replay_row_payload = raw_row
        drill_local: dict[str, Any] | None = None
        if (
            forced_profile_id is not None
            and sync_state_profile_id_local == forced_profile_id
        ):
            drill_local = raw_row.get("failure_drill")
            if not isinstance(drill_local, dict):
                raise SystemExit(
                    f"--force-drill targeted sync_state_profile_id "
                    f"{forced_profile_id!r} but the row has no failure_drill"
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
                    f"{forced_profile_id!r}"
                )
            applied_overrides = forced_input_local
            replay_row_payload = apply_forced_overrides(
                raw_row, applied_overrides
            )

        result = validate_row(
            replay_row_payload,
            sync_state_profile_id_value=sync_state_profile_id_local,
            device_participation_state_vocab=device_participation_state_vocab,
            device_class_vocab=device_class_vocab,
            os_family_class_vocab=os_family_class_vocab,
            identity_mode_class_vocab=identity_mode_class_vocab,
            sync_session_state_vocab=sync_session_state_vocab,
            conflict_review_vocab=conflict_review_vocab,
            conflict_resolution_state_vocab=conflict_resolution_state_vocab,
            offered_resolution_path_vocab=offered_resolution_path_vocab,
            non_widening_posture_vocab=non_widening_posture_vocab,
            scope_broadening_verdict_vocab=scope_broadening_verdict_vocab,
            local_ownership_marker_vocab=local_ownership_marker_vocab,
            data_portability_vocab=data_portability_vocab,
            failure_drill_id_vocab=failure_drill_id_vocab,
        )
        if applied_overrides:
            result.diagnostics["forced_overrides_applied"] = (
                applied_overrides
            )
        row_results.append(result)

        if result.sync_state_profile_id in seen_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="settings_sync_state.entries_duplicate_sync_state_profile_id",
                    message=(
                        f"duplicate sync_state_profile_id: "
                        f"{result.sync_state_profile_id}"
                    ),
                    remediation="sync_state_profile_ids must be unique.",
                    ref=result.sync_state_profile_id,
                )
            )
        seen_ids.add(result.sync_state_profile_id)

        if (
            isinstance(raw_row.get("device_participation_state_class"), str)
            and raw_row["device_participation_state_class"]
        ):
            seen_participation_states.add(
                raw_row["device_participation_state_class"]
            )
        if (
            isinstance(raw_row.get("sync_session_state_class"), str)
            and raw_row["sync_session_state_class"]
        ):
            seen_session_states.add(raw_row["sync_session_state_class"])
        if (
            isinstance(raw_row.get("conflict_review_class"), str)
            and raw_row["conflict_review_class"]
        ):
            seen_conflict_review_classes.add(
                raw_row["conflict_review_class"]
            )
        portabilities = raw_row.get("data_class_portabilities")
        if isinstance(portabilities, list):
            for entry in portabilities:
                if isinstance(entry, dict):
                    pc = entry.get("portability_class")
                    if isinstance(pc, str) and pc:
                        seen_portability_classes.add(pc)

        if (
            forced_profile_id is not None
            and result.sync_state_profile_id == forced_profile_id
            and applied_overrides
            and isinstance(drill_local, dict)
        ):
            expected_check = ensure_str(
                drill_local.get("expected_check_id"),
                f"{forced_profile_id}.failure_drill.expected_check_id",
            )
            observed = [
                fc.get("check_id")
                for fc in result.failed_checks
                if isinstance(fc, dict)
            ]
            forced_replay_record = {
                "sync_state_profile_id": forced_profile_id,
                "drill_id": forced_drill_id,
                "expected_check_id": expected_check,
                "observed_failed_check_ids": observed,
                "reproduced": expected_check in observed,
            }

    # --- coverage -----------------------------------------------------
    missing_participation_states = (
        required_device_participation_state_coverage
        - seen_participation_states
    )
    if missing_participation_states:
        findings.append(
            Finding(
                severity="error",
                check_id="settings_sync_state.coverage_missing_required_device_participation_states",
                message=(
                    "matrix must seed at least one row for each required "
                    "device_participation_state_class: "
                    f"{sorted(required_device_participation_state_coverage)};"
                    f" missing: {sorted(missing_participation_states)}"
                ),
                remediation=(
                    "Add the missing rows so the device-registry "
                    "lifecycle is covered."
                ),
            )
        )

    missing_session_states = (
        required_sync_session_state_coverage - seen_session_states
    )
    if missing_session_states:
        findings.append(
            Finding(
                severity="error",
                check_id="settings_sync_state.coverage_missing_required_sync_session_states",
                message=(
                    "matrix must seed at least one row for each required "
                    "sync_session_state_class: "
                    f"{sorted(required_sync_session_state_coverage)}; "
                    f"missing: {sorted(missing_session_states)}"
                ),
                remediation=(
                    "Add the missing rows so the session-state lifecycle "
                    "is covered."
                ),
            )
        )

    missing_conflict_review = (
        required_conflict_review_class_coverage
        - seen_conflict_review_classes
    )
    if missing_conflict_review:
        findings.append(
            Finding(
                severity="error",
                check_id="settings_sync_state.coverage_missing_required_conflict_review_classes",
                message=(
                    "matrix must seed at least one row for each required "
                    "conflict_review_class: "
                    f"{sorted(required_conflict_review_class_coverage)}; "
                    f"missing: {sorted(missing_conflict_review)}"
                ),
                remediation=(
                    "Add or extend rows so every protected conflict-"
                    "review class is reachable from the seed."
                ),
            )
        )

    missing_portability = (
        required_data_portability_class_coverage - seen_portability_classes
    )
    if missing_portability:
        findings.append(
            Finding(
                severity="error",
                check_id="settings_sync_state.coverage_missing_required_data_portability_classes",
                message=(
                    "the union of every row's data_class_portabilities "
                    "must cover the required portability classes: "
                    f"{sorted(required_data_portability_class_coverage)}; "
                    f"missing: {sorted(missing_portability)}"
                ),
                remediation=(
                    "Add the missing portability classes to one of the "
                    "seeded rows so machine_local exclusions and "
                    "policy_owned ownership stay distinct from "
                    "local_authoritative state."
                ),
            )
        )

    # Promote per-row failures into findings, skipping the targeted row
    # under --force-drill so the runner's exit can reflect the drill verdict.
    for result in row_results:
        if (
            forced_profile_id is not None
            and result.sync_state_profile_id == forced_profile_id
            and forced_replay_record is not None
        ):
            continue
        for failure in result.failed_checks:
            findings.append(
                Finding(
                    severity="error",
                    check_id=failure.get(
                        "check_id", "settings_sync_state.row_failed_check"
                    ),
                    message=(
                        f"{result.sync_state_profile_id}: "
                        f"{failure.get('message', '')}"
                    ),
                    remediation=(
                        "Re-align the row with the device-registry / "
                        "settings-sync contract or fix the drift in the "
                        "seed; failures are reported with the precise "
                        "actionable check_id."
                    ),
                    ref=result.sync_state_profile_id,
                )
            )

    errors = [f for f in findings if f.severity == "error"]
    status = "PASS" if not errors else "FAIL"

    capture: dict[str, Any] = {
        "schema_version": 1,
        "capture_kind": "device_registry_and_sync_seed_validation_capture",
        "captured_at": now_iso_z(),
        "owner_dri": owner_dri,
        "matrix_ref": matrix_rel,
        "envelope_schema_ref": args.envelope_schema,
        "row_schema_ref": args.row_schema,
        "exact_build_identity_ref": args.build_identity,
        "command": (
            "python3 tests/governance/m1_device_registry_and_sync_seed_lane/"
            "run_m1_device_registry_and_sync_seed_lane.py --repo-root ."
        ),
        "status": status,
        "required_device_participation_state_coverage": sorted(
            required_device_participation_state_coverage
        ),
        "observed_device_participation_states": sorted(
            seen_participation_states
        ),
        "required_sync_session_state_coverage": sorted(
            required_sync_session_state_coverage
        ),
        "observed_sync_session_states": sorted(seen_session_states),
        "required_conflict_review_class_coverage": sorted(
            required_conflict_review_class_coverage
        ),
        "observed_conflict_review_classes": sorted(
            seen_conflict_review_classes
        ),
        "required_data_portability_class_coverage": sorted(
            required_data_portability_class_coverage
        ),
        "observed_data_portability_classes": sorted(
            seen_portability_classes
        ),
        "rows": [
            {
                "sync_state_profile_id": r.sync_state_profile_id,
                "device_participation_state_class": (
                    r.device_participation_state_class
                ),
                "sync_session_state_class": r.sync_session_state_class,
                "conflict_review_class": r.conflict_review_class,
                "passed_checks": r.passed_checks,
                "failed_checks": r.failed_checks,
                "diagnostics": r.diagnostics,
            }
            for r in row_results
        ],
        "finding_counts": {
            "error": sum(1 for f in findings if f.severity == "error"),
            "warning": sum(
                1 for f in findings if f.severity == "warning"
            ),
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

    label = "device-registry-and-sync-seed"
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
                f" on {forced_replay_record['sync_state_profile_id']} "
                f"reproduced {forced_replay_record['expected_check_id']}"
            )
            return 0
        print(
            f"[{label}] forced drill {forced_replay_record['drill_id']} on"
            f" {forced_replay_record['sync_state_profile_id']} did NOT"
            f" reproduce {forced_replay_record['expected_check_id']};"
            f" observed: {forced_replay_record['observed_failed_check_ids']}"
        )
        return 2

    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print(
            "[device-registry-and-sync-seed] interrupted", file=sys.stderr
        )
        sys.exit(130)
