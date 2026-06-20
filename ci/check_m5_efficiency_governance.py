#!/usr/bin/env python3
"""Enforce the canonical M5 efficiency-state governance matrix.

This gate turns the M5 low-power, thermal, battery-efficiency, and hidden-pane
render-suppression contract into a single promotion-grade governance lane. It
validates the matrix against its schema, confirms the frozen closed
vocabularies (efficiency state, source-of-change, throttled subsystem,
hidden-pane behaviour, visibility state, override posture, and recovery state)
match the canonical token sets, and recomputes for every M5 surface row the
narrowing reasons that fire, the narrowed effective posture, and the
certification state directly from its inline efficiency-state evidence,
hidden-work suppression, protected-path preservation, override policy-awareness,
recovery staging, and consumer propagation. It fails closed when a stored value
drifts from the recompute, when a green tile would mask a fired reason, and when
the recomputed promotion verdict disagrees with the stored one; it holds
promotion when a claim-bearing row narrows below the posture it asserts; and it
replays the recompute fixtures and negative drills that prove each fail-closed
path.

Exit code is 0 when every check passes and 1 when any finding is at severity
``error``. With ``--require-proceed`` the gate also fails (exit code 2) when the
recomputed promotion verdict is ``hold``.
"""

from __future__ import annotations

import argparse
import copy
import json
import subprocess
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


MATRIX_REL = "artifacts/efficiency/m5-efficiency-governance.json"
MATRIX_SCHEMA_REL = "schemas/efficiency/m5-efficiency-governance.schema.json"
FIXTURE_SCHEMA_REL = "schemas/efficiency/m5-efficiency-governance-fixture.schema.json"
FIXTURE_REGISTER_REL = "fixtures/efficiency/m5-efficiency-governance/manifest.yaml"

CLAIM_BEARING_LEVELS = {"qualified_low_power", "certified_low_power"}
USER_OVERRIDABLE = {"user_override_session_only", "user_override_persistent"}

PILLARS = (
    "efficiency_state_evidence",
    "behavior_declaration",
    "hidden_work_suppression",
    "protected_path_preservation",
    "override_policy_awareness",
    "recovery_staging",
    "consumer_propagation",
)

# Each narrowing reason's owning pillar, so the cross-check of stored
# dimension_findings can map a fired reason back to the pillar that owns it.
REASON_PILLAR = {
    "missing_efficiency_state_evidence": "efficiency_state_evidence",
    "vague_low_power_badge": "behavior_declaration",
    "unqualified_hidden_work_suppression": "hidden_work_suppression",
    "protected_path_regression_under_pressure": "protected_path_preservation",
    "override_not_policy_aware": "override_policy_awareness",
    "recovery_not_staged": "recovery_staging",
    "missing_consumer_propagation": "consumer_propagation",
}

# The canonical closed vocabularies. These mirror the shell efficiency runtime
# (`crates/aureline-shell/src/efficiency/`) so the frozen matrix can never drift
# from the tokens that ship.
CANONICAL_VOCAB = {
    "efficiency_state": [
        "Nominal",
        "EfficiencyAware",
        "ThermalConstrained",
        "ProtectCore",
        "Recovery",
    ],
    "source_of_change": [
        "ac_power",
        "battery",
        "os_battery_saver",
        "user_low_power_mode",
        "low_battery",
        "critical_battery",
        "thermal_pressure",
        "frame_miss_pressure",
        "policy_cap",
        "pressure_cleared",
    ],
    "throttled_subsystem": [
        "ai_warmup",
        "speculative_prefetch",
        "upload_transfer",
        "non_essential_animation",
        "indexing_refresh",
        "extension_polling",
        "preview_refresh",
        "graph_enrichment",
        "remote_session_helper",
    ],
    "hidden_pane_behavior": [
        "render_suppressed",
        "animation_suppressed",
        "polling_paused",
        "correctness_poll_only",
        "fully_quiescent",
    ],
    "visibility_state": [
        "visible_focused",
        "visible_background",
        "occluded_window",
        "hidden_tab",
        "collapsed_split",
        "detached_offscreen",
    ],
    "override_posture": [
        "not_overridable",
        "user_override_session_only",
        "user_override_persistent",
        "policy_blocked",
        "admin_controlled",
    ],
    "recovery_state": [
        "not_in_recovery",
        "staged_resume",
        "awaiting_user_restore_power",
        "awaiting_reconnect",
        "awaiting_admin_policy",
        "recovered",
    ],
}

