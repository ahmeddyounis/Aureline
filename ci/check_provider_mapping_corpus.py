#!/usr/bin/env python3
"""Validate the M3 provider-account / target-mapping continuity drill corpus.

Each fixture under fixtures/providers/m3/account_scope_and_mapping_corpus/
encodes one deterministic drill case that extends a seeded mapping-review row
(see crates/aureline-provider/src/project_mapping/mod.rs and
schemas/providers/provider_target_mapping.schema.json) into a failure or
recovery scenario for a marketed beta provider lane. The corpus must:

- Cover every required drill class (board/project remap, stale token,
  installation-grant withdrawal, policy-locked mapping, offline capture,
  browser-blocked handoff, publish-later replay, queued-draft export/import),
  every provider lane, and every account profile.
- Prove the exact provider/account/mapping identity triple survives support
  export, activity-center reopen, and restart/restore without leaking raw
  credentials.
- Fail the lane closed when a queued draft would silently vanish, a narrowed
  session would still appear writable, or a mapping would change without a
  visible review.

Companion artifacts (continuity report, per-lane continuity matrix, reviewer
drills doc, enum-only corpus matrix) are required to exist and to reference the
corpus.
"""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any

from jsonschema import Draft202012Validator, FormatChecker


DEFAULT_SCHEMA_REL = "schemas/providers/provider_account_mapping_drill_case.schema.json"
DEFAULT_FIXTURE_DIR_REL = "fixtures/providers/m3/account_scope_and_mapping_corpus"
DEFAULT_CORPUS_MATRIX_REL = (
    "fixtures/providers/m3/account_scope_and_mapping_corpus/corpus_matrix.json"
)
DEFAULT_REPORT_REL = "artifacts/providers/m3/account_scope_and_mapping_report.md"
DEFAULT_CONTINUITY_MATRIX_REL = (
    "artifacts/providers/m3/account_scope_and_mapping_continuity_matrix.json"
)
DEFAULT_DRILLS_DOC_REL = "docs/providers/m3/provider_account_and_mapping_drills.md"
DEFAULT_VALIDATOR_REL = "ci/check_provider_mapping_corpus.py"
DEFAULT_SCRIPT_REL = "scripts/ci/run_provider_mapping_corpus.sh"

REQUIRED_DRILL_CLASSES = {
    "board_project_remap",
    "stale_token",
    "installation_grant_withdrawal",
    "policy_locked_mapping",
    "offline_capture",
    "browser_blocked_handoff",
    "publish_later_replay",
    "queued_draft_export_import",
}

REQUIRED_LANES = {
    "issue_or_work_item",
    "review_decision",
    "incident_handoff",
    "publish_later",
}

REQUIRED_PROFILES = {
    "connected",
    "mirror_only",
    "offline",
    "enterprise_managed",
}

NARROWED_SESSION_STATES = {
    "limited_scope",
    "stale_credential",
    "read_only",
    "offline_capture",
    "publish_later_only",
}

NON_LIVE_POSTURES = {
    "local_draft",
    "queued_publish_later",
    "read_only_inspection",
}

DEFERRED_POSTURES = {"local_draft", "queued_publish_later"}

LIVE_ADMITTING_RESOLUTIONS = {"resolved_single_target", "policy_locked_target"}

TRUTHFUL_HANDOFF_OUTCOMES = {
    "handoff_blocked_user_must_retry",
    "handoff_blocked_fallback_offered",
}

CONTINUITY_SURFACES = ("support_export", "activity_center_reopen", "restart_restore")

CONTINUITY_MATRIX_RECORD_KIND = "providers_account_mapping_continuity_matrix"
CORPUS_MATRIX_RECORD_KIND = "providers_account_mapping_drill_corpus_matrix"

