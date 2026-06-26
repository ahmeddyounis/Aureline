#!/usr/bin/env python3
"""Copy-parity and boundary-honesty lint for the boundary-wording catalog.

This validator reads the checked-in boundary-wording support export and fails when
hosted/managed/premium/self-hosted/local-only/BYOK/trial wording drifts across
settings, onboarding, marketplace, help/About, release notes, and account/upgrade
prompts, or when a boundary claim is dishonest about the actual product boundary. It
re-derives the same parity and honesty rules the Rust catalog enforces, so a CI gate
can fail on copy-parity or boundary-honesty drift even when the underlying feature
code still works.

Usage:

    python3 scripts/content/m5-copy-parity-lint/check_copy_parity.py
    python3 scripts/content/m5-copy-parity-lint/check_copy_parity.py --repo-root .
    python3 scripts/content/m5-copy-parity-lint/check_copy_parity.py --report report.json
    python3 scripts/content/m5-copy-parity-lint/check_copy_parity.py --self-test
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

ARTIFACT_REL = "artifacts/content/m5-boundary-wording-proof/support_export.json"
SCHEMA_REL = "schemas/content/m5-boundary-wording.schema.json"

RECORD_KIND = "m5_boundary_wording_catalog"

# Local-independence rank a term *claims*, mirroring BoundaryTerm::claimed_independence.
TERM_INDEPENDENCE = {
    "local_only": 5,
    "self_hosted": 4,
    "byok": 3,
    "hosted": 1,
    "managed": 1,
    "premium": 1,
    "trial": 1,
}
# Actual local-independence a posture provides, mirroring ActualBoundaryPosture.
POSTURE_INDEPENDENCE = {
    "local_independent": 5,
    "self_hostable": 4,
    "byok": 3,
    "managed_optional": 2,
    "managed_required": 1,
    "commercial_paid": 1,
}
MANAGED_OR_PAID_TERMS = {"hosted", "managed", "premium", "trial"}
LOCAL_OR_OPEN_PATHS = {"local_only", "byok", "self_hosted"}
MUST_DISCLOSE_SURFACES = {"help_about", "account_upgrade_prompt"}
SUPPORT_REQUIRED_CLAIMS = {"narrows_boundary", "widens_boundary"}
IMPLICATION_DIMENSIONS = ["identity", "network", "data", "export", "rollback"]


class Finding:
    """One lint finding."""

    def __init__(self, check_id: str, severity: str, where: str, message: str) -> None:
        self.check_id = check_id
        self.severity = severity
        self.where = where
        self.message = message

    def as_dict(self) -> dict:
        return {
            "check_id": self.check_id,
            "severity": self.severity,
            "where": self.where,
            "message": self.message,
        }


def _posture_map(entry: dict) -> dict:
    return {s["dimension"]: s["posture"] for s in entry.get("implications", [])}


def _alt_availability(entry: dict) -> dict:
    return {a["path"]: bool(a["available"]) for a in entry.get("alternative_paths", [])}


def _available_local_or_open(entry: dict) -> bool:
    return any(
        a["path"] in LOCAL_OR_OPEN_PATHS and a.get("available")
        for a in entry.get("alternative_paths", [])
    )


def lint_entries_honesty(entries: list[dict]) -> list[Finding]:
    """Per-entry boundary-honesty rules."""
    findings: list[Finding] = []
    for entry in entries:
        eid = entry.get("entry_id", "<unknown>")
        term = entry.get("term", "")
        posture = entry.get("actual_boundary_posture", "")
        postures = _posture_map(entry)

        # No boundary overstatement.
        if TERM_INDEPENDENCE.get(term, 0) > POSTURE_INDEPENDENCE.get(posture, 0):
            findings.append(
                Finding(
                    "boundary_overstates_actual_posture",
                    "error",
                    eid,
                    f"term '{term}' claims more local independence than actual posture '{posture}'",
                )
            )

        # Narrowing or widening references support metadata.
        if entry.get("claim_kind") in SUPPORT_REQUIRED_CLAIMS and not (
            entry.get("support_metadata_ref") or ""
        ).strip():
            findings.append(
                Finding(
                    "narrowing_widening_missing_support_metadata",
                    "error",
                    eid,
                    f"claim '{entry.get('claim_kind')}' must reference compatibility/support metadata",
                )
            )

        # Every implication dimension is explained.
        for dimension in IMPLICATION_DIMENSIONS:
            if dimension not in postures:
                findings.append(
                    Finding(
                        "implication_dimension_missing",
                        "error",
                        eid,
                        f"missing implication dimension '{dimension}'",
                    )
                )

        if term in MANAGED_OR_PAID_TERMS or entry.get("introduces_managed_or_paid"):
            # No false vendor dependence when the core workflow stays local.
            if entry.get("core_workflow_remains_local") and not _available_local_or_open(entry):
                findings.append(
                    Finding(
                        "implies_vendor_dependence_when_core_local",
                        "error",
                        eid,
                        "managed/paid claim with a local-capable core hides every local/open alternative",
                    )
                )
            # Managed/paid introductions keep an export and rollback route.
            if entry.get("introduces_managed_or_paid") and (
                postures.get("export") != "retained" or postures.get("rollback") != "retained"
            ):
                findings.append(
                    Finding(
                        "managed_or_paid_missing_export_or_rollback",
                        "error",
                        eid,
                        "managed/paid introduction must keep export and rollback retained",
                    )
                )
            # Upgrade/account/help surfaces disclose a local/BYOK/self-hosted alternative.
            if (
                entry.get("surface") in MUST_DISCLOSE_SURFACES
                and entry.get("introduces_managed_or_paid")
                and not _available_local_or_open(entry)
            ):
                findings.append(
                    Finding(
                        "upgrade_surface_missing_alternative_disclosure",
                        "error",
                        eid,
                        "upgrade/account/help surface must disclose a local/BYOK/self-hosted alternative",
                    )
                )
    return findings


def lint_parity(entries: list[dict]) -> list[Finding]:
    """Cross-surface copy-parity rules: surfaces sharing a concept agree on the facts."""
    findings: list[Finding] = []
    by_concept: dict[str, list[dict]] = {}
    for entry in entries:
        by_concept.setdefault(entry.get("concept_id", "<unknown>"), []).append(entry)

    for concept_id, group in sorted(by_concept.items()):
        ref = group[0]
        for other in group[1:]:
            pair = f"{ref.get('surface')} vs {other.get('surface')}"
            if ref.get("term") != other.get("term"):
                findings.append(
                    Finding(
                        "term_drift",
                        "error",
                        concept_id,
                        f"{pair}: term {ref.get('term')} vs {other.get('term')}",
                    )
                )
            if ref.get("support_metadata_ref") != other.get("support_metadata_ref"):
                findings.append(
                    Finding(
                        "support_metadata_drift",
                        "error",
                        concept_id,
                        f"{pair}: support metadata refs differ",
                    )
                )
            if _posture_map(ref) != _posture_map(other):
                findings.append(
                    Finding(
                        "implication_posture_drift",
                        "error",
                        concept_id,
                        f"{pair}: identity/network/data/export/rollback postures differ",
                    )
                )
            if ref.get("core_workflow_remains_local") != other.get("core_workflow_remains_local"):
                findings.append(
                    Finding(
                        "local_capability_posture_drift",
                        "error",
                        concept_id,
                        f"{pair}: core-workflow-remains-local posture differs",
                    )
                )
            if _alt_availability(ref) != _alt_availability(other):
                findings.append(
                    Finding(
                        "alternative_availability_drift",
                        "error",
                        concept_id,
                        f"{pair}: disclosed alternative availability differs",
                    )
                )
    return findings


def lint_catalog(catalog: dict) -> list[Finding]:
    """All copy-parity and boundary-honesty findings for a catalog."""
    findings: list[Finding] = []
    if catalog.get("record_kind") != RECORD_KIND:
        findings.append(
            Finding("wrong_record_kind", "error", "<catalog>", f"record_kind != {RECORD_KIND}")
        )
        return findings
    entries = catalog.get("entries", [])
    findings.extend(lint_entries_honesty(entries))
    findings.extend(lint_parity(entries))
    return findings


def maybe_schema_validate(repo_root: Path, catalog: dict) -> list[Finding]:
    """Validate against the JSON schema when jsonschema is installed; otherwise skip."""
    try:
        from jsonschema import Draft202012Validator  # type: ignore
    except Exception:
        return []
    schema_path = repo_root / SCHEMA_REL
    if not schema_path.exists():
        return [Finding("schema_missing", "error", SCHEMA_REL, "schema file not found")]
    schema = json.loads(schema_path.read_text(encoding="utf-8"))
    validator = Draft202012Validator(schema)
    return [
        Finding("schema_violation", "error", "/".join(str(p) for p in err.path), err.message)
        for err in sorted(validator.iter_errors(catalog), key=lambda e: list(e.path))
    ]


def render_summary(findings: list[Finding]) -> str:
    if not findings:
        return "boundary-wording copy-parity lint: OK (no parity or boundary-honesty drift)\n"
    lines = [f"boundary-wording copy-parity lint: {len(findings)} finding(s)"]
    for f in findings:
        lines.append(f"  [{f.severity}] {f.check_id} @ {f.where}: {f.message}")
    return "\n".join(lines) + "\n"


def run_self_test() -> int:
    """Prove the lint catches a synthetic drift/dishonesty without a checked-in bad fixture."""
    base_entry = {
        "entry_id": "entry.a",
        "concept_id": "concept.x",
        "term": "managed",
        "surface": "settings",
        "canonical_text": "x",
        "claim_kind": "states_boundary",
        "support_metadata_ref": "support.metadata.x",
        "actual_boundary_posture": "managed_optional",
        "introduces_managed_or_paid": True,
        "core_workflow_remains_local": True,
        "implications": [
            {"dimension": d, "posture": p, "disclosure": "d"}
            for d, p in [
                ("identity", "required"),
                ("network", "required"),
                ("data", "optional"),
                ("export", "retained"),
                ("rollback", "retained"),
            ]
        ],
        "alternative_paths": [
            {"path": "local_only", "available": True, "disclosure": "d", "reference_ref": "support.metadata.l"}
        ],
        "source_ref": "glossary.term.x",
    }
    drifted = json.loads(json.dumps(base_entry))
    drifted["entry_id"] = "entry.b"
    drifted["surface"] = "help_about"
    drifted["term"] = "hosted"  # term drift across the same concept
    overstating = json.loads(json.dumps(base_entry))
    overstating["entry_id"] = "entry.c"
    overstating["concept_id"] = "concept.y"
    overstating["term"] = "local_only"  # overstates a managed_optional posture
    catalog = {"record_kind": RECORD_KIND, "entries": [base_entry, drifted, overstating]}
    findings = lint_catalog(catalog)
    kinds = {f.check_id for f in findings}
    expected = {"term_drift", "boundary_overstates_actual_posture"}
    missing = expected - kinds
    sys.stdout.write(render_summary(findings))
    if missing:
        sys.stdout.write(f"self-test FAILED: lint missed {sorted(missing)}\n")
        return 1
    sys.stdout.write("self-test OK: lint catches injected drift and overstatement\n")
    return 0


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--artifact", default=None, help="Override the artifact path (repo-relative).")
    parser.add_argument("--report", default=None, help="Write a machine-readable JSON report here.")
    parser.add_argument("--self-test", action="store_true", help="Run the lint against synthetic drift.")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if args.self_test:
        return run_self_test()

    repo_root = Path(args.repo_root).resolve()
    artifact_path = repo_root / (args.artifact or ARTIFACT_REL)
    if not artifact_path.exists():
        sys.stderr.write(f"artifact not found: {artifact_path}\n")
        return 2
    catalog = json.loads(artifact_path.read_text(encoding="utf-8"))

    findings = maybe_schema_validate(repo_root, catalog)
    findings.extend(lint_catalog(catalog))

    sys.stdout.write(render_summary(findings))
    if args.report:
        report_path = repo_root / args.report
        report_path.parent.mkdir(parents=True, exist_ok=True)
        report_path.write_text(
            json.dumps([f.as_dict() for f in findings], indent=2, sort_keys=True) + "\n",
            encoding="utf-8",
        )
    return 1 if any(f.severity == "error" for f in findings) else 0


if __name__ == "__main__":
    raise SystemExit(main())
