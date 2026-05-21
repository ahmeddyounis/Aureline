#!/usr/bin/env python3
"""Validate the stable claim matrix, launch cutline, qualification rows, and
shiproom stop rules.

Stable launch-facing wording regresses when the decision to call a surface
"Stable" lives in prose, side spreadsheets, or optimistic badges. This gate
makes that decision explicit and enforceable. It reads the checked-in matrix at
``artifacts/release/stable_claim_matrix.json`` and:

  - asserts the closed vocabularies (claim levels, qualification states,
    downgrade reasons, stop actions) and the launch cutline are canonical;
  - asserts every claim row that is not qualified, has stale evidence, or relied
    on an expired waiver narrows below the cutline rather than inheriting an
    adjacent green row, and that a held stable claim carries current,
    proof-backed, owner-signed qualification with no active downgrade reason;
  - performs the date arithmetic the typed model cannot: against the matrix
    ``as_of`` date it recomputes waiver expiry and evidence staleness and fails
    when a row overstates its posture (holds a claim on an expired waiver or
    stale evidence);
  - recomputes the promotion verdict from the rows and stop rules and fails when
    the declared decision or blocking sets drift;
  - recomputes the summary block; and
  - runs negative drills proving the narrowing, stop-rule, waiver-expiry, and
    promotion rejections all fire.

With ``--require-proceed`` the gate additionally fails (exit code 2) when the
recomputed promotion verdict is ``hold``, so shiproom and release tooling can
block stable promotion directly from this artifact.

The typed Rust consumer
(``aureline_release::stable_claim_matrix::current_stable_claim_matrix``) reads
the same matrix and runs the same structural cross-check, so this gate and
``cargo test -p aureline-release`` agree without a cargo build in CI.
"""

from __future__ import annotations

import argparse
import copy
import dataclasses
import datetime as dt
import json
import sys
from pathlib import Path
from typing import Any


DEFAULT_MATRIX_REL = "artifacts/release/stable_claim_matrix.json"
DEFAULT_REPORT_REL = "artifacts/release/captures/stable_claim_matrix_validation_capture.json"

EXPECTED_SCHEMA_VERSION = 1
MATRIX_RECORD_KIND = "stable_claim_matrix"

CLAIM_LEVELS = ("lts", "stable", "beta", "preview", "withdrawn")
ABOVE_CUTLINE = ("lts", "stable")
BELOW_CUTLINE = ("beta", "preview", "withdrawn")
QUALIFICATION_STATES = (
    "qualified",
    "provisional_on_waiver",
    "not_qualified",
    "evidence_stale",
    "waiver_expired",
)
DOWNGRADE_REASONS = (
    "qualification_evidence_missing",
    "qualification_evidence_stale",
    "waiver_expired",
    "freshness_window_exceeded",
    "owner_signoff_missing",
    "compatibility_row_degraded",
    "blocking_defect_open",
)
STOP_ACTIONS = (
    "hold_promotion",
    "narrow_claim",
    "refresh_evidence_packet",
    "staff_correction_lane",
    "block_milestone_close",
)
HOLDING_STATES = ("qualified", "provisional_on_waiver")
NARROWING_STATES = ("not_qualified", "evidence_stale", "waiver_expired")

LEVEL_RANK = {"lts": 4, "stable": 3, "beta": 2, "preview": 1, "withdrawn": 0}
CUTLINE_RANK = LEVEL_RANK["stable"]


@dataclasses.dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    ref: str

    def as_report(self) -> dict[str, str]:
        return dataclasses.asdict(self)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--matrix", default=DEFAULT_MATRIX_REL)
    parser.add_argument("--report", default=None, help="Optional JSON validation capture path.")
    parser.add_argument(
        "--check",
        action="store_true",
        help="CI mode: validate and write the validation capture (default report path).",
    )
    parser.add_argument(
        "--no-capture",
        action="store_true",
        help="Validate without writing the validation capture.",
    )
    parser.add_argument(
        "--require-proceed",
        action="store_true",
        help="Promotion gate: also fail (exit 2) when the recomputed promotion verdict is hold.",
    )
    return parser.parse_args()