REQUIRED_CONSUMERS = {"release_promotion", "release_packet", "support_export", "docs_help"}


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
    parser.add_argument(
        "--report",
        default=None,
        help="Write the machine-readable JSON report to this repo-relative path.",
    )
    parser.add_argument(
        "--require-proceed",
        action="store_true",
        help="Publication gate: also fail (exit 2) when the recomputed verdict is hold.",
    )
    return parser.parse_args()


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
                "payload = YAML.safe_load(File.read(ARGV[0]), "
                "permitted_classes: [], aliases: false); "
                "STDOUT.write(JSON.generate(payload))"
            ),
            str(path),
        ],
        capture_output=True,
        text=True,
    )
    if ruby.returncode != 0:
        stderr = ruby.stderr.strip() or "unknown Ruby/Psych failure"
        raise SystemExit(f"failed to parse YAML at {path}: {stderr}")
    try:
        return json.loads(ruby.stdout)
    except json.JSONDecodeError as exc:
        raise SystemExit(f"Ruby/Psych emitted invalid JSON for {path}: {exc}") from exc


# --------------------------------------------------------------------------- #
# Governance engine. The same recompute backs the matrix rows and each fixture,
# so the narrowing logic is identical in both paths.
# --------------------------------------------------------------------------- #


class GovernanceEngine:
    """Recompute fired narrowing reasons, the narrowed posture, and the state."""

    def __init__(self, matrix: dict[str, Any]) -> None:
        self.rank = {row["level"]: row["rank"] for row in matrix.get("claim_levels", [])}
        self.narrows_to = {
            row["reason"]: row["narrows_to"]
            for row in matrix.get("narrowing_reasons", [])
        }

    def pillar_gaps(self, row: dict[str, Any]) -> dict[str, str | None]:
        gap: dict[str, str | None] = {p: None for p in PILLARS}
        ev = row.get("evidence", {})

        # 1. Efficiency-state evidence: the row must materialize an inspectable
        #    efficiency-state record with a named state and source-of-change.
        if (
            not ev.get("efficiency_state_evidence_present")
            or not row.get("efficiency_state")
            or not row.get("source_of_change")
        ):
            gap["efficiency_state_evidence"] = "missing_efficiency_state_evidence"

        # 2. Behaviour declaration (the vague-badge guardrail): a low-power state
        #    may not be a badge with no declared behaviour change.
        if not ev.get("declares_behavior_change"):
            gap["behavior_declaration"] = "vague_low_power_badge"

        # 3. Hidden-work suppression: a row that binds hidden or off-screen panes
        #    must prove qualified suppression with no committed render work.
        if ev.get("binds_hidden_panes"):
            if (
                not ev.get("hidden_work_suppression_qualified")
                or int(ev.get("hidden_pane_render_violation_count", 0)) > 0
            ):
                gap["hidden_work_suppression"] = "unqualified_hidden_work_suppression"

        # 4. Protected-path preservation: typing, save, navigation, and review
        #    authority may not regress under pressure.
        if not ev.get("protected_paths_preserved"):
            gap["protected_path_preservation"] = "protected_path_regression_under_pressure"

        # 5. Override policy-awareness: a user-overridable posture must carry an
        #    explicit, policy-aware override reference.
        if row.get("override_posture") in USER_OVERRIDABLE:
            if not ev.get("override_policy_aware") or not str(
                ev.get("override_policy_ref") or ""
            ).strip():
                gap["override_policy_awareness"] = "override_not_policy_aware"

        # 6. Recovery staging: when recovery applies, deferred work must resume in
        #    staged order rather than thrash back at once.
        if ev.get("recovery_required") and not ev.get("recovery_staged"):
            gap["recovery_staging"] = "recovery_not_staged"

        # 7. Consumer propagation: the row's posture must reach every required
        #    publication surface so later copy derives from one source of truth.
        required = set(row.get("required_publication_surfaces", []))
        propagated = set(ev.get("propagated_surfaces", []))
        if not required.issubset(propagated):
            gap["consumer_propagation"] = "missing_consumer_propagation"

        return gap

    def recompute(self, row: dict[str, Any]) -> dict[str, Any]:
        gap = self.pillar_gaps(row)
        fired = sorted({g for g in gap.values() if g})

        ceiling = row.get("published_claim_ceiling", "")
        candidates = [ceiling]
        candidates.extend(self.narrows_to[g] for g in fired if g in self.narrows_to)
        effective = min(candidates, key=lambda lv: self.rank.get(lv, 0))

        if self.rank.get(effective, 0) == 0:
            state = "quarantined"
        elif self.rank.get(effective, 0) < self.rank.get(ceiling, 0):
            state = "narrowed"
        else:
            state = "certified"

        posture = row.get("posture")
        blocks = posture in CLAIM_BEARING_LEVELS and self.rank.get(
            effective, 0
        ) < self.rank.get(posture, 0)
        blocker_reasons = fired if blocks else []
        return {
            "pillar_gaps": gap,
            "fired": fired,
            "effective": effective,
            "state": state,
            "blocks": blocks,
            "blocker_reasons": blocker_reasons,
        }