MATRIX_AXES = (
    "drill_class",
    "provider_lane",
    "profile",
    "session_state",
    "resolution_state",
    "publish_posture",
    "next_action",
    "fail_closed",
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


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--schema", default=DEFAULT_SCHEMA_REL)
    parser.add_argument("--fixture-dir", default=DEFAULT_FIXTURE_DIR_REL)
    parser.add_argument("--corpus-matrix", default=DEFAULT_CORPUS_MATRIX_REL)
    parser.add_argument("--report", default=DEFAULT_REPORT_REL)
    parser.add_argument("--continuity-matrix", default=DEFAULT_CONTINUITY_MATRIX_REL)
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
    for error in sorted(validator.iter_errors(payload), key=lambda item: list(item.path)):
        path = ".".join(str(part) for part in error.path) or "<root>"
        findings.append(
            Finding(
                severity="error",
                check_id="provider_mapping_corpus.schema.validation_failed",
                message=f"{label}: {path}: {error.message}",
                remediation=(
                    "Fix the drill case so it validates against "
                    f"{DEFAULT_SCHEMA_REL}."
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
        if path.name == "corpus_matrix.json":
            continue
        records[path.name] = load_json(path)
    if not records:
        raise SystemExit(f"no drill fixtures in {fixture_dir}")
    return records


def cross_check_record(label: str, record: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    drill_class = record.get("drill_class")
    provider_lane = record.get("provider_lane")
    identity = record.get("identity", {})
    drill_input = record.get("drill_input", {})
    continuity = record.get("continuity", {})
    expected = record.get("expected", {})

    session_state = drill_input.get("session_state")
    resolution_state = drill_input.get("resolution_state")
    publish_posture = drill_input.get("publish_posture")
    next_action = drill_input.get("next_action")
    scope_held = drill_input.get("effective_write_scope_held")
    queued_draft = drill_input.get("queued_draft")
    mapping_change = drill_input.get("mapping_change")
    handoff = drill_input.get("browser_handoff")

    # 1. Identity triple survives every continuity surface, verbatim.
    triple = (
        identity.get("provider_id"),
        identity.get("account_id"),
        identity.get("mapping_id"),
    )
    for surface in CONTINUITY_SURFACES:
        echo = continuity.get(surface, {})
        echo_triple = (
            echo.get("provider_id"),
            echo.get("account_id"),
            echo.get("mapping_id"),
        )
        if echo_triple != triple:
            findings.append(
                Finding(
                    severity="error",
                    check_id="provider_mapping_corpus.identity.drift",
                    message=(
                        f"{label} continuity.{surface} identity triple "
                        f"{echo_triple} does not match identity {triple}"
                    ),
                    remediation=(
                        "Echo the exact provider/account/mapping identity through "
                        "every continuity surface so support and reviewer surfaces "
                        "can always name which account acted and which target a "
                        "mutation would touch."
                    ),
                    ref=label,
                )
            )
        if echo.get("identity_preserved") is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id="provider_mapping_corpus.identity.preserved_flag_missing",
                    message=(
                        f"{label} continuity.{surface}.identity_preserved must be true"
                    ),
                    remediation="Every continuity surface must preserve identity.",
                    ref=label,
                )
            )
        if echo.get("raw_credentials_present") is not False:
            findings.append(
                Finding(
                    severity="error",
                    check_id="provider_mapping_corpus.identity.raw_credentials",
                    message=(
                        f"{label} continuity.{surface}.raw_credentials_present "
                        "must be false"
                    ),
                    remediation=(
                        "Continuity surfaces preserve stable IDs only; raw "
                        "credentials must never appear in a packet."
                    ),
                    ref=label,
                )
            )

    for axis in (
        "identity_survives_support_export",
        "identity_survives_activity_center_reopen",
        "identity_survives_restart_restore",
    ):
        if expected.get(axis) is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"provider_mapping_corpus.expected.{axis}_missing",
                    message=f"{label} expected.{axis} must be true",
                    remediation=(
                        "Each drill must assert provider/account/mapping identity "
                        "survives support export, activity-center reopen, and "
                        "restart/restore."
                    ),
                    ref=label,
                )
            )

    # 2. Raw-secret guardrails.
    if drill_input.get("raw_token_material_present") is not False:
        findings.append(
            Finding(
                severity="error",
                check_id="provider_mapping_corpus.raw_token.present",
                message=f"{label} drill_input.raw_token_material_present must be false",
                remediation="Drill cases must never carry raw token material.",
                ref=label,
            )
        )
    if expected.get("raw_credentials_leaked") is not False:
        findings.append(
            Finding(
                severity="error",
                check_id="provider_mapping_corpus.raw_credentials.leaked",
                message=f"{label} expected.raw_credentials_leaked must be false",
                remediation="No drill may leak raw credentials.",
                ref=label,
            )
        )

    # 3. Fail-closed under a stale or narrowed session (or without write scope).
    narrowed = session_state in NARROWED_SESSION_STATES or scope_held is False
    if narrowed:
        if publish_posture not in NON_LIVE_POSTURES:
            findings.append(
                Finding(
                    severity="error",
                    check_id="provider_mapping_corpus.fail_closed.writable_under_narrowed",
                    message=(
                        f"{label} session_state {session_state!r} / "
                        f"effective_write_scope_held {scope_held!r} but "
                        f"publish_posture is {publish_posture!r}"
                    ),
                    remediation=(
                        "A narrowed or stale session must degrade to a local draft, "
                        "queued publish-later, or read-only inspection — never a live "
                        "provider mutation."
                    ),
                    ref=label,
                )
            )
        if expected.get("fail_closed") is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id="provider_mapping_corpus.fail_closed.expected_missing",
                    message=(
                        f"{label} is narrowed/stale but expected.fail_closed is not true"
                    ),
                    remediation="Narrowed and stale sessions must fail closed.",
                    ref=label,
                )
            )
        if next_action == "none_proceed":
            findings.append(
                Finding(
                    severity="error",
                    check_id="provider_mapping_corpus.fail_closed.next_action_proceed",
                    message=(
                        f"{label} is narrowed/stale but next_action is none_proceed"
                    ),
                    remediation=(
                        "A narrowed or stale session must name a concrete next-safe "
                        "action instead of proceeding."
                    ),
                    ref=label,
                )
            )
    if expected.get("narrowed_session_appears_writable") is not False:
        findings.append(
            Finding(
                severity="error",
                check_id="provider_mapping_corpus.narrowed.appears_writable",
                message=(
                    f"{label} expected.narrowed_session_appears_writable must be false"
                ),
                remediation="A narrowed session must never appear writable.",
                ref=label,
            )
        )

    # 4. Live-posture safety (guards a future tampered fixture).
    if publish_posture == "live_provider_mutation":
        if session_state != "live":
            findings.append(
                Finding(
                    severity="error",
                    check_id="provider_mapping_corpus.live.non_live_session",
                    message=f"{label} live posture on non-live session {session_state!r}",
                    remediation="A live mutation requires a live provider session.",
                    ref=label,
                )
            )
        if scope_held is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id="provider_mapping_corpus.live.no_write_scope",
                    message=f"{label} live posture without effective write scope",
                    remediation="A live mutation requires effective write scope.",
                    ref=label,
                )
            )
        if resolution_state not in LIVE_ADMITTING_RESOLUTIONS:
            findings.append(
                Finding(
                    severity="error",
                    check_id="provider_mapping_corpus.live.unresolved_mapping",
                    message=(
                        f"{label} live posture on resolution {resolution_state!r}"
                    ),
                    remediation=(
                        "A live mutation requires a resolved or policy-locked single "
                        "target."
                    ),
                    ref=label,
                )
            )

    # 5. Queued drafts never silently vanish.
    if expected.get("queued_draft_silently_vanished") is not False:
        findings.append(
            Finding(
                severity="error",
                check_id="provider_mapping_corpus.queued_draft.vanished",
                message=f"{label} expected.queued_draft_silently_vanished must be false",
                remediation="A queued draft must never silently vanish.",
                ref=label,
            )
        )
    for axis in (
        "local_draft_preserved",
        "queued_transitions_preserved",
        "evidence_preserved",
        "retry_export_available",
    ):
        if expected.get(axis) is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"provider_mapping_corpus.durability.{axis}_missing",
                    message=f"{label} expected.{axis} must be true",
                    remediation=(
                        "Local drafts, queued transitions, and evidence stay durable "
                        "with retry/export available through every drill."
                    ),
                    ref=label,
                )
            )
    if isinstance(queued_draft, dict):
        if queued_draft.get("retained") is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id="provider_mapping_corpus.queued_draft.not_retained",
                    message=f"{label} queued_draft.retained must be true",
                    remediation="A captured queued draft must be retained.",
                    ref=label,
                )
            )
        if queued_draft.get("retry_export_available") is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id="provider_mapping_corpus.queued_draft.no_retry_export",
                    message=f"{label} queued_draft.retry_export_available must be true",
                    remediation="A queued draft must keep a retry/export path.",
                    ref=label,
                )
            )
        if publish_posture not in DEFERRED_POSTURES:
            findings.append(
                Finding(
                    severity="error",
                    check_id="provider_mapping_corpus.queued_draft.posture",
                    message=(
                        f"{label} carries a queued draft but publish_posture is "
                        f"{publish_posture!r}"
                    ),
                    remediation=(
                        "A queued draft belongs to a local-draft or "
                        "queued-publish-later posture."
                    ),
                    ref=label,
                )
            )

    # 6. Mapping changes are never applied without a visible review.
    if expected.get("mapping_changed_without_visible_review") is not False:
        findings.append(
            Finding(
                severity="error",
                check_id="provider_mapping_corpus.mapping_change.silent",
                message=(
                    f"{label} expected.mapping_changed_without_visible_review "
                    "must be false"
                ),
                remediation="A mapping change must always surface a visible review.",
                ref=label,
            )
        )
    if isinstance(mapping_change, dict):
        if mapping_change.get("requires_visible_review") is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id="provider_mapping_corpus.mapping_change.review_not_required",
                    message=f"{label} mapping_change.requires_visible_review must be true",
                    remediation="A mapping change must require a visible review.",
                    ref=label,
                )
            )
        if mapping_change.get("review_surfaced") is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id="provider_mapping_corpus.mapping_change.review_not_surfaced",
                    message=f"{label} mapping_change.review_surfaced must be true",
                    remediation="The visible review for a mapping change must be surfaced.",
                    ref=label,
                )
            )

    # 7. Drill-class specific consistency.
    findings.extend(
        check_drill_class(
            label,
            drill_class,
            identity,
            session_state,
            resolution_state,
            publish_posture,
            queued_draft,
            mapping_change,
            handoff,
        )
    )

    # 8. Lane/profile must be in the closed vocabulary (schema guards membership;
    #    this guards the spec-required coverage axes used by the matrix).
    if provider_lane not in REQUIRED_LANES:
        findings.append(
            Finding(
                severity="error",
                check_id="provider_mapping_corpus.lane.unknown",
                message=f"{label} provider_lane {provider_lane!r} is not a beta lane",
                remediation="Use one of the four marketed beta provider lanes.",
                ref=label,
            )
        )

    return findings


