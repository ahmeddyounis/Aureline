#!/usr/bin/env python3
"""Lint the source-anchor map.

This tool keeps canonical requirement ids, control-artifact rows, and
governance packet families anchored to the source documents under
`.t2/docs/`. It is intentionally stdlib-only for control flow and uses
Ruby's built-in Psych parser for YAML decoding (macOS and GitHub-hosted
Ubuntu both ship Ruby by default). Running the tool from the repository
root is the expected entry point.

Checks performed:

  - source_authority_requires_anchor:
    Rows whose class declares ``claims_source_authority`` MUST carry at
    least one canonical anchor.
  - source_authority_posture_matches_class:
    A row may not set ``claims_source_authority=true`` unless its class
    allows it, and ``evidence_label`` rows MUST have
    ``claims_source_authority=false``.
  - unique_anchor_ids / unique_alias_ids:
    Every anchor id and every alias id is unique across the whole map,
    and alias ids do not collide with anchor ids.
  - unknown_source_document / missing_source_document_file:
    Every ``doc_id`` resolves to a declared source document whose
    ``doc_ref`` exists on disk.
  - requirement_id_coverage / orphaned_requirement_row:
    Every canonical requirement id in the register maps to exactly one
    ``requirement_row`` here, and every ``requirement_row`` here
    resolves to one register row.
  - class_coverage_gap:
    Each ``claims_source_authority`` class has at least one row.

Exit code is 0 when every check passes and 1 when any finding is at
severity ``error``.
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from collections import Counter, defaultdict
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any

SOURCE_ANCHOR_MAP_REL = "artifacts/governance/source_anchor_map.yaml"
REQUIREMENT_REGISTER_REL = "artifacts/governance/requirement_register_seed.yaml"
CANONICAL_REFERENCE_RULES_REL = "docs/governance/canonical_reference_rules.md"
DRIFT_REPORT_DIR_REL = "artifacts/governance/source_drift_reports/"
DRIFT_REPORT_TEMPLATE_REL = "artifacts/governance/source_drift_reports/template.md"

AUTHORITATIVE_CLASSES = {
    "requirement_row",
    "control_artifact",
    "packet_family",
    "shiproom_artifact",
}


@dataclass
class Finding:
    severity: str
    check_id: str
    severity_id: str
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


def load_json(path: Path) -> Any:
    with path.open("rb") as fh:
        return json.load(fh)


def finding(
    check_id: str,
    severity_id: str,
    message: str,
    remediation: str,
    row_ref: str | None = None,
    **details: Any,
) -> Finding:
    return Finding(
        severity="error",
        check_id=check_id,
        severity_id=severity_id,
        message=message,
        remediation=remediation,
        row_ref=row_ref,
        details=details,
    )


def duplicates(values: list[str]) -> list[str]:
    return sorted([value for value, count in Counter(values).items() if count > 1])


def load_scenario(repo_root: Path, scenario: Path | None) -> dict[str, Any] | None:
    if scenario is None:
        return None
    path = scenario if scenario.is_absolute() else repo_root / scenario
    if not path.exists():
        raise SystemExit(f"scenario file does not exist: {path}")
    payload = load_json(path)
    if not isinstance(payload, dict):
        raise SystemExit(f"scenario file must contain a JSON object: {path}")
    return payload


def apply_scenario(
    source_map: dict[str, Any], scenario: dict[str, Any] | None
) -> dict[str, Any]:
    """Return a copy of ``source_map`` with scenario overrides applied.

    Scenario keys:

    - ``rows``: replace the entire rows list.
    - ``append_rows``: append rows after the existing ones.
    - ``override_row``: a row dict keyed by ``anchor_id`` whose fields
      shallow-replace the matching on-disk row.
    - ``remove_anchor_ids``: a list of anchor_ids to drop from the map.
    - ``canonical_source_documents``: replace the source-document list.
    - ``register_requirement_ids``: replace the set of canonical
      requirement ids used for the orphan/coverage check.
    """
    if scenario is None:
        return source_map
    result = {key: list(value) if isinstance(value, list) else value for key, value in source_map.items()}
    rows = [dict(row) for row in source_map.get("rows", [])]

    if "rows" in scenario and isinstance(scenario["rows"], list):
        rows = [dict(row) for row in scenario["rows"]]

    remove_ids = set(scenario.get("remove_anchor_ids") or [])
    if remove_ids:
        rows = [row for row in rows if row.get("anchor_id") not in remove_ids]

    override = scenario.get("override_row")
    if isinstance(override, dict) and override.get("anchor_id"):
        target = override["anchor_id"]
        rows = [{**row, **override} if row.get("anchor_id") == target else row for row in rows]

    append_rows = scenario.get("append_rows")
    if isinstance(append_rows, list):
        rows.extend(dict(row) for row in append_rows)

    result["rows"] = rows

    if "canonical_source_documents" in scenario and isinstance(
        scenario["canonical_source_documents"], list
    ):
        result["canonical_source_documents"] = list(scenario["canonical_source_documents"])

    return result


def collect_register_requirement_ids(
    repo_root: Path, scenario: dict[str, Any] | None
) -> set[str]:
    if scenario and isinstance(scenario.get("register_requirement_ids"), list):
        return {str(rid) for rid in scenario["register_requirement_ids"]}

    register_path = repo_root / REQUIREMENT_REGISTER_REL
    if not register_path.exists():
        return set()
    register = render_yaml_as_json(register_path)
    if not isinstance(register, dict):
        return set()
    rows = register.get("requirement_rows")
    if not isinstance(rows, list):
        return set()
    return {
        str(row["requirement_id"])
        for row in rows
        if isinstance(row, dict) and row.get("requirement_id")
    }


def check_source_documents(
    repo_root: Path, source_map: dict[str, Any]
) -> tuple[dict[str, dict[str, Any]], list[Finding]]:
    findings: list[Finding] = []
    docs: dict[str, dict[str, Any]] = {}
    for entry in source_map.get("canonical_source_documents", []):
        if not isinstance(entry, dict):
            continue
        doc_id = entry.get("doc_id")
        doc_ref = entry.get("doc_ref")
        if not doc_id or not doc_ref:
            continue
        docs[doc_id] = entry
        path = repo_root / doc_ref
        if not path.exists():
            findings.append(
                finding(
                    check_id="source_anchor_map.missing_source_document_file",
                    severity_id="missing_source_document_file",
                    message=f"canonical source document '{doc_id}' points at missing file '{doc_ref}'",
                    remediation="Restore the source document or remove the row if it was retired.",
                    row_ref=doc_id,
                    doc_ref=doc_ref,
                )
            )
    return docs, findings


def check_authority_and_anchors(
    rows: list[dict[str, Any]],
    class_postures: dict[str, str],
    known_doc_ids: set[str],
) -> list[Finding]:
    findings: list[Finding] = []
    for row in rows:
        anchor_id = row.get("anchor_id", "<missing>")
        artifact_class = row.get("artifact_class")
        authority_posture = class_postures.get(artifact_class, "")
        claims_authority = bool(row.get("claims_source_authority"))
        anchors = row.get("canonical_anchors") or []

        if authority_posture == "no_source_authority" and claims_authority:
            findings.append(
                finding(
                    check_id="source_anchor_map.source_authority_posture_matches_class",
                    severity_id="source_authority_without_anchor",
                    message=(
                        f"row '{anchor_id}' is in class '{artifact_class}' but sets claims_source_authority=true"
                    ),
                    remediation=(
                        "Move the row to a class whose authority_posture is claims_source_authority, "
                        "or set claims_source_authority=false."
                    ),
                    row_ref=anchor_id,
                )
            )

        if authority_posture == "claims_source_authority":
            if not claims_authority:
                findings.append(
                    finding(
                        check_id="source_anchor_map.source_authority_posture_matches_class",
                        severity_id="source_authority_without_anchor",
                        message=(
                            f"row '{anchor_id}' is in class '{artifact_class}' (claims_source_authority) "
                            "but sets claims_source_authority=false without a documented downgrade"
                        ),
                        remediation=(
                            "Either move the row to the evidence_label class, or restore "
                            "claims_source_authority=true and add a canonical anchor."
                        ),
                        row_ref=anchor_id,
                    )
                )
            if claims_authority and not anchors:
                findings.append(
                    finding(
                        check_id="source_anchor_map.source_authority_requires_anchor",
                        severity_id="missing_canonical_anchor",
                        message=(
                            f"row '{anchor_id}' claims source authority but carries no canonical_anchors"
                        ),
                        remediation=(
                            "Cite at least one source-document anchor that authorizes the row, "
                            "or move the row to the evidence_label class."
                        ),
                        row_ref=anchor_id,
                    )
                )

        for anchor in anchors:
            if not isinstance(anchor, dict):
                continue
            doc_id = anchor.get("doc_id")
            if doc_id and doc_id not in known_doc_ids:
                findings.append(
                    finding(
                        check_id="source_anchor_map.unknown_source_document",
                        severity_id="unknown_source_document",
                        message=(
                            f"row '{anchor_id}' cites doc_id '{doc_id}' that is not declared in "
                            "canonical_source_documents"
                        ),
                        remediation=(
                            "Add the missing doc_id to canonical_source_documents, or correct the anchor."
                        ),
                        row_ref=anchor_id,
                        doc_id=doc_id,
                    )
                )

    return findings


def check_identifier_uniqueness(rows: list[dict[str, Any]]) -> list[Finding]:
    findings: list[Finding] = []
    anchor_ids = [row.get("anchor_id") for row in rows if row.get("anchor_id")]
    for duplicate in duplicates(anchor_ids):
        findings.append(
            finding(
                check_id="source_anchor_map.unique_anchor_ids",
                severity_id="duplicate_alias",
                message=f"anchor_id '{duplicate}' appears on more than one row",
                remediation="Rename one of the rows so every anchor_id is unique across the map.",
                row_ref=duplicate,
            )
        )

    alias_pairs: list[tuple[str, str]] = []
    for row in rows:
        for alias in row.get("aliases") or []:
            if not isinstance(alias, dict):
                continue
            alias_id = alias.get("alias_id")
            if alias_id:
                alias_pairs.append((alias_id, row.get("anchor_id") or "<missing>"))

    alias_counter = Counter(alias_id for alias_id, _ in alias_pairs)
    for duplicate, count in alias_counter.items():
        if count <= 1:
            continue
        homes = sorted({home for alias_id, home in alias_pairs if alias_id == duplicate})
        findings.append(
            finding(
                check_id="source_anchor_map.unique_alias_ids",
                severity_id="duplicate_alias",
                message=(
                    f"alias_id '{duplicate}' is carried by more than one row: {', '.join(homes)}"
                ),
                remediation="Aliases are lookup handles; each one must resolve to exactly one canonical row.",
                row_ref=duplicate,
            )
        )

    anchor_id_set = set(anchor_ids)
    for alias_id, home in alias_pairs:
        if alias_id in anchor_id_set and not any(
            row.get("anchor_id") == home and alias_id == home for row in rows
        ):
            findings.append(
                finding(
                    check_id="source_anchor_map.unique_alias_ids",
                    severity_id="duplicate_alias",
                    message=(
                        f"alias_id '{alias_id}' on row '{home}' collides with another row's anchor_id"
                    ),
                    remediation=(
                        "Rename the alias so it does not collide with any anchor_id in the map."
                    ),
                    row_ref=home,
                )
            )
    return findings


def check_alias_targets(rows: list[dict[str, Any]]) -> list[Finding]:
    findings: list[Finding] = []
    class_by_anchor = {row.get("anchor_id"): row.get("artifact_class") for row in rows}
    for row in rows:
        host = row.get("anchor_id") or "<missing>"
        for alias in row.get("aliases") or []:
            if not isinstance(alias, dict):
                continue
            alias_id = alias.get("alias_id")
            if not alias_id:
                continue
            if alias_id in class_by_anchor and class_by_anchor[alias_id] == "evidence_label":
                findings.append(
                    finding(
                        check_id="source_anchor_map.alias_target_posture",
                        severity_id="alias_without_canonical_target",
                        message=(
                            f"alias '{alias_id}' on row '{host}' points at an evidence_label row; "
                            "aliases must resolve to an authoritative row"
                        ),
                        remediation=(
                            "Route the alias to a requirement_row, control_artifact, packet_family, "
                            "or shiproom_artifact that actually holds source authority."
                        ),
                        row_ref=host,
                        alias_id=alias_id,
                    )
                )
    return findings


def check_requirement_coverage(
    rows: list[dict[str, Any]], register_requirement_ids: set[str]
) -> list[Finding]:
    findings: list[Finding] = []
    mapped_ids: list[str] = []
    for row in rows:
        if row.get("artifact_class") != "requirement_row":
            continue
        subject_ref = row.get("subject_ref")
        if subject_ref:
            mapped_ids.append(str(subject_ref))

    mapped_counter = Counter(mapped_ids)
    for duplicate, count in mapped_counter.items():
        if count > 1:
            findings.append(
                finding(
                    check_id="source_anchor_map.requirement_id_coverage",
                    severity_id="duplicate_alias",
                    message=(
                        f"requirement id '{duplicate}' is claimed by {count} requirement_row rows"
                    ),
                    remediation="Keep one requirement_row per canonical requirement id; fold duplicates into aliases.",
                    row_ref=duplicate,
                )
            )

    mapped_set = set(mapped_ids)
    for orphan in sorted(register_requirement_ids - mapped_set):
        findings.append(
            finding(
                check_id="source_anchor_map.requirement_id_coverage",
                severity_id="orphaned_requirement_id",
                message=(
                    f"canonical requirement id '{orphan}' has no matching requirement_row in the map"
                ),
                remediation=(
                    "Add a requirement_row with subject_kind=canonical_requirement_id and a canonical anchor."
                ),
                row_ref=orphan,
            )
        )

    for stray in sorted(mapped_set - register_requirement_ids):
        findings.append(
            finding(
                check_id="source_anchor_map.orphaned_requirement_row",
                severity_id="orphaned_requirement_id",
                message=(
                    f"requirement_row '{stray}' does not resolve to any canonical requirement id in the register"
                ),
                remediation=(
                    "Add the id to the requirement register first, or remove the row from the map."
                ),
                row_ref=stray,
            )
        )

    return findings


def check_class_coverage(
    rows: list[dict[str, Any]], class_postures: dict[str, str]
) -> list[Finding]:
    findings: list[Finding] = []
    counts: dict[str, int] = defaultdict(int)
    for row in rows:
        artifact_class = row.get("artifact_class")
        if artifact_class:
            counts[artifact_class] += 1
    for class_id, posture in class_postures.items():
        if posture != "claims_source_authority":
            continue
        if counts.get(class_id, 0) == 0:
            findings.append(
                finding(
                    check_id="source_anchor_map.class_coverage_gap",
                    severity_id="class_coverage_gap",
                    message=(
                        f"class '{class_id}' declares claims_source_authority but has zero rows in the map"
                    ),
                    remediation=(
                        "Add at least one row for the class, or remove the class from artifact_classes "
                        "if it is no longer in scope."
                    ),
                    row_ref=class_id,
                )
            )
    return findings


def class_postures_from(source_map: dict[str, Any]) -> dict[str, str]:
    postures: dict[str, str] = {}
    for entry in source_map.get("artifact_classes") or []:
        if not isinstance(entry, dict):
            continue
        class_id = entry.get("class_id")
        posture = entry.get("authority_posture")
        if class_id and posture:
            postures[class_id] = posture
    return postures


def class_coverage_counts(rows: list[dict[str, Any]]) -> dict[str, int]:
    counts: dict[str, int] = defaultdict(int)
    for row in rows:
        artifact_class = row.get("artifact_class")
        if artifact_class:
            counts[artifact_class] += 1
    return dict(counts)


def render_summary(findings: list[Finding], analysis: dict[str, Any]) -> str:
    lines: list[str] = []
    lines.append("source-anchor map linter")
    lines.append("=========================")
    lines.append(f"rows: {analysis['row_count']}")
    lines.append(f"canonical source documents: {analysis['source_document_count']}")
    lines.append("class coverage:")
    for class_id, count in sorted(analysis["class_coverage"].items()):
        lines.append(f"  - {class_id}: {count}")
    lines.append("")
    if not findings:
        lines.append("PASS — no findings")
    else:
        lines.append(f"{len(findings)} finding(s):")
        for item in findings:
            ref = f" [{item.row_ref}]" if item.row_ref else ""
            lines.append(f"  - {item.severity.upper()} {item.check_id}{ref}: {item.message}")
            lines.append(f"      remediation: {item.remediation}")
    return "\n".join(lines) + "\n"


def lint(
    repo_root: Path, scenario: dict[str, Any] | None
) -> tuple[list[Finding], dict[str, Any]]:
    findings: list[Finding] = []

    map_path = repo_root / SOURCE_ANCHOR_MAP_REL
    if not map_path.exists():
        raise SystemExit(f"source-anchor map missing at {SOURCE_ANCHOR_MAP_REL}")

    source_map = render_yaml_as_json(map_path)
    if not isinstance(source_map, dict):
        raise SystemExit(f"source-anchor map at {SOURCE_ANCHOR_MAP_REL} must be a mapping")

    source_map = apply_scenario(source_map, scenario)

    for field_name in (
        "map_id",
        "overview_doc",
        "drift_report_dir",
        "drift_report_template_ref",
    ):
        if not source_map.get(field_name):
            findings.append(
                finding(
                    check_id=f"source_anchor_map.required_field.{field_name}",
                    severity_id="missing_canonical_anchor",
                    message=f"source-anchor map is missing required field '{field_name}'",
                    remediation="Populate the field so downstream tooling has a stable entry point.",
                )
            )

    overview_ref = source_map.get("overview_doc")
    if overview_ref and not (repo_root / overview_ref).exists():
        findings.append(
            finding(
                check_id="source_anchor_map.overview_doc_exists",
                severity_id="missing_source_document_file",
                message=f"overview_doc points at missing file '{overview_ref}'",
                remediation="Point overview_doc at the canonical-reference rules document.",
            )
        )

    template_ref = source_map.get("drift_report_template_ref")
    if template_ref and not (repo_root / template_ref).exists():
        findings.append(
            finding(
                check_id="source_anchor_map.drift_report_template_exists",
                severity_id="missing_source_document_file",
                message=f"drift_report_template_ref points at missing file '{template_ref}'",
                remediation="Point drift_report_template_ref at the drift-report template.",
            )
        )

    drift_dir = source_map.get("drift_report_dir")
    if drift_dir and not (repo_root / drift_dir).exists():
        findings.append(
            finding(
                check_id="source_anchor_map.drift_report_dir_exists",
                severity_id="missing_source_document_file",
                message=f"drift_report_dir points at missing directory '{drift_dir}'",
                remediation="Create the drift-report directory or adjust the path.",
            )
        )

    docs, doc_findings = check_source_documents(repo_root, source_map)
    findings.extend(doc_findings)

    rows = source_map.get("rows") or []
    class_postures = class_postures_from(source_map)
    known_doc_ids = set(docs.keys())

    findings.extend(check_authority_and_anchors(rows, class_postures, known_doc_ids))
    findings.extend(check_identifier_uniqueness(rows))
    findings.extend(check_alias_targets(rows))
    findings.extend(check_class_coverage(rows, class_postures))

    register_requirement_ids = collect_register_requirement_ids(repo_root, scenario)
    findings.extend(check_requirement_coverage(rows, register_requirement_ids))

    analysis = {
        "map_id": source_map.get("map_id"),
        "row_count": len(rows),
        "source_document_count": len(docs),
        "class_coverage": class_coverage_counts(rows),
        "register_requirement_id_count": len(register_requirement_ids),
        "findings": [item.as_report() for item in findings],
    }
    return findings, analysis


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument(
        "--report",
        default=None,
        help="Write the machine-readable JSON report to this repo-relative path.",
    )
    parser.add_argument(
        "--scenario",
        default=None,
        help="Optional JSON scenario that overrides the on-disk map for deterministic failing examples.",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    scenario_path = Path(args.scenario) if args.scenario else None
    scenario = load_scenario(repo_root, scenario_path)

    findings, analysis = lint(repo_root, scenario)
    sys.stdout.write(render_summary(findings, analysis))

    if args.report:
        report_path = repo_root / args.report
        report_path.parent.mkdir(parents=True, exist_ok=True)
        report_path.write_text(json.dumps(analysis, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    return 1 if any(item.severity == "error" for item in findings) else 0


if __name__ == "__main__":
    raise SystemExit(main())
