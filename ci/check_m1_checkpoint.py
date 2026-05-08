#!/usr/bin/env python3
"""Validate the shell/editor/workspace integration checkpoint dependency graph.

This check keeps the integration checkpoint executable by ensuring the
canonical dependency graph artifact:

- exists and parses;
- carries a stable schema/owner header;
- names the integrated flow steps explicitly; and
- references only real prerequisite artifacts (or explicit sentinels for
  planned-but-not-seeded assets).

It also validates the proof-index consumer that downstream review packets and
dashboards should read instead of cloning prerequisite lists.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import re
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any

DEFAULT_GRAPH_REL = "artifacts/milestones/m1/dependency_graph.yaml"
DEFAULT_INDEX_REL = "artifacts/milestones/m1/artifact_index.yaml"

SENTINEL_REFS = {
    "not_yet_seeded",
    "outline_only",
    "contract_not_yet_seeded",
    "planned_not_yet_seeded",
}

REQUIRED_FLOW_STEPS = ["start_center", "open_repo", "edit", "save", "restore_session"]
REQUIRED_SUBSYSTEMS = {"shell", "editor", "workspace", "telemetry", "recovery"}

BUILD_IDENTITY_REQUIRED_KEYS = {
    "schema_version",
    "commit",
    "commit_short",
    "dirty",
    "toolchain_channel",
    "rustc_version",
    "cargo_version",
    "host_triple",
    "target_triple",
    "profile",
    "workspace_version",
    "source_date_epoch",
    "build_timestamp_utc",
}

COMMIT_FULL_RE = re.compile(r"^(?:[0-9a-f]{40}|unknown)$")


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
    parser.add_argument("--graph", default=DEFAULT_GRAPH_REL)
    parser.add_argument("--index", default=DEFAULT_INDEX_REL)
    parser.add_argument(
        "--report",
        default=None,
        help="Write a machine-readable JSON report to this repo-relative path.",
    )
    return parser.parse_args()


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
                "payload = YAML.safe_load(File.read(ARGV[0]), permitted_classes: [], aliases: false); "
                "STDOUT.write(JSON.generate(payload))"
            ),
            str(path),
        ],
        capture_output=True,
        text=True,
    )
    if ruby.returncode != 0:
        stderr = ruby.stderr.strip() or "unknown Ruby/Psych failure"
        raise SystemExit(f"failed to parse YAML at {path} via Ruby/Psych: {stderr}")
    try:
        return json.loads(ruby.stdout)
    except json.JSONDecodeError as exc:
        raise SystemExit(f"Ruby/Psych emitted invalid JSON for {path}: {exc}") from exc


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be a YAML mapping/object")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a YAML list/array")
    return value


def ensure_str(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value.strip():
        raise SystemExit(f"{label} must be a non-empty string")
    return value.strip()


def ensure_int(value: Any, label: str) -> int:
    if not isinstance(value, int):
        raise SystemExit(f"{label} must be an integer")
    return value


def parse_iso_date(value: str, label: str) -> dt.date:
    try:
        return dt.date.fromisoformat(value)
    except ValueError as exc:
        raise SystemExit(f"{label} must be a YYYY-MM-DD date, got {value!r}") from exc


def strip_fragment(ref: str) -> str:
    return ref.split("#", 1)[0].strip()


def artifact_ref_exists(repo_root: Path, ref: str) -> bool:
    ref = ref.strip()
    if ref in SENTINEL_REFS:
        return False
    path = strip_fragment(ref)
    if not path:
        return False
    return (repo_root / path).exists()


def validate_required_refs(repo_root: Path, refs: list[Any], check_id: str, label: str) -> list[Finding]:
    findings: list[Finding] = []
    for idx, ref in enumerate(refs):
        if not isinstance(ref, str) or not ref.strip():
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"{check_id}.invalid_ref",
                    message=f"{label}[{idx}] must be a non-empty string",
                    remediation="Replace the empty or non-string ref with a repo-relative path (optionally with a #fragment).",
                )
            )
            continue
        ref = ref.strip()
        if ref in SENTINEL_REFS:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"{check_id}.sentinel_in_required",
                    message=f"{label}[{idx}] is a sentinel ref but is marked required: {ref}",
                    remediation="Replace the sentinel with a real artifact path, or move this ref under a planned/optional list.",
                    ref=ref,
                )
            )
            continue
        if not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"{check_id}.missing_ref",
                    message=f"{label}[{idx}] does not exist: {ref}",
                    remediation="Fix the path (or seed the referenced artifact) so the checkpoint does not rely on missing prerequisites.",
                    ref=ref,
                )
            )
    return findings


def validate_optional_refs(repo_root: Path, refs: list[Any], check_id: str, label: str) -> list[Finding]:
    findings: list[Finding] = []
    for idx, ref in enumerate(refs):
        if not isinstance(ref, str) or not ref.strip():
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"{check_id}.invalid_ref",
                    message=f"{label}[{idx}] must be a non-empty string",
                    remediation="Replace the empty or non-string ref with a repo-relative path or a sentinel value.",
                )
            )
            continue
        ref = ref.strip()
        if ref in SENTINEL_REFS:
            continue
        if not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="warning",
                    check_id=f"{check_id}.missing_optional_ref",
                    message=f"{label}[{idx}] does not exist (optional): {ref}",
                    remediation="Either seed the referenced artifact or change it to an explicit sentinel until it is ready.",
                    ref=ref,
                )
            )
    return findings


def validate_build_identity_record(repo_root: Path, ref: str, check_id: str) -> list[Finding]:
    findings: list[Finding] = []
    rel = strip_fragment(ref)
    path = repo_root / rel
    try:
        payload = json.loads(path.read_text(encoding="utf-8"))
    except Exception as exc:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{check_id}.parse_failed",
                message=f"failed to parse build identity JSON at {rel}: {exc}",
                remediation="Regenerate the build identity record (or fix the referenced path) so consumers can read it mechanically.",
                ref=rel,
            )
        )
        return findings

    if not isinstance(payload, dict):
        findings.append(
            Finding(
                severity="error",
                check_id=f"{check_id}.not_object",
                message=f"build identity payload must be a JSON object: {rel}",
                remediation="Replace the payload with a JSON object matching schemas/build/build_identity.schema.json.",
                ref=rel,
            )
        )
        return findings

    keys = set(payload.keys())
    missing = sorted(BUILD_IDENTITY_REQUIRED_KEYS - keys)
    extra = sorted(keys - BUILD_IDENTITY_REQUIRED_KEYS)
    if missing:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{check_id}.missing_fields",
                message=f"build identity payload is missing required fields: {', '.join(missing)}",
                remediation="Regenerate the build identity record so it matches schemas/build/build_identity.schema.json.",
                ref=rel,
            )
        )
    if extra:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{check_id}.extra_fields",
                message=f"build identity payload has unknown fields: {', '.join(extra)}",
                remediation="Remove unknown fields (or bump the schema and update validators) so the artifact stays stable.",
                ref=rel,
            )
        )

    schema_version = payload.get("schema_version")
    if not isinstance(schema_version, int) or schema_version < 1:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{check_id}.schema_version",
                message=f"build identity schema_version must be an integer >= 1, got {schema_version!r}",
                remediation="Regenerate the artifact with tools/build/print_build_identity.sh or update the schema and validators together.",
                ref=rel,
            )
        )

    commit = payload.get("commit")
    if not isinstance(commit, str) or not COMMIT_FULL_RE.match(commit):
        findings.append(
            Finding(
                severity="error",
                check_id=f"{check_id}.commit",
                message=f"build identity commit must be a 40-hex hash or 'unknown', got {commit!r}",
                remediation="Regenerate the artifact so commit is a full hash (or 'unknown' outside git).",
                ref=rel,
            )
        )

    commit_short = payload.get("commit_short")
    if not isinstance(commit_short, str) or not commit_short.strip():
        findings.append(
            Finding(
                severity="error",
                check_id=f"{check_id}.commit_short",
                message="build identity commit_short must be a non-empty string",
                remediation="Regenerate the artifact so commit_short is available for human-readable logs.",
                ref=rel,
            )
        )

    dirty = payload.get("dirty")
    if not isinstance(dirty, bool):
        findings.append(
            Finding(
                severity="error",
                check_id=f"{check_id}.dirty",
                message=f"build identity dirty must be a boolean, got {dirty!r}",
                remediation="Regenerate the artifact so tree state is explicit.",
                ref=rel,
            )
        )

    profile = payload.get("profile")
    if profile not in {"dev", "release"}:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{check_id}.profile",
                message=f"build identity profile must be 'dev' or 'release', got {profile!r}",
                remediation="Regenerate the artifact so profile is normalized to the schema vocabulary.",
                ref=rel,
            )
        )

    source_date_epoch = payload.get("source_date_epoch")
    if not isinstance(source_date_epoch, int) or source_date_epoch < 0:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{check_id}.source_date_epoch",
                message=f"build identity source_date_epoch must be an integer >= 0, got {source_date_epoch!r}",
                remediation="Regenerate the artifact so SOURCE_DATE_EPOCH is recorded deterministically.",
                ref=rel,
            )
        )

    build_timestamp_utc = payload.get("build_timestamp_utc")
    if not isinstance(build_timestamp_utc, str) or not build_timestamp_utc.strip():
        findings.append(
            Finding(
                severity="error",
                check_id=f"{check_id}.build_timestamp_utc",
                message="build identity build_timestamp_utc must be a non-empty string",
                remediation="Regenerate the artifact so a deterministic build timestamp is recorded.",
                ref=rel,
            )
        )

    return findings


def validate_graph(repo_root: Path, graph: dict[str, Any], graph_rel: str) -> list[Finding]:
    findings: list[Finding] = []

    schema_version = ensure_int(graph.get("schema_version"), "graph.schema_version")
    if schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="graph.schema_version.unsupported",
                message=f"unsupported schema_version {schema_version} (expected 1)",
                remediation="Bump the validator and document the new schema before changing schema_version.",
            )
        )

    as_of = ensure_str(graph.get("as_of"), "graph.as_of")
    parse_iso_date(as_of, "graph.as_of")

    owner = ensure_str(graph.get("owner"), "graph.owner")
    if not owner.startswith("@"):
        findings.append(
            Finding(
                severity="warning",
                check_id="graph.owner.format",
                message=f"owner does not look like a handle: {owner!r}",
                remediation="Use an @handle so reviews can route directly to the DRI.",
            )
        )

    checkpoint = ensure_dict(graph.get("checkpoint"), "graph.checkpoint")
    human_entry = ensure_str(checkpoint.get("human_entrypoint_ref"), "graph.checkpoint.human_entrypoint_ref")
    index_ref = ensure_str(checkpoint.get("artifact_index_ref"), "graph.checkpoint.artifact_index_ref")
    validator = ensure_dict(checkpoint.get("validator"), "graph.checkpoint.validator")
    validator_script = ensure_str(validator.get("script_ref"), "graph.checkpoint.validator.script_ref")
    ensure_str(validator.get("command"), "graph.checkpoint.validator.command")

    for required_ref, label in [
        (human_entry, "graph.checkpoint.human_entrypoint_ref"),
        (index_ref, "graph.checkpoint.artifact_index_ref"),
        (validator_script, "graph.checkpoint.validator.script_ref"),
    ]:
        if not artifact_ref_exists(repo_root, required_ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="graph.checkpoint.missing_ref",
                    message=f"{label} does not exist: {required_ref}",
                    remediation="Fix the path or add the referenced artifact so reviewers have a stable entry point.",
                    ref=required_ref,
                )
            )

    if strip_fragment(index_ref) != DEFAULT_INDEX_REL:
        findings.append(
            Finding(
                severity="warning",
                check_id="graph.checkpoint.index_path.unexpected",
                message=f"artifact_index_ref is not the expected default path: {index_ref}",
                remediation=f"Prefer {DEFAULT_INDEX_REL} so downstream automation can find the proof index reliably.",
                ref=index_ref,
            )
        )

    flow = ensure_dict(graph.get("flow"), "graph.flow")
    steps = ensure_list(flow.get("steps"), "graph.flow.steps")
    step_ids: list[str] = []
    for idx, raw_step in enumerate(steps):
        step = ensure_dict(raw_step, f"graph.flow.steps[{idx}]")
        step_id = ensure_str(step.get("step_id"), f"graph.flow.steps[{idx}].step_id")
        step_ids.append(step_id)
        ensure_str(step.get("title"), f"graph.flow.steps[{idx}].title")

        required_subsystems = ensure_list(step.get("required_subsystems"), f"graph.flow.steps[{idx}].required_subsystems")
        required_subsystem_ids: list[str] = []
        for jdx, subsystem_id in enumerate(required_subsystems):
            if not isinstance(subsystem_id, str) or not subsystem_id.strip():
                findings.append(
                    Finding(
                        severity="error",
                        check_id="graph.flow.step.invalid_subsystem",
                        message=f"graph.flow.steps[{idx}].required_subsystems[{jdx}] must be a non-empty string",
                        remediation="List subsystem ids as strings that match entries under graph.subsystems[].subsystem_id.",
                    )
                )
                continue
            required_subsystem_ids.append(subsystem_id.strip())

        blocking = ensure_list(step.get("blocking_interfaces"), f"graph.flow.steps[{idx}].blocking_interfaces")
        findings.extend(
            validate_required_refs(
                repo_root,
                blocking,
                check_id="graph.flow.step.blocking_interfaces",
                label=f"graph.flow.steps[{idx}].blocking_interfaces",
            )
        )

        proof_lanes = ensure_list(step.get("required_proof_lanes"), f"graph.flow.steps[{idx}].required_proof_lanes")
        findings.extend(
            validate_required_refs(
                repo_root,
                proof_lanes,
                check_id="graph.flow.step.required_proof_lanes",
                label=f"graph.flow.steps[{idx}].required_proof_lanes",
            )
        )

    missing_steps = [step for step in REQUIRED_FLOW_STEPS if step not in step_ids]
    if missing_steps:
        findings.append(
            Finding(
                severity="error",
                check_id="graph.flow.steps.missing",
                message=f"flow is missing required steps: {', '.join(missing_steps)}",
                remediation="Restore the missing step ids so the integrated path stays explicit.",
                details={"required_step_ids": REQUIRED_FLOW_STEPS, "actual_step_ids": step_ids},
            )
        )
    else:
        indices = [step_ids.index(step) for step in REQUIRED_FLOW_STEPS]
        if indices != sorted(indices):
            findings.append(
                Finding(
                    severity="error",
                    check_id="graph.flow.steps.order",
                    message="required step ids are present but not in the expected order",
                    remediation="Reorder steps so Start Center -> open repo -> edit -> save -> restore session is readable top-to-bottom.",
                    details={"required_step_ids": REQUIRED_FLOW_STEPS, "actual_step_ids": step_ids},
                )
            )

    subsystems = ensure_list(graph.get("subsystems"), "graph.subsystems")
    subsystem_by_id: dict[str, dict[str, Any]] = {}
    for idx, raw_subsystem in enumerate(subsystems):
        subsystem = ensure_dict(raw_subsystem, f"graph.subsystems[{idx}]")
        subsystem_id = ensure_str(subsystem.get("subsystem_id"), f"graph.subsystems[{idx}].subsystem_id")
        subsystem_by_id[subsystem_id] = subsystem

        ensure_str(subsystem.get("title"), f"graph.subsystems[{idx}].title")
        ensure_str(subsystem.get("owner_dri"), f"graph.subsystems[{idx}].owner_dri")

        ownership_refs = ensure_list(subsystem.get("ownership_refs"), f"graph.subsystems[{idx}].ownership_refs")
        findings.extend(
            validate_required_refs(
                repo_root,
                ownership_refs,
                check_id="graph.subsystems.ownership_refs",
                label=f"graph.subsystems[{idx}].ownership_refs",
            )
        )

        contract_refs = ensure_list(subsystem.get("required_contract_refs"), f"graph.subsystems[{idx}].required_contract_refs")
        findings.extend(
            validate_required_refs(
                repo_root,
                contract_refs,
                check_id="graph.subsystems.required_contract_refs",
                label=f"graph.subsystems[{idx}].required_contract_refs",
            )
        )

    missing_subsystems = sorted(REQUIRED_SUBSYSTEMS.difference(subsystem_by_id.keys()))
    if missing_subsystems:
        findings.append(
            Finding(
                severity="error",
                check_id="graph.subsystems.missing",
                message=f"missing required subsystems: {', '.join(missing_subsystems)}",
                remediation="Add the missing subsystem rows so ownership and contract boundaries stay explicit.",
                details={"required_subsystems": sorted(REQUIRED_SUBSYSTEMS), "present_subsystems": sorted(subsystem_by_id.keys())},
            )
        )

    graph_section = ensure_dict(graph.get("graph"), "graph.graph")
    nodes = ensure_list(graph_section.get("nodes"), "graph.graph.nodes")
    node_ids: set[str] = set()
    for idx, raw_node in enumerate(nodes):
        node = ensure_dict(raw_node, f"graph.graph.nodes[{idx}]")
        node_id = ensure_str(node.get("node_id"), f"graph.graph.nodes[{idx}].node_id")
        node_ids.add(node_id)
        ensure_str(node.get("kind"), f"graph.graph.nodes[{idx}].kind")
        required = node.get("required")
        if not isinstance(required, bool):
            raise SystemExit(f"graph.graph.nodes[{idx}].required must be a boolean")
        artifact_ref = ensure_str(node.get("artifact_ref"), f"graph.graph.nodes[{idx}].artifact_ref")
        if required and not artifact_ref_exists(repo_root, artifact_ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="graph.graph.node.required_missing",
                    message=f"required node artifact_ref does not exist: {artifact_ref}",
                    remediation="Fix the node artifact_ref so the graph roots remain real files.",
                    ref=artifact_ref,
                )
            )

    edges = ensure_list(graph_section.get("edges"), "graph.graph.edges")
    for idx, raw_edge in enumerate(edges):
        edge = ensure_dict(raw_edge, f"graph.graph.edges[{idx}]")
        from_id = ensure_str(edge.get("from"), f"graph.graph.edges[{idx}].from")
        to_id = ensure_str(edge.get("to"), f"graph.graph.edges[{idx}].to")
        ensure_str(edge.get("rationale"), f"graph.graph.edges[{idx}].rationale")
        if from_id not in node_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="graph.graph.edge.unknown_from",
                    message=f"edge.from references unknown node_id: {from_id}",
                    remediation="Fix the edge to point at a node declared in graph.graph.nodes.",
                )
            )
        if to_id not in node_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="graph.graph.edge.unknown_to",
                    message=f"edge.to references unknown node_id: {to_id}",
                    remediation="Fix the edge to point at a node declared in graph.graph.nodes.",
                )
            )

    proof_assets = ensure_dict(graph.get("proof_assets"), "graph.proof_assets")
    required_assets = ensure_list(proof_assets.get("required"), "graph.proof_assets.required")
    for idx, raw_asset in enumerate(required_assets):
        asset = ensure_dict(raw_asset, f"graph.proof_assets.required[{idx}]")
        ensure_str(asset.get("asset_id"), f"graph.proof_assets.required[{idx}].asset_id")
        artifact_ref = ensure_str(asset.get("artifact_ref"), f"graph.proof_assets.required[{idx}].artifact_ref")
        if not artifact_ref_exists(repo_root, artifact_ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="graph.proof_assets.required_missing",
                    message=f"required proof asset does not exist: {artifact_ref}",
                    remediation="Seed the proof asset or adjust the required proof set so validation-lane coverage remains real.",
                    ref=artifact_ref,
                )
            )

    planned_assets = ensure_list(proof_assets.get("planned"), "graph.proof_assets.planned")
    for idx, raw_asset in enumerate(planned_assets):
        asset = ensure_dict(raw_asset, f"graph.proof_assets.planned[{idx}]")
        ensure_str(asset.get("asset_id"), f"graph.proof_assets.planned[{idx}].asset_id")
        artifact_ref = ensure_str(asset.get("artifact_ref"), f"graph.proof_assets.planned[{idx}].artifact_ref")
        findings.extend(
            validate_optional_refs(
                repo_root,
                [artifact_ref],
                check_id="graph.proof_assets.planned",
                label=f"graph.proof_assets.planned[{idx}].artifact_ref",
            )
        )

    # Ensure the graph's self-ref points at the on-disk file.
    if not (repo_root / graph_rel).exists():
        findings.append(
            Finding(
                severity="error",
                check_id="graph.file.missing",
                message=f"graph file does not exist on disk: {graph_rel}",
                remediation="Restore the dependency graph file.",
                ref=graph_rel,
            )
        )

    return findings


def validate_index(repo_root: Path, index: dict[str, Any], graph_rel: str, index_rel: str) -> list[Finding]:
    findings: list[Finding] = []

    schema_version = ensure_int(index.get("schema_version"), "index.schema_version")
    if schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="index.schema_version.unsupported",
                message=f"unsupported schema_version {schema_version} (expected 1)",
                remediation="Bump the validator and document the new schema before changing schema_version.",
            )
        )

    as_of = ensure_str(index.get("as_of"), "index.as_of")
    parse_iso_date(as_of, "index.as_of")

    owner = ensure_str(index.get("owner"), "index.owner")
    if not owner.startswith("@"):
        findings.append(
            Finding(
                severity="warning",
                check_id="index.owner.format",
                message=f"owner does not look like a handle: {owner!r}",
                remediation="Use an @handle so review routing is explicit.",
            )
        )

    human_entry = ensure_str(index.get("human_entrypoint_ref"), "index.human_entrypoint_ref")
    if not artifact_ref_exists(repo_root, human_entry):
        findings.append(
            Finding(
                severity="error",
                check_id="index.human_entrypoint.missing",
                message=f"human_entrypoint_ref does not exist: {human_entry}",
                remediation="Fix the path so reviewers have a stable landing page.",
                ref=human_entry,
            )
        )

    canonical_artifacts = ensure_list(index.get("canonical_artifacts"), "index.canonical_artifacts")
    dep_row: dict[str, Any] | None = None
    validator_row: dict[str, Any] | None = None
    for idx, raw_row in enumerate(canonical_artifacts):
        row = ensure_dict(raw_row, f"index.canonical_artifacts[{idx}]")
        artifact_id = ensure_str(row.get("artifact_id"), f"index.canonical_artifacts[{idx}].artifact_id")
        artifact_ref = ensure_str(row.get("artifact_ref"), f"index.canonical_artifacts[{idx}].artifact_ref")
        ensure_str(row.get("title"), f"index.canonical_artifacts[{idx}].title")
        ensure_str(row.get("owner_dri"), f"index.canonical_artifacts[{idx}].owner_dri")
        if artifact_id == "dependency_graph":
            dep_row = row
            if strip_fragment(artifact_ref) != graph_rel:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="index.dependency_graph.mismatch",
                        message=f"dependency_graph artifact_ref must be {graph_rel}, got {artifact_ref}",
                        remediation="Update artifact_index.yaml so consumers always point at the canonical dependency graph path.",
                        ref=artifact_ref,
                    )
                )
        if artifact_id == "validator":
            validator_row = row
        if artifact_id in {"dependency_graph", "validator", "fixture_lane"} and not artifact_ref_exists(repo_root, artifact_ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.canonical_artifact.missing",
                    message=f"canonical_artifacts[{idx}] artifact_ref does not exist: {artifact_ref}",
                    remediation="Fix the path or seed the artifact so the proof index remains a reliable join point.",
                    ref=artifact_ref,
                )
            )

    if dep_row is None:
        findings.append(
            Finding(
                severity="error",
                check_id="index.dependency_graph.missing_row",
                message="canonical_artifacts must include artifact_id: dependency_graph",
                remediation="Add a dependency_graph row pointing at the canonical dependency graph artifact.",
            )
        )
    if validator_row is None:
        findings.append(
            Finding(
                severity="error",
                check_id="index.validator.missing_row",
                message="canonical_artifacts must include artifact_id: validator",
                remediation="Add a validator row pointing at the checkpoint validation script.",
            )
        )

    validation_lane = ensure_dict(index.get("validation_lane"), "index.validation_lane")
    fixture_lane_ref = ensure_str(validation_lane.get("fixture_lane_ref"), "index.validation_lane.fixture_lane_ref")
    if not artifact_ref_exists(repo_root, fixture_lane_ref):
        findings.append(
            Finding(
                severity="error",
                check_id="index.validation_lane.fixture_lane_ref.missing",
                message=f"fixture_lane_ref does not exist: {fixture_lane_ref}",
                remediation="Fix the fixture-lane ref so the validation lane is explicit and runnable.",
                ref=fixture_lane_ref,
            )
        )

    exact_build = ensure_dict(index.get("exact_build_identity"), "index.exact_build_identity")
    contract_ref = ensure_str(exact_build.get("contract_ref"), "index.exact_build_identity.contract_ref")
    baseline_ref = ensure_str(exact_build.get("baseline_ref"), "index.exact_build_identity.baseline_ref")
    findings.extend(
        validate_required_refs(
            repo_root,
            [contract_ref, baseline_ref],
            check_id="index.exact_build_identity",
            label="index.exact_build_identity",
        )
    )
    latest_identity_ref = ensure_str(exact_build.get("latest_identity_ref"), "index.exact_build_identity.latest_identity_ref")
    findings.extend(
        validate_required_refs(
            repo_root,
            [latest_identity_ref],
            check_id="index.exact_build_identity.latest",
            label="index.exact_build_identity.latest_identity_ref",
        )
    )
    if latest_identity_ref not in SENTINEL_REFS and artifact_ref_exists(repo_root, latest_identity_ref):
        findings.extend(
            validate_build_identity_record(
                repo_root,
                latest_identity_ref,
                check_id="index.exact_build_identity.latest_identity_ref",
            )
        )

    return findings


def write_report(repo_root: Path, report_rel: str, graph_rel: str, index_rel: str, findings: list[Finding]) -> None:
    report_path = repo_root / report_rel
    report_path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": 1,
        "check_id": "m1_shell_editor_workspace_checkpoint",
        "graph_ref": graph_rel,
        "index_ref": index_rel,
        "generated_at": dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
        "finding_counts": {
            "error": sum(1 for f in findings if f.severity == "error"),
            "warning": sum(1 for f in findings if f.severity == "warning"),
        },
        "findings": [f.as_report() for f in findings],
    }
    report_path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    if not (repo_root / ".git").exists():
        raise SystemExit(f"--repo-root does not look like a repository root: {repo_root}")

    graph_rel = str(args.graph)
    index_rel = str(args.index)

    graph_path = repo_root / graph_rel
    index_path = repo_root / index_rel

    graph_payload = ensure_dict(render_yaml_as_json(graph_path), graph_rel)
    index_payload = ensure_dict(render_yaml_as_json(index_path), index_rel)

    findings: list[Finding] = []
    findings.extend(validate_graph(repo_root, graph_payload, graph_rel=graph_rel))
    findings.extend(validate_index(repo_root, index_payload, graph_rel=graph_rel, index_rel=index_rel))

    checkpoint = ensure_dict(graph_payload.get("checkpoint"), "graph.checkpoint")
    expected_index_ref = ensure_str(checkpoint.get("artifact_index_ref"), "graph.checkpoint.artifact_index_ref")
    if strip_fragment(expected_index_ref) != index_rel:
        findings.append(
            Finding(
                severity="error",
                check_id="cross.index_ref.mismatch",
                message=f"dependency graph checkpoint.artifact_index_ref must be {index_rel}, got {expected_index_ref}",
                remediation="Update dependency_graph.yaml so the proof index path is stable and matches the checked-in consumer.",
                ref=expected_index_ref,
            )
        )

    human_entry_graph = ensure_str(checkpoint.get("human_entrypoint_ref"), "graph.checkpoint.human_entrypoint_ref")

    canonical_artifacts = ensure_list(index_payload.get("canonical_artifacts"), "index.canonical_artifacts")
    refs_by_id: dict[str, str] = {}
    for idx, raw_row in enumerate(canonical_artifacts):
        row = ensure_dict(raw_row, f"index.canonical_artifacts[{idx}]")
        artifact_id = ensure_str(row.get("artifact_id"), f"index.canonical_artifacts[{idx}].artifact_id")
        artifact_ref = ensure_str(row.get("artifact_ref"), f"index.canonical_artifacts[{idx}].artifact_ref")
        refs_by_id[artifact_id] = artifact_ref

    index_entrypoint = refs_by_id.get("integration_checkpoint_entrypoint")
    if index_entrypoint is None:
        findings.append(
            Finding(
                severity="error",
                check_id="cross.index_missing_checkpoint_entrypoint",
                message="artifact_index.yaml canonical_artifacts is missing artifact_id: integration_checkpoint_entrypoint",
                remediation=(
                    "Add the canonical_artifacts row so consumers can resolve the checkpoint entrypoint "
                    "without hardcoding paths."
                ),
            )
        )
    elif strip_fragment(index_entrypoint) != strip_fragment(human_entry_graph):
        findings.append(
            Finding(
                severity="error",
                check_id="cross.checkpoint_entrypoint.mismatch",
                message=(
                    "dependency graph and artifact index disagree on the checkpoint entrypoint: "
                    f"{human_entry_graph} vs {index_entrypoint}"
                ),
                remediation="Update both files so the checkpoint entrypoint is stable and indexed.",
                details={
                    "graph_human_entrypoint_ref": human_entry_graph,
                    "index_checkpoint_entrypoint_ref": index_entrypoint,
                },
            )
        )

    if args.report:
        write_report(repo_root, str(args.report), graph_rel=graph_rel, index_rel=index_rel, findings=findings)

    errors = [finding for finding in findings if finding.severity == "error"]
    warnings = [finding for finding in findings if finding.severity == "warning"]
    status = "PASS" if not errors else "FAIL"

    print(f"[m1-checkpoint] {status} ({len(errors)} errors, {len(warnings)} warnings)")
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        print(f"[m1-checkpoint] {prefix} {finding.check_id}: {finding.message}{ref_suffix}")
        print(f"[m1-checkpoint]   remediation: {finding.remediation}")
    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[m1-checkpoint] interrupted", file=sys.stderr)
        sys.exit(130)