def check_drill_class(
    label: str,
    drill_class: str | None,
    identity: dict[str, Any],
    session_state: str | None,
    resolution_state: str | None,
    publish_posture: str | None,
    queued_draft: Any,
    mapping_change: Any,
    handoff: Any,
) -> list[Finding]:
    findings: list[Finding] = []

    def fail(check_id: str, message: str, remediation: str) -> None:
        findings.append(
            Finding(
                severity="error",
                check_id=check_id,
                message=f"{label} {message}",
                remediation=remediation,
                ref=label,
            )
        )

    if drill_class == "stale_token":
        if session_state != "stale_credential":
            fail(
                "provider_mapping_corpus.drill.stale_token_session",
                f"stale_token drill must use session_state stale_credential, got {session_state!r}",
                "Model the stale-token drill with a stale_credential session.",
            )
    elif drill_class == "offline_capture":
        if session_state != "offline_capture":
            fail(
                "provider_mapping_corpus.drill.offline_capture_session",
                f"offline_capture drill must use session_state offline_capture, got {session_state!r}",
                "Model the offline-capture drill with an offline_capture session.",
            )
    elif drill_class == "policy_locked_mapping":
        if resolution_state != "policy_locked_target":
            fail(
                "provider_mapping_corpus.drill.policy_locked_resolution",
                f"policy_locked_mapping drill must use resolution_state policy_locked_target, got {resolution_state!r}",
                "Model the policy-locked drill with a policy_locked_target resolution.",
            )
    elif drill_class == "installation_grant_withdrawal":
        if identity.get("acting_identity_class") != "installation_grant":
            fail(
                "provider_mapping_corpus.drill.grant_withdrawal_identity",
                "installation_grant_withdrawal drill must act as an installation_grant",
                "Bind the grant-withdrawal drill to an installation_grant identity.",
            )
    elif drill_class == "board_project_remap":
        if not isinstance(mapping_change, dict):
            fail(
                "provider_mapping_corpus.drill.remap_change_missing",
                "board_project_remap drill must carry a mapping_change record",
                "Attach the remap mapping_change so the visible review is proven.",
            )
    elif drill_class == "browser_blocked_handoff":
        if not isinstance(handoff, dict):
            fail(
                "provider_mapping_corpus.drill.handoff_missing",
                "browser_blocked_handoff drill must carry a browser_handoff record",
                "Attach the browser_handoff record describing the blocked handoff.",
            )
        else:
            if handoff.get("blocked") is not True:
                fail(
                    "provider_mapping_corpus.drill.handoff_not_blocked",
                    "browser_blocked_handoff drill must set browser_handoff.blocked = true",
                    "A blocked-handoff drill must mark the handoff blocked.",
                )
            outcome = handoff.get("outcome_class")
            if outcome not in TRUTHFUL_HANDOFF_OUTCOMES:
                fail(
                    "provider_mapping_corpus.drill.handoff_outcome",
                    f"browser_blocked_handoff outcome_class {outcome!r} is not a truthful blocked outcome",
                    "A blocked handoff must degrade truthfully (retry or fallback).",
                )
            if (
                handoff.get("fallback_offered") is True
                and outcome != "handoff_blocked_fallback_offered"
            ):
                fail(
                    "provider_mapping_corpus.drill.handoff_fallback_mismatch",
                    "browser_handoff offers a fallback but outcome_class is not handoff_blocked_fallback_offered",
                    "Match outcome_class to the offered fallback.",
                )
    elif drill_class == "publish_later_replay":
        if not isinstance(queued_draft, dict):
            fail(
                "provider_mapping_corpus.drill.replay_queue_missing",
                "publish_later_replay drill must carry a queued_draft record",
                "Attach the queued_draft that is replayed on reconnect.",
            )
        if publish_posture != "queued_publish_later":
            fail(
                "provider_mapping_corpus.drill.replay_posture",
                f"publish_later_replay drill must use posture queued_publish_later, got {publish_posture!r}",
                "A replay drill stages the action as queued_publish_later.",
            )
    elif drill_class == "queued_draft_export_import":
        if not isinstance(queued_draft, dict):
            fail(
                "provider_mapping_corpus.drill.export_queue_missing",
                "queued_draft_export_import drill must carry a queued_draft record",
                "Attach the queued_draft that is exported and re-imported.",
            )
        elif not queued_draft.get("export_round_trip_id"):
            fail(
                "provider_mapping_corpus.drill.export_round_trip_missing",
                "queued_draft_export_import drill must name an export_round_trip_id",
                "Name the export/import round-trip id on the queued draft.",
            )

    return findings


