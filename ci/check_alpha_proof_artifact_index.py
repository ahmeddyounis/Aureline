#!/usr/bin/env python3
"""Validate the external alpha proof artifact index and truth workflow."""

from __future__ import annotations

import argparse
import datetime as dt
import glob
import json
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_INDEX_REL = "artifacts/milestones/m2/artifact_index.yaml"
DEFAULT_SCOREBOARD_REL = "artifacts/milestones/m2/exit_gate_scoreboard.yaml"
DEFAULT_SLO_REL = "artifacts/governance/evidence_freshness_slos.yaml"

REQUIRED_UPSTREAM_REFS = {
    "artifacts/milestones/m2/alpha_wedge_matrix.yaml",
    "artifacts/milestones/m2/exit_gate_scoreboard.yaml",
    "artifacts/milestones/m2/dependency_graph.yaml",
}

REQUIRED_TRUTH_CATEGORIES = {
    "docs_ref",
    "migration_ref",
    "help_truth_ref",
    "known_limits_ref",
    "support_export_ref",
}

REQUIRED_FRESHNESS_FIELDS = {
    "packet_id",
    "evidence_id",
    "captured_at",
    "stale_after",
    "source_revision",
    "trigger_revision",
    "channel_context",
    "deployment_context",
    "exact_build_identity_ref",
    "owner_dri",
    "claim_row_refs",
}

REQUIRED_ACCEPTANCE_STATES = {
    "registration_coverage",
    "review_packet_metadata",
    "same_change_truth",
}

TEMPLATE_REQUIRED_MARKERS = {
    "owner_dri",
    "freshness_date",
    "exact_build_identity_ref",
    "same_change_truth_refs",
    "docs_ref",
    "migration_ref",
    "help_truth_ref",
    "known_limits_ref",
}

WORKFLOW_REQUIRED_MARKERS = {
    "same-change-set",
    "artifacts/milestones/m2/artifact_index.yaml",
    "docs_ref",
    "migration_ref",
    "help_truth_ref",
    "known_limits_ref",
    "support_export_ref",
}

FRESHNESS_STATES = {
    "current",
    "needs_refresh",
    "stale",
    "planned_not_yet_seeded",
    "blocked_pending_packet",
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
    parser.add_argument("--index", default=DEFAULT_INDEX_REL)
    parser.add_argument("--scoreboard", default=DEFAULT_SCOREBOARD_REL)
    parser.add_argument("--slo-catalog", default=DEFAULT_SLO_REL)
    parser.add_argument("--report", default=None)
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
        raise SystemExit(f"failed to parse YAML at {path}: {stderr}")
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


def parse_iso_date(value: str, label: str) -> None:
    try:
        dt.date.fromisoformat(value)
    except ValueError as exc:
        raise SystemExit(f"{label} must be a YYYY-MM-DD date, got {value!r}") from exc


def parse_iso_datetime(value: str, label: str, findings: list[Finding], ref: str) -> None:
    try:
        dt.datetime.fromisoformat(value.replace("Z", "+00:00"))
    except ValueError:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.invalid_datetime",
                message=f"{label} must be an ISO-8601 timestamp, got {value!r}",
                remediation="Use an ISO-8601 timestamp with UTC Z preferred.",
                ref=ref,
            )
        )


def strip_fragment(ref: str) -> str:
    return ref.split("#", 1)[0].strip()


def artifact_ref_exists(repo_root: Path, ref: str) -> bool:
    clean = strip_fragment(ref)
    return bool(clean) and (repo_root / clean).exists()


def is_under_any_root(ref: str, roots: list[str]) -> bool:
    clean = strip_fragment(ref)
    for root in roots:
        normalized = root.rstrip("/")
        if clean == normalized or clean.startswith(normalized + "/"):
            return True
    return False


def validate_path_ref(repo_root: Path, ref: str, label: str, findings: list[Finding]) -> None:
    if not artifact_ref_exists(repo_root, ref):
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.missing_ref",
                message=f"{label} does not resolve: {ref}",
                remediation="Fix the path or seed the referenced artifact.",
                ref=ref,
            )
        )


def validate_header(index: dict[str, Any], findings: list[Finding]) -> None:
    schema_version = ensure_int(index.get("schema_version"), "index.schema_version")
    if schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="index.schema_version.unsupported",
                message=f"schema_version must be 1, got {schema_version}",
                remediation="Update the validator in the same change that changes the schema.",
            )
        )
    parse_iso_date(ensure_str(index.get("as_of"), "index.as_of"), "index.as_of")
    ensure_str(index.get("owner"), "index.owner")
    ensure_str(index.get("index_id"), "index.index_id")