# --------------------------------------------------------------------------- #
# Matrix validation.
# --------------------------------------------------------------------------- #


def validate_matrix_schema(
    repo_root: Path, matrix: dict[str, Any], findings: list[Finding]
) -> None:
    schema = load_json(repo_root / MATRIX_SCHEMA_REL)
    validator = Draft202012Validator(schema)
    for error in sorted(validator.iter_errors(matrix), key=lambda e: list(e.path)):
        location = "/".join(str(p) for p in error.path) or "<root>"
        add_finding(
            findings,
            "matrix.schema",
            f"matrix fails its schema at {location}: {error.message}",
            "Bring the governance matrix back into conformance with its boundary schema.",
            ref=MATRIX_REL,
        )


def validate_source_refs(
    repo_root: Path, matrix: dict[str, Any], findings: list[Finding]
) -> None:
    if MATRIX_SCHEMA_REL not in matrix.get("source_refs", []):
        add_finding(
            findings,
            "matrix.source_refs.schema",
            "matrix source_refs must cite its own schema",
            f"Add {MATRIX_SCHEMA_REL} to source_refs.",
            ref=MATRIX_REL,
        )
    index_ref = matrix.get("evidence_index_ref")
    if isinstance(index_ref, str) and "/" in index_ref and not (repo_root / index_ref).exists():
        add_finding(
            findings,
            "matrix.evidence_index.missing",
            f"matrix cites a missing M5 evidence index: {index_ref}",
            "Publish the referenced evidence index or correct evidence_index_ref.",
            ref=index_ref,
        )
    for ref in matrix.get("source_refs", []):
        file_part = ref.split("#", 1)[0] if isinstance(ref, str) else ref
        if (
            isinstance(file_part, str)
            and "/" in file_part
            and not file_part.endswith("/")
            and not (repo_root / file_part).exists()
        ):
            add_finding(
                findings,
                "matrix.source_refs.missing",
                f"matrix cites a missing source artifact: {ref}",
                "Seed the referenced artifact or correct the source ref.",
                ref=ref,
            )


def validate_vocabulary(matrix: dict[str, Any], findings: list[Finding]) -> None:
    vocab = matrix.get("closed_vocabularies", {})
    for key, expected in CANONICAL_VOCAB.items():
        got = vocab.get(key)
        if got != expected:
            add_finding(
                findings,
                "matrix.vocabulary.drift",
                f"closed vocabulary {key} drifted from the canonical token set",
                "Re-emit the closed vocabulary from the shell efficiency runtime tokens.",
                ref=MATRIX_REL,
                details={"expected": expected, "found": got},
            )


