#!/usr/bin/env python3
"""Validate the source-appendix seed completion matrix.

This gate keeps the seed artifacts promised in the source design-doc appendices
under `.t2/docs/` mechanically traceable to concrete repo outputs (or explicit,
time-boxed waivers).

Exit code is 0 when:
- the matrix parses;
- tracked source-document digests match (or a source-drift waiver is active); and
- every required seed-family row has at least one existing artifact ref or an
  active waiver.
"""

from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_MATRIX_REL = "artifacts/governance/source_seed_completion_matrix.yaml"

SENTINEL_REFS = {"not_yet_seeded", "outline_only", "contract_not_yet_seeded"}


@dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    remediation: str
    row_ref: str | None = None
    details: dict[str, Any] = field(default_factory=dict)

    def as_report(self) -> dict[str, Any]:
        payload = asdict(self)
        if payload["row_ref"] is None:
            payload.pop("row_ref")
        if not payload["details"]:
            payload.pop("details")
        return payload


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--matrix", default=DEFAULT_MATRIX_REL)
    parser.add_argument(
        "--report",
        default=None,
        help="Write a machine-readable JSON report to this repo-relative path.",
    )
    return parser.parse_args()


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


def sha256_hex(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as fh:
        for chunk in iter(lambda: fh.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def parse_date(value: str) -> dt.date:
    try:
        return dt.date.fromisoformat(value)
    except ValueError as exc:
        raise SystemExit(f"expected YYYY-MM-DD date, got {value!r}") from exc


def waiver_active(waiver: dict[str, Any], today: dt.date) -> bool:
    expires_on = waiver.get("expires_on")
    if not isinstance(expires_on, str) or not expires_on:
        return False
    return today <= parse_date(expires_on)


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
    return value


def artifact_ref_exists(repo_root: Path, ref: str) -> bool:
    if ref in SENTINEL_REFS:
        return False
    return (repo_root / ref).exists()


def validate_matrix(repo_root: Path, matrix: dict[str, Any]) -> list[Finding]:
    today = dt.date.today()
    findings: list[Finding] = []

    source_documents = ensure_list(matrix.get("source_documents"), "source_documents")
    doc_by_id: dict[str, dict[str, Any]] = {}
    for idx, doc in enumerate(source_documents):
        doc = ensure_dict(doc, f"source_documents[{idx}]")
        doc_id = ensure_str(doc.get("doc_id"), f"source_documents[{idx}].doc_id")
        doc_ref = ensure_str(doc.get("doc_ref"), f"source_documents[{idx}].doc_ref")
        sha256 = ensure_str(doc.get("sha256"), f"source_documents[{idx}].sha256")
        doc_by_id[doc_id] = {"doc_id": doc_id, "doc_ref": doc_ref, "sha256": sha256}

    drift_waiver = matrix.get("source_drift_waiver")
    drift_waiver_active = False
    if drift_waiver is not None:
        drift_waiver = ensure_dict(drift_waiver, "source_drift_waiver")
        drift_waiver_active = waiver_active(drift_waiver, today)
        if not drift_waiver_active:
            findings.append(
                Finding(
                    severity="error",
                    check_id="source_drift_waiver.expired_or_invalid",
                    message="source_drift_waiver is present but expired or invalid",
                    remediation="Update expires_on to a future date or set source_drift_waiver to null.",
                )
            )

    digest_mismatches: list[tuple[str, str, str]] = []
    for doc in doc_by_id.values():
        path = repo_root / doc["doc_ref"]
        if not path.exists():
            findings.append(
                Finding(
                    severity="error",
                    check_id="source_document.missing",
                    message=f"tracked source document does not exist: {doc['doc_ref']}",
                    remediation="Fix the doc_ref path or restore the source document.",
                    row_ref=doc["doc_id"],
                )
            )
            continue
        actual = sha256_hex(path)
        expected = doc["sha256"]
        if actual != expected:
            digest_mismatches.append((doc["doc_id"], doc["doc_ref"], actual))

    if digest_mismatches:
        severity = "warning" if drift_waiver_active else "error"
        remediation = (
            "Refresh source_documents[].sha256 to match the new source docs (or add a time-boxed source_drift_waiver)."
            if not drift_waiver_active
            else "Refresh source_documents[].sha256 and remove the source_drift_waiver once the matrix is updated."
        )
        for doc_id, doc_ref, actual in digest_mismatches:
            findings.append(
                Finding(
                    severity=severity,
                    check_id="source_document.digest_mismatch",
                    message=f"source document digest changed: {doc_ref}",
                    remediation=remediation,
                    row_ref=doc_id,
                    details={"expected_sha256": doc_by_id[doc_id]["sha256"], "actual_sha256": actual},
                )
            )

    seed_families = ensure_list(matrix.get("seed_families"), "seed_families")
    for idx, row in enumerate(seed_families):
        row = ensure_dict(row, f"seed_families[{idx}]")
        row_id = ensure_str(row.get("id"), f"seed_families[{idx}].id")
        ensure_str(row.get("label"), f"seed_families[{idx}].label")

        required = bool(row.get("required", False))
        owner_lane_id = row.get("owner_lane_id")
        if not isinstance(owner_lane_id, str) or not owner_lane_id.strip():
            findings.append(
                Finding(
                    severity="error",
                    check_id="seed_family.owner_missing",
                    message="seed family row is missing owner_lane_id",
                    remediation="Set owner_lane_id to an ownership_matrix lane id.",
                    row_ref=row_id,
                )
            )

        appendices = row.get("source_appendices")
        if appendices is None:
            findings.append(
                Finding(
                    severity="error",
                    check_id="seed_family.source_appendices_missing",
                    message="seed family row is missing source_appendices",
                    remediation="Add at least one source appendix reference (doc_id + title).",
                    row_ref=row_id,
                )
            )
            appendices_list: list[Any] = []
        else:
            appendices_list = ensure_list(appendices, f"{row_id}.source_appendices")

        for aidx, appendix in enumerate(appendices_list):
            appendix = ensure_dict(appendix, f"{row_id}.source_appendices[{aidx}]")
            doc_id = ensure_str(appendix.get("doc_id"), f"{row_id}.source_appendices[{aidx}].doc_id")
            if doc_id not in doc_by_id:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="seed_family.unknown_doc_id",
                        message=f"unknown doc_id {doc_id!r} in source_appendices",
                        remediation="Add the doc_id under source_documents[] or fix the reference.",
                        row_ref=row_id,
                        details={"doc_id": doc_id},
                    )
                )
            ensure_str(appendix.get("title"), f"{row_id}.source_appendices[{aidx}].title")

        artifact_refs = row.get("artifact_refs")
        if artifact_refs is None:
            findings.append(
                Finding(
                    severity="error",
                    check_id="seed_family.artifact_refs_missing",
                    message="seed family row is missing artifact_refs",
                    remediation="Add at least one artifact_refs entry (or a waiver if deferred).",
                    row_ref=row_id,
                )
            )
            artifact_list: list[str] = []
        else:
            artifact_list = [str(item) for item in ensure_list(artifact_refs, f"{row_id}.artifact_refs")]

        any_artifact_exists = any(artifact_ref_exists(repo_root, ref) for ref in artifact_list)

        waiver = row.get("waiver")
        waiver_is_active = False
        if waiver is not None:
            waiver = ensure_dict(waiver, f"{row_id}.waiver")
            waiver_is_active = waiver_active(waiver, today)
            if not waiver_is_active:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="seed_family.waiver_expired_or_invalid",
                        message="waiver is present but expired or invalid",
                        remediation="Update expires_on, or remove the waiver and supply an artifact ref.",
                        row_ref=row_id,
                    )
                )

        if required and not any_artifact_exists and not waiver_is_active:
            findings.append(
                Finding(
                    severity="error",
                    check_id="seed_family.missing_required_artifact",
                    message="required seed family has no existing artifact_refs and no active waiver",
                    remediation="Add a checked-in artifact path under artifact_refs or add a time-boxed waiver.",
                    row_ref=row_id,
                    details={"artifact_refs": artifact_list},
                )
            )
        elif required and not any_artifact_exists and waiver_is_active:
            findings.append(
                Finding(
                    severity="warning",
                    check_id="seed_family.waived",
                    message="required seed family is missing but covered by an active waiver",
                    remediation="Deliver the missing artifact and remove the waiver before expiry.",
                    row_ref=row_id,
                )
            )

        if row.get("gap") is not None and any_artifact_exists:
            findings.append(
                Finding(
                    severity="warning",
                    check_id="seed_family.gap_declared_but_artifact_exists",
                    message="row declares a gap, but at least one artifact_refs entry exists",
                    remediation="Remove the gap block or update artifact_refs to match the intended canonical home.",
                    row_ref=row_id,
                )
            )

    return findings


