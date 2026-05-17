#!/usr/bin/env python3
"""Validate beta publish, rollback, revocation, and advisory rehearsals."""

from __future__ import annotations

import argparse
import dataclasses
import json
import sys
from pathlib import Path
from typing import Any

from tools.ci.m3._common import render_yaml_as_json


DEFAULT_PACKET_REL = "artifacts/release/m3/rehearsals/packet.json"
DEFAULT_SCHEMA_REL = "schemas/release/rehearsal_packet.schema.json"
DEFAULT_SUPPORT_PROJECTION_REL = (
    "artifacts/release/m3/rehearsals/support_export_projection.json"
)
DEFAULT_CAPTURE_REL = (
    "artifacts/release/m3/rehearsals/captures/"
    "publish_rollback_revocation_rehearsal_validation_capture.json"
)
DEFAULT_FIXTURE_MANIFEST_REL = "fixtures/release/m3/rehearsal_inputs/manifest.yaml"
DEFAULT_ARTIFACT_GRAPH_REL = "artifacts/release/m3/artifact_graph.json"
DEFAULT_ROLLBACK_PLAN_REL = "artifacts/release/m3/update_rollback/rollback_plan.json"
DEFAULT_CORRECTION_TRAIN_REL = "artifacts/release/m3/correction_train/packet.json"

EXPECTED_SCHEMA_VERSION = 1
EXPECTED_PACKET_KIND = "release_rehearsal_packet"
EXPECTED_SUPPORT_KIND = "release_rehearsal_support_projection"
EXPECTED_CAPTURE_KIND = "release_rehearsal_validation_capture"
REQUIRED_FLOW_CLASSES = {"publish", "rollback", "revocation", "advisory"}
REQUIRED_SURFACES = {"headless_dry_run_output", "support_export", "docs_help"}
BLOCKING_DECISIONS = {"promotion_blocker", "support_blocker", "hold_channel"}
DOWNGRADE_DECISIONS = {"claim_downgrade"}
OPAQUE_PREFIXES = (
    "advisory_record:",
    "artifact_bundle:",
    "artifact_graph:",
    "artifact_node:",
    "build-id:",
    "channel:",
    "claim_row:",
    "compat_row:",
    "correction:",
    "docs:",
    "evidence:",
    "known_limit:",
    "policy:",
    "promotion_timeline:",
    "publish_target:",
    "release_candidate:",
    "release_line:",
    "rehearsal:",
    "rehearsal_flow:",
    "revocation_propagation_record:",
    "rollback_record:",
    "schema:",
    "support.packet:",
    "support_projection:",
    "update_manifest:",
)


@dataclasses.dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    remediation: str
    ref: str | None = None
    details: dict[str, Any] = dataclasses.field(default_factory=dict)

    def as_report(self) -> dict[str, Any]:
        payload = dataclasses.asdict(self)
        if payload["ref"] is None:
            payload.pop("ref")
        if not payload["details"]:
            payload.pop("details")
        return payload


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--packet", default=DEFAULT_PACKET_REL)
    parser.add_argument("--schema", default=DEFAULT_SCHEMA_REL)
    parser.add_argument("--support-projection", default=DEFAULT_SUPPORT_PROJECTION_REL)
    parser.add_argument("--capture", default=DEFAULT_CAPTURE_REL)
    parser.add_argument("--fixture-manifest", default=DEFAULT_FIXTURE_MANIFEST_REL)
    parser.add_argument("--artifact-graph", default=DEFAULT_ARTIFACT_GRAPH_REL)
    parser.add_argument("--rollback-plan", default=DEFAULT_ROLLBACK_PLAN_REL)
    parser.add_argument("--correction-train", default=DEFAULT_CORRECTION_TRAIN_REL)
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


def strip_fragment(ref: str) -> str:
    return ref.split("#", 1)[0]


def is_repo_ref(ref: Any) -> bool:
    return isinstance(ref, str) and ref and not ref.startswith(OPAQUE_PREFIXES)