def validate_reason_table(matrix: dict[str, Any], findings: list[Finding]) -> None:
    declared = {
        row.get("reason"): row.get("pillar")
        for row in matrix.get("narrowing_reasons", [])
    }
    for reason, pillar in REASON_PILLAR.items():
        if reason not in declared:
            add_finding(
                findings,
                "matrix.reason_table.missing_reason",
                f"narrowing_reasons omits a reason the engine can fire: {reason}",
                "Declare every fireable narrowing reason with its pillar and narrows_to.",
                ref=MATRIX_REL,
            )
        elif declared[reason] != pillar:
            add_finding(
                findings,
                "matrix.reason_table.pillar_mismatch",
                f"reason {reason} is declared under pillar {declared[reason]} but the engine owns it under {pillar}",
                "Keep each narrowing reason under the pillar that detects it.",
                ref=MATRIX_REL,
            )


def validate_consumer_bindings(matrix: dict[str, Any], findings: list[Finding]) -> None:
    consumers = {row.get("consumer", "") for row in matrix.get("consumer_bindings", [])}
    for consumer in sorted(REQUIRED_CONSUMERS):
        if consumer not in consumers:
            add_finding(
                findings,
                "matrix.consumer_bindings.missing",
                f"matrix omits the required consumer binding: {consumer}",
                "Bind release, support, docs, and help to the matrix so low-power copy derives from one source.",
                ref=MATRIX_REL,
            )


def validate_rows(
    engine: GovernanceEngine, matrix: dict[str, Any], findings: list[Finding]
) -> dict[str, Any]:
    blocking_row_ids: list[str] = []
    blocking_reasons: set[str] = set()
    for row in matrix.get("rows", []):
        rid = row.get("row_id", "<row>")
        result = engine.recompute(row)

        # Per-dimension stored findings must equal the recomputed pillar gaps.
        findings_obj = row.get("dimension_findings", {})
        for pillar in PILLARS:
            stored = findings_obj.get(pillar, {})
            want_reason = result["pillar_gaps"][pillar]
            stored_status = stored.get("certification_status")
            stored_reason = stored.get("narrowing_reason")
            want_status = "gap" if want_reason else "certified"
            if stored_status != want_status:
                add_finding(
                    findings,
                    "row.dimension_status_mismatch",
                    f"row {rid} dimension {pillar} stores status {stored_status} but recompute says {want_status}",
                    "A green dimension tile cannot mask a fired narrowing reason.",
                    ref=MATRIX_REL,
                )
            if stored_reason != want_reason:
                add_finding(
                    findings,
                    "row.dimension_reason_mismatch",
                    f"row {rid} dimension {pillar} stores reason {stored_reason} but recompute fires {want_reason}",
                    "Align the stored dimension reason with the engine.",
                    ref=MATRIX_REL,
                )

        if sorted(row.get("fired_narrowing_reasons", [])) != result["fired"]:
            add_finding(
                findings,
                "row.fired_reasons_mismatch",
                f"row {rid} stores fired reasons {sorted(row.get('fired_narrowing_reasons', []))} but recompute fires {result['fired']}",
                "Align the row's fired narrowing reasons with the engine.",
                ref=MATRIX_REL,
            )
        if row.get("effective_posture") != result["effective"]:
            add_finding(
                findings,
                "row.effective_posture_mismatch",
                f"row {rid} stores effective posture {row.get('effective_posture')} but recompute yields {result['effective']}",
                "Align the row's effective posture with the engine.",
                ref=MATRIX_REL,
            )
        if row.get("certification_state") != result["state"]:
            add_finding(
                findings,
                "row.state_mismatch",
                f"row {rid} stores state {row.get('certification_state')} but recompute yields {result['state']}",
                "Align the row's certification state with the engine.",
                ref=MATRIX_REL,
            )

        blocker = row.get("promotion_blocker", {})
        if bool(blocker.get("blocks_promotion")) != result["blocks"]:
            add_finding(
                findings,
                "row.blocks_mismatch",
                f"row {rid} stores blocks_promotion {blocker.get('blocks_promotion')} but recompute yields {result['blocks']}",
                "A claim-bearing row narrowed below its posture must hold promotion.",
                ref=MATRIX_REL,
            )
        if sorted(blocker.get("blocker_reasons", [])) != result["blocker_reasons"]:
            add_finding(
                findings,
                "row.blocker_reasons_mismatch",
                f"row {rid} stores blocker reasons {sorted(blocker.get('blocker_reasons', []))} but recompute yields {result['blocker_reasons']}",
                "Align the row's blocker reasons with the engine.",
                ref=MATRIX_REL,
            )

        release = row.get("release_binding", {})
        if release.get("declared_certification_state") != result["state"]:
            add_finding(
                findings,
                "row.release_binding_mismatch",
                f"row {rid} release binding declares state {release.get('declared_certification_state')} but recompute yields {result['state']}",
                "Keep the release binding aligned with the recomputed certification state.",
                ref=MATRIX_REL,
            )
        if release.get("declared_effective_posture") != result["effective"]:
            add_finding(
                findings,
                "row.release_binding_mismatch",
                f"row {rid} release binding declares posture {release.get('declared_effective_posture')} but recompute yields {result['effective']}",
                "Keep the release binding aligned with the recomputed effective posture.",
                ref=MATRIX_REL,
            )

        if result["blocks"]:
            blocking_row_ids.append(rid)
            blocking_reasons.update(result["blocker_reasons"])

    decision = "hold" if blocking_row_ids else "proceed"
    return {
        "decision": decision,
        "blocking_row_ids": sorted(blocking_row_ids),
        "blocking_reasons": sorted(blocking_reasons),
    }


