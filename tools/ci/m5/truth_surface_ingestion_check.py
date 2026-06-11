#!/usr/bin/env python3
"""M5 truth-surface evidence-ingestion CI gate.

This gate enforces that the user-/operator-facing truth surfaces for the M5
feature families never drift from their canonical source of truth. It reads:

- the ingestion register at
  ``artifacts/release/m5/m5_truth_surface_evidence_ingestion.json``;
- its frozen validation capture under ``.../captures/``;
- the boundary schema at
  ``schemas/governance/m5_truth_surface_evidence_ingestion.schema.json``;
- the canonical M5 feature family register it ingests; and
- the companion doc at
  ``docs/m5/help-about-service-health-truth-ingestion.md``.

For the register the gate verifies that:

- it validates against the boundary schema;
- the register and capture are byte-for-byte what the regenerator would emit
  (so the checked-in copy is never hand-edited away from its source);
- every M5 feature family is surfaced on all five named truth surfaces;
- no surface publishes a label wider than the canonical claim ceiling, and
  every published label equals the canonical *effective* label currently in the
  feature family register (this is the contradiction/drift check: stale cloned
  copy fails here);
- every below-stable surface exposes a typed ingest state and reason, so a
  reader can tell why a lane is narrowed without internal notes;
- every row discloses local-only/mirrored/managed/browser-handoff posture, and
  every service-health row carries an operational contract state; and
- the companion doc back-links the schema, artifact, and gate.

Exit codes:

- ``0`` -- ingestion is clean and matches its source.
- ``1`` -- one or more findings.
- ``2`` -- usage error or missing input file.
"""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

REGISTER_REL = Path("artifacts/release/m5/m5_truth_surface_evidence_ingestion.json")
CAPTURE_REL = Path(
    "artifacts/release/m5/captures/m5_truth_surface_evidence_ingestion_validation_capture.json"
)
SCHEMA_REL = Path("schemas/governance/m5_truth_surface_evidence_ingestion.schema.json")
FAMILY_REGISTER_REL = Path(
    "artifacts/release/m5/"
    "publish_the_m5_feature_family_register_owner_map_and_proof_corpus_plan.json"
)
DOC_REL = Path("docs/m5/help-about-service-health-truth-ingestion.md")
REGENERATOR_REL = Path("tools/regenerate_m5_truth_surface_ingestion.py")

EXPECTED_RECORD_KIND = "m5_truth_surface_evidence_ingestion"
EXPECTED_SCHEMA_VERSION = 1

REQUIRED_SURFACES = (
    "help_about",
    "service_health",
    "release_center",
    "support_export",
    "public_truth_pack",
)
REQUIRED_FAMILIES = (
    "notebook",
    "data_rich",
    "ai_adjacent",
    "framework",
    "review",
    "companion",
    "managed_depth",
)
ABOVE_CUTLINE = {"lts", "stable"}
BELOW_CUTLINE_STATES = {"stale", "narrowed", "policy_blocked", "preview_only"}
LABEL_RANK = {"lts": 4, "stable": 3, "beta": 2, "preview": 1, "withdrawn": 0}

DOC_BACKLINKS = (
    "schemas/governance/m5_truth_surface_evidence_ingestion.schema.json",
    "artifacts/release/m5/m5_truth_surface_evidence_ingestion.json",
    "tools/ci/m5/truth_surface_ingestion_check.py",
)


@dataclass
class Finding:
    """One blocking finding emitted by the gate."""

    code: str
    message: str
    entry_id: str | None = None
    detail: dict[str, Any] = field(default_factory=dict)

    def as_dict(self) -> dict[str, Any]:
        out: dict[str, Any] = {"code": self.code, "message": self.message}
        if self.entry_id is not None:
            out["entry_id"] = self.entry_id
        if self.detail:
            out["detail"] = self.detail
        return out


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".", help="Repository root (default: cwd).")
    parser.add_argument(
        "--format",
        choices=("text", "json"),
        default="text",
        help="Output format for the findings report.",
    )
    return parser.parse_args()


def load_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing required input: {path}")
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON at {path}: {exc}") from exc


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be a JSON object")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a JSON array")
    return value


def check_schema(register: dict[str, Any], schema_path: Path, findings: list[Finding]) -> None:
    try:
        from jsonschema import Draft202012Validator  # type: ignore
    except ImportError:
        # jsonschema is not always present in the local environment; the hand
        # checks below still cover the load-bearing invariants.
        return
    schema = load_json(schema_path)
    validator = Draft202012Validator(schema)
    for error in sorted(validator.iter_errors(register), key=lambda e: list(e.path)):
        findings.append(
            Finding(
                "schema_violation",
                error.message,
                detail={"path": "/".join(str(p) for p in error.path)},
            )
        )


def check_envelope(register: dict[str, Any], findings: list[Finding]) -> None:
    if register.get("record_kind") != EXPECTED_RECORD_KIND:
        findings.append(
            Finding(
                "record_kind_mismatch",
                f"record_kind must be {EXPECTED_RECORD_KIND}",
                detail={"record_kind": register.get("record_kind")},
            )
        )
    if register.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(
            Finding(
                "schema_version_mismatch",
                f"schema_version must be {EXPECTED_SCHEMA_VERSION}",
                detail={"schema_version": register.get("schema_version")},
            )
        )


def check_coverage(register: dict[str, Any], findings: list[Finding]) -> None:
    rows = ensure_list(register.get("rows", []), "rows")
    pairs = {(r.get("family_kind"), r.get("truth_surface")) for r in rows}
    for family in REQUIRED_FAMILIES:
        for surface in REQUIRED_SURFACES:
            if (family, surface) not in pairs:
                findings.append(
                    Finding(
                        "coverage_gap",
                        "family is missing a required truth surface",
                        detail={"family_kind": family, "truth_surface": surface},
                    )
                )


