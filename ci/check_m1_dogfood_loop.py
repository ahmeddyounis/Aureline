#!/usr/bin/env python3
"""Validate the internal dogfood guide, feedback taxonomy, and blocker-routing loop.

This check keeps the dogfood feedback loop executable by ensuring the canonical
dogfood intake artifacts:

- exist and parse;
- carry stable schema/owner headers;
- map feedback categories to the canonical blocker taxonomy; and
- are registered in the M1 proof artifact index so reviewers can locate the
  latest capture without hunting ad hoc notes.
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

DEFAULT_TAXONOMY_REL = "artifacts/dogfood/feedback_taxonomy.yaml"
DEFAULT_GUIDE_REL = "docs/dogfood/m1_internal_dogfood.md"
DEFAULT_ROUTING_REL = "docs/support/m1_blocker_routing.md"
DEFAULT_BLOCKER_TAXONOMY_REL = "artifacts/milestones/m1/blocker_taxonomy.yaml"
DEFAULT_INDEX_REL = "artifacts/milestones/m1/artifact_index.yaml"

SENTINEL_REFS = {
    "not_yet_seeded",
    "outline_only",
    "contract_not_yet_seeded",
    "planned_not_yet_seeded",
}

REQUIRED_CATEGORY_IDS = {
    "hot_path",
    "fidelity",
    "recovery",
    "trust",
    "boundary",
    "onboarding",
}

REQUIRED_GUIDE_REFS = {
    "artifacts/milestones/m1/dogfood_matrix.yaml",
    "artifacts/dogfood/feedback_taxonomy.yaml",
    "artifacts/milestones/m1/blocker_taxonomy.yaml",
    "artifacts/milestones/m1/known_gaps_ledger.yaml",
    "docs/governance/dogfood_issue_taxonomy.md",
    "artifacts/build/build_identity.json",
    "docs/support/m1_blocker_routing.md",
}

REQUIRED_ROUTING_REFS = {
    "artifacts/dogfood/feedback_taxonomy.yaml",
    "artifacts/milestones/m1/blocker_taxonomy.yaml",
    "artifacts/milestones/m1/known_gaps_ledger.yaml",
    "docs/governance/dogfood_issue_taxonomy.md",
}

EXPECTED_CANONICAL_ARTIFACT_IDS = {
    "dogfood_loop_entrypoint",
    "dogfood_feedback_taxonomy",
    "dogfood_blocker_routing_entrypoint",
    "dogfood_loop_packet",
    "dogfood_loop_validator",
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
    parser.add_argument("--taxonomy", default=DEFAULT_TAXONOMY_REL)
    parser.add_argument("--guide", default=DEFAULT_GUIDE_REL)
    parser.add_argument("--routing", default=DEFAULT_ROUTING_REL)
    parser.add_argument("--blocker-taxonomy", default=DEFAULT_BLOCKER_TAXONOMY_REL)
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

    _ = ensure_str(payload.get("owner"), f"{label}.owner")


def validate_markdown_refs(path: Path, required_refs: set[str], findings: list[Finding], check_id_prefix: str) -> None:
    try:
        text = path.read_text(encoding="utf-8")
    except Exception as exc:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{check_id_prefix}.read_failed",
                message=f"failed to read {path}: {exc}",
                remediation="Ensure the document exists and is readable as UTF-8.",
                ref=str(path),
            )
        )
        return

    missing = sorted(ref for ref in required_refs if ref not in text)
    for ref in missing:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{check_id_prefix}.missing_ref",
                message=f"{path} is missing required canonical ref: {ref}",
                remediation="Add the missing canonical ref so reviewers have one stable entrypoint.",
                ref=ref,
            )
        )


def validate_taxonomy(
    repo_root: Path,
    taxonomy: dict[str, Any],
    blocker_taxonomy: dict[str, Any],
    findings: list[Finding],
) -> None:
    validate_header(taxonomy, "taxonomy", findings)

    ensure_str(taxonomy.get("taxonomy_id"), "taxonomy.taxonomy_id")

    entrypoint = ensure_str(taxonomy.get("human_entrypoint_ref"), "taxonomy.human_entrypoint_ref")
    if not artifact_ref_exists(repo_root, entrypoint):
        findings.append(
            Finding(
                severity="error",
                check_id="taxonomy.human_entrypoint_ref.missing",
                message=f"human_entrypoint_ref does not exist: {entrypoint}",
                remediation="Fix the ref so reviewers have a stable landing page.",
                ref=entrypoint,
            )
        )

    inputs = ensure_dict(taxonomy.get("inputs"), "taxonomy.inputs")
    for key in ("blocker_taxonomy_ref", "known_gaps_ledger_ref", "dogfood_matrix_ref", "issue_taxonomy_ref", "build_identity_ref"):
        ref = ensure_str(inputs.get(key), f"taxonomy.inputs.{key}")
        if not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="taxonomy.inputs.missing_ref",
                    message=f"taxonomy input ref does not exist: {ref}",
                    remediation="Fix the ref so the taxonomy can be joined to real artifacts.",
                    ref=ref,
                    details={"input_key": key},
                )
            )

    required_fields = ensure_list(taxonomy.get("required_issue_fields_for_blocker"), "taxonomy.required_issue_fields_for_blocker")
    if not required_fields:
        findings.append(
            Finding(
                severity="error",
                check_id="taxonomy.required_issue_fields_for_blocker.empty",
                message="required_issue_fields_for_blocker must not be empty",
                remediation="List the minimum required dogfood issue fields for blocker triage.",
            )
        )

    blocker_classes = ensure_list(blocker_taxonomy.get("blocker_classes"), "blocker_taxonomy.blocker_classes")
    blocker_class_ids = {ensure_str(row.get("class_id"), "blocker_taxonomy.blocker_classes[].class_id") for row in blocker_classes if isinstance(row, dict)}

    severity_vocab = set(ensure_str(v, "blocker_taxonomy.severity_vocabulary[]") for v in ensure_list(blocker_taxonomy.get("severity_vocabulary"), "blocker_taxonomy.severity_vocabulary"))

    crosswalk = ensure_list(taxonomy.get("severity_crosswalk"), "taxonomy.severity_crosswalk")
    if not crosswalk:
        findings.append(
            Finding(
                severity="error",
                check_id="taxonomy.severity_crosswalk.empty",
                message="severity_crosswalk must not be empty",
                remediation="Provide a mapping from dogfood issue severity to blocker severity.",
            )
        )
    else:
        for idx, raw in enumerate(crosswalk):
            row = ensure_dict(raw, f"taxonomy.severity_crosswalk[{idx}]")
            issue_sev = ensure_str(row.get("issue_severity"), f"taxonomy.severity_crosswalk[{idx}].issue_severity")
            blocker_sev = ensure_str(row.get("blocker_severity"), f"taxonomy.severity_crosswalk[{idx}].blocker_severity")
            if blocker_sev not in severity_vocab:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="taxonomy.severity_crosswalk.blocker_severity.invalid",
                        message=f"severity_crosswalk blocker_severity is not in blocker taxonomy severity_vocabulary: {blocker_sev!r}",
                        remediation="Use one of the severities listed in artifacts/milestones/m1/blocker_taxonomy.yaml.",
                        ref=blocker_sev,
                    )
                )
            if issue_sev not in {"daily_blocker", "major", "scoped", "clarity_gap"}:
                findings.append(
                    Finding(
                        severity="warning",
                        check_id="taxonomy.severity_crosswalk.issue_severity.unknown",
                        message=f"severity_crosswalk issue_severity is not a known dogfood severity token: {issue_sev!r}",
                        remediation="Use one of: daily_blocker, major, scoped, clarity_gap.",
                        ref=issue_sev,
                    )
                )

    categories = ensure_list(taxonomy.get("feedback_categories"), "taxonomy.feedback_categories")
    if not categories:
        findings.append(
            Finding(
                severity="error",
                check_id="taxonomy.feedback_categories.empty",
                message="feedback_categories must not be empty",
                remediation="Declare the feedback category mapping used for dogfood routing.",
            )
        )
        return

    seen_ids: list[str] = []
    for idx, raw_cat in enumerate(categories):
        cat = ensure_dict(raw_cat, f"taxonomy.feedback_categories[{idx}]")
        category_id = ensure_str(cat.get("category_id"), f"taxonomy.feedback_categories[{idx}].category_id")
        ensure_str(cat.get("title"), f"taxonomy.feedback_categories[{idx}].title")
        ensure_str(cat.get("description"), f"taxonomy.feedback_categories[{idx}].description")
        ensure_str(cat.get("dogfood_category"), f"taxonomy.feedback_categories[{idx}].dogfood_category")
        labels = ensure_list(cat.get("default_issue_labels"), f"taxonomy.feedback_categories[{idx}].default_issue_labels")
        if not labels:
            findings.append(
                Finding(
                    severity="error",
                    check_id="taxonomy.feedback_categories.default_issue_labels.empty",
                    message=f"feedback category {category_id} default_issue_labels must not be empty",
                    remediation="Provide at least one suggested label token for routing.",
                    ref=category_id,
                )
            )
        blocker_class_id = ensure_str(cat.get("blocker_class_id"), f"taxonomy.feedback_categories[{idx}].blocker_class_id")
        if blocker_class_id not in blocker_class_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="taxonomy.feedback_categories.blocker_class_id.invalid",
                    message=f"feedback category {category_id} references unknown blocker_class_id: {blocker_class_id}",
                    remediation="Use a class_id from artifacts/milestones/m1/blocker_taxonomy.yaml.",
                    ref=blocker_class_id,
                    details={"category_id": category_id},
                )
            )
        seen_ids.append(category_id)

    duplicates = sorted({cid for cid in seen_ids if seen_ids.count(cid) > 1})
    for dup in duplicates:
        findings.append(
            Finding(
                severity="error",
                check_id="taxonomy.feedback_categories.duplicate",
                message=f"feedback category_id is duplicated: {dup}",
                remediation="Deduplicate category ids so joins remain stable.",
                ref=dup,
            )
        )

    missing_required = sorted(REQUIRED_CATEGORY_IDS - set(seen_ids))
    if missing_required:
        findings.append(
            Finding(
                severity="error",
                check_id="taxonomy.feedback_categories.required_missing",
                message=f"feedback_categories is missing required category ids: {missing_required}",
                remediation="Add the missing categories so dogfood intake can be routed consistently.",
            )
        )


def validate_artifact_index(repo_root: Path, index: dict[str, Any], findings: list[Finding]) -> None:
    validate_header(index, "index", findings)

    canonical = ensure_list(index.get("canonical_artifacts"), "index.canonical_artifacts")
    found_ids: set[str] = set()
    for idx, raw_row in enumerate(canonical):
        row = ensure_dict(raw_row, f"index.canonical_artifacts[{idx}]")
        artifact_id = ensure_str(row.get("artifact_id"), f"index.canonical_artifacts[{idx}].artifact_id")
        artifact_ref = ensure_str(row.get("artifact_ref"), f"index.canonical_artifacts[{idx}].artifact_ref")
        found_ids.add(artifact_id)
        if artifact_id in EXPECTED_CANONICAL_ARTIFACT_IDS and not artifact_ref_exists(repo_root, artifact_ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.canonical_artifacts.missing_ref",
                    message=f"canonical artifact {artifact_id} ref does not exist: {artifact_ref}",
                    remediation="Fix the ref path or seed the missing artifact so proof lanes stay discoverable.",
                    ref=artifact_ref,
                )
            )

    missing = sorted(EXPECTED_CANONICAL_ARTIFACT_IDS - found_ids)
    if missing:
        findings.append(
            Finding(
                severity="error",
                check_id="index.canonical_artifacts.required_missing",
                message=f"artifact index is missing required canonical artifact ids for dogfood loop: {missing}",
                remediation="Register the dogfood loop entrypoint, taxonomy, and validator in artifacts/milestones/m1/artifact_index.yaml.",
            )
        )

    lanes = ensure_list(index.get("proof_lanes"), "index.proof_lanes")
    dogfood_lane = next((lane for lane in lanes if isinstance(lane, dict) and lane.get("lane_id") == "dogfood_loop"), None)
    if dogfood_lane is None:
        findings.append(
            Finding(
                severity="error",
                check_id="index.proof_lanes.dogfood_loop.missing",
                message="proof_lanes must include lane_id: dogfood_loop",
                remediation="Register the dogfood loop proof lane so reviewers can find its captures and packet.",
                ref="dogfood_loop",
            )
        )
        return

    owning_packet = ensure_str(dogfood_lane.get("owning_packet_ref"), "index.proof_lanes[dogfood_loop].owning_packet_ref")
    if not artifact_ref_exists(repo_root, owning_packet):
        findings.append(
            Finding(
                severity="error",
                check_id="index.proof_lanes.dogfood_loop.owning_packet_ref.missing",
                message=f"dogfood_loop owning_packet_ref does not exist: {owning_packet}",
                remediation="Seed the owning packet under artifacts/milestones/m1/proof_packets/.",
                ref=owning_packet,
            )
        )


def write_report(repo_root: Path, report_rel: str, taxonomy_rel: str, findings: list[Finding]) -> None:
    report_path = repo_root / report_rel
    report_path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": 1,
        "check_id": "m1_internal_dogfood_loop",
        "generated_at": dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
        "taxonomy_ref": taxonomy_rel,
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

    taxonomy_rel = str(args.taxonomy)
    guide_rel = str(args.guide)
    routing_rel = str(args.routing)
    blocker_taxonomy_rel = str(args.blocker_taxonomy)
    index_rel = str(args.index)

    findings: list[Finding] = []

    taxonomy_path = repo_root / taxonomy_rel
    taxonomy_payload = ensure_dict(render_yaml_as_json(taxonomy_path), taxonomy_rel)

    blocker_taxonomy_path = repo_root / blocker_taxonomy_rel
    blocker_taxonomy_payload = ensure_dict(render_yaml_as_json(blocker_taxonomy_path), blocker_taxonomy_rel)

    validate_taxonomy(repo_root, taxonomy_payload, blocker_taxonomy_payload, findings)

    guide_path = repo_root / guide_rel
    if not guide_path.exists():
        findings.append(
            Finding(
                severity="error",
                check_id="guide.missing",
                message=f"internal dogfood guide is missing: {guide_rel}",
                remediation="Create the internal dogfood guide so participants have one entrypoint.",
                ref=guide_rel,
            )
        )
    else:
        validate_markdown_refs(guide_path, REQUIRED_GUIDE_REFS, findings, "guide")

    routing_path = repo_root / routing_rel
    if not routing_path.exists():
        findings.append(
            Finding(
                severity="error",
                check_id="routing.missing",
                message=f"blocker routing doc is missing: {routing_rel}",
                remediation="Create the blocker routing doc so triage rules are explicit.",
                ref=routing_rel,
            )
        )
    else:
        validate_markdown_refs(routing_path, REQUIRED_ROUTING_REFS, findings, "routing")

    index_path = repo_root / index_rel
    index_payload = ensure_dict(render_yaml_as_json(index_path), index_rel)
    validate_artifact_index(repo_root, index_payload, findings)

    if args.report:
        write_report(repo_root, str(args.report), taxonomy_rel, findings)

    errors = [f for f in findings if f.severity == "error"]
    warnings = [f for f in findings if f.severity == "warning"]

    status = "PASS" if not errors else "FAIL"
    print(f"[m1-dogfood-loop] {status} ({len(errors)} errors, {len(warnings)} warnings)")
    for finding in findings:
        prefix = "E" if finding.severity == "error" else "W"
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        print(f"[m1-dogfood-loop] {prefix} {finding.check_id}: {finding.message}{ref_suffix}")
        print(f"[m1-dogfood-loop]   remediation: {finding.remediation}")

    return 1 if errors else 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except KeyboardInterrupt:
        print("[m1-dogfood-loop] interrupted", file=sys.stderr)
        raise