def collect_scoreboard_ids(scoreboard: dict[str, Any]) -> set[str]:
    ids: set[str] = set()
    for idx, raw_row in enumerate(ensure_list(scoreboard.get("scoreboard_rows"), "scoreboard.scoreboard_rows")):
        row = ensure_dict(raw_row, f"scoreboard.scoreboard_rows[{idx}]")
        ids.add(ensure_str(row.get("row_id"), f"scoreboard.scoreboard_rows[{idx}].row_id"))
    return ids


def collect_proof_classes(slo_catalog: dict[str, Any]) -> set[str]:
    ids: set[str] = set()
    for idx, raw_row in enumerate(ensure_list(slo_catalog.get("proof_classes"), "slo_catalog.proof_classes")):
        row = ensure_dict(raw_row, f"slo_catalog.proof_classes[{idx}]")
        ids.add(ensure_str(row.get("proof_class_id"), f"slo_catalog.proof_classes[{idx}].proof_class_id"))
    return ids


def validate_primary_refs(repo_root: Path, index: dict[str, Any], findings: list[Finding]) -> None:
    for label in (
        "human_entrypoint_ref",
        "truth_update_workflow_ref",
        "matrix_ref",
        "scoreboard_ref",
        "dependency_graph_ref",
    ):
        validate_path_ref(repo_root, ensure_str(index.get(label), f"index.{label}"), f"index.{label}", findings)

    upstream_refs = set(ensure_list(index.get("upstream_scope_contract_refs"), "index.upstream_scope_contract_refs"))
    missing = REQUIRED_UPSTREAM_REFS - upstream_refs
    if missing:
        findings.append(
            Finding(
                severity="error",
                check_id="index.upstream_scope_contract_refs.missing_required",
                message="upstream_scope_contract_refs is missing required alpha scope contracts",
                remediation="Reference the existing matrix, scoreboard, and dependency graph directly.",
                details={"missing": sorted(missing)},
            )
        )
    for raw_ref in upstream_refs:
        validate_path_ref(repo_root, ensure_str(raw_ref, "index.upstream_scope_contract_refs[]"), "index.upstream_scope_contract_refs", findings)


def validate_freshness_policy(repo_root: Path, index: dict[str, Any], findings: list[Finding]) -> None:
    policy = ensure_dict(index.get("freshness_policy"), "index.freshness_policy")
    for label in (
        "policy_ref",
        "slo_catalog_ref",
        "rerun_trigger_catalog_ref",
        "metadata_catalog_ref",
        "shared_header_schema_ref",
    ):
        validate_path_ref(repo_root, ensure_str(policy.get(label), f"index.freshness_policy.{label}"), f"index.freshness_policy.{label}", findings)
    required_fields = set(ensure_list(policy.get("required_metadata_fields"), "index.freshness_policy.required_metadata_fields"))
    missing = REQUIRED_FRESHNESS_FIELDS - required_fields
    if missing:
        findings.append(
            Finding(
                severity="error",
                check_id="index.freshness_policy.required_metadata_fields.missing",
                message="freshness_policy.required_metadata_fields omits required metadata",
                remediation="Add the shared freshness, owner, exact-build, and claim-row fields.",
                details={"missing": sorted(missing)},
            )
        )


def validate_storage_roots(repo_root: Path, index: dict[str, Any], findings: list[Finding]) -> list[str]:
    roots: list[str] = []
    for idx, raw_row in enumerate(ensure_list(index.get("storage_roots"), "index.storage_roots")):
        row = ensure_dict(raw_row, f"index.storage_roots[{idx}]")
        ensure_str(row.get("root_id"), f"index.storage_roots[{idx}].root_id")
        root_ref = ensure_str(row.get("root_ref"), f"index.storage_roots[{idx}].root_ref")
        roots.append(root_ref)
        validate_path_ref(repo_root, root_ref, "index.storage_roots.root_ref", findings)
    return roots