def validate_coverage(records: dict[str, dict[str, Any]]) -> list[Finding]:
    findings: list[Finding] = []
    drill_classes = {r.get("drill_class") for r in records.values()}
    lanes = {r.get("provider_lane") for r in records.values()}
    profiles = {r.get("profile") for r in records.values()}

    for required, observed, axis, check_id in (
        (REQUIRED_DRILL_CLASSES, drill_classes, "drill_class", "drill_class"),
        (REQUIRED_LANES, lanes, "provider_lane", "lane"),
        (REQUIRED_PROFILES, profiles, "profile", "profile"),
    ):
        missing = required - observed
        if missing:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"provider_mapping_corpus.coverage.{check_id}_missing",
                    message=f"fixtures must cover every required {axis}",
                    remediation=f"Seed at least one drill case per required {axis}.",
                    details={"missing": sorted(filter(None, missing))},
                )
            )
    return findings


def validate_corpus_matrix(
    corpus_matrix_path: Path, records: dict[str, dict[str, Any]]
) -> list[Finding]:
    findings: list[Finding] = []
    if not corpus_matrix_path.exists():
        return [
            Finding(
                severity="error",
                check_id="provider_mapping_corpus.corpus_matrix.missing",
                message=f"enum-only corpus matrix missing: {corpus_matrix_path}",
                remediation="Land the enum-only corpus_matrix.json beside the fixtures.",
                ref=str(corpus_matrix_path),
            )
        ]
    matrix = load_json(corpus_matrix_path)
    if matrix.get("record_kind") != CORPUS_MATRIX_RECORD_KIND:
        findings.append(
            Finding(
                severity="error",
                check_id="provider_mapping_corpus.corpus_matrix.record_kind",
                message=(
                    f"corpus matrix record_kind must be {CORPUS_MATRIX_RECORD_KIND}"
                ),
                remediation="Set the record_kind field.",
                ref=str(corpus_matrix_path),
            )
        )
    if matrix.get("schema_version") != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="provider_mapping_corpus.corpus_matrix.schema_version",
                message="corpus matrix schema_version must be 1",
                remediation="Set schema_version to 1.",
                ref=str(corpus_matrix_path),
            )
        )
    rows = {
        row.get("case_name"): row
        for row in matrix.get("drill_cases", [])
        if isinstance(row, dict)
    }
    fixture_cases = {r.get("case_name") for r in records.values()}
    missing_rows = fixture_cases - set(rows)
    if missing_rows:
        findings.append(
            Finding(
                severity="error",
                check_id="provider_mapping_corpus.corpus_matrix.rows_missing",
                message="corpus matrix is missing rows for drill cases",
                remediation="Emit one corpus_matrix row per drill fixture.",
                details={"missing": sorted(filter(None, missing_rows))},
                ref=str(corpus_matrix_path),
            )
        )
    extra_rows = set(rows) - fixture_cases
    if extra_rows:
        findings.append(
            Finding(
                severity="error",
                check_id="provider_mapping_corpus.corpus_matrix.rows_extra",
                message="corpus matrix has rows without a matching fixture",
                remediation="Remove corpus_matrix rows that have no fixture.",
                details={"extra": sorted(filter(None, extra_rows))},
                ref=str(corpus_matrix_path),
            )
        )
    by_case = {r.get("case_name"): r for r in records.values()}
    for case_name, row in rows.items():
        record = by_case.get(case_name)
        if record is None:
            continue
        di = record.get("drill_input", {})
        source = {
            "drill_class": record.get("drill_class"),
            "provider_lane": record.get("provider_lane"),
            "profile": record.get("profile"),
            "session_state": di.get("session_state"),
            "resolution_state": di.get("resolution_state"),
            "publish_posture": di.get("publish_posture"),
            "next_action": di.get("next_action"),
            "fail_closed": record.get("expected", {}).get("fail_closed"),
        }
        for axis in MATRIX_AXES:
            if row.get(axis) != source[axis]:
                findings.append(
                    Finding(
                        severity="error",
                        check_id=f"provider_mapping_corpus.corpus_matrix.{axis}_drift",
                        message=(
                            f"corpus matrix row {case_name} {axis} {row.get(axis)!r} "
                            f"does not match fixture value {source[axis]!r}"
                        ),
                        remediation="Regenerate the corpus matrix from the fixtures.",
                        ref=str(corpus_matrix_path),
                    )
                )
    return findings


