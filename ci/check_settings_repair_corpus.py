#!/usr/bin/env python3
"""Validate the M3 settings-repair and wrong-scope-write conformance corpus.

Each fixture under fixtures/config/m3/settings_repair_corpus/ encodes
one settings-repair conformance case. The corpus must:

- Cover every required scenario class (user/profile/workspace scope
  confusion, locked policy values, stale sync/device data,
  imported-profile conflicts, partial migration fallout, Labs/experiment
  dependencies, support-center initiated repair suggestions, and the
  three negative axes: hidden broad reset refused, wrong-artifact write
  refused, silent policy override refused).
- Assert that every attempted write records target scope, selected
  artifact, checkpoint behavior, blocked-write reason, and resulting
  rollback path.
- Verify CLI/headless, UI, sync repair, and support export all compute
  the same winning scope and explain the same blocked-write result.
- Refuse hidden broad resets, wrong-artifact writes, silent policy
  overrides, and any collapse of policy-locked or unsupported repairs
  into generic ``reset settings`` guidance.
- Cross-check each anchor settings_repair_plan fixture so the corpus
  case cannot drift away from the plan it quotes.

Companion artifacts (safety report, wrong-scope-write matrix, reviewer
drills doc) are required to exist and to reference the corpus.
"""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any

from jsonschema import Draft202012Validator, FormatChecker


DEFAULT_SCHEMA_REL = "schemas/config/settings_repair_corpus_case.schema.json"
DEFAULT_FIXTURE_DIR_REL = "fixtures/config/m3/settings_repair_corpus"
DEFAULT_REPORT_REL = "artifacts/config/m3/settings_repair_safety_report.md"
DEFAULT_MATRIX_REL = "artifacts/config/m3/wrong_scope_write_matrix.json"
DEFAULT_DRILLS_DOC_REL = "docs/qe/m3/settings_repair_drills.md"
DEFAULT_PLAN_FIXTURE_DIR_REL = "fixtures/config/m3/settings_repair_and_reset"

REQUIRED_SCENARIO_CLASSES = {
    "user_profile_workspace_scope_confusion",
    "locked_policy_value_refused",
    "stale_sync_device_data",
    "imported_profile_conflict",
    "partial_migration_fallout",
    "labs_experiment_dependency",
    "support_center_initiated_repair",
    "hidden_broad_reset_refused",
    "wrong_artifact_write_refused",
    "silent_policy_override_refused",
}

REQUIRED_SURFACE_CLASSES = {
    "cli_headless",
    "ui_beta",
    "sync_repair",
    "support_export",
}

POLICY_DENIAL_TOKENS = {
    "denied_policy_owned",
    "denied_non_writable_scope",
    "denied_scope_broadening_refused",
}

