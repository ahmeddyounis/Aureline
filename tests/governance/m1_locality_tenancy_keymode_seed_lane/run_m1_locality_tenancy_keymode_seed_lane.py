#!/usr/bin/env python3
"""Unattended M1 locality/tenancy/key-mode vocabulary seed validation lane.

Replays every row in ``artifacts/governance/locality_examples.yaml``
against:

- ``schemas/governance/m1_locality_tenancy_keymode_seed.schema.json``
  — the envelope schema (vocabularies, required coverage, named
  consumers);
- ``schemas/governance/locality_tenancy_keymode.schema.json`` — the
  row vocabulary; and
- the canonical landing page at
  ``docs/governance/m1_locality_tenancy_keymode_vocabulary.md`` plus
  the upstream internal boundary manifest at
  ``artifacts/governance/m1_open_local_capability_matrix.yaml`` so
  the seed cannot quietly outlive its upstream.

Per-row assertions (every row):

- ``record_kind`` is ``locality_tenancy_keymode_row_record`` and
  ``locality_tenancy_keymode_row_schema_version`` is ``1``.
- ``locality_tenancy_keymode_profile_id`` is unique, non-empty, and
  matches the row schema's pattern.
- ``locality_class``, ``tenancy_scope_class``,
  ``key_storage_mode_class``, ``local_safe_fallback_class``,
  ``data_residency_disclosure_class``, ``surface_family_class``, and
  every member of ``truth_badge_classes`` /
  ``diagnostic_surface_classes`` are in their closed vocabularies.
- ``owner_dri`` is a non-empty ``@handle``.
- ``failure_drill`` is a non-null object whose ``drill_id`` is in
  ``failure_drill_id_vocabulary``, whose ``forced_input`` declares at
  least one drift, and whose ``expected_check_id`` and
  ``actionable_next_action`` are non-empty.
- Structural invariants:
    * ``local_only`` rows MUST publish ``tenancy_scope_class`` in
      {``not_applicable_local_only``, ``single_user_local``},
      ``key_storage_mode_class`` in
      {``not_applicable_local_only``, ``local_storage_only``,
      ``os_keychain_backed``},
      ``data_residency_disclosure_class`` =
      ``residency_local_device_only``, and
      ``local_safe_fallback_class`` =
      ``local_safe_fallback_present``;
    * ``managed_control_plane_bearing`` rows MUST publish
      ``tenancy_scope_class`` in {``single_user_managed_tenant``,
      ``org_tenant``, ``multi_tenant_isolated``,
      ``multi_tenant_shared``, ``unknown_tenancy``},
      ``key_storage_mode_class`` in {``byok_user_managed``,
      ``provider_managed_key``, ``managed_service_kms``,
      ``unknown_key_mode``},
      ``data_residency_disclosure_class`` in
      {``residency_managed_tenant_documented_region``,
      ``residency_provider_default``, ``residency_unknown``}, and
      MUST NOT publish ``local_safe_fallback_unavailable`` (managed
      rows in M1 keep the local-safe fallback explicit and optional);
    * ``unknown_locality`` rows MUST publish ``unknown_tenancy``,
      ``unknown_key_mode``, and ``residency_unknown``;
    * ``provider_linked`` rows MUST NOT publish
      ``not_applicable_local_only`` on ``tenancy_scope_class`` or
      ``key_storage_mode_class``.

Envelope assertions:

- ``schema_version = 1``, ``matrix_id =
  m1_locality_tenancy_keymode_seed``, ``status`` non-empty,
  ``owner_dri`` is a ``@handle``.
- ``overview_page``, ``upstream_boundary_manifest_ref``,
  ``row_schema_ref``, ``build_identity_ref``, ``validation_lane_ref``
  resolve on disk.
- Closed envelope vocabularies match the row schema $defs verbatim.
- Required coverage lists (locality classes, tenancy classes,
  key-mode classes, diagnostic surfaces) are satisfied by the union
  of seeded rows.
- Every ``named_runtime_consumers[].consumer_ref`` resolves on disk,
  the consumer_class is from the closed vocabulary, and
  ``consumed_fields`` is non-empty.

``--force-drill <locality_tenancy_keymode_profile_id>:<drill_id>``
replays the named drill on the named row and exits 0 only when the
runner reproduces the declared ``expected_check_id``. Drift in the
unforced rows still fails the lane.

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


DEFAULT_MATRIX_REL = "artifacts/governance/locality_examples.yaml"
DEFAULT_ENVELOPE_SCHEMA_REL = (
    "schemas/governance/m1_locality_tenancy_keymode_seed.schema.json"
)
DEFAULT_ROW_SCHEMA_REL = (
    "schemas/governance/locality_tenancy_keymode.schema.json"
)
DEFAULT_BUILD_IDENTITY_REL = "artifacts/build/build_identity.json"
DEFAULT_REPORT_REL = (
    "artifacts/milestones/m1/captures/"
    "locality_tenancy_keymode_vocabulary_validation_capture.json"
)

EXPECTED_RECORD_KIND = "locality_tenancy_keymode_row_record"
EXPECTED_ROW_SCHEMA_VERSION = 1
EXPECTED_MATRIX_ID = "m1_locality_tenancy_keymode_seed"
EXPECTED_OVERVIEW_PAGE = (
    "docs/governance/m1_locality_tenancy_keymode_vocabulary.md"
)
EXPECTED_ROW_SCHEMA_REF = (
    "schemas/governance/locality_tenancy_keymode.schema.json"
)

PROFILE_ID_PATTERN = re.compile(r"^[a-z0-9]+(?:[._-][a-z0-9]+)*$")
OWNER_DRI_PATTERN = re.compile(r"^@[a-zA-Z0-9_-]+$")

LOCAL_ONLY_TENANCY_ELIGIBLE = frozenset(
    {"not_applicable_local_only", "single_user_local"}
)
LOCAL_ONLY_KEY_MODE_ELIGIBLE = frozenset(
    {"not_applicable_local_only", "local_storage_only", "os_keychain_backed"}
)
MANAGED_TENANCY_ELIGIBLE = frozenset(
    {
        "single_user_managed_tenant",
        "org_tenant",
        "multi_tenant_isolated",
        "multi_tenant_shared",
        "unknown_tenancy",
    }
)
MANAGED_KEY_MODE_ELIGIBLE = frozenset(
    {
        "byok_user_managed",
        "provider_managed_key",
        "managed_service_kms",
        "unknown_key_mode",
    }
)
MANAGED_RESIDENCY_ELIGIBLE = frozenset(
    {
        "residency_managed_tenant_documented_region",
        "residency_provider_default",
        "residency_unknown",
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
    locality_tenancy_keymode_profile_id: str
    locality_class: str
    tenancy_scope_class: str
    key_storage_mode_class: str
    local_safe_fallback_class: str
    data_residency_disclosure_class: str
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
            "'<locality_tenancy_keymode_profile_id>:<drill_id>'. The "
            "runner exits 0 only when the row's failure drill reproduces "
            "the exact expected_check_id."
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

    for key, axis in (
        ("rewrite_locality_class", "locality_class"),
        ("rewrite_tenancy_scope_class", "tenancy_scope_class"),
        ("rewrite_key_storage_mode_class", "key_storage_mode_class"),
        ("rewrite_local_safe_fallback_class", "local_safe_fallback_class"),
        (
            "rewrite_data_residency_disclosure_class",
            "data_residency_disclosure_class",
        ),
        ("rewrite_surface_family_class", "surface_family_class"),
    ):
        if key in forced_overrides:
            row[axis] = forced_overrides[key]

    if forced_overrides.get("clear_local_core_continuity"):
        row["local_core_continuity"] = ""

    if forced_overrides.get("clear_absence_narrows_to"):
        row["absence_narrows_to"] = ""

    return row


def validate_row(
    row: dict[str, Any],
    *,
    locality_tenancy_keymode_profile_id_value: str,
    locality_class_vocab: set[str],
    tenancy_scope_class_vocab: set[str],
    key_storage_mode_class_vocab: set[str],
    local_safe_fallback_class_vocab: set[str],
    data_residency_disclosure_class_vocab: set[str],
    truth_badge_class_vocab: set[str],
    diagnostic_surface_class_vocab: set[str],
    surface_family_class_vocab: set[str],
    failure_drill_id_vocab: set[str],
) -> RowResult:
    profile_id = ensure_str(
        row.get("locality_tenancy_keymode_profile_id"),
        (
            f"{locality_tenancy_keymode_profile_id_value}."
            "locality_tenancy_keymode_profile_id"
        ),
    )
    locality_class = (
        row.get("locality_class")
        if isinstance(row.get("locality_class"), str)
        else ""
    )
    tenancy_scope_class = (
        row.get("tenancy_scope_class")
        if isinstance(row.get("tenancy_scope_class"), str)
        else ""
    )
    key_storage_mode_class = (
        row.get("key_storage_mode_class")
        if isinstance(row.get("key_storage_mode_class"), str)
        else ""
    )
    local_safe_fallback_class = (
        row.get("local_safe_fallback_class")
        if isinstance(row.get("local_safe_fallback_class"), str)
        else ""
    )
    data_residency_disclosure_class = (
        row.get("data_residency_disclosure_class")
        if isinstance(row.get("data_residency_disclosure_class"), str)
        else ""
    )

    result = RowResult(
        locality_tenancy_keymode_profile_id=profile_id,
        locality_class=locality_class,
        tenancy_scope_class=tenancy_scope_class,
        key_storage_mode_class=key_storage_mode_class,
        local_safe_fallback_class=local_safe_fallback_class,
        data_residency_disclosure_class=data_residency_disclosure_class,
    )

    # --- discriminator + version pin ----------------------------------
    if row.get("record_kind") != EXPECTED_RECORD_KIND:
        fail(
            result,
            "locality_tenancy_keymode.record_kind_wrong",
            (
                f"record_kind must be {EXPECTED_RECORD_KIND!r}; got "
                f"{row.get('record_kind')!r}"
            ),
        )
    if (
        row.get("locality_tenancy_keymode_row_schema_version")
        != EXPECTED_ROW_SCHEMA_VERSION
    ):
        fail(
            result,
            "locality_tenancy_keymode.schema_version_wrong",
            (
                "locality_tenancy_keymode_row_schema_version must be "
                f"{EXPECTED_ROW_SCHEMA_VERSION}; got "
                f"{row.get('locality_tenancy_keymode_row_schema_version')!r}"
            ),
        )

    # --- locality_tenancy_keymode_profile_id pattern ------------------
    if not PROFILE_ID_PATTERN.match(profile_id):
        fail(
            result,
            "locality_tenancy_keymode.profile_id_pattern_invalid",
            (
                f"locality_tenancy_keymode_profile_id {profile_id!r} does "
                f"not match {PROFILE_ID_PATTERN.pattern!r}"
            ),
        )

    # --- closed-vocabulary scalar axes --------------------------------
    closed_scalar_axes = [
        ("surface_family_class", surface_family_class_vocab),
        ("locality_class", locality_class_vocab),
        ("tenancy_scope_class", tenancy_scope_class_vocab),
        ("key_storage_mode_class", key_storage_mode_class_vocab),
        ("local_safe_fallback_class", local_safe_fallback_class_vocab),
        (
            "data_residency_disclosure_class",
            data_residency_disclosure_class_vocab,
        ),
    ]
    for field_name, vocab in closed_scalar_axes:
        value = row.get(field_name)
        if not isinstance(value, str) or not value.strip():
            fail(
                result,
                f"locality_tenancy_keymode.{field_name}_required",
                (
                    f"{field_name} must be a non-empty member of the "
                    "closed vocabulary"
                ),
            )
        elif value not in vocab:
            fail(
                result,
                f"locality_tenancy_keymode.{field_name}_unknown",
                (
                    f"{field_name} {value!r} is not in the row schema's "
                    f"{field_name} enum"
                ),
            )

    # --- truth_badge_classes ------------------------------------------
    badges = row.get("truth_badge_classes")
    if not isinstance(badges, list) or not badges:
        fail(
            result,
            "locality_tenancy_keymode.truth_badge_classes_required",
            "truth_badge_classes must be a non-empty list",
        )
    else:
        for b in badges:
            if not isinstance(b, str) or b not in truth_badge_class_vocab:
                fail(
                    result,
                    "locality_tenancy_keymode.truth_badge_class_unknown",
                    (
                        f"truth_badge_classes entry {b!r} is not in the "
                        "row schema's truth_badge_class enum"
                    ),
                )

    # --- diagnostic_surface_classes -----------------------------------
    surfaces = row.get("diagnostic_surface_classes")
    if not isinstance(surfaces, list) or not surfaces:
        fail(
            result,
            "locality_tenancy_keymode.diagnostic_surface_classes_required",
            "diagnostic_surface_classes must be a non-empty list",
        )
    else:
        for s in surfaces:
            if not isinstance(s, str) or s not in diagnostic_surface_class_vocab:
                fail(
                    result,
                    "locality_tenancy_keymode.diagnostic_surface_class_unknown",
                    (
                        f"diagnostic_surface_classes entry {s!r} is not in "
                        "the row schema's diagnostic_surface_class enum"
                    ),
                )

    # --- local_core_continuity / absence_narrows_to -------------------
    local_core = row.get("local_core_continuity")
    if not isinstance(local_core, str) or not local_core.strip():
        fail(
            result,
            "locality_tenancy_keymode.local_core_continuity_required",
            (
                "local_core_continuity must be a non-empty reviewer-facing "
                "description; managed and provider-linked rows MUST "
                "document the local-safe fallback verbatim"
            ),
        )
    absence = row.get("absence_narrows_to")
    if not isinstance(absence, str) or not absence.strip():
        fail(
            result,
            "locality_tenancy_keymode.absence_narrows_to_required",
            (
                "absence_narrows_to must be a non-empty reviewer-facing "
                "description; absence narrows a claim rather than removing "
                "the surface"
            ),
        )

    # --- owner_dri -----------------------------------------------------
    owner_dri = row.get("owner_dri")
    if not isinstance(owner_dri, str) or not owner_dri.strip():
        fail(
            result,
            "locality_tenancy_keymode.owner_dri_required",
            "owner_dri must be a non-empty @handle",
        )
    elif not OWNER_DRI_PATTERN.match(owner_dri):
        fail(
            result,
            "locality_tenancy_keymode.owner_dri_pattern_invalid",
            f"owner_dri {owner_dri!r} must match {OWNER_DRI_PATTERN.pattern!r}",
        )

    # --- structural invariants ----------------------------------------
    if locality_class == "local_only":
        if (
            tenancy_scope_class
            and tenancy_scope_class not in LOCAL_ONLY_TENANCY_ELIGIBLE
        ):
            fail(
                result,
                "locality_tenancy_keymode.local_only_invariant_relaxed_to_managed",
                (
                    "local_only rows MUST publish tenancy_scope_class in "
                    f"{sorted(LOCAL_ONLY_TENANCY_ELIGIBLE)}; got "
                    f"{tenancy_scope_class!r}. The local-only floor "
                    "cannot widen into a managed/control-plane claim "
                    "without an explicit decision row."
                ),
            )
        if (
            key_storage_mode_class
            and key_storage_mode_class not in LOCAL_ONLY_KEY_MODE_ELIGIBLE
        ):
            fail(
                result,
                "locality_tenancy_keymode.local_only_invariant_relaxed_to_managed",
                (
                    "local_only rows MUST publish key_storage_mode_class in "
                    f"{sorted(LOCAL_ONLY_KEY_MODE_ELIGIBLE)}; got "
                    f"{key_storage_mode_class!r}"
                ),
            )
        if (
            data_residency_disclosure_class
            and data_residency_disclosure_class
            != "residency_local_device_only"
        ):
            fail(
                result,
                "locality_tenancy_keymode.local_only_invariant_relaxed_to_managed",
                (
                    "local_only rows MUST publish "
                    "data_residency_disclosure_class = "
                    "'residency_local_device_only'; got "
                    f"{data_residency_disclosure_class!r}"
                ),
            )
        if (
            local_safe_fallback_class
            and local_safe_fallback_class != "local_safe_fallback_present"
        ):
            fail(
                result,
                "locality_tenancy_keymode.local_only_invariant_relaxed_to_managed",
                (
                    "local_only rows MUST publish "
                    "local_safe_fallback_class = "
                    "'local_safe_fallback_present'; got "
                    f"{local_safe_fallback_class!r}"
                ),
            )

    if locality_class == "remote_target":
        if data_residency_disclosure_class == "residency_local_device_only":
            fail(
                result,
                "locality_tenancy_keymode.remote_target_residency_disclosure_widened",
                (
                    "remote_target rows MUST NOT publish "
                    "data_residency_disclosure_class = "
                    "'residency_local_device_only' (a remote target moves "
                    "user data off the local device by definition)"
                ),
            )
        if (
            data_residency_disclosure_class
            and data_residency_disclosure_class
            not in {
                "residency_user_owned_remote_target",
                "residency_unknown",
            }
        ):
            fail(
                result,
                "locality_tenancy_keymode.remote_target_residency_disclosure_widened",
                (
                    "remote_target rows MUST publish "
                    "data_residency_disclosure_class in "
                    "{residency_user_owned_remote_target, residency_unknown}; "
                    f"got {data_residency_disclosure_class!r}"
                ),
            )

    if locality_class == "provider_linked":
        if tenancy_scope_class == "not_applicable_local_only":
            fail(
                result,
                "locality_tenancy_keymode.provider_linked_tenancy_class_relaxed_to_local_only_sentinel",
                (
                    "provider_linked rows MUST NOT publish "
                    "tenancy_scope_class = 'not_applicable_local_only'; "
                    "provider-linked surfaces name a tenancy class "
                    "(including unknown_tenancy) so the local-only "
                    "sentinel cannot be silently inherited"
                ),
            )
        if key_storage_mode_class == "not_applicable_local_only":
            fail(
                result,
                "locality_tenancy_keymode.provider_linked_key_storage_mode_class_relaxed_to_local_only_sentinel",
                (
                    "provider_linked rows MUST NOT publish "
                    "key_storage_mode_class = 'not_applicable_local_only'; "
                    "provider-linked surfaces name a key-mode class "
                    "(including unknown_key_mode)"
                ),
            )

    if locality_class == "managed_control_plane_bearing":
        if (
            tenancy_scope_class
            and tenancy_scope_class not in MANAGED_TENANCY_ELIGIBLE
        ):
            fail(
                result,
                "locality_tenancy_keymode.managed_control_plane_uncertainty_token_must_not_be_silently_certified",
                (
                    "managed_control_plane_bearing rows MUST publish "
                    "tenancy_scope_class in "
                    f"{sorted(MANAGED_TENANCY_ELIGIBLE)}; got "
                    f"{tenancy_scope_class!r}"
                ),
            )
        if (
            key_storage_mode_class
            and key_storage_mode_class not in MANAGED_KEY_MODE_ELIGIBLE
        ):
            fail(
                result,
                "locality_tenancy_keymode.managed_control_plane_uncertainty_token_must_not_be_silently_certified",
                (
                    "managed_control_plane_bearing rows MUST publish "
                    "key_storage_mode_class in "
                    f"{sorted(MANAGED_KEY_MODE_ELIGIBLE)}; got "
                    f"{key_storage_mode_class!r}"
                ),
            )
        if (
            data_residency_disclosure_class
            and data_residency_disclosure_class
            not in MANAGED_RESIDENCY_ELIGIBLE
        ):
            fail(
                result,
                "locality_tenancy_keymode.managed_control_plane_uncertainty_token_must_not_be_silently_certified",
                (
                    "managed_control_plane_bearing rows MUST publish "
                    "data_residency_disclosure_class in "
                    f"{sorted(MANAGED_RESIDENCY_ELIGIBLE)}; got "
                    f"{data_residency_disclosure_class!r}"
                ),
            )
        if local_safe_fallback_class == "local_safe_fallback_unavailable":
            fail(
                result,
                "locality_tenancy_keymode.managed_control_plane_local_safe_fallback_must_not_be_unavailable",
                (
                    "managed_control_plane_bearing rows MUST NOT publish "
                    "local_safe_fallback_class = "
                    "'local_safe_fallback_unavailable'; a managed surface "
                    "in M1 keeps the local-safe fallback explicit so the "
                    "surface stays optional and the local floor is not "
                    "silently removed"
                ),
            )
        # Uncertainty-token consistency. A managed/control-plane-bearing
        # row that declares residency_unknown has signalled that the
        # product cannot truthfully name where data rests; carrying a
        # certified tenancy or key-mode claim alongside residency_unknown
        # would let a hosted-control-plane surface render a tenant-region
        # badge the seed cannot back. Force full vocabulary uncertainty
        # whenever residency is unknown.
        if data_residency_disclosure_class == "residency_unknown" and (
            (tenancy_scope_class and tenancy_scope_class != "unknown_tenancy")
            or (
                key_storage_mode_class
                and key_storage_mode_class != "unknown_key_mode"
            )
        ):
            fail(
                result,
                "locality_tenancy_keymode.managed_control_plane_uncertainty_token_must_not_be_silently_certified",
                (
                    "managed_control_plane_bearing rows whose "
                    "data_residency_disclosure_class is 'residency_unknown' "
                    "MUST publish tenancy_scope_class = 'unknown_tenancy' "
                    "AND key_storage_mode_class = 'unknown_key_mode'. "
                    "Mixing residency uncertainty with a certified tenancy "
                    "or key-mode claim is a false-certainty regression."
                ),
            )

    if locality_class == "unknown_locality":
        if tenancy_scope_class and tenancy_scope_class != "unknown_tenancy":
            fail(
                result,
                "locality_tenancy_keymode.unknown_locality_tenancy_scope_must_be_unknown_tenancy",
                (
                    "unknown_locality rows MUST publish "
                    "tenancy_scope_class = 'unknown_tenancy'; got "
                    f"{tenancy_scope_class!r}"
                ),
            )
        if (
            key_storage_mode_class
            and key_storage_mode_class != "unknown_key_mode"
        ):
            fail(
                result,
                "locality_tenancy_keymode.unknown_locality_key_storage_mode_must_be_unknown_key_mode",
                (
                    "unknown_locality rows MUST publish "
                    "key_storage_mode_class = 'unknown_key_mode'; got "
                    f"{key_storage_mode_class!r}"
                ),
            )
        if (
            data_residency_disclosure_class
            and data_residency_disclosure_class != "residency_unknown"
        ):
            fail(
                result,
                "locality_tenancy_keymode.unknown_locality_residency_disclosure_must_be_unknown",
                (
                    "unknown_locality rows MUST publish "
                    "data_residency_disclosure_class = "
                    "'residency_unknown'; got "
                    f"{data_residency_disclosure_class!r}"
                ),
            )

    # --- failure_drill -------------------------------------------------
    drill = row.get("failure_drill")
    if not isinstance(drill, dict):
        fail(
            result,
            "locality_tenancy_keymode.failure_drill_required",
            "failure_drill must be a non-null object on every row",
        )
    else:
        drill_id = drill.get("drill_id")
        if not isinstance(drill_id, str) or not drill_id.strip():
            fail(
                result,
                "locality_tenancy_keymode.failure_drill_drill_id_required",
                "failure_drill.drill_id must be a non-empty string",
            )
        elif drill_id not in failure_drill_id_vocab:
            fail(
                result,
                "locality_tenancy_keymode.failure_drill_drill_id_unknown",
                (
                    f"failure_drill.drill_id {drill_id!r} is not in "
                    "failure_drill_id_vocabulary"
                ),
            )
        forced_input = drill.get("forced_input")
        if not isinstance(forced_input, dict) or not forced_input:
            fail(
                result,
                "locality_tenancy_keymode.failure_drill_forced_input_empty",
                "failure_drill.forced_input must declare at least one drift",
            )
        expected_check = drill.get("expected_check_id")
        if (
            not isinstance(expected_check, str)
            or not expected_check.strip()
        ):
            fail(
                result,
                "locality_tenancy_keymode.failure_drill_expected_check_id_required",
                "failure_drill.expected_check_id must be non-empty",
            )
        actionable = drill.get("actionable_next_action")
        if not isinstance(actionable, str) or not actionable.strip():
            fail(
                result,
                "locality_tenancy_keymode.failure_drill_actionable_next_action_required",
                "failure_drill.actionable_next_action must be non-empty",
            )

    result.diagnostics.update(
        {
            "locality_tenancy_keymode_profile_id": profile_id,
            "surface_family_class": row.get("surface_family_class"),
            "locality_class": locality_class,
            "tenancy_scope_class": tenancy_scope_class,
            "key_storage_mode_class": key_storage_mode_class,
            "local_safe_fallback_class": local_safe_fallback_class,
            "data_residency_disclosure_class": data_residency_disclosure_class,
            "truth_badge_classes": row.get("truth_badge_classes"),
            "diagnostic_surface_classes": row.get(
                "diagnostic_surface_classes"
            ),
        }
    )

    if not result.failed_checks:
        pass_(result, f"row {profile_id} passes")

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
                check_id="locality_tenancy_keymode.envelope_schema_version_wrong",
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
                check_id="locality_tenancy_keymode.envelope_matrix_id_wrong",
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
                check_id="locality_tenancy_keymode.envelope_owner_dri_pattern_invalid",
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
    if overview_page != EXPECTED_OVERVIEW_PAGE:
        findings.append(
            Finding(
                severity="error",
                check_id="locality_tenancy_keymode.envelope_overview_page_wrong",
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
                check_id="locality_tenancy_keymode.envelope_overview_page_missing",
                message=f"overview_page does not exist: {overview_page}",
                remediation="Create the reviewer landing page or fix the path.",
                ref=overview_page,
            )
        )

    upstream_boundary_manifest_ref = ensure_str(
        matrix.get("upstream_boundary_manifest_ref"),
        "matrix.upstream_boundary_manifest_ref",
    )
    if not artifact_ref_exists(repo_root, upstream_boundary_manifest_ref):
        findings.append(
            Finding(
                severity="error",
                check_id="locality_tenancy_keymode.envelope_upstream_boundary_manifest_ref_missing",
                message=(
                    "upstream_boundary_manifest_ref does not resolve: "
                    f"{upstream_boundary_manifest_ref}"
                ),
                remediation=(
                    "Fix the path or land the upstream internal boundary "
                    "manifest."
                ),
                ref=upstream_boundary_manifest_ref,
            )
        )

    for key in ("row_schema_ref", "build_identity_ref"):
        ref = ensure_str(matrix.get(key), f"matrix.{key}")
        if not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"locality_tenancy_keymode.envelope_{key}_missing",
                    message=f"{key} does not resolve: {ref}",
                    remediation=(
                        "Fix the path or land the referenced artifact."
                    ),
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
                check_id="locality_tenancy_keymode.envelope_row_schema_ref_wrong",
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
                check_id="locality_tenancy_keymode.envelope_validation_lane_ref_missing",
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

    locality_class_vocab = load_vocab("locality_class_vocabulary")
    tenancy_scope_class_vocab = load_vocab("tenancy_scope_class_vocabulary")
    key_storage_mode_class_vocab = load_vocab(
        "key_storage_mode_class_vocabulary"
    )
    local_safe_fallback_class_vocab = load_vocab(
        "local_safe_fallback_class_vocabulary"
    )
    data_residency_disclosure_class_vocab = load_vocab(
        "data_residency_disclosure_class_vocabulary"
    )
    truth_badge_class_vocab = load_vocab("truth_badge_class_vocabulary")
    diagnostic_surface_class_vocab = load_vocab(
        "diagnostic_surface_class_vocabulary"
    )
    surface_family_class_vocab = load_vocab(
        "surface_family_class_vocabulary"
    )
    failure_drill_id_vocab = load_vocab("failure_drill_id_vocabulary")
    required_locality_class_coverage = load_vocab(
        "required_locality_class_coverage"
    )
    required_tenancy_scope_class_coverage = load_vocab(
        "required_tenancy_scope_class_coverage"
    )
    required_key_storage_mode_class_coverage = load_vocab(
        "required_key_storage_mode_class_coverage"
    )
    required_diagnostic_surface_class_coverage = load_vocab(
        "required_diagnostic_surface_class_coverage"
    )

    # Closed-vocabulary agreement with the row schema $defs.
    vocab_agreements = [
        ("locality_class_vocabulary", "locality_class", locality_class_vocab),
        (
            "tenancy_scope_class_vocabulary",
            "tenancy_scope_class",
            tenancy_scope_class_vocab,
        ),
        (
            "key_storage_mode_class_vocabulary",
            "key_storage_mode_class",
            key_storage_mode_class_vocab,
        ),
        (
            "local_safe_fallback_class_vocabulary",
            "local_safe_fallback_class",
            local_safe_fallback_class_vocab,
        ),
        (
            "data_residency_disclosure_class_vocabulary",
            "data_residency_disclosure_class",
            data_residency_disclosure_class_vocab,
        ),
        (
            "truth_badge_class_vocabulary",
            "truth_badge_class",
            truth_badge_class_vocab,
        ),
        (
            "diagnostic_surface_class_vocabulary",
            "diagnostic_surface_class",
            diagnostic_surface_class_vocab,
        ),
        (
            "surface_family_class_vocabulary",
            "surface_family_class",
            surface_family_class_vocab,
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
                        "locality_tenancy_keymode.envelope_"
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
                check_id="locality_tenancy_keymode.envelope_named_runtime_consumers_empty",
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
                    check_id="locality_tenancy_keymode.named_runtime_consumer_ref_missing",
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
                    check_id="locality_tenancy_keymode.named_runtime_consumer_consumed_fields_empty",
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
                "'<locality_tenancy_keymode_profile_id>:<drill_id>'"
            )
        forced_profile_id, forced_drill_id = args.force_drill.rsplit(":", 1)
        forced_profile_id = forced_profile_id.strip()
        forced_drill_id = forced_drill_id.strip()

    rows = ensure_list(matrix.get("entries"), "matrix.entries")
    if not rows:
        findings.append(
            Finding(
                severity="error",
                check_id="locality_tenancy_keymode.envelope_entries_empty",
                message="matrix.entries must declare at least one row",
                remediation="Seed at least one locality/tenancy/key-mode profile row.",
            )
        )

    row_results: list[RowResult] = []
    seen_ids: set[str] = set()
    seen_locality_classes: set[str] = set()
    seen_tenancy_scope_classes: set[str] = set()
    seen_key_storage_mode_classes: set[str] = set()
    seen_diagnostic_surfaces: set[str] = set()
    forced_replay_record: dict[str, Any] | None = None

    for idx, raw_row in enumerate(rows):
        raw_row = ensure_dict(raw_row, f"matrix.entries[{idx}]")
        profile_id_local = ensure_str(
            raw_row.get("locality_tenancy_keymode_profile_id"),
            f"matrix.entries[{idx}].locality_tenancy_keymode_profile_id",
        )

        applied_overrides: dict[str, Any] = {}
        replay_row_payload = raw_row
        drill_local: dict[str, Any] | None = None
        if (
            forced_profile_id is not None
            and profile_id_local == forced_profile_id
        ):
            drill_local = raw_row.get("failure_drill")
            if not isinstance(drill_local, dict):
                raise SystemExit(
                    "--force-drill targeted "
                    "locality_tenancy_keymode_profile_id "
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
            locality_tenancy_keymode_profile_id_value=profile_id_local,
            locality_class_vocab=locality_class_vocab,
            tenancy_scope_class_vocab=tenancy_scope_class_vocab,
            key_storage_mode_class_vocab=key_storage_mode_class_vocab,
            local_safe_fallback_class_vocab=local_safe_fallback_class_vocab,
            data_residency_disclosure_class_vocab=data_residency_disclosure_class_vocab,
            truth_badge_class_vocab=truth_badge_class_vocab,
            diagnostic_surface_class_vocab=diagnostic_surface_class_vocab,
            surface_family_class_vocab=surface_family_class_vocab,
            failure_drill_id_vocab=failure_drill_id_vocab,
        )
        if applied_overrides:
            result.diagnostics["forced_overrides_applied"] = applied_overrides
        row_results.append(result)

        if result.locality_tenancy_keymode_profile_id in seen_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="locality_tenancy_keymode.entries_duplicate_locality_tenancy_keymode_profile_id",
                    message=(
                        "duplicate locality_tenancy_keymode_profile_id: "
                        f"{result.locality_tenancy_keymode_profile_id}"
                    ),
                    remediation=(
                        "locality_tenancy_keymode_profile_ids must be unique."
                    ),
                    ref=result.locality_tenancy_keymode_profile_id,
                )
            )
        seen_ids.add(result.locality_tenancy_keymode_profile_id)

        if (
            isinstance(raw_row.get("locality_class"), str)
            and raw_row["locality_class"]
        ):
            seen_locality_classes.add(raw_row["locality_class"])
        if (
            isinstance(raw_row.get("tenancy_scope_class"), str)
            and raw_row["tenancy_scope_class"]
        ):
            seen_tenancy_scope_classes.add(raw_row["tenancy_scope_class"])
        if (
            isinstance(raw_row.get("key_storage_mode_class"), str)
            and raw_row["key_storage_mode_class"]
        ):
            seen_key_storage_mode_classes.add(
                raw_row["key_storage_mode_class"]
            )
        diag_surfaces = raw_row.get("diagnostic_surface_classes")
        if isinstance(diag_surfaces, list):
            for s in diag_surfaces:
                if isinstance(s, str) and s:
                    seen_diagnostic_surfaces.add(s)

        if (
            forced_profile_id is not None
            and result.locality_tenancy_keymode_profile_id == forced_profile_id
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
                "locality_tenancy_keymode_profile_id": forced_profile_id,
                "drill_id": forced_drill_id,
                "expected_check_id": expected_check,
                "observed_failed_check_ids": observed,
                "reproduced": expected_check in observed,
            }

    # --- coverage -----------------------------------------------------
    missing_locality = (
        required_locality_class_coverage - seen_locality_classes
    )
    if missing_locality:
        findings.append(
            Finding(
                severity="error",
                check_id="locality_tenancy_keymode.coverage_missing_required_locality_classes",
                message=(
                    "matrix must seed at least one row for each required "
                    "locality_class: "
                    f"{sorted(required_locality_class_coverage)}; "
                    f"missing: {sorted(missing_locality)}"
                ),
                remediation=(
                    "Add the missing rows so the locality vocabulary is "
                    "covered."
                ),
            )
        )

    missing_tenancy = (
        required_tenancy_scope_class_coverage - seen_tenancy_scope_classes
    )
    if missing_tenancy:
        findings.append(
            Finding(
                severity="error",
                check_id="locality_tenancy_keymode.coverage_missing_required_tenancy_scope_classes",
                message=(
                    "matrix must seed at least one row for each required "
                    "tenancy_scope_class: "
                    f"{sorted(required_tenancy_scope_class_coverage)}; "
                    f"missing: {sorted(missing_tenancy)}"
                ),
                remediation=(
                    "Add or extend rows so the tenancy vocabulary is covered."
                ),
            )
        )

    missing_key_mode = (
        required_key_storage_mode_class_coverage
        - seen_key_storage_mode_classes
    )
    if missing_key_mode:
        findings.append(
            Finding(
                severity="error",
                check_id="locality_tenancy_keymode.coverage_missing_required_key_storage_mode_classes",
                message=(
                    "matrix must seed at least one row for each required "
                    "key_storage_mode_class: "
                    f"{sorted(required_key_storage_mode_class_coverage)}; "
                    f"missing: {sorted(missing_key_mode)}"
                ),
                remediation=(
                    "Add or extend rows so the key-mode vocabulary is "
                    "covered."
                ),
            )
        )

    missing_diag = (
        required_diagnostic_surface_class_coverage
        - seen_diagnostic_surfaces
    )
    if missing_diag:
        findings.append(
            Finding(
                severity="error",
                check_id="locality_tenancy_keymode.coverage_missing_required_diagnostic_surface_classes",
                message=(
                    "the union of every row's diagnostic_surface_classes "
                    "must cover the required diagnostic surfaces: "
                    f"{sorted(required_diagnostic_surface_class_coverage)}; "
                    f"missing: {sorted(missing_diag)}"
                ),
                remediation=(
                    "Add the missing diagnostic surfaces to one of the "
                    "seeded rows so help/about, service-health, support "
                    "exports, release-evidence packs, and CI validation "
                    "are all named consumers of the seed."
                ),
            )
        )

    # Promote per-row failures into findings, skipping the targeted row
    # under --force-drill so the runner's exit can reflect the drill verdict.
    for result in row_results:
        if (
            forced_profile_id is not None
            and result.locality_tenancy_keymode_profile_id == forced_profile_id
            and forced_replay_record is not None
        ):
            continue
        for failure in result.failed_checks:
            findings.append(
                Finding(
                    severity="error",
                    check_id=failure.get(
                        "check_id", "locality_tenancy_keymode.row_failed_check"
                    ),
                    message=(
                        f"{result.locality_tenancy_keymode_profile_id}: "
                        f"{failure.get('message', '')}"
                    ),
                    remediation=(
                        "Re-align the row with the locality/tenancy/key-mode "
                        "contract or fix the drift in the seed; failures "
                        "are reported with the precise actionable check_id."
                    ),
                    ref=result.locality_tenancy_keymode_profile_id,
                )
            )

    errors = [f for f in findings if f.severity == "error"]
    status = "PASS" if not errors else "FAIL"

    capture: dict[str, Any] = {
        "schema_version": 1,
        "capture_kind": "locality_tenancy_keymode_vocabulary_validation_capture",
        "captured_at": now_iso_z(),
        "owner_dri": owner_dri,
        "matrix_ref": matrix_rel,
        "envelope_schema_ref": args.envelope_schema,
        "row_schema_ref": args.row_schema,
        "exact_build_identity_ref": args.build_identity,
        "command": (
            "python3 tests/governance/m1_locality_tenancy_keymode_seed_lane/"
            "run_m1_locality_tenancy_keymode_seed_lane.py --repo-root ."
        ),
        "status": status,
        "required_locality_class_coverage": sorted(
            required_locality_class_coverage
        ),
        "observed_locality_classes": sorted(seen_locality_classes),
        "required_tenancy_scope_class_coverage": sorted(
            required_tenancy_scope_class_coverage
        ),
        "observed_tenancy_scope_classes": sorted(seen_tenancy_scope_classes),
        "required_key_storage_mode_class_coverage": sorted(
            required_key_storage_mode_class_coverage
        ),
        "observed_key_storage_mode_classes": sorted(
            seen_key_storage_mode_classes
        ),
        "required_diagnostic_surface_class_coverage": sorted(
            required_diagnostic_surface_class_coverage
        ),
        "observed_diagnostic_surface_classes": sorted(
            seen_diagnostic_surfaces
        ),
        "rows": [
            {
                "locality_tenancy_keymode_profile_id": r.locality_tenancy_keymode_profile_id,
                "locality_class": r.locality_class,
                "tenancy_scope_class": r.tenancy_scope_class,
                "key_storage_mode_class": r.key_storage_mode_class,
                "local_safe_fallback_class": r.local_safe_fallback_class,
                "data_residency_disclosure_class": r.data_residency_disclosure_class,
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

    label = "locality-tenancy-keymode-seed"
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
                f" on {forced_replay_record['locality_tenancy_keymode_profile_id']} "
                f"reproduced {forced_replay_record['expected_check_id']}"
            )
            return 0
        print(
            f"[{label}] forced drill {forced_replay_record['drill_id']} on"
            f" {forced_replay_record['locality_tenancy_keymode_profile_id']}"
            f" did NOT reproduce {forced_replay_record['expected_check_id']};"
            f" observed: {forced_replay_record['observed_failed_check_ids']}"
        )
        return 2

    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print(
            "[locality-tenancy-keymode-seed] interrupted", file=sys.stderr
        )
        sys.exit(130)
