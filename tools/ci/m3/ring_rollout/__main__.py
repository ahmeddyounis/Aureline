#!/usr/bin/env python3
"""Validate beta ring rollout, silent deployment, and state-root audit evidence."""

from __future__ import annotations

import argparse
import dataclasses
import json
import sys
from pathlib import Path
from typing import Any

from tools.ci.m3._common import render_yaml_as_json


DEFAULT_PACKET_REL = "artifacts/release/m3/ring_rollout/packet.json"
DEFAULT_SILENT_RESULTS_REL = "artifacts/release/m3/ring_rollout/silent_deployment_results.json"
DEFAULT_SUPPORT_PROJECTION_REL = "artifacts/release/m3/ring_rollout/support_export_projection.json"
DEFAULT_STATE_ROOT_AUDIT_REL = "artifacts/release/m3/state_root_audit.md"
DEFAULT_CAPTURE_REL = "artifacts/release/m3/captures/ring_rollout_validation_capture.json"
DEFAULT_INSTALL_DIAGNOSTICS_REL = "artifacts/release/m3/install_diagnostics/install_diagnostics_packet.json"
DEFAULT_ARTIFACT_GRAPH_REL = "artifacts/release/m3/artifact_graph.json"
DEFAULT_STATE_ROOT_MAP_REL = "artifacts/release/state_root_map.yaml"
DEFAULT_RING_HISTORY_REL = "artifacts/release/m3/ring_rollout/ring_history_packet.json"
DEFAULT_FIXTURE_MANIFEST_REL = "fixtures/release/ring_rollout_cases/manifest.yaml"