def validate_canonical_artifacts(
    repo_root: Path,
    index: dict[str, Any],
    report_rel: str | None,
    findings: list[Finding],
) -> None:
    seen: set[str] = set()
    for idx, raw_row in enumerate(ensure_list(index.get("canonical_artifacts"), "index.canonical_artifacts")):
        row = ensure_dict(raw_row, f"index.canonical_artifacts[{idx}]")
        artifact_id = ensure_str(row.get("artifact_id"), f"index.canonical_artifacts[{idx}].artifact_id")
        if artifact_id in seen:
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.canonical_artifacts.duplicate",
                    message=f"duplicate canonical artifact id: {artifact_id}",
                    remediation="Use stable unique artifact ids.",
                    ref=artifact_id,
                )
            )
        seen.add(artifact_id)
        ensure_str(row.get("title"), f"index.canonical_artifacts[{idx}].title")
        ensure_str(row.get("owner_dri"), f"index.canonical_artifacts[{idx}].owner_dri")
        artifact_ref = ensure_str(row.get("artifact_ref"), f"index.canonical_artifacts[{idx}].artifact_ref")
        if report_rel is not None and strip_fragment(artifact_ref) == report_rel:
            continue
        validate_path_ref(repo_root, artifact_ref, "index.canonical_artifacts.artifact_ref", findings)


def validate_registration_coverage(repo_root: Path, index: dict[str, Any], findings: list[Finding]) -> list[str]:
    coverage = ensure_dict(index.get("registration_coverage"), "index.registration_coverage")
    plan_glob = ensure_str(coverage.get("plan_source_glob"), "index.registration_coverage.plan_source_glob")
    expected = sorted(str(Path(path).as_posix()) for path in glob.glob(str(repo_root / plan_glob)))
    expected_rel = sorted(str(Path(path).relative_to(repo_root).as_posix()) for path in expected)
    required_refs = sorted(
        ensure_str(item, "index.registration_coverage.required_plan_refs[]")
        for item in ensure_list(coverage.get("required_plan_refs"), "index.registration_coverage.required_plan_refs")
    )
    if required_refs != expected_rel:
        findings.append(
            Finding(
                severity="error",
                check_id="index.registration_coverage.plan_refs_mismatch",
                message="required_plan_refs must match the current alpha plan glob",
                remediation="Add newly introduced alpha plan refs to the index or remove stale refs.",
                details={"expected": expected_rel, "actual": required_refs},
            )
        )
    for ref in required_refs:
        validate_path_ref(repo_root, ref, "index.registration_coverage.required_plan_refs", findings)
    return required_refs


def validate_truth_surfaces(repo_root: Path, index: dict[str, Any], findings: list[Finding]) -> set[str]:
    categories: set[str] = set()
    for idx, raw_row in enumerate(ensure_list(index.get("claim_truth_surfaces"), "index.claim_truth_surfaces")):
        row = ensure_dict(raw_row, f"index.claim_truth_surfaces[{idx}]")
        ensure_str(row.get("surface_id"), f"index.claim_truth_surfaces[{idx}].surface_id")
        category = ensure_str(row.get("category"), f"index.claim_truth_surfaces[{idx}].category")
        categories.add(category)
        artifact_ref = ensure_str(row.get("artifact_ref"), f"index.claim_truth_surfaces[{idx}].artifact_ref")
        validate_path_ref(repo_root, artifact_ref, "index.claim_truth_surfaces.artifact_ref", findings)
        if not isinstance(row.get("required_for_claim_change"), bool):
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.claim_truth_surfaces.required_for_claim_change_not_bool",
                    message=f"claim truth surface must set required_for_claim_change as boolean: {category}",
                    remediation="Set required_for_claim_change to true or false.",
                    ref=artifact_ref,
                )
            )
    missing = REQUIRED_TRUTH_CATEGORIES - categories
    if missing:
        findings.append(
            Finding(
                severity="error",
                check_id="index.claim_truth_surfaces.missing_categories",
                message="claim_truth_surfaces is missing required same-change-set categories",
                remediation="Add docs, migration, help, known-limits, and support-export categories.",
                details={"missing": sorted(missing)},
            )
        )
    return categories