def validate_continuity_matrix(
    matrix_path: Path,
    fixture_dir_rel: str,
    records: dict[str, dict[str, Any]],
) -> list[Finding]:
    findings: list[Finding] = []
    if not matrix_path.exists():
        return [
            Finding(
                severity="error",
                check_id="provider_mapping_corpus.continuity_matrix.missing",
                message=f"continuity matrix missing: {matrix_path}",
                remediation=(
                    "Land artifacts/providers/m3/account_scope_and_mapping_continuity_matrix.json "
                    "summarising per-lane identity continuity for beta scorecards."
                ),
                ref=str(matrix_path),
            )
        ]
    matrix = load_json(matrix_path)
    if matrix.get("record_kind") != CONTINUITY_MATRIX_RECORD_KIND:
        findings.append(
            Finding(
                severity="error",
                check_id="provider_mapping_corpus.continuity_matrix.record_kind",
                message=(
                    f"continuity matrix record_kind must be {CONTINUITY_MATRIX_RECORD_KIND}"
                ),
                remediation="Set the record_kind field.",
                ref=str(matrix_path),
            )
        )
    if matrix.get("schema_version") != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="provider_mapping_corpus.continuity_matrix.schema_version",
                message="continuity matrix schema_version must be 1",
                remediation="Set schema_version to 1.",
                ref=str(matrix_path),
            )
        )

    fixture_ref_paths = {
        f"{fixture_dir_rel}/{name}" for name in records.keys()
    }
    declared_refs = set(matrix.get("fixture_refs") or [])
    missing_refs = fixture_ref_paths - declared_refs
    if missing_refs:
        findings.append(
            Finding(
                severity="error",
                check_id="provider_mapping_corpus.continuity_matrix.fixture_refs_missing",
                message="continuity matrix is missing fixture refs",
                remediation="Every drill fixture must appear in fixture_refs.",
                details={"missing": sorted(missing_refs)},
                ref=str(matrix_path),
            )
        )

    # Per-case rows must match the fixtures.
    case_rows = {
        row.get("case_name"): row
        for row in matrix.get("cases", [])
        if isinstance(row, dict)
    }
    by_case = {r.get("case_name"): r for r in records.values()}
    missing_case_rows = set(by_case) - set(case_rows)
    if missing_case_rows:
        findings.append(
            Finding(
                severity="error",
                check_id="provider_mapping_corpus.continuity_matrix.case_rows_missing",
                message="continuity matrix is missing case rows",
                remediation="Emit one continuity-matrix case row per drill fixture.",
                details={"missing": sorted(filter(None, missing_case_rows))},
                ref=str(matrix_path),
            )
        )
    for case_name, row in case_rows.items():
        record = by_case.get(case_name)
        if record is None:
            findings.append(
                Finding(
                    severity="error",
                    check_id="provider_mapping_corpus.continuity_matrix.case_row_unknown",
                    message=f"continuity matrix case row {case_name} has no fixture",
                    remediation="Remove continuity-matrix case rows without a fixture.",
                    ref=str(matrix_path),
                )
            )
            continue
        for axis, source in (
            ("drill_class", record.get("drill_class")),
            ("provider_lane", record.get("provider_lane")),
            ("profile", record.get("profile")),
            ("fail_closed", record.get("expected", {}).get("fail_closed")),
        ):
            if row.get(axis) != source:
                findings.append(
                    Finding(
                        severity="error",
                        check_id=f"provider_mapping_corpus.continuity_matrix.case_{axis}_drift",
                        message=(
                            f"continuity matrix case {case_name} {axis} {row.get(axis)!r} "
                            f"does not match fixture value {source!r}"
                        ),
                        remediation="Regenerate the continuity matrix from the fixtures.",
                        ref=str(matrix_path),
                    )
                )
        for axis in (
            "identity_survives_support_export",
            "identity_survives_activity_center_reopen",
            "identity_survives_restart_restore",
        ):
            if row.get(axis) is not True:
                findings.append(
                    Finding(
                        severity="error",
                        check_id=f"provider_mapping_corpus.continuity_matrix.case_{axis}_missing",
                        message=f"continuity matrix case {case_name} {axis} must be true",
                        remediation="Every case must prove identity survives all surfaces.",
                        ref=str(matrix_path),
                    )
                )

    # Per-lane rows must cover every required lane.
    lane_rows = {
        row.get("provider_lane"): row
        for row in matrix.get("lanes", [])
        if isinstance(row, dict)
    }
    missing_lanes = REQUIRED_LANES - set(lane_rows)
    if missing_lanes:
        findings.append(
            Finding(
                severity="error",
                check_id="provider_mapping_corpus.continuity_matrix.lane_rows_missing",
                message="continuity matrix is missing per-lane rows",
                remediation="Beta scorecards reference a per-lane continuity packet; emit a row per lane.",
                details={"missing": sorted(missing_lanes)},
                ref=str(matrix_path),
            )
        )
    lane_to_cases: dict[str, set[str]] = {}
    for record in records.values():
        lane_to_cases.setdefault(record.get("provider_lane"), set()).add(
            record.get("case_name")
        )
    for lane, row in lane_rows.items():
        declared = set(row.get("case_names") or [])
        expected_cases = lane_to_cases.get(lane, set())
        if declared != expected_cases:
            findings.append(
                Finding(
                    severity="error",
                    check_id="provider_mapping_corpus.continuity_matrix.lane_cases_drift",
                    message=(
                        f"continuity matrix lane {lane} case_names {sorted(declared)} "
                        f"do not match fixtures {sorted(expected_cases)}"
                    ),
                    remediation="List exactly the drill cases per lane.",
                    ref=str(matrix_path),
                )
            )
        for axis in (
            "identity_survives_support_export",
            "identity_survives_activity_center_reopen",
            "identity_survives_restart_restore",
            "all_fail_closed",
        ):
            if row.get(axis) is not True:
                findings.append(
                    Finding(
                        severity="error",
                        check_id=f"provider_mapping_corpus.continuity_matrix.lane_{axis}_missing",
                        message=f"continuity matrix lane {lane} {axis} must be true",
                        remediation="Each lane packet must prove identity continuity and fail-closed behaviour.",
                        ref=str(matrix_path),
                    )
                )
    return findings


