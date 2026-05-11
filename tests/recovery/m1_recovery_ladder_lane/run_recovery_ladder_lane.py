#!/usr/bin/env python3
"""Unattended M1 recovery-ladder lane runner.

Replays every row in
``artifacts/support/recovery_ladder_cases.yaml`` against the canonical
sources the matrix joins (the per-rung seed cases under
``fixtures/support/recovery_ladder_cases/``, the per-rung reviewer
examples under ``artifacts/support/recovery_examples/``, the recovery
rung matrix at ``artifacts/recovery/recovery_rungs.yaml``, and the
Project Doctor scenario matrix at ``fixtures/support/scenario_matrix.yaml``)
and asserts:

- the row's ``recovery_action_id``, ``rung_class``, and ``reversal_class``
  agree with the seed case at ``seed_case_ref``;
- the row's ``recovery_action_id`` agrees with the reviewer example
  ``recovery_action_record`` at ``reviewer_example_ref`` (and the
  example's ``reversal_class`` matches the row);
- ``rung_class``, ``reversal_class``, ``destructive_class``,
  ``implementation_status_class``, every ``preserved_state_class``,
  every ``lost_capability_class``, and every ``escalation_trigger_class``
  are in the matrix's closed vocabularies (which are themselves the
  vocabularies frozen in ``schemas/support/recovery_action.schema.json``);
- ``user_authored_files`` is preserved on every rung (no recovery rung
  may mutate authored files in M1);
- ``reversal_class = no_undo_export_only`` rows list
  ``no_local_repair_path_available`` in ``escalation_trigger_classes``;
- ``reversal_class = checkpoint_restore`` rows pin a non-null
  ``checkpoint_ref`` in ``linkage_refs``;
- every row binds a ``project_doctor_finding_ref`` of the form
  ``doctor.finding.*`` so Doctor surfaces and recovery actions agree on
  identity;
- every row pins exactly one named failure drill drawn from
  ``failure_drill_id_vocabulary`` with a typed ``expected_check_id``,
  actionable owner, and next-action sentence; and
- the matrix covers the five required rungs:
  ``safe_mode``, ``extension_quarantine``, ``open_without_restore``,
  ``cache_reset_candidate``, ``restricted_reopen``.

The runner emits a durable, machine-readable capture (``--report``) and
exits non-zero if any row fails. ``--force-drill <row_id>:<drill_id>``
replays the named drill on the named row and exits 0 only when the
runner reproduces the declared ``expected_check_id``.

YAML decoding follows the existing repository convention: matrix and
fixture files are parsed via Ruby/Psych so this script does not require
a third-party Python YAML dependency.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import re
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_MATRIX_REL = "artifacts/support/recovery_ladder_cases.yaml"
DEFAULT_REPORT_REL = (
    "artifacts/milestones/m1/captures/recovery_ladder_validation_capture.json"
)
DEFAULT_BUILD_IDENTITY_REL = "artifacts/build/build_identity.json"

DOCTOR_FINDING_PREFIX = "doctor.finding."


# ---- finding/result types -------------------------------------------------


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
    row_id: str
    rung_class: str
    reversal_class: str
    destructive_class: str
    implementation_status_class: str
    passed_checks: list[str] = field(default_factory=list)
    failed_checks: list[dict[str, str]] = field(default_factory=list)
    diagnostics: dict[str, Any] = field(default_factory=dict)


def fail(result: RowResult, check_id: str, message: str) -> None:
    result.failed_checks.append({"check_id": check_id, "message": message})


def pass_(result: RowResult, message: str) -> None:
    result.passed_checks.append(message)


# ---- shell helpers --------------------------------------------------------


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
        help="Recovery-ladder matrix YAML path, repo-relative.",
    )
    parser.add_argument(
        "--report",
        default=DEFAULT_REPORT_REL,
        help="Where to write the durable JSON capture (repo-relative).",
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
            "Replay a named failure drill on a named row in the form "
            "'<row_id>:<drill_id>'. The runner exits 0 only when the "
            "row's failure drill reproduces the exact expected_check_id."
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
                "payload = YAML.safe_load(File.read(ARGV[0]), permitted_classes: [Time, Date, DateTime], aliases: false); "
                "STDOUT.write(JSON.generate(payload))"
            ),
            str(path),
        ],
        capture_output=True,
        text=True,
    )
    if ruby.returncode != 0:
        stderr = ruby.stderr.strip() or "unknown Ruby/Psych failure"
        raise SystemExit(f"failed to parse YAML at {path} via Ruby/Psych: {stderr}")
    try:
        return json.loads(ruby.stdout)
    except json.JSONDecodeError as exc:
        raise SystemExit(f"Ruby/Psych emitted invalid JSON for {path}: {exc}") from exc


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


# ---- canonical-source loaders --------------------------------------------


def load_seed_case(repo_root: Path, ref: str) -> dict[str, Any]:
    return ensure_dict(render_yaml_as_json(repo_root / ref), ref)


def load_reviewer_example(repo_root: Path, ref: str) -> dict[str, Any]:
    path = repo_root / ref
    if not path.exists():
        raise SystemExit(f"missing reviewer example file: {path}")
    return json.loads(path.read_text(encoding="utf-8"))


def load_doctor_finding_codes(repo_root: Path, ref: str) -> set[str]:
    payload = ensure_dict(render_yaml_as_json(repo_root / ref), ref)
    coverage = ensure_dict(payload.get("coverage"), f"{ref}.coverage")
    by_family = ensure_dict(
        coverage.get("by_scenario_family"),
        f"{ref}.coverage.by_scenario_family",
    )
    codes: set[str] = set()
    for family, codes_list in by_family.items():
        for idx, code in enumerate(ensure_list(codes_list, f"{ref}.coverage.by_scenario_family.{family}")):
            codes.add(ensure_str(code, f"{ref}.coverage.by_scenario_family.{family}[{idx}]"))
    return codes


def load_recovery_rung_classes(repo_root: Path, ref: str) -> set[str]:
    payload = ensure_dict(render_yaml_as_json(repo_root / ref), ref)
    rungs = ensure_list(payload.get("rungs"), f"{ref}.rungs")
    seen: set[str] = set()
    for idx, rung in enumerate(rungs):
        rung = ensure_dict(rung, f"{ref}.rungs[{idx}]")
        seen.add(ensure_str(rung.get("rung_class"), f"{ref}.rungs[{idx}].rung_class"))
    return seen


# ---- row replay ----------------------------------------------------------


def replay_row(
    row: dict[str, Any],
    *,
    repo_root: Path,
    rung_class_vocab: set[str],
    reversal_class_vocab: set[str],
    escalation_trigger_class_vocab: set[str],
    preserved_state_class_vocab: set[str],
    lost_capability_class_vocab: set[str],
    implementation_status_class_vocab: set[str],
    destructive_class_vocab: set[str],
    failure_drill_id_vocab: set[str],
    rung_matrix_classes: set[str],
    doctor_finding_codes: set[str],
    forced_overrides: dict[str, Any] | None = None,
) -> RowResult:
    forced_overrides = forced_overrides or {}
    row_id = ensure_str(row.get("row_id"), "row.row_id")
    rung_class = ensure_str(row.get("rung_class"), f"{row_id}.rung_class")
    reversal_class = ensure_str(row.get("reversal_class"), f"{row_id}.reversal_class")
    destructive_class = ensure_str(
        row.get("destructive_class"), f"{row_id}.destructive_class"
    )
    implementation_status = ensure_str(
        row.get("implementation_status_class"),
        f"{row_id}.implementation_status_class",
    )

    # Apply forced overrides on the row's "as-evaluated" copy.
    if "rewrite_destructive_class" in forced_overrides:
        destructive_class = ensure_str(
            forced_overrides["rewrite_destructive_class"],
            f"{row_id}.failure_drill.forced_input.rewrite_destructive_class",
        )
    if "rewrite_implementation_status" in forced_overrides:
        implementation_status = ensure_str(
            forced_overrides["rewrite_implementation_status"],
            f"{row_id}.failure_drill.forced_input.rewrite_implementation_status",
        )

    preserved_states_in = list(
        ensure_list(
            row.get("preserved_state_classes"),
            f"{row_id}.preserved_state_classes",
        )
    )
    drop_preserved = forced_overrides.get("drop_preserved_state_class")
    if isinstance(drop_preserved, str):
        preserved_states_in = [s for s in preserved_states_in if s != drop_preserved]

    lost_capabilities = list(
        ensure_list(
            row.get("lost_capability_classes"),
            f"{row_id}.lost_capability_classes",
        )
    )

    escalation_triggers_in = list(
        ensure_list(
            row.get("escalation_trigger_classes"),
            f"{row_id}.escalation_trigger_classes",
        )
    )
    drop_trigger = forced_overrides.get("drop_escalation_trigger")
    if isinstance(drop_trigger, str):
        escalation_triggers_in = [
            t for t in escalation_triggers_in if t != drop_trigger
        ]

    linkage_refs_in = ensure_dict(
        row.get("linkage_refs"), f"{row_id}.linkage_refs"
    )
    project_doctor_finding_ref = linkage_refs_in.get("project_doctor_finding_ref")
    if forced_overrides.get("drop_project_doctor_finding_ref"):
        project_doctor_finding_ref = None
    checkpoint_ref = linkage_refs_in.get("checkpoint_ref")

    result = RowResult(
        row_id=row_id,
        rung_class=rung_class,
        reversal_class=reversal_class,
        destructive_class=destructive_class,
        implementation_status_class=implementation_status,
    )

    # Closed-vocabulary checks on the row itself.
    if rung_class not in rung_class_vocab:
        fail(
            result,
            "recovery_ladder.rung_class_unknown",
            f"rung_class '{rung_class}' is not in rung_class_vocabulary",
        )
    else:
        pass_(result, f"rung_class '{rung_class}' is in vocabulary")

    # Advisory: artifacts/recovery/recovery_rungs.yaml is a narrower
    # supervisor-rung registry that does not include every rung_class in
    # the support recovery vocabulary (notably open_without_restore and
    # typed_repair_flow). Note when the rung is registered there so the
    # capture diagnostics surface the join, but do not fail the lane on
    # its absence.
    if rung_class in rung_matrix_classes:
        pass_(
            result,
            f"rung_class '{rung_class}' is registered in artifacts/recovery/recovery_rungs.yaml",
        )

    if reversal_class not in reversal_class_vocab:
        fail(
            result,
            "recovery_ladder.reversal_class_unknown",
            f"reversal_class '{reversal_class}' is not in reversal_class_vocabulary",
        )

    if destructive_class not in destructive_class_vocab:
        fail(
            result,
            "recovery_ladder.destructive_class_unknown",
            (
                f"destructive_class '{destructive_class}' is not in "
                "destructive_class_vocabulary"
            ),
        )
    else:
        pass_(result, f"destructive_class '{destructive_class}' is in vocabulary")

    if implementation_status not in implementation_status_class_vocab:
        fail(
            result,
            "recovery_ladder.implementation_status_unknown",
            (
                f"implementation_status_class '{implementation_status}' is not "
                "in implementation_status_class_vocabulary"
            ),
        )
    else:
        pass_(
            result,
            f"implementation_status_class '{implementation_status}' is in vocabulary",
        )

    for token in preserved_states_in:
        if token not in preserved_state_class_vocab:
            fail(
                result,
                "recovery_ladder.preserved_state_class_unknown",
                (
                    f"preserved_state_classes contains '{token}' which is not "
                    "in preserved_state_class_vocabulary"
                ),
            )

    for token in lost_capabilities:
        if token not in lost_capability_class_vocab:
            fail(
                result,
                "recovery_ladder.lost_capability_class_unknown",
                (
                    f"lost_capability_classes contains '{token}' which is not "
                    "in lost_capability_class_vocabulary"
                ),
            )

    for token in escalation_triggers_in:
        if token not in escalation_trigger_class_vocab:
            fail(
                result,
                "recovery_ladder.escalation_trigger_class_unknown",
                (
                    f"escalation_trigger_classes contains '{token}' which is "
                    "not in escalation_trigger_class_vocabulary"
                ),
            )

    # Rule: user_authored_files MUST be preserved on every rung in M1.
    if "user_authored_files" not in preserved_states_in:
        fail(
            result,
            "recovery_ladder.user_authored_files_must_be_preserved",
            (
                "preserved_state_classes must include 'user_authored_files'; "
                "no recovery rung may mutate authored files in M1"
            ),
        )
    else:
        pass_(result, "preserves user_authored_files")

    # Rule: no_undo_export_only rungs MUST list no_local_repair_path_available.
    if reversal_class == "no_undo_export_only":
        if "no_local_repair_path_available" not in escalation_triggers_in:
            fail(
                result,
                "recovery_ladder.no_local_repair_path_required_for_no_undo_export_only",
                (
                    "reversal_class 'no_undo_export_only' requires "
                    "'no_local_repair_path_available' in escalation_trigger_classes"
                ),
            )
        else:
            pass_(
                result,
                "no_undo_export_only carries no_local_repair_path_available",
            )

    # Rule: checkpoint_restore rungs MUST bind a non-null checkpoint_ref.
    if reversal_class == "checkpoint_restore":
        if not (isinstance(checkpoint_ref, str) and checkpoint_ref.strip()):
            fail(
                result,
                "recovery_ladder.checkpoint_ref_required_for_checkpoint_restore",
                (
                    "reversal_class 'checkpoint_restore' requires a non-null "
                    "linkage_refs.checkpoint_ref"
                ),
            )
        else:
            pass_(
                result,
                "checkpoint_restore binds a checkpoint_ref",
            )

    # Rule: project_doctor_finding_ref must be present and well-formed.
    if not isinstance(project_doctor_finding_ref, str) or not project_doctor_finding_ref.strip():
        fail(
            result,
            "recovery_ladder.project_doctor_finding_ref_missing",
            (
                "linkage_refs.project_doctor_finding_ref must be a non-empty "
                f"'{DOCTOR_FINDING_PREFIX}*' code"
            ),
        )
    else:
        if not project_doctor_finding_ref.startswith(DOCTOR_FINDING_PREFIX):
            fail(
                result,
                "recovery_ladder.project_doctor_finding_ref_unknown_prefix",
                (
                    f"project_doctor_finding_ref '{project_doctor_finding_ref}' "
                    f"must start with '{DOCTOR_FINDING_PREFIX}'"
                ),
            )
        elif project_doctor_finding_ref not in doctor_finding_codes:
            fail(
                result,
                "recovery_ladder.project_doctor_finding_ref_not_registered",
                (
                    f"project_doctor_finding_ref '{project_doctor_finding_ref}' "
                    "is not registered in fixtures/support/scenario_matrix.yaml"
                ),
            )
        else:
            pass_(
                result,
                f"project_doctor_finding_ref '{project_doctor_finding_ref}' is registered",
            )

    # Cross-file agreement: seed case.
    seed_case_ref = ensure_str(row.get("seed_case_ref"), f"{row_id}.seed_case_ref")
    if not artifact_ref_exists(repo_root, seed_case_ref):
        fail(
            result,
            "recovery_ladder.seed_case_ref_missing",
            f"seed_case_ref '{seed_case_ref}' does not exist on disk",
        )
    else:
        seed = load_seed_case(repo_root, seed_case_ref)
        seed_recovery_action_id = ensure_str(
            seed.get("recovery_action_id"),
            f"{seed_case_ref}.recovery_action_id",
        )
        seed_rung_class = ensure_str(
            seed.get("rung_class"), f"{seed_case_ref}.rung_class"
        )
        seed_reversal_class = ensure_str(
            seed.get("reversal_class"), f"{seed_case_ref}.reversal_class"
        )
        row_recovery_action_id = ensure_str(
            row.get("recovery_action_id"), f"{row_id}.recovery_action_id"
        )
        if seed_recovery_action_id != row_recovery_action_id:
            fail(
                result,
                "recovery_ladder.seed_case_recovery_action_id_mismatch",
                (
                    f"seed_case.recovery_action_id '{seed_recovery_action_id}' "
                    f"does not match row.recovery_action_id "
                    f"'{row_recovery_action_id}'"
                ),
            )
        else:
            pass_(result, f"seed case agrees on recovery_action_id")
        if seed_rung_class != rung_class:
            fail(
                result,
                "recovery_ladder.seed_case_rung_class_mismatch",
                (
                    f"seed_case.rung_class '{seed_rung_class}' does not match "
                    f"row.rung_class '{rung_class}'"
                ),
            )
        if seed_reversal_class != reversal_class:
            fail(
                result,
                "recovery_ladder.seed_case_reversal_class_mismatch",
                (
                    f"seed_case.reversal_class '{seed_reversal_class}' does "
                    f"not match row.reversal_class '{reversal_class}'"
                ),
            )

    # Cross-file agreement: reviewer example.
    reviewer_example_ref = ensure_str(
        row.get("reviewer_example_ref"), f"{row_id}.reviewer_example_ref"
    )
    if not artifact_ref_exists(repo_root, reviewer_example_ref):
        fail(
            result,
            "recovery_ladder.reviewer_example_ref_missing",
            f"reviewer_example_ref '{reviewer_example_ref}' does not exist on disk",
        )
    else:
        example = load_reviewer_example(repo_root, reviewer_example_ref)
        example_recovery_action_id = ensure_str(
            example.get("recovery_action_id"),
            f"{reviewer_example_ref}.recovery_action_id",
        )
        example_reversal_class = ensure_str(
            example.get("reversal_class"),
            f"{reviewer_example_ref}.reversal_class",
        )
        row_recovery_action_id = ensure_str(
            row.get("recovery_action_id"), f"{row_id}.recovery_action_id"
        )
        if example_recovery_action_id != row_recovery_action_id:
            fail(
                result,
                "recovery_ladder.reviewer_example_recovery_action_id_mismatch",
                (
                    f"reviewer_example.recovery_action_id "
                    f"'{example_recovery_action_id}' does not match "
                    f"row.recovery_action_id '{row_recovery_action_id}'"
                ),
            )
        else:
            pass_(result, "reviewer example agrees on recovery_action_id")
        if example_reversal_class != reversal_class:
            fail(
                result,
                "recovery_ladder.reviewer_example_reversal_class_mismatch",
                (
                    f"reviewer_example.reversal_class '{example_reversal_class}' "
                    f"does not match row.reversal_class '{reversal_class}'"
                ),
            )

    # Failure drill structure.
    failure_drill = ensure_dict(row.get("failure_drill"), f"{row_id}.failure_drill")
    drill_id = ensure_str(
        failure_drill.get("drill_id"), f"{row_id}.failure_drill.drill_id"
    )
    if drill_id not in failure_drill_id_vocab:
        fail(
            result,
            "recovery_ladder.failure_drill_id_unknown",
            (
                f"failure_drill.drill_id '{drill_id}' is not in "
                "failure_drill_id_vocabulary"
            ),
        )
    expected_check = ensure_str(
        failure_drill.get("expected_check_id"),
        f"{row_id}.failure_drill.expected_check_id",
    )
    actionable_owner = ensure_str(
        failure_drill.get("actionable_owner_ref"),
        f"{row_id}.failure_drill.actionable_owner_ref",
    )
    next_action = ensure_str(
        failure_drill.get("next_action"),
        f"{row_id}.failure_drill.next_action",
    )
    forced_input = ensure_dict(
        failure_drill.get("forced_input"),
        f"{row_id}.failure_drill.forced_input",
    )
    if not forced_input:
        fail(
            result,
            "recovery_ladder.failure_drill_forced_input_empty",
            "failure_drill.forced_input must declare at least one drift",
        )

    # Diagnostics for the capture.
    result.diagnostics.update(
        {
            "rung_class": rung_class,
            "reversal_class": reversal_class,
            "destructive_class": destructive_class,
            "implementation_status_class": implementation_status,
            "preserved_state_classes": preserved_states_in,
            "lost_capability_classes": lost_capabilities,
            "escalation_trigger_classes": escalation_triggers_in,
            "linkage_refs": {
                "support_bundle_case_ref": linkage_refs_in.get(
                    "support_bundle_case_ref"
                ),
                "object_handoff_case_ref": linkage_refs_in.get(
                    "object_handoff_case_ref"
                ),
                "project_doctor_finding_ref": project_doctor_finding_ref,
                "repair_transaction_ref": linkage_refs_in.get(
                    "repair_transaction_ref"
                ),
                "checkpoint_ref": checkpoint_ref,
            },
            "seed_case_ref": seed_case_ref,
            "reviewer_example_ref": reviewer_example_ref,
            "inherited_dogfood_row_id": row.get("inherited_dogfood_row_id"),
            "failure_drill": {
                "drill_id": drill_id,
                "expected_check_id": expected_check,
                "actionable_owner_ref": actionable_owner,
                "next_action": next_action,
            },
            "forced_overrides_applied": forced_overrides,
        }
    )

    return result


# ---- main -----------------------------------------------------------------


def now_iso_z() -> str:
    return (
        dt.datetime.now(dt.timezone.utc)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(
            f"--repo-root does not look like a repository root: {repo_root}"
        )

    matrix_rel = args.matrix
    matrix_path = repo_root / matrix_rel
    matrix = ensure_dict(render_yaml_as_json(matrix_path), matrix_rel)

    findings: list[Finding] = []

    schema_version = matrix.get("schema_version")
    if not isinstance(schema_version, int) or schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.schema_version",
                message=f"matrix schema_version must be the integer 1, got {schema_version!r}",
                remediation="Bump the runner together with the schema if the matrix shape changes.",
            )
        )

    ensure_str(matrix.get("status"), "matrix.status")
    ensure_str(matrix.get("matrix_id"), "matrix.matrix_id")
    ensure_str(matrix.get("owner_dri"), "matrix.owner_dri")
    overview_page = ensure_str(matrix.get("overview_page"), "matrix.overview_page")
    if not artifact_ref_exists(repo_root, overview_page):
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.overview_page.missing",
                message=f"overview_page does not exist: {overview_page}",
                remediation="Create the reviewer-facing landing page or fix the path.",
                ref=overview_page,
            )
        )

    # Required external refs the matrix anchors.
    for key in (
        "canonical_packet_doc_ref",
        "matrix_schema_ref",
        "recovery_action_schema_ref",
        "support_bundle_schema_ref",
        "object_handoff_packet_schema_ref",
        "recovery_rungs_artifact_ref",
        "scenario_matrix_ref",
        "seed_case_dir",
        "reviewer_examples_dir",
    ):
        ref = ensure_str(matrix.get(key), f"matrix.{key}")
        if not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"matrix.{key}.missing",
                    message=f"{key} does not resolve: {ref}",
                    remediation="Fix the path or seed the referenced contract.",
                    ref=ref,
                )
            )

    validation_lane_ref = ensure_str(
        matrix.get("validation_lane_ref"), "matrix.validation_lane_ref"
    )
    fragment = validation_lane_ref.split("#", 1)[0]
    if not (repo_root / fragment).exists():
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.validation_lane_ref.missing",
                message=(
                    f"validation_lane_ref base does not exist: {validation_lane_ref}"
                ),
                remediation="Point at a seeded QE lane registry entry.",
                ref=validation_lane_ref,
            )
        )

    # Closed vocabularies.
    rung_class_vocab = {
        ensure_str(item, "matrix.rung_class_vocabulary[]")
        for item in ensure_list(
            matrix.get("rung_class_vocabulary"),
            "matrix.rung_class_vocabulary",
        )
    }
    reversal_class_vocab = {
        ensure_str(item, "matrix.reversal_class_vocabulary[]")
        for item in ensure_list(
            matrix.get("reversal_class_vocabulary"),
            "matrix.reversal_class_vocabulary",
        )
    }
    escalation_trigger_class_vocab = {
        ensure_str(item, "matrix.escalation_trigger_class_vocabulary[]")
        for item in ensure_list(
            matrix.get("escalation_trigger_class_vocabulary"),
            "matrix.escalation_trigger_class_vocabulary",
        )
    }
    preserved_state_class_vocab = {
        ensure_str(item, "matrix.preserved_state_class_vocabulary[]")
        for item in ensure_list(
            matrix.get("preserved_state_class_vocabulary"),
            "matrix.preserved_state_class_vocabulary",
        )
    }
    lost_capability_class_vocab = {
        ensure_str(item, "matrix.lost_capability_class_vocabulary[]")
        for item in ensure_list(
            matrix.get("lost_capability_class_vocabulary"),
            "matrix.lost_capability_class_vocabulary",
        )
    }
    implementation_status_class_vocab = {
        ensure_str(item, "matrix.implementation_status_class_vocabulary[]")
        for item in ensure_list(
            matrix.get("implementation_status_class_vocabulary"),
            "matrix.implementation_status_class_vocabulary",
        )
    }
    destructive_class_vocab = {
        ensure_str(item, "matrix.destructive_class_vocabulary[]")
        for item in ensure_list(
            matrix.get("destructive_class_vocabulary"),
            "matrix.destructive_class_vocabulary",
        )
    }
    failure_drill_id_vocab = {
        ensure_str(item, "matrix.failure_drill_id_vocabulary[]")
        for item in ensure_list(
            matrix.get("failure_drill_id_vocabulary"),
            "matrix.failure_drill_id_vocabulary",
        )
    }
    required_rung_coverage = {
        ensure_str(item, "matrix.required_rung_coverage[]")
        for item in ensure_list(
            matrix.get("required_rung_coverage"),
            "matrix.required_rung_coverage",
        )
    }

    # Canonical sources for cross-file checks.
    rung_matrix_classes = load_recovery_rung_classes(
        repo_root, ensure_str(matrix.get("recovery_rungs_artifact_ref"), "matrix.recovery_rungs_artifact_ref")
    )
    doctor_finding_codes = load_doctor_finding_codes(
        repo_root, ensure_str(matrix.get("scenario_matrix_ref"), "matrix.scenario_matrix_ref")
    )

    rows = ensure_list(matrix.get("rows"), "matrix.rows")
    if not rows:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.rows.empty",
                message="matrix.rows must declare at least one recovery-ladder row",
                remediation="Seed the required rows.",
            )
        )

    # Resolve --force-drill if requested.
    forced_row_id: str | None = None
    forced_drill_id: str | None = None
    if args.force_drill:
        if ":" not in args.force_drill:
            raise SystemExit(
                "--force-drill must be of the form '<row_id>:<drill_id>'"
            )
        # Row ids contain ':' (e.g.
        # 'recovery_ladder_row:safe_mode.crash_loop_entry'), so split on
        # the rightmost ':' to recover the drill suffix.
        forced_row_id, forced_drill_id = args.force_drill.rsplit(":", 1)
        forced_row_id = forced_row_id.strip()
        forced_drill_id = forced_drill_id.strip()

    row_results: list[RowResult] = []
    seen_ids: set[str] = set()
    seen_rung_classes: set[str] = set()
    forced_replay_record: dict[str, Any] | None = None

    for row in rows:
        row = ensure_dict(row, "matrix.rows[]")

        forced_overrides: dict[str, Any] = {}
        if forced_row_id is not None and ensure_str(
            row.get("row_id"), "row.row_id"
        ) == forced_row_id:
            failure_drill = ensure_dict(
                row.get("failure_drill"), f"{forced_row_id}.failure_drill"
            )
            drill_id = ensure_str(
                failure_drill.get("drill_id"),
                f"{forced_row_id}.failure_drill.drill_id",
            )
            if drill_id != forced_drill_id:
                raise SystemExit(
                    f"--force-drill drill_id '{forced_drill_id}' does not match the "
                    f"row's failure_drill.drill_id '{drill_id}'"
                )
            forced_overrides = ensure_dict(
                failure_drill.get("forced_input"),
                f"{forced_row_id}.failure_drill.forced_input",
            )

        result = replay_row(
            row,
            repo_root=repo_root,
            rung_class_vocab=rung_class_vocab,
            reversal_class_vocab=reversal_class_vocab,
            escalation_trigger_class_vocab=escalation_trigger_class_vocab,
            preserved_state_class_vocab=preserved_state_class_vocab,
            lost_capability_class_vocab=lost_capability_class_vocab,
            implementation_status_class_vocab=implementation_status_class_vocab,
            destructive_class_vocab=destructive_class_vocab,
            failure_drill_id_vocab=failure_drill_id_vocab,
            rung_matrix_classes=rung_matrix_classes,
            doctor_finding_codes=doctor_finding_codes,
            forced_overrides=forced_overrides,
        )
        row_results.append(result)

        if result.row_id in seen_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.rows.duplicate_id",
                    message=f"duplicate row_id: {result.row_id}",
                    remediation="row_ids must be unique.",
                    ref=result.row_id,
                )
            )
        seen_ids.add(result.row_id)
        seen_rung_classes.add(result.rung_class)

        if (
            forced_row_id is not None
            and result.row_id == forced_row_id
            and forced_overrides
        ):
            failure_drill = ensure_dict(
                row.get("failure_drill"), f"{forced_row_id}.failure_drill"
            )
            expected_check = ensure_str(
                failure_drill.get("expected_check_id"),
                f"{forced_row_id}.failure_drill.expected_check_id",
            )
            observed = [
                fc.get("check_id")
                for fc in result.failed_checks
                if isinstance(fc, dict)
            ]
            forced_replay_record = {
                "row_id": forced_row_id,
                "drill_id": forced_drill_id,
                "expected_check_id": expected_check,
                "observed_failed_check_ids": observed,
                "reproduced": expected_check in observed,
            }

    missing_rungs = required_rung_coverage - seen_rung_classes
    if missing_rungs:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.coverage.missing_required_rungs",
                message=(
                    "matrix must seed at least one row each for "
                    f"{sorted(required_rung_coverage)}; missing: "
                    f"{sorted(missing_rungs)}"
                ),
                remediation=(
                    "Add the missing rows so every required rung is exercised."
                ),
            )
        )

    # Promote per-row failures into findings, but skip them under
    # force-drill mode for the targeted row so the runner's exit can
    # reflect the reproduce verdict rather than failing on the
    # reproduced check.
    for result in row_results:
        if (
            forced_row_id is not None
            and result.row_id == forced_row_id
            and forced_replay_record is not None
        ):
            continue
        for failure in result.failed_checks:
            findings.append(
                Finding(
                    severity="error",
                    check_id=failure.get("check_id", "matrix.row.failed_check"),
                    message=f"{result.row_id}: {failure.get('message', '')}",
                    remediation=(
                        "Re-align the row with the canonical sources or fix "
                        "the drift in the matrix; failures are reported with "
                        "the precise actionable check_id."
                    ),
                    ref=result.row_id,
                )
            )

    errors = [f for f in findings if f.severity == "error"]
    status = "PASS" if not errors else "FAIL"

    capture: dict[str, Any] = {
        "schema_version": 1,
        "capture_kind": "recovery_ladder_validation_capture",
        "captured_at": now_iso_z(),
        "owner_dri": ensure_str(matrix.get("owner_dri"), "matrix.owner_dri"),
        "matrix_ref": matrix_rel,
        "exact_build_identity_ref": args.build_identity,
        "command": (
            "python3 tests/recovery/m1_recovery_ladder_lane/"
            "run_recovery_ladder_lane.py --repo-root ."
        ),
        "status": status,
        "required_rung_coverage": sorted(required_rung_coverage),
        "observed_rung_classes": sorted(seen_rung_classes),
        "rows": [
            {
                "row_id": r.row_id,
                "rung_class": r.rung_class,
                "reversal_class": r.reversal_class,
                "destructive_class": r.destructive_class,
                "implementation_status_class": r.implementation_status_class,
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
        json.dumps(capture, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )

    label = "recovery-ladder"
    print(
        f"[{label}] {status} ({len(errors)} errors, "
        f"{len(findings) - len(errors)} warnings) — capture: {args.report}"
    )
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        print(f"[{label}] {prefix} {finding.check_id}: {finding.message}{ref_suffix}")
        print(f"[{label}]   remediation: {finding.remediation}")

    if forced_replay_record is not None:
        if forced_replay_record["reproduced"]:
            print(
                f"[{label}] forced drill {forced_replay_record['drill_id']} on "
                f"{forced_replay_record['row_id']} reproduced "
                f"{forced_replay_record['expected_check_id']}"
            )
            return 0
        print(
            f"[{label}] forced drill {forced_replay_record['drill_id']} on "
            f"{forced_replay_record['row_id']} did NOT reproduce "
            f"{forced_replay_record['expected_check_id']}; "
            f"observed: {forced_replay_record['observed_failed_check_ids']}"
        )
        return 2

    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[recovery-ladder] interrupted", file=sys.stderr)
        sys.exit(130)
