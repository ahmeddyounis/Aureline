#!/usr/bin/env python3
"""Enforce the M5 efficiency certification proof packet.

This gate makes the rule "a claimed low-power row may not stay green while its
current efficiency evidence is stale, partial, or missing" mechanically
enforceable. The proof packet certifies power, thermal, battery-efficiency, and
hidden-work-shedding truth for every claimed laptop/desktop profile and
long-running M5 surface family. Each row runs a fixed drill set against bound
energy/thermal, hidden-pane, and session-pressure evidence.

It validates the packet against its schema; re-derives every row's fired
narrowing reasons, narrowed effective posture, certification state, and promotion
blocker from the drill results and the declared vocabulary, so none of those can
be hand-edited; recomputes the promotion gate and summary counts; confirms each
surface-family row aligns with (and never over-claims beyond) its governance
matrix row; checks certified claim-bearing rows publish to every required
surface; rejects any raw-telemetry field that would leak across the boundary; and
runs negative drills proving the recompute fails closed when evidence goes stale,
a drill regresses, or a stored verdict is inflated.

Exit code is 0 when every check passes and 1 when any finding is at severity
``error``.
"""

from __future__ import annotations

import argparse
import copy
import json
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any

try:
    from jsonschema import Draft202012Validator
except Exception as exc:  # pragma: no cover - dependency guard
    raise SystemExit(
        "python jsonschema is required: pip install jsonschema"
    ) from exc


PACKET_REL = "artifacts/efficiency/m5-efficiency-proof-packet.json"
SCHEMA_REL = "schemas/efficiency/m5-efficiency-certification.schema.json"
MATRIX_REL = "artifacts/efficiency/m5-efficiency-governance.json"

# Canonical claim-level ranks; higher is a stronger claim. Mirrors the shell
# certification lane and the governance matrix.
CLAIM_RANK: dict[str, int] = {
    "undeclared_badge": 0,
    "state_declared": 1,
    "qualified_low_power": 2,
    "certified_low_power": 3,
}
CLAIM_BEARING: dict[str, bool] = {
    "undeclared_badge": False,
    "state_declared": False,
    "qualified_low_power": True,
    "certified_low_power": True,
}

# Narrowing reasons in canonical order, with the posture floor each narrows to.
NARROWS_TO: dict[str, str] = {
    "missing_efficiency_evidence": "undeclared_badge",
    "stale_efficiency_evidence": "state_declared",
    "partial_evidence_coverage": "state_declared",
    "unqualified_hidden_work_suppression": "state_declared",
    "protected_path_regression_under_pressure": "state_declared",
    "session_shed_order_violation": "state_declared",
    "recovery_not_staged": "qualified_low_power",
}
REASON_ORDER: tuple[str, ...] = tuple(NARROWS_TO.keys())

# The narrowing reason a present-and-current drill fires when it does not pass.
DRILL_FAILURE_REASON: dict[str, str] = {
    "efficiency_state_behavior": "missing_efficiency_evidence",
    "hidden_work_suppression": "unqualified_hidden_work_suppression",
    "protected_path_preservation": "protected_path_regression_under_pressure",
    "session_aware_shedding": "session_shed_order_violation",
    "staged_recovery": "recovery_not_staged",
}

# The narrowing reason a non-current freshness grade fires.
FRESHNESS_REASON: dict[str, str] = {
    "stale": "stale_efficiency_evidence",
    "partial": "partial_evidence_coverage",
    "missing": "missing_efficiency_evidence",
}

REQUIRED_PUBLICATION_SURFACES: tuple[str, ...] = ("release", "support", "docs", "help")