def validate_report_and_doc(
    report_path: Path,
    drills_doc_path: Path,
    records: dict[str, dict[str, Any]],
) -> list[Finding]:
    findings: list[Finding] = []
    report_refs = (
        DEFAULT_SCHEMA_REL,
        DEFAULT_FIXTURE_DIR_REL,
        DEFAULT_VALIDATOR_REL,
        DEFAULT_SCRIPT_REL,
        DEFAULT_CONTINUITY_MATRIX_REL,
        DEFAULT_DRILLS_DOC_REL,
    )
    doc_refs = (
        DEFAULT_SCHEMA_REL,
        DEFAULT_FIXTURE_DIR_REL,
        DEFAULT_REPORT_REL,
        DEFAULT_CONTINUITY_MATRIX_REL,
        DEFAULT_VALIDATOR_REL,
    )

    for path, refs, kind in (
        (report_path, report_refs, "report"),
        (drills_doc_path, doc_refs, "drills_doc"),
    ):
        if not path.exists():
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"provider_mapping_corpus.{kind}.missing",
                    message=f"{kind} artifact is missing: {path}",
                    remediation=f"Land the {kind} for the corpus.",
                    ref=str(path),
                )
            )
            continue
        text = path.read_text(encoding="utf-8")
        for ref in refs:
            if ref not in text:
                findings.append(
                    Finding(
                        severity="error",
                        check_id=f"provider_mapping_corpus.{kind}.missing_ref",
                        message=f"{kind} does not mention {ref}",
                        remediation=(
                            "Keep the report and drills doc referencing the schema, "
                            "fixtures, validator, script, continuity matrix, and each "
                            "drill class."
                        ),
                        ref=str(path),
                    )
                )
        for drill_class in REQUIRED_DRILL_CLASSES:
            if drill_class not in text:
                findings.append(
                    Finding(
                        severity="error",
                        check_id=f"provider_mapping_corpus.{kind}.missing_drill_class",
                        message=f"{kind} does not mention drill class {drill_class}",
                        remediation="Document every drill class.",
                        ref=str(path),
                    )
                )
    return findings


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    schema_path = repo_root / args.schema
    fixture_dir = repo_root / args.fixture_dir
    corpus_matrix_path = repo_root / args.corpus_matrix
    report_path = repo_root / args.report
    continuity_matrix_path = repo_root / args.continuity_matrix
    drills_doc_path = repo_root / args.drills_doc

    schema = load_json(schema_path)
    records = collect_records(fixture_dir)

    findings: list[Finding] = []
    for name, record in records.items():
        findings.extend(schema_validate(schema, name, record))
        findings.extend(cross_check_record(name, record))

    findings.extend(validate_coverage(records))
    findings.extend(validate_corpus_matrix(corpus_matrix_path, records))
    findings.extend(
        validate_continuity_matrix(continuity_matrix_path, args.fixture_dir, records)
    )
    findings.extend(validate_report_and_doc(report_path, drills_doc_path, records))

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
    print(
        f"[provider-mapping-corpus] PASS ({len(records)} drill cases)",
        file=sys.stderr,
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
