#!/usr/bin/env python3
"""Validate beta correction-train, hotfix, and backport packets."""

from __future__ import annotations

import argparse
import copy
import dataclasses
import json
import sys
from pathlib import Path
from typing import Any

from tools.ci.m3._common import render_yaml_as_json


DEFAULT_PACKET_REL = "artifacts/release/m3/correction_train/packet.json"
DEFAULT_SCHEMA_REL = "schemas/release/correction_train_packet.schema.json"
DEFAULT_SUPPORT_PROJECTION_REL = (
    "artifacts/release/m3/correction_train/support_export_projection.json"
)
DEFAULT_CAPTURE_REL = (
    "artifacts/release/m3/captures/correction_train_validation_capture.json"
)
DEFAULT_ARTIFACT_GRAPH_REL = "artifacts/release/m3/artifact_graph.json"
DEFAULT_RING_ROLLOUT_REL = "artifacts/release/m3/ring_rollout/packet.json"
DEFAULT_UPDATE_ROLLBACK_REL = "artifacts/release/m3/update_rollback/rollback_plan.json"
DEFAULT_FIXTURE_MANIFEST_REL = "fixtures/release/correction_train_cases/manifest.yaml"

EXPECTED_SCHEMA_VERSION = 1
EXPECTED_PACKET_RECORD_KIND = "correction_train_packet_record"
EXPECTED_SUPPORT_RECORD_KIND = "correction_train_support_export"
EXPECTED_CAPTURE_RECORD_KIND = "correction_train_validation_capture"

LANE_DECISIONS = {"hotfix", "backport", "correction_train_only", "next_cycle"}
BACKPORT_DECISIONS = {"yes", "no", "defer", "not_applicable"}
SUPPORTED_LINE_CLASSES = {"stable", "lts"}
REQUIRED_TERMS = {
    "correction_scope",
    "correction_risk",
    "correction_evidence",
    "target_channels",
    "triage_lane",
    "backport_decision",
    "rollback_target",
    "known_issue_update",
}
SECURITY_OR_TRUST_CLASSES = {
    "security_policy_escape",
    "trust_boundary_or_permission_failure",
}
DATA_OR_INTERFACE_CLASSES = {
    "data_loss_or_migration_breakage",
    "sdk_interface_or_extension_regression",
}
OPAQUE_PREFIXES = (
    "artifact_node:",
    "build-id:",
    "channel:",
    "claim_row:",
    "compat_row:",
    "correction:",
    "correction_train:",
    "hotfix:",
    "m3_claim_row:",
    "profile:",
    "protected_path:",
    "release_candidate:",
    "release_line:",
    "release_train:",
    "support_bundle:",
)