def validate_template_and_workflow(repo_root: Path, index: dict[str, Any], findings: list[Finding]) -> None:
    template_ref = ensure_str(index.get("human_entrypoint_ref"), "index.human_entrypoint_ref")
    workflow_ref = ensure_str(index.get("truth_update_workflow_ref"), "index.truth_update_workflow_ref")
    template_text = (repo_root / template_ref).read_text(encoding="utf-8") if artifact_ref_exists(repo_root, template_ref) else ""
    workflow_text = (repo_root / workflow_ref).read_text(encoding="utf-8") if artifact_ref_exists(repo_root, workflow_ref) else ""

    missing_template_markers = sorted(marker for marker in TEMPLATE_REQUIRED_MARKERS if marker not in template_text)
    if missing_template_markers:
        findings.append(
            Finding(
                severity="error",
                check_id="review_template.missing_required_markers",
                message="review packet template does not require all owner, freshness, build, and truth fields",
                remediation="Add the missing metadata fields to the packet header template.",
                ref=template_ref,
                details={"missing": missing_template_markers},
            )
        )

    missing_workflow_markers = sorted(marker for marker in WORKFLOW_REQUIRED_MARKERS if marker not in workflow_text)
    if missing_workflow_markers:
        findings.append(
            Finding(
                severity="error",
                check_id="truth_workflow.missing_required_markers",
                message="truth update workflow does not name all required same-change-set refs",
                remediation="Add the missing same-change-set fields and canonical index ref.",
                ref=workflow_ref,
                details={"missing": missing_workflow_markers},
            )
        )