def render_json(payload: dict[str, Any]) -> str:
    return json.dumps(payload, indent=2, sort_keys=True) + "\n"


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
                "rehearsal_packet.schema.validation",
                f"{path}: {error.message}",
                "Update the rehearsal packet or schema so the checked-in record validates.",
                f"{payload_ref}#{path}",
            )
        )
    return findings


def validate_repo_ref(
    repo_root: Path,
    ref: Any,
    findings: list[Finding],
    check_id: str,
    owner: str,
    generated_refs: set[str] | None = None,
) -> None:
    if not isinstance(ref, str) or not ref:
        findings.append(
            Finding(
                "error",
                check_id,
                "reference must be a non-empty string",
                "Provide a stable repo-relative or opaque ref.",
                owner,
            )
        )
        return
    target = strip_fragment(ref)
    if generated_refs and target in generated_refs:
        return
    if is_repo_ref(ref) and not (repo_root / target).exists():
        findings.append(
            Finding(
                "error",
                check_id,
                f"referenced artifact does not exist: {ref}",
                "Add the missing artifact or correct the rehearsal reference.",
                owner,
            )
        )


def build_support_projection(packet: dict[str, Any], packet_rel: str) -> dict[str, Any]:
    support = ensure_dict(packet.get("support_projection"), "packet.support_projection")
    scope = ensure_dict(packet.get("release_scope"), "packet.release_scope")
    rows: list[dict[str, Any]] = []
    decision_counts: dict[str, int] = {}
    result_counts: dict[str, int] = {}
    mirror_states: dict[str, int] = {}
    offline_states: dict[str, int] = {}
    for flow in ensure_list(packet.get("flows"), "packet.flows"):
        flow = ensure_dict(flow, "packet.flows[]")
        decision = ensure_dict(flow.get("gap_decision"), f"{flow.get('flow_id')}.gap_decision")
        mirror = ensure_dict(
            flow.get("mirror_offline_implications"),
            f"{flow.get('flow_id')}.mirror_offline_implications",
        )
        decision_class = decision.get("decision_class")
        result_state = flow.get("result_state")
        mirror_state = mirror.get("mirror_state")
        offline_state = mirror.get("offline_state")
        decision_counts[str(decision_class)] = decision_counts.get(str(decision_class), 0) + 1
        result_counts[str(result_state)] = result_counts.get(str(result_state), 0) + 1
        mirror_states[str(mirror_state)] = mirror_states.get(str(mirror_state), 0) + 1
        offline_states[str(offline_state)] = offline_states.get(str(offline_state), 0) + 1
        rows.append(
            {
                "flow_id": flow.get("flow_id"),
                "flow_class": flow.get("flow_class"),
                "title": flow.get("title"),
                "result_state": result_state,
                "evidence_state": flow.get("evidence_state"),
                "decision_class": decision_class,
                "decision_refs": decision.get("decision_refs", []),
                "blocks_channel_widening": decision.get("blocks_channel_widening"),
                "mirror_state": mirror_state,
                "offline_state": offline_state,
                "implication_refs": mirror.get("implication_refs", []),
                "exact_build_identity_ref": flow.get("exact_build_identity_ref"),
                "primary_artifact_refs": flow.get("primary_artifact_refs", []),
                "release_center_refs": flow.get("release_center_refs", []),
                "validation_refs": flow.get("validation_refs", []),
                "surface_classes": [
                    projection.get("surface_class")
                    for projection in ensure_list(
                        flow.get("surface_projections", []),
                        f"{flow.get('flow_id')}.surface_projections",
                    )
                    if isinstance(projection, dict)
                ],
                "support_ref": flow.get("support_ref"),
            }
        )

    return {
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "record_kind": EXPECTED_SUPPORT_KIND,
        "projection_id": support.get("projection_id"),
        "source_packet_ref": packet_rel,
        "generated_at": packet.get("generated_at"),
        "release_candidate_ref": scope.get("release_candidate_ref"),
        "version": scope.get("version"),
        "artifact_graph_id": scope.get("artifact_graph_id"),
        "artifact_bundle_ref": scope.get("artifact_bundle_ref"),
        "rollback_target_ref": scope.get("rollback_target_ref"),
        "exact_build_identity_refs": scope.get("exact_build_identity_refs", []),
        "summary": {
            "flow_count": len(rows),
            "flow_classes": sorted({str(row["flow_class"]) for row in rows}),
            "result_states": result_counts,
            "decision_classes": decision_counts,
            "mirror_states": mirror_states,
            "offline_states": offline_states,
            "blocking_flow_count": sum(
                1 for row in rows if row["decision_class"] in BLOCKING_DECISIONS
            ),
            "downgraded_flow_count": sum(
                1 for row in rows if row["decision_class"] in DOWNGRADE_DECISIONS
            ),
            "raw_private_material_included": False,
        },
        "rows": rows,
        "consumer_refs": support.get("consumer_refs", []),
        "required_fields": support.get("required_fields", []),
    }


