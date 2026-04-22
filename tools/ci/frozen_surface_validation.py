#!/usr/bin/env python3
"""Validate Aureline's frozen-surface change-control manifest."""

from __future__ import annotations

import datetime as dt
import json
import os
import subprocess
from collections import Counter
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any

FROZEN_SURFACE_MANIFEST_REL = "artifacts/contracts/frozen_surface_manifest.yaml"
FROZEN_SURFACE_POLICY_REL = "docs/governance/frozen_surface_ci_policy.md"
FROZEN_SURFACE_TOOL_REL = "tools/check_frozen_surfaces.py"
FROZEN_SURFACE_SCENARIO_REL = "fixtures/ci/contract_validation/missing_frozen_surface_metadata.json"

OWNERSHIP_MATRIX_REL = "artifacts/governance/ownership_matrix.yaml"
STABLE_SURFACE_INVENTORY_REL = "artifacts/governance/stable_surface_inventory.yaml"
INTERFACE_FREEZE_MATRIX_REL = "artifacts/governance/interface_freeze_matrix.yaml"

REQUIRED_SURFACE_IDS = {
    "command_plane.command_descriptor_and_invocation_session",
    "docs.docs_pack_manifest",
    "build.exact_build_identity_fields",
    "runtime.action_origin_target_route_taxonomy",
    "support.object_issue_handoff_packet",
    "repo.protected_path_dependency_rules",
}

IGNORED_CHANGED_FILE_PREFIXES = ("target/",)
IGNORED_CHANGED_FILE_PARTS = ("__pycache__",)
IGNORED_CHANGED_FILE_SUFFIXES = (".pyc",)


@dataclass
class FrozenSurfaceFinding:
    severity: str
    check_id: str
    artifact_ref: str
    owner_artifact_ref: str
    message: str
    remediation: str
    row_ref: str | None = None
    details: dict[str, Any] = field(default_factory=dict)

    def as_report(self) -> dict[str, Any]:
        payload = asdict(self)
        if not payload["details"]:
            payload.pop("details")
        if payload["row_ref"] is None:
            payload.pop("row_ref")
        return payload


def now_utc() -> str:
    return dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def load_json(path: Path) -> Any:
    with path.open("rb") as fh:
        return json.load(fh)


def render_yaml_as_json(path: Path) -> Any:
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


def strip_path_annotations(ref: str) -> str:
    path = ref
    for separator in ("#", " §", " line ", " @"):
        if separator in path:
            path = path.split(separator, 1)[0]
    return path.strip()


def register_duplicates(values: list[str]) -> list[str]:
    return sorted([value for value, count in Counter(values).items() if count > 1])


def item_ref(row_id: str) -> str:
    return f"{FROZEN_SURFACE_MANIFEST_REL}#{row_id}"


def make_finding(
    severity: str,
    check_id: str,
    artifact_ref: str,
    owner_artifact_ref: str,
    message: str,
    remediation: str,
    row_ref: str | None = None,
    **details: Any,
) -> FrozenSurfaceFinding:
    return FrozenSurfaceFinding(
        severity=severity,
        check_id=check_id,
        artifact_ref=artifact_ref,
        owner_artifact_ref=owner_artifact_ref,
        message=message,
        remediation=remediation,
        row_ref=row_ref,
        details=details,
    )


def existing_path_ref(repo_root: Path, ref: str) -> bool:
    return (repo_root / strip_path_annotations(ref)).exists()


def path_matches(repo_root: Path, changed_file: str, ref: str) -> bool:
    candidate = Path(changed_file).as_posix()
    normalized_ref = strip_path_annotations(ref).rstrip("/")
    if not normalized_ref:
        return False
    ref_path = repo_root / normalized_ref
    if ref.endswith("/") or ref_path.is_dir():
        return candidate == normalized_ref or candidate.startswith(f"{normalized_ref}/")
    return candidate == normalized_ref