def render_human_summary(findings: list[Finding]) -> str:
    lines: list[str] = []
    errors = [f for f in findings if f.severity == "error"]
    warnings = [f for f in findings if f.severity == "warning"]
    status = "PASS" if not errors else "FAIL"
    lines.append(
        f"[source-seed-completion] {status}: {len(errors)} error(s), {len(warnings)} warning(s)\n"
    )

    for finding in findings:
        row = f" ({finding.row_ref})" if finding.row_ref else ""
        lines.append(f"- {finding.severity.upper():7} {finding.check_id}{row}: {finding.message}\n")
        if finding.remediation:
            lines.append(f"          remediation: {finding.remediation}\n")
    return "".join(lines)


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    matrix_rel = args.matrix
    matrix_path = (repo_root / matrix_rel).resolve()

    if not matrix_path.exists():
        sys.stdout.write(
            render_human_summary(
                [
                    Finding(
                        severity="error",
                        check_id="matrix.missing",
                        message=f"matrix file does not exist: {matrix_rel}",
                        remediation="Check in the matrix at the canonical path.",
                    )
                ]
            )
        )
        return 1

    matrix = render_yaml_as_json(matrix_path)
    matrix = ensure_dict(matrix, "matrix")
    findings = validate_matrix(repo_root, matrix)

    sys.stdout.write(render_human_summary(findings))

    if args.report:
        report_path = repo_root / args.report
        report_path.parent.mkdir(parents=True, exist_ok=True)
        report_payload = {
            "matrix": matrix_rel,
            "findings": [finding.as_report() for finding in findings],
        }
        report_path.write_text(json.dumps(report_payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    return 1 if any(f.severity == "error" for f in findings) else 0


if __name__ == "__main__":
    raise SystemExit(main())

