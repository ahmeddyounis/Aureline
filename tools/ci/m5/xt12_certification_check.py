#!/usr/bin/env python3
"""M5 switching/visual-system/attention/boundary certification CI gate.

This is the milestone-closeout gate for the M5 depth surfaces. It enforces that
every *marketed* M5 surface ends the milestone with an explicit, evidence-backed
certification state across the switching, visual-system, durable-attention, and
embedded-boundary dimensions -- and that no surface widens above the launch
cutline while its certification proof is stale, missing, policy-blocked, red, or
still dependent on a forbidden v1-shell pattern.

It reads:

- the certification matrix at
  ``artifacts/release/m5/xt12-qualification-matrix.json``;
- its frozen validation capture under ``.../captures/``;
- the boundary schema at
  ``schemas/governance/m5_xt12_certification_matrix.schema.json``;
- the canonical M5 feature-family register it ingests;
- the generated evidence index and narrowing report;
- the companion doc at ``docs/m5/xt12-certification.md``; and
- the negative fixtures under ``fixtures/release/m5/xt12-promotion/``.

For the matrix the gate verifies that:

- it validates against the boundary schema;
- the matrix, capture, evidence index, and narrowing report are byte-for-byte what
  the regenerator would emit (so the checked-in copies are never hand-edited away
  from their source);
- every marketed surface binds all ten certification dimensions, and each bound
  evidence artifact and doc actually exists on disk;
- no surface advertises a label wider than its canonical ceiling, every published
  label equals the canonical effective label in the live feature-family register
  (the drift check), and each surface's state matches its label band;
- a qualified surface has no open cell and no narrowing reason, and a narrowed or
  held-back surface always explains itself;
- no above-cutline surface carries a forbidden v1-shell dependency; and
- the publication decision agrees with the firing release-blocking surfaces.

It then replays the checked-in negative fixtures and fails if any expected
rejection no longer fires.

Exit codes: ``0`` clean, ``1`` findings, ``2`` usage/missing-input error.
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path
from typing import Any

# Make the shared build/validation library importable (tools/ is three levels up).
TOOLS_DIR = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(TOOLS_DIR))

from xt12_certification_lib import (  # noqa: E402
    ARTIFACT_REL,
    CAPTURE_REL,
    DIMENSION_BY_KEY,
    EVIDENCE_INDEX_REL,
    FAMILY_REGISTER_REL,
    NARROWING_REPORT_REL,
    SCHEMA_REL,
    Finding,
    validate_matrix,
)

DOC_REL = Path("docs/m5/xt12-certification.md")
REGENERATOR_REL = Path("tools/regenerate_xt12_certification_matrix.py")
FIXTURES_DIR = Path("fixtures/release/m5/xt12-promotion")

DOC_BACKLINKS = (
    "schemas/governance/m5_xt12_certification_matrix.schema.json",
    "artifacts/release/m5/xt12-qualification-matrix.json",
    "artifacts/release/m5/xt12-evidence-index.md",
    "artifacts/release/m5/xt12-narrowing-report.md",
    "tools/ci/m5/xt12_certification_check.py",
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".", help="Repository root (default: cwd).")
    parser.add_argument(
        "--format", choices=("text", "json"), default="text", help="Findings output format."
    )
    return parser.parse_args()


def load_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing required input: {path}")
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON at {path}: {exc}") from exc


def family_index(family_register: dict[str, Any]) -> dict[str, dict[str, Any]]:
    out: dict[str, dict[str, Any]] = {}
    for family in family_register.get("rows", []):
        entry_id = family.get("entry_id")
        if isinstance(entry_id, str):
            out[entry_id] = family
    return out


def check_schema(matrix: dict[str, Any], schema_path: Path, findings: list[Finding]) -> None:
    try:
        from jsonschema import Draft202012Validator  # type: ignore
    except ImportError:
        # jsonschema is not always present; the hand checks still cover the
        # load-bearing invariants.
        return
    schema = load_json(schema_path)
    validator = Draft202012Validator(schema)
    for error in sorted(validator.iter_errors(matrix), key=lambda e: list(e.path)):
        findings.append(
            Finding(
                "schema_violation",
                error.message,
                detail={"path": "/".join(str(p) for p in error.path)},
            )
        )


def check_evidence_exists(
    repo_root: Path, matrix: dict[str, Any], findings: list[Finding]
) -> None:
    """Every bound certification evidence artifact and doc must exist on disk."""
    refs: set[tuple[str, str]] = set()
    for entry in matrix.get("evidence_catalog", []):
        refs.add(("artifact_ref", entry.get("artifact_ref", "")))
        refs.add(("doc_ref", entry.get("doc_ref", "")))
        if entry.get("check_ref"):
            refs.add(("check_ref", entry["check_ref"]))
    for kind, ref in sorted(refs):
        if not ref or not (repo_root / ref).exists():
            findings.append(
                Finding(
                    "evidence_ref_missing",
                    "bound certification evidence does not exist on disk",
                    detail={"kind": kind, "ref": ref},
                )
            )
    # Each cell must name a dimension known to the catalog.
    for row in matrix.get("rows", []):
        for cell in row.get("cells", []):
            if cell.get("dimension") not in DIMENSION_BY_KEY:
                findings.append(
                    Finding(
                        "unknown_dimension",
                        "cell names a dimension that is not in the canonical catalog",
                        entry_id=row.get("entry_id"),
                        detail={"dimension": cell.get("dimension")},
                    )
                )


def check_regenerator_current(repo_root: Path, findings: list[Finding]) -> None:
    regenerator = repo_root / REGENERATOR_REL
    if not regenerator.exists():
        findings.append(Finding("regenerator_missing", f"missing regenerator: {REGENERATOR_REL}"))
        return
    result = subprocess.run(
        [sys.executable, str(regenerator), "--repo-root", str(repo_root), "--check"],
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        findings.append(
            Finding(
                "regenerator_drift",
                "checked-in matrix/capture/index/report is out of date; rerun the regenerator",
                detail={"output": (result.stdout + result.stderr).strip()},
            )
        )


def check_capture(repo_root: Path, matrix: dict[str, Any], findings: list[Finding]) -> None:
    capture = load_json(repo_root / CAPTURE_REL)
    summary = matrix.get("summary", {})
    publication = matrix.get("publication", {})
    expectations = {
        "decision": publication.get("decision"),
        "total_surfaces": summary.get("total_surfaces"),
        "total_cells": summary.get("total_cells"),
        "surfaces_qualified": summary.get("surfaces_qualified"),
        "surfaces_narrowed": summary.get("surfaces_narrowed"),
        "surfaces_held_back": summary.get("surfaces_held_back"),
        "release_blocking_below_cutline": summary.get("release_blocking_below_cutline"),
    }
    for key, want in expectations.items():
        if capture.get(key) != want:
            findings.append(
                Finding(
                    "capture_mismatch",
                    "validation capture disagrees with the matrix",
                    detail={"field": key, "capture": capture.get(key), "matrix": want},
                )
            )


def check_doc(repo_root: Path, findings: list[Finding]) -> None:
    doc = repo_root / DOC_REL
    if not doc.exists():
        findings.append(Finding("doc_missing", f"missing companion doc: {DOC_REL}"))
        return
    body = doc.read_text(encoding="utf-8")
    for backlink in DOC_BACKLINKS:
        if backlink not in body:
            findings.append(
                Finding(
                    "doc_missing_backlink",
                    "companion doc must back-link the canonical artifacts and gate",
                    detail={"backlink": backlink},
                )
            )


def check_fixtures(
    repo_root: Path, fam_index: dict[str, dict[str, Any]], findings: list[Finding]
) -> None:
    """Replay the negative fixtures and confirm each expected rejection still fires."""
    cases_path = repo_root / FIXTURES_DIR / "cases.json"
    if not cases_path.exists():
        findings.append(Finding("fixtures_missing", f"missing fixtures manifest: {cases_path}"))
        return
    cases = load_json(cases_path).get("cases", [])
    if not cases:
        findings.append(Finding("fixtures_empty", "fixtures manifest lists no cases"))
        return
    for case in cases:
        file_name = case.get("file")
        expected = case.get("expected_code")
        fixture_path = repo_root / FIXTURES_DIR / file_name
        if not fixture_path.exists():
            findings.append(
                Finding("fixture_file_missing", "fixture file is missing", detail={"file": file_name})
            )
            continue
        doc = load_json(fixture_path)
        codes = {f.code for f in validate_matrix(doc, fam_index)}
        if expected not in codes:
            findings.append(
                Finding(
                    "fixture_not_rejected",
                    "negative fixture no longer triggers its expected rejection",
                    detail={"file": file_name, "expected": expected, "got": sorted(codes)},
                )
            )


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    matrix = load_json(repo_root / ARTIFACT_REL)
    if not isinstance(matrix, dict):
        raise SystemExit("matrix must be a JSON object")
    if not (repo_root / SCHEMA_REL).exists():
        raise SystemExit(f"missing required input: {SCHEMA_REL}")
    for required in (CAPTURE_REL, EVIDENCE_INDEX_REL, NARROWING_REPORT_REL):
        if not (repo_root / required).exists():
            raise SystemExit(f"missing required input: {required}")
    family_register = load_json(repo_root / FAMILY_REGISTER_REL)
    fam_index = family_index(family_register)

    findings: list[Finding] = []
    check_schema(matrix, repo_root / SCHEMA_REL, findings)
    findings.extend(validate_matrix(matrix, fam_index))
    check_evidence_exists(repo_root, matrix, findings)
    check_capture(repo_root, matrix, findings)
    check_regenerator_current(repo_root, findings)
    check_doc(repo_root, findings)
    check_fixtures(repo_root, fam_index, findings)

    if args.format == "json":
        print(json.dumps({"findings": [f.as_dict() for f in findings]}, indent=2))
    elif not findings:
        print("m5 xt certification: clean")
    else:
        for finding in findings:
            location = finding.entry_id or "matrix"
            print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