def validate_proof_lanes(
    repo_root: Path,
    index: dict[str, Any],
    scoreboard_ids: set[str],
    proof_class_ids: set[str],
    required_plan_refs: list[str],
    storage_roots: list[str],
    report_rel: str | None,
    findings: list[Finding],
) -> list[str]:
    lanes = ensure_list(index.get("proof_lanes"), "index.proof_lanes")
    seen_lanes: set[str] = set()
    covered_plans: set[str] = set()
    lane_ids: list[str] = []
    for idx, raw_lane in enumerate(lanes):
        lane = ensure_dict(raw_lane, f"index.proof_lanes[{idx}]")
        lane_id = ensure_str(lane.get("lane_id"), f"index.proof_lanes[{idx}].lane_id")
        lane_ids.append(lane_id)
        if lane_id in seen_lanes:
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.proof_lanes.duplicate",
                    message=f"duplicate proof lane id: {lane_id}",
                    remediation="Use one stable proof lane row per lane.",
                    ref=lane_id,
                )
            )
        seen_lanes.add(lane_id)
        ensure_str(lane.get("title"), f"index.proof_lanes[{idx}].title")
        ensure_str(lane.get("owner_dri"), f"index.proof_lanes[{idx}].owner_dri")

        for plan_ref in ensure_list(lane.get("source_plan_refs"), f"index.proof_lanes[{idx}].source_plan_refs"):
            plan_ref = ensure_str(plan_ref, f"index.proof_lanes[{idx}].source_plan_refs[]")
            covered_plans.add(plan_ref)
            if plan_ref not in required_plan_refs:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="index.proof_lanes.source_plan_refs.unknown",
                        message=f"proof lane cites a plan ref outside registration_coverage: {plan_ref}",
                        remediation="Add the plan ref to registration_coverage or correct the lane ref.",
                        ref=lane_id,
                    )
                )
            validate_path_ref(repo_root, plan_ref, "index.proof_lanes.source_plan_refs", findings)

        for row_ref in ensure_list(lane.get("scoreboard_row_refs"), f"index.proof_lanes[{idx}].scoreboard_row_refs"):
            row_ref = ensure_str(row_ref, f"index.proof_lanes[{idx}].scoreboard_row_refs[]")
            if row_ref not in scoreboard_ids:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="index.proof_lanes.scoreboard_row_refs.unknown",
                        message=f"proof lane cites unknown scoreboard row: {row_ref}",
                        remediation="Add the scoreboard row or correct the index reference.",
                        ref=lane_id,
                    )
                )

        exact_build_ref = ensure_str(lane.get("exact_build_identity_ref"), f"index.proof_lanes[{idx}].exact_build_identity_ref")
        validate_path_ref(repo_root, exact_build_ref, "index.proof_lanes.exact_build_identity_ref", findings)

        for packet_ref in ensure_list(lane.get("proof_packet_refs"), f"index.proof_lanes[{idx}].proof_packet_refs"):
            packet_ref = ensure_str(packet_ref, f"index.proof_lanes[{idx}].proof_packet_refs[]")
            validate_path_ref(repo_root, packet_ref, "index.proof_lanes.proof_packet_refs", findings)
            if artifact_ref_exists(repo_root, packet_ref) and not is_under_any_root(packet_ref, storage_roots):
                findings.append(
                    Finding(
                        severity="error",
                        check_id="index.proof_lanes.proof_packet_refs.outside_roots",
                        message=f"proof packet ref is outside declared storage roots: {packet_ref}",
                        remediation="Move the proof packet under a declared storage root or add the root.",
                        ref=lane_id,
                    )
                )

        for evidence_ref in ensure_list(lane.get("evidence_refs"), f"index.proof_lanes[{idx}].evidence_refs"):
            evidence_ref = ensure_str(evidence_ref, f"index.proof_lanes[{idx}].evidence_refs[]")
            if report_rel is not None and strip_fragment(evidence_ref) == report_rel:
                continue
            validate_path_ref(repo_root, evidence_ref, "index.proof_lanes.evidence_refs", findings)

        freshness = ensure_dict(lane.get("freshness"), f"index.proof_lanes[{idx}].freshness")
        state = ensure_str(freshness.get("state"), f"index.proof_lanes[{idx}].freshness.state")
        if state not in FRESHNESS_STATES:
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.proof_lanes.freshness.invalid_state",
                    message=f"proof lane uses unknown freshness state: {state}",
                    remediation="Use a state from the artifact-index freshness vocabulary.",
                    ref=lane_id,
                )
            )
        proof_class_ref = ensure_str(freshness.get("proof_class_ref"), f"index.proof_lanes[{idx}].freshness.proof_class_ref")
        if proof_class_ref not in proof_class_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.proof_lanes.freshness.unknown_proof_class",
                    message=f"proof lane cites unknown proof class: {proof_class_ref}",
                    remediation="Use a proof class from artifacts/governance/evidence_freshness_slos.yaml.",
                    ref=lane_id,
                )
            )
        parse_iso_datetime(
            ensure_str(freshness.get("captured_at"), f"index.proof_lanes[{idx}].freshness.captured_at"),
            "index.proof_lanes.freshness.captured_at",
            findings,
            lane_id,
        )
        ensure_str(freshness.get("stale_after"), f"index.proof_lanes[{idx}].freshness.stale_after")
        ensure_str(freshness.get("source_revision"), f"index.proof_lanes[{idx}].freshness.source_revision")
        ensure_str(freshness.get("trigger_revision"), f"index.proof_lanes[{idx}].freshness.trigger_revision")
        ensure_str(freshness.get("channel_context"), f"index.proof_lanes[{idx}].freshness.channel_context")
        if not ensure_list(freshness.get("deployment_context"), f"index.proof_lanes[{idx}].freshness.deployment_context"):
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.proof_lanes.freshness.deployment_context.empty",
                    message="proof lane freshness deployment_context must not be empty",
                    remediation="Name the deployment context covered by the evidence.",
                    ref=lane_id,
                )
            )

        latest_capture = ensure_dict(lane.get("latest_capture"), f"index.proof_lanes[{idx}].latest_capture")
        parse_iso_datetime(
            ensure_str(latest_capture.get("captured_at"), f"index.proof_lanes[{idx}].latest_capture.captured_at"),
            "index.proof_lanes.latest_capture.captured_at",
            findings,
            lane_id,
        )
        ensure_str(latest_capture.get("command"), f"index.proof_lanes[{idx}].latest_capture.command")
        report_ref = ensure_str(latest_capture.get("report_ref"), f"index.proof_lanes[{idx}].latest_capture.report_ref")
        if report_rel is None or strip_fragment(report_ref) != report_rel:
            validate_path_ref(repo_root, report_ref, "index.proof_lanes.latest_capture.report_ref", findings)
        if artifact_ref_exists(repo_root, report_ref) and not is_under_any_root(report_ref, storage_roots):
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.proof_lanes.latest_capture.outside_roots",
                    message=f"latest capture is outside declared storage roots: {report_ref}",
                    remediation="Store validation captures under a declared storage root.",
                    ref=lane_id,
                )
            )

        truth_refs = ensure_dict(lane.get("same_change_truth_refs"), f"index.proof_lanes[{idx}].same_change_truth_refs")
        missing_truth = REQUIRED_TRUTH_CATEGORIES - set(truth_refs)
        if missing_truth:
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.proof_lanes.same_change_truth_refs.missing",
                    message="proof lane is missing required same-change truth refs",
                    remediation="Add docs, migration, help, known-limits, and support-export refs.",
                    ref=lane_id,
                    details={"missing": sorted(missing_truth)},
                )
            )
        for truth_key in REQUIRED_TRUTH_CATEGORIES | {"review_packet_ref"}:
            if truth_key in truth_refs:
                validate_path_ref(
                    repo_root,
                    ensure_str(truth_refs.get(truth_key), f"index.proof_lanes[{idx}].same_change_truth_refs.{truth_key}"),
                    f"index.proof_lanes.same_change_truth_refs.{truth_key}",
                    findings,
                )

    missing_plans = sorted(set(required_plan_refs) - covered_plans)
    if missing_plans:
        findings.append(
            Finding(
                severity="error",
                check_id="index.proof_lanes.plan_coverage_missing",
                message="one or more alpha plan refs are not covered by a proof lane",
                remediation="Add a proof_lanes row for each missing plan ref.",
                details={"missing": missing_plans},
            )
        )
    return sorted(lane_ids)


