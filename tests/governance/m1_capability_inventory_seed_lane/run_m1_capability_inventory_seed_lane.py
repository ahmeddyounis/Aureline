#!/usr/bin/env python3
"""Unattended M1 capability-inventory seed validation lane.

Replays every row in ``artifacts/governance/capability_inventory_seed.yaml``
against:

- ``schemas/governance/capability_inventory.schema.json`` — the seed
  envelope schema (vocabularies, required coverage, named consumers,
  required M1 surface coverage list);
- ``schemas/governance/capability_inventory_entry.schema.json`` — the
  row vocabulary; and
- the canonical landing page at
  ``docs/governance/capability_lifecycle_seed.md`` so the seed cannot
  quietly point at a missing reviewer entry.

Per-row assertions (every row):

- ``record_kind`` is ``capability_inventory_entry_record`` and
  ``capability_inventory_entry_schema_version`` is ``1``;
- ``capability_id`` is unique, non-empty, and matches the row schema's
  pattern;
- ``capability_kind``, ``surface_families[]``, ``lifecycle_state``,
  ``public_label_policy``, ``public_claim_posture``, and
  ``export_visibility`` are members of their closed vocabularies in
  the row schema;
- ``surface_families`` is non-empty;
- ``owner_dri`` is a non-empty ``@handle``;
- ``owning_lane`` is non-empty;
- ``claim_lanes`` is a list (empty when ``public_claim_posture`` is
  ``forbidden``);
- ``dependency_marker_refs`` is a list (may be empty);
- ``rollout_gate`` is null or carries ``gate_kind``, ``gate_ref``, and
  ``public_disclosure_required``; ``gate_kind`` is in the closed
  rollout-gate-kind vocabulary;
- ``public_label_policy = public_label_forbidden`` forces
  ``public_label`` to null;
- ``public_claim_posture = forbidden`` forces ``claim_lanes`` empty
  and ``export_visibility`` to ``internal_redacted``;
- ``kill_switch_path`` is null or non-empty; rows that carry a non-
  null ``rollout_gate`` MUST also carry a non-null
  ``kill_switch_path`` so operators can disable the capability without
  a code change;
- ``retirement_metadata`` is null when ``lifecycle_state`` is not in
  ``{deprecated, disabled_by_policy, retired}``; non-null otherwise,
  with a non-empty ``retirement_target_window_note``.

Per-row assertions (M1 surface seed members only):

- ``m1_surface_seed_membership`` is exactly ``true`` and the row's
  ``capability_id`` is listed in
  ``required_m1_surface_coverage``;
- ``failure_drill`` is a non-null object whose ``drill_id`` is in
  ``failure_drill_id_vocabulary``, whose ``forced_input`` declares at
  least one drift, and whose ``expected_check_id`` and
  ``actionable_next_action`` are non-empty;
- pre-stable rows (``lifecycle_state`` in ``{labs, preview, beta}``)
  whose ``rollout_gate`` is non-null MUST set
  ``rollout_gate.public_disclosure_required = true`` so claim copy
  cannot quietly read as stable.

Envelope assertions:

- ``schema_version = 1``, ``matrix_id =
  m1_capability_inventory_seed``, status is non-empty, owner_dri is a
  ``@handle``;
- ``overview_page``, ``contract_doc_ref``, ``entry_schema_ref``,
  ``build_identity_ref``, ``validation_lane_ref`` resolve on disk;
- closed envelope vocabularies (``lifecycle_state_vocabulary``,
  ``failure_drill_id_vocabulary``,
  ``required_lifecycle_state_coverage``,
  ``required_m1_surface_coverage``) are non-empty;
- the ``lifecycle_state_vocabulary`` agrees with the row schema's
  ``lifecycle_state`` ``$defs`` enum;
- every ``required_lifecycle_state_coverage`` member appears at least
  once across the inventory's rows;
- every ``required_m1_surface_coverage`` member is present as an M1
  surface seed row;
- every ``named_runtime_consumers[].consumer_ref`` resolves on disk
  and ``consumed_fields`` is non-empty;
- support-export widening rule: rows with
  ``capability_kind = support_capability`` MUST have
  ``export_visibility`` in ``{support_export_only,
  internal_redacted}``.

``--force-drill <capability_id>:<drill_id>`` replays the named drill
on the named row and exits 0 only when the runner reproduces the
declared ``expected_check_id``. Drift in the unforced rows still
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


DEFAULT_MATRIX_REL = "artifacts/governance/capability_inventory_seed.yaml"
DEFAULT_ENVELOPE_SCHEMA_REL = (
    "schemas/governance/capability_inventory.schema.json"
)
DEFAULT_ROW_SCHEMA_REL = (
    "schemas/governance/capability_inventory_entry.schema.json"
)
DEFAULT_BUILD_IDENTITY_REL = "artifacts/build/build_identity.json"
DEFAULT_REPORT_REL = (
    "artifacts/milestones/m1/captures/"
    "capability_inventory_seed_validation_capture.json"
)

EXPECTED_RECORD_KIND = "capability_inventory_entry_record"
EXPECTED_ROW_SCHEMA_VERSION = 1
EXPECTED_MATRIX_ID = "m1_capability_inventory_seed"

CAPABILITY_ID_PATTERN = re.compile(r"^[a-z0-9]+(?:[._-][a-z0-9]+)*$")
OWNER_DRI_PATTERN = re.compile(r"^@[a-zA-Z0-9_-]+$")

PRE_STABLE_LIFECYCLE_STATES = {"labs", "preview", "beta"}
RETIRING_LIFECYCLE_STATES = {"deprecated", "disabled_by_policy", "retired"}


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
    capability_id: str
    capability_kind: str
    lifecycle_state: str
    m1_surface_seed_membership: bool
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
            "'<capability_id>:<drill_id>'. The runner exits 0 only when "
            "the row's failure drill reproduces the exact "
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

    if forced_overrides.get("clear_lifecycle_state"):
        row["lifecycle_state"] = ""

    if forced_overrides.get("clear_kill_switch_path"):
        row["kill_switch_path"] = ""

    if forced_overrides.get("clear_owner_dri"):
        row["owner_dri"] = ""

    if forced_overrides.get("clear_public_label"):
        row["public_label"] = None

    if "rewrite_lifecycle_state" in forced_overrides:
        row["lifecycle_state"] = forced_overrides["rewrite_lifecycle_state"]

    if "rewrite_export_visibility" in forced_overrides:
        row["export_visibility"] = forced_overrides[
            "rewrite_export_visibility"
        ]

    if forced_overrides.get("relax_rollout_gate_disclosure"):
        gate = row.get("rollout_gate")
        if isinstance(gate, dict):
            gate["public_disclosure_required"] = False

    return row


def validate_row(
    row: dict[str, Any],
    *,
    capability_id_value: str,
    capability_kind_vocab: set[str],
    surface_family_vocab: set[str],
    lifecycle_state_vocab: set[str],
    public_label_policy_vocab: set[str],
    public_claim_posture_vocab: set[str],
    export_visibility_vocab: set[str],
    rollout_gate_kind_vocab: set[str],
    failure_drill_id_vocab: set[str],
    required_m1_surface_coverage: set[str],
) -> RowResult:
    capability_id = ensure_str(
        row.get("capability_id"), f"{capability_id_value}.capability_id"
    )
    capability_kind = (
        row.get("capability_kind")
        if isinstance(row.get("capability_kind"), str)
        else ""
    )
    lifecycle_state = (
        row.get("lifecycle_state")
        if isinstance(row.get("lifecycle_state"), str)
        else ""
    )
    m1_member = bool(row.get("m1_surface_seed_membership", False))

    result = RowResult(
        capability_id=capability_id,
        capability_kind=capability_kind,
        lifecycle_state=lifecycle_state,
        m1_surface_seed_membership=m1_member,
    )

    # --- discriminator and version pins ---------------------------------
    if row.get("record_kind") != EXPECTED_RECORD_KIND:
        fail(
            result,
            "capability_inventory.record_kind_wrong",
            (
                f"record_kind must be {EXPECTED_RECORD_KIND!r}; got "
                f"{row.get('record_kind')!r}"
            ),
        )
    if (
        row.get("capability_inventory_entry_schema_version")
        != EXPECTED_ROW_SCHEMA_VERSION
    ):
        fail(
            result,
            "capability_inventory.schema_version_wrong",
            (
                "capability_inventory_entry_schema_version must be "
                f"{EXPECTED_ROW_SCHEMA_VERSION}; got "
                f"{row.get('capability_inventory_entry_schema_version')!r}"
            ),
        )

    # --- capability_id pattern ------------------------------------------
    if not CAPABILITY_ID_PATTERN.match(capability_id):
        fail(
            result,
            "capability_inventory.capability_id_pattern_invalid",
            (
                f"capability_id {capability_id!r} does not match "
                f"{CAPABILITY_ID_PATTERN.pattern!r}"
            ),
        )

    # --- capability_kind ------------------------------------------------
    if capability_kind not in capability_kind_vocab:
        fail(
            result,
            "capability_inventory.capability_kind_unknown",
            (
                f"capability_kind {capability_kind!r} is not in the "
                "row schema's capability_kind enum"
            ),
        )

    # --- surface_families -----------------------------------------------
    surface_families = row.get("surface_families")
    if not isinstance(surface_families, list) or not surface_families:
        fail(
            result,
            "capability_inventory.surface_families_required",
            "surface_families must be a non-empty list",
        )
    else:
        for sf in surface_families:
            if not isinstance(sf, str) or sf not in surface_family_vocab:
                fail(
                    result,
                    "capability_inventory.surface_family_unknown",
                    (
                        f"surface_families entry {sf!r} is not in the "
                        "row schema's surface_family enum"
                    ),
                )

    # --- lifecycle_state ------------------------------------------------
    if not lifecycle_state:
        fail(
            result,
            "capability_inventory.lifecycle_state_required",
            "lifecycle_state must be a non-empty member of the canonical vocabulary",
        )
    elif lifecycle_state not in lifecycle_state_vocab:
        fail(
            result,
            "capability_inventory.lifecycle_state_unknown",
            (
                f"lifecycle_state {lifecycle_state!r} is not in the "
                "row schema's lifecycle_state enum"
            ),
        )

    # --- owner_dri ------------------------------------------------------
    owner_dri = row.get("owner_dri")
    if not isinstance(owner_dri, str) or not owner_dri.strip():
        fail(
            result,
            "capability_inventory.owner_dri_required",
            "owner_dri must be a non-empty @handle",
        )
    elif not OWNER_DRI_PATTERN.match(owner_dri):
        fail(
            result,
            "capability_inventory.owner_dri_pattern_invalid",
            f"owner_dri {owner_dri!r} must match {OWNER_DRI_PATTERN.pattern!r}",
        )

    # --- owning_lane ----------------------------------------------------
    owning_lane = row.get("owning_lane")
    if not isinstance(owning_lane, str) or not owning_lane.strip():
        fail(
            result,
            "capability_inventory.owning_lane_required",
            "owning_lane must be a non-empty string",
        )

    # --- claim_lanes ----------------------------------------------------
    claim_lanes = row.get("claim_lanes")
    if not isinstance(claim_lanes, list):
        fail(
            result,
            "capability_inventory.claim_lanes_must_be_list",
            "claim_lanes must be a list (may be empty)",
        )

    # --- dependency_marker_refs ----------------------------------------
    marker_refs = row.get("dependency_marker_refs")
    if not isinstance(marker_refs, list):
        fail(
            result,
            "capability_inventory.dependency_marker_refs_must_be_list",
            "dependency_marker_refs must be a list (may be empty)",
        )

    # --- public_label_policy / public_label ----------------------------
    public_label_policy = row.get("public_label_policy")
    if public_label_policy not in public_label_policy_vocab:
        fail(
            result,
            "capability_inventory.public_label_policy_unknown",
            (
                f"public_label_policy {public_label_policy!r} is not in "
                "the row schema's public_label_policy enum"
            ),
        )
    public_label = row.get("public_label", "__missing__")
    if public_label == "__missing__":
        fail(
            result,
            "capability_inventory.public_label_required_field_missing",
            "public_label must be present (null or string)",
        )
    elif public_label_policy == "public_label_forbidden":
        if public_label is not None:
            fail(
                result,
                "capability_inventory.public_label_must_be_null_when_forbidden",
                "public_label must be null when public_label_policy is public_label_forbidden",
            )
    elif public_label_policy == "public_label_required":
        if not isinstance(public_label, str) or not public_label.strip():
            fail(
                result,
                "capability_inventory.public_label_required_when_policy_required",
                (
                    "public_label must be a non-empty string when "
                    "public_label_policy is public_label_required"
                ),
            )

    # --- public_claim_posture ------------------------------------------
    public_claim_posture = row.get("public_claim_posture")
    if public_claim_posture not in public_claim_posture_vocab:
        fail(
            result,
            "capability_inventory.public_claim_posture_unknown",
            (
                f"public_claim_posture {public_claim_posture!r} is not "
                "in the row schema's public_claim_posture enum"
            ),
        )

    # --- export_visibility ---------------------------------------------
    export_visibility = row.get("export_visibility")
    if export_visibility not in export_visibility_vocab:
        fail(
            result,
            "capability_inventory.export_visibility_unknown",
            (
                f"export_visibility {export_visibility!r} is not in the "
                "row schema's export_visibility enum"
            ),
        )

    if (
        public_claim_posture == "forbidden"
        and isinstance(claim_lanes, list)
        and claim_lanes
    ):
        fail(
            result,
            "capability_inventory.forbidden_claim_has_no_claim_lanes",
            "claim_lanes must be empty when public_claim_posture is forbidden",
        )
    if (
        public_claim_posture == "forbidden"
        and export_visibility != "internal_redacted"
    ):
        fail(
            result,
            "capability_inventory.forbidden_claim_is_internal_redacted",
            (
                "export_visibility must be internal_redacted when "
                "public_claim_posture is forbidden"
            ),
        )

    # support-export widening rule
    if (
        capability_kind == "support_capability"
        and export_visibility == "public_exportable"
    ):
        fail(
            result,
            "capability_inventory.support_export_visibility_widening_blocked",
            (
                "support_capability rows MUST keep export_visibility in "
                "{support_export_only, internal_redacted}; widening to "
                "public_exportable requires a separately reviewed decision row"
            ),
        )

    # --- rollout_gate ---------------------------------------------------
    gate = row.get("rollout_gate")
    if gate is not None and not isinstance(gate, dict):
        fail(
            result,
            "capability_inventory.rollout_gate_shape_invalid",
            "rollout_gate must be null or an object",
        )
    if isinstance(gate, dict):
        for key in ("gate_kind", "gate_ref", "public_disclosure_required"):
            if key not in gate:
                fail(
                    result,
                    "capability_inventory.rollout_gate_required_fields_missing",
                    f"rollout_gate is missing required field {key!r}",
                )
        if (
            isinstance(gate.get("gate_kind"), str)
            and gate["gate_kind"] not in rollout_gate_kind_vocab
        ):
            fail(
                result,
                "capability_inventory.rollout_gate_kind_unknown",
                (
                    f"rollout_gate.gate_kind {gate['gate_kind']!r} is not "
                    "in the row schema's rollout_gate_kind enum"
                ),
            )
        if (
            lifecycle_state in PRE_STABLE_LIFECYCLE_STATES
            and gate.get("public_disclosure_required") is not True
        ):
            fail(
                result,
                "capability_inventory.rollout_gate_public_disclosure_required_for_pre_stable",
                (
                    "pre-stable rows (labs / preview / beta) with a "
                    "non-null rollout_gate MUST set "
                    "public_disclosure_required = true so claim copy "
                    "cannot quietly read as stable"
                ),
            )

    # --- kill_switch_path ----------------------------------------------
    if "kill_switch_path" not in row:
        fail(
            result,
            "capability_inventory.kill_switch_path_field_missing",
            "kill_switch_path must be present (null or string)",
        )
    else:
        kill_switch_path = row["kill_switch_path"]
        if kill_switch_path is not None and (
            not isinstance(kill_switch_path, str)
            or not kill_switch_path.strip()
        ):
            fail(
                result,
                "capability_inventory.kill_switch_path_required",
                "kill_switch_path must be null or a non-empty string",
            )
        if isinstance(gate, dict) and (
            kill_switch_path is None
            or (
                isinstance(kill_switch_path, str)
                and not kill_switch_path.strip()
            )
        ):
            fail(
                result,
                "capability_inventory.kill_switch_path_required_when_rollout_gate_present",
                (
                    "kill_switch_path MUST be a non-empty opaque ref when "
                    "rollout_gate is non-null so operators can disable the "
                    "capability without a code change"
                ),
            )

    # --- retirement_metadata -------------------------------------------
    if "retirement_metadata" not in row:
        fail(
            result,
            "capability_inventory.retirement_metadata_field_missing",
            "retirement_metadata must be present (null or object)",
        )
    else:
        retirement_metadata = row["retirement_metadata"]
        if lifecycle_state in RETIRING_LIFECYCLE_STATES:
            if not isinstance(retirement_metadata, dict):
                fail(
                    result,
                    "capability_inventory.retirement_metadata_required_for_retiring_lifecycle_state",
                    (
                        f"retirement_metadata must be a non-null object when "
                        f"lifecycle_state is {lifecycle_state!r}"
                    ),
                )
            else:
                window_note = retirement_metadata.get(
                    "retirement_target_window_note"
                )
                if (
                    not isinstance(window_note, str)
                    or not window_note.strip()
                ):
                    fail(
                        result,
                        "capability_inventory.retirement_target_window_note_required",
                        (
                            "retirement_metadata.retirement_target_window_note "
                            "must be a non-empty reviewable sentence"
                        ),
                    )
        else:
            if retirement_metadata is not None and not isinstance(
                retirement_metadata, dict
            ):
                fail(
                    result,
                    "capability_inventory.retirement_metadata_shape_invalid",
                    "retirement_metadata must be null or an object",
                )

    # --- M1 surface seed membership ------------------------------------
    if m1_member:
        if capability_id not in required_m1_surface_coverage:
            fail(
                result,
                "capability_inventory.m1_surface_seed_membership_not_in_required_coverage",
                (
                    f"capability_id {capability_id!r} marks "
                    "m1_surface_seed_membership = true but is not listed "
                    "in required_m1_surface_coverage"
                ),
            )
        drill = row.get("failure_drill")
        if not isinstance(drill, dict):
            fail(
                result,
                "capability_inventory.failure_drill_required_for_m1_surface_seed",
                (
                    "failure_drill must be a non-null object when "
                    "m1_surface_seed_membership is true"
                ),
            )
        else:
            drill_id = drill.get("drill_id")
            if not isinstance(drill_id, str) or not drill_id.strip():
                fail(
                    result,
                    "capability_inventory.failure_drill_drill_id_required",
                    "failure_drill.drill_id must be a non-empty string",
                )
            elif drill_id not in failure_drill_id_vocab:
                fail(
                    result,
                    "capability_inventory.failure_drill_drill_id_unknown",
                    (
                        f"failure_drill.drill_id {drill_id!r} is not in "
                        "failure_drill_id_vocabulary"
                    ),
                )
            forced_input = drill.get("forced_input")
            if (
                not isinstance(forced_input, dict)
                or not forced_input
            ):
                fail(
                    result,
                    "capability_inventory.failure_drill_forced_input_empty",
                    "failure_drill.forced_input must declare at least one drift",
                )
            expected_check = drill.get("expected_check_id")
            if (
                not isinstance(expected_check, str)
                or not expected_check.strip()
            ):
                fail(
                    result,
                    "capability_inventory.failure_drill_expected_check_id_required",
                    "failure_drill.expected_check_id must be non-empty",
                )
            actionable = drill.get("actionable_next_action")
            if not isinstance(actionable, str) or not actionable.strip():
                fail(
                    result,
                    "capability_inventory.failure_drill_actionable_next_action_required",
                    "failure_drill.actionable_next_action must be non-empty",
                )

    result.diagnostics.update(
        {
            "capability_id": capability_id,
            "capability_kind": capability_kind,
            "lifecycle_state": lifecycle_state,
            "m1_surface_seed_membership": m1_member,
            "kill_switch_path": row.get("kill_switch_path"),
            "retirement_metadata_present": isinstance(
                row.get("retirement_metadata"), dict
            ),
        }
    )

    if not result.failed_checks:
        pass_(result, f"row {capability_id} passes")

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
                check_id="capability_inventory.envelope_schema_version_wrong",
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
                check_id="capability_inventory.envelope_matrix_id_wrong",
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
                check_id="capability_inventory.envelope_owner_dri_pattern_invalid",
                message=f"owner_dri {owner_dri!r} must match {OWNER_DRI_PATTERN.pattern!r}",
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
                check_id="capability_inventory.envelope_overview_page_missing",
                message=f"overview_page does not exist: {overview_page}",
                remediation="Create the reviewer landing page or fix the path.",
                ref=overview_page,
            )
        )

    for key in (
        "contract_doc_ref",
        "entry_schema_ref",
        "build_identity_ref",
    ):
        ref = ensure_str(matrix.get(key), f"matrix.{key}")
        if not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"capability_inventory.envelope_{key}_missing",
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
                check_id="capability_inventory.envelope_validation_lane_ref_missing",
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

    lifecycle_state_vocab = load_vocab("lifecycle_state_vocabulary")
    failure_drill_id_vocab = load_vocab("failure_drill_id_vocabulary")
    required_lifecycle_state_coverage = load_vocab(
        "required_lifecycle_state_coverage"
    )
    required_m1_surface_coverage = load_vocab("required_m1_surface_coverage")

    row_schema_ref = ensure_str(
        matrix.get("entry_schema_ref"), "matrix.entry_schema_ref"
    )
    schema_lifecycle_enum = set(
        load_schema_enum(repo_root, row_schema_ref, "lifecycle_state")
    )
    if schema_lifecycle_enum:
        diff = lifecycle_state_vocab.symmetric_difference(
            schema_lifecycle_enum
        )
        if diff:
            findings.append(
                Finding(
                    severity="error",
                    check_id="capability_inventory.envelope_lifecycle_state_vocabulary_disagrees_with_row_schema",
                    message=(
                        f"matrix.lifecycle_state_vocabulary disagrees with "
                        f"{row_schema_ref}#$defs.lifecycle_state; "
                        f"matrix-only: {sorted(lifecycle_state_vocab - schema_lifecycle_enum)}; "
                        f"schema-only: {sorted(schema_lifecycle_enum - lifecycle_state_vocab)}"
                    ),
                    remediation=(
                        "Keep the matrix vocabulary in lock-step with the "
                        "row schema; the schema is canonical."
                    ),
                )
            )

    capability_kind_vocab = set(
        load_schema_enum(repo_root, row_schema_ref, "capability_kind")
    )
    surface_family_vocab = set(
        load_schema_enum(repo_root, row_schema_ref, "surface_family")
    )
    public_label_policy_vocab = set(
        load_schema_enum(repo_root, row_schema_ref, "public_label_policy")
    )
    public_claim_posture_vocab = set(
        load_schema_enum(repo_root, row_schema_ref, "public_claim_posture")
    )
    export_visibility_vocab = set(
        load_schema_enum(repo_root, row_schema_ref, "export_visibility")
    )
    rollout_gate_kind_vocab = set(
        load_schema_enum(repo_root, row_schema_ref, "rollout_gate_kind")
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
                check_id="capability_inventory.envelope_named_runtime_consumers_empty",
                message="named_runtime_consumers must declare at least one consumer",
                remediation="Add at least one named runtime consumer that reads the inventory.",
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
                    check_id="capability_inventory.named_runtime_consumer_ref_missing",
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
                    check_id="capability_inventory.named_runtime_consumer_consumed_fields_empty",
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

    # --force-drill plumbing ---------------------------------------------
    forced_capability_id: str | None = None
    forced_drill_id: str | None = None
    if args.force_drill:
        if ":" not in args.force_drill:
            raise SystemExit(
                "--force-drill must be of the form '<capability_id>:<drill_id>'"
            )
        forced_capability_id, forced_drill_id = args.force_drill.rsplit(
            ":", 1
        )
        forced_capability_id = forced_capability_id.strip()
        forced_drill_id = forced_drill_id.strip()

    rows = ensure_list(matrix.get("entries"), "matrix.entries")
    if not rows:
        findings.append(
            Finding(
                severity="error",
                check_id="capability_inventory.envelope_entries_empty",
                message="matrix.entries must declare at least one row",
                remediation="Seed at least one inventory row.",
            )
        )

    row_results: list[RowResult] = []
    seen_ids: set[str] = set()
    seen_lifecycle_states: set[str] = set()
    seen_m1_surface_ids: set[str] = set()
    forced_replay_record: dict[str, Any] | None = None

    for idx, raw_row in enumerate(rows):
        raw_row = ensure_dict(raw_row, f"matrix.entries[{idx}]")
        capability_id_local = ensure_str(
            raw_row.get("capability_id"),
            f"matrix.entries[{idx}].capability_id",
        )

        applied_overrides: dict[str, Any] = {}
        replay_row_payload = raw_row
        drill_local: dict[str, Any] | None = None
        if (
            forced_capability_id is not None
            and capability_id_local == forced_capability_id
        ):
            drill_local = raw_row.get("failure_drill")
            if not isinstance(drill_local, dict):
                raise SystemExit(
                    f"--force-drill targeted capability_id "
                    f"{forced_capability_id!r} but the row has no failure_drill"
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
                    f"{forced_capability_id!r}"
                )
            applied_overrides = forced_input_local
            replay_row_payload = apply_forced_overrides(
                raw_row, applied_overrides
            )

        result = validate_row(
            replay_row_payload,
            capability_id_value=capability_id_local,
            capability_kind_vocab=capability_kind_vocab,
            surface_family_vocab=surface_family_vocab,
            lifecycle_state_vocab=lifecycle_state_vocab,
            public_label_policy_vocab=public_label_policy_vocab,
            public_claim_posture_vocab=public_claim_posture_vocab,
            export_visibility_vocab=export_visibility_vocab,
            rollout_gate_kind_vocab=rollout_gate_kind_vocab,
            failure_drill_id_vocab=failure_drill_id_vocab,
            required_m1_surface_coverage=required_m1_surface_coverage,
        )
        if applied_overrides:
            result.diagnostics["forced_overrides_applied"] = (
                applied_overrides
            )
        row_results.append(result)

        if result.capability_id in seen_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="capability_inventory.entries_duplicate_capability_id",
                    message=f"duplicate capability_id: {result.capability_id}",
                    remediation="capability_ids must be unique.",
                    ref=result.capability_id,
                )
            )
        seen_ids.add(result.capability_id)

        if (
            isinstance(raw_row.get("lifecycle_state"), str)
            and raw_row["lifecycle_state"]
        ):
            seen_lifecycle_states.add(raw_row["lifecycle_state"])

        if raw_row.get("m1_surface_seed_membership") is True:
            seen_m1_surface_ids.add(result.capability_id)

        if (
            forced_capability_id is not None
            and result.capability_id == forced_capability_id
            and applied_overrides
            and isinstance(drill_local, dict)
        ):
            expected_check = ensure_str(
                drill_local.get("expected_check_id"),
                f"{forced_capability_id}.failure_drill.expected_check_id",
            )
            observed = [
                fc.get("check_id")
                for fc in result.failed_checks
                if isinstance(fc, dict)
            ]
            forced_replay_record = {
                "capability_id": forced_capability_id,
                "drill_id": forced_drill_id,
                "expected_check_id": expected_check,
                "observed_failed_check_ids": observed,
                "reproduced": expected_check in observed,
            }

    # --- coverage -------------------------------------------------------
    missing_lifecycles = (
        required_lifecycle_state_coverage - seen_lifecycle_states
    )
    if missing_lifecycles:
        findings.append(
            Finding(
                severity="error",
                check_id="capability_inventory.coverage_missing_required_lifecycle_states",
                message=(
                    "matrix must seed at least one row for each required "
                    f"lifecycle_state: {sorted(required_lifecycle_state_coverage)};"
                    f" missing: {sorted(missing_lifecycles)}"
                ),
                remediation=(
                    "Add the missing rows so the canonical "
                    "Labs / Preview / Beta / Stable / Deprecated set is "
                    "covered."
                ),
            )
        )

    missing_m1_surfaces = required_m1_surface_coverage - seen_m1_surface_ids
    if missing_m1_surfaces:
        findings.append(
            Finding(
                severity="error",
                check_id="capability_inventory.coverage_missing_required_m1_surfaces",
                message=(
                    "matrix must mark every required M1 surface with "
                    "m1_surface_seed_membership = true; missing: "
                    f"{sorted(missing_m1_surfaces)}"
                ),
                remediation=(
                    "Add the missing rows or set "
                    "m1_surface_seed_membership = true on the existing "
                    "rows so the protected M1 surfaces are covered."
                ),
            )
        )

    # Promote per-row failures into findings, skipping the targeted row
    # under --force-drill so the runner's exit can reflect the drill verdict.
    for result in row_results:
        if (
            forced_capability_id is not None
            and result.capability_id == forced_capability_id
            and forced_replay_record is not None
        ):
            continue
        for failure in result.failed_checks:
            findings.append(
                Finding(
                    severity="error",
                    check_id=failure.get(
                        "check_id", "capability_inventory.row_failed_check"
                    ),
                    message=f"{result.capability_id}: {failure.get('message', '')}",
                    remediation=(
                        "Re-align the row with the capability-inventory "
                        "contract or fix the drift in the seed; failures "
                        "are reported with the precise actionable check_id."
                    ),
                    ref=result.capability_id,
                )
            )

    errors = [f for f in findings if f.severity == "error"]
    status = "PASS" if not errors else "FAIL"

    capture: dict[str, Any] = {
        "schema_version": 1,
        "capture_kind": "capability_inventory_seed_validation_capture",
        "captured_at": now_iso_z(),
        "owner_dri": owner_dri,
        "matrix_ref": matrix_rel,
        "envelope_schema_ref": args.envelope_schema,
        "row_schema_ref": args.row_schema,
        "exact_build_identity_ref": args.build_identity,
        "command": (
            "python3 tests/governance/m1_capability_inventory_seed_lane/"
            "run_m1_capability_inventory_seed_lane.py --repo-root ."
        ),
        "status": status,
        "required_lifecycle_state_coverage": sorted(
            required_lifecycle_state_coverage
        ),
        "observed_lifecycle_states": sorted(seen_lifecycle_states),
        "required_m1_surface_coverage": sorted(required_m1_surface_coverage),
        "observed_m1_surface_ids": sorted(seen_m1_surface_ids),
        "rows": [
            {
                "capability_id": r.capability_id,
                "capability_kind": r.capability_kind,
                "lifecycle_state": r.lifecycle_state,
                "m1_surface_seed_membership": r.m1_surface_seed_membership,
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

    label = "capability-inventory-seed"
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
                f" on {forced_replay_record['capability_id']} reproduced"
                f" {forced_replay_record['expected_check_id']}"
            )
            return 0
        print(
            f"[{label}] forced drill {forced_replay_record['drill_id']} on"
            f" {forced_replay_record['capability_id']} did NOT reproduce"
            f" {forced_replay_record['expected_check_id']};"
            f" observed: {forced_replay_record['observed_failed_check_ids']}"
        )
        return 2

    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[capability-inventory-seed] interrupted", file=sys.stderr)
        sys.exit(130)
