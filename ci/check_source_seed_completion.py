#!/usr/bin/env python3
# SPDX-FileCopyrightText: 2026 Aureline contributors
# SPDX-License-Identifier: Apache-2.0

"""Validate the source-appendix seed completion matrix.

This gate keeps the seed artifacts promised in the source design-doc appendices
under `.t2/docs/` mechanically traceable to concrete repo outputs (or explicit,
time-boxed waivers).

Exit code is 0 when:
- the matrix parses;
- tracked source-document digests match (or a fully governed source-drift
  waiver is active);
- every required seed-family row has at least one existing artifact ref or a
  fully governed, active waiver; and
- source documents satisfy the selected provisioning policy.
"""

from __future__ import annotations

import argparse
import copy
import datetime as dt
import hashlib
import json
import subprocess
import sys
import tempfile
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_MATRIX_REL = "artifacts/governance/source_seed_completion_matrix.yaml"

SENTINEL_REFS = {"not_yet_seeded", "outline_only", "contract_not_yet_seeded"}

SOURCE_DOCUMENT_POLICIES = ("required", "if-present")

WAIVER_REQUIRED_TEXT_FIELDS = (
    "waiver_id",
    "scope",
    "justification",
    "risk",
    "owner",
    "mitigation",
    "exit_plan",
)

WAIVER_APPROVAL_FORUMS = {
    "architecture_council",
    "performance_council",
    "security_trust_review",
    "accessibility_review",
    "compatibility_ecosystem_review",
    "product_scope_review",
    "release_council",
    "shiproom_executive_scope_review",
}


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
        "--source-doc-policy",
        choices=SOURCE_DOCUMENT_POLICIES,
        default="required",
        help=(
            "Use 'required' to require every source document, or 'if-present' "
            "to permit a clean checkout with none provisioned while still "
            "failing partial provisioning and validating all documents when present."
        ),
    )
    parser.add_argument(
        "--self-test",
        action="store_true",
        help="Run fail-closed waiver and source-document policy regression drills.",
    )
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


def parse_date(value: Any) -> dt.date | None:
    if (
        not isinstance(value, str)
        or len(value) != 10
        or value[4] != "-"
        or value[7] != "-"
    ):
        return None
    try:
        return dt.datetime.strptime(value, "%Y-%m-%d").date()
    except ValueError:
        return None


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


def is_non_empty_string(value: Any) -> bool:
    return isinstance(value, str) and bool(value.strip())


def repo_relative_path(repo_root: Path, ref: str) -> Path | None:
    """Resolve a repository ref without allowing absolute or escaping paths."""

    ref_path = Path(ref)
    if ref_path.is_absolute():
        return None
    resolved_root = repo_root.resolve()
    resolved = (resolved_root / ref_path).resolve(strict=False)
    try:
        resolved.relative_to(resolved_root)
    except ValueError:
        return None
    return resolved


def validate_waiver(
    waiver: Any,
    *,
    today: dt.date,
    check_id: str,
    row_ref: str | None,
) -> tuple[bool, list[Finding]]:
    """Validate one waiver and return whether it currently grants authority.

    A waiver is active only when every required governance field is present,
    opening and expiry chronology is coherent, approval is explicit, and the
    expiry window remains open. Any malformed or incomplete field therefore
    fails closed instead of falling back to the expiry date alone.
    """

    if not isinstance(waiver, dict):
        return (
            False,
            [
                Finding(
                    severity="error",
                    check_id=check_id,
                    message="waiver is incomplete, malformed, not yet approved, or expired",
                    remediation="Replace the waiver value with a complete governed waiver object.",
                    row_ref=row_ref,
                    details={"problems": ["waiver must be an object"]},
                )
            ],
        )

    problems: list[str] = []
    for field_name in WAIVER_REQUIRED_TEXT_FIELDS:
        if not is_non_empty_string(waiver.get(field_name)):
            problems.append(f"{field_name} must be a non-empty string")

    opened_on = parse_date(waiver.get("opened_on"))
    if opened_on is None:
        problems.append("opened_on must be a valid YYYY-MM-DD date")

    expires_on = parse_date(waiver.get("expires_on"))
    if expires_on is None:
        problems.append("expires_on must be a valid YYYY-MM-DD date")

    approval = waiver.get("approval")
    if not isinstance(approval, dict):
        problems.append("approval must be an object")
    else:
        approved_by = approval.get("approved_by")
        if (
            not isinstance(approved_by, list)
            or not approved_by
            or any(not is_non_empty_string(approver) for approver in approved_by)
        ):
            problems.append("approval.approved_by must be a non-empty list of non-empty strings")
        forum = approval.get("forum")
        if forum not in WAIVER_APPROVAL_FORUMS:
            problems.append(
                "approval.forum must name a recognized waiver-authority forum"
            )

    if opened_on is not None and expires_on is not None and opened_on > expires_on:
        problems.append("opened_on must not be after expires_on")
    if opened_on is not None and opened_on > today:
        problems.append("opened_on is in the future")
    if expires_on is not None and expires_on < today:
        problems.append(f"waiver expired on {expires_on.isoformat()}")

    if not problems:
        return True, []

    return (
        False,
        [
            Finding(
                severity="error",
                check_id=check_id,
                message="waiver is incomplete, malformed, not yet approved, or expired",
                remediation=(
                    "Provide waiver_id, scope, justification, risk, owner, mitigation, "
                    "opened_on, a complete approval (approved_by and forum), "
                    "expires_on, and exit_plan; then obtain a current approval."
                ),
                row_ref=row_ref,
                details={"problems": problems},
            )
        ],
    )