def load_scenario(repo_root: Path, scenario: Path | dict[str, Any] | None) -> dict[str, Any] | None:
    if scenario is None:
        return None
    if isinstance(scenario, dict):
        return scenario
    path = scenario if scenario.is_absolute() else repo_root / scenario
    if not path.exists():
        raise SystemExit(f"scenario file does not exist: {path}")
    payload = load_json(path)
    if not isinstance(payload, dict):
        raise SystemExit(f"scenario file must contain a JSON object: {path}")
    changed_files = payload.get("changed_files")
    if changed_files is not None and (
        not isinstance(changed_files, list) or not all(isinstance(item, str) and item for item in changed_files)
    ):
        raise SystemExit(f"scenario 'changed_files' must be a list of non-empty strings: {path}")
    payload["_path"] = str(path.relative_to(repo_root))
    return payload


def run_git_lines(repo_root: Path, args: list[str]) -> list[str]:
    proc = subprocess.run(
        ["git", "-C", str(repo_root), *args],
        capture_output=True,
        text=True,
    )
    if proc.returncode != 0:
        return []
    return sorted({line.strip() for line in proc.stdout.splitlines() if line.strip()})


def filter_changed_files(paths: list[str]) -> list[str]:
    filtered: set[str] = set()
    for raw_path in paths:
        path = Path(raw_path).as_posix()
        if path.startswith(IGNORED_CHANGED_FILE_PREFIXES):
            continue
        if path.endswith(IGNORED_CHANGED_FILE_SUFFIXES):
            continue
        if any(part in IGNORED_CHANGED_FILE_PARTS for part in Path(path).parts):
            continue
        filtered.add(path)
    return sorted(filtered)


def detect_changed_files(repo_root: Path, scenario: dict[str, Any] | None) -> list[str]:
    if scenario is not None and scenario.get("changed_files"):
        return filter_changed_files([Path(path).as_posix() for path in scenario["changed_files"]])

    changed: set[str] = set()
    for args in (
        ["diff", "--name-only", "--relative", "--"],
        ["diff", "--name-only", "--relative", "--cached", "--"],
        ["ls-files", "--others", "--exclude-standard"],
    ):
        changed.update(run_git_lines(repo_root, args))

    if changed:
        return filter_changed_files(sorted(changed))

    if os.getenv("GITHUB_ACTIONS") != "true" and os.getenv("CI") != "true":
        return []

    head_parent = subprocess.run(
        ["git", "-C", str(repo_root), "rev-parse", "--verify", "HEAD^1"],
        capture_output=True,
        text=True,
    )
    if head_parent.returncode != 0:
        return []

    changed.update(run_git_lines(repo_root, ["diff", "--name-only", "--relative", "HEAD^1", "HEAD", "--"]))
    return filter_changed_files(sorted(changed))