NEGATIVE_SCENARIOS = {
    "hidden_broad_reset_refused",
    "wrong_artifact_write_refused",
    "silent_policy_override_refused",
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


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--schema", default=DEFAULT_SCHEMA_REL)
    parser.add_argument("--fixture-dir", default=DEFAULT_FIXTURE_DIR_REL)
    parser.add_argument(
        "--plan-fixture-dir", default=DEFAULT_PLAN_FIXTURE_DIR_REL
    )
    parser.add_argument("--report", default=DEFAULT_REPORT_REL)
    parser.add_argument("--matrix", default=DEFAULT_MATRIX_REL)
    parser.add_argument("--drills-doc", default=DEFAULT_DRILLS_DOC_REL)
    parser.add_argument("--report-json", default=None)
    return parser.parse_args()


def load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError as exc:
        raise SystemExit(f"missing JSON file: {path}") from exc
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON at {path}: {exc}") from exc


def schema_validate(
    schema: dict[str, Any], label: str, payload: dict[str, Any]
) -> list[Finding]:
    Draft202012Validator.check_schema(schema)
    validator = Draft202012Validator(schema, format_checker=FormatChecker())
    findings: list[Finding] = []
    for error in sorted(
        validator.iter_errors(payload), key=lambda item: list(item.path)
    ):
        path = ".".join(str(part) for part in error.path) or "<root>"
        findings.append(
            Finding(
                severity="error",
                check_id="settings_repair_corpus.schema.validation_failed",
                message=f"{label}: {path}: {error.message}",
                remediation=(
                    "Fix the corpus case so it validates against "
                    "schemas/config/settings_repair_corpus_case.schema.json."
                ),
                ref=label,
                details={"schema_path": list(error.schema_path)},
            )
        )
    return findings


def collect_records(fixture_dir: Path) -> dict[str, dict[str, Any]]:
    if not fixture_dir.exists():
        raise SystemExit(f"missing fixture dir: {fixture_dir}")
    records: dict[str, dict[str, Any]] = {}
    for path in sorted(fixture_dir.glob("*.json")):
        records[path.name] = load_json(path)
    if not records:
        raise SystemExit(f"no fixtures in {fixture_dir}")
    return records


def parity_token_for_verdict(
    verdict: str, blocked_reason_codes: list[str]
) -> str | None:
    if verdict != "denied":
        return verdict
    if "policy_owned_class" in blocked_reason_codes:
        return "denied_policy_owned"
    if "capability_dependency_unmet" in blocked_reason_codes:
        return "denied_capability_locked"
    if "retired_setting" in blocked_reason_codes:
        return "denied_retired_setting"
    if "managed_mode_only" in blocked_reason_codes:
        return "denied_managed_mode_only"
    if "non_writable_scope" in blocked_reason_codes:
        return "denied_non_writable_scope"
    if "unknown_setting" in blocked_reason_codes:
        return "denied_unknown_setting"
    if "scope_broadening_refused" in blocked_reason_codes:
        return "denied_scope_broadening_refused"
    if "adjacent_setting_refused" in blocked_reason_codes:
        return "denied_adjacent_setting_refused"
    return None


def cross_check_record(
    label: str,
    record: dict[str, Any],
    plan_fixture_dir: Path,
    repo_root: Path,
) -> list[Finding]:
    findings: list[Finding] = []
    expected = record.get("expected", {})
    parity = record.get("surface_parity", {})
    negative = record.get("negative_assertions", {})
    support = record.get("support_export", {})
    scenario_class = record.get("scenario_class")
    selected = record.get("selected_setting_ids") or []

    blocked_codes = list(expected.get("blocked_reason_codes") or [])
    locked_classes = list(expected.get("locked_classes") or [])
    verdict = expected.get("verdict")
    target_scope = expected.get("target_scope")
    target_scope_class = expected.get("target_scope_class")
    target_artifact_ref = expected.get("target_artifact_ref")
    rollback_present = expected.get("rollback_action_ref_present")
    checkpoint_required = expected.get("checkpoint_required")
    approval_required = expected.get("approval_required")
    hidden = expected.get("hidden_reset_guard", {})
    refused_ids = list(hidden.get("refused_setting_ids") or [])

    # Cross-check parity: every surface must agree on winning scope,
    # winning artifact ref, and blocked-write result token.
    parity_scope = parity.get("winning_scope_token")
    parity_artifact = parity.get("winning_artifact_ref")
    parity_blocked_token = parity.get("blocked_write_result_token")
    surfaces = parity.get("surfaces") or []
    surface_classes = {
        s.get("surface_class") for s in surfaces if isinstance(s, dict)
    }
    missing_surfaces = REQUIRED_SURFACE_CLASSES - surface_classes
    if missing_surfaces:
        findings.append(
            Finding(
                severity="error",
                check_id="settings_repair_corpus.surface_parity.surface_missing",
                message=(
                    f"{label} surface_parity.surfaces is missing "
                    f"{sorted(missing_surfaces)}"
                ),
                remediation=(
                    "Declare cli_headless, ui_beta, sync_repair, and "
                    "support_export entries so each surface agrees on the "
                    "winning scope and blocked-write result."
                ),
                ref=label,
            )
        )
    for surface in surfaces:
        if not isinstance(surface, dict):
            continue
        surface_class = surface.get("surface_class")
        if surface.get("winning_scope_token") != parity_scope:
            findings.append(
                Finding(
                    severity="error",
                    check_id="settings_repair_corpus.surface_parity.scope_drift",
                    message=(
                        f"{label} surface {surface_class} winning_scope_token "
                        f"{surface.get('winning_scope_token')!r} does not "
                        f"match parity winning scope {parity_scope!r}"
                    ),
                    remediation=(
                        "Every surface must compute the same winning scope; "
                        "do not let a surface diverge from the parity scope."
                    ),
                    ref=label,
                )
            )
        if surface.get("winning_artifact_ref") != parity_artifact:
            findings.append(
                Finding(
                    severity="error",
                    check_id="settings_repair_corpus.surface_parity.artifact_drift",
                    message=(
                        f"{label} surface {surface_class} winning_artifact_ref "
                        f"{surface.get('winning_artifact_ref')!r} does not "
                        f"match parity winning artifact {parity_artifact!r}"
                    ),
                    remediation=(
                        "Every surface must agree on the artifact ref the "
                        "write would land on."
                    ),
                    ref=label,
                )
            )
        if surface.get("blocked_write_result_token") != parity_blocked_token:
            findings.append(
                Finding(
                    severity="error",
                    check_id="settings_repair_corpus.surface_parity.blocked_result_drift",
                    message=(
                        f"{label} surface {surface_class} "
                        f"blocked_write_result_token "
                        f"{surface.get('blocked_write_result_token')!r} "
                        f"does not match parity result {parity_blocked_token!r}"
                    ),
                    remediation=(
                        "Every surface must explain the same blocked-write "
                        "result token."
                    ),
                    ref=label,
                )
            )
        if surface.get("uses_canonical_write_intent_pipeline") is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id=(
                        "settings_repair_corpus.surface_parity.private_pipeline"
                    ),
                    message=(
                        f"{label} surface {surface_class} does not route "
                        "through the canonical write-intent pipeline"
                    ),
                    remediation=(
                        "Every repair surface must reuse the canonical "
                        "write-intent pipeline; private 'broad reset' paths "
                        "are not allowed."
                    ),
                    ref=label,
                )
            )

    # Cross-check parity token matches expected verdict + blocked codes.
    derived_token = parity_token_for_verdict(verdict, blocked_codes)
    if derived_token is not None and parity_blocked_token != derived_token:
        findings.append(
            Finding(
                severity="error",
                check_id=(
                    "settings_repair_corpus.surface_parity.token_mismatch"
                ),
                message=(
                    f"{label} parity blocked_write_result_token "
                    f"{parity_blocked_token!r} does not match the token "
                    f"derived from expected verdict {verdict!r} and blocked "
                    f"reason codes {blocked_codes!r} (expected "
                    f"{derived_token!r})"
                ),
                remediation=(
                    "Pick the parity token that matches the expected verdict "
                    "and primary blocked-write reason."
                ),
                ref=label,
            )
        )

    # Artifact ref shape rules.
    if parity_artifact != target_artifact_ref:
        findings.append(
            Finding(
                severity="error",
                check_id=(
                    "settings_repair_corpus.parity.artifact_diverges_from_expected"
                ),
                message=(
                    f"{label} surface_parity.winning_artifact_ref "
                    f"{parity_artifact!r} does not match "
                    f"expected.target_artifact_ref {target_artifact_ref!r}"
                ),
                remediation=(
                    "Keep the parity artifact ref equal to the expected "
                    "target_artifact_ref so wrong-artifact writes cannot "
                    "hide between the two fields."
                ),
                ref=label,
            )
        )
    if (
        isinstance(target_artifact_ref, str)
        and not target_artifact_ref.startswith("settings://")
    ):
        findings.append(
            Finding(
                severity="error",
                check_id=(
                    "settings_repair_corpus.expected.artifact_ref_shape"
                ),
                message=(
                    f"{label} expected.target_artifact_ref "
                    f"{target_artifact_ref!r} does not start with settings://"
                ),
                remediation=(
                    "Use a settings:// artifact ref so the corpus can never "
                    "claim a write to an unrelated artifact."
                ),
                ref=label,
            )
        )

    # Denied verdict requires at least one blocked-reason code.
    if verdict == "denied" and not blocked_codes:
        findings.append(
            Finding(
                severity="error",
                check_id="settings_repair_corpus.verdict.denied_without_reason",
                message=(
                    f"{label} expected.verdict is denied but "
                    "blocked_reason_codes is empty"
                ),
                remediation=(
                    "Every denied plan must record at least one typed "
                    "blocked-write reason."
                ),
                ref=label,
            )
        )
    if verdict == "awaiting_checkpoint" and "checkpoint_missing" not in blocked_codes:
        findings.append(
            Finding(
                severity="error",
                check_id=(
                    "settings_repair_corpus.verdict.checkpoint_missing_reason"
                ),
                message=(
                    f"{label} expected.verdict is awaiting_checkpoint but "
                    "blocked_reason_codes does not include checkpoint_missing"
                ),
                remediation=(
                    "Awaiting-checkpoint plans must surface the missing "
                    "checkpoint as a typed blocked-write reason."
                ),
                ref=label,
            )
        )
    if verdict == "awaiting_approval" and "approval_missing" not in blocked_codes:
        findings.append(
            Finding(
                severity="error",
                check_id=(
                    "settings_repair_corpus.verdict.approval_missing_reason"
                ),
                message=(
                    f"{label} expected.verdict is awaiting_approval but "
                    "blocked_reason_codes does not include approval_missing"
                ),
                remediation=(
                    "Awaiting-approval plans must surface the missing "
                    "approval ticket as a typed blocked-write reason."
                ),
                ref=label,
            )
        )

    # Rollback action ref must be absent for denied plans.
    if verdict == "denied" and rollback_present:
        findings.append(
            Finding(
                severity="error",
                check_id="settings_repair_corpus.rollback.present_for_denied",
                message=(
                    f"{label} expected.rollback_action_ref_present is true "
                    "but expected.verdict is denied"
                ),
                remediation=(
                    "Denied plans must not expose a rollback action; the "
                    "write was never staged."
                ),
                ref=label,
            )
        )
    if (
        verdict not in {"denied"}
        and not rollback_present
    ):
        findings.append(
            Finding(
                severity="error",
                check_id="settings_repair_corpus.rollback.missing_for_apply_path",
                message=(
                    f"{label} expected.verdict {verdict!r} requires "
                    "expected.rollback_action_ref_present = true"
                ),
                remediation=(
                    "Any non-denied plan must expose a rollback action ref "
                    "the user can route to after apply."
                ),
                ref=label,
            )
        )

    # Policy-owned target scope class implies non-writable + locked class.
    if target_scope_class == "policy_owned":
        if "policy_owned_class" not in locked_classes:
            findings.append(
                Finding(
                    severity="error",
                    check_id=(
                        "settings_repair_corpus.policy.locked_class_missing"
                    ),
                    message=(
                        f"{label} target_scope_class is policy_owned but "
                        "locked_classes does not include policy_owned_class"
                    ),
                    remediation=(
                        "Policy-owned plans must surface "
                        "policy_owned_class in locked_classes."
                    ),
                    ref=label,
                )
            )
        if "policy_owned_class" not in blocked_codes:
            findings.append(
                Finding(
                    severity="error",
                    check_id=(
                        "settings_repair_corpus.policy.blocked_code_missing"
                    ),
                    message=(
                        f"{label} target_scope_class is policy_owned but "
                        "blocked_reason_codes does not include "
                        "policy_owned_class"
                    ),
                    remediation=(
                        "Policy-owned plans must refuse writes with the "
                        "policy_owned_class blocked reason."
                    ),
                    ref=label,
                )
            )

    # Hidden-reset guard semantics.
    if hidden.get("would_touch_adjacent_settings") and not refused_ids:
        findings.append(
            Finding(
                severity="error",
                check_id=(
                    "settings_repair_corpus.hidden_reset.refused_ids_missing"
                ),
                message=(
                    f"{label} hidden_reset_guard would_touch_adjacent_settings "
                    "is true but refused_setting_ids is empty"
                ),
                remediation=(
                    "Refused adjacent rows must be enumerated in "
                    "refused_setting_ids."
                ),
                ref=label,
            )
        )
    if (
        refused_ids
        and "adjacent_setting_refused" not in blocked_codes
    ):
        findings.append(
            Finding(
                severity="error",
                check_id=(
                    "settings_repair_corpus.hidden_reset.adjacent_code_missing"
                ),
                message=(
                    f"{label} refused_setting_ids contains rows but "
                    "blocked_reason_codes does not include "
                    "adjacent_setting_refused"
                ),
                remediation=(
                    "Refused adjacent rows must surface as the "
                    "adjacent_setting_refused blocked-write reason."
                ),
                ref=label,
            )
        )
    for refused in refused_ids:
        if refused in selected:
            findings.append(
                Finding(
                    severity="error",
                    check_id=(
                        "settings_repair_corpus.hidden_reset.refused_in_selection"
                    ),
                    message=(
                        f"{label} refused_setting_ids contains {refused!r} "
                        "which is also in selected_setting_ids"
                    ),
                    remediation=(
                        "Refused setting ids must fall outside the user "
                        "selection."
                    ),
                    ref=label,
                )
            )

    # Negative-axis assertions are required on every case.
    for axis in (
        "refuses_hidden_broad_reset",
        "refuses_wrong_artifact_write",
        "refuses_silent_policy_override",
        "refuses_generic_reset_collapse",
    ):
        if negative.get(axis) is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"settings_repair_corpus.negative.{axis}",
                    message=(
                        f"{label} negative_assertions.{axis} must be true"
                    ),
                    remediation=(
                        "Every corpus case must refuse hidden broad resets, "
                        "wrong-artifact writes, silent policy overrides, and "
                        "collapses into generic 'reset settings' guidance."
                    ),
                    ref=label,
                )
            )

    # Scenario-specific assertions.
    if scenario_class == "locked_policy_value_refused":
        if target_scope_class != "policy_owned":
            findings.append(
                Finding(
                    severity="error",
                    check_id=(
                        "settings_repair_corpus.scenario.policy_target_required"
                    ),
                    message=(
                        f"{label} scenario_class locked_policy_value_refused "
                        "requires target_scope_class = policy_owned"
                    ),
                    remediation=(
                        "Aim the plan at the admin_policy_narrowing scope so "
                        "the policy-owned class is invoked."
                    ),
                    ref=label,
                )
            )
    if scenario_class == "hidden_broad_reset_refused":
        if not refused_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id=(
                        "settings_repair_corpus.scenario.hidden_broad_reset_no_refused"
                    ),
                    message=(
                        f"{label} hidden_broad_reset_refused must list "
                        "refused_setting_ids"
                    ),
                    remediation=(
                        "Hidden broad reset cases must enumerate refused "
                        "adjacent rows."
                    ),
                    ref=label,
                )
            )
        if verdict != "denied":
            findings.append(
                Finding(
                    severity="error",
                    check_id=(
                        "settings_repair_corpus.scenario.hidden_broad_reset_not_denied"
                    ),
                    message=(
                        f"{label} hidden_broad_reset_refused must have "
                        "verdict = denied"
                    ),
                    remediation=(
                        "Hidden broad reset cases must refuse the plan."
                    ),
                    ref=label,
                )
            )
    if scenario_class == "wrong_artifact_write_refused":
        if "non_writable_scope" not in blocked_codes and "scope_broadening_refused" not in blocked_codes:
            findings.append(
                Finding(
                    severity="error",
                    check_id=(
                        "settings_repair_corpus.scenario.wrong_artifact_no_code"
                    ),
                    message=(
                        f"{label} wrong_artifact_write_refused must surface "
                        "either non_writable_scope or scope_broadening_refused"
                    ),
                    remediation=(
                        "Wrong-artifact write cases must explicitly refuse "
                        "the non-writable scope or broadened scope."
                    ),
                    ref=label,
                )
            )
        if verdict != "denied":
            findings.append(
                Finding(
                    severity="error",
                    check_id=(
                        "settings_repair_corpus.scenario.wrong_artifact_not_denied"
                    ),
                    message=(
                        f"{label} wrong_artifact_write_refused must have "
                        "verdict = denied"
                    ),
                    remediation=(
                        "Wrong-artifact write cases must refuse the plan."
                    ),
                    ref=label,
                )
            )
    if scenario_class == "silent_policy_override_refused":
        if parity_blocked_token not in POLICY_DENIAL_TOKENS:
            findings.append(
                Finding(
                    severity="error",
                    check_id=(
                        "settings_repair_corpus.scenario.silent_policy_token"
                    ),
                    message=(
                        f"{label} silent_policy_override_refused parity "
                        f"blocked_write_result_token {parity_blocked_token!r} "
                        "is not a policy denial token"
                    ),
                    remediation=(
                        "Silent policy override cases must use a policy "
                        "denial parity token "
                        f"({sorted(POLICY_DENIAL_TOKENS)})."
                    ),
                    ref=label,
                )
            )
    if scenario_class == "partial_migration_fallout":
        if expected.get("action_class") != "revert_migration_step":
            findings.append(
                Finding(
                    severity="error",
                    check_id=(
                        "settings_repair_corpus.scenario.migration_action"
                    ),
                    message=(
                        f"{label} partial_migration_fallout must use "
                        "action_class = revert_migration_step"
                    ),
                    remediation=(
                        "Partial migration fallout cases must revert a "
                        "specific migration step."
                    ),
                    ref=label,
                )
            )
        if checkpoint_required is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id=(
                        "settings_repair_corpus.scenario.migration_checkpoint"
                    ),
                    message=(
                        f"{label} partial_migration_fallout must set "
                        "checkpoint_required = true"
                    ),
                    remediation=(
                        "Revert-migration-step plans must preserve a "
                        "rollback checkpoint before apply."
                    ),
                    ref=label,
                )
            )
    if scenario_class == "imported_profile_conflict":
        if expected.get("action_class") != "reapply_imported_profile_fragment":
            findings.append(
                Finding(
                    severity="error",
                    check_id=(
                        "settings_repair_corpus.scenario.import_action"
                    ),
                    message=(
                        f"{label} imported_profile_conflict must use "
                        "action_class = reapply_imported_profile_fragment"
                    ),
                    remediation=(
                        "Imported-profile conflict cases must re-apply one "
                        "fragment from the imported profile artifact."
                    ),
                    ref=label,
                )
            )
    if scenario_class == "labs_experiment_dependency":
        if "capability_dependency_unmet" not in blocked_codes:
            findings.append(
                Finding(
                    severity="error",
                    check_id=(
                        "settings_repair_corpus.scenario.labs_capability_missing"
                    ),
                    message=(
                        f"{label} labs_experiment_dependency must surface "
                        "capability_dependency_unmet"
                    ),
                    remediation=(
                        "Labs/experiment dependency cases must refuse the "
                        "plan with capability_dependency_unmet."
                    ),
                    ref=label,
                )
            )
        if "capability_locked" not in locked_classes:
            findings.append(
                Finding(
                    severity="error",
                    check_id=(
                        "settings_repair_corpus.scenario.labs_locked_class_missing"
                    ),
                    message=(
                        f"{label} labs_experiment_dependency must surface "
                        "capability_locked in locked_classes"
                    ),
                    remediation=(
                        "Labs/experiment dependency cases must mark the "
                        "capability_locked class on the plan."
                    ),
                    ref=label,
                )
            )
    if scenario_class == "stale_sync_device_data":
        if not approval_required and verdict not in {"awaiting_checkpoint", "denied"} and not checkpoint_required:
            findings.append(
                Finding(
                    severity="error",
                    check_id=(
                        "settings_repair_corpus.scenario.stale_sync_safety_posture"
                    ),
                    message=(
                        f"{label} stale_sync_device_data must require a "
                        "checkpoint or an approval ticket before apply"
                    ),
                    remediation=(
                        "Stale sync/device data cases must keep the "
                        "checkpoint/approval posture explicit; never apply "
                        "blindly."
                    ),
                    ref=label,
                )
            )
    if scenario_class == "support_center_initiated_repair":
        if support.get("preserves_user_decision") is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id=(
                        "settings_repair_corpus.scenario.support_center_decision"
                    ),
                    message=(
                        f"{label} support_center_initiated_repair must "
                        "preserve the user decision in the support export"
                    ),
                    remediation=(
                        "Support-center initiated repair cases must record "
                        "whether the user accepted, declined, or withdrew "
                        "the plan."
                    ),
                    ref=label,
                )
            )

    # Support-export posture.
    if support.get("raw_secret_export_allowed") is not False:
        findings.append(
            Finding(
                severity="error",
                check_id=(
                    "settings_repair_corpus.support_export.raw_secrets"
                ),
                message=(
                    f"{label} support_export.raw_secret_export_allowed must "
                    "be false"
                ),
                remediation=(
                    "Support exports must never carry raw secret material."
                ),
                ref=label,
            )
        )
    if support.get("replays_local_write_intent") is not True:
        findings.append(
            Finding(
                severity="error",
                check_id=(
                    "settings_repair_corpus.support_export.replays_intent"
                ),
                message=(
                    f"{label} support_export.replays_local_write_intent must "
                    "be true"
                ),
                remediation=(
                    "Every support export must replay the same write intent "
                    "the user saw locally."
                ),
                ref=label,
            )
        )
    if support.get("replays_local_repair_outcome") is not True:
        findings.append(
            Finding(
                severity="error",
                check_id=(
                    "settings_repair_corpus.support_export.replays_outcome"
                ),
                message=(
                    f"{label} support_export.replays_local_repair_outcome "
                    "must be true"
                ),
                remediation=(
                    "Every support export must replay the same repair "
                    "outcome the user saw locally."
                ),
                ref=label,
            )
        )

    # Cross-check anchor plan fixture if seed_plan_fixture_ref is set.
    seed = record.get("seed_plan_fixture_ref")
    if isinstance(seed, str) and seed:
        plan_path = repo_root / seed
        if not plan_path.exists():
            findings.append(
                Finding(
                    severity="error",
                    check_id=(
                        "settings_repair_corpus.seed.plan_missing"
                    ),
                    message=(
                        f"{label} seed_plan_fixture_ref {seed!r} does not "
                        "point to an existing fixture"
                    ),
                    remediation=(
                        "Cite a settings_repair_plan fixture that exists in "
                        "the repo."
                    ),
                    ref=label,
                )
            )
        else:
            plan = load_json(plan_path)
            if not isinstance(plan, dict):
                return findings
            findings.extend(
                cross_check_seed_plan(label, expected, parity, plan, seed)
            )

    return findings