class Validator:
    def __init__(
        self,
        repo_root: Path,
        packet: dict[str, Any],
        artifact_graph: dict[str, Any],
        rollback_plan: dict[str, Any],
        correction_train: dict[str, Any],
        fixture_manifest: dict[str, Any],
        packet_rel: str,
        support_projection_rel: str,
        fixture_manifest_rel: str,
    ) -> None:
        self.repo_root = repo_root
        self.packet = packet
        self.artifact_graph = artifact_graph
        self.rollback_plan = rollback_plan
        self.correction_train = correction_train
        self.fixture_manifest = fixture_manifest
        self.packet_rel = packet_rel
        self.support_projection_rel = support_projection_rel
        self.fixture_manifest_rel = fixture_manifest_rel
        self.findings: list[Finding] = []
        self.fixture_results: list[dict[str, Any]] = []
        self.generated_refs = {support_projection_rel, DEFAULT_CAPTURE_REL}
        self.graph_node_ids = {
            node.get("node_id")
            for node in ensure_list(artifact_graph.get("artifact_nodes", []), "artifact_graph.artifact_nodes")
            if isinstance(node, dict)
        }
        self.graph_exact_build_refs = {
            row.get("exact_build_identity_ref")
            for row in ensure_list(
                artifact_graph.get("exact_build_identities", []),
                "artifact_graph.exact_build_identities",
            )
            if isinstance(row, dict)
        }
        self.correction_item_ids = {
            row.get("item_id")
            for row in ensure_list(
                correction_train.get("correction_items", []),
                "correction_train.correction_items",
            )
            if isinstance(row, dict)
        }

    def push(
        self,
        check_id: str,
        message: str,
        remediation: str,
        ref: str | None = None,
        severity: str = "error",
        details: dict[str, Any] | None = None,
    ) -> None:
        self.findings.append(
            Finding(severity, check_id, message, remediation, ref, details or {})
        )

    def validate(self) -> tuple[list[Finding], list[dict[str, Any]]]:
        self.validate_header()
        self.validate_source_refs()
        self.validate_scope()
        self.validate_acceptance()
        self.validate_flows()
        self.validate_fixtures()
        return self.findings, self.fixture_results

    def validate_header(self) -> None:
        if self.packet.get("schema_version") != EXPECTED_SCHEMA_VERSION:
            self.push(
                "packet.schema_version",
                "schema_version must be 1",
                "Keep the packet on schema version 1 until a governed migration exists.",
                self.packet_rel,
            )
        if self.packet.get("record_kind") != EXPECTED_PACKET_KIND:
            self.push(
                "packet.record_kind",
                f"record_kind must be {EXPECTED_PACKET_KIND}",
                "Use the release rehearsal packet discriminator.",
                self.packet_rel,
            )

    def validate_source_refs(self) -> None:
        source_refs = ensure_dict(self.packet.get("source_refs"), "packet.source_refs")
        for key, ref in source_refs.items():
            validate_repo_ref(
                self.repo_root,
                ref,
                self.findings,
                "packet.source_refs.missing",
                f"source_refs.{key}",
                self.generated_refs,
            )
        support = ensure_dict(self.packet.get("support_projection"), "packet.support_projection")
        if support.get("output_ref") != self.support_projection_rel:
            self.push(
                "support_projection.output_ref",
                "support projection output does not match validator output",
                "Keep packet.support_projection.output_ref aligned with the headless validator.",
                "support_projection.output_ref",
            )
        for ref in ensure_list(support.get("consumer_refs"), "support_projection.consumer_refs"):
            validate_repo_ref(
                self.repo_root,
                ref,
                self.findings,
                "support_projection.consumer_refs.missing",
                "support_projection.consumer_refs",
                self.generated_refs,
            )

    def validate_scope(self) -> None:
        scope = ensure_dict(self.packet.get("release_scope"), "packet.release_scope")
        graph_candidate = ensure_dict(self.artifact_graph.get("candidate"), "artifact_graph.candidate")
        graph_bundle = ensure_dict(
            self.artifact_graph.get("artifact_bundle"),
            "artifact_graph.artifact_bundle",
        )
        graph_channel = ensure_dict(
            self.artifact_graph.get("release_channel"),
            "artifact_graph.release_channel",
        )
        expected = {
            "release_candidate_ref": graph_candidate.get("candidate_ref"),
            "version": graph_candidate.get("version"),
            "channel_class": graph_channel.get("channel_class"),
            "stage_class": graph_channel.get("stage_class"),
            "artifact_graph_id": self.artifact_graph.get("graph_id"),
            "artifact_bundle_ref": graph_bundle.get("bundle_id"),
            "rollback_target_ref": graph_bundle.get("rollback_target_ref"),
        }
        for key, expected_value in expected.items():
            if scope.get(key) != expected_value:
                self.push(
                    f"release_scope.{key}.mismatch",
                    f"release scope {key} does not match the artifact graph",
                    "Refresh the rehearsal packet from the current beta artifact graph.",
                    f"release_scope.{key}",
                    details={"expected": expected_value, "actual": scope.get(key)},
                )
        scope_exact = set(ensure_list(scope.get("exact_build_identity_refs"), "release_scope.exact_build_identity_refs"))
        if scope_exact != self.graph_exact_build_refs:
            self.push(
                "release_scope.exact_build_identity_refs.mismatch",
                "release scope exact-build refs do not match the artifact graph",
                "Keep rehearsal exact-build identity refs identical to artifact_graph.exact_build_identities.",
                "release_scope.exact_build_identity_refs",
                details={"expected": sorted(self.graph_exact_build_refs), "actual": sorted(scope_exact)},
            )

    def validate_acceptance(self) -> None:
        acceptance = ensure_dict(self.packet.get("acceptance"), "packet.acceptance")
        if acceptance.get("fixture_manifest_ref") != self.fixture_manifest_rel:
            self.push(
                "acceptance.fixture_manifest_ref",
                "acceptance fixture manifest ref does not match validator input",
                "Keep the fixture manifest path aligned with the validator default.",
                "acceptance.fixture_manifest_ref",
            )
        required = set(ensure_list(acceptance.get("required_flow_classes"), "acceptance.required_flow_classes"))
        if required != REQUIRED_FLOW_CLASSES:
            self.push(
                "acceptance.required_flow_classes",
                "acceptance must require publish, rollback, revocation, and advisory flows",
                "List all required flow classes exactly once.",
                "acceptance.required_flow_classes",
                details={"expected": sorted(REQUIRED_FLOW_CLASSES), "actual": sorted(required)},
            )
        command = "python3 -m tools.ci.m3.rehearsals --repo-root . --check"
        commands = set(ensure_list(acceptance.get("validation_commands"), "acceptance.validation_commands"))
        if command not in commands:
            self.push(
                "acceptance.validation_commands.missing",
                "acceptance must include the headless rehearsal validation command",
                "Add the validator command so release reviewers can reproduce the packet.",
                "acceptance.validation_commands",
            )

    def validate_flows(self) -> None:
        flows = [
            ensure_dict(flow, "packet.flows[]")
            for flow in ensure_list(self.packet.get("flows"), "packet.flows")
        ]
        flow_classes = {flow.get("flow_class") for flow in flows}
        missing = REQUIRED_FLOW_CLASSES - flow_classes
        if missing:
            self.push(
                "flows.required_classes.missing",
                "rehearsal packet is missing required flow classes",
                "Add one current flow for each required release-control action.",
                "packet.flows",
                details={"missing": sorted(missing)},
            )
        duplicate_classes = sorted(
            flow_class
            for flow_class in flow_classes
            if sum(1 for flow in flows if flow.get("flow_class") == flow_class) > 1
        )
        if duplicate_classes:
            self.push(
                "flows.required_classes.duplicate",
                "each required flow class must appear once",
                "Split variants into fixtures, not duplicate canonical packet rows.",
                "packet.flows",
                details={"duplicates": duplicate_classes},
            )
        for flow in flows:
            self.validate_flow(flow)

    def validate_flow(self, flow: dict[str, Any]) -> None:
        flow_id = str(flow.get("flow_id"))
        if flow.get("exact_build_identity_ref") not in self.graph_exact_build_refs:
            self.push(
                "flow.exact_build_identity_ref.unknown",
                "flow exact-build identity is not in the artifact graph",
                "Use an exact-build identity from the current beta graph.",
                flow_id,
            )
        for ref in ensure_list(flow.get("primary_artifact_refs"), f"{flow_id}.primary_artifact_refs"):
            if isinstance(ref, str) and ref.startswith("artifact_node:") and ref not in self.graph_node_ids:
                self.push(
                    "flow.primary_artifact_refs.unknown_node",
                    "flow cites an artifact node that is not in the graph",
                    "Use artifact node refs from the current beta graph.",
                    f"{flow_id}#{ref}",
                )
            validate_repo_ref(
                self.repo_root,
                ref,
                self.findings,
                "flow.primary_artifact_refs.missing",
                flow_id,
                self.generated_refs,
            )
        for ref in ensure_list(flow.get("validation_refs"), f"{flow_id}.validation_refs"):
            validate_repo_ref(
                self.repo_root,
                ref,
                self.findings,
                "flow.validation_refs.missing",
                flow_id,
                self.generated_refs,
            )
        validate_repo_ref(
            self.repo_root,
            flow.get("input_fixture_ref"),
            self.findings,
            "flow.input_fixture_ref.missing",
            flow_id,
            self.generated_refs,
        )
        decision = ensure_dict(flow.get("gap_decision"), f"{flow_id}.gap_decision")
        decision_class = decision.get("decision_class")
        result_state = flow.get("result_state")
        decision_refs = ensure_list(decision.get("decision_refs"), f"{flow_id}.decision_refs")
        if decision_class == "no_gap" and decision_refs:
            self.push(
                "flow.gap_decision.no_gap_has_refs",
                "no_gap decisions must not carry downgrade or blocker refs",
                "Use a downgrade/blocker decision class when there is a cited release decision.",
                flow_id,
            )
        if decision_class != "no_gap" and not decision_refs:
            self.push(
                "flow.gap_decision.refs_missing",
                "gap decisions must cite the downgrade or blocker record",
                "Add at least one decision ref so gaps feed release control.",
                flow_id,
            )
        if result_state in {"passed_with_downgrade", "blocked_with_decision"} and decision_class == "no_gap":
            self.push(
                "flow.gap_decision.result_incoherent",
                "downgraded or blocked flows must name a release decision",
                "Change the decision class or result state so they agree.",
                flow_id,
            )
        for ref in decision_refs:
            validate_repo_ref(
                self.repo_root,
                ref,
                self.findings,
                "flow.gap_decision.decision_refs.missing",
                flow_id,
                self.generated_refs,
            )
            fragment = str(ref).split("#", 1)[1] if "#" in str(ref) else str(ref)
            if fragment.startswith("correction:item.") and fragment not in self.correction_item_ids:
                self.push(
                    "flow.gap_decision.unknown_correction_item",
                    "gap decision cites an unknown correction-train item",
                    "Use a correction item id from the current correction-train packet.",
                    f"{flow_id}#{fragment}",
                )
        mirror = ensure_dict(
            flow.get("mirror_offline_implications"),
            f"{flow_id}.mirror_offline_implications",
        )
        if not ensure_list(mirror.get("implication_refs"), f"{flow_id}.implication_refs"):
            self.push(
                "flow.mirror_offline_implications.refs_empty",
                "mirror/offline implications must cite at least one source",
                "Add mirror, offline, or propagation refs for the flow.",
                flow_id,
            )
        for ref in ensure_list(mirror.get("implication_refs"), f"{flow_id}.implication_refs"):
            validate_repo_ref(
                self.repo_root,
                ref,
                self.findings,
                "flow.mirror_offline_implications.refs_missing",
                flow_id,
                self.generated_refs,
            )
        surfaces = {
            projection.get("surface_class")
            for projection in ensure_list(
                flow.get("surface_projections"), f"{flow_id}.surface_projections"
            )
            if isinstance(projection, dict)
        }
        missing_surfaces = REQUIRED_SURFACES - surfaces
        if missing_surfaces:
            self.push(
                "flow.surface_projections.required_missing",
                "flow is missing required consumer surface projections",
                "Expose every flow through headless output, support export, and docs/help.",
                flow_id,
                details={"missing": sorted(missing_surfaces)},
            )
        if flow.get("flow_class") == "rollback":
            self.validate_rollback_flow(flow)
        if flow.get("flow_class") in {"revocation", "advisory"} and decision_class == "no_gap":
            self.push(
                "flow.response_decision.missing",
                "revocation and advisory rehearsals must feed a downgrade or blocker decision",
                "Bind the response rehearsal to correction, claim downgrade, or release-hold refs.",
                flow_id,
            )

    def validate_rollback_flow(self, flow: dict[str, Any]) -> None:
        flow_id = str(flow.get("flow_id"))
        scope = ensure_dict(self.packet.get("release_scope"), "packet.release_scope")
        current = ensure_dict(self.rollback_plan.get("current_build"), "rollback_plan.current_build")
        target = ensure_dict(self.rollback_plan.get("rollback_target"), "rollback_plan.rollback_target")
        downgrade = ensure_dict(self.rollback_plan.get("downgrade_truth"), "rollback_plan.downgrade_truth")
        if current.get("release_candidate_ref") != scope.get("release_candidate_ref"):
            self.push(
                "rollback_flow.current_candidate.mismatch",
                "rollback plan current candidate does not match rehearsal scope",
                "Refresh the rollback plan or rehearsal packet from the same artifact graph.",
                flow_id,
            )
        if target.get("release_candidate_ref") != scope.get("rollback_target_ref"):
            self.push(
                "rollback_flow.target.mismatch",
                "rollback plan target does not match rehearsal scope",
                "Use the same rollback target across artifact graph, rollback plan, and rehearsal packet.",
                flow_id,
            )
        if downgrade.get("eligibility_state") in {"blocked", "unsupported"}:
            self.push(
                "rollback_flow.downgrade_truth.not_admitted",
                "rollback flow cannot pass while downgrade truth is blocked or unsupported",
                "Downgrade the rehearsal result or fix the rollback plan.",
                flow_id,
                details={"eligibility_state": downgrade.get("eligibility_state")},
            )

    def validate_fixtures(self) -> None:
        manifest = self.fixture_manifest
        if manifest.get("schema_version") != EXPECTED_SCHEMA_VERSION:
            self.push(
                "fixtures.schema_version",
                "fixture manifest schema_version must be 1",
                "Keep fixture manifest schema aligned with the validator.",
                self.fixture_manifest_rel,
            )
        if manifest.get("record_kind") != "release_rehearsal_fixture_manifest":
            self.push(
                "fixtures.record_kind",
                "fixture manifest has the wrong record kind",
                "Use release_rehearsal_fixture_manifest.",
                self.fixture_manifest_rel,
            )
        if manifest.get("packet_ref") != self.packet_rel:
            self.push(
                "fixtures.packet_ref",
                "fixture manifest must point at the canonical rehearsal packet",
                "Keep fixture manifest packet_ref aligned with the validator input.",
                self.fixture_manifest_rel,
            )
        cases = [
            ensure_dict(case, "fixture_manifest.cases[]")
            for case in ensure_list(manifest.get("cases"), "fixture_manifest.cases")
        ]
        case_classes = {case.get("flow_class") for case in cases}
        if case_classes != REQUIRED_FLOW_CLASSES:
            self.push(
                "fixtures.flow_classes",
                "fixture manifest must cover every rehearsal flow class",
                "Add one fixture case per required flow class.",
                self.fixture_manifest_rel,
                details={"expected": sorted(REQUIRED_FLOW_CLASSES), "actual": sorted(case_classes)},
            )
        flows_by_class = {
            flow.get("flow_class"): flow
            for flow in ensure_list(self.packet.get("flows"), "packet.flows")
            if isinstance(flow, dict)
        }
        for case in cases:
            self.validate_fixture_case(case, flows_by_class)

    def validate_fixture_case(
        self,
        case: dict[str, Any],
        flows_by_class: dict[Any, dict[str, Any]],
    ) -> None:
        case_id = str(case.get("case_id"))
        case_ref = str(case.get("case_ref"))
        validate_repo_ref(
            self.repo_root,
            case_ref,
            self.findings,
            "fixtures.case_ref.missing",
            case_id,
            self.generated_refs,
        )
        if not case_ref or not (self.repo_root / case_ref).exists():
            self.fixture_results.append({"case_id": case_id, "status": "missing"})
            return
        payload = ensure_dict(load_json(self.repo_root / case_ref), case_ref)
        flow_class = case.get("flow_class")
        flow = flows_by_class.get(flow_class)
        case_findings: list[str] = []
        if flow is None:
            case_findings.append("missing_flow_for_case_class")
        else:
            if case.get("expected_result_state") != flow.get("result_state"):
                case_findings.append("manifest_result_state_mismatch")
            if case.get("expected_decision_class") != flow.get("gap_decision", {}).get("decision_class"):
                case_findings.append("manifest_decision_class_mismatch")
            if payload.get("flow_id") != flow.get("flow_id"):
                case_findings.append("flow_id_mismatch")
            if payload.get("expected_result_state") != flow.get("result_state"):
                case_findings.append("result_state_mismatch")
            payload_decision = ensure_dict(payload.get("gap_decision"), f"{case_ref}.gap_decision")
            flow_decision = ensure_dict(flow.get("gap_decision"), f"{flow.get('flow_id')}.gap_decision")
            if payload_decision.get("decision_class") != flow_decision.get("decision_class"):
                case_findings.append("decision_class_mismatch")
            payload_mirror = ensure_dict(
                payload.get("mirror_offline_implications"),
                f"{case_ref}.mirror_offline_implications",
            )
            flow_mirror = ensure_dict(
                flow.get("mirror_offline_implications"),
                f"{flow.get('flow_id')}.mirror_offline_implications",
            )
            if payload_mirror.get("mirror_state") != flow_mirror.get("mirror_state"):
                case_findings.append("mirror_state_mismatch")
            if payload_mirror.get("offline_state") != flow_mirror.get("offline_state"):
                case_findings.append("offline_state_mismatch")
        scope = ensure_dict(self.packet.get("release_scope"), "packet.release_scope")
        if payload.get("record_kind") != "release_rehearsal_input":
            case_findings.append("record_kind_mismatch")
        if payload.get("flow_class") != flow_class:
            case_findings.append("flow_class_mismatch")
        if payload.get("release_candidate_ref") != scope.get("release_candidate_ref"):
            case_findings.append("release_candidate_ref_mismatch")
        if payload.get("artifact_graph_ref") != self.packet["source_refs"]["artifact_graph_ref"]:
            case_findings.append("artifact_graph_ref_mismatch")
        if payload.get("exact_build_identity_ref") not in self.graph_exact_build_refs:
            case_findings.append("unknown_exact_build_identity_ref")
        status = "passed" if not case_findings else "failed"
        self.fixture_results.append(
            {
                "case_id": case_id,
                "case_ref": case_ref,
                "flow_class": flow_class,
                "status": status,
                "findings": case_findings,
            }
        )
        if case_findings:
            self.push(
                "fixtures.case.failed",
                "rehearsal fixture does not match the canonical flow",
                "Update the fixture or canonical packet so the rehearsal input is inspectable.",
                case_ref,
                details={"case_findings": case_findings},
            )


