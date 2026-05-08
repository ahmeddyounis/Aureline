#!/usr/bin/env python3
"""Validate the late-M1 public-truth checkpoint pack.

This check keeps the public-truth freeze pack executable by ensuring the
canonical pack artifact:

- exists and parses;
- carries a stable schema/owner header;
- enumerates the required checkpoint outputs; and
- references real artifacts for seeded rows (or explicit sentinels for planned
  rows).

It also validates the proof-index consumer so downstream packets and dashboards
can locate the canonical artifacts from one index file.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_PACK_REL = "artifacts/milestones/m1/late_freeze_pack.yaml"
DEFAULT_INDEX_REL = "artifacts/milestones/m1/artifact_index.yaml"

SENTINEL_REFS = {
    "not_yet_seeded",
    "outline_only",
    "planned_not_yet_seeded",
}

REQUIRED_OUTPUT_IDS = {
    "badge_vocabulary_draft",
    "command_diagnostics_skeleton",
    "deprecation_packet_template",
    "docs_browser_source_version_rows",
    "durable_job_row_prototype",
    "embedded_origin_chrome",
    "help_about_skeleton",
    "help_about_truth_prototype",
    "repo_hygiene_scaffolding",
    "reproducibility_packet_seed",
    "start_center_truth_path",
    "support_bundle_redaction_guide",
    "target_origin_badges",
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
    parser.add_argument("--pack", default=DEFAULT_PACK_REL)
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


def ensure_str(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value.strip():
        raise SystemExit(f"{label} must be a non-empty string, got {value!r}")
    return value.strip()


def ensure_int(value: Any, label: str) -> int:
    if not isinstance(value, int):
        raise SystemExit(f"{label} must be an integer, got {value!r}")
    return value


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be a mapping/object, got {value!r}")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a list/array, got {value!r}")
    return value


def parse_iso_date(value: str, label: str) -> dt.date:
    try:
        return dt.date.fromisoformat(value)
    except ValueError as exc:
        raise SystemExit(f"{label} must be a YYYY-MM-DD date, got {value!r}") from exc


def strip_fragment(ref: str) -> str:
    return ref.split("#", 1)[0].strip()


def artifact_ref_exists(repo_root: Path, ref: str) -> bool:
    ref = strip_fragment(ref)
    return bool(ref) and (repo_root / ref).exists()


def is_sentinel_ref(ref: str) -> bool:
    return ref.strip() in SENTINEL_REFS


def validate_header(payload: dict[str, Any], label: str, findings: list[Finding]) -> None:
    schema_version = ensure_int(payload.get("schema_version"), f"{label}.schema_version")
    if schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.schema_version.unsupported",
                message=f"schema_version must be 1, got {schema_version}",
                remediation="Bump the validator in the same change that bumps the schema version.",
            )
        )

    as_of = ensure_str(payload.get("as_of"), f"{label}.as_of")
    _ = parse_iso_date(as_of, f"{label}.as_of")

    owner = ensure_str(payload.get("owner"), f"{label}.owner")
    if not owner.startswith("@"):
        findings.append(
            Finding(
                severity="warning",
                check_id=f"{label}.owner.format",
                message=f"owner does not look like a handle: {owner!r}",
                remediation="Use an @handle so review routing is explicit.",
            )
        )


def validate_pack(repo_root: Path, pack: dict[str, Any], pack_rel: str, findings: list[Finding]) -> None:
    validate_header(pack, "pack", findings)

    ensure_str(pack.get("pack_id"), "pack.pack_id")
    ensure_str(pack.get("title"), "pack.title")

    human_entrypoint_ref = ensure_str(pack.get("human_entrypoint_ref"), "pack.human_entrypoint_ref")
    if not artifact_ref_exists(repo_root, human_entrypoint_ref):
        findings.append(
            Finding(
                severity="error",
                check_id="pack.human_entrypoint_ref.missing",
                message=f"human_entrypoint_ref does not exist: {human_entrypoint_ref}",
                remediation="Fix the path so reviewers have a stable landing page.",
                ref=human_entrypoint_ref,
            )
        )

    review_packet_ref = ensure_str(pack.get("review_packet_ref"), "pack.review_packet_ref")
    if not artifact_ref_exists(repo_root, review_packet_ref):
        findings.append(
            Finding(
                severity="error",
                check_id="pack.review_packet_ref.missing",
                message=f"review_packet_ref does not exist: {review_packet_ref}",
                remediation="Fix the path so the review workflow stays discoverable.",
                ref=review_packet_ref,
            )
        )

    artifact_index_ref = ensure_str(pack.get("artifact_index_ref"), "pack.artifact_index_ref")
    if not artifact_ref_exists(repo_root, artifact_index_ref):
        findings.append(
            Finding(
                severity="error",
                check_id="pack.artifact_index_ref.missing",
                message=f"artifact_index_ref does not exist: {artifact_index_ref}",
                remediation="Fix the path so the pack can be discovered via the proof artifact index.",
                ref=artifact_index_ref,
            )
        )

    validator = ensure_dict(pack.get("validator"), "pack.validator")
    script_ref = ensure_str(validator.get("script_ref"), "pack.validator.script_ref")
    if not artifact_ref_exists(repo_root, script_ref):
        findings.append(
            Finding(
                severity="error",
                check_id="pack.validator.script_ref.missing",
                message=f"validator script_ref does not exist: {script_ref}",
                remediation="Fix the path (or seed the validator script) so the pack remains executable.",
                ref=script_ref,
            )
        )
    _ = ensure_str(validator.get("command"), "pack.validator.command")
    ci_gate_ref = ensure_str(validator.get("ci_gate_ref"), "pack.validator.ci_gate_ref")
    if not artifact_ref_exists(repo_root, ci_gate_ref):
        findings.append(
            Finding(
                severity="warning",
                check_id="pack.validator.ci_gate_ref.missing",
                message=f"ci gate ref does not exist (CI will not enforce this pack): {ci_gate_ref}",
                remediation="Add the workflow gate (or fix the ref) so the validator runs in CI.",
                ref=ci_gate_ref,
            )
        )

    seed_vocab = ensure_list(pack.get("seed_state_vocabulary"), "pack.seed_state_vocabulary")
    seed_vocab = [ensure_str(value, "pack.seed_state_vocabulary[]") for value in seed_vocab]
    for required in ("seeded", "planned_not_yet_seeded"):
        if required not in seed_vocab:
            findings.append(
                Finding(
                    severity="error",
                    check_id="pack.seed_state_vocabulary.missing_required",
                    message=f"seed_state_vocabulary must include {required!r}",
                    remediation="Add the missing vocabulary value so planned/seeded rows remain explicit.",
                    ref=required,
                )
            )

    outputs = ensure_list(pack.get("outputs"), "pack.outputs")
    if not outputs:
        findings.append(
            Finding(
                severity="error",
                check_id="pack.outputs.empty",
                message="outputs must not be empty",
                remediation="Add the required checkpoint output rows so the pack can govern public-truth hardening.",
            )
        )
        return

    seen: set[str] = set()
    for idx, raw_row in enumerate(outputs):
        row = ensure_dict(raw_row, f"pack.outputs[{idx}]")
        output_id = ensure_str(row.get("output_id"), f"pack.outputs[{idx}].output_id")
        ensure_str(row.get("title"), f"pack.outputs[{idx}].title")
        seed_state = ensure_str(row.get("seed_state"), f"pack.outputs[{idx}].seed_state")
        if seed_state not in seed_vocab:
            findings.append(
                Finding(
                    severity="error",
                    check_id="pack.outputs.seed_state.invalid",
                    message=f"output {output_id} seed_state must be one of {sorted(set(seed_vocab))}, got {seed_state!r}",
                    remediation="Use a value from seed_state_vocabulary (seeded or planned_not_yet_seeded).",
                    ref=output_id,
                )
            )

        owner_dri = ensure_str(row.get("owner_dri"), f"pack.outputs[{idx}].owner_dri")
        if not owner_dri.startswith("@"):
            findings.append(
                Finding(
                    severity="warning",
                    check_id="pack.outputs.owner_dri.format",
                    message=f"output {output_id} owner_dri does not look like a handle: {owner_dri!r}",
                    remediation="Use an @handle so review routing is explicit.",
                    ref=output_id,
                )
            )

        review_forum_ref = ensure_str(row.get("review_forum_ref"), f"pack.outputs[{idx}].review_forum_ref")
        if not artifact_ref_exists(repo_root, review_forum_ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="pack.outputs.review_forum_ref.missing",
                    message=f"output {output_id} review_forum_ref does not exist: {review_forum_ref}",
                    remediation="Point at an existing governance/ownership artifact so routing stays stable.",
                    ref=output_id,
                )
            )

        proof_refs = ensure_list(row.get("proof_artifact_refs"), f"pack.outputs[{idx}].proof_artifact_refs")
        if not proof_refs:
            findings.append(
                Finding(
                    severity="error",
                    check_id="pack.outputs.proof_artifact_refs.empty",
                    message=f"output {output_id} must list at least one proof_artifact_refs entry",
                    remediation="List the canonical contracts/schemas/seeds that back this output.",
                    ref=output_id,
                )
            )
            proof_refs = []

        saw_sentinel = False
        for ref_idx, raw_ref in enumerate(proof_refs):
            ref = ensure_str(raw_ref, f"pack.outputs[{idx}].proof_artifact_refs[{ref_idx}]")
            if is_sentinel_ref(ref):
                saw_sentinel = True
                continue
            if not artifact_ref_exists(repo_root, ref):
                findings.append(
                    Finding(
                        severity="error",
                        check_id="pack.outputs.proof_artifact_refs.missing",
                        message=f"output {output_id} proof artifact does not exist: {ref}",
                        remediation="Fix the ref path (or mark the output planned_not_yet_seeded with an explicit sentinel).",
                        ref=output_id,
                        details={"proof_ref": ref},
                    )
                )

        if seed_state != "planned_not_yet_seeded" and saw_sentinel:
            findings.append(
                Finding(
                    severity="error",
                    check_id="pack.outputs.seeded_row_has_sentinel",
                    message=f"output {output_id} is {seed_state} but uses sentinel proof refs",
                    remediation="Remove sentinel refs (or mark the row planned_not_yet_seeded).",
                    ref=output_id,
                )
            )

        if output_id in seen:
            findings.append(
                Finding(
                    severity="error",
                    check_id="pack.outputs.output_id.duplicate",
                    message=f"duplicate output_id: {output_id}",
                    remediation="Deduplicate output_id values so downstream joins are stable.",
                    ref=output_id,
                )
            )
        seen.add(output_id)

    missing = REQUIRED_OUTPUT_IDS - seen
    if missing:
        findings.append(
            Finding(
                severity="error",
                check_id="pack.outputs.required.missing",
                message=f"pack is missing required output ids: {sorted(missing)}",
                remediation="Add the missing output rows so late public-truth checkpoints remain explicit and reviewable.",
                ref=pack_rel,
            )
        )


def validate_index(repo_root: Path, index: dict[str, Any], pack_rel: str, findings: list[Finding]) -> None:
    validate_header(index, "index", findings)

    canonical_artifacts = ensure_list(index.get("canonical_artifacts"), "index.canonical_artifacts")
    refs_by_id: dict[str, str] = {}
    for idx, raw_row in enumerate(canonical_artifacts):
        row = ensure_dict(raw_row, f"index.canonical_artifacts[{idx}]")
        artifact_id = ensure_str(row.get("artifact_id"), f"index.canonical_artifacts[{idx}].artifact_id")
        artifact_ref = ensure_str(row.get("artifact_ref"), f"index.canonical_artifacts[{idx}].artifact_ref")
        refs_by_id[artifact_id] = artifact_ref

    expected = {
        "late_freeze_pack": pack_rel,
        "late_freeze_pack_entrypoint": "docs/milestones/m1/late_freeze_pack.md",
        "late_freeze_pack_packet": "artifacts/milestones/m1/proof_packets/late_freeze_pack.md",
        "late_freeze_pack_validator": "ci/check_m1_late_freeze_pack.py",
    }

    for artifact_id, expected_ref in expected.items():
        actual = refs_by_id.get(artifact_id)
        if actual is None:
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.canonical_artifact.missing_row",
                    message=f"artifact_index.yaml canonical_artifacts is missing artifact_id: {artifact_id}",
                    remediation="Add the missing canonical_artifacts row so consumers can locate the canonical artifacts.",
                )
            )
            continue
        if strip_fragment(actual) != expected_ref:
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.canonical_artifact.mismatch",
                    message=f"{artifact_id} artifact_ref must be {expected_ref}, got {actual}",
                    remediation="Update artifact_index.yaml so consumers always point at the canonical path.",
                    ref=actual,
                )
            )
        if not artifact_ref_exists(repo_root, actual):
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.canonical_artifact.missing_ref",
                    message=f"artifact_index.yaml canonical_artifacts ref does not exist: {actual}",
                    remediation="Fix the path or seed the referenced artifact.",
                    ref=actual,
                )
            )

    proof_lanes = ensure_list(index.get("proof_lanes"), "index.proof_lanes")
    lane = None
    for raw_lane in proof_lanes:
        cand = ensure_dict(raw_lane, "index.proof_lanes[]")
        lane_id = cand.get("lane_id")
        if isinstance(lane_id, str) and lane_id.strip() == "late_freeze_pack":
            lane = cand
            break

    if lane is None:
        findings.append(
            Finding(
                severity="error",
                check_id="index.proof_lanes.missing_lane",
                message="artifact_index.yaml proof_lanes is missing lane_id: late_freeze_pack",
                remediation="Register the pack as a proof lane so dashboards and shiproom review can discover it.",
                ref="late_freeze_pack",
            )
        )
        return

    owning_packet_ref = ensure_str(lane.get("owning_packet_ref"), "index.proof_lanes[late_freeze_pack].owning_packet_ref")
    if strip_fragment(owning_packet_ref) != "artifacts/milestones/m1/proof_packets/late_freeze_pack.md":
        findings.append(
            Finding(
                severity="error",
                check_id="index.proof_lanes.owning_packet_ref.mismatch",
                message=f"late_freeze_pack owning_packet_ref must be artifacts/milestones/m1/proof_packets/late_freeze_pack.md, got {owning_packet_ref}",
                remediation="Update artifact_index.yaml to point to the canonical owning packet.",
                ref=owning_packet_ref,
            )
        )

    evidence_refs = ensure_list(lane.get("evidence_refs"), "index.proof_lanes[late_freeze_pack].evidence_refs")
    evidence_set = {strip_fragment(ensure_str(ref, "index.proof_lanes[late_freeze_pack].evidence_refs[]")) for ref in evidence_refs}

    for required in (pack_rel, "docs/milestones/m1/late_freeze_pack.md"):
        if required not in evidence_set:
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.proof_lanes.evidence_refs.missing_required",
                    message=f"late_freeze_pack evidence_refs is missing required ref: {required}",
                    remediation="Add the missing evidence ref so reviewers can locate the canonical artifacts.",
                    ref=required,
                )
            )


def write_report(repo_root: Path, report_rel: str, pack_rel: str, findings: list[Finding]) -> None:
    report_path = repo_root / report_rel
    report_path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": 1,
        "check_id": "m1_late_freeze_pack",
        "generated_at": dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
        "pack_ref": pack_rel,
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

    pack_rel = ensure_str(args.pack, "args.pack")
    index_rel = ensure_str(args.index, "args.index")

    findings: list[Finding] = []
    pack = ensure_dict(render_yaml_as_json(repo_root / pack_rel), "pack")
    validate_pack(repo_root, pack, pack_rel, findings)

    index = ensure_dict(render_yaml_as_json(repo_root / index_rel), "index")
    validate_index(repo_root, index, pack_rel, findings)

    if args.report:
        write_report(repo_root, ensure_str(args.report, "args.report"), pack_rel, findings)

    errors = [f for f in findings if f.severity == "error"]
    warnings = [f for f in findings if f.severity == "warning"]
    for finding in errors + warnings:
        prefix = "error" if finding.severity == "error" else "warning"
        ref = f" ({finding.ref})" if finding.ref else ""
        print(f"[late-freeze-pack] {prefix}: {finding.check_id}{ref}: {finding.message}")
        if finding.details:
            print(json.dumps(finding.details, indent=2, sort_keys=True))
        print(f"[late-freeze-pack] remediation: {finding.remediation}")

    return 1 if errors else 0


if __name__ == "__main__":
    raise SystemExit(main())