def validate_promotion_gate(
    matrix: dict[str, Any], recomputed: dict[str, Any], findings: list[Finding]
) -> None:
    gate = matrix.get("promotion_gate", {})
    if gate.get("decision") != recomputed["decision"]:
        add_finding(
            findings,
            "promotion.decision_mismatch",
            f"promotion gate stores decision {gate.get('decision')} but recompute yields {recomputed['decision']}",
            "Recompute the promotion verdict from the row blockers.",
            ref=MATRIX_REL,
        )
    if sorted(gate.get("blocking_row_ids", [])) != recomputed["blocking_row_ids"]:
        add_finding(
            findings,
            "promotion.blocking_rows_mismatch",
            f"promotion gate stores blocking rows {sorted(gate.get('blocking_row_ids', []))} but recompute yields {recomputed['blocking_row_ids']}",
            "Align the promotion gate's blocking rows with the row recompute.",
            ref=MATRIX_REL,
        )
    if sorted(gate.get("blocking_reasons", [])) != recomputed["blocking_reasons"]:
        add_finding(
            findings,
            "promotion.blocking_reasons_mismatch",
            f"promotion gate stores blocking reasons {sorted(gate.get('blocking_reasons', []))} but recompute yields {recomputed['blocking_reasons']}",
            "Align the promotion gate's blocking reasons with the row recompute.",
            ref=MATRIX_REL,
        )


def validate_matrix(
    repo_root: Path, matrix: dict[str, Any], findings: list[Finding]
) -> str:
    validate_matrix_schema(repo_root, matrix, findings)
    validate_source_refs(repo_root, matrix, findings)
    validate_vocabulary(matrix, findings)
    validate_reason_table(matrix, findings)
    validate_consumer_bindings(matrix, findings)
    engine = GovernanceEngine(matrix)
    recomputed = validate_rows(engine, matrix, findings)
    validate_promotion_gate(matrix, recomputed, findings)
    return recomputed["decision"]


# --------------------------------------------------------------------------- #
# Fixture replay.
# --------------------------------------------------------------------------- #