def build_capture(
    packet: dict[str, Any],
    packet_rel: str,
    support_projection_rel: str,
    fixture_manifest_rel: str,
    fixture_results: list[dict[str, Any]],
    findings: list[Finding],
) -> dict[str, Any]:
    support = build_support_projection(packet, packet_rel)
    return {
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "record_kind": EXPECTED_CAPTURE_KIND,
        "capture_id": "release_rehearsal_validation:beta.2_1_0_beta_1",
        "generated_at": packet.get("generated_at"),
        "packet_ref": packet_rel,
        "support_projection_ref": support_projection_rel,
        "fixture_manifest_ref": fixture_manifest_rel,
        "result": "failed" if any(f.severity == "error" for f in findings) else "passed",
        "summary": support["summary"],
        "fixture_results": fixture_results,
        "findings": [finding.as_report() for finding in findings],
    }


def write_or_check(path: Path, content: str, check: bool) -> bool:
    if check:
        if not path.exists() or path.read_text(encoding="utf-8") != content:
            print(f"would update {path}", file=sys.stderr)
            return False
        return True
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")
    return True


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    packet = ensure_dict(load_json(repo_root / args.packet), args.packet)
    schema = ensure_dict(load_json(repo_root / args.schema), args.schema)
    artifact_graph = ensure_dict(load_json(repo_root / args.artifact_graph), args.artifact_graph)
    rollback_plan = ensure_dict(load_json(repo_root / args.rollback_plan), args.rollback_plan)
    correction_train = ensure_dict(load_json(repo_root / args.correction_train), args.correction_train)
    fixture_manifest = ensure_dict(
        render_yaml_as_json(repo_root / args.fixture_manifest),
        args.fixture_manifest,
    )

    schema_findings = validate_against_schema(packet, schema, args.packet)
    validator = Validator(
        repo_root,
        packet,
        artifact_graph,
        rollback_plan,
        correction_train,
        fixture_manifest,
        args.packet,
        args.support_projection,
        args.fixture_manifest,
    )
    findings, fixture_results = validator.validate()
    findings = schema_findings + findings
    support_projection = build_support_projection(packet, args.packet)
    capture = build_capture(
        packet,
        args.packet,
        args.support_projection,
        args.fixture_manifest,
        fixture_results,
        findings,
    )

    outputs_current = True
    outputs_current &= write_or_check(
        repo_root / args.support_projection,
        render_json(support_projection),
        args.check,
    )
    outputs_current &= write_or_check(
        repo_root / args.capture,
        render_json(capture),
        args.check,
    )

    errors = [finding for finding in findings if finding.severity == "error"]
    if errors:
        for finding in errors:
            print(
                f"{finding.check_id}: {finding.message}"
                + (f" ({finding.ref})" if finding.ref else ""),
                file=sys.stderr,
            )
        return 1
    if not outputs_current:
        return 1
    print(
        "release rehearsal validation passed: "
        f"{len(support_projection['rows'])} flows, "
        f"{len(fixture_results)} fixtures"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