@dataclasses.dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    ref: str
    remediation: str

    def as_report(self) -> dict[str, str]:
        return dataclasses.asdict(self)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--packet", default=DEFAULT_PACKET_REL)
    parser.add_argument("--schema", default=DEFAULT_SCHEMA_REL)
    parser.add_argument("--support-projection", default=DEFAULT_SUPPORT_PROJECTION_REL)
    parser.add_argument("--capture", default=DEFAULT_CAPTURE_REL)
    parser.add_argument("--artifact-graph", default=DEFAULT_ARTIFACT_GRAPH_REL)
    parser.add_argument("--ring-rollout", default=DEFAULT_RING_ROLLOUT_REL)
    parser.add_argument("--update-rollback", default=DEFAULT_UPDATE_ROLLBACK_REL)
    parser.add_argument("--fixture-manifest", default=DEFAULT_FIXTURE_MANIFEST_REL)
    parser.add_argument(
        "--check",
        action="store_true",
        help="Fail if generated support projection or validation capture would change.",
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


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a JSON array")
    return value


def as_json_text(payload: dict[str, Any]) -> str:
    return json.dumps(payload, indent=2, sort_keys=False) + "\n"


def strip_fragment(ref: str) -> str:
    return ref.split("#", 1)[0]


def is_repo_ref(ref: Any) -> bool:
    if not isinstance(ref, str) or not ref:
        return False
    if ref.startswith(OPAQUE_PREFIXES):
        return False
    if " " in ref and not ref.startswith(("./", "/", "artifacts/", "docs/", "fixtures/", "schemas/", "tools/", ".t2/")):
        return False
    return True


def repo_ref_exists(repo_root: Path, ref: str) -> bool:
    return (repo_root / strip_fragment(ref)).exists()


def validate_against_schema(
    payload: dict[str, Any],
    schema: dict[str, Any],
    payload_ref: str,
) -> list[Finding]:
    try:
        from jsonschema import Draft202012Validator, FormatChecker  # type: ignore
    except Exception:  # noqa: BLE001
        return []

    Draft202012Validator.check_schema(schema)
    validator = Draft202012Validator(schema, format_checker=FormatChecker())
    findings: list[Finding] = []
    for error in sorted(validator.iter_errors(payload), key=lambda err: list(err.path)):
        path = ".".join(str(part) for part in error.path) or "<root>"
        findings.append(
            Finding(
                "error",
                "correction_train.schema.validation",
                f"{path}: {error.message}",
                payload_ref,
                "Update the packet, support projection, or schema so the checked-in records validate.",
            )
        )
    return findings


def build_support_projection(packet: dict[str, Any], packet_rel: str) -> dict[str, Any]:
    lane_summary = {lane: 0 for lane in sorted(LANE_DECISIONS)}
    items: list[dict[str, Any]] = []
    for item in ensure_list(packet.get("correction_items", []), "packet.correction_items"):
        if not isinstance(item, dict):
            continue
        triage = ensure_dict(item.get("triage", {}), "item.triage")
        scope = ensure_dict(item.get("scope", {}), "item.scope")
        risk = ensure_dict(item.get("risk", {}), "item.risk")
        evidence = ensure_dict(item.get("evidence", {}), "item.evidence")
        lane = str(triage.get("lane_decision", ""))
        if lane in lane_summary:
            lane_summary[lane] += 1
        backport_decisions = []
        for row in ensure_list(item.get("backport_matrix", []), "item.backport_matrix"):
            if not isinstance(row, dict):
                continue
            backport_decisions.append(
                {
                    "release_line_ref": row.get("release_line_ref"),
                    "support_line_class": row.get("support_line_class"),
                    "channel_class": row.get("channel_class"),
                    "decision": row.get("decision"),
                    "target_release_ref": row.get("target_release_ref"),
                    "rollback_target_ref": row.get("rollback_target_ref"),
                    "known_issue_ref": row.get("known_issue_ref"),
                    "support_note_ref": row.get("support_note_ref"),
                }
            )
        items.append(
            {
                "item_id": item.get("item_id"),
                "title": item.get("title"),
                "issue_class": item.get("issue_class"),
                "severity_class": item.get("severity_class"),
                "lane_decision": lane,
                "decision_state": triage.get("decision_state"),
                "risk_level": risk.get("risk_level"),
                "target_channel_refs": scope.get("target_channel_refs", []),
                "rollback_target_ref": scope.get("rollback_target_ref"),
                "evidence_refs": evidence.get("evidence_refs", []),
                "backport_decisions": backport_decisions,
            }
        )
    support = ensure_dict(packet.get("support_projection", {}), "packet.support_projection")
    return {
        "record_kind": EXPECTED_SUPPORT_RECORD_KIND,
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "packet_id": packet.get("packet_id"),
        "generated_at": packet.get("generated_at"),
        "source_packet_ref": packet_rel,
        "train_ref": packet.get("train_ref"),
        "release_candidate_ref": packet.get("release_candidate_ref"),
        "exact_build_identity_refs": packet.get("exact_build_identity_refs", []),
        "lane_summary": lane_summary,
        "correction_items": items,
        "support_bundle_refs": support.get("support_bundle_refs", []),
        "vocabulary_terms": support.get("vocabulary_terms", []),
        "redaction_class": support.get("redaction_class"),
    }


class Validator:
    def __init__(
        self,
        repo_root: Path,
        packet: dict[str, Any],
        support_projection: dict[str, Any],
        artifact_graph: dict[str, Any],
        ring_rollout: dict[str, Any],
        update_rollback: dict[str, Any],
        packet_rel: str,
        support_projection_rel: str,
        artifact_graph_rel: str,
        ring_rollout_rel: str,
        update_rollback_rel: str,
    ) -> None:
        self.repo_root = repo_root
        self.packet = packet
        self.support_projection = support_projection
        self.artifact_graph = artifact_graph
        self.ring_rollout = ring_rollout
        self.update_rollback = update_rollback
        self.packet_rel = packet_rel
        self.support_projection_rel = support_projection_rel
        self.artifact_graph_rel = artifact_graph_rel
        self.ring_rollout_rel = ring_rollout_rel
        self.update_rollback_rel = update_rollback_rel
        self.findings: list[Finding] = []

    def push(
        self,
        check_id: str,
        message: str,
        ref: str,
        remediation: str,
        severity: str = "error",
    ) -> None:
        self.findings.append(Finding(severity, check_id, message, ref, remediation))

    def validate(self, *, include_surfaces: bool = True) -> list[Finding]:
        self.validate_header()
        self.validate_source_refs()
        self.validate_templates()
        self.validate_correction_items()
        self.validate_support_contract()
        if include_surfaces:
            self.validate_consuming_surfaces()
        return self.findings

    def validate_header(self) -> None:
        if self.packet.get("record_kind") != EXPECTED_PACKET_RECORD_KIND:
            self.push(
                "correction_train.record_kind",
                f"record_kind must be {EXPECTED_PACKET_RECORD_KIND}",
                str(self.packet.get("packet_id", "<packet>")),
                "Use the correction train packet discriminator.",
            )
        if self.packet.get("schema_version") != EXPECTED_SCHEMA_VERSION:
            self.push(
                "correction_train.schema_version",
                "schema_version must be 1",
                str(self.packet.get("packet_id", "<packet>")),
                "Keep the packet on schema version 1 until a governed migration exists.",
            )
        candidate = ensure_dict(self.artifact_graph.get("candidate", {}), "artifact_graph.candidate")
        if self.packet.get("release_candidate_ref") != candidate.get("candidate_ref"):
            self.push(
                "correction_train.release_candidate_mismatch",
                "correction packet must target the artifact graph release candidate",
                str(self.packet.get("release_candidate_ref")),
                "Refresh release_candidate_ref from the artifact graph candidate.",
            )
        if self.packet.get("release_candidate_ref") != self.ring_rollout.get("release_candidate_ref"):
            self.push(
                "correction_train.release_candidate_not_in_ring_rollout",
                "correction packet must target the ring rollout release candidate",
                str(self.packet.get("release_candidate_ref")),
                "Keep correction train and rollout packet on the same candidate.",
            )
        exact_refs = set(ensure_list(self.packet.get("exact_build_identity_refs", []), "packet.exact_build_identity_refs"))
        graph_exact_refs = {
            row.get("exact_build_identity_ref")
            for row in ensure_list(
                self.artifact_graph.get("exact_build_identities", []),
                "artifact_graph.exact_build_identities",
            )
            if isinstance(row, dict)
        }
        rollout_exact_refs = set(self.ring_rollout.get("exact_build_identity_refs", []))
        for ref in exact_refs:
            if ref not in graph_exact_refs:
                self.push(
                    "correction_train.exact_build.not_in_artifact_graph",
                    "exact-build ref is absent from the artifact graph",
                    str(ref),
                    "Add the exact-build identity to the artifact graph or correct the packet.",
                )
            if ref not in rollout_exact_refs:
                self.push(
                    "correction_train.exact_build.not_in_ring_rollout",
                    "exact-build ref is absent from ring rollout",
                    str(ref),
                    "Refresh ring rollout exact-build refs or correct the correction packet.",
                )

    def validate_source_refs(self) -> None:
        refs = ensure_dict(self.packet.get("source_refs", {}), "packet.source_refs")
        expected_paths = {
            "artifact_graph_ref": self.artifact_graph_rel,
            "ring_rollout_ref": self.ring_rollout_rel,
            "update_rollback_ref": self.update_rollback_rel,
            "correction_train_doc_ref": "docs/release/m3/correction_train_beta.md",
        }
        for field, expected in expected_paths.items():
            if refs.get(field) != expected:
                self.push(
                    "correction_train.source_ref.mismatch",
                    f"{field} must match the validated beta artifact",
                    f"source_refs.{field}",
                    f"Set {field} to {expected}.",
                )
        for field, ref in refs.items():
            if not isinstance(ref, str) or not ref:
                self.push(
                    "correction_train.source_ref.empty",
                    "source refs must be non-empty strings",
                    f"source_refs.{field}",
                    "Populate the source ref with a repo-relative path.",
                )
                continue
            if is_repo_ref(ref) and not repo_ref_exists(self.repo_root, ref):
                self.push(
                    "correction_train.source_ref.missing_on_disk",
                    "repo-relative source ref does not exist",
                    ref,
                    "Add the source artifact or correct the correction packet ref.",
                )

    def validate_templates(self) -> None:
        templates = ensure_dict(self.packet.get("packet_templates", {}), "packet.packet_templates")
        terms = set(templates.get("shared_packet_format_terms", []))
        missing = REQUIRED_TERMS - terms
        if missing:
            self.push(
                "packet_templates.required_terms_missing",
                "packet templates must advertise the shared correction form",
                str(self.packet.get("packet_id", "<packet>")),
                f"Add shared terms: {', '.join(sorted(missing))}.",
            )
        for field in (
            "correction_train_template_ref",
            "hotfix_packet_template_ref",
            "backport_packet_template_ref",
        ):
            ref = templates.get(field)
            if not isinstance(ref, str) or not ref:
                self.push(
                    "packet_templates.ref_empty",
                    "template refs must be non-empty strings",
                    field,
                    "Populate every packet template ref.",
                )
                continue
            path = self.repo_root / strip_fragment(ref)
            if not path.exists():
                self.push(
                    "packet_templates.ref_missing_on_disk",
                    "declared packet template does not exist",
                    ref,
                    "Add the packet template or correct the ref.",
                )
                continue
            text = path.read_text(encoding="utf-8")
            for term in REQUIRED_TERMS:
                if term not in text:
                    self.push(
                        "packet_templates.term_missing_from_template",
                        "template does not quote the shared correction vocabulary",
                        ref,
                        f"Add {term} to the template.",
                    )

    def validate_correction_items(self) -> None:
        items = ensure_list(self.packet.get("correction_items", []), "packet.correction_items")
        if not items:
            self.push(
                "correction_items.empty",
                "correction packet must contain at least one row",
                str(self.packet.get("packet_id", "<packet>")),
                "Add correction rows for the train.",
            )
            return

        seen_ids: set[str] = set()
        observed_lanes: set[str] = set()
        for item in items:
            if not isinstance(item, dict):
                self.push(
                    "correction_items.row_shape",
                    "correction rows must be objects",
                    str(item),
                    "Replace the row with a correction item object.",
                )
                continue
            item_id = str(item.get("item_id", ""))
            if item_id in seen_ids:
                self.push(
                    "correction_items.duplicate_item_id",
                    "correction item ids must be unique",
                    item_id,
                    "Give each correction row one stable item_id.",
                )
            seen_ids.add(item_id)
            triage = ensure_dict(item.get("triage", {}), f"{item_id}.triage")
            lane = str(triage.get("lane_decision", ""))
            observed_lanes.add(lane)
            self.validate_lane_policy(item, item_id, lane)
            self.validate_backport_matrix(item, item_id, lane)
            self.validate_target_channels(item, item_id, lane)
            self.validate_evidence(item, item_id, lane)

        missing_lanes = LANE_DECISIONS - observed_lanes
        if missing_lanes:
            self.push(
                "triage.lane_coverage_missing",
                "canonical packet must exercise every triage lane",
                str(self.packet.get("packet_id", "<packet>")),
                f"Add rows for lanes: {', '.join(sorted(missing_lanes))}.",
            )

    def validate_lane_policy(self, item: dict[str, Any], item_id: str, lane: str) -> None:
        issue_class = str(item.get("issue_class", ""))
        scope = ensure_dict(item.get("scope", {}), f"{item_id}.scope")
        risk = ensure_dict(item.get("risk", {}), f"{item_id}.risk")
        if lane not in LANE_DECISIONS:
            self.push(
                "triage.lane_decision_unknown",
                "triage lane must use the controlled correction vocabulary",
                item_id,
                "Use hotfix, backport, correction_train_only, or next_cycle.",
            )
        if issue_class in SECURITY_OR_TRUST_CLASSES and lane != "hotfix":
            self.push(
                "triage.security_or_trust_requires_hotfix",
                "security and trust-boundary defects on claimed surfaces require hotfix triage",
                item_id,
                "Use the hotfix lane or narrow the affected claim before publishing the packet.",
            )
        if issue_class == "data_loss_or_migration_breakage" and lane not in {"hotfix", "backport"}:
            self.push(
                "triage.data_or_migration_requires_hotfix_or_backport",
                "data-loss, rollback, and migration defects must not be train-only or next-cycle while claimed",
                item_id,
                "Use hotfix/backport or withdraw the affected claim.",
            )
        if issue_class == "protected_path_regression" and lane == "next_cycle":
            self.push(
                "triage.protected_path_not_next_cycle",
                "protected-path regressions must not be deferred as next-cycle work",
                item_id,
                "Use correction_train_only, backport, or hotfix based on support exposure.",
            )
        if issue_class == "non_protected_polish" and lane in {"hotfix", "backport"}:
            self.push(
                "triage.polish_not_emergency",
                "non-protected polish must not ride hotfix or backport lanes",
                item_id,
                "Use next_cycle or a planned correction train lane.",
            )
        if lane == "hotfix" and not triage_hotfix_ref_present(item):
            self.push(
                "triage.hotfix_packet_ref_missing",
                "hotfix rows must point at the hotfix packet/template ref",
                item_id,
                "Populate triage.hotfix_packet_ref.",
            )
        if risk.get("claim_narrowing_required") is True and not item.get("release_notes", {}).get("claim_update_refs"):
            self.push(
                "triage.claim_narrowing_without_claim_update",
                "claim-narrowing corrections must name claim update refs",
                item_id,
                "Add claim_update_refs to release_notes.",
            )
        if lane in {"hotfix", "backport", "correction_train_only"} and not scope.get("rollback_target_ref"):
            self.push(
                "triage.rollback_target_missing",
                "correction rows that ship on a release lane must name a rollback target",
                item_id,
                "Populate scope.rollback_target_ref.",
            )

    def validate_backport_matrix(self, item: dict[str, Any], item_id: str, lane: str) -> None:
        scope = ensure_dict(item.get("scope", {}), f"{item_id}.scope")
        issue_class = str(item.get("issue_class", ""))
        matrix = [
            row
            for row in ensure_list(item.get("backport_matrix", []), f"{item_id}.backport_matrix")
            if isinstance(row, dict)
        ]
        matrix_lines = {row.get("release_line_ref") for row in matrix}
        for release_line in ensure_list(scope.get("affected_release_lines", []), f"{item_id}.scope.affected_release_lines"):
            if release_line not in matrix_lines:
                self.push(
                    "backport_matrix.affected_line_missing",
                    "every affected release line must appear in the backport matrix",
                    f"{item_id}:{release_line}",
                    "Add an explicit matrix row with yes/no/defer/not_applicable.",
                )
        for row in matrix:
            line_ref = str(row.get("release_line_ref", ""))
            line_class = str(row.get("support_line_class", ""))
            decision = str(row.get("decision", ""))
            affected = row.get("affected") is True
            if decision not in BACKPORT_DECISIONS:
                self.push(
                    "backport_matrix.decision_unknown",
                    "backport decision must use yes, no, defer, or not_applicable",
                    f"{item_id}:{line_ref}",
                    "Use the controlled backport decision vocabulary.",
                )
            if affected and line_class in SUPPORTED_LINE_CLASSES and decision == "not_applicable":
                self.push(
                    "backport_matrix.affected_line_no_decision",
                    "affected supported lines must record yes, no, or defer",
                    f"{item_id}:{line_ref}",
                    "Replace not_applicable with an explicit supported-line backport decision.",
                )
            if (
                affected
                and issue_class in SECURITY_OR_TRUST_CLASSES
                and line_class in SUPPORTED_LINE_CLASSES
                and decision != "yes"
            ):
                self.push(
                    "backport_matrix.security_supported_line_not_yes",
                    "security and trust hotfixes must backport to affected supported lines",
                    f"{item_id}:{line_ref}",
                    "Use yes or narrow the supported-line claim before publishing.",
                )
            if decision == "yes":
                if not row.get("target_release_ref"):
                    self.push(
                        "backport_matrix.yes_missing_target_release",
                        "yes backport decisions must name a target release",
                        f"{item_id}:{line_ref}",
                        "Populate target_release_ref.",
                    )
                if not row.get("rollback_target_ref"):
                    self.push(
                        "backport_matrix.yes_missing_rollback_target",
                        "yes backport decisions must name a rollback target",
                        f"{item_id}:{line_ref}",
                        "Populate rollback_target_ref.",
                    )
            if lane == "backport" and line_class in SUPPORTED_LINE_CLASSES and affected and decision not in {"yes", "no", "defer"}:
                self.push(
                    "backport_matrix.backport_lane_missing_supported_decision",
                    "backport lane rows require supported-line yes/no/defer decisions",
                    f"{item_id}:{line_ref}",
                    "Record the supported-line decision explicitly.",
                )

    def validate_target_channels(self, item: dict[str, Any], item_id: str, lane: str) -> None:
        updates = [
            row
            for row in ensure_list(item.get("target_channel_updates", []), f"{item_id}.target_channel_updates")
            if isinstance(row, dict)
        ]
        dispositions = {row.get("disposition") for row in updates}
        if lane == "hotfix" and "ship_hotfix" not in dispositions:
            self.push(
                "target_channels.hotfix_disposition_missing",
                "hotfix rows must include a ship_hotfix target-channel disposition",
                item_id,
                "Add a target_channel_updates row with disposition ship_hotfix.",
            )
        if lane == "backport" and "ship_backport" not in dispositions:
            self.push(
                "target_channels.backport_disposition_missing",
                "backport rows must include a ship_backport target-channel disposition",
                item_id,
                "Add a target_channel_updates row with disposition ship_backport.",
            )
        if lane == "next_cycle" and dispositions & {"ship_hotfix", "ship_backport"}:
            self.push(
                "target_channels.next_cycle_emergency_disposition",
                "next-cycle rows must not target hotfix or backport channels",
                item_id,
                "Move the row to hotfix/backport or remove emergency dispositions.",
            )
        for row in updates:
            if row.get("disposition") in {"ship_hotfix", "ship_backport", "ship_train"}:
                for field in ("rollback_target_ref", "known_issue_ref", "docs_update_ref", "support_note_ref"):
                    if not row.get(field):
                        self.push(
                            "target_channels.release_lane_truth_ref_missing",
                            f"{field} is required for release-lane target updates",
                            f"{item_id}:{row.get('channel_ref')}",
                            f"Populate {field}.",
                        )

    def validate_evidence(self, item: dict[str, Any], item_id: str, lane: str) -> None:
        evidence = ensure_dict(item.get("evidence", {}), f"{item_id}.evidence")
        if lane in {"hotfix", "backport", "correction_train_only"} and evidence.get("freshness_state") not in {"current", "waived_current"}:
            self.push(
                "evidence.not_current_for_claimed_correction",
                "release-lane corrections require current or explicitly waived evidence",
                item_id,
                "Refresh rerun evidence or mark the claim narrowed until proof is current.",
            )
        if lane in {"hotfix", "backport"} and not evidence.get("adjacent_sweep_refs"):
            self.push(
                "evidence.adjacent_sweep_missing",
                "hotfix and backport rows must include adjacent failure-domain sweep refs",
                item_id,
                "Add adjacent_sweep_refs before closure.",
            )

    def validate_support_contract(self) -> None:
        support = ensure_dict(self.packet.get("support_projection", {}), "packet.support_projection")
        if support.get("support_projection_ref") != self.support_projection_rel:
            self.push(
                "support_projection.output_ref_mismatch",
                "packet support projection ref must match the generated projection path",
                str(support.get("support_projection_ref")),
                f"Set support_projection_ref to {self.support_projection_rel}.",
            )
        missing = REQUIRED_TERMS - set(support.get("vocabulary_terms", []))
        if missing:
            self.push(
                "support_projection.required_vocabulary_missing",
                "support projection is missing required correction vocabulary",
                str(self.packet.get("packet_id", "<packet>")),
                f"Add vocabulary terms: {', '.join(sorted(missing))}.",
            )
        expected = build_support_projection(self.packet, self.packet_rel)
        if self.support_projection != expected:
            self.push(
                "support_projection.generated_payload_stale",
                "checked-in support projection does not match the correction packet",
                self.support_projection_rel,
                "Run the correction train validator without --check and commit the generated projection.",
            )

    def validate_consuming_surfaces(self) -> None:
        support = ensure_dict(self.packet.get("support_projection", {}), "packet.support_projection")
        required_tokens = [
            str(self.packet.get("packet_id", "")),
            *sorted(REQUIRED_TERMS),
            *sorted(LANE_DECISIONS),
        ]
        for surface_ref in ensure_list(
            support.get("consuming_surface_refs", []),
            "support_projection.consuming_surface_refs",
        ):
            if not isinstance(surface_ref, str) or not surface_ref:
                self.push(
                    "support_surface.ref_empty",
                    "support surface refs must be non-empty strings",
                    str(surface_ref),
                    "Populate consuming_surface_refs with repo-relative paths.",
                )
                continue
            path = self.repo_root / strip_fragment(surface_ref)
            if not path.exists():
                self.push(
                    "support_surface.missing_on_disk",
                    "declared consuming surface does not exist",
                    surface_ref,
                    "Add the docs/help/support artifact or remove the consuming surface ref.",
                )
                continue
            text = path.read_text(encoding="utf-8")
            for token in required_tokens:
                if token and token not in text:
                    self.push(
                        "support_surface.correction_truth_token_missing",
                        "consuming surface does not quote shared correction vocabulary or lane states",
                        surface_ref,
                        f"Quote {token} from the correction packet or generated support projection.",
                    )


def triage_hotfix_ref_present(item: dict[str, Any]) -> bool:
    triage = ensure_dict(item.get("triage", {}), "item.triage")
    ref = triage.get("hotfix_packet_ref")
    return isinstance(ref, str) and bool(ref.strip())


def set_payload_path(payload: Any, path: str, value: Any) -> bool:
    segments = [segment.strip() for segment in path.split(".") if segment.strip()]
    if not segments:
        return False
    cursor = payload
    for segment in segments[:-1]:
        if isinstance(cursor, list):
            try:
                index = int(segment)
            except ValueError:
                return False
            if index < 0 or index >= len(cursor):
                return False
            cursor = cursor[index]
        elif isinstance(cursor, dict) and segment in cursor:
            cursor = cursor[segment]
        else:
            return False
    last = segments[-1]
    if isinstance(cursor, list):
        try:
            index = int(last)
        except ValueError:
            return False
        if index < 0 or index >= len(cursor):
            return False
        cursor[index] = value
        return True
    if isinstance(cursor, dict) and last in cursor:
        cursor[last] = value
        return True
    return False


def validate_fixture_manifest(
    *,
    repo_root: Path,
    manifest_rel: str,
    packet: dict[str, Any],
    artifact_graph: dict[str, Any],
    ring_rollout: dict[str, Any],
    update_rollback: dict[str, Any],
    packet_rel: str,
    support_projection_rel: str,
    schema_rel: str,
    artifact_graph_rel: str,
    ring_rollout_rel: str,
    update_rollback_rel: str,
) -> tuple[list[dict[str, Any]], list[Finding]]:
    manifest = ensure_dict(render_yaml_as_json(repo_root / manifest_rel), "fixture_manifest")
    findings: list[Finding] = []
    results: list[dict[str, Any]] = []
    if manifest.get("record_kind") != "correction_train_fixture_manifest":
        findings.append(
            Finding(
                "error",
                "correction_train.fixture_manifest.record_kind",
                "fixture manifest must use correction_train_fixture_manifest",
                str(manifest.get("record_kind", "<fixture-manifest>")),
                "Use the correction train fixture manifest discriminator.",
            )
        )
    expected_refs = {
        "canonical_packet_ref": packet_rel,
        "canonical_schema_ref": schema_rel,
        "canonical_support_projection_ref": support_projection_rel,
        "validator_ref": "tools/ci/m3/correction_train",
    }
    for field, expected in expected_refs.items():
        ref = manifest.get(field)
        if ref != expected:
            findings.append(
                Finding(
                    "error",
                    "correction_train.fixture_manifest.ref_mismatch",
                    f"{field} must match the canonical correction artifact",
                    str(ref),
                    f"Set {field} to {expected}.",
                )
            )
        if isinstance(ref, str) and is_repo_ref(ref) and not repo_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    "error",
                    "correction_train.fixture_manifest.ref_missing_on_disk",
                    "fixture manifest ref does not exist",
                    ref,
                    "Add the referenced file or correct the fixture manifest.",
                )
            )
    mutations = ensure_list(
        manifest.get("failure_drill_mutations", []),
        "fixture_manifest.failure_drill_mutations",
    )
    if not mutations:
        findings.append(
            Finding(
                "error",
                "correction_train.fixture_manifest.mutations_missing",
                "fixture manifest must name failure-drill mutations",
                manifest_rel,
                "Add mutation rows for expected correction rejection classes.",
            )
        )
    for mutation in mutations:
        mutation = ensure_dict(mutation, "fixture_manifest.failure_drill_mutations[]")
        mutation_id = str(mutation.get("mutation_id", ""))
        payload_path = str(mutation.get("payload_path", ""))
        expected_finding = str(mutation.get("expected_finding", ""))
        mutated_packet = copy.deepcopy(packet)
        set_ok = set_payload_path(mutated_packet, payload_path, mutation.get("replacement"))
        case_findings: list[str] = []
        if not set_ok:
            case_findings.append("payload_path_unresolved")
        else:
            validator = Validator(
                repo_root,
                mutated_packet,
                build_support_projection(mutated_packet, packet_rel),
                artifact_graph,
                ring_rollout,
                update_rollback,
                packet_rel,
                support_projection_rel,
                artifact_graph_rel,
                ring_rollout_rel,
                update_rollback_rel,
            )
            mutation_findings = validator.validate(include_surfaces=False)
            check_ids = {finding.check_id for finding in mutation_findings}
            if expected_finding not in check_ids:
                case_findings.append(f"expected_finding_missing:{expected_finding}")
        status = "passed" if not case_findings else "failed"
        results.append(
            {
                "mutation_id": mutation_id,
                "target_ref": mutation.get("target_ref"),
                "expected_finding": expected_finding,
                "status": status,
                "findings": case_findings,
            }
        )
        for item in case_findings:
            findings.append(
                Finding(
                    "error",
                    "correction_train.fixture_manifest.mutation_failed",
                    f"correction fixture mutation failed: {item}",
                    mutation_id,
                    "Update the mutation path, expected finding, or validator rule.",
                )
            )
    return results, findings


