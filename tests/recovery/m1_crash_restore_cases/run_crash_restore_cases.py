#!/usr/bin/env python3
"""Unattended crash-recovery and restore-fidelity drill runner.

Replays the rows in
``fixtures/recovery/restore_fidelity_cases/m1_crash_restore_matrix.yaml``
against:

- the existing session-restore JSON fixtures at
  ``fixtures/recovery/session_restore_cases/``, which are the canonical
  output of the restore-proposal builder
  (``crates/aureline-recovery/src/session_restore/proposal.rs``); and
- a pure-Python projection of the safe-mode profile and crash-loop
  containment record emitted by
  ``crates/aureline-shell/src/recovery/safe_mode.rs`` and
  ``crates/aureline-shell/src/recovery/crash_loop.rs``.

The runner asserts:

- restore-fidelity classes are honest (a layout-only proposal never
  silently claims exact_restore; corrupt frames always pin
  manual_repair_required);
- side-effectful surfaces (terminal/debugger/notebook/ai_panel) stay
  blocked_side_effectful so restore never auto-reruns commands;
- crash-loop containment exposes the four required first-class offers
  (Open safe mode, Disable suspect extension/runtime, Open without
  restore, Export evidence);
- safe mode preserves the named state classes verbatim and forbids
  trust widening without review; and
- the named failure drills, when forced, reproduce the precise
  failing check_id the matrix declares — never silently passing on a
  partial replay.

The runner emits a durable, machine-readable capture (``--report``)
and exits non-zero if any row fails an invariant or an unknown token
appears outside the matrix's closed vocabulary.

YAML decoding follows the existing repository convention: the matrix
is parsed via Ruby/Psych so this script does not require a third-party
Python YAML dependency.
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


DEFAULT_MATRIX_REL = "fixtures/recovery/restore_fidelity_cases/m1_crash_restore_matrix.yaml"
DEFAULT_REPORT_REL = (
    "artifacts/milestones/m1/captures/crash_restore_validation_capture.json"
)
DEFAULT_BUILD_IDENTITY_REL = "artifacts/build/build_identity.json"


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
    drill_row_id: str
    drill_kind: str
    passed_checks: list[str] = field(default_factory=list)
    failed_checks: list[dict[str, str]] = field(default_factory=list)
    diagnostics: dict[str, Any] = field(default_factory=dict)


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
        help="Drill matrix YAML path, repo-relative.",
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
            "Optional drill_id to force. The runner mutates the row's "
            "input per the row's failure_drill.forced_input and asserts "
            "the resulting capture lists the row's expected check_id "
            "as a failure. Exits 0 only when the drill reproduces."
        ),
    )
    return parser.parse_args()


# ---- YAML helpers ---------------------------------------------------------


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
                "payload = YAML.safe_load(File.read(ARGV[0]), permitted_classes: [], aliases: false); "
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


# ---- matrix vocabulary ----------------------------------------------------


@dataclass
class MatrixVocab:
    restore_class: set[str]
    restore_plan_kind: set[str]
    side_effectful_surface_role: set[str]
    downgrade_trigger: set[str]
    frame_integrity_state: set[str]
    replay_posture: set[str]
    guided_choice: set[str]
    recovery_rung: set[str]
    safe_mode_entry_reason: set[str]
    crash_loop_offer_key: set[str]
    required_first_class_offers: set[str]
    required_scenario_class_coverage: set[str]


def load_matrix_vocab(matrix: dict[str, Any]) -> MatrixVocab:
    def vocab_for(field_name: str) -> set[str]:
        raw = ensure_list(matrix.get(field_name), f"matrix.{field_name}")
        return {ensure_str(item, f"matrix.{field_name}[]") for item in raw}

    return MatrixVocab(
        restore_class=vocab_for("restore_class_vocabulary"),
        restore_plan_kind=vocab_for("restore_plan_kind_vocabulary"),
        side_effectful_surface_role=vocab_for("side_effectful_surface_role_vocabulary"),
        downgrade_trigger=vocab_for("downgrade_trigger_vocabulary"),
        frame_integrity_state=vocab_for("frame_integrity_state_vocabulary"),
        replay_posture=vocab_for("replay_posture_vocabulary"),
        guided_choice=vocab_for("guided_choice_vocabulary"),
        recovery_rung=vocab_for("recovery_rung_vocabulary"),
        safe_mode_entry_reason=vocab_for("safe_mode_entry_reason_vocabulary"),
        crash_loop_offer_key=vocab_for("crash_loop_offer_key_vocabulary"),
        required_first_class_offers=vocab_for("required_first_class_crash_loop_offers"),
        required_scenario_class_coverage=vocab_for(
            "required_scenario_class_coverage"
        ),
    )


# ---- safe-mode / crash-loop projection ------------------------------------
#
# These pure-Python mirrors echo the upstream Rust modules so the runner
# can replay the safe-mode and crash-loop drill row without spinning up
# a Cargo build. The constants are kept in lock-step with
# crates/aureline-shell/src/recovery/safe_mode.rs and
# crates/aureline-shell/src/recovery/crash_loop.rs; the matrix declares
# the same expected tokens so any drift is caught loudly.

SAFE_MODE_DISABLED_CAPABILITIES = [
    "extension_auto_activation",
    "extension_host_launch",
    "session_restore_auto_reopen",
    "remote_helper_attach",
    "ai_runtime_access",
    "background_rebuild",
    "terminal_repo_recipe_launch",
    "debug_launch",
    "notebook_kernel_connect",
    "environment_activator_run",
]
SAFE_MODE_PRESERVED_STATE_CLASSES = [
    "user_authored_files",
    "open_buffer_selection",
    "durable_workspace_indexes",
    "workspace_trust_store",
    "credential_store",
    "managed_policy_overrides",
    "session_restore_store",
    "support_export_store",
]
SAFE_MODE_NEXT_OPTIONS = [
    "widen_to_full_mode_with_review",
    "escalate_to_extension_quarantine",
    "escalate_to_cache_reset_candidate",
    "open_without_restore",
    "export_escalation_packet",
]
SAFE_MODE_ENTER_COMMAND_ID = "cmd:workspace.enter_safe_mode"
SAFE_MODE_EXIT_COMMAND_ID = "cmd:workspace.exit_safe_mode_after_review"

CRASH_LOOP_FIRST_CLASS_OFFERS = [
    "open_safe_mode",
    "disable_suspect_extension_or_runtime",
    "open_without_restore",
    "export_evidence",
]
CRASH_LOOP_GATED_OFFERS = ["repair_cache_or_index"]
CRASH_LOOP_OFFER_COMMAND_IDS = {
    "open_safe_mode": "cmd:workspace.enter_safe_mode",
    "disable_suspect_extension_or_runtime": "cmd:workspace.quarantine_suspect_extension",
    "open_without_restore": "cmd:workspace.open_without_restore",
    "export_evidence": "cmd:workspace.export_recovery_evidence",
    "repair_cache_or_index": "cmd:workspace.repair_cache_or_index_candidate",
}


def project_safe_mode_profile(reason_class: str) -> dict[str, Any]:
    return {
        "record_kind": "safe_mode_profile_record",
        "safe_mode_profile_schema_version": 1,
        "entry_reason_class": reason_class,
        "trust_state_after_entry": "restricted_recovery_fallback",
        "disabled_or_narrowed_capabilities": list(SAFE_MODE_DISABLED_CAPABILITIES),
        "preserved_state_classes": list(SAFE_MODE_PRESERVED_STATE_CLASSES),
        "entry_visible_and_logged": True,
        "entry_confirmation_required": reason_class == "user_requested",
        "trust_widening_forbidden_without_review": True,
        "auto_restore_forbidden": True,
        "safe_mode_enter_command_id": SAFE_MODE_ENTER_COMMAND_ID,
        "safe_mode_exit_command_id": SAFE_MODE_EXIT_COMMAND_ID,
        "next_options_after_entry": list(SAFE_MODE_NEXT_OPTIONS),
    }


def project_crash_loop_containment(reason_class: str) -> dict[str, Any]:
    safe_mode_profile = project_safe_mode_profile("crash_loop_detected")
    offers: list[dict[str, Any]] = []
    for key in CRASH_LOOP_FIRST_CLASS_OFFERS:
        offers.append(
            {
                "offer_key": key,
                "command_id": CRASH_LOOP_OFFER_COMMAND_IDS[key],
                "first_class": True,
            }
        )
    for key in CRASH_LOOP_GATED_OFFERS:
        offers.append(
            {
                "offer_key": key,
                "command_id": CRASH_LOOP_OFFER_COMMAND_IDS[key],
                "first_class": False,
            }
        )
    return {
        "record_kind": "crash_loop_containment_record",
        "crash_loop_record_schema_version": 1,
        "reason_class": reason_class,
        "safe_mode_profile": safe_mode_profile,
        "offers": offers,
        "auto_rerun_forbidden": True,
        "never_deletes_state": True,
    }


# ---- restore-proposal row replay -----------------------------------------


def load_proposal_fixture(repo_root: Path, fixture_ref: str) -> dict[str, Any]:
    path = repo_root / fixture_ref
    if not path.exists():
        raise SystemExit(f"missing proposal fixture: {fixture_ref}")
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise SystemExit(
            f"proposal fixture {fixture_ref} is not valid JSON: {exc}"
        ) from exc


def apply_proposal_forced_input(
    proposal: dict[str, Any], forced_input: dict[str, Any]
) -> dict[str, Any]:
    """Apply the named forced-input mutation to a proposal copy."""
    mutated = json.loads(json.dumps(proposal))  # deep copy

    if "rewrite_terminal_plan_kind" in forced_input:
        new_kind = forced_input["rewrite_terminal_plan_kind"]
        for plan in mutated.get("pane_plans", []):
            if plan.get("surface_role") == "terminal":
                plan["plan_kind"] = new_kind

    if "rewrite_restore_class" in forced_input:
        mutated["restore_class"] = forced_input["rewrite_restore_class"]

    if "drop_downgrade_trigger" in forced_input:
        target = forced_input["drop_downgrade_trigger"]
        mutated["downgrade_triggers"] = [
            trig for trig in mutated.get("downgrade_triggers", []) if trig != target
        ]

    if "rewrite_window_count" in forced_input:
        counts = mutated.setdefault("counts", {})
        counts["windows"] = int(forced_input["rewrite_window_count"])

    return mutated


def replay_proposal_row(
    repo_root: Path,
    row: dict[str, Any],
    vocab: MatrixVocab,
    force_forced_input: bool,
) -> RowResult:
    drill_row_id = ensure_str(row.get("drill_row_id"), "row.drill_row_id")
    result = RowResult(drill_row_id=drill_row_id, drill_kind="restore_proposal")

    fixture_ref = ensure_str(
        row.get("proposal_fixture_ref"), f"{drill_row_id}.proposal_fixture_ref"
    )
    proposal = load_proposal_fixture(repo_root, fixture_ref)
    if force_forced_input:
        forced = ensure_dict(
            row.get("failure_drill", {}).get("forced_input"),
            f"{drill_row_id}.failure_drill.forced_input",
        )
        proposal = apply_proposal_forced_input(proposal, forced)

    expected = ensure_dict(row.get("expected"), f"{drill_row_id}.expected")

    # Closed-vocabulary check: every restore_class on the proposal must
    # be in the matrix vocabulary.
    actual_class = ensure_str(
        proposal.get("restore_class"), f"{fixture_ref}.restore_class"
    )
    if actual_class not in vocab.restore_class:
        result.failed_checks.append(
            {
                "check_id": "row.restore_class_vocab_violation",
                "message": (
                    f"restore_class '{actual_class}' from {fixture_ref} "
                    f"is not in matrix vocabulary"
                ),
            }
        )
    else:
        result.passed_checks.append(
            f"restore_class '{actual_class}' is in matrix vocabulary"
        )

    expected_class = ensure_str(
        expected.get("restore_class"), f"{drill_row_id}.expected.restore_class"
    )
    if actual_class != expected_class:
        result.failed_checks.append(
            {
                "check_id": "row.restore_class_mismatch",
                "message": (
                    f"restore_class mismatch: expected={expected_class}, "
                    f"actual={actual_class}"
                ),
            }
        )
    else:
        result.passed_checks.append(
            f"restore_class matches expected ({expected_class})"
        )

    actual_prior_abnormal = bool(proposal.get("prior_run_abnormal"))
    if actual_prior_abnormal != bool(expected.get("prior_run_abnormal")):
        result.failed_checks.append(
            {
                "check_id": "row.prior_run_abnormal_mismatch",
                "message": (
                    f"prior_run_abnormal mismatch: "
                    f"expected={expected.get('prior_run_abnormal')}, "
                    f"actual={actual_prior_abnormal}"
                ),
            }
        )
    else:
        result.passed_checks.append("prior_run_abnormal matches expected")

    actual_auto_rerun_forbidden = bool(proposal.get("auto_rerun_forbidden"))
    if actual_auto_rerun_forbidden is not True:
        result.failed_checks.append(
            {
                "check_id": "row.auto_rerun_forbidden_violation",
                "message": (
                    f"auto_rerun_forbidden must be true on every restore "
                    f"proposal; observed false on {fixture_ref}"
                ),
            }
        )
    else:
        result.passed_checks.append("auto_rerun_forbidden is true")

    expected_counts = ensure_dict(
        expected.get("counts"), f"{drill_row_id}.expected.counts"
    )
    actual_counts = ensure_dict(
        proposal.get("counts"), f"{fixture_ref}.counts"
    )
    for key, expected_value in expected_counts.items():
        actual_value = actual_counts.get(key)
        if actual_value != expected_value:
            result.failed_checks.append(
                {
                    "check_id": "row.counts_mismatch",
                    "message": (
                        f"counts.{key} mismatch: expected={expected_value}, "
                        f"actual={actual_value}"
                    ),
                }
            )
        else:
            result.passed_checks.append(
                f"counts.{key} matches expected ({expected_value})"
            )

    # Side-effectful surfaces must stay blocked_side_effectful.
    side_effectful_required = ensure_list(
        expected.get("side_effectful_pane_plans_required", []),
        f"{drill_row_id}.expected.side_effectful_pane_plans_required",
    )
    pane_plans = ensure_list(
        proposal.get("pane_plans", []), f"{fixture_ref}.pane_plans"
    )
    for required in side_effectful_required:
        required = ensure_dict(
            required,
            f"{drill_row_id}.expected.side_effectful_pane_plans_required[]",
        )
        role = ensure_str(required.get("surface_role"), "surface_role")
        plan_kind = ensure_str(required.get("plan_kind"), "plan_kind")
        if role not in vocab.side_effectful_surface_role:
            result.failed_checks.append(
                {
                    "check_id": "row.side_effectful_surface_role_vocab_violation",
                    "message": (
                        f"surface_role '{role}' is not in the side-effectful "
                        f"vocabulary"
                    ),
                }
            )
            continue
        match = next(
            (p for p in pane_plans if p.get("surface_role") == role),
            None,
        )
        if match is None:
            result.failed_checks.append(
                {
                    "check_id": "row.side_effectful_pane_missing",
                    "message": (
                        f"required side-effectful pane (role={role}) not "
                        f"present on {fixture_ref}"
                    ),
                }
            )
            continue
        actual_kind = match.get("plan_kind")
        if actual_kind != plan_kind:
            result.failed_checks.append(
                {
                    "check_id": "row.side_effectful_pane_plan_violation",
                    "message": (
                        f"side-effectful pane (role={role}) must be "
                        f"plan_kind={plan_kind}; observed plan_kind={actual_kind}"
                    ),
                }
            )
        else:
            result.passed_checks.append(
                f"side-effectful pane (role={role}) is {plan_kind}"
            )

    # Live skeletons declared by the row must show up.
    live_skeleton_required = ensure_list(
        expected.get("live_skeleton_pane_plans_required", []),
        f"{drill_row_id}.expected.live_skeleton_pane_plans_required",
    )
    for required in live_skeleton_required:
        required = ensure_dict(
            required, f"{drill_row_id}.expected.live_skeleton_pane_plans_required[]"
        )
        role = ensure_str(required.get("surface_role"), "surface_role")
        plan_kind = ensure_str(required.get("plan_kind"), "plan_kind")
        match = next(
            (p for p in pane_plans if p.get("surface_role") == role),
            None,
        )
        if match is None:
            result.failed_checks.append(
                {
                    "check_id": "row.live_skeleton_pane_missing",
                    "message": (
                        f"required live-skeleton pane (role={role}) not "
                        f"present on {fixture_ref}"
                    ),
                }
            )
            continue
        if match.get("plan_kind") != plan_kind:
            result.failed_checks.append(
                {
                    "check_id": "row.live_skeleton_pane_plan_violation",
                    "message": (
                        f"live-skeleton pane (role={role}) must be "
                        f"plan_kind={plan_kind}; observed plan_kind={match.get('plan_kind')}"
                    ),
                }
            )
        else:
            result.passed_checks.append(
                f"live-skeleton pane (role={role}) is {plan_kind}"
            )

    # Dirty-buffer entries: closed-vocabulary checks plus required posture.
    actual_entries = ensure_list(
        proposal.get("dirty_buffer_entries", []),
        f"{fixture_ref}.dirty_buffer_entries",
    )
    for entry in actual_entries:
        entry = ensure_dict(entry, f"{fixture_ref}.dirty_buffer_entries[]")
        if entry.get("frame_integrity") not in vocab.frame_integrity_state:
            result.failed_checks.append(
                {
                    "check_id": "row.frame_integrity_vocab_violation",
                    "message": (
                        f"frame_integrity '{entry.get('frame_integrity')}' is "
                        f"not in matrix vocabulary"
                    ),
                }
            )
        if entry.get("replay_posture") not in vocab.replay_posture:
            result.failed_checks.append(
                {
                    "check_id": "row.replay_posture_vocab_violation",
                    "message": (
                        f"replay_posture '{entry.get('replay_posture')}' is "
                        f"not in matrix vocabulary"
                    ),
                }
            )
        if entry.get("recommended_choice") not in vocab.guided_choice:
            result.failed_checks.append(
                {
                    "check_id": "row.guided_choice_vocab_violation",
                    "message": (
                        f"recommended_choice '{entry.get('recommended_choice')}' "
                        f"is not in matrix vocabulary"
                    ),
                }
            )

    expected_entries = ensure_list(
        expected.get("dirty_buffer_entries_required", []),
        f"{drill_row_id}.expected.dirty_buffer_entries_required",
    )
    for required in expected_entries:
        required = ensure_dict(
            required, f"{drill_row_id}.expected.dirty_buffer_entries_required[]"
        )
        match = next(
            (
                e
                for e in actual_entries
                if e.get("frame_integrity") == required.get("frame_integrity")
            ),
            None,
        )
        if match is None:
            result.failed_checks.append(
                {
                    "check_id": "row.dirty_buffer_entry_missing",
                    "message": (
                        f"required dirty_buffer_entry with frame_integrity="
                        f"{required.get('frame_integrity')} not present on "
                        f"{fixture_ref}"
                    ),
                }
            )
            continue
        for key in ("replay_posture", "recommended_choice"):
            if match.get(key) != required.get(key):
                result.failed_checks.append(
                    {
                        "check_id": "row.dirty_buffer_entry_posture_mismatch",
                        "message": (
                            f"dirty_buffer_entry.{key} mismatch: expected="
                            f"{required.get(key)}, actual={match.get(key)}"
                        ),
                    }
                )

    # Downgrade triggers — closed vocabulary plus expected set.
    actual_triggers = ensure_list(
        proposal.get("downgrade_triggers", []),
        f"{fixture_ref}.downgrade_triggers",
    )
    for trig in actual_triggers:
        if trig not in vocab.downgrade_trigger:
            result.failed_checks.append(
                {
                    "check_id": "row.downgrade_trigger_vocab_violation",
                    "message": (
                        f"downgrade_trigger '{trig}' is not in matrix "
                        f"vocabulary"
                    ),
                }
            )
    expected_triggers = ensure_list(
        expected.get("downgrade_triggers", []),
        f"{drill_row_id}.expected.downgrade_triggers",
    )
    if sorted(actual_triggers) != sorted(expected_triggers):
        result.failed_checks.append(
            {
                "check_id": "row.downgrade_trigger_missing"
                if expected_triggers
                else "row.downgrade_trigger_unexpected",
                "message": (
                    f"downgrade_triggers mismatch: expected="
                    f"{sorted(expected_triggers)}, actual="
                    f"{sorted(actual_triggers)}"
                ),
            }
        )
    else:
        result.passed_checks.append(
            f"downgrade_triggers matches expected "
            f"({sorted(expected_triggers) if expected_triggers else '[]'})"
        )

    result.diagnostics["proposal_fixture_ref"] = fixture_ref
    result.diagnostics["observed_restore_class"] = actual_class
    result.diagnostics["observed_counts"] = actual_counts
    result.diagnostics["observed_downgrade_triggers"] = actual_triggers
    result.diagnostics["force_forced_input_applied"] = force_forced_input
    return result


# ---- safe-mode / crash-loop row replay -----------------------------------


def apply_safe_mode_forced_input(
    record: dict[str, Any], forced_input: dict[str, Any]
) -> dict[str, Any]:
    mutated = json.loads(json.dumps(record))

    if "drop_first_class_offer" in forced_input:
        target = forced_input["drop_first_class_offer"]
        mutated["offers"] = [
            o for o in mutated["offers"] if o["offer_key"] != target
        ]

    if "widen_disabled_capability_drop" in forced_input:
        target = forced_input["widen_disabled_capability_drop"]
        profile = mutated.get("safe_mode_profile", {})
        profile["disabled_or_narrowed_capabilities"] = [
            c for c in profile.get("disabled_or_narrowed_capabilities", []) if c != target
        ]

    return mutated


def replay_safe_mode_row(
    row: dict[str, Any],
    vocab: MatrixVocab,
    force_forced_input: bool,
) -> RowResult:
    drill_row_id = ensure_str(row.get("drill_row_id"), "row.drill_row_id")
    result = RowResult(
        drill_row_id=drill_row_id, drill_kind="safe_mode_crash_loop_projection"
    )

    safe_mode_input = ensure_dict(
        row.get("safe_mode_input"), f"{drill_row_id}.safe_mode_input"
    )
    reason_class = ensure_str(
        safe_mode_input.get("reason_class"),
        f"{drill_row_id}.safe_mode_input.reason_class",
    )
    if reason_class not in vocab.safe_mode_entry_reason:
        result.failed_checks.append(
            {
                "check_id": "row.safe_mode_entry_reason_vocab_violation",
                "message": (
                    f"reason_class '{reason_class}' is not in matrix vocabulary"
                ),
            }
        )
    crash_loop_reason = ensure_str(
        safe_mode_input.get("crash_loop_reason_class"),
        f"{drill_row_id}.safe_mode_input.crash_loop_reason_class",
    )

    record = project_crash_loop_containment(crash_loop_reason)
    if force_forced_input:
        forced = ensure_dict(
            row.get("failure_drill", {}).get("forced_input"),
            f"{drill_row_id}.failure_drill.forced_input",
        )
        record = apply_safe_mode_forced_input(record, forced)

    expected = ensure_dict(row.get("expected"), f"{drill_row_id}.expected")
    expected_profile = ensure_dict(
        expected.get("safe_mode_profile"),
        f"{drill_row_id}.expected.safe_mode_profile",
    )
    expected_containment = ensure_dict(
        expected.get("crash_loop_containment"),
        f"{drill_row_id}.expected.crash_loop_containment",
    )

    profile = record["safe_mode_profile"]

    # Profile invariants.
    if profile["entry_reason_class"] != expected_profile["entry_reason_class"]:
        result.failed_checks.append(
            {
                "check_id": "row.safe_mode_entry_reason_mismatch",
                "message": (
                    f"safe_mode_profile.entry_reason_class mismatch: "
                    f"expected={expected_profile['entry_reason_class']}, "
                    f"actual={profile['entry_reason_class']}"
                ),
            }
        )
    else:
        result.passed_checks.append("safe_mode_profile.entry_reason_class matches")

    for boolean_key in (
        "auto_restore_forbidden",
        "entry_visible_and_logged",
        "trust_widening_forbidden_without_review",
    ):
        if profile.get(boolean_key) is not True:
            result.failed_checks.append(
                {
                    "check_id": "row.safe_mode_invariant_violation",
                    "message": (
                        f"safe_mode_profile.{boolean_key} must be true; "
                        f"observed {profile.get(boolean_key)!r}"
                    ),
                }
            )
        else:
            result.passed_checks.append(
                f"safe_mode_profile.{boolean_key} is true"
            )

    if profile.get("entry_confirmation_required") != expected_profile.get(
        "entry_confirmation_required"
    ):
        result.failed_checks.append(
            {
                "check_id": "row.safe_mode_entry_confirmation_required_mismatch",
                "message": (
                    f"safe_mode_profile.entry_confirmation_required mismatch: "
                    f"expected={expected_profile.get('entry_confirmation_required')}, "
                    f"actual={profile.get('entry_confirmation_required')}"
                ),
            }
        )

    for label, key in (
        ("required_disabled_capabilities", "disabled_or_narrowed_capabilities"),
        ("required_preserved_state_classes", "preserved_state_classes"),
        ("required_next_options", "next_options_after_entry"),
    ):
        required = set(expected_profile.get(label, []))
        actual = set(profile.get(key, []))
        missing = required - actual
        if missing:
            result.failed_checks.append(
                {
                    "check_id": f"row.safe_mode_{label}_missing",
                    "message": (
                        f"safe_mode_profile.{key} missing required tokens: "
                        f"{sorted(missing)}"
                    ),
                }
            )
        else:
            result.passed_checks.append(
                f"safe_mode_profile.{key} carries every required token"
            )

    if (
        profile.get("safe_mode_enter_command_id")
        != expected_profile.get("safe_mode_enter_command_id")
    ):
        result.failed_checks.append(
            {
                "check_id": "row.safe_mode_enter_command_id_mismatch",
                "message": (
                    f"safe_mode_profile.safe_mode_enter_command_id mismatch: "
                    f"expected={expected_profile.get('safe_mode_enter_command_id')}, "
                    f"actual={profile.get('safe_mode_enter_command_id')}"
                ),
            }
        )
    if (
        profile.get("safe_mode_exit_command_id")
        != expected_profile.get("safe_mode_exit_command_id")
    ):
        result.failed_checks.append(
            {
                "check_id": "row.safe_mode_exit_command_id_mismatch",
                "message": (
                    f"safe_mode_profile.safe_mode_exit_command_id mismatch: "
                    f"expected={expected_profile.get('safe_mode_exit_command_id')}, "
                    f"actual={profile.get('safe_mode_exit_command_id')}"
                ),
            }
        )

    # Crash-loop containment invariants.
    if record.get("auto_rerun_forbidden") is not True:
        result.failed_checks.append(
            {
                "check_id": "row.crash_loop_auto_rerun_forbidden_violation",
                "message": "crash_loop_containment.auto_rerun_forbidden must be true",
            }
        )
    if record.get("never_deletes_state") is not True:
        result.failed_checks.append(
            {
                "check_id": "row.crash_loop_never_deletes_state_violation",
                "message": "crash_loop_containment.never_deletes_state must be true",
            }
        )

    if record.get("reason_class") != expected_containment.get("reason_class"):
        result.failed_checks.append(
            {
                "check_id": "row.crash_loop_reason_class_mismatch",
                "message": (
                    f"crash_loop_containment.reason_class mismatch: "
                    f"expected={expected_containment.get('reason_class')}, "
                    f"actual={record.get('reason_class')}"
                ),
            }
        )

    offers = record.get("offers", [])
    first_class_observed = {
        o["offer_key"] for o in offers if o.get("first_class") is True
    }
    gated_observed = {
        o["offer_key"] for o in offers if o.get("first_class") is False
    }

    required_first_class = set(expected_containment.get("required_first_class_offers", []))
    if not required_first_class.issubset(vocab.required_first_class_offers):
        result.failed_checks.append(
            {
                "check_id": "row.crash_loop_first_class_required_outside_matrix",
                "message": (
                    f"row declares required_first_class_offers outside the "
                    f"matrix-level required_first_class_crash_loop_offers list"
                ),
            }
        )
    missing_first_class = required_first_class - first_class_observed
    if missing_first_class:
        result.failed_checks.append(
            {
                "check_id": "row.crash_loop_first_class_offer_missing",
                "message": (
                    f"crash_loop_containment first-class offers missing: "
                    f"{sorted(missing_first_class)}"
                ),
            }
        )
    else:
        result.passed_checks.append(
            "crash_loop_containment exposes every required first-class offer"
        )

    required_gated = set(expected_containment.get("required_gated_offers", []))
    missing_gated = required_gated - gated_observed
    if missing_gated:
        result.failed_checks.append(
            {
                "check_id": "row.crash_loop_gated_offer_missing",
                "message": (
                    f"crash_loop_containment gated offers missing: "
                    f"{sorted(missing_gated)}"
                ),
            }
        )
    else:
        result.passed_checks.append(
            "crash_loop_containment exposes every required gated offer"
        )

    expected_command_ids = ensure_dict(
        expected_containment.get("offer_command_ids", {}),
        f"{drill_row_id}.expected.crash_loop_containment.offer_command_ids",
    )
    for offer_key, expected_cmd in expected_command_ids.items():
        if offer_key not in vocab.crash_loop_offer_key:
            result.failed_checks.append(
                {
                    "check_id": "row.crash_loop_offer_key_vocab_violation",
                    "message": (
                        f"offer_key '{offer_key}' is not in matrix vocabulary"
                    ),
                }
            )
            continue
        match = next((o for o in offers if o.get("offer_key") == offer_key), None)
        if match is None:
            result.failed_checks.append(
                {
                    "check_id": "row.crash_loop_offer_command_id_offer_missing",
                    "message": (
                        f"offer '{offer_key}' missing from crash_loop_containment"
                    ),
                }
            )
            continue
        if match.get("command_id") != expected_cmd:
            result.failed_checks.append(
                {
                    "check_id": "row.crash_loop_offer_command_id_mismatch",
                    "message": (
                        f"offer '{offer_key}' command_id mismatch: "
                        f"expected={expected_cmd}, actual={match.get('command_id')}"
                    ),
                }
            )

    result.diagnostics["projected_safe_mode_profile_keys"] = sorted(profile.keys())
    result.diagnostics["projected_first_class_offers"] = sorted(first_class_observed)
    result.diagnostics["projected_gated_offers"] = sorted(gated_observed)
    result.diagnostics["force_forced_input_applied"] = force_forced_input
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

    vocab_sources = ensure_dict(
        matrix.get("vocabulary_sources"), "matrix.vocabulary_sources"
    )
    for key, value in vocab_sources.items():
        ref = ensure_str(value, f"matrix.vocabulary_sources.{key}")
        if not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.vocabulary_sources.missing",
                    message=f"vocabulary_sources.{key} does not resolve: {ref}",
                    remediation="Fix the path or seed the referenced contract.",
                    ref=ref,
                )
            )

    hard_deps = ensure_list(
        matrix.get("hard_dependency_refs"), "matrix.hard_dependency_refs"
    )
    for idx, ref in enumerate(hard_deps):
        ref = ensure_str(ref, f"matrix.hard_dependency_refs[{idx}]")
        if not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.hard_dependency_refs.missing",
                    message=f"hard_dependency_refs[{idx}] does not exist: {ref}",
                    remediation="Fix the dependency path; the lane must consume live upstream surfaces.",
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
                message=f"validation_lane_ref base does not exist: {validation_lane_ref}",
                remediation="Point at a seeded QE lane registry entry.",
                ref=validation_lane_ref,
            )
        )

    vocab = load_matrix_vocab(matrix)

    rows = ensure_list(matrix.get("rows"), "matrix.rows")
    if not rows:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.rows.empty",
                message="matrix.rows must declare at least five rows",
                remediation="Seed the required scenario rows.",
            )
        )

    forced_drill_row: dict[str, Any] | None = None
    if args.force_drill is not None:
        target_id = args.force_drill.strip()
        for row in rows:
            row_dict = ensure_dict(row, "matrix.rows[]")
            failure_drill = row_dict.get("failure_drill") or {}
            if (
                isinstance(failure_drill, dict)
                and failure_drill.get("drill_id") == target_id
            ):
                forced_drill_row = row_dict
                break
        if forced_drill_row is None:
            raise SystemExit(
                f"--force-drill {target_id!r} does not match any row's failure_drill.drill_id"
            )

    row_results: list[RowResult] = []
    seen_ids: set[str] = set()
    seen_scenario_classes: set[str] = set()
    for row in rows:
        row = ensure_dict(row, "matrix.rows[]")
        drill_row_id = ensure_str(row.get("drill_row_id"), "row.drill_row_id")
        for cls in ensure_list(
            row.get("scenario_classes"), f"{drill_row_id}.scenario_classes"
        ):
            cls = ensure_str(cls, f"{drill_row_id}.scenario_classes[]")
            seen_scenario_classes.add(cls)

        force_forced_input = (
            forced_drill_row is not None
            and row is forced_drill_row
        )

        drill_kind = ensure_str(row.get("drill_kind"), f"{drill_row_id}.drill_kind")
        if drill_kind == "restore_proposal":
            result = replay_proposal_row(
                repo_root, row, vocab, force_forced_input
            )
        elif drill_kind == "safe_mode_crash_loop_projection":
            result = replay_safe_mode_row(row, vocab, force_forced_input)
        else:
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.row.unknown_drill_kind",
                    message=f"unknown drill_kind '{drill_kind}' on row {drill_row_id}",
                    remediation="Use restore_proposal or safe_mode_crash_loop_projection.",
                    ref=drill_row_id,
                )
            )
            continue
        row_results.append(result)
        if result.drill_row_id in seen_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.rows.duplicate_id",
                    message=f"duplicate drill_row_id: {result.drill_row_id}",
                    remediation="drill_row_ids must be unique.",
                    ref=result.drill_row_id,
                )
            )
        seen_ids.add(result.drill_row_id)

    missing_coverage = vocab.required_scenario_class_coverage - seen_scenario_classes
    if missing_coverage:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.coverage.missing_required_classes",
                message=(
                    "matrix must seed at least one row each for "
                    f"{sorted(vocab.required_scenario_class_coverage)}; missing: "
                    f"{sorted(missing_coverage)}"
                ),
                remediation="Add the missing rows so every required scenario class is exercised.",
            )
        )

    # Promote per-row failures into findings.
    for result in row_results:
        for failure in result.failed_checks:
            findings.append(
                Finding(
                    severity="error",
                    check_id=failure["check_id"],
                    message=f"{result.drill_row_id}: {failure['message']}",
                    remediation="Fix the row geometry, vocabulary, or expected truth so the drill replay holds.",
                    ref=result.drill_row_id,
                )
            )

    # Drill-mode evaluation: a successful drill reproduces the expected
    # check_id as a failure on the targeted row.
    drill_status: str | None = None
    drill_diagnostics: dict[str, Any] = {}
    if forced_drill_row is not None:
        expected_report = ensure_dict(
            forced_drill_row["failure_drill"].get("expected_report"),
            f"{forced_drill_row.get('drill_row_id')}.failure_drill.expected_report",
        )
        expected_check = ensure_str(
            expected_report.get("check_id"),
            "failure_drill.expected_report.check_id",
        )
        target_row = next(
            (r for r in row_results if r.drill_row_id == forced_drill_row["drill_row_id"]),
            None,
        )
        if target_row is None:
            drill_status = "FAIL"
            drill_diagnostics["error"] = "drill row produced no result"
        else:
            actual_check_ids = [c["check_id"] for c in target_row.failed_checks]
            drill_diagnostics["expected_check_id"] = expected_check
            drill_diagnostics["observed_check_ids"] = actual_check_ids
            if expected_check in actual_check_ids:
                drill_status = "REPRODUCED"
            else:
                drill_status = "DRILL_DID_NOT_REPRODUCE"
                findings.append(
                    Finding(
                        severity="error",
                        check_id="drill.expected_check_id_not_reproduced",
                        message=(
                            f"forced drill {args.force_drill}: expected check_id "
                            f"'{expected_check}' was not raised; observed: "
                            f"{actual_check_ids}"
                        ),
                        remediation=(
                            "Adjust the row's forced_input or the runner so the "
                            "drill reproduces the precise failing check."
                        ),
                        ref=forced_drill_row["drill_row_id"],
                    )
                )

    errors = [f for f in findings if f.severity == "error"]

    if forced_drill_row is not None:
        # Drill mode succeeds when the forced drill reproduced the
        # named check_id and there are no *other* errors. Other errors
        # still mean the drill is unstable, which we want to surface.
        drill_only_failure = (
            drill_status == "REPRODUCED"
            and len(errors) == sum(
                1
                for f in errors
                if f.ref == forced_drill_row["drill_row_id"]
            )
        )
        status = "DRILL_REPRODUCED" if drill_only_failure else "DRILL_FAILED"
    else:
        status = "PASS" if not errors else "FAIL"

    capture: dict[str, Any] = {
        "schema_version": 1,
        "capture_kind": "crash_restore_validation_capture",
        "captured_at": now_iso_z(),
        "owner_dri": ensure_str(matrix.get("owner_dri"), "matrix.owner_dri"),
        "matrix_ref": matrix_rel,
        "exact_build_identity_ref": args.build_identity,
        "command": (
            "python3 tests/recovery/m1_crash_restore_cases/run_crash_restore_cases.py --repo-root ."
        ),
        "status": status,
        "drill_status": drill_status,
        "force_drill_id": args.force_drill,
        "drill_diagnostics": drill_diagnostics,
        "required_scenario_class_coverage": sorted(
            vocab.required_scenario_class_coverage
        ),
        "observed_scenario_classes": sorted(seen_scenario_classes),
        "rows": [
            {
                "drill_row_id": r.drill_row_id,
                "drill_kind": r.drill_kind,
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

    report_path = repo_root / args.report
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text(
        json.dumps(capture, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )

    print(
        f"[crash-restore] {status} ({len(errors)} errors, "
        f"{len(findings) - len(errors)} warnings) — capture: {args.report}"
    )
    if drill_status is not None:
        print(f"[crash-restore] drill_status={drill_status}")
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        print(
            f"[crash-restore] {prefix} {finding.check_id}: {finding.message}{ref_suffix}"
        )
        print(f"[crash-restore]   remediation: {finding.remediation}")

    if forced_drill_row is not None:
        return 0 if status == "DRILL_REPRODUCED" else 1
    return 0 if status == "PASS" else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[crash-restore] interrupted", file=sys.stderr)
        sys.exit(130)
