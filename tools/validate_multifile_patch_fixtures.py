#!/usr/bin/env python3
from __future__ import annotations

import json
import pathlib
import sys
from dataclasses import dataclass
from typing import Any, Iterable

import yaml
from jsonschema import Draft202012Validator


@dataclass(frozen=True)
class FixtureIndex:
    review_summary: dict[str, Any] | None
    validation_summaries: list[dict[str, Any]]
    approvals: list[dict[str, Any]]
    apply_audits: list[dict[str, Any]]


def load_schema(repo_root: pathlib.Path) -> Draft202012Validator:
    schema_path = repo_root / "schemas/ai/patch_review_summary.schema.json"
    schema = json.loads(schema_path.read_text(encoding="utf-8"))
    return Draft202012Validator(schema)


def iter_fixture_docs(path: pathlib.Path) -> Iterable[dict[str, Any]]:
    for doc in yaml.safe_load_all(path.read_text(encoding="utf-8")):
        if doc is None:
            continue
        if not isinstance(doc, dict):
            continue
        if "__fixture__" in doc:
            continue
        yield doc


def index_fixture(docs: list[dict[str, Any]]) -> FixtureIndex:
    review_summary = None
    validation_summaries: list[dict[str, Any]] = []
    approvals: list[dict[str, Any]] = []
    apply_audits: list[dict[str, Any]] = []

    for doc in docs:
        kind = doc.get("record_kind")
        if kind == "patch_review_summary_record":
            review_summary = doc
        elif kind == "patch_validation_summary_record":
            validation_summaries.append(doc)
        elif kind == "patch_review_approval_record":
            approvals.append(doc)
        elif kind == "patch_apply_audit_record":
            apply_audits.append(doc)

    return FixtureIndex(
        review_summary=review_summary,
        validation_summaries=validation_summaries,
        approvals=approvals,
        apply_audits=apply_audits,
    )


def fail(path: pathlib.Path, message: str) -> None:
    raise ValueError(f"{path}: {message}")


def require(condition: bool, path: pathlib.Path, message: str) -> None:
    if not condition:
        fail(path, message)


def validate_cross_doc_invariants(path: pathlib.Path, index: FixtureIndex) -> None:
    require(index.review_summary is not None, path, "missing patch_review_summary_record")
    review_summary = index.review_summary
    review_id = review_summary["patch_review_summary_id"]
    proposal_state = review_summary["proposal_state"]
    patch_digest = review_summary["patch_artifact_digest"]
    hunk_rows = review_summary.get("patch_hunk_rows") or []
    hunk_ids = {row["hunk_id"] for row in hunk_rows if isinstance(row, dict) and "hunk_id" in row}

    for vs in index.validation_summaries:
        require(vs.get("patch_review_summary_id_ref") == review_id, path, "validation summary points at wrong review id")
        if proposal_state == "proposal_ready_for_review":
            require(vs.get("patch_artifact_digest") == patch_digest, path, "validation summary digest mismatch")

    approval_by_id = {row.get("patch_review_approval_id"): row for row in index.approvals}
    for approval in index.approvals:
        require(
            approval.get("patch_review_summary_id_ref") == review_id,
            path,
            "approval record points at wrong review id",
        )
        scope = approval.get("approval_scope")
        approved_hunks = approval.get("approved_hunk_ids")
        if scope == "hunk_allowlist":
            require(isinstance(approved_hunks, list) and approved_hunks, path, "hunk_allowlist approval missing hunks")
            require(set(approved_hunks).issubset(hunk_ids), path, "approved_hunk_ids not subset of reviewed hunks")
        if scope == "all_hunks":
            require(approved_hunks is None, path, "all_hunks approval must set approved_hunk_ids: null")

    for audit in index.apply_audits:
        require(audit.get("patch_review_summary_id_ref") == review_id, path, "apply audit points at wrong review id")
        if proposal_state == "proposal_ready_for_review":
            require(audit.get("patch_artifact_digest") == patch_digest, path, "apply audit digest mismatch")

        approval_id = audit.get("patch_review_approval_id_ref")
        require(approval_id in approval_by_id, path, "apply audit cites missing approval id")
        approval = approval_by_id[approval_id]

        applied_hunks = audit.get("applied_hunk_ids") or []
        unapproved_hunks = audit.get("unapproved_hunk_ids") or []
        require(set(applied_hunks).issubset(hunk_ids), path, "apply audit applied_hunk_ids not subset of reviewed hunks")
        require(
            set(unapproved_hunks).issubset(hunk_ids),
            path,
            "apply audit unapproved_hunk_ids not subset of reviewed hunks",
        )

        approval_scope = approval.get("approval_scope")
        approved_allowlist = approval.get("approved_hunk_ids")
        if approval_scope == "hunk_allowlist":
            require(
                set(applied_hunks).issubset(set(approved_allowlist or [])),
                path,
                "apply audit applied_hunk_ids exceeds approved allowlist",
            )
        if approval_scope == "all_hunks":
            require(
                set(applied_hunks).issubset(hunk_ids),
                path,
                "apply audit applied_hunk_ids exceeds reviewed hunk set",
            )

        if audit.get("apply_outcome") == "applied_success":
            require(not unapproved_hunks, path, "applied_success must not report unapproved hunks")


def main() -> int:
    repo_root = pathlib.Path(__file__).resolve().parents[1]
    fixture_dir = repo_root / "fixtures/ai/multifile_patch_cases"
    schema_path = repo_root / "schemas/ai/patch_review_summary.schema.json"

    if not schema_path.exists():
        print(f"Missing schema: {schema_path.relative_to(repo_root)}", file=sys.stderr)
        return 2

    if not fixture_dir.exists():
        print(f"Missing fixtures directory: {fixture_dir.relative_to(repo_root)}", file=sys.stderr)
        return 2

    fixture_paths = sorted(fixture_dir.glob("*.yaml"))
    if not fixture_paths:
        print(f"No fixtures found under {fixture_dir.relative_to(repo_root)}", file=sys.stderr)
        return 2

    validator = load_schema(repo_root)
    error_count = 0

    for fixture_path in fixture_paths:
        docs = list(iter_fixture_docs(fixture_path))
        for doc in docs:
            errors = sorted(validator.iter_errors(doc), key=lambda e: list(e.path))
            if not errors:
                continue
            error_count += len(errors)
            print(f"{fixture_path.relative_to(repo_root)}:")
            for error in errors:
                location = ".".join(str(part) for part in error.path) or "<root>"
                print(f"  - {location}: {error.message}")

        try:
            validate_cross_doc_invariants(fixture_path.relative_to(repo_root), index_fixture(docs))
        except Exception as exc:
            error_count += 1
            print(f"{fixture_path.relative_to(repo_root)}:", file=sys.stderr)
            print(f"  - cross-doc: {exc}", file=sys.stderr)

    if error_count:
        print(f"\nFAILED: {error_count} validation error(s)", file=sys.stderr)
        return 1

    print(
        "OK: validated "
        f"{len(fixture_paths)} multi-file patch fixture(s) against {schema_path.relative_to(repo_root)}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