def compare_or_write(
    path: Path,
    expected_text: str,
    check: bool,
    findings: list[Finding],
    check_id: str,
) -> None:
    if check:
        actual = path.read_text(encoding="utf-8") if path.exists() else ""
        if actual != expected_text:
            findings.append(
                Finding(
                    "error",
                    check_id,
                    "checked-in generated file is stale",
                    str(path),
                    "Run the correction train validator without --check and commit the generated output.",
                )
            )
        return
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(expected_text, encoding="utf-8")


def build_capture(
    *,
    packet: dict[str, Any],
    findings: list[Finding],
    fixture_results: list[dict[str, Any]],
    packet_rel: str,
    schema_rel: str,
    support_projection_rel: str,
) -> dict[str, Any]:
    items = [
        item for item in packet.get("correction_items", []) if isinstance(item, dict)
    ]
    lane_counts = {lane: 0 for lane in sorted(LANE_DECISIONS)}
    backport_counts = {decision: 0 for decision in sorted(BACKPORT_DECISIONS)}
    affected_supported_decisions = 0
    for item in items:
        triage = item.get("triage", {})
        if isinstance(triage, dict) and triage.get("lane_decision") in lane_counts:
            lane_counts[triage["lane_decision"]] += 1
        for row in item.get("backport_matrix", []):
            if not isinstance(row, dict):
                continue
            decision = row.get("decision")
            if decision in backport_counts:
                backport_counts[decision] += 1
            if row.get("affected") is True and row.get("support_line_class") in SUPPORTED_LINE_CLASSES:
                affected_supported_decisions += 1
    error_count = sum(1 for finding in findings if finding.severity == "error")
    return {
        "record_kind": EXPECTED_CAPTURE_RECORD_KIND,
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "validated_at": packet.get("generated_at"),
        "packet_id": packet.get("packet_id"),
        "source_packet_ref": packet_rel,
        "schema_ref": schema_rel,
        "support_projection_ref": support_projection_rel,
        "passed": error_count == 0,
        "coverage": {
            "train_ref": packet.get("train_ref"),
            "release_candidate_ref": packet.get("release_candidate_ref"),
            "exact_build_identity_refs": packet.get("exact_build_identity_refs", []),
            "correction_item_count": len(items),
            "lane_counts": lane_counts,
            "backport_decision_counts": backport_counts,
            "affected_supported_decision_count": affected_supported_decisions,
            "fixture_mutation_count": len(fixture_results),
            "error_count": error_count,
        },
        "fixture_results": fixture_results,
        "findings": [finding.as_report() for finding in findings],
    }