def cross_check_seed_plan(
    label: str,
    expected: dict[str, Any],
    parity: dict[str, Any],
    plan: dict[str, Any],
    seed_ref: str,
) -> list[Finding]:
    findings: list[Finding] = []
    pairs = (
        ("action_class", expected.get("action_class"), plan.get("action_class")),
        ("target_scope", expected.get("target_scope"), plan.get("target_scope")),
        (
            "target_scope_class",
            expected.get("target_scope_class"),
            plan.get("target_scope_class"),
        ),
        (
            "target_artifact_ref",
            expected.get("target_artifact_ref"),
            plan.get("target_artifact_ref"),
        ),
        ("verdict", expected.get("verdict"), plan.get("verdict")),
        (
            "checkpoint_required",
            expected.get("checkpoint_required"),
            plan.get("checkpoint_required"),
        ),
        (
            "approval_required",
            expected.get("approval_required"),
            plan.get("approval_required"),
        ),
    )
    for field_name, expected_value, plan_value in pairs:
        if expected_value != plan_value:
            findings.append(
                Finding(
                    severity="error",
                    check_id=(
                        f"settings_repair_corpus.seed_plan.{field_name}_drift"
                    ),
                    message=(
                        f"{label} expected.{field_name} {expected_value!r} "
                        f"does not match anchor plan {seed_ref} "
                        f"{field_name} = {plan_value!r}"
                    ),
                    remediation=(
                        "Keep the corpus case in sync with its anchor "
                        "settings_repair_plan fixture."
                    ),
                    ref=label,
                )
            )

    plan_rollback_present = plan.get("rollback_action_ref") is not None
    if expected.get("rollback_action_ref_present") != plan_rollback_present:
        findings.append(
            Finding(
                severity="error",
                check_id=(
                    "settings_repair_corpus.seed_plan.rollback_drift"
                ),
                message=(
                    f"{label} expected.rollback_action_ref_present "
                    f"{expected.get('rollback_action_ref_present')!r} does "
                    f"not match anchor plan rollback presence "
                    f"{plan_rollback_present!r}"
                ),
                remediation=(
                    "Mirror the rollback action ref presence from the anchor "
                    "plan."
                ),
                ref=label,
            )
        )

    plan_blocked_codes = sorted(
        {reason.get("code") for reason in plan.get("blocked_write_reasons") or []}
    )
    expected_blocked_codes = sorted(expected.get("blocked_reason_codes") or [])
    if plan_blocked_codes != expected_blocked_codes:
        findings.append(
            Finding(
                severity="error",
                check_id=(
                    "settings_repair_corpus.seed_plan.blocked_codes_drift"
                ),
                message=(
                    f"{label} expected.blocked_reason_codes "
                    f"{expected_blocked_codes!r} does not match anchor plan "
                    f"blocked-write reason codes {plan_blocked_codes!r}"
                ),
                remediation=(
                    "Mirror the typed blocked-write reasons from the anchor "
                    "plan; do not invent new codes in the corpus."
                ),
                ref=label,
            )
        )

    plan_locked = sorted(plan.get("locked_classes") or [])
    expected_locked = sorted(expected.get("locked_classes") or [])
    if plan_locked != expected_locked:
        findings.append(
            Finding(
                severity="error",
                check_id=(
                    "settings_repair_corpus.seed_plan.locked_classes_drift"
                ),
                message=(
                    f"{label} expected.locked_classes {expected_locked!r} "
                    f"does not match anchor plan locked_classes "
                    f"{plan_locked!r}"
                ),
                remediation=(
                    "Mirror the locked_classes set from the anchor plan."
                ),
                ref=label,
            )
        )

    plan_hidden = plan.get("hidden_reset_guard") or {}
    expected_hidden = expected.get("hidden_reset_guard") or {}
    for field_name in (
        "would_broaden_scope",
        "would_touch_adjacent_settings",
    ):
        if expected_hidden.get(field_name) != plan_hidden.get(field_name):
            findings.append(
                Finding(
                    severity="error",
                    check_id=(
                        f"settings_repair_corpus.seed_plan.hidden_{field_name}_drift"
                    ),
                    message=(
                        f"{label} hidden_reset_guard.{field_name} "
                        f"{expected_hidden.get(field_name)!r} does not match "
                        f"anchor plan {plan_hidden.get(field_name)!r}"
                    ),
                    remediation=(
                        "Mirror the hidden-reset guard verdict from the "
                        "anchor plan."
                    ),
                    ref=label,
                )
            )
    plan_refused = sorted(plan_hidden.get("refused_setting_ids") or [])
    expected_refused = sorted(expected_hidden.get("refused_setting_ids") or [])
    if plan_refused != expected_refused:
        findings.append(
            Finding(
                severity="error",
                check_id=(
                    "settings_repair_corpus.seed_plan.refused_settings_drift"
                ),
                message=(
                    f"{label} refused_setting_ids {expected_refused!r} does "
                    f"not match anchor plan refused_setting_ids "
                    f"{plan_refused!r}"
                ),
                remediation=(
                    "Mirror refused_setting_ids from the anchor plan."
                ),
                ref=label,
            )
        )

    if parity.get("winning_scope_token") != plan.get("target_scope"):
        findings.append(
            Finding(
                severity="error",
                check_id=(
                    "settings_repair_corpus.seed_plan.parity_scope_drift"
                ),
                message=(
                    f"{label} surface_parity.winning_scope_token "
                    f"{parity.get('winning_scope_token')!r} does not match "
                    f"anchor plan target_scope {plan.get('target_scope')!r}"
                ),
                remediation=(
                    "Keep the parity winning scope aligned with the anchor "
                    "plan's target_scope."
                ),
                ref=label,
            )
        )

    return findings


