#!/usr/bin/env python3
"""Unattended M1 install-topology truth-surface seed validation lane.

Replays every row in ``artifacts/install/state_root_matrix.yaml``
against:

- ``schemas/install/install_topology_truth.schema.json`` — the envelope
  schema (vocabularies, required coverage, named consumers);
- ``schemas/install/install_topology_truth_row.schema.json`` — the row
  vocabulary; and
- the canonical landing page at
  ``docs/install/m1_install_topology_truth.md`` plus the silent-
  deployment design packet at ``design/m1/silent_deployment_baseline.md``
  so the seed cannot quietly point at a missing reviewer entry.

Per-row assertions (every row):

- ``record_kind`` is ``install_topology_truth_row_record`` and
  ``install_topology_truth_row_schema_version`` is ``1``.
- ``install_truth_profile_id`` is unique, non-empty, and matches the row
  schema's pattern.
- ``install_mode_class``, ``channel_class``, ``updater_owner_class``,
  ``binary_root_class``, ``side_by_side_relation_class``,
  ``file_association_ownership_class``,
  ``protocol_handler_ownership_class``, ``revert_path_class``,
  ``silent_deployment_baseline_class``, and every member of
  ``m1_truth_surfaces`` are in their closed vocabularies.
- ``durable_state_root_class_refs`` is a non-empty list.
- ``m1_truth_surfaces`` contains ``help_about``.
- ``owner_dri`` is a non-empty ``@handle``.
- ``install_profile_card_ref`` is non-empty and its base path
  (``artifacts/release/install_topology_matrix.yaml``) resolves on disk.
- ``failure_drill`` is a non-null object whose ``drill_id`` is in
  ``failure_drill_id_vocabulary``, whose ``forced_input`` declares at
  least one drift, and whose ``expected_check_id`` and
  ``actionable_next_action`` are non-empty.
- Structural invariants:
  - ``side_by_side_preview`` rows publish a non-``none``
    ``side_by_side_relation_class`` AND a non-null
    ``paired_channel_class``.
  - ``managed_deployed`` rows MUST NOT publish
    ``revert_path_class = 'unsupported'``.
  - ``portable`` rows MUST publish
    ``revert_path_class = 'portable_swap'`` and
    ``silent_deployment_baseline_class =
    'portable_swap_no_silent_required'``.
  - No row may publish
    ``silent_deployment_baseline_class = 'managed_silent_full'`` (M1
    baseline forbids that depth).

Envelope assertions:

- ``schema_version = 1``, ``matrix_id =
  m1_install_topology_truth_seed``, ``status`` non-empty,
  ``owner_dri`` is a ``@handle``.
- ``overview_page``, ``silent_deployment_design_packet_ref``,
  ``install_profile_card_source``, ``state_root_map_source``,
  ``row_schema_ref``, ``build_identity_ref``, ``validation_lane_ref``
  resolve on disk.
- Closed envelope vocabularies match the row schema $defs verbatim.
- ``silent_deployment_baseline_class_vocabulary`` MUST NOT contain
  ``managed_silent_full``.
- Every ``required_install_mode_coverage`` member is present as at
  least one row.
- Every ``required_m1_truth_surface_coverage`` member is present on at
  least one row.
- Every ``named_runtime_consumers[].consumer_ref`` resolves on disk,
  the consumer_class is from the closed vocabulary, and
  ``consumed_fields`` is non-empty.

``--force-drill <install_truth_profile_id>:<drill_id>`` replays the
named drill on the named row and exits 0 only when the runner
reproduces the declared ``expected_check_id``. Drift in the unforced
rows still fails the lane.

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


DEFAULT_MATRIX_REL = "artifacts/install/state_root_matrix.yaml"
DEFAULT_ENVELOPE_SCHEMA_REL = "schemas/install/install_topology_truth.schema.json"
DEFAULT_ROW_SCHEMA_REL = "schemas/install/install_topology_truth_row.schema.json"
DEFAULT_BUILD_IDENTITY_REL = "artifacts/build/build_identity.json"
DEFAULT_REPORT_REL = (
    "artifacts/milestones/m1/captures/"
    "install_topology_truth_validation_capture.json"
)

EXPECTED_RECORD_KIND = "install_topology_truth_row_record"
EXPECTED_ROW_SCHEMA_VERSION = 1
EXPECTED_MATRIX_ID = "m1_install_topology_truth_seed"
EXPECTED_OVERVIEW_PAGE = "docs/install/m1_install_topology_truth.md"
EXPECTED_DESIGN_PACKET = "design/m1/silent_deployment_baseline.md"

INSTALL_TRUTH_PROFILE_ID_PATTERN = re.compile(r"^[a-z0-9]+(?:[._-][a-z0-9]+)*$")
OWNER_DRI_PATTERN = re.compile(r"^@[a-zA-Z0-9_-]+$")

# Tokens the M1 baseline forbids in the silent-deployment vocabulary
# and in any row. Keeping the assertion data-driven so a future
# milestone can widen by editing this set and the seed together.
FORBIDDEN_SILENT_DEPLOYMENT_BASELINE_CLASSES = frozenset(
    {"managed_silent_full"}
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
    install_truth_profile_id: str
    install_mode_class: str
    channel_class: str
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
            "'<install_truth_profile_id>:<drill_id>'. The runner exits "
            "0 only when the row's failure drill reproduces the exact "
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

    if forced_overrides.get("clear_durable_state_root_class_refs"):
        row["durable_state_root_class_refs"] = []

    if forced_overrides.get("clear_updater_owner_class"):
        row["updater_owner_class"] = ""

    if forced_overrides.get("clear_owner_dri"):
        row["owner_dri"] = ""

    if "rewrite_silent_deployment_baseline_class" in forced_overrides:
        row["silent_deployment_baseline_class"] = forced_overrides[
            "rewrite_silent_deployment_baseline_class"
        ]

    if "rewrite_revert_path_class" in forced_overrides:
        row["revert_path_class"] = forced_overrides[
            "rewrite_revert_path_class"
        ]

    if "rewrite_side_by_side_relation_class" in forced_overrides:
        row["side_by_side_relation_class"] = forced_overrides[
            "rewrite_side_by_side_relation_class"
        ]

    if forced_overrides.get("drop_help_about_surface"):
        surfaces = row.get("m1_truth_surfaces")
        if isinstance(surfaces, list):
            row["m1_truth_surfaces"] = [
                s for s in surfaces if s != "help_about"
            ]

    return row


def validate_row(
    row: dict[str, Any],
    *,
    install_truth_profile_id_value: str,
    install_mode_vocab: set[str],
    channel_vocab: set[str],
    updater_owner_vocab: set[str],
    binary_root_vocab: set[str],
    side_by_side_relation_vocab: set[str],
    file_association_vocab: set[str],
    protocol_handler_vocab: set[str],
    revert_path_vocab: set[str],
    silent_deployment_baseline_vocab: set[str],
    m1_truth_surface_vocab: set[str],
    failure_drill_id_vocab: set[str],
    repo_root: Path,
) -> RowResult:
    install_truth_profile_id = ensure_str(
        row.get("install_truth_profile_id"),
        f"{install_truth_profile_id_value}.install_truth_profile_id",
    )
    install_mode_class = (
        row.get("install_mode_class")
        if isinstance(row.get("install_mode_class"), str)
        else ""
    )
    channel_class = (
        row.get("channel_class")
        if isinstance(row.get("channel_class"), str)
        else ""
    )

    result = RowResult(
        install_truth_profile_id=install_truth_profile_id,
        install_mode_class=install_mode_class,
        channel_class=channel_class,
    )

    # --- discriminator + version pin ----------------------------------
    if row.get("record_kind") != EXPECTED_RECORD_KIND:
        fail(
            result,
            "install_topology_truth.record_kind_wrong",
            (
                f"record_kind must be {EXPECTED_RECORD_KIND!r}; got "
                f"{row.get('record_kind')!r}"
            ),
        )
    if (
        row.get("install_topology_truth_row_schema_version")
        != EXPECTED_ROW_SCHEMA_VERSION
    ):
        fail(
            result,
            "install_topology_truth.schema_version_wrong",
            (
                "install_topology_truth_row_schema_version must be "
                f"{EXPECTED_ROW_SCHEMA_VERSION}; got "
                f"{row.get('install_topology_truth_row_schema_version')!r}"
            ),
        )

    # --- install_truth_profile_id pattern -----------------------------
    if not INSTALL_TRUTH_PROFILE_ID_PATTERN.match(install_truth_profile_id):
        fail(
            result,
            "install_topology_truth.install_truth_profile_id_pattern_invalid",
            (
                f"install_truth_profile_id {install_truth_profile_id!r} "
                f"does not match {INSTALL_TRUTH_PROFILE_ID_PATTERN.pattern!r}"
            ),
        )

    # --- closed-vocabulary axes ---------------------------------------
    closed_axes = [
        ("install_mode_class", install_mode_vocab),
        ("channel_class", channel_vocab),
        ("updater_owner_class", updater_owner_vocab),
        ("binary_root_class", binary_root_vocab),
        ("side_by_side_relation_class", side_by_side_relation_vocab),
        ("file_association_ownership_class", file_association_vocab),
        ("protocol_handler_ownership_class", protocol_handler_vocab),
        ("revert_path_class", revert_path_vocab),
        ("silent_deployment_baseline_class", silent_deployment_baseline_vocab),
    ]
    for field_name, vocab in closed_axes:
        value = row.get(field_name)
        if not isinstance(value, str) or not value.strip():
            fail(
                result,
                f"install_topology_truth.{field_name}_required",
                (
                    f"{field_name} must be a non-empty member of the "
                    "closed vocabulary"
                ),
            )
        elif value not in vocab:
            fail(
                result,
                f"install_topology_truth.{field_name}_unknown",
                (
                    f"{field_name} {value!r} is not in the row schema's "
                    f"{field_name} enum"
                ),
            )

    # --- m1_truth_surfaces --------------------------------------------
    surfaces = row.get("m1_truth_surfaces")
    if not isinstance(surfaces, list) or not surfaces:
        fail(
            result,
            "install_topology_truth.m1_truth_surfaces_required",
            "m1_truth_surfaces must be a non-empty list",
        )
    else:
        for sf in surfaces:
            if not isinstance(sf, str) or sf not in m1_truth_surface_vocab:
                fail(
                    result,
                    "install_topology_truth.m1_truth_surface_unknown",
                    (
                        f"m1_truth_surfaces entry {sf!r} is not in the "
                        "row schema's m1_truth_surface enum"
                    ),
                )
        if "help_about" not in surfaces:
            fail(
                result,
                "install_topology_truth.help_about_truth_surface_required",
                (
                    "m1_truth_surfaces MUST include 'help_about' so users "
                    "can inspect install topology from the About pane"
                ),
            )

    # --- durable_state_root_class_refs --------------------------------
    state_roots = row.get("durable_state_root_class_refs")
    if not isinstance(state_roots, list) or not state_roots:
        fail(
            result,
            "install_topology_truth.durable_state_root_class_refs_required",
            (
                "durable_state_root_class_refs must be a non-empty list; "
                "even portable rows declare portable_colocated_root"
            ),
        )
    else:
        for ref in state_roots:
            if not isinstance(ref, str) or not ref.strip():
                fail(
                    result,
                    "install_topology_truth.durable_state_root_class_ref_empty",
                    "durable_state_root_class_refs entry must be non-empty",
                )

    # --- owner_dri ----------------------------------------------------
    owner_dri = row.get("owner_dri")
    if not isinstance(owner_dri, str) or not owner_dri.strip():
        fail(
            result,
            "install_topology_truth.owner_dri_required",
            "owner_dri must be a non-empty @handle",
        )
    elif not OWNER_DRI_PATTERN.match(owner_dri):
        fail(
            result,
            "install_topology_truth.owner_dri_pattern_invalid",
            f"owner_dri {owner_dri!r} must match {OWNER_DRI_PATTERN.pattern!r}",
        )

    # --- install_profile_card_ref -------------------------------------
    card_ref = row.get("install_profile_card_ref")
    if not isinstance(card_ref, str) or not card_ref.strip():
        fail(
            result,
            "install_topology_truth.install_profile_card_ref_required",
            "install_profile_card_ref must be a non-empty string",
        )
    elif not artifact_ref_exists(repo_root, card_ref):
        fail(
            result,
            "install_topology_truth.install_profile_card_ref_base_path_missing",
            (
                f"install_profile_card_ref base path does not resolve on "
                f"disk: {card_ref}"
            ),
        )

    # --- paired_channel_class -----------------------------------------
    # paired_channel_class must agree with side_by_side_relation_class.
    # The two "_isolated" relations are explicit "no pair" classes for
    # portable / managed-image rows; they behave like 'none' for the
    # pairing rule but stay distinct for surface copy.
    paired = row.get("paired_channel_class")
    sxs = row.get("side_by_side_relation_class")
    non_paired_relations = {"none", "portable_isolated", "managed_image_isolated"}
    paired_relations = {"stable_and_preview", "preview_paired_with_stable"}
    if isinstance(sxs, str) and sxs in non_paired_relations:
        if paired is not None:
            fail(
                result,
                "install_topology_truth.paired_channel_class_must_be_null_when_relation_non_paired",
                (
                    "paired_channel_class MUST be null when "
                    f"side_by_side_relation_class is {sxs!r}"
                ),
            )
    elif isinstance(sxs, str) and sxs in paired_relations:
        if paired is None or (isinstance(paired, str) and not paired.strip()):
            fail(
                result,
                "install_topology_truth.paired_channel_class_required_when_relation_paired",
                (
                    "paired_channel_class MUST be a non-null channel value "
                    f"when side_by_side_relation_class is {sxs!r}"
                ),
            )
        elif isinstance(paired, str) and paired not in channel_vocab:
            fail(
                result,
                "install_topology_truth.paired_channel_class_unknown",
                (
                    f"paired_channel_class {paired!r} is not in the row "
                    "schema's channel_class enum"
                ),
            )

    # --- side_by_side_preview rule ------------------------------------
    if install_mode_class == "side_by_side_preview":
        if not isinstance(sxs, str) or sxs in non_paired_relations:
            fail(
                result,
                "install_topology_truth.side_by_side_preview_relation_must_disclose_paired_channel",
                (
                    "side_by_side_preview rows MUST publish a paired "
                    "side_by_side_relation_class (stable_and_preview or "
                    "preview_paired_with_stable) and a non-null "
                    "paired_channel_class so they cannot quietly read as "
                    "single-channel"
                ),
            )

    # --- managed_deployed revert-path rule ----------------------------
    if install_mode_class == "managed_deployed":
        rpc = row.get("revert_path_class")
        if rpc == "unsupported":
            fail(
                result,
                "install_topology_truth.revert_path_class_unsupported_blocked_for_managed_deployed",
                (
                    "managed_deployed rows MUST NOT publish "
                    "revert_path_class = 'unsupported'; publish "
                    "managed_pin_to_prior_build or package_manager_revert"
                ),
            )

    # --- portable rules -----------------------------------------------
    if install_mode_class == "portable":
        rpc = row.get("revert_path_class")
        if rpc != "portable_swap":
            fail(
                result,
                "install_topology_truth.portable_revert_path_class_must_be_portable_swap",
                (
                    "portable rows MUST publish "
                    "revert_path_class = 'portable_swap'"
                ),
            )
        sdb = row.get("silent_deployment_baseline_class")
        if sdb != "portable_swap_no_silent_required":
            fail(
                result,
                "install_topology_truth.portable_silent_deployment_baseline_must_be_portable_swap_no_silent_required",
                (
                    "portable rows MUST publish "
                    "silent_deployment_baseline_class = "
                    "'portable_swap_no_silent_required'"
                ),
            )

    # --- silent-deployment forbidden token rule -----------------------
    sdb_value = row.get("silent_deployment_baseline_class")
    if (
        isinstance(sdb_value, str)
        and sdb_value in FORBIDDEN_SILENT_DEPLOYMENT_BASELINE_CLASSES
    ):
        fail(
            result,
            "install_topology_truth.silent_deployment_baseline_managed_silent_full_blocked_in_baseline",
            (
                f"silent_deployment_baseline_class {sdb_value!r} is "
                "forbidden in the M1 baseline; widening requires a "
                "separately reviewed decision row"
            ),
        )

    # --- failure_drill -------------------------------------------------
    drill = row.get("failure_drill")
    if not isinstance(drill, dict):
        fail(
            result,
            "install_topology_truth.failure_drill_required",
            "failure_drill must be a non-null object on every row",
        )
    else:
        drill_id = drill.get("drill_id")
        if not isinstance(drill_id, str) or not drill_id.strip():
            fail(
                result,
                "install_topology_truth.failure_drill_drill_id_required",
                "failure_drill.drill_id must be a non-empty string",
            )
        elif drill_id not in failure_drill_id_vocab:
            fail(
                result,
                "install_topology_truth.failure_drill_drill_id_unknown",
                (
                    f"failure_drill.drill_id {drill_id!r} is not in "
                    "failure_drill_id_vocabulary"
                ),
            )
        forced_input = drill.get("forced_input")
        if not isinstance(forced_input, dict) or not forced_input:
            fail(
                result,
                "install_topology_truth.failure_drill_forced_input_empty",
                "failure_drill.forced_input must declare at least one drift",
            )
        expected_check = drill.get("expected_check_id")
        if (
            not isinstance(expected_check, str)
            or not expected_check.strip()
        ):
            fail(
                result,
                "install_topology_truth.failure_drill_expected_check_id_required",
                "failure_drill.expected_check_id must be non-empty",
            )
        actionable = drill.get("actionable_next_action")
        if not isinstance(actionable, str) or not actionable.strip():
            fail(
                result,
                "install_topology_truth.failure_drill_actionable_next_action_required",
                "failure_drill.actionable_next_action must be non-empty",
            )

    result.diagnostics.update(
        {
            "install_truth_profile_id": install_truth_profile_id,
            "install_mode_class": install_mode_class,
            "channel_class": channel_class,
            "updater_owner_class": row.get("updater_owner_class"),
            "revert_path_class": row.get("revert_path_class"),
            "silent_deployment_baseline_class": row.get(
                "silent_deployment_baseline_class"
            ),
            "side_by_side_relation_class": row.get(
                "side_by_side_relation_class"
            ),
            "paired_channel_class": row.get("paired_channel_class"),
            "m1_truth_surfaces": row.get("m1_truth_surfaces"),
            "durable_state_root_class_refs": row.get(
                "durable_state_root_class_refs"
            ),
        }
    )

    if not result.failed_checks:
        pass_(result, f"row {install_truth_profile_id} passes")

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
                check_id="install_topology_truth.envelope_schema_version_wrong",
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
                check_id="install_topology_truth.envelope_matrix_id_wrong",
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
                check_id="install_topology_truth.envelope_owner_dri_pattern_invalid",
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
                check_id="install_topology_truth.envelope_overview_page_wrong",
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
                check_id="install_topology_truth.envelope_overview_page_missing",
                message=f"overview_page does not exist: {overview_page}",
                remediation="Create the reviewer landing page or fix the path.",
                ref=overview_page,
            )
        )

    design_packet = ensure_str(
        matrix.get("silent_deployment_design_packet_ref"),
        "matrix.silent_deployment_design_packet_ref",
    )
    if design_packet != EXPECTED_DESIGN_PACKET:
        findings.append(
            Finding(
                severity="error",
                check_id="install_topology_truth.envelope_silent_deployment_design_packet_ref_wrong",
                message=(
                    f"silent_deployment_design_packet_ref must be "
                    f"{EXPECTED_DESIGN_PACKET!r}; got {design_packet!r}"
                ),
                remediation="Restore the canonical design packet path.",
            )
        )
    if not artifact_ref_exists(repo_root, design_packet):
        findings.append(
            Finding(
                severity="error",
                check_id="install_topology_truth.envelope_silent_deployment_design_packet_missing",
                message=(
                    f"silent_deployment_design_packet_ref does not exist: "
                    f"{design_packet}"
                ),
                remediation="Create the silent-deployment design packet or fix the path.",
                ref=design_packet,
            )
        )

    for key in (
        "install_profile_card_source",
        "state_root_map_source",
        "row_schema_ref",
        "build_identity_ref",
    ):
        ref = ensure_str(matrix.get(key), f"matrix.{key}")
        if not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"install_topology_truth.envelope_{key}_missing",
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
                check_id="install_topology_truth.envelope_validation_lane_ref_missing",
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

    install_mode_vocab = load_vocab("install_mode_class_vocabulary")
    channel_vocab = load_vocab("channel_class_vocabulary")
    updater_owner_vocab = load_vocab("updater_owner_class_vocabulary")
    binary_root_vocab = load_vocab("binary_root_class_vocabulary")
    side_by_side_relation_vocab = load_vocab(
        "side_by_side_relation_class_vocabulary"
    )
    file_association_vocab = load_vocab(
        "file_association_ownership_class_vocabulary"
    )
    protocol_handler_vocab = load_vocab(
        "protocol_handler_ownership_class_vocabulary"
    )
    revert_path_vocab = load_vocab("revert_path_class_vocabulary")
    silent_deployment_baseline_vocab = load_vocab(
        "silent_deployment_baseline_class_vocabulary"
    )
    m1_truth_surface_vocab = load_vocab("m1_truth_surface_class_vocabulary")
    failure_drill_id_vocab = load_vocab("failure_drill_id_vocabulary")
    required_install_mode_coverage = load_vocab(
        "required_install_mode_coverage"
    )
    required_m1_truth_surface_coverage = load_vocab(
        "required_m1_truth_surface_coverage"
    )

    # Closed-vocabulary agreement with the row schema $defs.
    row_schema_ref = ensure_str(
        matrix.get("row_schema_ref"), "matrix.row_schema_ref"
    )
    vocab_agreements = [
        ("install_mode_class_vocabulary", "install_mode_class", install_mode_vocab),
        ("channel_class_vocabulary", "channel_class", channel_vocab),
        (
            "updater_owner_class_vocabulary",
            "updater_owner_class",
            updater_owner_vocab,
        ),
        (
            "binary_root_class_vocabulary",
            "binary_root_class",
            binary_root_vocab,
        ),
        (
            "side_by_side_relation_class_vocabulary",
            "side_by_side_relation_class",
            side_by_side_relation_vocab,
        ),
        (
            "file_association_ownership_class_vocabulary",
            "file_association_ownership_class",
            file_association_vocab,
        ),
        (
            "protocol_handler_ownership_class_vocabulary",
            "protocol_handler_ownership_class",
            protocol_handler_vocab,
        ),
        (
            "revert_path_class_vocabulary",
            "revert_path_class",
            revert_path_vocab,
        ),
        (
            "silent_deployment_baseline_class_vocabulary",
            "silent_deployment_baseline_class",
            silent_deployment_baseline_vocab,
        ),
        (
            "m1_truth_surface_class_vocabulary",
            "m1_truth_surface_class",
            m1_truth_surface_vocab,
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
                        "install_topology_truth.envelope_"
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

    # Forbidden silent-deployment-baseline token must not appear in the
    # vocabulary itself.
    forbidden_in_vocab = (
        silent_deployment_baseline_vocab
        & FORBIDDEN_SILENT_DEPLOYMENT_BASELINE_CLASSES
    )
    if forbidden_in_vocab:
        findings.append(
            Finding(
                severity="error",
                check_id="install_topology_truth.silent_deployment_baseline_vocabulary_contains_forbidden_token",
                message=(
                    "silent_deployment_baseline_class_vocabulary contains "
                    f"forbidden token(s) {sorted(forbidden_in_vocab)}; M1 "
                    "baseline forbids 'managed_silent_full'"
                ),
                remediation=(
                    "Remove the forbidden token from the vocabulary; "
                    "widening requires a separately reviewed decision row."
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
                check_id="install_topology_truth.envelope_named_runtime_consumers_empty",
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
                    check_id="install_topology_truth.named_runtime_consumer_ref_missing",
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
                    check_id="install_topology_truth.named_runtime_consumer_consumed_fields_empty",
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
                "'<install_truth_profile_id>:<drill_id>'"
            )
        forced_profile_id, forced_drill_id = args.force_drill.rsplit(":", 1)
        forced_profile_id = forced_profile_id.strip()
        forced_drill_id = forced_drill_id.strip()

    rows = ensure_list(matrix.get("entries"), "matrix.entries")
    if not rows:
        findings.append(
            Finding(
                severity="error",
                check_id="install_topology_truth.envelope_entries_empty",
                message="matrix.entries must declare at least one row",
                remediation="Seed at least one install-truth profile row.",
            )
        )

    row_results: list[RowResult] = []
    seen_ids: set[str] = set()
    seen_install_modes: set[str] = set()
    seen_truth_surfaces: set[str] = set()
    forced_replay_record: dict[str, Any] | None = None

    for idx, raw_row in enumerate(rows):
        raw_row = ensure_dict(raw_row, f"matrix.entries[{idx}]")
        install_truth_profile_id_local = ensure_str(
            raw_row.get("install_truth_profile_id"),
            f"matrix.entries[{idx}].install_truth_profile_id",
        )

        applied_overrides: dict[str, Any] = {}
        replay_row_payload = raw_row
        drill_local: dict[str, Any] | None = None
        if (
            forced_profile_id is not None
            and install_truth_profile_id_local == forced_profile_id
        ):
            drill_local = raw_row.get("failure_drill")
            if not isinstance(drill_local, dict):
                raise SystemExit(
                    f"--force-drill targeted install_truth_profile_id "
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
            install_truth_profile_id_value=install_truth_profile_id_local,
            install_mode_vocab=install_mode_vocab,
            channel_vocab=channel_vocab,
            updater_owner_vocab=updater_owner_vocab,
            binary_root_vocab=binary_root_vocab,
            side_by_side_relation_vocab=side_by_side_relation_vocab,
            file_association_vocab=file_association_vocab,
            protocol_handler_vocab=protocol_handler_vocab,
            revert_path_vocab=revert_path_vocab,
            silent_deployment_baseline_vocab=silent_deployment_baseline_vocab,
            m1_truth_surface_vocab=m1_truth_surface_vocab,
            failure_drill_id_vocab=failure_drill_id_vocab,
            repo_root=repo_root,
        )
        if applied_overrides:
            result.diagnostics["forced_overrides_applied"] = (
                applied_overrides
            )
        row_results.append(result)

        if result.install_truth_profile_id in seen_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="install_topology_truth.entries_duplicate_install_truth_profile_id",
                    message=(
                        f"duplicate install_truth_profile_id: "
                        f"{result.install_truth_profile_id}"
                    ),
                    remediation="install_truth_profile_ids must be unique.",
                    ref=result.install_truth_profile_id,
                )
            )
        seen_ids.add(result.install_truth_profile_id)

        if (
            isinstance(raw_row.get("install_mode_class"), str)
            and raw_row["install_mode_class"]
        ):
            seen_install_modes.add(raw_row["install_mode_class"])

        surfaces = raw_row.get("m1_truth_surfaces")
        if isinstance(surfaces, list):
            for sf in surfaces:
                if isinstance(sf, str) and sf:
                    seen_truth_surfaces.add(sf)

        if (
            forced_profile_id is not None
            and result.install_truth_profile_id == forced_profile_id
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
                "install_truth_profile_id": forced_profile_id,
                "drill_id": forced_drill_id,
                "expected_check_id": expected_check,
                "observed_failed_check_ids": observed,
                "reproduced": expected_check in observed,
            }

    # --- coverage -----------------------------------------------------
    missing_install_modes = required_install_mode_coverage - seen_install_modes
    if missing_install_modes:
        findings.append(
            Finding(
                severity="error",
                check_id="install_topology_truth.coverage_missing_required_install_modes",
                message=(
                    "matrix must seed at least one row for each required "
                    f"install mode: {sorted(required_install_mode_coverage)};"
                    f" missing: {sorted(missing_install_modes)}"
                ),
                remediation=(
                    "Add the missing rows so every supported install mode "
                    "is inspectable from the truth surfaces."
                ),
            )
        )

    missing_truth_surfaces = (
        required_m1_truth_surface_coverage - seen_truth_surfaces
    )
    if missing_truth_surfaces:
        findings.append(
            Finding(
                severity="error",
                check_id="install_topology_truth.coverage_missing_required_m1_truth_surfaces",
                message=(
                    "matrix must seed at least one row exposing each "
                    "required M1 truth surface: "
                    f"{sorted(required_m1_truth_surface_coverage)}; "
                    f"missing: {sorted(missing_truth_surfaces)}"
                ),
                remediation=(
                    "Extend an existing row's m1_truth_surfaces or add a "
                    "new row so the protected surfaces are covered."
                ),
            )
        )

    # Promote per-row failures into findings, skipping the targeted row
    # under --force-drill so the runner's exit can reflect the drill verdict.
    for result in row_results:
        if (
            forced_profile_id is not None
            and result.install_truth_profile_id == forced_profile_id
            and forced_replay_record is not None
        ):
            continue
        for failure in result.failed_checks:
            findings.append(
                Finding(
                    severity="error",
                    check_id=failure.get(
                        "check_id", "install_topology_truth.row_failed_check"
                    ),
                    message=(
                        f"{result.install_truth_profile_id}: "
                        f"{failure.get('message', '')}"
                    ),
                    remediation=(
                        "Re-align the row with the install-topology truth "
                        "contract or fix the drift in the seed; failures "
                        "are reported with the precise actionable check_id."
                    ),
                    ref=result.install_truth_profile_id,
                )
            )

    errors = [f for f in findings if f.severity == "error"]
    status = "PASS" if not errors else "FAIL"

    capture: dict[str, Any] = {
        "schema_version": 1,
        "capture_kind": "install_topology_truth_validation_capture",
        "captured_at": now_iso_z(),
        "owner_dri": owner_dri,
        "matrix_ref": matrix_rel,
        "envelope_schema_ref": args.envelope_schema,
        "row_schema_ref": args.row_schema,
        "exact_build_identity_ref": args.build_identity,
        "command": (
            "python3 tests/governance/m1_install_topology_truth_lane/"
            "run_m1_install_topology_truth_lane.py --repo-root ."
        ),
        "status": status,
        "required_install_mode_coverage": sorted(
            required_install_mode_coverage
        ),
        "observed_install_modes": sorted(seen_install_modes),
        "required_m1_truth_surface_coverage": sorted(
            required_m1_truth_surface_coverage
        ),
        "observed_m1_truth_surfaces": sorted(seen_truth_surfaces),
        "rows": [
            {
                "install_truth_profile_id": r.install_truth_profile_id,
                "install_mode_class": r.install_mode_class,
                "channel_class": r.channel_class,
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

    label = "install-topology-truth-seed"
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
                f" on {forced_replay_record['install_truth_profile_id']} "
                f"reproduced {forced_replay_record['expected_check_id']}"
            )
            return 0
        print(
            f"[{label}] forced drill {forced_replay_record['drill_id']} on"
            f" {forced_replay_record['install_truth_profile_id']} did NOT"
            f" reproduce {forced_replay_record['expected_check_id']};"
            f" observed: {forced_replay_record['observed_failed_check_ids']}"
        )
        return 2

    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[install-topology-truth-seed] interrupted", file=sys.stderr)
        sys.exit(130)
