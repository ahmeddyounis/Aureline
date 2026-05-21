#!/usr/bin/env python3
"""Validate the frozen beta correction-train / hotfix / backport packet.

This is the release-gating consumer for the checked-in correction packet:

  - artifacts/release/m3/correction_train/packet.json

Correction trains, emergency hotfixes, and supported-line backports share one
packet form. This gate mirrors the typed Rust consumer
(``aureline_release::correction_train::CorrectionTrainPacket``) so CI catches
drift without a cargo build. It enforces that every correction row keeps:

  - affected-build scope (affected release lines, plus packet-level
    exact-build identities);
  - rollback linkage (a named rollback target on every release-shipping lane);
  - lane policy (security and trust defects take the hotfix lane; polish never
    rides an emergency lane);
  - an explicit backport decision for every affected supported line; and
  - the shared correction vocabulary on the support projection.

After validating the canonical packet, the gate runs negative drills that
mutate an in-memory copy and confirm the validator rejects a packet missing
rollback linkage or affected-build scope.
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


DEFAULT_PACKET_REL = "artifacts/release/m3/correction_train/packet.json"

EXPECTED_RECORD_KIND = "correction_train_packet_record"
EXPECTED_SCHEMA_VERSION = 1

SHIPPING_LANES = {"hotfix", "backport", "correction_train_only"}
LANE_DECISIONS = {"hotfix", "backport", "correction_train_only", "next_cycle"}
BACKPORT_DECISIONS = {"yes", "no", "defer", "not_applicable"}
SUPPORTED_LINE_CLASSES = {"stable", "lts"}
SECURITY_OR_TRUST_ISSUE_CLASSES = {
    "security_policy_escape",
    "trust_boundary_or_permission_failure",
}
SHARED_PACKET_FORM_TERMS = {
    "correction_scope",
    "correction_risk",
    "correction_evidence",
    "target_channels",
    "triage_lane",
    "backport_decision",
    "rollback_target",
    "known_issue_update",
}


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
    parser.add_argument("--packet", default=DEFAULT_PACKET_REL)
    parser.add_argument(
        "--report",
        default=None,
        help="Optional path for a JSON validation capture.",
    )
    return parser.parse_args()


def load_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing JSON file: {path}")
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON at {path}: {exc}") from exc


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be a JSON object")
    return value


def as_list(value: Any) -> list[Any]:
    return value if isinstance(value, list) else []


def is_present(value: Any) -> bool:
    return isinstance(value, str) and bool(value.strip())


def validate_packet(packet: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    packet_id = str(packet.get("packet_id", "<packet>"))

    if packet.get("record_kind") != EXPECTED_RECORD_KIND:
        findings.append(
            Finding(
                "error",
                "packet.record_kind",
                "correction packet record_kind is not the correction-train packet kind",
                packet_id,
            )
        )
    if packet.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(
            Finding(
                "error",
                "packet.schema_version",
                "correction packet schema_version must be 1",
                packet_id,
            )
        )

    build_refs = as_list(packet.get("exact_build_identity_refs"))
    if not build_refs:
        findings.append(
            Finding(
                "error",
                "packet.exact_build_identity_refs_missing",
                "correction packet must name the affected exact-build identities",
                packet_id,
            )
        )
    for build_ref in build_refs:
        if not (isinstance(build_ref, str) and build_ref.startswith("build-id:aureline:")):
            findings.append(
                Finding(
                    "error",
                    "packet.exact_build_identity_ref_vocabulary",
                    "exact-build identity must use the Aureline build-id vocabulary",
                    str(build_ref),
                )
            )

    findings.extend(validate_templates(packet, packet_id))

    items = as_list(packet.get("correction_items"))
    if not items:
        findings.append(
            Finding(
                "error",
                "correction_items.empty",
                "correction packet must contain at least one correction row",
                packet_id,
            )
        )

    seen_ids: set[str] = set()
    for item in items:
        if not isinstance(item, dict):
            continue
        item_id = str(item.get("item_id", "<item>"))
        if item_id in seen_ids:
            findings.append(
                Finding(
                    "error",
                    "correction_items.duplicate_item_id",
                    "correction item ids must be unique",
                    item_id,
                )
            )
        seen_ids.add(item_id)
        findings.extend(validate_item(item, item_id))

    findings.extend(validate_support_projection(packet, packet_id))
    return findings


def validate_templates(packet: dict[str, Any], packet_id: str) -> list[Finding]:
    findings: list[Finding] = []
    templates = packet.get("packet_templates")
    if not isinstance(templates, dict):
        findings.append(
            Finding(
                "error",
                "packet_templates.missing",
                "correction packet must declare the correction/hotfix/backport templates",
                packet_id,
            )
        )
        return findings
    terms = set(as_list(templates.get("shared_packet_format_terms")))
    for required in sorted(SHARED_PACKET_FORM_TERMS - terms):
        findings.append(
            Finding(
                "error",
                "packet_templates.required_terms_missing",
                "packet templates must advertise the shared correction form vocabulary",
                required,
            )
        )
    for field in (
        "correction_train_template_ref",
        "hotfix_packet_template_ref",
        "backport_packet_template_ref",
    ):
        if not is_present(templates.get(field)):
            findings.append(
                Finding(
                    "error",
                    "packet_templates.ref_empty",
                    "every packet template ref must be a non-empty path",
                    field,
                )
            )
    return findings


def validate_item(item: dict[str, Any], item_id: str) -> list[Finding]:
    findings: list[Finding] = []
    scope = ensure_dict(item.get("scope", {}), f"{item_id}.scope")
    risk = ensure_dict(item.get("risk", {}), f"{item_id}.risk")
    evidence = ensure_dict(item.get("evidence", {}), f"{item_id}.evidence")
    triage = ensure_dict(item.get("triage", {}), f"{item_id}.triage")
    release_notes = ensure_dict(item.get("release_notes", {}), f"{item_id}.release_notes")
    issue_class = str(item.get("issue_class", ""))
    lane = str(triage.get("lane_decision", ""))

    if lane not in LANE_DECISIONS:
        findings.append(
            Finding(
                "error",
                "triage.lane_decision_unknown",
                "triage lane must use the controlled correction vocabulary",
                item_id,
            )
        )

    # Affected-build scope: every correction row names its affected release lines.
    affected_lines = as_list(scope.get("affected_release_lines"))
    if not affected_lines:
        findings.append(
            Finding(
                "error",
                "correction_scope.affected_build_scope_missing",
                "correction rows must name the affected release lines",
                item_id,
            )
        )

    # Rollback linkage: release-shipping lanes must name a rollback target.
    if lane in SHIPPING_LANES and not is_present(scope.get("rollback_target_ref")):
        findings.append(
            Finding(
                "error",
                "correction_scope.rollback_target_missing",
                "correction rows that ship on a release lane must name a rollback target",
                item_id,
            )
        )

    if issue_class in SECURITY_OR_TRUST_ISSUE_CLASSES and lane != "hotfix":
        findings.append(
            Finding(
                "error",
                "triage.security_or_trust_requires_hotfix",
                "security and trust-boundary defects on claimed surfaces require the hotfix lane",
                item_id,
            )
        )
    if issue_class == "data_loss_or_migration_breakage" and lane not in {"hotfix", "backport"}:
        findings.append(
            Finding(
                "error",
                "triage.data_or_migration_requires_hotfix_or_backport",
                "data-loss and migration defects must not be train-only or next-cycle while claimed",
                item_id,
            )
        )
    if issue_class == "protected_path_regression" and lane == "next_cycle":
        findings.append(
            Finding(
                "error",
                "triage.protected_path_not_next_cycle",
                "protected-path regressions must not be deferred as next-cycle work",
                item_id,
            )
        )
    if issue_class == "non_protected_polish" and lane in {"hotfix", "backport"}:
        findings.append(
            Finding(
                "error",
                "triage.polish_not_emergency",
                "non-protected polish must not ride hotfix or backport lanes",
                item_id,
            )
        )

    if lane == "hotfix" and not is_present(triage.get("hotfix_packet_ref")):
        findings.append(
            Finding(
                "error",
                "triage.hotfix_packet_ref_missing",
                "hotfix rows must point at the hotfix packet ref",
                item_id,
            )
        )

    if risk.get("claim_narrowing_required") is True and not as_list(
        release_notes.get("claim_update_refs")
    ):
        findings.append(
            Finding(
                "error",
                "triage.claim_narrowing_without_claim_update",
                "claim-narrowing corrections must name claim update refs",
                item_id,
            )
        )

    findings.extend(validate_backport_matrix(item, item_id, issue_class, affected_lines))
    findings.extend(validate_evidence(evidence, item_id, lane))
    return findings


def validate_backport_matrix(
    item: dict[str, Any],
    item_id: str,
    issue_class: str,
    affected_lines: list[Any],
) -> list[Finding]:
    findings: list[Finding] = []
    matrix = [row for row in as_list(item.get("backport_matrix")) if isinstance(row, dict)]
    matrix_lines = {row.get("release_line_ref") for row in matrix}
    for line in affected_lines:
        if line not in matrix_lines:
            findings.append(
                Finding(
                    "error",
                    "backport_matrix.affected_line_missing",
                    "every affected release line must appear in the backport matrix",
                    f"{item_id}:{line}",
                )
            )
    for row in matrix:
        line_ref = f"{item_id}:{row.get('release_line_ref')}"
        line_class = str(row.get("support_line_class", ""))
        decision = str(row.get("decision", ""))
        affected = row.get("affected") is True
        supported = line_class in SUPPORTED_LINE_CLASSES
        if decision not in BACKPORT_DECISIONS:
            findings.append(
                Finding(
                    "error",
                    "backport_matrix.decision_unknown",
                    "backport decision must use yes, no, defer, or not_applicable",
                    line_ref,
                )
            )
        if affected and supported and decision == "not_applicable":
            findings.append(
                Finding(
                    "error",
                    "backport_matrix.affected_line_no_decision",
                    "affected supported lines must record yes, no, or defer",
                    line_ref,
                )
            )
        if (
            affected
            and supported
            and issue_class in SECURITY_OR_TRUST_ISSUE_CLASSES
            and decision != "yes"
        ):
            findings.append(
                Finding(
                    "error",
                    "backport_matrix.security_supported_line_not_yes",
                    "security and trust hotfixes must backport to affected supported lines",
                    line_ref,
                )
            )
        if decision == "yes":
            if not is_present(row.get("target_release_ref")):
                findings.append(
                    Finding(
                        "error",
                        "backport_matrix.yes_missing_target_release",
                        "yes backport decisions must name a target release",
                        line_ref,
                    )
                )
            if not is_present(row.get("rollback_target_ref")):
                findings.append(
                    Finding(
                        "error",
                        "backport_matrix.yes_missing_rollback_target",
                        "yes backport decisions must name a rollback target",
                        line_ref,
                    )
                )
    return findings


def validate_evidence(evidence: dict[str, Any], item_id: str, lane: str) -> list[Finding]:
    findings: list[Finding] = []
    if lane in SHIPPING_LANES and evidence.get("freshness_state") not in {
        "current",
        "waived_current",
    }:
        findings.append(
            Finding(
                "error",
                "evidence.not_current_for_claimed_correction",
                "release-lane corrections require current or explicitly waived evidence",
                item_id,
            )
        )
    if lane in {"hotfix", "backport"} and not as_list(evidence.get("adjacent_sweep_refs")):
        findings.append(
            Finding(
                "error",
                "evidence.adjacent_sweep_missing",
                "hotfix and backport rows must include adjacent failure-domain sweep refs",
                item_id,
            )
        )
    return findings


def validate_support_projection(packet: dict[str, Any], packet_id: str) -> list[Finding]:
    findings: list[Finding] = []
    support = packet.get("support_projection")
    if not isinstance(support, dict):
        findings.append(
            Finding(
                "error",
                "support_projection.missing",
                "correction packet must declare a support projection",
                packet_id,
            )
        )
        return findings
    terms = set(as_list(support.get("vocabulary_terms")))
    if SHARED_PACKET_FORM_TERMS - terms:
        findings.append(
            Finding(
                "error",
                "support_projection.required_vocabulary_missing",
                "support projection is missing required correction vocabulary",
                packet_id,
            )
        )
    if not is_present(support.get("redaction_class")):
        findings.append(
            Finding(
                "error",
                "support_projection.redaction_class_empty",
                "support projection must declare a redaction class",
                packet_id,
            )
        )
    return findings


@dataclasses.dataclass
class Drill:
    drill_id: str
    expected_check_id: str
    mutate: Any


def first_index(items: list[Any], predicate: Any) -> int:
    for index, value in enumerate(items):
        if isinstance(value, dict) and predicate(value):
            return index
    return -1


def drop_rollback_linkage(packet: dict[str, Any]) -> bool:
    items = as_list(packet.get("correction_items"))
    index = first_index(
        items,
        lambda item: ensure_dict(item.get("triage", {}), "triage").get("lane_decision")
        in SHIPPING_LANES,
    )
    if index < 0:
        return False
    items[index]["scope"]["rollback_target_ref"] = None
    return True


def drop_affected_build_scope(packet: dict[str, Any]) -> bool:
    items = as_list(packet.get("correction_items"))
    if not items or not isinstance(items[0], dict):
        return False
    items[0]["scope"]["affected_release_lines"] = []
    return True


NEGATIVE_DRILLS = [
    Drill(
        "missing_rollback_linkage_rejected",
        "correction_scope.rollback_target_missing",
        drop_rollback_linkage,
    ),
    Drill(
        "missing_affected_build_scope_rejected",
        "correction_scope.affected_build_scope_missing",
        drop_affected_build_scope,
    ),
]


def run_negative_drills(packet: dict[str, Any]) -> tuple[list[dict[str, str]], list[Finding]]:
    results: list[dict[str, str]] = []
    findings: list[Finding] = []
    for drill in NEGATIVE_DRILLS:
        mutated = copy.deepcopy(packet)
        applied = drill.mutate(mutated)
        check_ids = {finding.check_id for finding in validate_packet(mutated)}
        passed = applied and drill.expected_check_id in check_ids
        results.append(
            {
                "drill_id": drill.drill_id,
                "expected_check_id": drill.expected_check_id,
                "status": "passed" if passed else "failed",
            }
        )
        if not passed:
            findings.append(
                Finding(
                    "error",
                    "negative_drill.not_rejected",
                    f"negative drill {drill.drill_id} did not raise {drill.expected_check_id}",
                    drill.drill_id,
                )
            )
    return results, findings


def write_report(
    path: Path,
    packet: dict[str, Any],
    findings: list[Finding],
    drill_results: list[dict[str, str]],
) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": 1,
        "record_kind": "correction_train_gate_validation_capture",
        "status": "pass" if not any(f.severity == "error" for f in findings) else "fail",
        "generated_at": dt.datetime.now(dt.UTC)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z"),
        "packet_id": packet.get("packet_id"),
        "correction_item_count": len(as_list(packet.get("correction_items"))),
        "negative_drills": drill_results,
        "findings": [f.as_report() for f in findings],
    }
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    packet = ensure_dict(load_json(repo_root / args.packet), "packet")

    findings = validate_packet(packet)
    drill_results, drill_findings = run_negative_drills(packet)
    findings.extend(drill_findings)

    if args.report:
        write_report(repo_root / args.report, packet, findings, drill_results)

    errors = [f for f in findings if f.severity == "error"]
    if errors:
        for item in errors:
            print(f"ERROR [{item.check_id}] ({item.ref}): {item.message}", file=sys.stderr)
        return 1

    print(
        "beta correction-train packet validated "
        f"({len(as_list(packet.get('correction_items')))} rows, "
        f"{len(drill_results)} negative drills)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