def replay_fixtures(
    repo_root: Path, matrix: dict[str, Any], findings: list[Finding]
) -> int:
    register_path = repo_root / FIXTURE_REGISTER_REL
    if not register_path.exists():
        add_finding(
            findings,
            "fixtures.register_missing",
            f"fixture register not found at {FIXTURE_REGISTER_REL}",
            "Add a manifest listing the recompute fixtures.",
            ref=FIXTURE_REGISTER_REL,
        )
        return 0

    register = render_yaml_as_json(register_path)
    fixture_schema = load_json(repo_root / FIXTURE_SCHEMA_REL)
    validator = Draft202012Validator(fixture_schema)
    engine = GovernanceEngine(matrix)

    count = 0
    for rel in register.get("fixtures", []):
        fixture = load_json(repo_root / rel)
        count += 1

        if list(validator.iter_errors(fixture)):
            add_finding(
                findings,
                "fixture.schema",
                f"fixture {rel} fails the recompute-fixture schema",
                "Bring the fixture into conformance with the fixture-record schema.",
                ref=rel,
            )
            continue

        result = engine.recompute(fixture.get("row", {}))
        fid = fixture.get("fixture_id")

        expected_fired = sorted(fixture.get("expected_fired_reasons", []))
        if result["fired"] != expected_fired:
            add_finding(
                findings,
                "fixture.fired_reasons_mismatch",
                f"fixture {fid} expects {expected_fired} but recompute fires {result['fired']}",
                "Align the fixture row or its expectation.",
                ref=rel,
            )
        if result["effective"] != fixture.get("expected_effective_posture"):
            add_finding(
                findings,
                "fixture.effective_posture_mismatch",
                f"fixture {fid} expects posture {fixture.get('expected_effective_posture')} but recompute yields {result['effective']}",
                "Align the fixture expectation with the governance engine.",
                ref=rel,
            )
        if result["state"] != fixture.get("expected_certification_state"):
            add_finding(
                findings,
                "fixture.state_mismatch",
                f"fixture {fid} expects state {fixture.get('expected_certification_state')} but recompute yields {result['state']}",
                "Align the fixture expectation with the governance engine.",
                ref=rel,
            )
        if "expected_blocks_promotion" in fixture and result["blocks"] != fixture.get(
            "expected_blocks_promotion"
        ):
            add_finding(
                findings,
                "fixture.blocks_mismatch",
                f"fixture {fid} expects blocks_promotion {fixture.get('expected_blocks_promotion')} but recompute yields {result['blocks']}",
                "Align the fixture expectation with the promotion gate.",
                ref=rel,
            )
    return count


# --------------------------------------------------------------------------- #
# Negative drills.
# --------------------------------------------------------------------------- #