def validate_acceptance_coverage(repo_root: Path, index: dict[str, Any], findings: list[Finding]) -> None:
    states: set[str] = set()
    for idx, raw_row in enumerate(ensure_list(index.get("acceptance_state_coverage"), "index.acceptance_state_coverage")):
        row = ensure_dict(raw_row, f"index.acceptance_state_coverage[{idx}]")
        ensure_str(row.get("case_id"), f"index.acceptance_state_coverage[{idx}].case_id")
        states.add(ensure_str(row.get("exercises_state"), f"index.acceptance_state_coverage[{idx}].exercises_state"))
        validate_path_ref(
            repo_root,
            ensure_str(row.get("fixture_ref"), f"index.acceptance_state_coverage[{idx}].fixture_ref"),
            "index.acceptance_state_coverage.fixture_ref",
            findings,
        )
        ensure_str(row.get("expected_validator_result"), f"index.acceptance_state_coverage[{idx}].expected_validator_result")
    missing = REQUIRED_ACCEPTANCE_STATES - states
    if missing:
        findings.append(
            Finding(
                severity="error",
                check_id="index.acceptance_state_coverage.missing",
                message="acceptance_state_coverage does not exercise all required states",
                remediation="Add coverage for registration, review-packet metadata, and same-change truth refs.",
                details={"missing": sorted(missing)},
            )
        )


def write_report(path: Path, index_rel: str, lane_ids: list[str], plan_refs: list[str], findings: list[Finding]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": 1,
        "status": "pass" if not any(item.severity == "error" for item in findings) else "fail",
        "generated_at": dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
        "index_ref": index_rel,
        "summary": {
            "errors": sum(1 for item in findings if item.severity == "error"),
            "warnings": sum(1 for item in findings if item.severity == "warning"),
            "checked_lanes": lane_ids,
            "checked_plan_refs": plan_refs,
        },
        "findings": [item.as_report() for item in findings],
    }
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(f"--repo-root does not look like a repository root: {repo_root}")

    index_rel = str(args.index)
    report_rel = str(args.report) if args.report else None
    index = ensure_dict(render_yaml_as_json(repo_root / index_rel), "index")
    scoreboard = ensure_dict(render_yaml_as_json(repo_root / args.scoreboard), "scoreboard")
    slo_catalog = ensure_dict(render_yaml_as_json(repo_root / args.slo_catalog), "slo_catalog")

    findings: list[Finding] = []
    validate_header(index, findings)
    validate_primary_refs(repo_root, index, findings)
    validate_freshness_policy(repo_root, index, findings)
    storage_roots = validate_storage_roots(repo_root, index, findings)
    validate_canonical_artifacts(repo_root, index, report_rel, findings)
    required_plan_refs = validate_registration_coverage(repo_root, index, findings)
    validate_truth_surfaces(repo_root, index, findings)
    validate_template_and_workflow(repo_root, index, findings)
    validate_acceptance_coverage(repo_root, index, findings)
    lane_ids = validate_proof_lanes(
        repo_root=repo_root,
        index=index,
        scoreboard_ids=collect_scoreboard_ids(scoreboard),
        proof_class_ids=collect_proof_classes(slo_catalog),
        required_plan_refs=required_plan_refs,
        storage_roots=storage_roots,
        report_rel=report_rel,
        findings=findings,
    )

    if report_rel:
        write_report(repo_root / report_rel, index_rel, lane_ids, required_plan_refs, findings)

    errors = [finding for finding in findings if finding.severity == "error"]
    warnings = [finding for finding in findings if finding.severity == "warning"]
    status = "PASS" if not errors else "FAIL"
    print(f"[alpha-proof-artifact-index] {status} ({len(errors)} errors, {len(warnings)} warnings)")
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        print(f"[alpha-proof-artifact-index] {prefix} {finding.check_id}: {finding.message}{ref_suffix}")
        print(f"[alpha-proof-artifact-index]   remediation: {finding.remediation}")
    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[alpha-proof-artifact-index] interrupted", file=sys.stderr)
        sys.exit(130)
