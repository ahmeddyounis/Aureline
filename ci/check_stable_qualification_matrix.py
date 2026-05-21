#!/usr/bin/env python3
"""Validate the stable qualification matrix and its mixed-version sections.

The stable claim matrix decides which subjects may publish as Stable; this
qualification matrix finalizes the per-lane qualification rows that ground those
claims — desktop, remote/helper, ecosystem, state/schema, provider, and
accessibility — and adds the piece a flat claim row cannot carry: for every
cross-binary or cross-service boundary, a mixed-version section publishing the
negotiated fields, supported skew window, upgrade order, rollback order, and
unsupported-state behavior.

This gate reads the checked-in matrix at
``artifacts/release/stable_qualification_matrix.json`` and:

  - asserts the closed vocabularies (row scopes, claim levels, qualification
    states, downgrade reasons, downgrade actions, mixed-version postures,
    boundary families, out-of-window postures) and the launch cutline are
    canonical;
  - asserts every lane that is not qualified, has stale evidence, or relied on an
    expired waiver narrows below the cutline rather than inheriting an adjacent
    green row, and that a held stable claim carries current, proof-backed,
    owner-signed qualification with no active downgrade reason;
  - enforces the mixed-version contract: every cross-binary lane carries a
    mixed-version section, the accessibility lane carries none, a section lacking
    complete negotiation data is coordinated-upgrade-only and names the
    ``mixed_version_data_incomplete`` reason, no effective posture widens past its
    claim, and a Stable mixed-version claim (rolling/bounded effective posture)
    rides only complete data on a row that itself holds stable;
  - asserts every boundary family the spec enumerates is covered by at least one
    cross-binary row;
  - performs the date arithmetic the typed model cannot: against the matrix
    ``as_of`` date it recomputes waiver expiry and evidence staleness and fails
    when a row overstates its posture;
  - recomputes the promotion verdict from the rows and downgrade rules and fails
    when the declared decision or blocking sets drift;
  - recomputes the summary block; and
  - runs negative drills proving the narrowing, mixed-version, waiver-expiry, and
    promotion rejections all fire.

With ``--require-proceed`` the gate additionally fails (exit code 2) when the
recomputed promotion verdict is ``hold``, so shiproom and release tooling can
block stable promotion directly from this artifact.

The typed Rust consumer
(``aureline_release::stable_qualification_matrix::current_stable_qualification_matrix``)
reads the same matrix and runs the same structural cross-check, so this gate and
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


DEFAULT_MATRIX_REL = "artifacts/release/stable_qualification_matrix.json"
DEFAULT_REPORT_REL = (
    "artifacts/release/captures/stable_qualification_matrix_validation_capture.json"
)

EXPECTED_SCHEMA_VERSION = 1
MATRIX_RECORD_KIND = "stable_qualification_matrix"

ROW_SCOPES = (
    "desktop",
    "remote_helper",
    "ecosystem",
    "state_schema",
    "provider",
    "accessibility",
)
# Every lane but accessibility crosses a binary or service boundary.
NON_BOUNDARY_SCOPES = ("accessibility",)

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
    "backing_claim_narrowed",
    "mixed_version_data_incomplete",
    "mixed_version_skew_unsupported",
)
DOWNGRADE_ACTIONS = (
    "hold_promotion",
    "narrow_claim",
    "refresh_evidence_packet",
    "require_coordinated_upgrade",
    "staff_correction_lane",
)
MIXED_VERSION_POSTURES = (
    "rolling_skew_supported",
    "bounded_skew_supported",
    "coordinated_upgrade_only",
)
BOUNDARY_FAMILIES = (
    "launcher_and_local_sidecars",
    "desktop_cli_and_remote_agent",
    "desktop_cli_browser_and_managed_control_plane",
    "extension_host_and_abi",
    "saved_artifact_and_schema_readers_writers",
    "provider_adapters",
)
OUT_OF_WINDOW_POSTURES = ("fail_closed", "read_only", "degraded", "explicitly_unsupported")

HOLDING_STATES = ("qualified", "provisional_on_waiver")
NARROWING_STATES = ("not_qualified", "evidence_stale", "waiver_expired")

LEVEL_RANK = {"lts": 4, "stable": 3, "beta": 2, "preview": 1, "withdrawn": 0}
CUTLINE_RANK = LEVEL_RANK["stable"]
POSTURE_RANK = {
    "rolling_skew_supported": 2,
    "bounded_skew_supported": 1,
    "coordinated_upgrade_only": 0,
}
STABLE_MIXED_RANK = POSTURE_RANK["bounded_skew_supported"]


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
    return dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")


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


def requires_mixed_version(scope: Any) -> bool:
    return isinstance(scope, str) and scope not in NON_BOUNDARY_SCOPES


def mixed_is_complete(mixed: dict[str, Any]) -> bool:
    """A mixed-version section publishes complete negotiation data only when it
    declares the negotiated fields, a complete supported skew window, an upgrade
    order, a rollback order, and the unsupported-state behavior."""
    if not mixed.get("negotiated_fields"):
        return False
    window = mixed.get("supported_skew_window")
    if not isinstance(window, dict) or not all(
        is_str(window.get(key)) for key in ("window_class", "summary", "skew_register_ref")
    ):
        return False
    for order_key in ("upgrade_order", "rollback_order"):
        order = mixed.get(order_key)
        if not isinstance(order, dict) or not order.get("declared_order") or not is_str(
            order.get("notes")
        ):
            return False
    behavior = mixed.get("unsupported_state_behavior")
    if not isinstance(behavior, dict) or not (
        is_str(behavior.get("state_class")) and is_str(behavior.get("contract_rule"))
    ):
        return False
    return True


def posture_is_stable_mixed_claim(posture: Any) -> bool:
    return isinstance(posture, str) and POSTURE_RANK.get(posture, -1) >= STABLE_MIXED_RANK


def downgrade_rule_fires(rule: dict[str, Any], rows: list[dict[str, Any]]) -> bool:
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
    rules = matrix.get("downgrade_rules", [])
    blocking_rules = [
        rule
        for rule in rules
        if rule.get("blocks_promotion") is True and downgrade_rule_fires(rule, rows)
    ]
    rule_ids = sorted(str(rule.get("rule_id")) for rule in blocking_rules)
    blocking_triggers = {rule.get("trigger_reason") for rule in blocking_rules}
    row_ids: set[str] = set()
    for row in rows:
        if is_above_cutline(row.get("claimed_level")) and blocking_triggers.intersection(
            row.get("active_downgrade_reasons", [])
        ):
            row_ids.add(str(row.get("row_id")))
    decision = "hold" if blocking_rules else "proceed"
    return decision, rule_ids, sorted(row_ids)


def computed_summary(matrix: dict[str, Any]) -> dict[str, int]:
    rows = matrix.get("rows", [])
    rules = matrix.get("downgrade_rules", [])

    def mixed(row: dict[str, Any]) -> dict[str, Any] | None:
        mv = row.get("mixed_version")
        return mv if isinstance(mv, dict) else None

    holding = [r for r in rows if is_above_cutline(r.get("effective_level"))]
    cross_binary = [r for r in rows if mixed(r) is not None]
    return {
        "total_rows": len(rows),
        "rows_holding_stable": len(holding),
        "rows_narrowed_below_cutline": len(rows) - len(holding),
        "rows_on_active_waiver": sum(
            1 for r in rows if r.get("qualification_state") == "provisional_on_waiver"
        ),
        "cross_binary_rows": len(cross_binary),
        "rows_with_stable_mixed_version_claim": sum(
            1
            for r in cross_binary
            if posture_is_stable_mixed_claim(mixed(r).get("effective_posture"))
        ),
        "coordinated_upgrade_only_rows": sum(
            1
            for r in cross_binary
            if mixed(r).get("effective_posture") == "coordinated_upgrade_only"
        ),
        "total_active_downgrade_reasons": sum(
            len(r.get("active_downgrade_reasons", []))
            for r in rows
            if isinstance(r.get("active_downgrade_reasons"), list)
        ),
        "downgrade_rules_firing": sum(
            1 for rule in rules if downgrade_rule_fires(rule, rows)
        ),
    }


def validate_envelope(matrix: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    matrix_id = str(matrix.get("matrix_id", "<matrix>"))

    if matrix.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(Finding("error", "matrix.schema_version", "schema_version must be 1", matrix_id))
    if matrix.get("record_kind") != MATRIX_RECORD_KIND:
        findings.append(Finding("error", "matrix.record_kind", "record_kind is not supported", matrix_id))
    for field in ("matrix_id", "status", "overview_page", "as_of", "claim_matrix_ref"):
        if not is_str(matrix.get(field)):
            findings.append(
                Finding("error", "matrix.empty_field", f"{field} must be a non-empty string", matrix_id)
            )
    if parse_date(matrix.get("as_of")) is None:
        findings.append(Finding("error", "matrix.as_of_invalid", "as_of must be an ISO date", matrix_id))

    for key, expected in (
        ("row_scopes", list(ROW_SCOPES)),
        ("claim_levels", list(CLAIM_LEVELS)),
        ("qualification_states", list(QUALIFICATION_STATES)),
        ("downgrade_reasons", list(DOWNGRADE_REASONS)),
        ("downgrade_actions", list(DOWNGRADE_ACTIONS)),
        ("mixed_version_postures", list(MIXED_VERSION_POSTURES)),
        ("boundary_families", list(BOUNDARY_FAMILIES)),
        ("out_of_window_postures", list(OUT_OF_WINDOW_POSTURES)),
    ):
        if list(matrix.get(key, [])) != expected:
            findings.append(
                Finding("error", "matrix.vocabulary", f"matrix.{key} is not the closed vocabulary", key)
            )

    cutline = matrix.get("launch_cutline")
    if not isinstance(cutline, dict):
        findings.append(Finding("error", "cutline.missing", "matrix must carry a launch_cutline", matrix_id))
    else:
        if cutline.get("cutline_level") != "stable":
            findings.append(Finding("error", "cutline.level", "cutline_level must be stable", matrix_id))
        if list(cutline.get("above_cutline_levels", [])) != list(ABOVE_CUTLINE):
            findings.append(Finding("error", "cutline.above", "above_cutline_levels is not canonical", matrix_id))
        if list(cutline.get("below_cutline_levels", [])) != list(BELOW_CUTLINE):
            findings.append(Finding("error", "cutline.below", "below_cutline_levels is not canonical", matrix_id))
        if not is_str(cutline.get("description")):
            findings.append(Finding("error", "cutline.description", "cutline description must be non-empty", matrix_id))
    return findings


def validate_rules(matrix: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    rules = matrix.get("downgrade_rules")
    if not isinstance(rules, list) or not rules:
        findings.append(Finding("error", "rules.empty", "matrix must enumerate at least one downgrade rule", "<matrix>"))
        return findings

    seen: set[str] = set()
    covered: set[str] = set()
    for rule in rules:
        rule_id = str(rule.get("rule_id", "<rule>"))
        for field in ("rule_id", "title", "rationale"):
            if not is_str(rule.get(field)):
                findings.append(Finding("error", "rule.empty_field", f"downgrade rule {field} must be non-empty", rule_id))
        if rule_id in seen:
            findings.append(Finding("error", "rule.duplicate_id", "downgrade rule ids must be unique", rule_id))
        seen.add(rule_id)
        if rule.get("trigger_reason") not in DOWNGRADE_REASONS:
            findings.append(Finding("error", "rule.trigger_invalid", "trigger_reason is outside the vocabulary", rule_id))
        else:
            covered.add(rule["trigger_reason"])
        if rule.get("default_action") not in DOWNGRADE_ACTIONS:
            findings.append(Finding("error", "rule.action_invalid", "default_action is outside the vocabulary", rule_id))
        levels = rule.get("applies_to_levels")
        if not isinstance(levels, list) or not levels:
            findings.append(Finding("error", "rule.levels_empty", "downgrade rule must watch at least one level", rule_id))
        elif any(level not in CLAIM_LEVELS for level in levels):
            findings.append(Finding("error", "rule.levels_invalid", "applies_to_levels has an unknown level", rule_id))
        if not isinstance(rule.get("blocks_promotion"), bool):
            findings.append(Finding("error", "rule.blocks_invalid", "blocks_promotion must be a boolean", rule_id))

    for reason in DOWNGRADE_REASONS:
        if reason not in covered:
            findings.append(
                Finding("error", "rule.reason_uncovered", "downgrade reason has no rule watching for it", reason)
            )
    return findings


def validate_mixed_version(row: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings: list[Finding] = []
    row_id = str(row.get("row_id", "<row>"))
    scope = row.get("row_scope")
    mixed = row.get("mixed_version")
    reasons = row.get("active_downgrade_reasons", [])
    if not isinstance(reasons, list):
        reasons = []

    if requires_mixed_version(scope):
        if not isinstance(mixed, dict):
            findings.append(
                Finding(
                    "error",
                    "row.mixed_version_missing",
                    "a cross-binary lane must carry a mixed-version section",
                    row_id,
                )
            )
            return findings
    else:
        if mixed is not None:
            findings.append(
                Finding(
                    "error",
                    "row.mixed_version_unexpected",
                    "a non-boundary lane must not carry a mixed-version section",
                    row_id,
                )
            )
        return findings

    for field in ("boundary_label", "rationale"):
        if not is_str(mixed.get(field)):
            findings.append(Finding("error", "mixed.empty_field", f"mixed_version.{field} must be non-empty", row_id))
    if mixed.get("boundary_family") not in BOUNDARY_FAMILIES:
        findings.append(Finding("error", "mixed.boundary_invalid", "boundary_family is outside the vocabulary", row_id))
    claimed = mixed.get("claimed_posture")
    effective = mixed.get("effective_posture")
    if claimed not in MIXED_VERSION_POSTURES:
        findings.append(Finding("error", "mixed.claimed_invalid", "claimed_posture is invalid", row_id))
    if effective not in MIXED_VERSION_POSTURES:
        findings.append(Finding("error", "mixed.effective_invalid", "effective_posture is invalid", row_id))
    behavior = mixed.get("unsupported_state_behavior")
    if isinstance(behavior, dict) and behavior.get("out_of_window_posture") not in OUT_OF_WINDOW_POSTURES:
        findings.append(Finding("error", "mixed.out_of_window_invalid", "out_of_window_posture is invalid", row_id))

    # No widening: the effective posture may not be stronger than the claimed one.
    if (
        claimed in POSTURE_RANK
        and effective in POSTURE_RANK
        and POSTURE_RANK[effective] > POSTURE_RANK[claimed]
    ):
        findings.append(
            Finding("error", "mixed.posture_widened", "effective_posture is wider than claimed_posture", row_id)
        )

    complete = mixed_is_complete(mixed)

    # The v15 core: a boundary lacking complete negotiation data is
    # coordinated-upgrade-only and must name the incompleteness reason.
    if not complete:
        if effective != "coordinated_upgrade_only":
            findings.append(
                Finding(
                    "error",
                    "row.incomplete_mixed_not_coordinated",
                    "a mixed-version section without complete data must be coordinated_upgrade_only",
                    row_id,
                )
            )
        if "mixed_version_data_incomplete" not in reasons:
            findings.append(
                Finding(
                    "error",
                    "row.incomplete_mixed_without_reason",
                    "a mixed-version section without complete data must name mixed_version_data_incomplete",
                    row_id,
                )
            )

    # A Stable mixed-version claim rides only complete data on a row that holds stable.
    if posture_is_stable_mixed_claim(effective):
        if not complete:
            findings.append(
                Finding(
                    "error",
                    "row.stable_mixed_without_complete_data",
                    "a Stable mixed-version claim must ride complete negotiation data",
                    row_id,
                )
            )
        if not is_above_cutline(row.get("effective_level")):
            findings.append(
                Finding(
                    "error",
                    "row.stable_mixed_on_narrowed_row",
                    "a narrowed row may not inherit a Stable mixed-version claim",
                    row_id,
                )
            )

    # Mixed-version evidence refs that are repo paths must exist.
    for ref in mixed.get("evidence_refs", []) or []:
        if is_str(ref):
            base = ref.split("#", 1)[0]
            if "/" in base and not (repo_root / base).exists():
                findings.append(
                    Finding("error", "mixed.ref_missing", f"referenced artifact does not exist: {base}", row_id)
                )
    return findings


def validate_row(row: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings: list[Finding] = []
    row_id = str(row.get("row_id", "<row>"))

    for field in ("row_id", "title", "subject_family", "rationale"):
        if not is_str(row.get(field)):
            findings.append(Finding("error", "row.empty_field", f"row {field} must be non-empty", row_id))
    if row.get("row_scope") not in ROW_SCOPES:
        findings.append(Finding("error", "row.scope_invalid", "row_scope is invalid", row_id))

    claimed = row.get("claimed_level")
    effective = row.get("effective_level")
    state = row.get("qualification_state")
    reasons = row.get("active_downgrade_reasons", [])
    if not isinstance(reasons, list):
        reasons = []

    if claimed not in CLAIM_LEVELS:
        findings.append(Finding("error", "row.claimed_invalid", "claimed_level is invalid", row_id))
    elif not is_above_cutline(claimed):
        findings.append(Finding("error", "row.claimed_below_cutline", "claimed_level must be at or above the cutline", row_id))
    if effective not in CLAIM_LEVELS:
        findings.append(Finding("error", "row.effective_invalid", "effective_level is invalid", row_id))
    if state not in QUALIFICATION_STATES:
        findings.append(Finding("error", "row.state_invalid", "qualification_state is invalid", row_id))
    if any(reason not in DOWNGRADE_REASONS for reason in reasons):
        findings.append(Finding("error", "row.reason_invalid", "active_downgrade_reasons has an unknown reason", row_id))

    if claimed in LEVEL_RANK and effective in LEVEL_RANK and LEVEL_RANK[effective] > LEVEL_RANK[claimed]:
        findings.append(Finding("error", "row.effective_wider_than_claimed", "effective_level is wider than claimed_level", row_id))

    evidence = row.get("evidence")
    if not isinstance(evidence, dict):
        findings.append(Finding("error", "row.evidence_missing", "row must carry an evidence block", row_id))
        evidence = {}
    if not is_str(evidence.get("proof_index_ref")):
        findings.append(Finding("error", "row.proof_index_missing", "evidence.proof_index_ref must be non-empty", row_id))
    window = evidence.get("freshness_window_days")
    if not isinstance(window, int) or isinstance(window, bool) or window < 1:
        findings.append(Finding("error", "row.freshness_window_invalid", "freshness_window_days must be an integer >= 1", row_id))

    holds_stable = is_above_cutline(effective)

    if state in NARROWING_STATES:
        if holds_stable:
            findings.append(Finding("error", "row.effective_not_narrowed", "a lane that is not stable-qualified must narrow below the cutline", row_id))
        if not reasons:
            findings.append(Finding("error", "row.narrowing_without_reason", "a narrowing lane must name an active downgrade reason", row_id))
    if holds_stable:
        if state in NARROWING_STATES:
            findings.append(Finding("error", "row.stable_with_narrowing_state", "a held stable claim must not carry a narrowing state", row_id))
        if reasons:
            findings.append(Finding("error", "row.stable_with_active_downgrade", "a held stable claim must carry no active downgrade reason", row_id))
        if not evidence.get("evidence_refs"):
            findings.append(Finding("error", "row.stable_without_evidence", "a held stable claim must name qualification evidence", row_id))
        signoff = row.get("owner_signoff", {})
        if not (signoff.get("signed_off") is True and is_str(signoff.get("signed_at"))):
            findings.append(Finding("error", "row.stable_without_signoff", "a held stable claim must carry an owner sign-off with a date", row_id))

    findings.extend(validate_state_reason_coherence(row_id, state, reasons, row))
    findings.extend(validate_refs_exist(row_id, evidence, repo_root))
    findings.extend(validate_mixed_version(row, repo_root))
    return findings


def validate_state_reason_coherence(
    row_id: str, state: Any, reasons: list[str], row: dict[str, Any]
) -> list[Finding]:
    findings: list[Finding] = []
    # not_qualified covers two structural failures: the surface proof is missing,
    # or — for a cross-binary boundary — the mixed-version claim could not be
    # backed.
    if state == "not_qualified" and not (
        "qualification_evidence_missing" in reasons or "mixed_version_data_incomplete" in reasons
    ):
        findings.append(Finding("error", "row.state_reason_incoherent", "not_qualified requires a qualification-gap or mixed-version-incomplete reason", row_id))
    if state == "evidence_stale" and not (
        "qualification_evidence_stale" in reasons or "freshness_window_exceeded" in reasons
    ):
        findings.append(Finding("error", "row.state_reason_incoherent", "evidence_stale requires a staleness reason", row_id))
    if state == "waiver_expired":
        if "waiver_expired" not in reasons:
            findings.append(Finding("error", "row.state_reason_incoherent", "waiver_expired requires the waiver_expired reason", row_id))
        if not isinstance(row.get("waiver"), dict):
            findings.append(Finding("error", "row.waiver_state_without_waiver", "waiver_expired state must name a waiver", row_id))
    if state == "provisional_on_waiver":
        waiver = row.get("waiver")
        if not isinstance(waiver, dict) or not is_str(waiver.get("waiver_ref")) or not is_str(waiver.get("expires_at")):
            findings.append(Finding("error", "row.waiver_state_without_waiver", "provisional_on_waiver state must name a waiver", row_id))
    return findings


def validate_refs_exist(row_id: str, evidence: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings: list[Finding] = []
    refs: list[str] = []
    proof_ref = evidence.get("proof_index_ref")
    if is_str(proof_ref):
        refs.append(proof_ref.split("#", 1)[0])
    for ref in evidence.get("evidence_refs", []) or []:
        if is_str(ref):
            refs.append(ref.split("#", 1)[0])
    for ref in refs:
        if not (repo_root / ref).exists():
            findings.append(Finding("error", "row.ref_missing", f"referenced artifact does not exist: {ref}", row_id))
    return findings


def validate_dates(matrix: dict[str, Any]) -> list[Finding]:
    """Date arithmetic the typed model cannot do: against as_of, fail when a row
    overstates its posture on an expired waiver or stale evidence."""
    findings: list[Finding] = []
    as_of = parse_date(matrix.get("as_of"))
    if as_of is None:
        return findings
    for row in matrix.get("rows", []):
        row_id = str(row.get("row_id", "<row>"))
        state = row.get("qualification_state")

        waiver = row.get("waiver")
        if isinstance(waiver, dict):
            expires = parse_date(waiver.get("expires_at"))
            if expires is None:
                findings.append(Finding("error", "row.waiver_expiry_invalid", "waiver expires_at must be an ISO date", row_id))
            else:
                expired = as_of >= expires
                if expired and state == "provisional_on_waiver":
                    findings.append(Finding("error", "row.provisional_on_expired_waiver", "a provisional claim relies on a waiver that has expired against as_of", row_id))
                if not expired and state == "waiver_expired":
                    findings.append(Finding("error", "row.waiver_expired_but_active", "a row is marked waiver_expired but the waiver is still active against as_of", row_id))

        evidence = row.get("evidence", {})
        captured = parse_date(evidence.get("captured_at")) if isinstance(evidence, dict) else None
        window = evidence.get("freshness_window_days") if isinstance(evidence, dict) else None
        if captured is not None and isinstance(window, int) and not isinstance(window, bool):
            stale = as_of > captured + dt.timedelta(days=window)
            if stale and state in HOLDING_STATES:
                findings.append(Finding("error", "row.evidence_stale_but_claimed", "a held stable claim rides evidence past its freshness window against as_of", row_id))
    return findings


def validate_boundary_coverage(matrix: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    covered: set[str] = set()
    for row in matrix.get("rows", []):
        mixed = row.get("mixed_version")
        if isinstance(mixed, dict) and is_str(mixed.get("boundary_family")):
            covered.add(mixed["boundary_family"])
    for family in BOUNDARY_FAMILIES:
        if family not in covered:
            findings.append(Finding("error", "matrix.boundary_uncovered", "boundary family is covered by no cross-binary row", family))
    return findings


def validate_promotion(matrix: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    promotion = matrix.get("promotion")
    if not isinstance(promotion, dict):
        findings.append(Finding("error", "promotion.missing", "matrix must carry a promotion block", "<matrix>"))
        return findings
    for field in ("promotion_gate", "rationale"):
        if not is_str(promotion.get(field)):
            findings.append(Finding("error", "promotion.empty_field", f"promotion.{field} must be non-empty", "<matrix>"))
    decision, rule_ids, row_ids = computed_promotion(matrix)
    if promotion.get("decision") != decision:
        findings.append(
            Finding("error", "promotion.decision_inconsistent", f"declared decision {promotion.get('decision')!r} disagrees with computed {decision!r}", "<matrix>")
        )
    if list(promotion.get("blocking_rule_ids", [])) != rule_ids:
        findings.append(Finding("error", "promotion.blocking_rules_mismatch", "blocking_rule_ids disagrees with the firing downgrade rules", "<matrix>"))
    if list(promotion.get("blocking_row_ids", [])) != row_ids:
        findings.append(Finding("error", "promotion.blocking_rows_mismatch", "blocking_row_ids disagrees with the firing downgrade rules", "<matrix>"))
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
            findings.append(Finding("error", "summary.count_mismatch", f"summary.{key} must equal {value}", key))
    return findings


def validate_matrix(matrix: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings = validate_envelope(matrix)
    findings.extend(validate_rules(matrix))

    rows = matrix.get("rows")
    if not isinstance(rows, list) or not rows:
        findings.append(Finding("error", "matrix.rows_empty", "matrix must enumerate at least one row", "<matrix>"))
        return findings

    seen: set[str] = set()
    for raw in rows:
        row = ensure_dict(raw, "matrix.rows[]")
        findings.extend(validate_row(row, repo_root))
        row_id = str(row.get("row_id", "<row>"))
        if row_id in seen:
            findings.append(Finding("error", "row.duplicate_id", "row ids must be unique", row_id))
        seen.add(row_id)

    findings.extend(validate_boundary_coverage(matrix))
    findings.extend(validate_dates(matrix))
    findings.extend(validate_promotion(matrix))
    findings.extend(validate_summary(matrix))
    return findings


def _resync(mutated: dict[str, Any]) -> None:
    mutated["summary"] = computed_summary(mutated)
    decision, rule_ids, row_ids = computed_promotion(mutated)
    mutated["promotion"].update(
        {"decision": decision, "blocking_rule_ids": rule_ids, "blocking_row_ids": row_ids}
    )


def run_negative_drills(matrix: dict[str, Any], repo_root: Path) -> tuple[list[dict[str, str]], list[Finding]]:
    results: list[dict[str, str]] = []
    findings: list[Finding] = []

    def record(drill_id: str, expected_check_id: str, fired: bool) -> None:
        results.append({"drill_id": drill_id, "expected_check_id": expected_check_id, "status": "passed" if fired else "failed"})
        if not fired:
            findings.append(Finding("error", "negative_drill.not_rejected", f"negative drill {drill_id} did not fire", drill_id))

    def check_ids(candidate: dict[str, Any]) -> set[str]:
        return {f.check_id for f in validate_matrix(candidate, repo_root)}

    def first_row(predicate) -> dict[str, Any] | None:
        return next((r for r in matrix["rows"] if predicate(r)), None)

    def is_incomplete_mixed(row: dict[str, Any]) -> bool:
        mv = row.get("mixed_version")
        return isinstance(mv, dict) and not mixed_is_complete(mv)

    # A lane that is not qualified but still holds its claimed level must be flagged.
    mutated = copy.deepcopy(matrix)
    target = next((r for r in mutated["rows"] if r.get("qualification_state") in NARROWING_STATES), None)
    if target is not None:
        target["effective_level"] = target["claimed_level"]
        _resync(mutated)
        record("effective_not_narrowed_rejected", "row.effective_not_narrowed", "row.effective_not_narrowed" in check_ids(mutated))

    # An incomplete mixed-version section that is not coordinated-upgrade-only must be flagged.
    mutated = copy.deepcopy(matrix)
    target = next((r for r in mutated["rows"] if is_incomplete_mixed(r)), None)
    if target is not None:
        target["mixed_version"]["effective_posture"] = "bounded_skew_supported"
        _resync(mutated)
        record("incomplete_mixed_not_coordinated_rejected", "row.incomplete_mixed_not_coordinated", "row.incomplete_mixed_not_coordinated" in check_ids(mutated))

    # A cross-binary lane missing its mixed-version section must be flagged.
    mutated = copy.deepcopy(matrix)
    target = next((r for r in mutated["rows"] if requires_mixed_version(r.get("row_scope"))), None)
    if target is not None:
        target["mixed_version"] = None
        _resync(mutated)
        record("mixed_version_missing_rejected", "row.mixed_version_missing", "row.mixed_version_missing" in check_ids(mutated))

    # A narrowed row that inherits a Stable mixed-version claim must be flagged.
    mutated = copy.deepcopy(matrix)
    target = next(
        (
            r
            for r in mutated["rows"]
            if not is_above_cutline(r.get("effective_level"))
            and isinstance(r.get("mixed_version"), dict)
            and mixed_is_complete(r["mixed_version"])
        ),
        None,
    )
    if target is not None:
        target["mixed_version"]["claimed_posture"] = "bounded_skew_supported"
        target["mixed_version"]["effective_posture"] = "bounded_skew_supported"
        _resync(mutated)
        record("stable_mixed_on_narrowed_rejected", "row.stable_mixed_on_narrowed_row", "row.stable_mixed_on_narrowed_row" in check_ids(mutated))

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

    # A promotion decision that disagrees with the firing rules must be flagged.
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
    decision, rule_ids, row_ids = computed_promotion(matrix)
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "record_kind": "stable_qualification_matrix_validation_capture",
        "status": "pass" if not any(f.severity == "error" for f in findings) else "fail",
        "generated_at": generated_at_now(),
        "matrix_id": matrix.get("matrix_id"),
        "as_of": matrix.get("as_of"),
        "summary": matrix.get("summary"),
        "promotion": {
            "decision": decision,
            "blocking_rule_ids": rule_ids,
            "blocking_row_ids": row_ids,
        },
        "negative_drills": drill_results,
        "findings": [f.as_report() for f in findings],
    }
    path.write_text(json_text(payload), encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    matrix = ensure_dict(
        load_json(repo_root / args.matrix, "stable qualification matrix"),
        "stable qualification matrix",
    )

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
    decision, rule_ids, _row_ids = computed_promotion(matrix)
    print(
        "stable qualification matrix validated "
        f"({summary.get('total_rows')} rows, "
        f"{summary.get('rows_holding_stable')} holding stable, "
        f"{summary.get('rows_narrowed_below_cutline')} narrowed below cutline; "
        f"{summary.get('cross_binary_rows')} cross-binary rows, "
        f"{summary.get('rows_with_stable_mixed_version_claim')} with a stable mixed-version claim, "
        f"{summary.get('coordinated_upgrade_only_rows')} coordinated-upgrade-only; "
        f"{summary.get('downgrade_rules_firing')} rules firing; "
        f"promotion={decision}; {len(drill_results)} negative drills)"
    )

    if args.require_proceed and decision == "hold":
        print(
            f"PROMOTION HELD: stable train blocked by {len(rule_ids)} downgrade rule(s): {', '.join(rule_ids)}",
            file=sys.stderr,
        )
        return 2

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