def artifact_ref_exists(repo_root: Path, ref: str) -> bool:
    if ref in SENTINEL_REFS:
        return False
    path = repo_relative_path(repo_root, ref)
    return path is not None and path.exists()


def validate_matrix(
    repo_root: Path,
    matrix: dict[str, Any],
    *,
    today: dt.date | None = None,
    source_doc_policy: str = "required",
) -> list[Finding]:
    if source_doc_policy not in SOURCE_DOCUMENT_POLICIES:
        raise ValueError(f"unknown source document policy: {source_doc_policy}")
    if today is None:
        today = dt.date.today()
    findings: list[Finding] = []

    source_documents = ensure_list(matrix.get("source_documents"), "source_documents")
    doc_by_id: dict[str, dict[str, Any]] = {}
    seen_doc_ids: set[str] = set()
    if not source_documents:
        findings.append(
            Finding(
                severity="error",
                check_id="source_document.inventory_empty",
                message=(
                    "source_documents must enumerate at least one authoritative source document"
                ),
                remediation="Restore the source-document inventory in the completion matrix.",
            )
        )
    for idx, doc in enumerate(source_documents):
        doc = ensure_dict(doc, f"source_documents[{idx}]")
        doc_id = ensure_str(doc.get("doc_id"), f"source_documents[{idx}].doc_id")
        doc_ref = ensure_str(doc.get("doc_ref"), f"source_documents[{idx}].doc_ref")
        sha256 = ensure_str(doc.get("sha256"), f"source_documents[{idx}].sha256")
        if doc_id in seen_doc_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="source_document.duplicate_id",
                    message=f"duplicate source document id: {doc_id}",
                    remediation="Give every source_documents row a unique doc_id.",
                    row_ref=doc_id,
                )
            )
        seen_doc_ids.add(doc_id)
        if len(sha256) != 64 or any(char not in "0123456789abcdef" for char in sha256):
            findings.append(
                Finding(
                    severity="error",
                    check_id="source_document.digest_invalid",
                    message=f"source document digest is not lowercase SHA-256: {doc_ref}",
                    remediation="Set sha256 to the exact 64-character lowercase digest.",
                    row_ref=doc_id,
                )
            )
        path = repo_relative_path(repo_root, doc_ref)
        if path is None:
            findings.append(
                Finding(
                    severity="error",
                    check_id="source_document.path_outside_repo",
                    message=f"source document path escapes the repository: {doc_ref}",
                    remediation="Use a repository-relative source document path.",
                    row_ref=doc_id,
                )
            )
        doc_by_id[doc_id] = {
            "doc_id": doc_id,
            "doc_ref": doc_ref,
            "sha256": sha256,
            "path": path,
        }

    drift_waiver = matrix.get("source_drift_waiver")
    drift_waiver_active = False
    if drift_waiver is not None:
        drift_waiver_active, waiver_findings = validate_waiver(
            drift_waiver,
            today=today,
            check_id="source_drift_waiver.expired_or_invalid",
            row_ref=None,
        )
        findings.extend(waiver_findings)

    digest_mismatches: list[tuple[str, str, str]] = []
    any_source_path_present = any(
        doc["path"] is not None
        and (doc["path"].exists() or doc["path"].is_symlink())
        for doc in doc_by_id.values()
    )
    no_source_docs_provisioned = bool(doc_by_id) and not any_source_path_present
    if source_doc_policy == "if-present" and no_source_docs_provisioned:
        findings.append(
            Finding(
                severity="warning",
                check_id="source_document.not_provisioned",
                message=(
                    "source documents are not provisioned in this checkout; "
                    "digest verification was skipped by the if-present policy"
                ),
                remediation=(
                    "Provision the ignored .t2/docs source pack and rerun with "
                    "--source-doc-policy required for authoritative local verification."
                ),
                details={"expected_doc_refs": [doc["doc_ref"] for doc in doc_by_id.values()]},
            )
        )

    for doc in doc_by_id.values():
        path = doc["path"]
        if path is None:
            continue
        if not path.exists():
            if source_doc_policy == "if-present" and no_source_docs_provisioned:
                continue
            findings.append(
                Finding(
                    severity="error",
                    check_id="source_document.missing",
                    message=f"tracked source document does not exist: {doc['doc_ref']}",
                    remediation=(
                        "Fix the doc_ref path, provision the complete source pack, or use "
                        "--source-doc-policy if-present only in a checkout where none of the "
                        "source documents are provisioned."
                    ),
                    row_ref=doc["doc_id"],
                )
            )
            continue
        if not path.is_file():
            findings.append(
                Finding(
                    severity="error",
                    check_id="source_document.not_a_file",
                    message=f"tracked source document is not a regular file: {doc['doc_ref']}",
                    remediation="Replace the path with the expected source-document file.",
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
    seen_seed_ids: set[str] = set()
    for idx, row in enumerate(seed_families):
        row = ensure_dict(row, f"seed_families[{idx}]")
        row_id = ensure_str(row.get("id"), f"seed_families[{idx}].id")
        ensure_str(row.get("label"), f"seed_families[{idx}].label")
        if row_id in seen_seed_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="seed_family.duplicate_id",
                    message=f"duplicate seed family id: {row_id}",
                    remediation="Give every seed_families row a unique id.",
                    row_ref=row_id,
                )
            )
        seen_seed_ids.add(row_id)

        required_value = row.get("required", False)
        if not isinstance(required_value, bool):
            findings.append(
                Finding(
                    severity="error",
                    check_id="seed_family.required_not_boolean",
                    message="seed family required must be a boolean",
                    remediation="Set required to true or false without quotes.",
                    row_ref=row_id,
                )
            )
        required = required_value is True
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
            if not appendices_list:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="seed_family.source_appendices_empty",
                        message="seed family row has no source appendix references",
                        remediation="Add at least one source appendix reference.",
                        row_ref=row_id,
                    )
                )

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
            ensure_str(appendix.get("anchor"), f"{row_id}.source_appendices[{aidx}].anchor")

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
            artifact_list = []
            for ref_idx, item in enumerate(
                ensure_list(artifact_refs, f"{row_id}.artifact_refs")
            ):
                if not is_non_empty_string(item):
                    findings.append(
                        Finding(
                            severity="error",
                            check_id="seed_family.artifact_ref_invalid",
                            message="artifact ref must be a non-empty string",
                            remediation="Use a repository-relative artifact path or a declared sentinel.",
                            row_ref=row_id,
                            details={"artifact_ref_index": ref_idx},
                        )
                    )
                    continue
                artifact_list.append(item)
                if item not in SENTINEL_REFS and repo_relative_path(repo_root, item) is None:
                    findings.append(
                        Finding(
                            severity="error",
                            check_id="seed_family.artifact_ref_outside_repo",
                            message=f"artifact ref escapes the repository: {item}",
                            remediation="Use a repository-relative artifact path.",
                            row_ref=row_id,
                        )
                    )

        any_artifact_exists = any(artifact_ref_exists(repo_root, ref) for ref in artifact_list)

        waiver = row.get("waiver")
        waiver_is_active = False
        if waiver is not None:
            waiver_is_active, waiver_findings = validate_waiver(
                waiver,
                today=today,
                check_id="seed_family.waiver_expired_or_invalid",
                row_ref=row_id,
            )
            findings.extend(waiver_findings)

        if required and not any_artifact_exists and not waiver_is_active:
            findings.append(
                Finding(
                    severity="error",
                    check_id="seed_family.missing_required_artifact",
                    message="required seed family has no existing artifact_refs and no active waiver",
                    remediation=(
                        "Add a checked-in artifact path under artifact_refs or add a complete, "
                        "approved, time-boxed waiver."
                    ),
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


def self_test_waiver() -> dict[str, Any]:
    return {
        "waiver_id": "waiver.source_seed.self_test",
        "scope": "Only the missing source-seed fixture in this regression drill.",
        "justification": "The fixture intentionally exercises the governed deferral path.",
        "risk": "The expected seed artifact is temporarily unavailable.",
        "owner": "@source-seed-test-owner",
        "mitigation": "Keep the affected claim narrowed while the artifact is absent.",
        "opened_on": "2026-07-01",
        "approval": {
            "approved_by": ["@source-seed-test-approver"],
            "forum": "architecture_council",
        },
        "expires_on": "2026-08-01",
        "exit_plan": "Check in the fixture and remove this waiver before expiry.",
    }


def run_self_tests() -> tuple[int, list[str]]:
    today = dt.date(2026, 7, 20)
    failures: list[str] = []
    drill_count = 0

    def expect(drill_id: str, condition: bool) -> None:
        nonlocal drill_count
        drill_count += 1
        if not condition:
            failures.append(drill_id)

    waiver = self_test_waiver()
    active, waiver_findings = validate_waiver(
        waiver,
        today=today,
        check_id="self_test.waiver_invalid",
        row_ref="self_test",
    )
    expect("complete_waiver_is_active", active and not waiver_findings)

    for field_name in (*WAIVER_REQUIRED_TEXT_FIELDS, "opened_on", "expires_on", "approval"):
        mutated = copy.deepcopy(waiver)
        mutated.pop(field_name)
        active, findings = validate_waiver(
            mutated,
            today=today,
            check_id="self_test.waiver_invalid",
            row_ref="self_test",
        )
        expect(f"missing_{field_name}_fails_closed", not active and bool(findings))

    for approval_field in ("approved_by", "forum"):
        mutated = copy.deepcopy(waiver)
        mutated["approval"].pop(approval_field)
        active, findings = validate_waiver(
            mutated,
            today=today,
            check_id="self_test.waiver_invalid",
            row_ref="self_test",
        )
        expect(f"missing_approval_{approval_field}_fails_closed", not active and bool(findings))

    malformed_expiry = copy.deepcopy(waiver)
    malformed_expiry["expires_on"] = "20260731"
    active, findings = validate_waiver(
        malformed_expiry,
        today=today,
        check_id="self_test.waiver_invalid",
        row_ref="self_test",
    )
    expect("malformed_expiry_fails_closed", not active and bool(findings))

    expired = copy.deepcopy(waiver)
    expired["expires_on"] = "2026-07-19"
    active, findings = validate_waiver(
        expired,
        today=today,
        check_id="self_test.waiver_invalid",
        row_ref="self_test",
    )
    expect("expired_waiver_fails_closed", not active and bool(findings))

    future_opening = copy.deepcopy(waiver)
    future_opening["opened_on"] = "2026-07-21"
    active, findings = validate_waiver(
        future_opening,
        today=today,
        check_id="self_test.waiver_invalid",
        row_ref="self_test",
    )
    expect("future_opening_fails_closed", not active and bool(findings))

    active, findings = validate_waiver(
        "expires_on: 2099-01-01",
        today=today,
        check_id="self_test.waiver_invalid",
        row_ref="self_test",
    )
    expect("non_object_waiver_fails_closed", not active and bool(findings))

    unknown_forum = copy.deepcopy(waiver)
    unknown_forum["approval"]["forum"] = "unrecorded_chat_approval"
    active, findings = validate_waiver(
        unknown_forum,
        today=today,
        check_id="self_test.waiver_invalid",
        row_ref="self_test",
    )
    expect("unknown_approval_forum_fails_closed", not active and bool(findings))

    with tempfile.TemporaryDirectory(prefix="aureline-source-seed-self-test-") as temp_dir:
        repo_root = Path(temp_dir)
        artifact_path = repo_root / "artifacts" / "seed.txt"
        artifact_path.parent.mkdir(parents=True)
        artifact_path.write_text("seed\n", encoding="utf-8")
        matrix: dict[str, Any] = {
            "source_documents": [
                {
                    "doc_id": "doc_a",
                    "doc_ref": ".t2/docs/doc_a.md",
                    "sha256": hashlib.sha256(b"doc a\n").hexdigest(),
                },
                {
                    "doc_id": "doc_b",
                    "doc_ref": ".t2/docs/doc_b.md",
                    "sha256": hashlib.sha256(b"doc b\n").hexdigest(),
                },
            ],
            "source_drift_waiver": None,
            "seed_families": [
                {
                    "id": "seed.self_test",
                    "label": "Self-test seed",
                    "required": True,
                    "owner_lane_id": "governance_packets",
                    "source_appendices": [
                        {
                            "doc_id": "doc_a",
                            "anchor": "#test-appendix",
                            "title": "Test appendix",
                        }
                    ],
                    "artifact_refs": ["artifacts/seed.txt"],
                    "waiver": None,
                }
            ],
        }

        findings = validate_matrix(
            repo_root,
            matrix,
            today=today,
            source_doc_policy="if-present",
        )
        check_ids = {finding.check_id for finding in findings}
        expect(
            "clean_checkout_if_present_warns_and_passes",
            check_ids == {"source_document.not_provisioned"}
            and not any(finding.severity == "error" for finding in findings),
        )

        findings = validate_matrix(
            repo_root,
            matrix,
            today=today,
            source_doc_policy="required",
        )
        expect(
            "clean_checkout_required_fails",
            any(finding.check_id == "source_document.missing" for finding in findings),
        )

        first_doc_path = repo_root / ".t2" / "docs" / "doc_a.md"
        first_doc_path.parent.mkdir(parents=True)
        first_doc_path.write_text("doc a\n", encoding="utf-8")
        findings = validate_matrix(
            repo_root,
            matrix,
            today=today,
            source_doc_policy="if-present",
        )
        expect(
            "partial_checkout_if_present_fails",
            any(finding.check_id == "source_document.missing" for finding in findings),
        )

        second_doc_path = repo_root / ".t2" / "docs" / "doc_b.md"
        second_doc_path.write_text("doc b\n", encoding="utf-8")
        findings = validate_matrix(
            repo_root,
            matrix,
            today=today,
            source_doc_policy="if-present",
        )
        expect(
            "fully_provisioned_if_present_verifies_digests",
            not findings,
        )

        incomplete_seed_waiver = copy.deepcopy(matrix)
        incomplete_seed_waiver["seed_families"][0]["artifact_refs"] = ["not_yet_seeded"]
        incomplete_seed_waiver["seed_families"][0]["waiver"] = {
            "expires_on": "2099-01-01"
        }
        findings = validate_matrix(
            repo_root,
            incomplete_seed_waiver,
            today=today,
            source_doc_policy="required",
        )
        check_ids = {finding.check_id for finding in findings}
        expect(
            "expiry_only_seed_waiver_cannot_satisfy_missing_artifact",
            {
                "seed_family.waiver_expired_or_invalid",
                "seed_family.missing_required_artifact",
            }.issubset(check_ids),
        )

        incomplete_drift_waiver = copy.deepcopy(matrix)
        incomplete_drift_waiver["source_documents"][0]["sha256"] = "0" * 64
        incomplete_drift_waiver["source_drift_waiver"] = {"expires_on": "2099-01-01"}
        findings = validate_matrix(
            repo_root,
            incomplete_drift_waiver,
            today=today,
            source_doc_policy="required",
        )
        errors_by_id = {
            finding.check_id: finding.severity
            for finding in findings
            if finding.severity == "error"
        }
        expect(
            "expiry_only_drift_waiver_cannot_narrow_digest_error",
            "source_drift_waiver.expired_or_invalid" in errors_by_id
            and "source_document.digest_mismatch" in errors_by_id,
        )

        escaping_artifact = copy.deepcopy(matrix)
        escaping_artifact["seed_families"][0]["artifact_refs"] = ["../outside.txt"]
        findings = validate_matrix(
            repo_root,
            escaping_artifact,
            today=today,
            source_doc_policy="required",
        )
        check_ids = {finding.check_id for finding in findings}
        expect(
            "artifact_ref_cannot_escape_repository",
            {
                "seed_family.artifact_ref_outside_repo",
                "seed_family.missing_required_artifact",
            }.issubset(check_ids),
        )

    return drill_count, failures


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
    if args.self_test:
        drill_count, failures = run_self_tests()
        if failures:
            for drill_id in failures:
                print(f"[source-seed-completion-self-test] FAIL: {drill_id}", file=sys.stderr)
            return 1
        print(f"[source-seed-completion-self-test] PASS: {drill_count} regression drills")
        return 0

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
    evaluated_on = dt.date.today()
    findings = validate_matrix(
        repo_root,
        matrix,
        today=evaluated_on,
        source_doc_policy=args.source_doc_policy,
    )

    sys.stdout.write(render_human_summary(findings))

    if args.report:
        report_path = repo_root / args.report
        report_path.parent.mkdir(parents=True, exist_ok=True)
        report_payload = {
            "matrix": matrix_rel,
            "evaluated_on": evaluated_on.isoformat(),
            "source_doc_policy": args.source_doc_policy,
            "findings": [finding.as_report() for finding in findings],
        }
        report_path.write_text(json.dumps(report_payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    return 1 if any(f.severity == "error" for f in findings) else 0


if __name__ == "__main__":
    raise SystemExit(main())