def load_support_projection_or_expected(
    repo_root: Path,
    support_rel: str,
    expected: dict[str, Any],
    check: bool,
) -> dict[str, Any]:
    path = repo_root / support_rel
    if path.exists():
        return ensure_dict(load_json(path), "support_projection")
    if check:
        return {}
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(as_json_text(expected), encoding="utf-8")
    return expected


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    packet = ensure_dict(load_json(repo_root / args.packet), "packet")
    schema = ensure_dict(load_json(repo_root / args.schema), "schema")
    artifact_graph = ensure_dict(load_json(repo_root / args.artifact_graph), "artifact_graph")
    ring_rollout = ensure_dict(load_json(repo_root / args.ring_rollout), "ring_rollout")
    update_rollback = ensure_dict(load_json(repo_root / args.update_rollback), "update_rollback")
    expected_support = build_support_projection(packet, args.packet)
    support_projection = load_support_projection_or_expected(
        repo_root,
        args.support_projection,
        expected_support,
        args.check,
    )

    findings: list[Finding] = []
    findings.extend(validate_against_schema(packet, schema, args.packet))
    if support_projection:
        findings.extend(
            validate_against_schema(support_projection, schema, args.support_projection)
        )

    validator = Validator(
        repo_root,
        packet,
        support_projection,
        artifact_graph,
        ring_rollout,
        update_rollback,
        args.packet,
        args.support_projection,
        args.artifact_graph,
        args.ring_rollout,
        args.update_rollback,
    )
    findings.extend(validator.validate())

    compare_or_write(
        repo_root / args.support_projection,
        as_json_text(expected_support),
        args.check,
        findings,
        "correction_train.generated.support_projection_stale",
    )

    fixture_results, fixture_findings = validate_fixture_manifest(
        repo_root=repo_root,
        manifest_rel=args.fixture_manifest,
        packet=packet,
        artifact_graph=artifact_graph,
        ring_rollout=ring_rollout,
        update_rollback=update_rollback,
        packet_rel=args.packet,
        support_projection_rel=args.support_projection,
        schema_rel=args.schema,
        artifact_graph_rel=args.artifact_graph,
        ring_rollout_rel=args.ring_rollout,
        update_rollback_rel=args.update_rollback,
    )
    findings.extend(fixture_findings)

    capture = build_capture(
        packet=packet,
        findings=findings,
        fixture_results=fixture_results,
        packet_rel=args.packet,
        schema_rel=args.schema,
        support_projection_rel=args.support_projection,
    )
    compare_or_write(
        repo_root / args.capture,
        as_json_text(capture),
        args.check,
        findings,
        "correction_train.generated.capture_stale",
    )

    if any(finding.severity == "error" for finding in findings):
        for finding in findings:
            print(
                f"{finding.severity}: {finding.check_id}: {finding.message} [{finding.ref}]",
                file=sys.stderr,
            )
        return 1
    print("correction train validation passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