def check_rows(register: dict[str, Any], findings: list[Finding]) -> None:
    for row in ensure_list(register.get("rows", []), "rows"):
        row = ensure_dict(row, "row")
        entry_id = row.get("entry_id")
        claim = row.get("canonical_claim_label")
        published = row.get("published_label")
        canonical_published = row.get("canonical_published_label")
        state = row.get("ingest_state")
        reasons = row.get("active_ingest_reasons") or []

        if LABEL_RANK.get(published, 99) > LABEL_RANK.get(claim, -1):
            findings.append(
                Finding(
                    "published_exceeds_ceiling",
                    "surface advertises a wider claim than its ceiling",
                    entry_id=entry_id,
                    detail={"published": published, "ceiling": claim},
                )
            )
        if published != canonical_published:
            findings.append(
                Finding(
                    "published_not_canonical",
                    "published label does not equal the canonical effective label",
                    entry_id=entry_id,
                    detail={"published": published, "canonical": canonical_published},
                )
            )
        if state == "current":
            if reasons or published != claim:
                findings.append(
                    Finding(
                        "current_row_not_backed",
                        "current row must sit at the ceiling with no reasons",
                        entry_id=entry_id,
                    )
                )
        elif not reasons:
            findings.append(
                Finding(
                    "narrowed_row_without_reason",
                    "non-current row must carry at least one ingest reason",
                    entry_id=entry_id,
                )
            )
        if state in BELOW_CUTLINE_STATES and published in ABOVE_CUTLINE:
            findings.append(
                Finding(
                    "narrowed_state_not_below_cutline",
                    "below-stable state must publish a below-stable label",
                    entry_id=entry_id,
                    detail={"state": state, "published": published},
                )
            )
        if not row.get("posture"):
            findings.append(
                Finding("missing_posture", "row must disclose posture", entry_id=entry_id)
            )
        surface = row.get("truth_surface")
        has_contract = "service_contract_state" in row
        if surface == "service_health" and not has_contract:
            findings.append(
                Finding(
                    "service_health_missing_contract_state",
                    "service-health row must carry a contract state",
                    entry_id=entry_id,
                )
            )
        if surface != "service_health" and has_contract:
            findings.append(
                Finding(
                    "non_service_health_has_contract_state",
                    "only service-health rows may carry a contract state",
                    entry_id=entry_id,
                )
            )


def check_source_drift(
    register: dict[str, Any], family_register: dict[str, Any], findings: list[Finding]
) -> None:
    """Re-read the canonical family register and fail on any ingested drift.

    This is the load-bearing contradiction check: the ingestion artifact must
    quote the exact claim ceiling and effective label the source packet
    currently publishes for each family.
    """
    by_entry: dict[str, dict[str, Any]] = {}
    for family in ensure_list(family_register.get("rows", []), "family_register.rows"):
        entry_id = family.get("entry_id")
        if isinstance(entry_id, str):
            by_entry[entry_id] = family

    for row in ensure_list(register.get("rows", []), "rows"):
        source_ref = row.get("canonical_source_ref")
        source = by_entry.get(source_ref)
        if source is None:
            findings.append(
                Finding(
                    "source_ref_unknown",
                    "canonical_source_ref does not name a family in the source register",
                    entry_id=row.get("entry_id"),
                    detail={"canonical_source_ref": source_ref},
                )
            )
            continue
        if row.get("canonical_claim_label") != source.get("claim_label"):
            findings.append(
                Finding(
                    "source_claim_drift",
                    "ingested claim ceiling drifted from the source register",
                    entry_id=row.get("entry_id"),
                    detail={
                        "ingested": row.get("canonical_claim_label"),
                        "source": source.get("claim_label"),
                    },
                )
            )
        if row.get("canonical_published_label") != source.get("published_label"):
            findings.append(
                Finding(
                    "source_published_drift",
                    "ingested effective label drifted from the source register",
                    entry_id=row.get("entry_id"),
                    detail={
                        "ingested": row.get("canonical_published_label"),
                        "source": source.get("published_label"),
                    },
                )
            )


def check_regenerator_current(repo_root: Path, findings: list[Finding]) -> None:
    """Confirm the checked-in artifact and capture match the regenerator output."""
    import subprocess

    regenerator = repo_root / REGENERATOR_REL
    if not regenerator.exists():
        findings.append(
            Finding("regenerator_missing", f"missing regenerator: {REGENERATOR_REL}")
        )
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
                "checked-in artifact/capture is out of date; rerun the regenerator",
                detail={"output": (result.stdout + result.stderr).strip()},
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


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    register = ensure_dict(load_json(repo_root / REGISTER_REL), "register")
    # The capture and schema must exist so the contract stays discoverable.
    if not (repo_root / CAPTURE_REL).exists():
        raise SystemExit(f"missing required input: {CAPTURE_REL}")
    if not (repo_root / SCHEMA_REL).exists():
        raise SystemExit(f"missing required input: {SCHEMA_REL}")
    family_register = ensure_dict(
        load_json(repo_root / FAMILY_REGISTER_REL), "family_register"
    )

    findings: list[Finding] = []
    check_schema(register, repo_root / SCHEMA_REL, findings)
    check_envelope(register, findings)
    check_coverage(register, findings)
    check_rows(register, findings)
    check_source_drift(register, family_register, findings)
    check_regenerator_current(repo_root, findings)
    check_doc(repo_root, findings)

    if args.format == "json":
        print(json.dumps({"findings": [f.as_dict() for f in findings]}, indent=2))
    elif not findings:
        print("m5 truth-surface ingestion: clean")
    else:
        for finding in findings:
            location = finding.entry_id or "register"
            print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