def validate_frozen_surface_manifest(
    repo_root: Path,
    scenario: Path | dict[str, Any] | None = None,
) -> tuple[list[FrozenSurfaceFinding], dict[str, Any]]:
    findings: list[FrozenSurfaceFinding] = []
    scenario_payload = load_scenario(repo_root, scenario)
    changed_files = detect_changed_files(repo_root, scenario_payload)

    manifest = render_yaml_as_json(repo_root / FROZEN_SURFACE_MANIFEST_REL)
    ownership = render_yaml_as_json(repo_root / OWNERSHIP_MATRIX_REL)
    stable_surface_inventory = render_yaml_as_json(repo_root / STABLE_SURFACE_INVENTORY_REL)
    freeze_matrix = render_yaml_as_json(repo_root / INTERFACE_FREEZE_MATRIX_REL)

    rows = manifest.get("rows", [])
    if not isinstance(rows, list):
        raise SystemExit(f"{FROZEN_SURFACE_MANIFEST_REL} must define a top-level rows list")

    lanes = {row["id"] for row in ownership.get("governance_lanes", [])}
    stable_surface_ids = {row["surface_id"] for row in stable_surface_inventory.get("rows", [])}
    freeze_row_ids = {row["row_id"] for row in freeze_matrix.get("rows", [])}
    obligation_classes = set(manifest.get("obligation_class_values", []))
    freeze_status_values = set(manifest.get("freeze_status_values", []))

    for field_name in ("manifest_id", "title", "overview_ref", "control_artifact_ref"):
        if not manifest.get(field_name):
            findings.append(
                make_finding(
                    "error",
                    f"frozen_surface_manifest.required_field.{field_name}",
                    FROZEN_SURFACE_MANIFEST_REL,
                    FROZEN_SURFACE_MANIFEST_REL,
                    f"frozen-surface manifest is missing required field '{field_name}'",
                    "Populate the missing manifest metadata so the change-control lane has a stable home and overview.",
                )
            )

    if not existing_path_ref(repo_root, manifest.get("overview_ref", "")):
        findings.append(
            make_finding(
                "error",
                "frozen_surface_manifest.overview_ref_exists",
                FROZEN_SURFACE_MANIFEST_REL,
                FROZEN_SURFACE_MANIFEST_REL,
                f"frozen-surface manifest overview_ref is missing: {manifest.get('overview_ref')}",
                "Point overview_ref at the narrative CI policy document.",
            )
        )

    for ref in manifest.get("validator_entrypoint_refs", []):
        if not existing_path_ref(repo_root, ref):
            findings.append(
                make_finding(
                    "error",
                    "frozen_surface_manifest.validator_entrypoints_exist",
                    FROZEN_SURFACE_MANIFEST_REL,
                    FROZEN_SURFACE_MANIFEST_REL,
                    f"frozen-surface manifest references missing validator entrypoint '{ref}'",
                    "Keep each validator entrypoint path real so the manifest does not outlive its enforcing tool.",
                )
            )

    row_ids = [row.get("surface_id") for row in rows if isinstance(row, dict) and row.get("surface_id")]
    for duplicate in register_duplicates(row_ids):
        findings.append(
            make_finding(
                "error",
                "frozen_surface_manifest.unique_surface_ids",
                FROZEN_SURFACE_MANIFEST_REL,
                FROZEN_SURFACE_MANIFEST_REL,
                f"frozen-surface manifest duplicates surface_id '{duplicate}'",
                "Keep each frozen-surface row keyed by one unique surface_id.",
                row_ref=duplicate,
            )
        )

    present_surface_ids = set(row_ids)
    for required_surface_id in sorted(REQUIRED_SURFACE_IDS - present_surface_ids):
        findings.append(
            make_finding(
                "error",
                "frozen_surface_manifest.required_surface_rows",
                FROZEN_SURFACE_MANIFEST_REL,
                FROZEN_SURFACE_MANIFEST_REL,
                f"frozen-surface manifest is missing required row '{required_surface_id}'",
                "Seed manifest rows for the command surface, docs-pack schema, exact-build fields, route taxonomy, object-handoff packet, and protected-path rules.",
                row_ref=required_surface_id,
            )
        )

    changed_surfaces: list[dict[str, Any]] = []
    for row in rows:
        surface_id = row.get("surface_id", "<unknown>")
        owner_ref = item_ref(surface_id)
        for field_name in (
            "title",
            "surface_class",
            "freeze_status",
            "owner_dri",
            "owning_lane",
            "freeze_basis_refs",
            "monitored_paths",
            "validation_hooks",
            "diff_report_refs",
            "same_train_obligations",
            "waiver_packet_refs",
            "linked_evidence_refs",
        ):
            if not row.get(field_name):
                findings.append(
                    make_finding(
                        "error",
                        f"frozen_surface_manifest.required_row_field.{field_name}",
                        FROZEN_SURFACE_MANIFEST_REL,
                        owner_ref,
                        f"frozen-surface row '{surface_id}' is missing required field '{field_name}'",
                        "Populate the missing field so the row has owner, traceability, and machine-checkable follow-up obligations.",
                        row_ref=surface_id,
                    )
                )

        freeze_status = row.get("freeze_status")
        if freeze_status_values and freeze_status not in freeze_status_values:
            findings.append(
                make_finding(
                    "error",
                    "frozen_surface_manifest.freeze_status_values",
                    FROZEN_SURFACE_MANIFEST_REL,
                    owner_ref,
                    f"frozen-surface row '{surface_id}' uses unknown freeze_status '{freeze_status}'",
                    "Use one of the manifest's declared freeze_status_values.",
                    row_ref=surface_id,
                )
            )

        owning_lane = row.get("owning_lane")
        if owning_lane and owning_lane not in lanes:
            findings.append(
                make_finding(
                    "error",
                    "frozen_surface_manifest.owning_lane_resolves",
                    FROZEN_SURFACE_MANIFEST_REL,
                    owner_ref,
                    f"frozen-surface row '{surface_id}' points at unknown owning_lane '{owning_lane}'",
                    "Use an ownership_matrix governance_lanes id for every frozen-surface row.",
                    row_ref=surface_id,
                )
            )

        for ref in row.get("freeze_basis_refs", []):
            if ref.startswith(f"{STABLE_SURFACE_INVENTORY_REL}#"):
                fragment = ref.split("#", 1)[1]
                if fragment not in stable_surface_ids:
                    findings.append(
                        make_finding(
                            "error",
                            "frozen_surface_manifest.freeze_basis_stable_surface_resolves",
                            FROZEN_SURFACE_MANIFEST_REL,
                            owner_ref,
                            f"frozen-surface row '{surface_id}' points at unknown stable-surface row '{fragment}'",
                            "Keep freeze_basis_refs aligned with stable_surface_inventory row ids.",
                            row_ref=surface_id,
                        )
                    )
            elif ref.startswith(f"{INTERFACE_FREEZE_MATRIX_REL}#"):
                fragment = ref.split("#", 1)[1]
                if fragment not in freeze_row_ids:
                    findings.append(
                        make_finding(
                            "error",
                            "frozen_surface_manifest.freeze_basis_freeze_row_resolves",
                            FROZEN_SURFACE_MANIFEST_REL,
                            owner_ref,
                            f"frozen-surface row '{surface_id}' points at unknown interface-freeze row '{fragment}'",
                            "Keep freeze_basis_refs aligned with interface_freeze_matrix row ids.",
                            row_ref=surface_id,
                        )
                    )
            elif not existing_path_ref(repo_root, ref):
                findings.append(
                    make_finding(
                        "error",
                        "frozen_surface_manifest.freeze_basis_refs_exist",
                        FROZEN_SURFACE_MANIFEST_REL,
                        owner_ref,
                        f"frozen-surface row '{surface_id}' references missing freeze basis '{ref}'",
                        "Keep freeze_basis_refs pointed at real contract or policy artifacts.",
                        row_ref=surface_id,
                    )
                )

        for ref_group_name in ("monitored_paths", "diff_report_refs", "waiver_packet_refs", "linked_evidence_refs"):
            for ref in row.get(ref_group_name, []):
                if not existing_path_ref(repo_root, ref):
                    findings.append(
                        make_finding(
                            "error",
                            f"frozen_surface_manifest.{ref_group_name}_exist",
                            FROZEN_SURFACE_MANIFEST_REL,
                            owner_ref,
                            f"frozen-surface row '{surface_id}' references missing artifact '{ref}' in {ref_group_name}",
                            "Keep every monitored path, diff-report ref, waiver ref, and linked evidence ref current.",
                            row_ref=surface_id,
                        )
                    )

        for hook in row.get("validation_hooks", []):
            hook_id = hook.get("hook_id", "<unknown>")
            entrypoint = hook.get("entrypoint")
            if not hook.get("hook_id") or not hook.get("expectation"):
                findings.append(
                    make_finding(
                        "error",
                        "frozen_surface_manifest.validation_hooks_complete",
                        FROZEN_SURFACE_MANIFEST_REL,
                        owner_ref,
                        f"frozen-surface row '{surface_id}' has incomplete validation hook metadata for '{hook_id}'",
                        "Every validation hook must name a hook_id, entrypoint, and expectation.",
                        row_ref=surface_id,
                    )
                )
            if not entrypoint or not existing_path_ref(repo_root, entrypoint):
                findings.append(
                    make_finding(
                        "error",
                        "frozen_surface_manifest.validation_hook_entrypoints_exist",
                        FROZEN_SURFACE_MANIFEST_REL,
                        owner_ref,
                        f"frozen-surface row '{surface_id}' points at missing validation hook entrypoint '{entrypoint}'",
                        "Keep validation_hooks.entrypoint pointed at a real local tool or script.",
                        row_ref=surface_id,
                    )
                )

        for obligation in row.get("same_train_obligations", []):
            obligation_id = obligation.get("obligation_id", "<unknown>")
            obligation_class = obligation.get("obligation_class")
            refs = obligation.get("refs", [])
            if not obligation.get("obligation_id") or not refs:
                findings.append(
                    make_finding(
                        "error",
                        "frozen_surface_manifest.same_train_obligations_complete",
                        FROZEN_SURFACE_MANIFEST_REL,
                        owner_ref,
                        f"frozen-surface row '{surface_id}' has incomplete obligation metadata for '{obligation_id}'",
                        "Every same-train obligation must name an obligation_id, obligation_class, and one or more refs.",
                        row_ref=surface_id,
                    )
                )
            if obligation_classes and obligation_class not in obligation_classes:
                findings.append(
                    make_finding(
                        "error",
                        "frozen_surface_manifest.same_train_obligation_classes",
                        FROZEN_SURFACE_MANIFEST_REL,
                        owner_ref,
                        f"frozen-surface row '{surface_id}' uses unknown obligation_class '{obligation_class}'",
                        "Use one of the manifest's declared obligation_class_values.",
                        row_ref=surface_id,
                    )
                )
            for ref in refs:
                if not existing_path_ref(repo_root, ref):
                    findings.append(
                        make_finding(
                            "error",
                            "frozen_surface_manifest.same_train_obligation_refs_exist",
                            FROZEN_SURFACE_MANIFEST_REL,
                            owner_ref,
                            f"frozen-surface row '{surface_id}' references missing obligation artifact '{ref}'",
                            "Keep same-train obligation refs pointed at real docs, packets, or compatibility artifacts.",
                            row_ref=surface_id,
                        )
                    )

        matched_paths = sorted(
            {
                changed_file
                for changed_file in changed_files
                for ref in row.get("monitored_paths", [])
                if path_matches(repo_root, changed_file, ref)
            }
        )
        if not matched_paths:
            continue

        diff_hits = sorted(
            {
                changed_file
                for changed_file in changed_files
                for ref in row.get("diff_report_refs", [])
                if path_matches(repo_root, changed_file, ref)
            }
        )

        obligation_hits: list[dict[str, Any]] = []
        for obligation in row.get("same_train_obligations", []):
            hits = sorted(
                {
                    changed_file
                    for changed_file in changed_files
                    for ref in obligation.get("refs", [])
                    if path_matches(repo_root, changed_file, ref)
                }
            )
            if hits:
                obligation_hits.append(
                    {
                        "obligation_id": obligation.get("obligation_id"),
                        "obligation_class": obligation.get("obligation_class"),
                        "matched_paths": hits,
                    }
                )

        waiver_hits = sorted(
            {
                changed_file
                for changed_file in changed_files
                for ref in row.get("waiver_packet_refs", [])
                if path_matches(repo_root, changed_file, ref)
            }
        )

        surface_findings_before = len(findings)
        if not diff_hits:
            findings.append(
                make_finding(
                    "error",
                    "frozen_surface_manifest.diff_metadata_required",
                    FROZEN_SURFACE_MANIFEST_REL,
                    owner_ref,
                    f"changed frozen surface '{surface_id}' is missing a manifest or diff-report touch in the same change",
                    "Touch artifacts/contracts/frozen_surface_manifest.yaml or one of the row's diff_report_refs so the change carries explicit diff metadata.",
                    row_ref=surface_id,
                    changed_paths=matched_paths,
                )
            )

        if not obligation_hits and not waiver_hits:
            findings.append(
                make_finding(
                    "error",
                    "frozen_surface_manifest.same_train_follow_up_required",
                    FROZEN_SURFACE_MANIFEST_REL,
                    owner_ref,
                    f"changed frozen surface '{surface_id}' is missing same-train companion updates or an explicit waiver/exception touch",
                    "Update at least one same-train obligation ref or land a waiver/exception packet update in the same change.",
                    row_ref=surface_id,
                    changed_paths=matched_paths,
                )
            )

        changed_surfaces.append(
            {
                "surface_id": surface_id,
                "title": row.get("title"),
                "matched_paths": matched_paths,
                "diff_metadata_touches": diff_hits,
                "obligation_hits": obligation_hits,
                "waiver_hits": waiver_hits,
                "status": "fail" if len(findings) > surface_findings_before else "pass",
            }
        )

    analysis = {
        "report_kind": "frozen_surface_validation_report",
        "schema_version": 1,
        "generated_at": now_utc(),
        "repo_root": str(repo_root),
        "manifest_ref": FROZEN_SURFACE_MANIFEST_REL,
        "policy_ref": FROZEN_SURFACE_POLICY_REL,
        "scenario": None
        if scenario_payload is None
        else {
            "path": scenario_payload.get("_path"),
            "scenario_id": scenario_payload.get("scenario_id"),
            "description": scenario_payload.get("description"),
        },
        "summary": {
            "changed_file_count": len(changed_files),
            "changed_surface_count": len(changed_surfaces),
            "error_count": sum(1 for finding in findings if finding.severity == "error"),
            "warning_count": sum(1 for finding in findings if finding.severity == "warning"),
        },
        "changed_files": changed_files,
        "changed_surfaces": changed_surfaces,
        "findings": [finding.as_report() for finding in findings],
    }
    return findings, analysis