def load_json(path: Path, label: str) -> Any:
    if not path.exists():
        raise SystemExit(f"missing {label}: {path}")
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise SystemExit(f"{label} is not valid JSON: {exc}") from exc


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be a mapping/object")
    return value


def generated_at_now() -> str:
    return (
        dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")
    )


def json_text(payload: dict[str, Any]) -> str:
    return json.dumps(payload, indent=2, sort_keys=True) + "\n"


def is_str(value: Any) -> bool:
    return isinstance(value, str) and bool(value.strip())


def parse_date(value: Any) -> dt.date | None:
    if not isinstance(value, str):
        return None
    try:
        return dt.date.fromisoformat(value)
    except ValueError:
        return None


def is_above_cutline(level: Any) -> bool:
    return isinstance(level, str) and LEVEL_RANK.get(level, -1) >= CUTLINE_RANK


def stop_rule_fires(rule: dict[str, Any], rows: list[dict[str, Any]]) -> bool:
    trigger = rule.get("trigger_reason")
    levels = rule.get("applies_to_levels", [])
    if not isinstance(levels, list):
        return False
    for row in rows:
        if row.get("claimed_level") in levels and trigger in row.get(
            "active_downgrade_reasons", []
        ):
            return True
    return False


def computed_promotion(matrix: dict[str, Any]) -> tuple[str, list[str], list[str]]:
    rows = matrix.get("rows", [])
    stop_rules = matrix.get("stop_rules", [])
    blocking_rules = [
        rule
        for rule in stop_rules
        if rule.get("blocks_promotion") is True and stop_rule_fires(rule, rows)
    ]
    rule_ids = sorted(str(rule.get("rule_id")) for rule in blocking_rules)
    blocking_triggers = {rule.get("trigger_reason") for rule in blocking_rules}
    claim_ids: set[str] = set()
    for row in rows:
        if is_above_cutline(row.get("claimed_level")) and blocking_triggers.intersection(
            row.get("active_downgrade_reasons", [])
        ):
            claim_ids.add(str(row.get("claim_id")))
    decision = "hold" if blocking_rules else "proceed"
    return decision, rule_ids, sorted(claim_ids)


def computed_summary(matrix: dict[str, Any]) -> dict[str, int]:
    rows = matrix.get("rows", [])
    stop_rules = matrix.get("stop_rules", [])
    holding = [r for r in rows if is_above_cutline(r.get("effective_level"))]
    return {
        "total_rows": len(rows),
        "rows_holding_stable": len(holding),
        "rows_narrowed_below_cutline": len(rows) - len(holding),
        "rows_on_active_waiver": sum(
            1 for r in rows if r.get("qualification_state") == "provisional_on_waiver"
        ),
        "total_active_downgrade_reasons": sum(
            len(r.get("active_downgrade_reasons", []))
            for r in rows
            if isinstance(r.get("active_downgrade_reasons"), list)
        ),
        "stop_rules_firing": sum(1 for rule in stop_rules if stop_rule_fires(rule, rows)),
    }