# Raw telemetry, payload, and content fields that must never cross the boundary
# into the export-safe proof packet.
FORBIDDEN_FIELDS: tuple[str, ...] = (
    "raw_energy_trace",
    "raw_power_samples",
    "raw_thermal_samples",
    "raw_battery_telemetry",
    "raw_log",
    "provider_payload",
    "secret_material",
    "user_content",
    "file_path",
    "machine_label",
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


def add_finding(
    findings: list[Finding],
    check_id: str,
    message: str,
    remediation: str,
    *,
    ref: str | None = None,
    severity: str = "error",
    details: dict[str, Any] | None = None,
) -> None:
    findings.append(
        Finding(
            severity=severity,
            check_id=check_id,
            message=message,
            remediation=remediation,
            ref=ref,
            details=details or {},
        )
    )


def load_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing JSON file: {path}")
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON at {path}: {exc}") from exc


# --------------------------------------------------------------------------- #
# Recompute (the core fail-closed engine).
# --------------------------------------------------------------------------- #


def drill_reason(result: dict[str, Any]) -> str | None:
    """The narrowing reason one drill result fires, from its freshness/outcome."""
    freshness = result.get("freshness")
    if freshness in FRESHNESS_REASON:
        return FRESHNESS_REASON[freshness]
    # Present and current evidence: a failed drill fires its failure reason.
    if not result.get("passed", False):
        return DRILL_FAILURE_REASON.get(result.get("drill"))
    return None


def recompute_row(row: dict[str, Any]) -> dict[str, Any]:
    """Re-derives a row's verdict purely from its drill results and ceiling."""
    fired: list[str] = []
    for reason in REASON_ORDER:
        if any(drill_reason(r) == reason for r in row.get("drill_results", [])):
            fired.append(reason)

    ceiling = row.get("published_claim_ceiling")
    effective = ceiling
    for reason in fired:
        floor = NARROWS_TO[reason]
        if CLAIM_RANK.get(floor, 0) < CLAIM_RANK.get(effective, 0):
            effective = floor

    if not fired:
        cert_state = "certified"
    elif effective == "undeclared_badge":
        cert_state = "quarantined"
    else:
        cert_state = "narrowed"

    blocks = CLAIM_BEARING.get(ceiling, False) and CLAIM_RANK.get(
        effective, 0
    ) < CLAIM_RANK.get(ceiling, 0)

    return {
        "fired_narrowing_reasons": fired,
        "effective_posture": effective,
        "certification_state": cert_state,
        "blocks_promotion": blocks,
        "blocker_reasons": fired if blocks else [],
    }


# --------------------------------------------------------------------------- #
# Validation passes.
# --------------------------------------------------------------------------- #


def validate_schema(repo_root: Path, packet: dict[str, Any], findings: list[Finding]) -> None:
    schema = load_json(repo_root / SCHEMA_REL)
    validator = Draft202012Validator(schema)
    for error in sorted(validator.iter_errors(packet), key=lambda e: list(e.path)):
        location = "/".join(str(p) for p in error.path) or "<root>"
        add_finding(
            findings,
            "packet.schema",
            f"packet fails its schema at {location}: {error.message}",
            "Bring the proof packet back into conformance with its boundary schema.",
            ref=PACKET_REL,
        )


def validate_refs(repo_root: Path, packet: dict[str, Any], findings: list[Finding]) -> None:
    if SCHEMA_REL not in packet.get("source_refs", []):
        add_finding(
            findings,
            "packet.source_refs.schema",
            "packet source_refs must cite its own schema",
            f"Add {SCHEMA_REL} to source_refs.",
            ref=PACKET_REL,
        )
    for ref in packet.get("source_refs", []):
        if isinstance(ref, str) and "/" in ref and not (repo_root / ref).exists():
            add_finding(
                findings,
                "packet.source_refs.missing",
                f"packet cites a missing source artifact: {ref}",
                "Seed the referenced artifact or correct the source ref.",
                ref=ref,
            )
    for key in ("matrix_ref", "schema_ref", "matrix_schema_ref"):
        ref = packet.get(key)
        if isinstance(ref, str) and not (repo_root / ref).exists():
            add_finding(
                findings,
                f"packet.{key}.missing",
                f"packet cites a missing artifact for {key}: {ref}",
                "Publish the referenced artifact or correct the ref.",
                ref=ref,
            )


def validate_vocabulary(packet: dict[str, Any], findings: list[Finding]) -> None:
    declared_levels = {row["level"]: row for row in packet.get("claim_levels", [])}
    for level, rank in CLAIM_RANK.items():
        row = declared_levels.get(level)
        if row is None:
            add_finding(
                findings,
                "vocab.claim_level_missing",
                f"claim_levels omits {level}",
                "Declare every claim level the lane uses.",
                ref=PACKET_REL,
            )
            continue
        if int(row.get("rank", -1)) != rank or bool(row.get("claim_bearing")) != CLAIM_BEARING[level]:
            add_finding(
                findings,
                "vocab.claim_level_mismatch",
                f"claim_level {level} rank/claim_bearing disagrees with the canonical vocabulary",
                "Align the declared claim level with the canonical rank and claim-bearing flag.",
                ref=PACKET_REL,
            )

    declared_reasons = {row["reason"]: row for row in packet.get("narrowing_reasons", [])}
    for reason, floor in NARROWS_TO.items():
        row = declared_reasons.get(reason)
        if row is None:
            add_finding(
                findings,
                "vocab.narrowing_reason_missing",
                f"narrowing_reasons omits {reason}",
                "Declare every narrowing reason the lane fires.",
                ref=PACKET_REL,
            )
            continue
        if row.get("narrows_to") != floor:
            add_finding(
                findings,
                "vocab.narrowing_reason_mismatch",
                f"narrowing reason {reason} narrows_to disagrees with the canonical floor",
                "Align the declared narrowing-reason floor with the canonical vocabulary.",
                ref=PACKET_REL,
                details={"expected": floor},
            )

    declared_drills = {row["drill"]: row for row in packet.get("drills", [])}
    for drill, reason in DRILL_FAILURE_REASON.items():
        row = declared_drills.get(drill)
        if row is None:
            add_finding(
                findings,
                "vocab.drill_missing",
                f"drills omits {drill}",
                "Declare every drill the lane runs.",
                ref=PACKET_REL,
            )
        elif row.get("failure_reason") != reason:
            add_finding(
                findings,
                "vocab.drill_failure_reason_mismatch",
                f"drill {drill} failure_reason disagrees with the canonical vocabulary",
                "Align the declared drill failure reason with the canonical vocabulary.",
                ref=PACKET_REL,
            )


def validate_drill_results(row: dict[str, Any], findings: list[Finding]) -> None:
    row_id = row.get("row_id", "<row>")
    for result in row.get("drill_results", []):
        freshness = result.get("freshness")
        passed = bool(result.get("passed"))
        outcome = result.get("outcome")
        reason = result.get("narrowing_reason")
        expected_reason = drill_reason(result)

        # outcome must agree with freshness/passed.
        if freshness in FRESHNESS_REASON:
            expected_outcome = freshness
        else:
            expected_outcome = "pass" if passed else "fail"
        if outcome != expected_outcome:
            add_finding(
                findings,
                "drill.outcome_mismatch",
                f"row {row_id} drill {result.get('drill')} outcome {outcome} != {expected_outcome}",
                "Set the drill outcome to match its freshness and passed flag.",
                ref=row_id,
            )

        if reason != expected_reason:
            add_finding(
                findings,
                "drill.reason_mismatch",
                f"row {row_id} drill {result.get('drill')} narrowing_reason {reason} != {expected_reason}",
                "Record the narrowing reason the drill's freshness/outcome implies, or none when it passed.",
                ref=row_id,
            )

        if passed and freshness != "current":
            add_finding(
                findings,
                "drill.passed_on_stale_evidence",
                f"row {row_id} drill {result.get('drill')} is marked passed on {freshness} evidence",
                "A drill may only pass against current evidence.",
                ref=row_id,
            )


def validate_rows(packet: dict[str, Any], findings: list[Finding]) -> None:
    seen: set[str] = set()
    for row in packet.get("rows", []):
        row_id = row.get("row_id", "<row>")
        if row_id in seen:
            add_finding(
                findings,
                "row.duplicate_id",
                f"duplicate row id {row_id}",
                "Row ids must be unique.",
                ref=row_id,
            )
        seen.add(row_id)

        validate_drill_results(row, findings)

        recomputed = recompute_row(row)
        if list(row.get("fired_narrowing_reasons", [])) != recomputed["fired_narrowing_reasons"]:
            add_finding(
                findings,
                "row.fired_reasons_mismatch",
                f"row {row_id} fired_narrowing_reasons disagrees with the recompute",
                "Fire exactly the narrowing reasons the drill results imply, in canonical order.",
                ref=row_id,
                details={"recomputed": recomputed["fired_narrowing_reasons"]},
            )
        if row.get("effective_posture") != recomputed["effective_posture"]:
            add_finding(
                findings,
                "row.effective_posture_mismatch",
                f"row {row_id} effective_posture {row.get('effective_posture')} != {recomputed['effective_posture']}",
                "Set the effective posture to the lowest of the ceiling and every fired reason's floor.",
                ref=row_id,
            )
        if row.get("certification_state") != recomputed["certification_state"]:
            add_finding(
                findings,
                "row.certification_state_mismatch",
                f"row {row_id} certification_state {row.get('certification_state')} != {recomputed['certification_state']}",
                "Derive the certification state from the fired reasons and the effective posture.",
                ref=row_id,
            )
        blocker = row.get("promotion_blocker") or {}
        if bool(blocker.get("blocks_promotion")) != recomputed["blocks_promotion"]:
            add_finding(
                findings,
                "row.blocker_mismatch",
                f"row {row_id} blocks_promotion {blocker.get('blocks_promotion')} != {recomputed['blocks_promotion']}",
                "A claim-bearing ceiling narrowed below itself holds promotion; nothing else does.",
                ref=row_id,
            )
        if sorted(blocker.get("blocker_reasons", [])) != sorted(recomputed["blocker_reasons"]):
            add_finding(
                findings,
                "row.blocker_reasons_mismatch",
                f"row {row_id} blocker_reasons disagrees with the recompute",
                "List exactly the reasons that narrowed a claim-bearing row below its ceiling.",
                ref=row_id,
            )

        # Certified claim-bearing rows must publish to every required surface.
        ceiling = row.get("published_claim_ceiling")
        certified = row.get("certification_state") == "certified"
        targets = row.get("publication_targets", [])
        if certified and CLAIM_BEARING.get(ceiling, False):
            if sorted(targets) != sorted(REQUIRED_PUBLICATION_SURFACES):
                add_finding(
                    findings,
                    "row.publication_incomplete",
                    f"certified claim-bearing row {row_id} does not publish to every required surface",
                    "Publish a certified claim-bearing row to release, support, docs, and help.",
                    ref=row_id,
                    details={"required": list(REQUIRED_PUBLICATION_SURFACES)},
                )
        elif targets:
            add_finding(
                findings,
                "row.publication_overreach",
                f"row {row_id} publishes a claim it has not certified",
                "Only a certified claim-bearing row publishes to the required surfaces.",
                ref=row_id,
            )


def validate_promotion_gate(packet: dict[str, Any], findings: list[Finding]) -> None:
    blocking = sorted(
        row.get("row_id")
        for row in packet.get("rows", [])
        if recompute_row(row)["blocks_promotion"]
    )
    gate = packet.get("promotion_gate") or {}
    expected_decision = "hold" if blocking else "proceed"
    if gate.get("decision") != expected_decision:
        add_finding(
            findings,
            "gate.decision_mismatch",
            f"promotion gate decision {gate.get('decision')} != {expected_decision}",
            "Hold promotion when any claim-bearing row narrows below its ceiling; otherwise proceed.",
            ref=PACKET_REL,
        )
    if sorted(gate.get("blocking_row_ids", [])) != blocking:
        add_finding(
            findings,
            "gate.blocking_rows_mismatch",
            "promotion gate blocking_row_ids disagrees with the recompute",
            "List exactly the rows that hold promotion.",
            ref=PACKET_REL,
            details={"recomputed": blocking},
        )


def validate_summary_counts(packet: dict[str, Any], findings: list[Finding]) -> None:
    rows = packet.get("rows", [])
    expected = {
        "total_rows": len(rows),
        "rows_certified": sum(1 for r in rows if r.get("certification_state") == "certified"),
        "rows_narrowed": sum(1 for r in rows if r.get("certification_state") == "narrowed"),
        "rows_quarantined": sum(1 for r in rows if r.get("certification_state") == "quarantined"),
        "profile_rows": sum(1 for r in rows if r.get("subject_kind") == "laptop_or_desktop_profile"),
        "surface_family_rows": sum(1 for r in rows if r.get("subject_kind") == "m5_surface_family"),
        "claim_bearing_rows": sum(
            1 for r in rows if CLAIM_BEARING.get(r.get("published_claim_ceiling"), False)
        ),
        "rows_blocking_promotion": sum(1 for r in rows if recompute_row(r)["blocks_promotion"]),
    }
    counts = packet.get("summary_counts") or {}
    for key, value in expected.items():
        if counts.get(key) != value:
            add_finding(
                findings,
                "summary.count_mismatch",
                f"summary_counts.{key} {counts.get(key)} != {value}",
                "Recompute the summary counts from the rows.",
                ref=PACKET_REL,
            )


def validate_governance_alignment(
    repo_root: Path, packet: dict[str, Any], findings: list[Finding]
) -> None:
    matrix = load_json(repo_root / MATRIX_REL)
    matrix_rows = {
        r.get("row_id"): r for r in matrix.get("rows", []) if isinstance(r, dict)
    }
    for row in packet.get("rows", []):
        if row.get("subject_kind") != "m5_surface_family":
            continue
        gov_ref = row.get("governance_row_ref")
        if not gov_ref:
            add_finding(
                findings,
                "alignment.missing_governance_ref",
                f"surface-family row {row.get('row_id')} cites no governance row",
                "Point governance_row_ref at the matching efficiency governance matrix row.",
                ref=row.get("row_id"),
            )
            continue
        gov_row = matrix_rows.get(gov_ref)
        if gov_row is None:
            add_finding(
                findings,
                "alignment.unresolved_governance_ref",
                f"surface-family row {row.get('row_id')} cites unknown governance row {gov_ref}",
                "Cite a real governance matrix row id.",
                ref=row.get("row_id"),
            )
            continue
        gov_effective = gov_row.get("effective_posture")
        cert_effective = row.get("effective_posture")
        if CLAIM_RANK.get(cert_effective, 0) > CLAIM_RANK.get(gov_effective, 0):
            add_finding(
                findings,
                "alignment.over_claims_governance",
                f"row {row.get('row_id')} certifies {cert_effective} above the governance posture {gov_effective}",
                "A certification row may narrow, but never inflate, the surface's governed claim.",
                ref=row.get("row_id"),
            )


def scan_forbidden_fields(node: Any, findings: list[Finding], path: str = "<root>") -> None:
    if isinstance(node, dict):
        for key, value in node.items():
            if key in FORBIDDEN_FIELDS:
                add_finding(
                    findings,
                    "boundary.forbidden_field",
                    f"proof packet carries forbidden field {key} at {path}",
                    "Raw energy, power, thermal, battery, log, provider, content, and secret data never cross this boundary.",
                    ref=PACKET_REL,
                )
            scan_forbidden_fields(value, findings, f"{path}/{key}")
    elif isinstance(node, list):
        for index, item in enumerate(node):
            scan_forbidden_fields(item, findings, f"{path}[{index}]")


def validate_packet(
    repo_root: Path, packet: dict[str, Any], findings: list[Finding]
) -> None:
    validate_schema(repo_root, packet, findings)
    validate_refs(repo_root, packet, findings)
    validate_vocabulary(packet, findings)
    validate_rows(packet, findings)
    validate_promotion_gate(packet, findings)
    validate_summary_counts(packet, findings)
    validate_governance_alignment(repo_root, packet, findings)
    scan_forbidden_fields(packet, findings)


# --------------------------------------------------------------------------- #
# Negative drills.
# --------------------------------------------------------------------------- #


def run_negative_drills(
    repo_root: Path, packet: dict[str, Any], findings: list[Finding]
) -> list[dict[str, str]]:
    results: list[dict[str, str]] = []

    def check_ids(candidate: dict[str, Any]) -> set[str]:
        local: list[Finding] = []
        validate_packet(repo_root, candidate, local)
        return {f.check_id for f in local}

    def record(drill_id: str, expected: str, fired: bool) -> None:
        results.append(
            {
                "drill_id": drill_id,
                "expected_check_id": expected,
                "status": "passed" if fired else "failed",
            }
        )
        if not fired:
            add_finding(
                findings,
                "negative_drill.not_rejected",
                f"negative drill {drill_id} did not fire {expected}",
                "The recompute must reject this mutation.",
                ref=drill_id,
            )

    def claim_bearing_row(candidate: dict[str, Any]) -> dict[str, Any] | None:
        return next(
            (
                r
                for r in candidate["rows"]
                if CLAIM_BEARING.get(r.get("published_claim_ceiling"), False)
                and r.get("certification_state") == "certified"
            ),
            None,
        )

    # 1. Stale evidence on a certified claim-bearing row must narrow it and hold
    #    promotion — a claim cannot outrun its evidence.
    mutated = copy.deepcopy(packet)
    target = claim_bearing_row(mutated)
    if target is not None:
        target["drill_results"][0]["freshness"] = "stale"
        target["drill_results"][0]["outcome"] = "stale"
        target["drill_results"][0]["passed"] = False
        target["drill_results"][0]["narrowing_reason"] = "stale_efficiency_evidence"
        record(
            "stale_evidence_blocks_claim",
            "gate.decision_mismatch",
            "gate.decision_mismatch" in check_ids(mutated),
        )

    # 2. A protected-path drill that regressed must narrow the row.
    mutated = copy.deepcopy(packet)
    target = claim_bearing_row(mutated)
    if target is not None:
        for result in target["drill_results"]:
            if result["drill"] == "protected_path_preservation":
                result["passed"] = False
                result["outcome"] = "fail"
                result["narrowing_reason"] = "protected_path_regression_under_pressure"
        record(
            "protected_path_regression_narrows",
            "row.effective_posture_mismatch",
            "row.effective_posture_mismatch" in check_ids(mutated),
        )

    # 3. An inflated stored effective posture must be caught by the recompute.
    mutated = copy.deepcopy(packet)
    target = next(
        (r for r in mutated["rows"] if r.get("certification_state") == "quarantined"),
        None,
    )
    if target is not None:
        target["effective_posture"] = "certified_low_power"
        record(
            "quarantine_reinflated",
            "row.effective_posture_mismatch",
            "row.effective_posture_mismatch" in check_ids(mutated),
        )

    # 4. A surface row over-claiming beyond its governance posture must be caught.
    mutated = copy.deepcopy(packet)
    matrix = load_json(repo_root / MATRIX_REL)
    matrix_rows = {r.get("row_id"): r for r in matrix.get("rows", [])}
    target = next(
        (
            r
            for r in mutated["rows"]
            if r.get("subject_kind") == "m5_surface_family"
            and CLAIM_RANK.get(
                matrix_rows.get(r.get("governance_row_ref"), {}).get("effective_posture"),
                0,
            )
            < 3
        ),
        None,
    )
    if target is not None:
        # Force the row to certify above its governance posture without tripping
        # the recompute (raise ceiling and effective together).
        target["published_claim_ceiling"] = "certified_low_power"
        target["effective_posture"] = "certified_low_power"
        target["certification_state"] = "certified"
        record(
            "surface_over_claims_governance",
            "alignment.over_claims_governance",
            "alignment.over_claims_governance" in check_ids(mutated),
        )

    # 5. Dropping a publication target from a certified claim-bearing row.
    mutated = copy.deepcopy(packet)
    target = claim_bearing_row(mutated)
    if target is not None and target.get("publication_targets"):
        target["publication_targets"] = target["publication_targets"][:-1]
        record(
            "publication_target_dropped",
            "row.publication_incomplete",
            "row.publication_incomplete" in check_ids(mutated),
        )

    # 6. A leaked raw-telemetry field must be rejected at the boundary.
    mutated = copy.deepcopy(packet)
    mutated["rows"][0]["drill_results"][0]["raw_energy_trace"] = "leaked"
    record(
        "raw_telemetry_leaked",
        "boundary.forbidden_field",
        "boundary.forbidden_field" in check_ids(mutated),
    )

    # 7. A drill marked passed on stale evidence must be rejected.
    mutated = copy.deepcopy(packet)
    mutated["rows"][0]["drill_results"][0]["freshness"] = "stale"
    mutated["rows"][0]["drill_results"][0]["passed"] = True
    record(
        "passed_on_stale_evidence",
        "drill.passed_on_stale_evidence",
        "drill.passed_on_stale_evidence" in check_ids(mutated),
    )

    return results


# --------------------------------------------------------------------------- #
# Entry point.
# --------------------------------------------------------------------------- #


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument(
        "--report",
        default=None,
        help="Write the machine-readable JSON report to this repo-relative path.",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(f"--repo-root does not look like a repository root: {repo_root}")

    findings: list[Finding] = []
    packet = load_json(repo_root / PACKET_REL)
    if not isinstance(packet, dict):
        raise SystemExit("proof packet must be a JSON object")

    validate_packet(repo_root, packet, findings)
    drill_results = run_negative_drills(repo_root, packet, findings)

    errors = [f for f in findings if f.severity == "error"]
    warnings = [f for f in findings if f.severity == "warning"]
    status = "PASS" if not errors else "FAIL"
    print(
        f"[efficiency-certification] {status} "
        f"({len(errors)} errors, {len(warnings)} warnings) -- "
        f"rows: {len(packet.get('rows', []))}, "
        f"promotion: {packet.get('promotion_gate', {}).get('decision')}, "
        f"drills: {len(drill_results)}, as_of: {packet.get('as_of')}"
    )
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        suffix = f" [{finding.ref}]" if finding.ref else ""
        print(f"[efficiency-certification] {prefix} {finding.check_id}: {finding.message}{suffix}")
        print(f"[efficiency-certification]   remediation: {finding.remediation}")

    if args.report:
        report = {
            "schema_version": 1,
            "check_id": "efficiency_certification",
            "evaluated_on": packet.get("as_of"),
            "status": "pass" if not errors else "fail",
            "packet_ref": PACKET_REL,
            "row_count": len(packet.get("rows", [])),
            "promotion_decision": packet.get("promotion_gate", {}).get("decision"),
            "drill_count": len(drill_results),
            "negative_drills": drill_results,
            "finding_counts": {"error": len(errors), "warning": len(warnings)},
            "findings": [f.as_report() for f in findings],
        }
        report_path = repo_root / args.report
        report_path.parent.mkdir(parents=True, exist_ok=True)
        report_path.write_text(
            json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8"
        )

    return 1 if errors else 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[efficiency-certification] interrupted", file=sys.stderr)
        sys.exit(130)
