#!/usr/bin/env python3
"""Validate the M3 merge-queue and browser-handoff audit corpus.

Each fixture under fixtures/review/m3/merge_queue_audit/ encodes one
release-evidence audit case. The corpus must:

- Cover every required scenario class (provider outage, expired auth,
  stale base/head, check invalidation, parent-stack blockage, queue
  policy change, local-CI parity disagreement, browser handoff open
  flow, browser handoff reopen/return flow).
- Surface drift between provider state, local drafts, and queue
  eligibility as a labeled state instead of passing as green.
- Reject landing/queue actions that skip command-graph preview, lose
  local-draft state, or hide stale-base invalidation.
- Prove browser handoff/reopen returns the exact provider object or a
  truthful placeholder.
- Reconstruct review identity, provider freshness, queue eligibility
  reason, and local-draft state from each support-export packet.

Companion artifacts (audit report, provider staleness matrix, reviewer
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


DEFAULT_SCHEMA_REL = "schemas/review/merge_queue_audit_case.schema.json"
DEFAULT_FIXTURE_DIR_REL = "fixtures/review/m3/merge_queue_audit"
DEFAULT_REPORT_REL = "artifacts/review/m3/merge_queue_audit_report.md"
DEFAULT_MATRIX_REL = "artifacts/review/m3/provider_staleness_matrix.json"
DEFAULT_DRILLS_DOC_REL = "docs/qe/m3/review_merge_queue_drills.md"
DEFAULT_LANDING_FIXTURE_DIR_REL = "fixtures/review/m3/merge_queue_and_landing"

REQUIRED_SCENARIO_CLASSES = {
    "provider_outage",
    "expired_auth",
    "stale_base_head",
    "check_invalidation",
    "parent_stack_blockage",
    "queue_policy_change",
    "local_ci_parity_disagreement",
    "browser_handoff_open_provider",
    "browser_handoff_reopen_return",
}

REQUIRED_LANDING_ACTION_AUDIT_CLASSES = {
    "enqueue_audited",
    "dequeue_audited",
    "rerun_pipeline_audited",
    "refresh_provider_overlay_audited",
    "publish_to_provider_minted_not_launched",
    "request_changes_audited",
}

REQUIRED_LABELED_DRIFT_STATES = {
    "provider_freshness_drift_labeled",
    "queue_eligibility_drift_labeled",
    "parent_stack_drift_labeled",
    "queue_policy_drift_labeled",
    "local_ci_parity_drift_labeled",
    "browser_handoff_drift_labeled",
}

PROVIDER_FRESHNESS_DRIFT_STATES = {
    "provider_reachable_stale_within_grace",
    "provider_overlay_unavailable",
    "provider_outage_user_must_retry",
    "provider_auth_expired_user_must_reauth",
    "provider_payload_unparseable",
    "provider_overlay_stale_blocks_landing",
}

STALE_BASE_BLOCKING_STATES = {"base_stale_blocks_landing"}

BROWSER_HANDOFF_TRUTHFUL_OUTCOMES = {
    "handoff_returned_exact_object",
    "handoff_returned_truthful_placeholder",
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
        "--landing-fixture-dir", default=DEFAULT_LANDING_FIXTURE_DIR_REL
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
                check_id="merge_queue_audit.schema.validation_failed",
                message=f"{label}: {path}: {error.message}",
                remediation=(
                    "Fix the audit case so it validates against "
                    "schemas/review/merge_queue_audit_case.schema.json."
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


def collect_landing_candidate_ids(landing_dir: Path) -> set[str]:
    ids: set[str] = set()
    if not landing_dir.exists():
        return ids
    for path in sorted(landing_dir.glob("*.json")):
        record = load_json(path)
        if not isinstance(record, dict):
            continue
        landing = record.get("landing_input")
        if isinstance(landing, dict):
            candidate_id = landing.get("landing_candidate_id")
            if isinstance(candidate_id, str):
                ids.add(candidate_id)
    return ids


def cross_check_record(
    label: str,
    record: dict[str, Any],
    landing_candidate_ids: set[str],
) -> list[Finding]:
    findings: list[Finding] = []
    audit_input = record.get("audit_input", {})
    expected = record.get("expected", {})
    scenario_class = record.get("scenario_class")

    landing_candidate_id = audit_input.get("landing_candidate_id_ref")
    if (
        landing_candidate_ids
        and isinstance(landing_candidate_id, str)
        and landing_candidate_id not in landing_candidate_ids
    ):
        findings.append(
            Finding(
                severity="error",
                check_id="merge_queue_audit.landing_candidate_id.unknown",
                message=(
                    f"{label} landing_candidate_id_ref {landing_candidate_id} "
                    "does not match any landing-candidate fixture under "
                    f"{DEFAULT_LANDING_FIXTURE_DIR_REL}"
                ),
                remediation=(
                    "Reference a landing candidate id minted by the "
                    "checked-in landing-candidate fixtures."
                ),
                ref=label,
            )
        )

    provider_freshness = audit_input.get("provider_freshness_class")
    provider_drift = provider_freshness in PROVIDER_FRESHNESS_DRIFT_STATES
    eligibility_drift = (
        audit_input.get("eligibility_state") == "queue_not_eligible"
        or bool(audit_input.get("blocked_reasons"))
        or bool(audit_input.get("invalidation_reasons"))
    )
    has_any_drift = provider_drift or eligibility_drift

    labeled_drift = expected.get("labeled_drift_state")
    if has_any_drift and labeled_drift == "no_drift_labeled":
        findings.append(
            Finding(
                severity="error",
                check_id="merge_queue_audit.drift.label_missing",
                message=(
                    f"{label} reports drift in provider freshness or queue "
                    "eligibility but expected.labeled_drift_state is "
                    "no_drift_labeled"
                ),
                remediation=(
                    "Choose a non-no_drift_labeled labeled_drift_state so the "
                    "corpus surfaces drift instead of passing as green."
                ),
                ref=label,
            )
        )

    if expected.get("passes_green_under_drift") is not False:
        findings.append(
            Finding(
                severity="error",
                check_id="merge_queue_audit.green.under_drift",
                message=(
                    f"{label} expected.passes_green_under_drift must be false"
                ),
                remediation=(
                    "Drift must never pass the audit as green; keep "
                    "passes_green_under_drift = false."
                ),
                ref=label,
            )
        )

    commands = audit_input.get("commands") or []
    missing_preview = [
        c.get("command_id")
        for c in commands
        if isinstance(c, dict) and c.get("preview_supported") is not True
    ]
    if missing_preview:
        findings.append(
            Finding(
                severity="error",
                check_id="merge_queue_audit.command.preview_missing",
                message=(
                    f"{label} commands without preview_supported = true: "
                    f"{missing_preview}"
                ),
                remediation=(
                    "Landing and queue actions must advertise command-graph "
                    "preview before mutation; preview_supported = true."
                ),
                ref=label,
            )
        )
    missing_audit = [
        c.get("command_id")
        for c in commands
        if isinstance(c, dict) and c.get("emits_audit_event") is not True
    ]
    if missing_audit:
        findings.append(
            Finding(
                severity="error",
                check_id="merge_queue_audit.command.audit_missing",
                message=(
                    f"{label} commands without emits_audit_event = true: "
                    f"{missing_audit}"
                ),
                remediation=(
                    "Provider mutations must remain attributable; each "
                    "command must emit an audit event."
                ),
                ref=label,
            )
        )

    if (
        expected.get("command_preview_required") is True
        and expected.get("command_preview_present") is not True
    ):
        findings.append(
            Finding(
                severity="error",
                check_id="merge_queue_audit.command.preview_required_but_missing",
                message=(
                    f"{label} expected.command_preview_required is true but "
                    "expected.command_preview_present is not true"
                ),
                remediation=(
                    "Audit cases that require preview must also assert "
                    "preview presence."
                ),
                ref=label,
            )
        )

    drafts = audit_input.get("local_drafts") or []
    has_misrepresentation = any(
        isinstance(d, dict)
        and d.get("misrepresented_as_provider_synced") is not False
        for d in drafts
    )
    if has_misrepresentation:
        findings.append(
            Finding(
                severity="error",
                check_id="merge_queue_audit.local_draft.misrepresented",
                message=(
                    f"{label} contains a local draft marked as "
                    "misrepresented_as_provider_synced"
                ),
                remediation=(
                    "Local drafts must never be misrepresented as "
                    "provider-synced truth."
                ),
                ref=label,
            )
        )
    if (
        expected.get("local_draft_misrepresented_as_provider_synced")
        is not False
    ):
        findings.append(
            Finding(
                severity="error",
                check_id="merge_queue_audit.local_draft.expected_misrepresented",
                message=(
                    f"{label} expected.local_draft_misrepresented_as_provider_synced "
                    "must be false"
                ),
                remediation=(
                    "The audit corpus must reject any path that misrepresents "
                    "local drafts as provider-synced truth."
                ),
                ref=label,
            )
        )

    if drafts and expected.get("local_draft_preserved") is not True:
        findings.append(
            Finding(
                severity="error",
                check_id="merge_queue_audit.local_draft.preservation_missing",
                message=(
                    f"{label} carries local drafts but expected.local_draft_preserved "
                    "is not true"
                ),
                remediation=(
                    "Cases with local drafts must assert local_draft_preserved = true."
                ),
                ref=label,
            )
        )

    stale_base_state = audit_input.get("stale_base_state")
    if (
        stale_base_state in STALE_BASE_BLOCKING_STATES
        and "stale_base" not in (audit_input.get("invalidation_reasons") or [])
    ):
        findings.append(
            Finding(
                severity="error",
                check_id="merge_queue_audit.stale_base.invalidation_missing",
                message=(
                    f"{label} stale_base_state is {stale_base_state} but "
                    "invalidation_reasons does not include 'stale_base'"
                ),
                remediation=(
                    "Stale-base invalidation must be visible in the "
                    "invalidation_reasons axis."
                ),
                ref=label,
            )
        )
    if expected.get("stale_base_hidden") is not False:
        findings.append(
            Finding(
                severity="error",
                check_id="merge_queue_audit.stale_base.hidden",
                message=(
                    f"{label} expected.stale_base_hidden must be false"
                ),
                remediation=(
                    "Stale-base invalidation must never be hidden behind a "
                    "collapsed status."
                ),
                ref=label,
            )
        )

    handoff = audit_input.get("browser_handoff")
    if isinstance(handoff, dict):
        outcome = handoff.get("outcome_class")
        if outcome not in BROWSER_HANDOFF_TRUTHFUL_OUTCOMES and outcome not in {
            "handoff_expired_user_must_re_open",
            "handoff_provider_outage_user_must_retry",
        }:
            findings.append(
                Finding(
                    severity="error",
                    check_id="merge_queue_audit.handoff.outcome_unknown",
                    message=(
                        f"{label} browser handoff outcome_class {outcome} is "
                        "not from the closed audit vocabulary"
                    ),
                    remediation=(
                        "Use one of the closed outcome_class tokens."
                    ),
                    ref=label,
                )
            )
        if (
            outcome in BROWSER_HANDOFF_TRUTHFUL_OUTCOMES
            and expected.get("browser_handoff_returns_exact_object_or_truthful_placeholder")
            is not True
        ):
            findings.append(
                Finding(
                    severity="error",
                    check_id="merge_queue_audit.handoff.expected_truthful_routing",
                    message=(
                        f"{label} reports a truthful handoff outcome but expected "
                        "browser_handoff_returns_exact_object_or_truthful_placeholder "
                        "is not true"
                    ),
                    remediation=(
                        "Cases that route handoff truthfully must assert "
                        "browser_handoff_returns_exact_object_or_truthful_placeholder = true."
                    ),
                    ref=label,
                )
            )

    support_export = audit_input.get("support_export") or {}
    for axis_key in (
        "reconstructs_review_identity",
        "reconstructs_provider_freshness",
        "reconstructs_queue_eligibility_reason",
        "reconstructs_local_draft_state",
    ):
        if support_export.get(axis_key) is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"merge_queue_audit.support_export.{axis_key}_missing",
                    message=(
                        f"{label} support_export.{axis_key} must be true"
                    ),
                    remediation=(
                        "Support exports must reconstruct review identity, "
                        "provider freshness, queue eligibility reason, and "
                        "local-draft state."
                    ),
                    ref=label,
                )
            )
    if expected.get("support_export_reconstructs_all_axes") is not True:
        findings.append(
            Finding(
                severity="error",
                check_id="merge_queue_audit.support_export.expected_all_axes",
                message=(
                    f"{label} expected.support_export_reconstructs_all_axes "
                    "must be true"
                ),
                remediation=(
                    "Every audit case must assert support_export_reconstructs_all_axes = true."
                ),
                ref=label,
            )
        )

    audited_classes = set(expected.get("audited_action_classes") or [])
    declared_classes = {
        c.get("audit_class") for c in commands if isinstance(c, dict)
    }
    missing_declared = audited_classes - declared_classes
    if missing_declared:
        findings.append(
            Finding(
                severity="error",
                check_id="merge_queue_audit.audited_action.declared_missing",
                message=(
                    f"{label} expected audited_action_classes {sorted(missing_declared)} "
                    "are not present on any command"
                ),
                remediation=(
                    "Every expected audited action class must be attached to a "
                    "command record."
                ),
                ref=label,
            )
        )

    if scenario_class == "stale_base_head" and stale_base_state not in STALE_BASE_BLOCKING_STATES:
        findings.append(
            Finding(
                severity="error",
                check_id="merge_queue_audit.scenario.stale_base_head_inconsistent",
                message=(
                    f"{label} scenario_class is stale_base_head but "
                    f"stale_base_state is {stale_base_state}"
                ),
                remediation=(
                    "Stale-base scenarios must set stale_base_state = "
                    "base_stale_blocks_landing."
                ),
                ref=label,
            )
        )
    if scenario_class == "provider_outage" and not provider_drift:
        findings.append(
            Finding(
                severity="error",
                check_id="merge_queue_audit.scenario.provider_outage_freshness_inconsistent",
                message=(
                    f"{label} scenario_class is provider_outage but "
                    f"provider_freshness_class is {provider_freshness}"
                ),
                remediation=(
                    "Provider-outage scenarios must use a provider freshness "
                    "class that represents real drift."
                ),
                ref=label,
            )
        )
    if (
        scenario_class
        in {"browser_handoff_open_provider", "browser_handoff_reopen_return"}
        and not isinstance(handoff, dict)
    ):
        findings.append(
            Finding(
                severity="error",
                check_id="merge_queue_audit.scenario.handoff_record_missing",
                message=(
                    f"{label} scenario_class is {scenario_class} but no "
                    "browser_handoff record is present"
                ),
                remediation=(
                    "Browser-handoff scenarios must attach a browser_handoff "
                    "record."
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
                check_id="merge_queue_audit.coverage.scenario_missing",
                message=(
                    "fixtures must cover every required scenario_class"
                ),
                remediation=(
                    "Seed one audit case per required scenario class."
                ),
                details={"missing": sorted(missing_scenarios)},
            )
        )

    drift_labels: set[str] = set()
    audited_classes: set[str] = set()
    for record in records.values():
        expected = record.get("expected", {})
        if isinstance(expected, dict):
            label = expected.get("labeled_drift_state")
            if label:
                drift_labels.add(label)
            for cls in expected.get("audited_action_classes") or []:
                audited_classes.add(cls)
    missing_labels = REQUIRED_LABELED_DRIFT_STATES - drift_labels
    if missing_labels:
        findings.append(
            Finding(
                severity="error",
                check_id="merge_queue_audit.coverage.labeled_drift_missing",
                message=(
                    "fixtures must cover every required labeled_drift_state"
                ),
                remediation=(
                    "Every drift class needed by the spec must appear "
                    "as expected.labeled_drift_state in at least one case."
                ),
                details={"missing": sorted(missing_labels)},
            )
        )
    missing_actions = REQUIRED_LANDING_ACTION_AUDIT_CLASSES - audited_classes
    if missing_actions:
        findings.append(
            Finding(
                severity="error",
                check_id="merge_queue_audit.coverage.audited_action_missing",
                message=(
                    "fixtures must cover every required landing_action_audit_class"
                ),
                remediation=(
                    "Spread audited_action_classes across the corpus so "
                    "enqueue/dequeue/approve/request-changes/rerun/refresh/publish "
                    "are each audited at least once."
                ),
                details={"missing": sorted(missing_actions)},
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
                check_id="merge_queue_audit.report.missing",
                message=f"audit report artifact is missing: {report_path}",
                remediation=(
                    "Land artifacts/review/m3/merge_queue_audit_report.md "
                    "summarising the corpus for docs and partner packets."
                ),
                ref=str(report_path),
            )
        )
    else:
        report_text = report_path.read_text(encoding="utf-8")
        for ref in (
            DEFAULT_SCHEMA_REL,
            DEFAULT_FIXTURE_DIR_REL,
            "ci/check_merge_queue_audit_corpus.py",
            DEFAULT_MATRIX_REL,
            DEFAULT_DRILLS_DOC_REL,
        ):
            if ref not in report_text:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="merge_queue_audit.report.missing_ref",
                        message=(
                            f"audit report does not mention {ref}"
                        ),
                        remediation=(
                            "Keep the report referencing the schema, fixtures, "
                            "validator, provider staleness matrix, and reviewer "
                            "drills doc."
                        ),
                        ref=str(report_path),
                    )
                )
        for scenario in REQUIRED_SCENARIO_CLASSES:
            if scenario not in report_text:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="merge_queue_audit.report.missing_scenario",
                        message=(
                            f"audit report does not mention scenario {scenario}"
                        ),
                        remediation=(
                            "Document every audit scenario class in the report."
                        ),
                        ref=str(report_path),
                    )
                )

    if not matrix_path.exists():
        findings.append(
            Finding(
                severity="error",
                check_id="merge_queue_audit.matrix.missing",
                message=f"provider staleness matrix is missing: {matrix_path}",
                remediation=(
                    "Land artifacts/review/m3/provider_staleness_matrix.json "
                    "summarising audit-case provider freshness state."
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
                    check_id="merge_queue_audit.matrix.invalid_json",
                    message=f"provider staleness matrix is not valid JSON: {exc}",
                    remediation="Repair the JSON file.",
                    ref=str(matrix_path),
                )
            )
            return findings
        if not isinstance(matrix, dict):
            findings.append(
                Finding(
                    severity="error",
                    check_id="merge_queue_audit.matrix.shape",
                    message="provider staleness matrix must be a JSON object",
                    remediation="See the report for the expected shape.",
                    ref=str(matrix_path),
                )
            )
        else:
            if (
                matrix.get("record_kind")
                != "review_merge_queue_audit_provider_staleness_matrix"
            ):
                findings.append(
                    Finding(
                        severity="error",
                        check_id="merge_queue_audit.matrix.record_kind",
                        message=(
                            "provider staleness matrix record_kind must be "
                            "review_merge_queue_audit_provider_staleness_matrix"
                        ),
                        remediation="Set the record_kind field.",
                        ref=str(matrix_path),
                    )
                )
            if matrix.get("schema_version") != 1:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="merge_queue_audit.matrix.schema_version",
                        message=(
                            "provider staleness matrix schema_version must be 1"
                        ),
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
                        check_id="merge_queue_audit.matrix.fixture_refs_missing",
                        message=(
                            "provider staleness matrix is missing fixture refs"
                        ),
                        remediation=(
                            "Every audit fixture must appear in fixture_refs."
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
                        check_id="merge_queue_audit.matrix.rows_missing",
                        message=(
                            "provider staleness matrix is missing rows for "
                            "audit cases"
                        ),
                        remediation=(
                            "Emit one matrix row per audit fixture."
                        ),
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
                audit_input = record.get("audit_input", {})
                expected = record.get("expected", {})
                for axis_key, source in (
                    ("scenario_class", record.get("scenario_class")),
                    (
                        "provider_freshness_class",
                        audit_input.get("provider_freshness_class"),
                    ),
                    (
                        "labeled_drift_state",
                        expected.get("labeled_drift_state"),
                    ),
                    (
                        "eligibility_state",
                        audit_input.get("eligibility_state"),
                    ),
                    (
                        "queue_state",
                        audit_input.get("queue_state"),
                    ),
                    (
                        "stale_base_state",
                        audit_input.get("stale_base_state"),
                    ),
                ):
                    if row.get(axis_key) != source:
                        findings.append(
                            Finding(
                                severity="error",
                                check_id=f"merge_queue_audit.matrix.row.{axis_key}_drift",
                                message=(
                                    f"matrix row {case_name} {axis_key} "
                                    f"{row.get(axis_key)!r} does not match "
                                    f"fixture value {source!r}"
                                ),
                                remediation=(
                                    "Regenerate the provider staleness matrix "
                                    "from the audit fixtures."
                                ),
                                ref=str(matrix_path),
                            )
                        )

    if not drills_doc_path.exists():
        findings.append(
            Finding(
                severity="error",
                check_id="merge_queue_audit.drills_doc.missing",
                message=(
                    f"reviewer drills doc is missing: {drills_doc_path}"
                ),
                remediation=(
                    "Land docs/qe/m3/review_merge_queue_drills.md describing "
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
            "ci/check_merge_queue_audit_corpus.py",
        ):
            if ref not in drills_text:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="merge_queue_audit.drills_doc.missing_ref",
                        message=(
                            f"reviewer drills doc does not mention {ref}"
                        ),
                        remediation=(
                            "Reference the schema, fixtures, report, matrix, "
                            "and validator in the drills doc."
                        ),
                        ref=str(drills_doc_path),
                    )
                )
        for scenario in REQUIRED_SCENARIO_CLASSES:
            if scenario not in drills_text:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="merge_queue_audit.drills_doc.missing_scenario",
                        message=(
                            f"reviewer drills doc does not mention scenario {scenario}"
                        ),
                        remediation=(
                            "Document every audit scenario class in the drills doc."
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
    landing_fixture_dir = repo_root / args.landing_fixture_dir
    report_path = repo_root / args.report
    matrix_path = repo_root / args.matrix
    drills_doc_path = repo_root / args.drills_doc

    schema = load_json(schema_path)
    records = collect_records(fixture_dir)
    landing_candidate_ids = collect_landing_candidate_ids(landing_fixture_dir)

    findings: list[Finding] = []
    for name, record in records.items():
        findings.extend(schema_validate(schema, name, record))
        findings.extend(cross_check_record(name, record, landing_candidate_ids))

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