def run_negative_drills(
    repo_root: Path, matrix: dict[str, Any], findings: list[Finding]
) -> list[dict[str, str]]:
    results: list[dict[str, str]] = []

    def check_ids(candidate: dict[str, Any]) -> set[str]:
        local: list[Finding] = []
        validate_matrix(repo_root, candidate, local)
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

    def first_quarantined(candidate: dict[str, Any]) -> dict[str, Any] | None:
        return next(
            (r for r in candidate["rows"] if r.get("certification_state") == "quarantined"),
            None,
        )

    def first_claim_bearing(candidate: dict[str, Any]) -> dict[str, Any] | None:
        return next(
            (r for r in candidate["rows"] if r.get("posture") in CLAIM_BEARING_LEVELS),
            None,
        )

    # 1. Clearing a fired narrowing reason behind a quiet tile must be rejected.
    mutated = copy.deepcopy(matrix)
    target = first_quarantined(mutated)
    if target is not None:
        target["fired_narrowing_reasons"] = []
        record(
            "hidden_reason_rejected",
            "row.fired_reasons_mismatch",
            "row.fired_reasons_mismatch" in check_ids(mutated),
        )

    # 2. Overstating a row's effective posture must be rejected.
    mutated = copy.deepcopy(matrix)
    target = first_claim_bearing(mutated)
    if target is not None:
        target["effective_posture"] = "certified_low_power"
        target["published_claim_ceiling"] = "qualified_low_power"
        record(
            "overstated_posture_rejected",
            "row.effective_posture_mismatch",
            "row.effective_posture_mismatch" in check_ids(mutated),
        )

    # 3. A dimension that claims certified while its pillar gap fires must be rejected.
    mutated = copy.deepcopy(matrix)
    target = first_quarantined(mutated)
    if target is not None:
        target["dimension_findings"]["efficiency_state_evidence"] = {
            "certification_status": "certified",
            "narrowing_reason": None,
            "bound_refs": [],
        }
        record(
            "dimension_overstated_rejected",
            "row.dimension_status_mismatch",
            "row.dimension_status_mismatch" in check_ids(mutated),
        )

    # 4. A proceed verdict that hides a row narrowed below its posture must be rejected.
    mutated = copy.deepcopy(matrix)
    target = first_claim_bearing(mutated)
    if target is not None:
        target["evidence"]["protected_paths_preserved"] = False
        record(
            "blocked_row_hidden_by_proceed",
            "promotion.decision_mismatch",
            "promotion.decision_mismatch" in check_ids(mutated),
        )

    # 5. A release binding that publishes a fresher posture than the recompute must be rejected.
    mutated = copy.deepcopy(matrix)
    target = first_claim_bearing(mutated)
    if target is not None:
        target["release_binding"]["declared_effective_posture"] = "undeclared_badge"
        record(
            "release_binding_overstated_rejected",
            "row.release_binding_mismatch",
            "row.release_binding_mismatch" in check_ids(mutated),
        )

    return results


# --------------------------------------------------------------------------- #
# Entry point.
# --------------------------------------------------------------------------- #


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(f"--repo-root does not look like a repository root: {repo_root}")

    findings: list[Finding] = []
    matrix = load_json(repo_root / MATRIX_REL)
    if not isinstance(matrix, dict):
        raise SystemExit("governance matrix must be a JSON object")

    decision = validate_matrix(repo_root, matrix, findings)
    fixture_count = replay_fixtures(repo_root, matrix, findings)
    drill_results = run_negative_drills(repo_root, matrix, findings)

    errors = [f for f in findings if f.severity == "error"]
    warnings = [f for f in findings if f.severity == "warning"]
    status = "PASS" if not errors else "FAIL"
    print(
        f"[m5-efficiency-governance] {status} "
        f"({len(errors)} errors, {len(warnings)} warnings) -- "
        f"rows: {len(matrix.get('rows', []))}, "
        f"fixtures: {fixture_count}, drills: {len(drill_results)}, "
        f"promotion: {decision}, as_of: {matrix.get('as_of')}"
    )
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        suffix = f" [{finding.ref}]" if finding.ref else ""
        print(
            f"[m5-efficiency-governance] {prefix} {finding.check_id}: {finding.message}{suffix}"
        )
        print(f"[m5-efficiency-governance]   remediation: {finding.remediation}")

    if args.report:
        report = {
            "schema_version": 1,
            "check_id": "m5_efficiency_governance",
            "evaluated_on": matrix.get("as_of"),
            "status": "pass" if not errors else "fail",
            "matrix_ref": MATRIX_REL,
            "row_count": len(matrix.get("rows", [])),
            "fixture_count": fixture_count,
            "drill_count": len(drill_results),
            "promotion_decision": decision,
            "negative_drills": drill_results,
            "finding_counts": {"error": len(errors), "warning": len(warnings)},
            "findings": [f.as_report() for f in findings],
        }
        report_path = repo_root / args.report
        report_path.parent.mkdir(parents=True, exist_ok=True)
        report_path.write_text(
            json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8"
        )

    if errors:
        return 1
    if args.require_proceed and decision == "hold":
        print(
            "[m5-efficiency-governance] PUBLICATION HELD: governance promotion blocked",
            file=sys.stderr,
        )
        return 2
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[m5-efficiency-governance] interrupted", file=sys.stderr)
        sys.exit(130)
