#!/usr/bin/env python3
"""Validate the known-gaps ledger, blocker taxonomy, and exit-gate checklist.

This check keeps the milestone proof foundation executable by ensuring the
canonical artifacts:

- exist and parse;
- carry stable schema/owner headers;
- use a shared taxonomy for blocker classes; and
- link ledger rows to exit-gate checklist items so exceptions cannot drift into
  untracked prose.

It also validates the proof-index consumer so downstream packets and dashboards
have one canonical join point.
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


DEFAULT_LEDGER_REL = "artifacts/milestones/m1/known_gaps_ledger.yaml"
DEFAULT_TAXONOMY_REL = "artifacts/milestones/m1/blocker_taxonomy.yaml"
DEFAULT_EXIT_GATE_REL = "artifacts/milestones/m1/exit_gate_checklist.yaml"
DEFAULT_INDEX_REL = "artifacts/milestones/m1/artifact_index.yaml"


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
    parser.add_argument("--ledger", default=DEFAULT_LEDGER_REL)
    parser.add_argument("--taxonomy", default=DEFAULT_TAXONOMY_REL)
    parser.add_argument("--exit-gate", dest="exit_gate", default=DEFAULT_EXIT_GATE_REL)
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
    ref = strip_fragment(ref)
    return bool(ref) and (repo_root / ref).exists()


def validate_header(payload: dict[str, Any], label: str, findings: list[Finding]) -> None:
    schema_version = ensure_int(payload.get("schema_version"), f"{label}.schema_version")
    if schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.schema_version.unsupported",
                message=f"unsupported schema_version {schema_version} (expected 1)",
                remediation="Bump the validator and document the new schema before changing schema_version.",
            )
        )

    as_of = ensure_str(payload.get("as_of"), f"{label}.as_of")
    parse_iso_date(as_of, f"{label}.as_of")

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


def validate_taxonomy(taxonomy: dict[str, Any], findings: list[Finding]) -> set[str]:
    validate_header(taxonomy, "taxonomy", findings)

    ensure_str(taxonomy.get("taxonomy_id"), "taxonomy.taxonomy_id")
    severity_vocab = ensure_list(taxonomy.get("severity_vocabulary"), "taxonomy.severity_vocabulary")
    if not severity_vocab:
        findings.append(
            Finding(
                severity="error",
                check_id="taxonomy.severity_vocabulary.empty",
                message="severity_vocabulary must not be empty",
                remediation="List the allowed severity values so ledger rows remain consistent.",
            )
        )

    waiver_vocab = ensure_list(taxonomy.get("waiver_status_vocabulary"), "taxonomy.waiver_status_vocabulary")
    if "none" not in waiver_vocab:
        findings.append(
            Finding(
                severity="error",
                check_id="taxonomy.waiver_status_vocabulary.missing_none",
                message="waiver_status_vocabulary must include 'none'",
                remediation="Include 'none' so rows without waivers stay explicit and machine-readable.",
            )
        )

    classes = ensure_list(taxonomy.get("blocker_classes"), "taxonomy.blocker_classes")
    class_ids: list[str] = []
    for idx, raw_row in enumerate(classes):
        row = ensure_dict(raw_row, f"taxonomy.blocker_classes[{idx}]")
        class_id = ensure_str(row.get("class_id"), f"taxonomy.blocker_classes[{idx}].class_id")
        ensure_str(row.get("title"), f"taxonomy.blocker_classes[{idx}].title")
        ensure_str(row.get("description"), f"taxonomy.blocker_classes[{idx}].description")
        class_ids.append(class_id)

    required = {
        "source_fidelity",
        "hot_path",
        "recovery",
        "trust",
        "accessibility",
        "onboarding",
        "boundary_truth",
        "public_proof",
    }
    missing = sorted(required.difference(class_ids))
    if missing:
        findings.append(
            Finding(
                severity="error",
                check_id="taxonomy.blocker_classes.missing_required",
                message=f"blocker_classes is missing required class ids: {', '.join(missing)}",
                remediation="Add the missing blocker class rows so the ledger has a stable taxonomy.",
            )
        )

    duplicates = sorted({cid for cid in class_ids if class_ids.count(cid) > 1})
    for dup in duplicates:
        findings.append(
            Finding(
                severity="error",
                check_id="taxonomy.blocker_classes.duplicate",
                message=f"blocker class id is duplicated: {dup}",
                remediation="Deduplicate blocker class ids so joins remain stable.",
                ref=dup,
            )
        )

    return set(class_ids)


def validate_exit_gate(repo_root: Path, exit_gate: dict[str, Any], findings: list[Finding]) -> set[str]:
    validate_header(exit_gate, "exit_gate", findings)

    ensure_str(exit_gate.get("checklist_id"), "exit_gate.checklist_id")
    human_entry = ensure_str(exit_gate.get("human_entrypoint_ref"), "exit_gate.human_entrypoint_ref")
    if not artifact_ref_exists(repo_root, human_entry):
        findings.append(
            Finding(
                severity="error",
                check_id="exit_gate.human_entrypoint_ref.missing",
                message=f"human_entrypoint_ref does not exist: {human_entry}",
                remediation="Fix the path so reviewers have a stable landing page.",
                ref=human_entry,
            )
        )

    inputs = ensure_dict(exit_gate.get("inputs"), "exit_gate.inputs")
    for key in (
        "known_gaps_ledger_ref",
        "blocker_taxonomy_ref",
        "dependency_graph_ref",
        "dogfood_matrix_ref",
        "build_identity_ref",
    ):
        ref = ensure_str(inputs.get(key), f"exit_gate.inputs.{key}")
        if not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="exit_gate.inputs.missing_ref",
                    message=f"exit-gate input ref does not exist: {ref}",
                    remediation="Fix the ref so the exit-gate checklist can be joined to real artifacts.",
                    ref=ref,
                    details={"input_key": key},
                )
            )

    status_vocab = ensure_list(exit_gate.get("status_vocabulary"), "exit_gate.status_vocabulary")
    if "unknown" not in status_vocab or "green" not in status_vocab:
        findings.append(
            Finding(
                severity="error",
                check_id="exit_gate.status_vocabulary.required",
                message="status_vocabulary must include at least 'unknown' and 'green'",
                remediation="Include the minimum required statuses so tooling can interpret the checklist.",
            )
        )

    items = ensure_list(exit_gate.get("items"), "exit_gate.items")
    item_ids: list[str] = []
    for idx, raw_item in enumerate(items):
        item = ensure_dict(raw_item, f"exit_gate.items[{idx}]")
        item_id = ensure_str(item.get("item_id"), f"exit_gate.items[{idx}].item_id")
        ensure_str(item.get("title"), f"exit_gate.items[{idx}].title")
        status = ensure_str(item.get("status"), f"exit_gate.items[{idx}].status")
        if status not in status_vocab:
            findings.append(
                Finding(
                    severity="error",
                    check_id="exit_gate.item.status.invalid",
                    message=f"exit-gate item has invalid status {status!r}",
                    remediation="Use one of the statuses listed in status_vocabulary.",
                    ref=item_id,
                    details={"status": status},
                )
            )
        ensure_str(item.get("definition_of_green"), f"exit_gate.items[{idx}].definition_of_green")
        evidence_refs = ensure_list(item.get("evidence_refs"), f"exit_gate.items[{idx}].evidence_refs")
        if not evidence_refs:
            findings.append(
                Finding(
                    severity="error",
                    check_id="exit_gate.item.evidence_refs.empty",
                    message="exit-gate item evidence_refs must not be empty",
                    remediation="List at least one canonical artifact ref used to judge the item.",
                    ref=item_id,
                )
            )
        item_ids.append(item_id)

    duplicates = sorted({iid for iid in item_ids if item_ids.count(iid) > 1})
    for dup in duplicates:
        findings.append(
            Finding(
                severity="error",
                check_id="exit_gate.item_id.duplicate",
                message=f"exit-gate item_id is duplicated: {dup}",
                remediation="Deduplicate item ids so joins remain stable.",
                ref=dup,
            )
        )

    return set(item_ids)


def validate_ledger(
    repo_root: Path,
    ledger: dict[str, Any],
    taxonomy_ids: set[str],
    exit_gate_item_ids: set[str],
    waiver_vocab: set[str],
    severity_vocab: set[str],
    findings: list[Finding],
) -> None:
    validate_header(ledger, "ledger", findings)

    ensure_str(ledger.get("ledger_id"), "ledger.ledger_id")
    human_entry = ensure_str(ledger.get("human_entrypoint_ref"), "ledger.human_entrypoint_ref")
    if not artifact_ref_exists(repo_root, human_entry):
        findings.append(
            Finding(
                severity="error",
                check_id="ledger.human_entrypoint_ref.missing",
                message=f"human_entrypoint_ref does not exist: {human_entry}",
                remediation="Fix the path so reviewers have a stable landing page.",
                ref=human_entry,
            )
        )

    inputs = ensure_dict(ledger.get("inputs"), "ledger.inputs")
    for key in (
        "blocker_taxonomy_ref",
        "exit_gate_checklist_ref",
        "dependency_graph_ref",
        "dogfood_matrix_ref",
        "build_identity_ref",
    ):
        ref = ensure_str(inputs.get(key), f"ledger.inputs.{key}")
        if not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="ledger.inputs.missing_ref",
                    message=f"ledger input ref does not exist: {ref}",
                    remediation="Fix the ref so ledger rows can be joined to real artifacts.",
                    ref=ref,
                    details={"input_key": key},
                )
            )

    row_kind_vocab = {ensure_str(v, "ledger.row_kind_vocabulary[]") for v in ensure_list(ledger.get("row_kind_vocabulary"), "ledger.row_kind_vocabulary")}
    row_status_vocab = {ensure_str(v, "ledger.row_status_vocabulary[]") for v in ensure_list(ledger.get("row_status_vocabulary"), "ledger.row_status_vocabulary")}

    rows = ensure_list(ledger.get("rows"), "ledger.rows")
    for idx, raw_row in enumerate(rows):
        row = ensure_dict(raw_row, f"ledger.rows[{idx}]")
        row_id = ensure_str(row.get("row_id"), f"ledger.rows[{idx}].row_id")
        kind = ensure_str(row.get("kind"), f"ledger.rows[{idx}].kind")
        if kind not in row_kind_vocab:
            findings.append(
                Finding(
                    severity="error",
                    check_id="ledger.row.kind.invalid",
                    message=f"ledger row has invalid kind {kind!r}",
                    remediation="Use one of the values in row_kind_vocabulary.",
                    ref=row_id,
                )
            )
        status = ensure_str(row.get("status"), f"ledger.rows[{idx}].status")
        if status not in row_status_vocab:
            findings.append(
                Finding(
                    severity="error",
                    check_id="ledger.row.status.invalid",
                    message=f"ledger row has invalid status {status!r}",
                    remediation="Use one of the values in row_status_vocabulary.",
                    ref=row_id,
                )
            )
        owner = ensure_str(row.get("owner_dri"), f"ledger.rows[{idx}].owner_dri")
        if not owner.startswith("@"):
            findings.append(
                Finding(
                    severity="warning",
                    check_id="ledger.row.owner.format",
                    message=f"ledger row owner_dri does not look like a handle: {owner!r}",
                    remediation="Use an @handle so routing is explicit.",
                    ref=row_id,
                )
            )
        severity = ensure_str(row.get("severity"), f"ledger.rows[{idx}].severity")
        if severity not in severity_vocab:
            findings.append(
                Finding(
                    severity="error",
                    check_id="ledger.row.severity.invalid",
                    message=f"ledger row severity is not in taxonomy severity_vocabulary: {severity!r}",
                    remediation="Use one of the severity values listed in blocker_taxonomy.yaml.",
                    ref=row_id,
                )
            )
        waiver_status = ensure_str(row.get("waiver_status"), f"ledger.rows[{idx}].waiver_status")
        if waiver_status not in waiver_vocab:
            findings.append(
                Finding(
                    severity="error",
                    check_id="ledger.row.waiver_status.invalid",
                    message=f"ledger row waiver_status is not in taxonomy waiver_status_vocabulary: {waiver_status!r}",
                    remediation="Use one of the waiver status values listed in blocker_taxonomy.yaml.",
                    ref=row_id,
                )
            )
        ensure_str(row.get("next_action"), f"ledger.rows[{idx}].next_action")
        blocker_class = ensure_str(row.get("blocker_class"), f"ledger.rows[{idx}].blocker_class")
        if blocker_class not in taxonomy_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="ledger.row.blocker_class.unknown",
                    message=f"ledger row blocker_class is not in blocker taxonomy: {blocker_class!r}",
                    remediation="Use a class_id from blocker_taxonomy.yaml or extend the taxonomy first.",
                    ref=row_id,
                    details={"blocker_class": blocker_class},
                )
            )

        blocked_items = ensure_list(row.get("blocked_exit_gate_items"), f"ledger.rows[{idx}].blocked_exit_gate_items")
        if not blocked_items:
            findings.append(
                Finding(
                    severity="error",
                    check_id="ledger.row.blocked_exit_gate_items.empty",
                    message="ledger row must list at least one blocked_exit_gate_items entry",
                    remediation="Add the exit-gate checklist item id(s) this row blocks so exit review cannot silently ignore it.",
                    ref=row_id,
                )
            )
        for blocked in blocked_items:
            if not isinstance(blocked, str) or not blocked.strip():
                findings.append(
                    Finding(
                        severity="error",
                        check_id="ledger.row.blocked_exit_gate_items.invalid",
                        message="blocked_exit_gate_items entries must be non-empty strings",
                        remediation="Replace the empty entry with an exit-gate checklist item id.",
                        ref=row_id,
                    )
                )
                continue
            blocked = blocked.strip()
            if blocked not in exit_gate_item_ids:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="ledger.row.blocked_exit_gate_items.unknown",
                        message=f"ledger row references unknown exit-gate item id: {blocked}",
                        remediation="Fix the id or add the missing exit-gate item to exit_gate_checklist.yaml.",
                        ref=row_id,
                        details={"exit_gate_item_id": blocked},
                    )
                )

        for key in ("fixture_ref", "build_identity_ref"):
            ref = row.get(key)
            if not isinstance(ref, str) or not ref.strip():
                findings.append(
                    Finding(
                        severity="error",
                        check_id="ledger.row.required_ref.missing",
                        message=f"ledger row is missing required ref field: {key}",
                        remediation="Set the ref to a repo-relative path so the row remains traceable.",
                        ref=row_id,
                        details={"missing_field": key},
                    )
                )
                continue
            ref = ref.strip()
            if not artifact_ref_exists(repo_root, ref):
                findings.append(
                    Finding(
                        severity="error",
                        check_id="ledger.row.required_ref.missing_target",
                        message=f"ledger row {key} does not exist: {ref}",
                        remediation="Fix the ref path (or seed the referenced artifact) so the row stays traceable.",
                        ref=row_id,
                        details={"field": key, "ref": ref},
                    )
                )


def validate_index(repo_root: Path, index: dict[str, Any], ledger_rel: str, findings: list[Finding]) -> None:
    validate_header(index, "index", findings)

    canonical_artifacts = ensure_list(index.get("canonical_artifacts"), "index.canonical_artifacts")
    refs_by_id: dict[str, str] = {}
    for idx, raw_row in enumerate(canonical_artifacts):
        row = ensure_dict(raw_row, f"index.canonical_artifacts[{idx}]")
        artifact_id = ensure_str(row.get("artifact_id"), f"index.canonical_artifacts[{idx}].artifact_id")
        artifact_ref = ensure_str(row.get("artifact_ref"), f"index.canonical_artifacts[{idx}].artifact_ref")
        refs_by_id[artifact_id] = artifact_ref

    expected = {
        "known_gaps_ledger": ledger_rel,
        "blocker_taxonomy": DEFAULT_TAXONOMY_REL,
        "exit_gate_checklist": DEFAULT_EXIT_GATE_REL,
        "known_gaps_validator": "ci/check_m1_known_gaps.py",
        "known_gaps_entrypoint": "docs/milestones/m1/known_gaps.md",
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


def write_report(repo_root: Path, report_rel: str, ledger_rel: str, findings: list[Finding]) -> None:
    report_path = repo_root / report_rel
    report_path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": 1,
        "check_id": "m1_known_gaps_ledger",
        "generated_at": dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
        "ledger_ref": ledger_rel,
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

    ledger_rel = args.ledger
    taxonomy_rel = args.taxonomy
    exit_gate_rel = args.exit_gate
    index_rel = args.index

    ledger = ensure_dict(render_yaml_as_json(repo_root / ledger_rel), "ledger")
    taxonomy = ensure_dict(render_yaml_as_json(repo_root / taxonomy_rel), "taxonomy")
    exit_gate = ensure_dict(render_yaml_as_json(repo_root / exit_gate_rel), "exit_gate")
    index = ensure_dict(render_yaml_as_json(repo_root / index_rel), "index")

    findings: list[Finding] = []

    taxonomy_class_ids = validate_taxonomy(taxonomy, findings)
    exit_gate_item_ids = validate_exit_gate(repo_root, exit_gate, findings)

    waiver_vocab = set(ensure_list(taxonomy.get("waiver_status_vocabulary"), "taxonomy.waiver_status_vocabulary"))
    severity_vocab = set(ensure_list(taxonomy.get("severity_vocabulary"), "taxonomy.severity_vocabulary"))
    validate_ledger(
        repo_root,
        ledger,
        taxonomy_ids=taxonomy_class_ids,
        exit_gate_item_ids=exit_gate_item_ids,
        waiver_vocab=waiver_vocab,
        severity_vocab=severity_vocab,
        findings=findings,
    )
    validate_index(repo_root, index, ledger_rel=ledger_rel, findings=findings)

    if args.report:
        write_report(repo_root, args.report, ledger_rel=ledger_rel, findings=findings)

    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        suffix = f" ({finding.ref})" if finding.ref else ""
        print(f"{prefix}: {finding.check_id}: {finding.message}{suffix}")

    return 1 if any(f.severity == "error" for f in findings) else 0


if __name__ == "__main__":
    raise SystemExit(main())