def validate_coverage(records: dict[str, dict[str, Any]]) -> list[Finding]:
    findings: list[Finding] = []
    scenarios = {
        record.get("scenario_class") for record in records.values()
    }
    missing_scenarios = REQUIRED_SCENARIO_CLASSES - scenarios
    if missing_scenarios:
        findings.append(
            Finding(
                severity="error",
                check_id="settings_repair_corpus.coverage.scenario_missing",
                message=(
                    "fixtures must cover every required scenario_class"
                ),
                remediation=(
                    "Seed one corpus case per required scenario class."
                ),
                details={"missing": sorted(missing_scenarios)},
            )
        )
    return findings


def validate_report_and_matrix(
    report_path: Path,
    matrix_path: Path,
    drills_doc_path: Path,
    records: dict[str, dict[str, Any]],
) -> list[Finding]:
    findings: list[Finding] = []
    fixture_filenames = sorted(records.keys())
    fixture_ref_paths = [
        f"{DEFAULT_FIXTURE_DIR_REL}/{name}" for name in fixture_filenames
    ]

    if not report_path.exists():
        findings.append(
            Finding(
                severity="error",
                check_id="settings_repair_corpus.report.missing",
                message=f"safety report artifact is missing: {report_path}",
                remediation=(
                    "Land artifacts/config/m3/settings_repair_safety_report.md "
                    "summarising the corpus."
                ),
                ref=str(report_path),
            )
        )
    else:
        report_text = report_path.read_text(encoding="utf-8")
        for ref in (
            DEFAULT_SCHEMA_REL,
            DEFAULT_FIXTURE_DIR_REL,
            "ci/check_settings_repair_corpus.py",
            DEFAULT_MATRIX_REL,
            DEFAULT_DRILLS_DOC_REL,
        ):
            if ref not in report_text:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="settings_repair_corpus.report.missing_ref",
                        message=(
                            f"safety report does not mention {ref}"
                        ),
                        remediation=(
                            "Keep the report referencing the schema, "
                            "fixtures, validator, wrong-scope-write matrix, "
                            "and reviewer drills doc."
                        ),
                        ref=str(report_path),
                    )
                )
        for scenario in REQUIRED_SCENARIO_CLASSES:
            if scenario not in report_text:
                findings.append(
                    Finding(
                        severity="error",
                        check_id=(
                            "settings_repair_corpus.report.missing_scenario"
                        ),
                        message=(
                            f"safety report does not mention scenario "
                            f"{scenario}"
                        ),
                        remediation=(
                            "Document every scenario class in the report."
                        ),
                        ref=str(report_path),
                    )
                )

    if not matrix_path.exists():
        findings.append(
            Finding(
                severity="error",
                check_id="settings_repair_corpus.matrix.missing",
                message=f"wrong-scope-write matrix is missing: {matrix_path}",
                remediation=(
                    "Land artifacts/config/m3/wrong_scope_write_matrix.json "
                    "summarising the corpus rows."
                ),
                ref=str(matrix_path),
            )
        )
    else:
        try:
            matrix = json.loads(matrix_path.read_text(encoding="utf-8"))
        except json.JSONDecodeError as exc:
            findings.append(
                Finding(
                    severity="error",
                    check_id="settings_repair_corpus.matrix.invalid_json",
                    message=f"matrix is not valid JSON: {exc}",
                    remediation="Repair the JSON file.",
                    ref=str(matrix_path),
                )
            )
            return findings
        if not isinstance(matrix, dict):
            findings.append(
                Finding(
                    severity="error",
                    check_id="settings_repair_corpus.matrix.shape",
                    message="wrong-scope-write matrix must be a JSON object",
                    remediation="See the report for the expected shape.",
                    ref=str(matrix_path),
                )
            )
            return findings
        if (
            matrix.get("record_kind")
            != "settings_repair_corpus_wrong_scope_write_matrix"
        ):
            findings.append(
                Finding(
                    severity="error",
                    check_id="settings_repair_corpus.matrix.record_kind",
                    message=(
                        "matrix record_kind must be "
                        "settings_repair_corpus_wrong_scope_write_matrix"
                    ),
                    remediation="Set the record_kind field.",
                    ref=str(matrix_path),
                )
            )
        if matrix.get("schema_version") != 1:
            findings.append(
                Finding(
                    severity="error",
                    check_id="settings_repair_corpus.matrix.schema_version",
                    message="matrix schema_version must be 1",
                    remediation="Set schema_version to 1.",
                    ref=str(matrix_path),
                )
            )
        fixture_refs = set(matrix.get("fixture_refs") or [])
        missing_refs = set(fixture_ref_paths) - fixture_refs
        if missing_refs:
            findings.append(
                Finding(
                    severity="error",
                    check_id=(
                        "settings_repair_corpus.matrix.fixture_refs_missing"
                    ),
                    message="matrix is missing fixture refs",
                    remediation=(
                        "Every corpus fixture must appear in fixture_refs."
                    ),
                    details={"missing": sorted(missing_refs)},
                    ref=str(matrix_path),
                )
            )
        rows = matrix.get("rows") or []
        row_case_names = {
            row.get("case_name") for row in rows if isinstance(row, dict)
        }
        expected_case_names = {
            record.get("case_name") for record in records.values()
        }
        missing_rows = expected_case_names - row_case_names
        if missing_rows:
            findings.append(
                Finding(
                    severity="error",
                    check_id="settings_repair_corpus.matrix.rows_missing",
                    message="matrix is missing rows for corpus cases",
                    remediation="Emit one matrix row per corpus fixture.",
                    details={"missing": sorted(filter(None, missing_rows))},
                    ref=str(matrix_path),
                )
            )
        row_index: dict[str, dict[str, Any]] = {}
        for row in rows:
            if not isinstance(row, dict):
                continue
            case_name = row.get("case_name")
            if isinstance(case_name, str):
                row_index[case_name] = row
        for record in records.values():
            case_name = record.get("case_name")
            if not isinstance(case_name, str) or case_name not in row_index:
                continue
            row = row_index[case_name]
            expected = record.get("expected", {})
            parity = record.get("surface_parity", {})
            for axis_key, source in (
                ("scenario_class", record.get("scenario_class")),
                ("action_class", expected.get("action_class")),
                ("target_scope", expected.get("target_scope")),
                (
                    "target_scope_class",
                    expected.get("target_scope_class"),
                ),
                (
                    "target_artifact_ref",
                    expected.get("target_artifact_ref"),
                ),
                ("verdict", expected.get("verdict")),
                (
                    "blocked_write_result_token",
                    parity.get("blocked_write_result_token"),
                ),
            ):
                if row.get(axis_key) != source:
                    findings.append(
                        Finding(
                            severity="error",
                            check_id=(
                                f"settings_repair_corpus.matrix.row.{axis_key}_drift"
                            ),
                            message=(
                                f"matrix row {case_name} {axis_key} "
                                f"{row.get(axis_key)!r} does not match "
                                f"corpus value {source!r}"
                            ),
                            remediation=(
                                "Regenerate the wrong-scope-write matrix "
                                "from the corpus fixtures."
                            ),
                            ref=str(matrix_path),
                        )
                    )

    if not drills_doc_path.exists():
        findings.append(
            Finding(
                severity="error",
                check_id="settings_repair_corpus.drills_doc.missing",
                message=(
                    f"reviewer drills doc is missing: {drills_doc_path}"
                ),
                remediation=(
                    "Land docs/qe/m3/settings_repair_drills.md describing "
                    "the corpus drills."
                ),
                ref=str(drills_doc_path),
            )
        )
    else:
        drills_text = drills_doc_path.read_text(encoding="utf-8")
        for ref in (
            DEFAULT_SCHEMA_REL,
            DEFAULT_FIXTURE_DIR_REL,
            DEFAULT_REPORT_REL,
            DEFAULT_MATRIX_REL,
            "ci/check_settings_repair_corpus.py",
        ):
            if ref not in drills_text:
                findings.append(
                    Finding(
                        severity="error",
                        check_id=(
                            "settings_repair_corpus.drills_doc.missing_ref"
                        ),
                        message=(
                            f"reviewer drills doc does not mention {ref}"
                        ),
                        remediation=(
                            "Reference the schema, fixtures, report, "
                            "matrix, and validator in the drills doc."
                        ),
                        ref=str(drills_doc_path),
                    )
                )
        for scenario in REQUIRED_SCENARIO_CLASSES:
            if scenario not in drills_text:
                findings.append(
                    Finding(
                        severity="error",
                        check_id=(
                            "settings_repair_corpus.drills_doc.missing_scenario"
                        ),
                        message=(
                            f"reviewer drills doc does not mention scenario "
                            f"{scenario}"
                        ),
                        remediation=(
                            "Document every scenario class in the drills doc."
                        ),
                        ref=str(drills_doc_path),
                    )
                )

    return findings


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    schema_path = repo_root / args.schema
    fixture_dir = repo_root / args.fixture_dir
    plan_fixture_dir = repo_root / args.plan_fixture_dir
    report_path = repo_root / args.report
    matrix_path = repo_root / args.matrix
    drills_doc_path = repo_root / args.drills_doc

    schema = load_json(schema_path)
    records = collect_records(fixture_dir)

    findings: list[Finding] = []
    for name, record in records.items():
        findings.extend(schema_validate(schema, name, record))
        findings.extend(
            cross_check_record(name, record, plan_fixture_dir, repo_root)
        )

    findings.extend(validate_coverage(records))
    findings.extend(
        validate_report_and_matrix(
            report_path, matrix_path, drills_doc_path, records
        )
    )

    report = {
        "status": "pass" if not findings else "fail",
        "schema_ref": args.schema,
        "fixture_dir": args.fixture_dir,
        "record_count": len(records),
        "findings": [finding.as_report() for finding in findings],
    }
    if args.report_json:
        Path(args.report_json).write_text(
            json.dumps(report, indent=2) + "\n", encoding="utf-8"
        )
    if findings:
        print(json.dumps(report, indent=2), file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