def render_human_summary(findings: list[FrozenSurfaceFinding], analysis: dict[str, Any]) -> str:
    summary = analysis["summary"]
    status = "FAIL" if summary["error_count"] else "PASS"
    lines = [
        (
            "[frozen-surface-check] "
            f"{status} ({summary['changed_file_count']} changed files, "
            f"{summary['changed_surface_count']} changed surfaces, "
            f"{summary['error_count']} errors, {summary['warning_count']} warnings)"
        )
    ]
    scenario = analysis.get("scenario")
    if scenario is not None:
        lines.append(f"[frozen-surface-check] scenario: {scenario.get('path', '<unknown>')}")

    if analysis["changed_files"]:
        lines.append("[frozen-surface-check] changed files:")
        for changed_file in analysis["changed_files"]:
            lines.append(f"  - {changed_file}")

    if not findings:
        lines.append("[frozen-surface-check] no findings")
        return "\n".join(lines) + "\n"

    for finding in sorted(findings, key=lambda item: (item.severity != "error", item.check_id, item.owner_artifact_ref)):
        lines.append(f"[{finding.severity.upper()}] {finding.check_id} :: {finding.message}")
        lines.append(f"  artifact: {finding.artifact_ref}")
        lines.append(f"  owner:    {finding.owner_artifact_ref}")
        if finding.row_ref:
            lines.append(f"  row:      {finding.row_ref}")
        lines.append(f"  fix:      {finding.remediation}")
    return "\n".join(lines) + "\n"