def validate_envelope(matrix: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    matrix_id = str(matrix.get("matrix_id", "<matrix>"))

    if matrix.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(
            Finding("error", "matrix.schema_version", "schema_version must be 1", matrix_id)
        )
    if matrix.get("record_kind") != MATRIX_RECORD_KIND:
        findings.append(
            Finding("error", "matrix.record_kind", "record_kind is not supported", matrix_id)
        )
    for field in ("matrix_id", "status", "overview_page", "as_of"):
        if not is_str(matrix.get(field)):
            findings.append(
                Finding("error", "matrix.empty_field", f"{field} must be a non-empty string", matrix_id)
            )
    if parse_date(matrix.get("as_of")) is None:
        findings.append(
            Finding("error", "matrix.as_of_invalid", "as_of must be an ISO date", matrix_id)
        )

    for key, expected in (
        ("claim_levels", list(CLAIM_LEVELS)),
        ("qualification_states", list(QUALIFICATION_STATES)),
        ("downgrade_reasons", list(DOWNGRADE_REASONS)),
        ("stop_rule_actions", list(STOP_ACTIONS)),
    ):
        if list(matrix.get(key, [])) != expected:
            findings.append(
                Finding("error", "matrix.vocabulary", f"matrix.{key} is not the closed vocabulary", key)
            )

    cutline = matrix.get("launch_cutline")
    if not isinstance(cutline, dict):
        findings.append(
            Finding("error", "cutline.missing", "matrix must carry a launch_cutline", matrix_id)
        )
    else:
        if cutline.get("cutline_level") != "stable":
            findings.append(
                Finding("error", "cutline.level", "cutline_level must be stable", matrix_id)
            )
        if list(cutline.get("above_cutline_levels", [])) != list(ABOVE_CUTLINE):
            findings.append(
                Finding("error", "cutline.above", "above_cutline_levels is not canonical", matrix_id)
            )
        if list(cutline.get("below_cutline_levels", [])) != list(BELOW_CUTLINE):
            findings.append(
                Finding("error", "cutline.below", "below_cutline_levels is not canonical", matrix_id)
            )
        if not is_str(cutline.get("description")):
            findings.append(
                Finding("error", "cutline.description", "cutline description must be non-empty", matrix_id)
            )
    return findings


def validate_stop_rules(matrix: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    stop_rules = matrix.get("stop_rules")
    if not isinstance(stop_rules, list) or not stop_rules:
        findings.append(
            Finding("error", "stop_rules.empty", "matrix must enumerate at least one stop rule", "<matrix>")
        )
        return findings

    seen: set[str] = set()
    covered: set[str] = set()
    for rule in stop_rules:
        rule_id = str(rule.get("rule_id", "<rule>"))
        for field in ("rule_id", "title", "rationale"):
            if not is_str(rule.get(field)):
                findings.append(
                    Finding("error", "stop_rule.empty_field", f"stop rule {field} must be non-empty", rule_id)
                )
        if rule_id in seen:
            findings.append(Finding("error", "stop_rule.duplicate_id", "stop rule ids must be unique", rule_id))
        seen.add(rule_id)
        if rule.get("trigger_reason") not in DOWNGRADE_REASONS:
            findings.append(
                Finding("error", "stop_rule.trigger_invalid", "trigger_reason is outside the vocabulary", rule_id)
            )
        else:
            covered.add(rule["trigger_reason"])
        if rule.get("default_action") not in STOP_ACTIONS:
            findings.append(
                Finding("error", "stop_rule.action_invalid", "default_action is outside the vocabulary", rule_id)
            )
        levels = rule.get("applies_to_levels")
        if not isinstance(levels, list) or not levels:
            findings.append(
                Finding("error", "stop_rule.levels_empty", "stop rule must watch at least one level", rule_id)
            )
        elif any(level not in CLAIM_LEVELS for level in levels):
            findings.append(
                Finding("error", "stop_rule.levels_invalid", "applies_to_levels has an unknown level", rule_id)
            )
        if not isinstance(rule.get("blocks_promotion"), bool):
            findings.append(
                Finding("error", "stop_rule.blocks_invalid", "blocks_promotion must be a boolean", rule_id)
            )

    for reason in DOWNGRADE_REASONS:
        if reason not in covered:
            findings.append(
                Finding(
                    "error",
                    "stop_rule.reason_uncovered",
                    "downgrade reason has no stop rule watching for it",
                    reason,
                )
            )
    return findings


def validate_row(row: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings: list[Finding] = []
    claim_id = str(row.get("claim_id", "<row>"))

    for field in ("claim_id", "title", "subject_family", "rationale"):
        if not is_str(row.get(field)):
            findings.append(
                Finding("error", "row.empty_field", f"row {field} must be non-empty", claim_id)
            )

    claimed = row.get("claimed_level")
    effective = row.get("effective_level")
    state = row.get("qualification_state")
    reasons = row.get("active_downgrade_reasons", [])
    if not isinstance(reasons, list):
        reasons = []

    if claimed not in CLAIM_LEVELS:
        findings.append(Finding("error", "row.claimed_invalid", "claimed_level is invalid", claim_id))
    elif not is_above_cutline(claimed):
        findings.append(
            Finding("error", "row.claimed_below_cutline", "claimed_level must be at or above the cutline", claim_id)
        )
    if effective not in CLAIM_LEVELS:
        findings.append(Finding("error", "row.effective_invalid", "effective_level is invalid", claim_id))
    if state not in QUALIFICATION_STATES:
        findings.append(Finding("error", "row.state_invalid", "qualification_state is invalid", claim_id))
    if any(reason not in DOWNGRADE_REASONS for reason in reasons):
        findings.append(
            Finding("error", "row.reason_invalid", "active_downgrade_reasons has an unknown reason", claim_id)
        )

    if claimed in LEVEL_RANK and effective in LEVEL_RANK and LEVEL_RANK[effective] > LEVEL_RANK[claimed]:
        findings.append(
            Finding("error", "row.effective_wider_than_claimed", "effective_level is wider than claimed_level", claim_id)
        )

    evidence = row.get("evidence")
    if not isinstance(evidence, dict):
        findings.append(Finding("error", "row.evidence_missing", "row must carry an evidence block", claim_id))
        evidence = {}
    if not is_str(evidence.get("proof_index_ref")):
        findings.append(
            Finding("error", "row.proof_index_missing", "evidence.proof_index_ref must be non-empty", claim_id)
        )
    window = evidence.get("freshness_window_days")
    if not isinstance(window, int) or isinstance(window, bool) or window < 1:
        findings.append(
            Finding("error", "row.freshness_window_invalid", "freshness_window_days must be an integer >= 1", claim_id)
        )

    holds_stable = is_above_cutline(effective)

    # Acceptance core: a narrowing state must drop below the cutline and name a
    # reason; a held stable claim must be clean, proof-backed, and signed off.
    if state in NARROWING_STATES:
        if holds_stable:
            findings.append(
                Finding(
                    "error",
                    "row.effective_not_narrowed",
                    "a row that is not stable-qualified must narrow below the cutline",
                    claim_id,
                )
            )
        if not reasons:
            findings.append(
                Finding("error", "row.narrowing_without_reason", "a narrowing row must name an active downgrade reason", claim_id)
            )
    if holds_stable:
        if state in NARROWING_STATES:
            findings.append(
                Finding("error", "row.stable_with_narrowing_state", "a held stable claim must not carry a narrowing state", claim_id)
            )
        if reasons:
            findings.append(
                Finding("error", "row.stable_with_active_downgrade", "a held stable claim must carry no active downgrade reason", claim_id)
            )
        if not evidence.get("evidence_refs"):
            findings.append(
                Finding("error", "row.stable_without_evidence", "a held stable claim must name qualification evidence", claim_id)
            )
        signoff = row.get("owner_signoff", {})
        if not (signoff.get("signed_off") is True and is_str(signoff.get("signed_at"))):
            findings.append(
                Finding("error", "row.stable_without_signoff", "a held stable claim must carry an owner sign-off with a date", claim_id)
            )

    findings.extend(validate_state_reason_coherence(claim_id, state, reasons, row))
    findings.extend(validate_refs_exist(claim_id, evidence, repo_root))
    return findings


def validate_state_reason_coherence(
    claim_id: str, state: Any, reasons: list[str], row: dict[str, Any]
) -> list[Finding]:
    findings: list[Finding] = []
    if state == "not_qualified" and "qualification_evidence_missing" not in reasons:
        findings.append(
            Finding("error", "row.state_reason_incoherent", "not_qualified requires qualification_evidence_missing", claim_id)
        )
    if state == "evidence_stale" and not (
        "qualification_evidence_stale" in reasons or "freshness_window_exceeded" in reasons
    ):
        findings.append(
            Finding("error", "row.state_reason_incoherent", "evidence_stale requires a staleness reason", claim_id)
        )
    if state == "waiver_expired":
        if "waiver_expired" not in reasons:
            findings.append(
                Finding("error", "row.state_reason_incoherent", "waiver_expired requires the waiver_expired reason", claim_id)
            )
        if not isinstance(row.get("waiver"), dict):
            findings.append(
                Finding("error", "row.waiver_state_without_waiver", "waiver_expired state must name a waiver", claim_id)
            )
    if state == "provisional_on_waiver":
        waiver = row.get("waiver")
        if not isinstance(waiver, dict) or not is_str(waiver.get("waiver_ref")) or not is_str(
            waiver.get("expires_at")
        ):
            findings.append(
                Finding("error", "row.waiver_state_without_waiver", "provisional_on_waiver state must name a waiver", claim_id)
            )
    return findings


def validate_refs_exist(claim_id: str, evidence: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings: list[Finding] = []
    proof_ref = evidence.get("proof_index_ref")
    refs: list[str] = []
    if is_str(proof_ref):
        refs.append(proof_ref.split("#", 1)[0])
    for ref in evidence.get("evidence_refs", []) or []:
        if is_str(ref):
            refs.append(ref.split("#", 1)[0])
    for ref in refs:
        if not (repo_root / ref).exists():
            findings.append(
                Finding("error", "row.ref_missing", f"referenced artifact does not exist: {ref}", claim_id)
            )
    return findings


def validate_dates(matrix: dict[str, Any]) -> list[Finding]:
    """Date arithmetic the typed model cannot do: against as_of, fail when a row
    overstates its posture on an expired waiver or stale evidence."""
    findings: list[Finding] = []
    as_of = parse_date(matrix.get("as_of"))
    if as_of is None:
        return findings
    for row in matrix.get("rows", []):
        claim_id = str(row.get("claim_id", "<row>"))
        state = row.get("qualification_state")

        waiver = row.get("waiver")
        if isinstance(waiver, dict):
            expires = parse_date(waiver.get("expires_at"))
            if expires is None:
                findings.append(
                    Finding("error", "row.waiver_expiry_invalid", "waiver expires_at must be an ISO date", claim_id)
                )
            else:
                expired = as_of >= expires
                if expired and state == "provisional_on_waiver":
                    findings.append(
                        Finding(
                            "error",
                            "row.provisional_on_expired_waiver",
                            "a provisional claim relies on a waiver that has expired against as_of",
                            claim_id,
                        )
                    )
                if not expired and state == "waiver_expired":
                    findings.append(
                        Finding(
                            "error",
                            "row.waiver_expired_but_active",
                            "a row is marked waiver_expired but the waiver is still active against as_of",
                            claim_id,
                        )
                    )

        evidence = row.get("evidence", {})
        captured = parse_date(evidence.get("captured_at")) if isinstance(evidence, dict) else None
        window = evidence.get("freshness_window_days") if isinstance(evidence, dict) else None
        if captured is not None and isinstance(window, int) and not isinstance(window, bool):
            stale = as_of > captured + dt.timedelta(days=window)
            if stale and state in HOLDING_STATES:
                findings.append(
                    Finding(
                        "error",
                        "row.evidence_stale_but_claimed",
                        "a held stable claim rides evidence past its freshness window against as_of",
                        claim_id,
                    )
                )
    return findings


def validate_promotion(matrix: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    promotion = matrix.get("promotion")
    if not isinstance(promotion, dict):
        findings.append(Finding("error", "promotion.missing", "matrix must carry a promotion block", "<matrix>"))
        return findings
    for field in ("promotion_gate", "rationale"):
        if not is_str(promotion.get(field)):
            findings.append(
                Finding("error", "promotion.empty_field", f"promotion.{field} must be non-empty", "<matrix>")
            )
    decision, rule_ids, claim_ids = computed_promotion(matrix)
    if promotion.get("decision") != decision:
        findings.append(
            Finding(
                "error",
                "promotion.decision_inconsistent",
                f"declared decision {promotion.get('decision')!r} disagrees with computed {decision!r}",
                "<matrix>",
            )
        )
    if list(promotion.get("blocking_rule_ids", [])) != rule_ids:
        findings.append(
            Finding("error", "promotion.blocking_rules_mismatch", "blocking_rule_ids disagrees with the firing stop rules", "<matrix>")
        )
    if list(promotion.get("blocking_claim_ids", [])) != claim_ids:
        findings.append(
            Finding("error", "promotion.blocking_claims_mismatch", "blocking_claim_ids disagrees with the firing stop rules", "<matrix>")
        )
    return findings


def validate_summary(matrix: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    summary = matrix.get("summary")
    if not isinstance(summary, dict):
        findings.append(Finding("error", "summary.missing", "matrix must carry a summary block", "<matrix>"))
        return findings
    expected = computed_summary(matrix)
    for key, value in expected.items():
        if summary.get(key) != value:
            findings.append(
                Finding("error", "summary.count_mismatch", f"summary.{key} must equal {value}", key)
            )
    return findings


def validate_matrix(matrix: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings = validate_envelope(matrix)
    findings.extend(validate_stop_rules(matrix))

    rows = matrix.get("rows")
    if not isinstance(rows, list) or not rows:
        findings.append(Finding("error", "matrix.rows_empty", "matrix must enumerate at least one row", "<matrix>"))
        return findings

    seen: set[str] = set()
    for raw in rows:
        row = ensure_dict(raw, "matrix.rows[]")
        findings.extend(validate_row(row, repo_root))
        claim_id = str(row.get("claim_id", "<row>"))
        if claim_id in seen:
            findings.append(Finding("error", "row.duplicate_id", "claim ids must be unique", claim_id))
        seen.add(claim_id)

    findings.extend(validate_dates(matrix))
    findings.extend(validate_promotion(matrix))
    findings.extend(validate_summary(matrix))
    return findings


def run_negative_drills(matrix: dict[str, Any], repo_root: Path) -> tuple[list[dict[str, str]], list[Finding]]:
    results: list[dict[str, str]] = []
    findings: list[Finding] = []

    def record(drill_id: str, expected_check_id: str, fired: bool) -> None:
        results.append({"drill_id": drill_id, "expected_check_id": expected_check_id, "status": "passed" if fired else "failed"})
        if not fired:
            findings.append(Finding("error", "negative_drill.not_rejected", f"negative drill {drill_id} did not fire", drill_id))

    def check_ids(candidate: dict[str, Any]) -> set[str]:
        return {f.check_id for f in validate_matrix(candidate, repo_root)}

    # A row that is not qualified but still holds its claimed level must be flagged.
    mutated = copy.deepcopy(matrix)
    target = next((r for r in mutated["rows"] if r.get("qualification_state") in NARROWING_STATES), None)
    if target is not None:
        target["effective_level"] = target["claimed_level"]
        mutated["summary"] = computed_summary(mutated)
        decision, rule_ids, claim_ids = computed_promotion(mutated)
        mutated["promotion"].update(
            {"decision": decision, "blocking_rule_ids": rule_ids, "blocking_claim_ids": claim_ids}
        )
        record("effective_not_narrowed_rejected", "row.effective_not_narrowed", "row.effective_not_narrowed" in check_ids(mutated))

    # A held stable claim that carries an active downgrade reason must be flagged.
    mutated = copy.deepcopy(matrix)
    target = next((r for r in mutated["rows"] if is_above_cutline(r.get("effective_level"))), None)
    if target is not None:
        target["active_downgrade_reasons"] = ["compatibility_row_degraded"]
        mutated["summary"] = computed_summary(mutated)
        decision, rule_ids, claim_ids = computed_promotion(mutated)
        mutated["promotion"].update(
            {"decision": decision, "blocking_rule_ids": rule_ids, "blocking_claim_ids": claim_ids}
        )
        record("stable_with_active_downgrade_rejected", "row.stable_with_active_downgrade", "row.stable_with_active_downgrade" in check_ids(mutated))

    # A provisional claim whose waiver has expired against as_of must be flagged.
    mutated = copy.deepcopy(matrix)
    target = next((r for r in mutated["rows"] if r.get("qualification_state") == "provisional_on_waiver"), None)
    if target is not None and isinstance(target.get("waiver"), dict):
        target["waiver"]["expires_at"] = "2000-01-01"
        record("provisional_on_expired_waiver_rejected", "row.provisional_on_expired_waiver", "row.provisional_on_expired_waiver" in check_ids(mutated))

    # A held stable claim riding stale evidence against as_of must be flagged.
    mutated = copy.deepcopy(matrix)
    target = next(
        (r for r in mutated["rows"] if r.get("qualification_state") == "qualified" and isinstance(r.get("evidence"), dict)),
        None,
    )
    if target is not None:
        target["evidence"]["captured_at"] = "2000-01-01"
        record("evidence_stale_but_claimed_rejected", "row.evidence_stale_but_claimed", "row.evidence_stale_but_claimed" in check_ids(mutated))

    # A promotion decision that disagrees with the firing stop rules must be flagged.
    mutated = copy.deepcopy(matrix)
    mutated["promotion"]["decision"] = "proceed" if mutated["promotion"].get("decision") == "hold" else "hold"
    record("promotion_decision_inconsistent_rejected", "promotion.decision_inconsistent", "promotion.decision_inconsistent" in check_ids(mutated))

    return results, findings


def write_report(
    path: Path,
    matrix: dict[str, Any],
    findings: list[Finding],
    drill_results: list[dict[str, str]],
) -> None:
    decision, rule_ids, claim_ids = computed_promotion(matrix)
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "record_kind": "stable_claim_matrix_validation_capture",
        "status": "pass" if not any(f.severity == "error" for f in findings) else "fail",
        "generated_at": generated_at_now(),
        "matrix_id": matrix.get("matrix_id"),
        "as_of": matrix.get("as_of"),
        "summary": matrix.get("summary"),
        "promotion": {
            "decision": decision,
            "blocking_rule_ids": rule_ids,
            "blocking_claim_ids": claim_ids,
        },
        "negative_drills": drill_results,
        "findings": [f.as_report() for f in findings],
    }
    path.write_text(json_text(payload), encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    matrix = ensure_dict(load_json(repo_root / args.matrix, "stable claim matrix"), "stable claim matrix")

    findings = validate_matrix(matrix, repo_root)
    drill_results, drill_findings = run_negative_drills(matrix, repo_root)
    findings.extend(drill_findings)

    report_rel = args.report
    if report_rel is None and not args.no_capture:
        report_rel = DEFAULT_REPORT_REL
    if report_rel:
        write_report(repo_root / report_rel, matrix, findings, drill_results)

    errors = [f for f in findings if f.severity == "error"]
    if errors:
        for item in errors:
            print(f"ERROR [{item.check_id}] ({item.ref}): {item.message}", file=sys.stderr)
        return 1

    summary = matrix.get("summary", {})
    decision, rule_ids, _claim_ids = computed_promotion(matrix)
    print(
        "stable claim matrix validated "
        f"({summary.get('total_rows')} rows, "
        f"{summary.get('rows_holding_stable')} holding stable, "
        f"{summary.get('rows_narrowed_below_cutline')} narrowed below cutline; "
        f"{summary.get('stop_rules_firing')} stop rules firing; "
        f"promotion={decision}; {len(drill_results)} negative drills)"
    )

    if args.require_proceed and decision == "hold":
        print(
            f"PROMOTION HELD: stable train blocked by {len(rule_ids)} stop rule(s): {', '.join(rule_ids)}",
            file=sys.stderr,
        )
        return 2

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
