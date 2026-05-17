#!/usr/bin/env python3
"""Validate the beta open-project packet.

The packet publishes two beta baselines:

  - standards and interchange support posture;
  - public/private issue and RFC routing.

This check treats the packet's canonical YAML block as the reviewed beta
publication layer, then cross-checks it against the canonical standards
matrix, issue-routing matrix, starter-pack community lane, and docs/help
consumer pages.
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


DEFAULT_PACKET_REL = "artifacts/milestones/m3/open_project_beta_packet.md"
DEFAULT_STANDARDS_REL = "artifacts/governance/standards_matrix.yaml"
DEFAULT_ISSUE_ROUTING_REL = "artifacts/governance/issue_routing.yaml"
DEFAULT_STARTER_PACK_REL = "artifacts/milestones/m3/beta_enablement_starter_pack.yaml"
DEFAULT_CAPTURE_REL = (
    "artifacts/milestones/m3/captures/"
    "open_project_beta_packet_validation_capture.json"
)

CANONICAL_BLOCK_BEGIN = "<!-- BEGIN canonical:open_project_beta_packet -->"
CANONICAL_BLOCK_END = "<!-- END canonical:open_project_beta_packet -->"

DEFERRED_SUPPORT_CLASSES = {"standard_deferred_placeholder"}
BRIDGE_SUPPORT_CLASSES = {"custom_but_mirrorable", "custom_with_bridge_planned"}
CLAIM_BEARING_POSTURES = {"claim_bearing", "narrowed_export_only", "narrowed_import_only"}


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
    parser.add_argument("--packet", default=DEFAULT_PACKET_REL)
    parser.add_argument("--standards", default=DEFAULT_STANDARDS_REL)
    parser.add_argument("--issue-routing", default=DEFAULT_ISSUE_ROUTING_REL)
    parser.add_argument("--starter-pack", default=DEFAULT_STARTER_PACK_REL)
    parser.add_argument("--capture", default=DEFAULT_CAPTURE_REL)
    parser.add_argument(
        "--check",
        action="store_true",
        help="Fail when the validation capture on disk would change.",
    )
    return parser.parse_args()


def render_yaml_as_json(text: str, label: str) -> Any:
    ruby = subprocess.run(
        [
            "ruby",
            "-rjson",
            "-ryaml",
            "-e",
            (
                "require 'date';"
                " payload = YAML.safe_load(STDIN.read,"
                " permitted_classes: [Date, Time], aliases: false);"
                " STDOUT.write(JSON.generate(payload))"
            ),
        ],
        input=text,
        capture_output=True,
        text=True,
    )
    if ruby.returncode != 0:
        stderr = ruby.stderr.strip() or "unknown Ruby/Psych failure"
        raise SystemExit(f"failed to parse YAML for {label}: {stderr}")
    try:
        return json.loads(ruby.stdout)
    except json.JSONDecodeError as exc:
        raise SystemExit(f"Ruby/Psych emitted invalid JSON for {label}: {exc}") from exc


def render_yaml_file_as_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing YAML file: {path}")
    return render_yaml_as_json(path.read_text(encoding="utf-8"), str(path))


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be a mapping/object")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a list/array")
    return value


def ensure_str(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value.strip():
        raise SystemExit(f"{label} must be a non-empty string")
    return value.strip()


def ensure_int(value: Any, label: str) -> int:
    if not isinstance(value, int):
        raise SystemExit(f"{label} must be an integer")
    return value


def extract_canonical_block(packet_text: str) -> str:
    if CANONICAL_BLOCK_BEGIN not in packet_text:
        raise SystemExit("open-project packet missing BEGIN canonical sentinel")
    if CANONICAL_BLOCK_END not in packet_text:
        raise SystemExit("open-project packet missing END canonical sentinel")
    block = packet_text.split(CANONICAL_BLOCK_BEGIN, 1)[1].split(
        CANONICAL_BLOCK_END, 1
    )[0]
    if "```yaml" not in block or "```" not in block.split("```yaml", 1)[1]:
        raise SystemExit("canonical block must be wrapped in one ```yaml fence")
    yaml_body = block.split("```yaml", 1)[1].split("```", 1)[0]
    if not yaml_body.strip():
        raise SystemExit("canonical YAML block is empty")
    return yaml_body


def parse_iso_date(value: str, label: str) -> None:
    try:
        dt.date.fromisoformat(value)
    except ValueError as exc:
        raise SystemExit(f"{label} must be a YYYY-MM-DD date, got {value!r}") from exc


def strip_fragment(ref: str) -> str:
    return ref.split("#", 1)[0].strip()


def ref_exists(repo_root: Path, ref: str) -> bool:
    clean = strip_fragment(ref)
    return bool(clean) and (repo_root / clean).exists()


def validate_path_map(
    repo_root: Path, mapping: dict[str, Any], label: str, findings: list[Finding]
) -> None:
    for key, raw_ref in mapping.items():
        if not isinstance(raw_ref, str) or not raw_ref.strip():
            findings.append(
                Finding(
                    "error",
                    f"{label}.invalid_ref",
                    f"{label}.{key} must be a non-empty path string",
                    "Replace the value with a repo-relative artifact path.",
                    ref=str(key),
                )
            )
            continue
        if not ref_exists(repo_root, raw_ref):
            findings.append(
                Finding(
                    "error",
                    f"{label}.missing_ref",
                    f"{label}.{key} does not resolve: {raw_ref}",
                    "Fix the path or add the referenced artifact in this change.",
                    ref=raw_ref,
                )
            )


def validate_path_list(
    repo_root: Path, refs: list[Any], label: str, findings: list[Finding]
) -> None:
    for idx, raw_ref in enumerate(refs):
        if not isinstance(raw_ref, str) or not raw_ref.strip():
            findings.append(
                Finding(
                    "error",
                    f"{label}.invalid_ref",
                    f"{label}[{idx}] must be a non-empty path string",
                    "Replace the value with a repo-relative artifact path.",
                )
            )
            continue
        if not ref_exists(repo_root, raw_ref):
            findings.append(
                Finding(
                    "error",
                    f"{label}.missing_ref",
                    f"{label}[{idx}] does not resolve: {raw_ref}",
                    "Fix the path or add the referenced artifact in this change.",
                    ref=raw_ref,
                )
            )


def collect_standards_by_id(standards: dict[str, Any]) -> dict[str, dict[str, Any]]:
    rows = ensure_list(standards.get("rows"), "standards.rows")
    out: dict[str, dict[str, Any]] = {}
    for idx, raw_row in enumerate(rows):
        row = ensure_dict(raw_row, f"standards.rows[{idx}]")
        row_id = ensure_str(row.get("id"), f"standards.rows[{idx}].id")
        out[row_id] = row
    return out


def collect_issue_classes_by_id(issue_routing: dict[str, Any]) -> dict[str, dict[str, Any]]:
    rows = ensure_list(issue_routing.get("issue_classes"), "issue_routing.issue_classes")
    out: dict[str, dict[str, Any]] = {}
    for idx, raw_row in enumerate(rows):
        row = ensure_dict(raw_row, f"issue_routing.issue_classes[{idx}]")
        row_id = ensure_str(row.get("id"), f"issue_routing.issue_classes[{idx}].id")
        out[row_id] = row
    return out


def collect_transition_ids(issue_routing: dict[str, Any]) -> set[str]:
    rows = ensure_list(
        issue_routing.get("disclosure_transitions"),
        "issue_routing.disclosure_transitions",
    )
    return {
        ensure_str(row.get("id"), f"issue_routing.disclosure_transitions[{idx}].id")
        for idx, row in enumerate(rows)
        if isinstance(row, dict)
    }


def markdown_row_for(body: str, first_cell: str) -> str | None:
    needle = f"`{first_cell}`"
    for line in body.splitlines():
        if line.startswith("|") and needle in line:
            return line
    return None


def validate_header(
    repo_root: Path, packet: dict[str, Any], findings: list[Finding]
) -> None:
    if ensure_int(packet.get("schema_version"), "packet.schema_version") != 1:
        findings.append(
            Finding(
                "error",
                "packet.schema_version.unsupported",
                "schema_version must be 1",
                "Update the validator in the same change that changes schema_version.",
            )
        )
    parse_iso_date(ensure_str(packet.get("as_of"), "packet.as_of"), "packet.as_of")
    for key in ("packet_id", "milestone_id", "release_channel_scope", "owner", "packet_state"):
        ensure_str(packet.get(key), f"packet.{key}")
    if packet.get("release_channel_scope") != "beta":
        findings.append(
            Finding(
                "error",
                "packet.release_channel_scope.not_beta",
                "release_channel_scope must be beta",
                "This packet is the beta publication baseline; use beta scope.",
            )
        )
    if packet.get("packet_state") != "frozen":
        findings.append(
            Finding(
                "error",
                "packet.packet_state.not_frozen",
                "packet_state must be frozen",
                "Freeze the packet once the beta publication baseline is reviewable.",
            )
        )

    validate_path_map(
        repo_root,
        ensure_dict(packet.get("human_entrypoints"), "packet.human_entrypoints"),
        "packet.human_entrypoints",
        findings,
    )
    validate_path_map(
        repo_root,
        ensure_dict(packet.get("canonical_sources"), "packet.canonical_sources"),
        "packet.canonical_sources",
        findings,
    )
    validator = ensure_dict(packet.get("validator"), "packet.validator")
    validate_path_map(
        repo_root,
        {"script_ref": validator.get("script_ref")},
        "packet.validator",
        findings,
    )
    ensure_str(validator.get("command"), "packet.validator.command")
    ensure_str(
        validator.get("validation_capture_ref"),
        "packet.validator.validation_capture_ref",
    )


def validate_standards_rows(
    repo_root: Path,
    packet: dict[str, Any],
    standards_by_id: dict[str, dict[str, Any]],
    findings: list[Finding],
) -> list[dict[str, Any]]:
    rows = ensure_list(packet.get("standards_rows"), "packet.standards_rows")
    seen: set[str] = set()
    summaries: list[dict[str, Any]] = []
    for idx, raw_row in enumerate(rows):
        row = ensure_dict(raw_row, f"packet.standards_rows[{idx}]")
        row_id = ensure_str(row.get("row_id"), f"packet.standards_rows[{idx}].row_id")
        seen.add(row_id)
        source = standards_by_id.get(row_id)
        if source is None:
            findings.append(
                Finding(
                    "error",
                    "packet.standards_rows.unknown_row",
                    f"standards row does not resolve: {row_id}",
                    "Use a row id from artifacts/governance/standards_matrix.yaml.",
                    ref=row_id,
                )
            )
            continue
        for packet_field, source_field in (
            ("source_support_class", "support_class"),
            ("source_import_expectation", "import_expectation"),
            ("source_export_expectation", "export_expectation"),
        ):
            packet_value = ensure_str(
                row.get(packet_field), f"packet.standards_rows[{idx}].{packet_field}"
            )
            source_value = ensure_str(source.get(source_field), f"{row_id}.{source_field}")
            if packet_value != source_value:
                findings.append(
                    Finding(
                        "error",
                        f"packet.standards_rows.{packet_field}.mismatch",
                        f"{row_id} {packet_field}={packet_value!r} does not match {source_field}={source_value!r}",
                        "Update the packet row or the standards matrix in the same change.",
                        ref=row_id,
                    )
                )
        posture = ensure_str(
            row.get("beta_claim_posture"),
            f"packet.standards_rows[{idx}].beta_claim_posture",
        )
        support_class = ensure_str(
            row.get("source_support_class"),
            f"packet.standards_rows[{idx}].source_support_class",
        )
        if support_class in DEFERRED_SUPPORT_CLASSES and posture in CLAIM_BEARING_POSTURES:
            findings.append(
                Finding(
                    "error",
                    "packet.standards_rows.deferred_claim_bearing",
                    f"{row_id} is deferred but beta_claim_posture is {posture}",
                    "Use a deferred posture or lift the standards row with evidence first.",
                    ref=row_id,
                )
            )
        if support_class in BRIDGE_SUPPORT_CLASSES and posture == "claim_bearing":
            findings.append(
                Finding(
                    "error",
                    "packet.standards_rows.bridge_claim_bearing",
                    f"{row_id} is bridge/custom but marked claim_bearing",
                    "Use bridge_only or bridge_reserved unless the standard row is lifted.",
                    ref=row_id,
                )
            )
        evidence_refs = ensure_list(
            row.get("evidence_refs"), f"packet.standards_rows[{idx}].evidence_refs"
        )
        if not evidence_refs:
            findings.append(
                Finding(
                    "error",
                    "packet.standards_rows.evidence_refs.empty",
                    f"{row_id} must cite at least one evidence ref",
                    "Cite the standards evidence fixture or governing docs for this row.",
                    ref=row_id,
                )
            )
        validate_path_list(
            repo_root,
            evidence_refs,
            f"packet.standards_rows[{idx}].evidence_refs",
            findings,
        )
        summaries.append(
            {
                "row_id": row_id,
                "source_support_class": support_class,
                "source_import_expectation": ensure_str(
                    row.get("source_import_expectation"),
                    f"packet.standards_rows[{idx}].source_import_expectation",
                ),
                "source_export_expectation": ensure_str(
                    row.get("source_export_expectation"),
                    f"packet.standards_rows[{idx}].source_export_expectation",
                ),
                "beta_claim_posture": posture,
                "public_claim_ceiling": ensure_str(
                    row.get("public_claim_ceiling"),
                    f"packet.standards_rows[{idx}].public_claim_ceiling",
                ),
            }
        )
    missing = set(standards_by_id) - seen
    extra = seen - set(standards_by_id)
    if missing:
        findings.append(
            Finding(
                "error",
                "packet.standards_rows.missing_matrix_rows",
                "packet does not publish every standards matrix row",
                "Add each missing standards row with an explicit beta posture.",
                details={"missing": sorted(missing)},
            )
        )
    if extra:
        findings.append(
            Finding(
                "error",
                "packet.standards_rows.extra_rows",
                "packet names standards rows absent from the matrix",
                "Remove the row or add it to the standards matrix first.",
                details={"extra": sorted(extra)},
            )
        )
    return summaries


def validate_issue_rows(
    packet: dict[str, Any],
    issue_classes_by_id: dict[str, dict[str, Any]],
    transition_ids: set[str],
    findings: list[Finding],
) -> list[dict[str, Any]]:
    rows = ensure_list(packet.get("issue_routing_rows"), "packet.issue_routing_rows")
    seen: set[str] = set()
    summaries: list[dict[str, Any]] = []
    fields = (
        "default_route_class",
        "privacy_class",
        "disclosure_class",
        "public_summary_expectation",
        "redaction_class",
    )
    for idx, raw_row in enumerate(rows):
        row = ensure_dict(raw_row, f"packet.issue_routing_rows[{idx}]")
        issue_id = ensure_str(
            row.get("issue_class_id"), f"packet.issue_routing_rows[{idx}].issue_class_id"
        )
        seen.add(issue_id)
        source = issue_classes_by_id.get(issue_id)
        if source is None:
            findings.append(
                Finding(
                    "error",
                    "packet.issue_routing_rows.unknown_issue_class",
                    f"issue class does not resolve: {issue_id}",
                    "Use an issue_classes[].id from artifacts/governance/issue_routing.yaml.",
                    ref=issue_id,
                )
            )
            continue
        for field_name in fields:
            packet_value = ensure_str(
                row.get(field_name), f"packet.issue_routing_rows[{idx}].{field_name}"
            )
            source_value = ensure_str(source.get(field_name), f"{issue_id}.{field_name}")
            if packet_value != source_value:
                findings.append(
                    Finding(
                        "error",
                        f"packet.issue_routing_rows.{field_name}.mismatch",
                        f"{issue_id} {field_name}={packet_value!r} does not match source {source_value!r}",
                        "Update the packet row or the canonical issue-routing row in the same change.",
                        ref=issue_id,
                    )
                )
        summaries.append(
            {
                "issue_class_id": issue_id,
                "default_route_class": row["default_route_class"],
                "privacy_class": row["privacy_class"],
            }
        )
    missing = set(issue_classes_by_id) - seen
    if missing:
        findings.append(
            Finding(
                "error",
                "packet.issue_routing_rows.missing_issue_classes",
                "packet does not publish every issue-routing class",
                "Add each issue class with its canonical route/privacy posture.",
                details={"missing": sorted(missing)},
            )
        )
    for raw_ref in ensure_list(
        packet.get("private_to_public_transition_refs"),
        "packet.private_to_public_transition_refs",
    ):
        ref = ensure_str(raw_ref, "packet.private_to_public_transition_refs[]")
        if ref not in transition_ids:
            findings.append(
                Finding(
                    "error",
                    "packet.private_to_public_transition_refs.unknown",
                    f"transition ref does not resolve: {ref}",
                    "Use a disclosure_transitions[].id from issue_routing.yaml.",
                    ref=ref,
                )
            )
    return summaries


def validate_docs(
    repo_root: Path,
    packet: dict[str, Any],
    standards_summaries: list[dict[str, Any]],
    issue_summaries: list[dict[str, Any]],
    findings: list[Finding],
) -> None:
    entrypoints = ensure_dict(packet.get("human_entrypoints"), "packet.human_entrypoints")
    standards_doc_ref = ensure_str(
        entrypoints.get("standards_interchange_matrix"),
        "packet.human_entrypoints.standards_interchange_matrix",
    )
    issue_doc_ref = ensure_str(
        entrypoints.get("issue_rfc_routing"), "packet.human_entrypoints.issue_rfc_routing"
    )
    standards_body = (repo_root / standards_doc_ref).read_text(encoding="utf-8")
    issue_body = (repo_root / issue_doc_ref).read_text(encoding="utf-8")
    packet_ref = "artifacts/milestones/m3/open_project_beta_packet.md"

    for summary in standards_summaries:
        row_id = summary["row_id"]
        standards_row = markdown_row_for(standards_body, row_id)
        if standards_row is None:
            findings.append(
                Finding(
                    "error",
                    "packet.docs.standards_missing_row",
                    f"standards entrypoint is missing {row_id}",
                    "Cite every packet standards row in the beta matrix page.",
                    ref=standards_doc_ref,
                )
            )
            continue
        for field_name in (
            "source_support_class",
            "source_import_expectation",
            "source_export_expectation",
        ):
            expected = summary[field_name]
            if f"`{expected}`" not in standards_row:
                findings.append(
                    Finding(
                        "error",
                        "packet.docs.standards_row_value_mismatch",
                        f"standards entrypoint row {row_id} is missing {field_name}={expected}",
                        "Update the beta standards matrix row to match the canonical packet.",
                        ref=standards_doc_ref,
                    )
                )
    if packet_ref not in standards_body:
        findings.append(
            Finding(
                "error",
                "packet.docs.standards_missing_packet_ref",
                "standards entrypoint does not cite the open-project beta packet",
                "Link the beta packet so reviewers can find the machine source.",
                ref=standards_doc_ref,
            )
        )

    for summary in issue_summaries:
        issue_id = summary["issue_class_id"]
        issue_row = markdown_row_for(issue_body, issue_id)
        if issue_row is None:
            findings.append(
                Finding(
                    "error",
                    "packet.docs.issue_missing_class",
                    f"issue/RFC entrypoint is missing {issue_id}",
                    "Cite every packet issue class in the beta routing page.",
                    ref=issue_doc_ref,
                )
            )
            continue
        for field_name in (
            "default_route_class",
            "privacy_class",
        ):
            expected = summary[field_name]
            if f"`{expected}`" not in issue_row:
                findings.append(
                    Finding(
                        "error",
                        "packet.docs.issue_row_value_mismatch",
                        f"issue/RFC entrypoint row {issue_id} is missing {field_name}={expected}",
                        "Update the beta issue/RFC routing row to match the canonical packet.",
                        ref=issue_doc_ref,
                    )
                )
    if packet_ref not in issue_body:
        findings.append(
            Finding(
                "error",
                "packet.docs.issue_missing_packet_ref",
                "issue/RFC entrypoint does not cite the open-project beta packet",
                "Link the beta packet so reviewers can find the machine source.",
                ref=issue_doc_ref,
            )
        )

    for idx, raw_consumer in enumerate(
        ensure_list(packet.get("consuming_surfaces"), "packet.consuming_surfaces")
    ):
        consumer = ensure_dict(raw_consumer, f"packet.consuming_surfaces[{idx}]")
        consumer_ref = ensure_str(
            consumer.get("consumer_ref"), f"packet.consuming_surfaces[{idx}].consumer_ref"
        )
        if not ref_exists(repo_root, consumer_ref):
            findings.append(
                Finding(
                    "error",
                    "packet.consuming_surfaces.consumer_ref.missing",
                    f"consumer ref does not resolve: {consumer_ref}",
                    "Add the consuming doc/help surface or fix the ref.",
                    ref=consumer_ref,
                )
            )
            continue
        body = (repo_root / consumer_ref).read_text(encoding="utf-8")
        for raw_term in ensure_list(
            consumer.get("required_terms", []),
            f"packet.consuming_surfaces[{idx}].required_terms",
        ):
            term = ensure_str(raw_term, f"packet.consuming_surfaces[{idx}].required_terms[]")
            if term not in body:
                findings.append(
                    Finding(
                        "error",
                        "packet.consuming_surfaces.required_term_missing",
                        f"{consumer_ref} is missing required term {term}",
                        "Mention the canonical issue/standards term on the consuming surface.",
                        ref=consumer_ref,
                    )
                )


def validate_starter_pack(
    packet: dict[str, Any], starter_pack: dict[str, Any], findings: list[Finding]
) -> None:
    issue_entrypoint = ensure_str(
        ensure_dict(packet.get("human_entrypoints"), "packet.human_entrypoints").get(
            "issue_rfc_routing"
        ),
        "packet.human_entrypoints.issue_rfc_routing",
    )
    entrypoints = ensure_dict(
        starter_pack.get("human_entrypoint_refs"),
        "starter_pack.human_entrypoint_refs",
    )
    if entrypoints.get("community") != issue_entrypoint:
        findings.append(
            Finding(
                "error",
                "starter_pack.community_entrypoint.mismatch",
                "starter pack community entrypoint does not point to the beta issue/RFC routing page",
                "Update artifacts/milestones/m3/beta_enablement_starter_pack.yaml.",
                ref="artifacts/milestones/m3/beta_enablement_starter_pack.yaml",
                details={
                    "expected": issue_entrypoint,
                    "actual": entrypoints.get("community"),
                },
            )
        )
    lanes = ensure_list(starter_pack.get("starter_pack_lanes"), "starter_pack.starter_pack_lanes")
    community_lanes = [
        lane for lane in lanes if isinstance(lane, dict) and lane.get("lane_id") == "starter_pack_lane:community"
    ]
    if len(community_lanes) != 1:
        findings.append(
            Finding(
                "error",
                "starter_pack.community_lane.count",
                "starter pack must have exactly one community lane",
                "Keep a single starter_pack_lane:community row.",
            )
        )
        return
    if community_lanes[0].get("human_entrypoint_ref") != issue_entrypoint:
        findings.append(
            Finding(
                "error",
                "starter_pack.community_lane.entrypoint.mismatch",
                "community lane does not consume the beta issue/RFC routing page",
                "Update the community lane human_entrypoint_ref.",
                ref="starter_pack_lane:community",
                details={
                    "expected": issue_entrypoint,
                    "actual": community_lanes[0].get("human_entrypoint_ref"),
                },
            )
        )


def write_capture(
    repo_root: Path,
    capture_rel: str,
    packet_rel: str,
    standards_summaries: list[dict[str, Any]],
    issue_summaries: list[dict[str, Any]],
    findings: list[Finding],
    generated_at: str,
    check_only: bool,
) -> bool:
    payload = {
        "schema_version": 1,
        "check_id": "m3_open_project_beta_packet",
        "generated_at": generated_at,
        "packet_ref": packet_rel,
        "status": "pass" if not any(f.severity == "error" for f in findings) else "fail",
        "finding_counts": {
            "error": sum(1 for f in findings if f.severity == "error"),
            "warning": sum(1 for f in findings if f.severity == "warning"),
        },
        "findings": [f.as_report() for f in findings],
        "standards_summary": standards_summaries,
        "issue_routing_summary": issue_summaries,
    }
    capture_path = repo_root / capture_rel
    capture_path.parent.mkdir(parents=True, exist_ok=True)
    new_text = json.dumps(payload, indent=2, sort_keys=True) + "\n"
    old_text = capture_path.read_text(encoding="utf-8") if capture_path.exists() else None
    changed = old_text is None or _normalize(old_text) != _normalize(new_text)
    if not check_only:
        capture_path.write_text(new_text, encoding="utf-8")
    return changed


_GENERATED_AT_RE = re.compile(r'"generated_at":\s*"[^"]*"')


def _normalize(text: str) -> str:
    return _GENERATED_AT_RE.sub('"generated_at": "__generated_at__"', text)


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(f"--repo-root does not look like a repository root: {repo_root}")

    packet_path = repo_root / args.packet
    if not packet_path.exists():
        raise SystemExit(f"missing packet: {packet_path}")
    packet = ensure_dict(
        render_yaml_as_json(
            extract_canonical_block(packet_path.read_text(encoding="utf-8")),
            args.packet,
        ),
        "packet",
    )
    standards = ensure_dict(render_yaml_file_as_json(repo_root / args.standards), "standards")
    issue_routing = ensure_dict(
        render_yaml_file_as_json(repo_root / args.issue_routing), "issue_routing"
    )
    starter_pack = ensure_dict(
        render_yaml_file_as_json(repo_root / args.starter_pack), "starter_pack"
    )

    findings: list[Finding] = []
    validate_header(repo_root, packet, findings)
    standards_summaries = validate_standards_rows(
        repo_root, packet, collect_standards_by_id(standards), findings
    )
    issue_summaries = validate_issue_rows(
        packet,
        collect_issue_classes_by_id(issue_routing),
        collect_transition_ids(issue_routing),
        findings,
    )
    validate_docs(repo_root, packet, standards_summaries, issue_summaries, findings)
    validate_starter_pack(packet, starter_pack, findings)

    generated_at = dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat()
    changed = write_capture(
        repo_root,
        args.capture,
        args.packet,
        standards_summaries,
        issue_summaries,
        findings,
        generated_at,
        args.check,
    )

    error_count = sum(1 for f in findings if f.severity == "error")
    warning_count = sum(1 for f in findings if f.severity == "warning")
    if args.check and changed:
        print(
            f"capture drift: {args.capture}; rerun without --check and commit the result",
            file=sys.stderr,
        )
        return 1
    if error_count:
        for finding in findings:
            if finding.severity == "error":
                print(f"error[{finding.check_id}]: {finding.message}", file=sys.stderr)
        return 1
    print(
        f"ok: validated {len(standards_summaries)} standards rows and "
        f"{len(issue_summaries)} issue-routing rows ({warning_count} warnings)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