EXPECTED_SCHEMA_VERSION = 1
EXPECTED_PACKET_RECORD_KIND = "ring_rollout_beta_packet"
EXPECTED_SILENT_PACKET_RECORD_KIND = "unattended_deployment_result_packet"
EXPECTED_SUPPORT_RECORD_KIND = "ring_rollout_support_export"
EXPECTED_CAPTURE_RECORD_KIND = "ring_rollout_validation_capture"
CONTROLLED_RING_VOCABULARY = ["canary", "pilot", "broad", "lts"]
REQUIRED_SURFACES = {"about", "diagnostics", "cli", "support_export"}
REQUIRED_SILENT_RESULT_KINDS = {"install", "update", "rollback", "uninstall", "verify"}
SUCCESS_RETURN_CODES = {
    "success": 0,
    "partial_success": 2,
    "user_config_error": 3,
    "trust_policy_denial": 4,
    "missing_dependency": 5,
    "network_transport": 6,
    "internal_failure": 7,
    "rollback_required": 8,
    "verification_failed": 9,
    "admin_required": 10,
}


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
    parser.add_argument("--silent-results", default=DEFAULT_SILENT_RESULTS_REL)
    parser.add_argument("--support-projection", default=DEFAULT_SUPPORT_PROJECTION_REL)
    parser.add_argument("--state-root-audit", default=DEFAULT_STATE_ROOT_AUDIT_REL)
    parser.add_argument("--capture", default=DEFAULT_CAPTURE_REL)
    parser.add_argument("--install-diagnostics", default=DEFAULT_INSTALL_DIAGNOSTICS_REL)
    parser.add_argument("--artifact-graph", default=DEFAULT_ARTIFACT_GRAPH_REL)
    parser.add_argument("--state-root-map", default=DEFAULT_STATE_ROOT_MAP_REL)
    parser.add_argument("--ring-history", default=DEFAULT_RING_HISTORY_REL)
    parser.add_argument("--fixture-manifest", default=DEFAULT_FIXTURE_MANIFEST_REL)
    parser.add_argument(
        "--check",
        action="store_true",
        help="Fail if generated projection, audit, or capture files would change.",
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


def as_json_text(payload: Any) -> str:
    return json.dumps(payload, indent=2, sort_keys=False) + "\n"


class Validator:
    def __init__(
        self,
        repo_root: Path,
        packet: dict[str, Any],
        silent_results: dict[str, Any],
        install_diagnostics: dict[str, Any],
        artifact_graph: dict[str, Any],
        state_root_map: dict[str, Any],
        ring_history: dict[str, Any],
        fixture_manifest: dict[str, Any],
    ) -> None:
        self.repo_root = repo_root
        self.packet = packet
        self.silent_results = silent_results
        self.install_diagnostics = install_diagnostics
        self.artifact_graph = artifact_graph
        self.state_root_map = state_root_map
        self.ring_history = ring_history
        self.fixture_manifest = fixture_manifest
        self.findings: list[Finding] = []
        self.diagnostic_rows = {
            row.get("diagnostic_row_id"): row
            for row in ensure_list(install_diagnostics.get("rows", []), "install_diagnostics.rows")
            if isinstance(row, dict)
        }
        self.state_root_rows = {
            row.get("id"): row
            for row in ensure_list(state_root_map.get("state_roots", []), "state_root_map.state_roots")
            if isinstance(row, dict)
        }

    def push(
        self,
        check_id: str,
        message: str,
        ref: str,
        remediation: str,
        severity: str = "error",
    ) -> None:
        self.findings.append(Finding(severity, check_id, message, ref, remediation))

    def validate(self) -> list[Finding]:
        self.validate_packet_header()
        self.validate_exact_build_identity()
        self.validate_state_root_audit()
        self.validate_silent_results()
        self.validate_lanes()
        self.validate_rollout_actions()
        self.validate_ring_history()
        self.validate_fixture_manifest()
        return self.findings

    def validate_packet_header(self) -> None:
        if self.packet.get("record_kind") != EXPECTED_PACKET_RECORD_KIND:
            self.push(
                "ring_rollout.packet.record_kind",
                f"packet.record_kind must be {EXPECTED_PACKET_RECORD_KIND}",
                str(self.packet.get("packet_id", "<packet>")),
                "Use the ring rollout packet discriminator.",
            )
        if self.packet.get("schema_version") != EXPECTED_SCHEMA_VERSION:
            self.push(
                "ring_rollout.packet.schema_version",
                "packet.schema_version must be 1",
                str(self.packet.get("packet_id", "<packet>")),
                "Keep the packet on schema version 1 until a governed migration exists.",
            )
        if self.packet.get("controlled_ring_vocabulary") != CONTROLLED_RING_VOCABULARY:
            self.push(
                "ring_rollout.packet.ring_vocabulary",
                "controlled ring vocabulary must be canary, pilot, broad, lts in order",
                str(self.packet.get("packet_id", "<packet>")),
                "Update the packet to use the install-topology rollout ring vocabulary.",
            )

    def validate_exact_build_identity(self) -> None:
        packet_refs = set(ensure_list(self.packet.get("exact_build_identity_refs", []), "packet.exact_build_identity_refs"))
        graph_refs = {
            row.get("exact_build_identity_ref")
            for row in ensure_list(
                self.artifact_graph.get("exact_build_identities", []),
                "artifact_graph.exact_build_identities",
            )
            if isinstance(row, dict)
        }
        diagnostic_refs = set(
            ensure_list(
                self.install_diagnostics.get("exact_build_identity_refs", []),
                "install_diagnostics.exact_build_identity_refs",
            )
        )
        for ref in sorted(packet_refs):
            if not isinstance(ref, str) or not ref.startswith("build-id:aureline:"):
                self.push(
                    "ring_rollout.exact_build.ref_invalid",
                    "exact-build refs must use the build-id:aureline namespace",
                    str(ref),
                    "Replace the value with a stable exact-build identity ref.",
                )
            if ref not in graph_refs:
                self.push(
                    "ring_rollout.exact_build.not_in_artifact_graph",
                    "exact-build ref is absent from the release artifact graph",
                    ref,
                    "Add the exact-build identity to the artifact graph or correct the rollout packet.",
                )
            if ref not in diagnostic_refs:
                self.push(
                    "ring_rollout.exact_build.not_in_install_diagnostics",
                    "exact-build ref is absent from install diagnostics",
                    ref,
                    "Refresh install diagnostics so rollout evidence and diagnostics share identity.",
                )

    def validate_state_root_audit(self) -> None:
        audit = ensure_dict(self.packet.get("state_root_audit", {}), "packet.state_root_audit")
        rows = ensure_list(audit.get("rows", []), "packet.state_root_audit.rows")
        seen_refs_by_channel: dict[str, set[str]] = {}
        for row in rows:
            if not isinstance(row, dict):
                self.push(
                    "ring_rollout.state_root_audit.row_shape",
                    "state-root audit rows must be objects",
                    str(row),
                    "Replace the audit row with an object.",
                )
                continue
            row_id = str(row.get("diagnostic_row_id", ""))
            diagnostic = self.diagnostic_rows.get(row_id)
            if diagnostic is None:
                self.push(
                    "ring_rollout.state_root_audit.diagnostic_missing",
                    "state-root audit row must reference an install diagnostics row",
                    row_id,
                    "Use a diagnostic_row_id from the install diagnostics packet.",
                )
                continue
            diagnostic_roots = {
                root.get("state_root_ref")
                for root in ensure_list(
                    diagnostic.get("durable_state_roots", []),
                    f"diagnostic[{row_id}].durable_state_roots",
                )
                if isinstance(root, dict)
            }
            audit_roots = set(ensure_list(row.get("state_root_refs", []), f"audit[{row_id}].state_root_refs"))
            if audit_roots != diagnostic_roots:
                self.push(
                    "ring_rollout.state_root_audit.diagnostic_mismatch",
                    "audit state roots must match the install diagnostics row",
                    row_id,
                    "Regenerate the audit from install diagnostics or update the packet row.",
                )
            missing_roots = sorted(ref for ref in audit_roots if ref not in self.state_root_rows)
            for root_ref in missing_roots:
                self.push(
                    "ring_rollout.state_root_audit.root_unknown",
                    "state-root ref is absent from the state-root map",
                    root_ref,
                    "Add the state-root row to the map or correct the audit ref.",
                )
            surfaces = set(ensure_list(row.get("surface_claims", []), f"audit[{row_id}].surface_claims"))
            missing_surfaces = REQUIRED_SURFACES - surfaces
            if missing_surfaces:
                self.push(
                    "ring_rollout.state_root_audit.surface_missing",
                    "state-root audit row must be visible in About, diagnostics, CLI, and support export",
                    row_id,
                    f"Add missing surface claims: {', '.join(sorted(missing_surfaces))}.",
                )
            if row.get("audit_status") != "pass":
                self.push(
                    "ring_rollout.state_root_audit.not_pass",
                    "claimed beta rollout audit rows must pass",
                    row_id,
                    "Narrow the claim or repair the audit row before publishing it.",
                )
            channel = str(row.get("channel_class", ""))
            seen_refs_by_channel.setdefault(channel, set()).update(audit_roots)

        channels = sorted(seen_refs_by_channel)
        for left_idx, left_channel in enumerate(channels):
            for right_channel in channels[left_idx + 1 :]:
                if left_channel == right_channel:
                    continue
                overlap = seen_refs_by_channel[left_channel] & seen_refs_by_channel[right_channel]
                mutable_overlap = [
                    ref
                    for ref in sorted(overlap)
                    if not self.state_root_rows.get(ref, {}).get("shared_across_channels", False)
                ]
                if mutable_overlap:
                    self.push(
                        "ring_rollout.state_root_audit.cross_channel_overlap",
                        "different channels must not share mutable state roots",
                        f"{left_channel}<->{right_channel}",
                        f"Separate or mark read-only shared roots: {', '.join(mutable_overlap)}.",
                    )

    def validate_silent_results(self) -> None:
        if self.silent_results.get("record_kind") != EXPECTED_SILENT_PACKET_RECORD_KIND:
            self.push(
                "ring_rollout.silent_results.record_kind",
                "silent deployment results must use the unattended packet record kind",
                str(self.silent_results.get("packet_id", "<silent-results>")),
                "Wrap rollout results in an unattended_deployment_result_packet.",
            )
        result_kinds: set[str] = set()
        packet_exact_refs = set(self.packet.get("exact_build_identity_refs", []))
        install_card_refs = set(self.packet.get("install_profile_card_refs", []))
        action_ids = {
            action.get("action_id")
            for action in ensure_list(self.packet.get("rollout_actions", []), "packet.rollout_actions")
            if isinstance(action, dict)
        }
        for result in ensure_list(self.silent_results.get("results", []), "silent_results.results"):
            if not isinstance(result, dict):
                self.push(
                    "ring_rollout.silent_results.row_shape",
                    "silent deployment result rows must be objects",
                    str(result),
                    "Replace the result row with an object.",
                )
                continue
            result_id = str(result.get("result_id", ""))
            result_kinds.add(str(result.get("result_kind", "")))
            family = result.get("return_code_family")
            expected_code = SUCCESS_RETURN_CODES.get(str(family))
            if expected_code is None or result.get("return_code_numeric") != expected_code:
                self.push(
                    "ring_rollout.silent_results.return_code",
                    "silent result numeric code must match its return-code family",
                    result_id,
                    "Use the numeric code frozen by the silent deployment seed.",
                )
            if result.get("running_build_identity_ref") not in packet_exact_refs:
                self.push(
                    "ring_rollout.silent_results.exact_build_missing",
                    "silent result must emit the packet exact-build identity",
                    result_id,
                    "Populate running_build_identity_ref from install diagnostics.",
                )
            if result.get("install_profile_card_ref") not in install_card_refs:
                self.push(
                    "ring_rollout.silent_results.install_card_unclaimed",
                    "silent result install-profile card must be claimed by the rollout packet",
                    result_id,
                    "Add the install profile card to the rollout packet or correct the result.",
                )
            if result.get("fleet_ring_context_ref") not in action_ids:
                self.push(
                    "ring_rollout.silent_results.action_missing",
                    "silent result must point at a rollout action",
                    result_id,
                    "Set fleet_ring_context_ref to a rollout action id.",
                )
            state_root_refs = result.get("state_root_refs") or []
            for root_ref in state_root_refs:
                if root_ref not in self.state_root_rows:
                    self.push(
                        "ring_rollout.silent_results.state_root_unknown",
                        "silent result state-root ref is absent from the state-root map",
                        str(root_ref),
                        "Use a state-root ref from artifacts/release/state_root_map.yaml.",
                    )
            if result.get("result_status") == "success" and result.get("failure_reason_class") is not None:
                self.push(
                    "ring_rollout.silent_results.success_failure_reason",
                    "successful silent results must not carry a failure reason",
                    result_id,
                    "Set failure_reason_class to null for success rows.",
                )
        missing_kinds = REQUIRED_SILENT_RESULT_KINDS - result_kinds
        if missing_kinds:
            self.push(
                "ring_rollout.silent_results.kind_coverage",
                "silent deployment packet must cover install, update, rollback, uninstall, and verify",
                str(self.silent_results.get("packet_id", "<silent-results>")),
                f"Add result rows for: {', '.join(sorted(missing_kinds))}.",
            )

    def validate_lanes(self) -> None:
        lane_types: set[str] = set()
        for lane in ensure_list(self.packet.get("lanes", []), "packet.lanes"):
            if not isinstance(lane, dict):
                self.push(
                    "ring_rollout.lane.row_shape",
                    "lane rows must be objects",
                    str(lane),
                    "Replace the lane row with an object.",
                )
                continue
            lane_id = str(lane.get("lane_id", ""))
            lane_types.add(str(lane.get("lane_type", "")))
            if lane.get("controlled_ring_vocabulary") != CONTROLLED_RING_VOCABULARY:
                self.push(
                    "ring_rollout.lane.ring_vocabulary",
                    "managed and self-serve lanes must quote the same controlled ring vocabulary",
                    lane_id,
                    "Set controlled_ring_vocabulary to canary, pilot, broad, lts.",
                )
            current_ring = lane.get("current_ring")
            if current_ring not in CONTROLLED_RING_VOCABULARY:
                self.push(
                    "ring_rollout.lane.current_ring",
                    "lane current ring must come from the controlled vocabulary",
                    lane_id,
                    "Use canary, pilot, broad, or lts.",
                )
            for row_id in ensure_list(lane.get("install_diagnostic_row_refs", []), f"lane[{lane_id}].install_diagnostic_row_refs"):
                if row_id not in self.diagnostic_rows:
                    self.push(
                        "ring_rollout.lane.diagnostic_missing",
                        "lane must reference existing install diagnostics rows",
                        str(row_id),
                        "Use diagnostic row ids from the install diagnostics packet.",
                    )
        if not {"managed", "self_serve"}.issubset(lane_types):
            self.push(
                "ring_rollout.lane.type_coverage",
                "rollout packet must include both managed and self-serve lanes",
                str(self.packet.get("packet_id", "<packet>")),
                "Add one lane_type=managed row and one lane_type=self_serve row.",
            )

    def validate_rollout_actions(self) -> None:
        lane_ids = {
            lane.get("lane_id")
            for lane in ensure_list(self.packet.get("lanes", []), "packet.lanes")
            if isinstance(lane, dict)
        }
        packet_exact_refs = set(self.packet.get("exact_build_identity_refs", []))
        silent_result_ids = {
            result.get("result_id")
            for result in ensure_list(self.silent_results.get("results", []), "silent_results.results")
            if isinstance(result, dict)
        }
        for action in ensure_list(self.packet.get("rollout_actions", []), "packet.rollout_actions"):
            if not isinstance(action, dict):
                self.push(
                    "ring_rollout.action.row_shape",
                    "rollout action rows must be objects",
                    str(action),
                    "Replace the rollout action with an object.",
                )
                continue
            action_id = str(action.get("action_id", ""))
            action_kind = action.get("action_kind")
            if action.get("lane_id") not in lane_ids:
                self.push(
                    "ring_rollout.action.lane_missing",
                    "rollout action must reference a packet lane",
                    action_id,
                    "Use a lane_id declared in packet.lanes.",
                )
            if action.get("to_ring") not in CONTROLLED_RING_VOCABULARY:
                self.push(
                    "ring_rollout.action.to_ring",
                    "rollout action target ring must come from the controlled vocabulary",
                    action_id,
                    "Use canary, pilot, broad, or lts.",
                )
            from_ring = action.get("from_ring")
            if from_ring is not None and from_ring not in CONTROLLED_RING_VOCABULARY:
                self.push(
                    "ring_rollout.action.from_ring",
                    "rollout action source ring must come from the controlled vocabulary",
                    action_id,
                    "Use canary, pilot, broad, lts, or null.",
                )
            if action.get("exact_build_identity_ref") not in packet_exact_refs:
                self.push(
                    "ring_rollout.action.exact_build_missing",
                    "rollout action must carry the packet exact-build identity",
                    action_id,
                    "Populate exact_build_identity_ref from install diagnostics.",
                )
            if action.get("diagnostic_row_ref") not in self.diagnostic_rows:
                self.push(
                    "ring_rollout.action.diagnostic_missing",
                    "rollout action must reference an install diagnostics row",
                    action_id,
                    "Use a diagnostic_row_id from the install diagnostics packet.",
                )
            for result_ref in ensure_list(action.get("silent_deployment_result_refs", []), f"action[{action_id}].silent_deployment_result_refs"):
                if result_ref not in silent_result_ids:
                    self.push(
                        "ring_rollout.action.silent_result_missing",
                        "rollout action must reference existing silent deployment results",
                        str(result_ref),
                        "Use result ids from the silent deployment result packet.",
                    )
            if action_kind in {"promote", "rollback"}:
                self.validate_action_visibility(action)

    def validate_action_visibility(self, action: dict[str, Any]) -> None:
        action_id = str(action.get("action_id", ""))
        prior = action.get("prior_package_ref")
        candidate = action.get("candidate_package_ref")
        pre = ensure_dict(action.get("pre_action", {}), f"action[{action_id}].pre_action")
        post = ensure_dict(action.get("post_action", {}), f"action[{action_id}].post_action")
        pre_visibility = ensure_dict(pre.get("package_visibility", {}), f"action[{action_id}].pre_action.package_visibility")
        post_visibility = ensure_dict(post.get("package_visibility", {}), f"action[{action_id}].post_action.package_visibility")
        if prior not in pre_visibility:
            self.push(
                "ring_rollout.action.prior_visibility_before_missing",
                "prior package visibility must be explicit before promotion or rollback",
                action_id,
                "Add prior_package_ref to pre_action.package_visibility.",
            )
        if post_visibility.get(prior) not in {"visible_active", "visible_rollback_target"}:
            self.push(
                "ring_rollout.action.prior_visibility_after_missing",
                "prior package must remain visible as active or rollback target after the action",
                action_id,
                "Preserve the prior package in post_action.package_visibility.",
            )
        if candidate not in post_visibility:
            self.push(
                "ring_rollout.action.candidate_visibility_after_missing",
                "candidate package visibility must be explicit after promotion or rollback",
                action_id,
                "Add candidate_package_ref to post_action.package_visibility.",
            )
        self.validate_channel_state(action_id, "pre_action", pre)
        self.validate_channel_state(action_id, "post_action", post)

    def validate_channel_state(self, action_id: str, stage: str, payload: dict[str, Any]) -> None:
        active_by_channel: dict[str, list[str]] = {}
        channel_rows = ensure_list(payload.get("channel_state", []), f"action[{action_id}].{stage}.channel_state")
        channel_classes: set[str] = set()
        for row in channel_rows:
            if not isinstance(row, dict):
                self.push(
                    "ring_rollout.action.channel_state_shape",
                    "channel state entries must be objects",
                    f"{action_id}.{stage}",
                    "Replace channel state entries with objects.",
                )
                continue
            channel_classes.add(str(row.get("channel_class", "")))
            if row.get("visibility_state") == "active":
                active_by_channel.setdefault(str(row.get("channel_class", "")), []).append(str(row.get("package_ref", "")))
        for channel in sorted(channel_classes):
            active_packages = active_by_channel.get(channel, [])
            if len(active_packages) != 1 or not active_packages[0]:
                self.push(
                    "ring_rollout.action.channel_state_ambiguous",
                    "each channel must have exactly one active package state",
                    f"{action_id}.{stage}.{channel}",
                    "Remove duplicate active rows or provide the active package ref.",
                )

    def validate_ring_history(self) -> None:
        release_validation = ensure_dict(self.packet.get("release_validation", {}), "packet.release_validation")
        if self.ring_history.get("record_kind") != "ring_history_packet_record":
            self.push(
                "ring_rollout.ring_history.record_kind",
                "ring history packet must use ring_history_packet_record",
                str(self.ring_history.get("packet_id", "<ring-history>")),
                "Use the ring history packet schema discriminator.",
            )
        if self.ring_history.get("current_ring") != release_validation.get("current_validation_ring"):
            self.push(
                "ring_rollout.ring_history.current_ring_mismatch",
                "ring history current ring must match the rollout packet release validation row",
                str(self.ring_history.get("packet_id", "<ring-history>")),
                "Refresh ring history or the rollout packet so validation posture agrees.",
            )
        latest = self.ring_history.get("latest_transition_id")
        transition_ids = {
            transition.get("transition_id")
            for transition in ensure_list(self.ring_history.get("transitions", []), "ring_history.transitions")
            if isinstance(transition, dict)
        }
        if latest not in transition_ids:
            self.push(
                "ring_rollout.ring_history.latest_transition_missing",
                "latest transition id must resolve inside the ring history packet",
                str(latest),
                "Set latest_transition_id to one of transitions[].transition_id.",
            )

    def validate_fixture_manifest(self) -> None:
        if self.fixture_manifest.get("record_kind") != "ring_rollout_fixture_manifest":
            self.push(
                "ring_rollout.fixture_manifest.record_kind",
                "fixture manifest must use ring_rollout_fixture_manifest",
                str(self.fixture_manifest.get("record_kind", "<fixture-manifest>")),
                "Use the fixture manifest discriminator.",
            )
        for ref_field in [
            "canonical_packet_ref",
            "canonical_silent_results_ref",
            "canonical_state_root_audit_ref",
            "canonical_support_projection_ref",
            "validator_ref",
        ]:
            ref = self.fixture_manifest.get(ref_field)
            if not isinstance(ref, str) or not ref:
                self.push(
                    "ring_rollout.fixture_manifest.ref_missing",
                    f"{ref_field} must be a non-empty repo ref",
                    ref_field,
                    "Populate the fixture manifest canonical refs.",
                )
                continue
            ref_path = ref.split("#", 1)[0]
            if not (self.repo_root / ref_path).exists():
                self.push(
                    "ring_rollout.fixture_manifest.ref_missing_on_disk",
                    "fixture manifest ref does not exist",
                    ref,
                    "Add the referenced file or correct the fixture manifest.",
                )
        mutations = ensure_list(
            self.fixture_manifest.get("failure_drill_mutations", []),
            "fixture_manifest.failure_drill_mutations",
        )
        if not mutations:
            self.push(
                "ring_rollout.fixture_manifest.mutations_missing",
                "fixture manifest must name failure-drill mutations",
                DEFAULT_FIXTURE_MANIFEST_REL,
                "Add mutation rows for expected rollout rejection classes.",
            )
        for mutation in mutations:
            if not isinstance(mutation, dict):
                self.push(
                    "ring_rollout.fixture_manifest.mutation_shape",
                    "fixture manifest mutations must be objects",
                    str(mutation),
                    "Replace the mutation row with an object.",
                )
                continue
            for field in ["mutation_id", "target_ref", "expected_finding", "summary"]:
                if not mutation.get(field):
                    self.push(
                        "ring_rollout.fixture_manifest.mutation_field_missing",
                        f"fixture mutation must include {field}",
                        str(mutation.get("mutation_id", "<mutation>")),
                        "Populate the missing mutation field.",
                    )


def build_support_projection(packet: dict[str, Any], silent_results: dict[str, Any]) -> dict[str, Any]:
    results_by_action: dict[str, list[str]] = {}
    for result in silent_results.get("results", []):
        if not isinstance(result, dict):
            continue
        action_ref = result.get("fleet_ring_context_ref")
        results_by_action.setdefault(str(action_ref), []).append(str(result.get("result_id")))

    action_rows = []
    for action in packet.get("rollout_actions", []):
        if not isinstance(action, dict):
            continue
        post = action.get("post_action", {})
        active_channel_states = [
            row
            for row in post.get("channel_state", [])
            if isinstance(row, dict) and row.get("visibility_state") == "active"
        ]
        action_rows.append(
            {
                "action_id": action.get("action_id"),
                "lane_id": action.get("lane_id"),
                "action_kind": action.get("action_kind"),
                "from_ring": action.get("from_ring"),
                "to_ring": action.get("to_ring"),
                "release_channel_class": action.get("release_channel_class"),
                "install_channel_class": action.get("install_channel_class"),
                "exact_build_identity_ref": action.get("exact_build_identity_ref"),
                "diagnostic_row_ref": action.get("diagnostic_row_ref"),
                "state_root_audit_ref": action.get("state_root_audit_ref"),
                "silent_deployment_result_refs": results_by_action.get(action.get("action_id"), []),
                "prior_package_ref": action.get("prior_package_ref"),
                "candidate_package_ref": action.get("candidate_package_ref"),
                "rollback_target_ref": action.get("rollback_target_ref"),
                "post_action_package_visibility": post.get("package_visibility", {}),
                "active_channel_states": active_channel_states,
            }
        )

    return {
        "record_kind": EXPECTED_SUPPORT_RECORD_KIND,
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "packet_id": packet.get("packet_id"),
        "source_packet_ref": DEFAULT_PACKET_REL,
        "generated_at": packet.get("generated_at"),
        "controlled_ring_vocabulary": CONTROLLED_RING_VOCABULARY,
        "exact_build_identity_refs": packet.get("exact_build_identity_refs", []),
        "lanes": [
            {
                "lane_id": lane.get("lane_id"),
                "lane_type": lane.get("lane_type"),
                "current_ring": lane.get("current_ring"),
                "install_diagnostic_row_refs": lane.get("install_diagnostic_row_refs", []),
            }
            for lane in packet.get("lanes", [])
            if isinstance(lane, dict)
        ],
        "rollout_actions": action_rows,
        "state_root_audit_rows": [
            {
                "diagnostic_row_id": row.get("diagnostic_row_id"),
                "channel_class": row.get("channel_class"),
                "install_mode_class": row.get("install_mode_class"),
                "state_root_refs": row.get("state_root_refs", []),
                "audit_status": row.get("audit_status"),
            }
            for row in packet.get("state_root_audit", {}).get("rows", [])
            if isinstance(row, dict)
        ],
        "redaction_class": "metadata_only_no_paths_or_secrets",
    }


def build_state_root_audit_markdown(packet: dict[str, Any]) -> str:
    audit = packet.get("state_root_audit", {})
    rows = audit.get("rows", [])
    lines = [
        "# Beta state-root audit",
        "",
        "This audit is generated from the beta ring-rollout packet and the exact-build install diagnostics packet. It keeps state-root ownership inspectable for silent deployment, managed rollout, self-serve rollout, and rollback review without embedding host-specific paths.",
        "",
        f"- **Audit id:** `{audit.get('audit_id')}`",
        f"- **Source packet:** `artifacts/release/m3/ring_rollout/packet.json`",
        f"- **Generated at:** `{packet.get('generated_at')}`",
        f"- **Exact build:** `{', '.join(packet.get('exact_build_identity_refs', []))}`",
        f"- **State-root map:** `{packet.get('contract_refs', {}).get('state_root_map_ref')}`",
        "",
        "| Diagnostic row | Channel | Install mode | Updater owner | State roots | Review | Result |",
        "|---|---|---|---|---|---|---|",
    ]
    for row in rows:
        roots = "<br>".join(f"`{root}`" for root in row.get("state_root_refs", []))
        lines.append(
            "| `{diagnostic}` | `{channel}` | `{install_mode}` | `{updater}` | {roots} | `{review}` | `{status}` |".format(
                diagnostic=row.get("diagnostic_row_id"),
                channel=row.get("channel_class"),
                install_mode=row.get("install_mode_class"),
                updater=row.get("updater_owner_class"),
                roots=roots,
                review=row.get("state_root_review_class"),
                status=row.get("audit_status"),
            )
        )
    lines.extend(["", "## Audit Row Refs", ""])
    for row in rows:
        row_id = row.get("diagnostic_row_id")
        lines.append(f'<a id="audit-row-{row_id}"></a>')
        lines.append(f"- `{row_id}`")
    lines.extend(["", "## Findings", ""])
    for finding in audit.get("findings", []):
        lines.append(f"- `{finding.get('finding_id')}`: {finding.get('summary')}")
    lines.extend(
        [
            "",
            "## Consumer Rule",
            "",
            "About, diagnostics, CLI, silent-deployment summaries, and support export quote these state-root refs. A rollout or rollback action is non-conforming if it names a state root that is absent from this audit or if it creates more than one active package state for a channel.",
            "",
        ]
    )
    return "\n".join(lines)


def build_capture(
    packet: dict[str, Any],
    findings: list[Finding],
    support_projection_rel: str,
    state_root_audit_rel: str,
    fixture_manifest: dict[str, Any],
) -> dict[str, Any]:
    lanes = [lane for lane in packet.get("lanes", []) if isinstance(lane, dict)]
    actions = [action for action in packet.get("rollout_actions", []) if isinstance(action, dict)]
    return {
        "record_kind": EXPECTED_CAPTURE_RECORD_KIND,
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "validated_at": packet.get("generated_at"),
        "packet_id": packet.get("packet_id"),
        "source_packet_ref": DEFAULT_PACKET_REL,
        "support_projection_ref": support_projection_rel,
        "state_root_audit_ref": state_root_audit_rel,
        "passed": not findings,
        "coverage": {
            "controlled_ring_vocabulary": CONTROLLED_RING_VOCABULARY,
            "lane_types": sorted({lane.get("lane_type") for lane in lanes}),
            "rollout_action_kinds": sorted({action.get("action_kind") for action in actions}),
            "exact_build_identity_refs": packet.get("exact_build_identity_refs", []),
            "state_root_audit_row_count": len(packet.get("state_root_audit", {}).get("rows", [])),
            "rollout_action_count": len(actions),
            "fixture_mutation_count": len(fixture_manifest.get("failure_drill_mutations", [])),
        },
        "findings": [finding.as_report() for finding in findings],
    }


def compare_or_write(path: Path, expected_text: str, check: bool, findings: list[Finding], check_id: str) -> None:
    if check:
        actual = path.read_text(encoding="utf-8") if path.exists() else ""
        if actual != expected_text:
            findings.append(
                Finding(
                    "error",
                    check_id,
                    "checked-in generated file is stale",
                    str(path),
                    "Run the ring rollout validator without --check and commit the generated output.",
                )
            )
        return
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(expected_text, encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    packet = ensure_dict(load_json(repo_root / args.packet), "packet")
    silent_results = ensure_dict(load_json(repo_root / args.silent_results), "silent_results")
    install_diagnostics = ensure_dict(load_json(repo_root / args.install_diagnostics), "install_diagnostics")
    artifact_graph = ensure_dict(load_json(repo_root / args.artifact_graph), "artifact_graph")
    state_root_map = ensure_dict(render_yaml_as_json(repo_root / args.state_root_map), "state_root_map")
    ring_history = ensure_dict(load_json(repo_root / args.ring_history), "ring_history")
    fixture_manifest = ensure_dict(
        render_yaml_as_json(repo_root / args.fixture_manifest),
        "fixture_manifest",
    )

    validator = Validator(
        repo_root,
        packet,
        silent_results,
        install_diagnostics,
        artifact_graph,
        state_root_map,
        ring_history,
        fixture_manifest,
    )
    findings = validator.validate()

    support_projection = build_support_projection(packet, silent_results)
    state_root_audit = build_state_root_audit_markdown(packet)
    capture = build_capture(
        packet,
        findings,
        args.support_projection,
        args.state_root_audit,
        fixture_manifest,
    )

    compare_or_write(
        repo_root / args.support_projection,
        as_json_text(support_projection),
        args.check,
        findings,
        "ring_rollout.generated.support_projection_stale",
    )
    compare_or_write(
        repo_root / args.state_root_audit,
        state_root_audit,
        args.check,
        findings,
        "ring_rollout.generated.state_root_audit_stale",
    )
    capture = build_capture(
        packet,
        findings,
        args.support_projection,
        args.state_root_audit,
        fixture_manifest,
    )
    compare_or_write(
        repo_root / args.capture,
        as_json_text(capture),
        args.check,
        findings,
        "ring_rollout.generated.capture_stale",
    )

    if findings:
        for finding in findings:
            print(
                f"{finding.severity}: {finding.check_id}: {finding.message} [{finding.ref}]",
                file=sys.stderr,
            )
        return 1
    print("ring rollout validation passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
