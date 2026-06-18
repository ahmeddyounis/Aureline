#!/usr/bin/env python3
"""M5 automation surface certification gate.

This gate enforces that the checked-in automation certification packet stays
honest: every claimed M5 notebook/request-API/package/test-debug/incident/AI
automation surface is present once, authors through the declarative recipe
builder, cites machine-readable upstream automation evidence, and certifies across
the six certification dimensions (builder parity, parameter review,
dry-run/explain coverage, run-history integrity, macro-scope safety, and label
reuse). A surface that authors outside the declarative builder, cites no evidence,
routes unreviewed inputs, shows no side-effect preview, keeps no durable run
history, records unsafe macros, or invents a label vocabulary blocks stable; a
surface that presents itself as shareable without full proof is itself a finding;
a surface whose proof has aged past its freshness window narrows below stable. It
reads:

- the packet at ``artifacts/m5/automation/automation-certification/packet.json``;
- the support export, AI evidence join, incident packet join, and CLI/headless
  view alongside it;
- the boundary schema at
  ``schemas/automation/m5-automation-certification.schema.json`` and the reused
  ``schemas/automation/automation-contract-baseline.schema.json``; and
- the companion doc at ``docs/m5/automation-certification.md``.

The typed Rust consumer mints the same packet, so
``cargo test -p aureline-runtime --test m5_automation_certification`` enforces the
same invariants and that the fixtures and artifacts are bit-for-bit derivable from
the seed.

Exit codes:

- ``0`` -- packet is clean.
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

ARTIFACT_DIR = Path("artifacts/m5/automation/automation-certification")
PACKET_REL = ARTIFACT_DIR / "packet.json"
SUPPORT_EXPORT_REL = ARTIFACT_DIR / "support_export.json"
AI_EVIDENCE_REL = ARTIFACT_DIR / "ai_evidence.json"
INCIDENT_PACKET_REL = ARTIFACT_DIR / "incident_packet.json"
CLI_HEADLESS_REL = ARTIFACT_DIR / "cli_headless.json"
CERTIFICATION_SCHEMA_REL = Path("schemas/automation/m5-automation-certification.schema.json")
CONTRACT_BASELINE_SCHEMA_REL = Path("schemas/automation/automation-contract-baseline.schema.json")
DOC_REL = Path("docs/m5/automation-certification.md")

EXPECTED_RECORD_KIND = "m5_automation_certification_packet"
EXPECTED_SUPPORT_RECORD_KIND = "m5_automation_certification_support_export"
EXPECTED_EVIDENCE_RECORD_KIND = "m5_automation_certification_evidence_join"
EXPECTED_CLI_RECORD_KIND = "m5_automation_certification_cli_headless"
EXPECTED_SCHEMA_VERSION = 1

REQUIRED_SURFACES = [
    "notebook_automation",
    "request_api_automation",
    "package_automation",
    "test_debug_automation",
    "incident_automation",
    "ai_linked_automation",
]

DIMENSIONS = (
    "builder_parity",
    "parameter_review",
    "dry_run_explain_coverage",
    "run_history_integrity",
    "macro_scope_safety",
    "label_reuse",
)

DIMENSION_FINDING = {
    "parameter_review": ("parameter_review_missing", "a surface routes inputs without a typed, secret-safe parameter review"),
    "dry_run_explain_coverage": ("side_effect_preview_missing", "a surface applies automation with no dry-run/explain side-effect preview"),
    "run_history_integrity": ("run_history_integrity_missing", "a surface keeps no durable, redaction-safe, rerun-under-policy run history"),
    "macro_scope_safety": ("macro_scope_unsafe", "a surface records macros that are not scope-safe and fail-closed"),
    "label_reuse": ("label_reuse_broken", "a surface invents a label vocabulary instead of reusing the controlled set"),
}

DOC_BACKLINKS = (
    "schemas/automation/m5-automation-certification.schema.json",
    "artifacts/m5/automation/automation-certification/",
    "fixtures/automation/m5/automation-certification/",
    "tools/ci/m5/automation_certification_check.py",
)


@dataclass
class Finding:
    """One blocking finding emitted by the gate."""

    code: str
    message: str
    subject: str | None = None
    detail: dict[str, Any] = field(default_factory=dict)

    def as_dict(self) -> dict[str, Any]:
        out: dict[str, Any] = {"code": self.code, "message": self.message}
        if self.subject is not None:
            out["subject"] = self.subject
        if self.detail:
            out["detail"] = self.detail
        return out


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".", help="Path to the repository root (default: cwd).")
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


def surface_digest(surface_tokens: list[str]) -> str:
    """Order-stable FNV-1a 64-bit digest over sorted surface tokens, matching Rust."""
    mask = (1 << 64) - 1
    prime = 0x0000_0100_0000_01B3
    h = 0xCBF2_9CE4_8422_2325
    for token in sorted(surface_tokens):
        for byte in token.encode("utf-8"):
            h ^= byte
            h = (h * prime) & mask
        h ^= 0x0A
        h = (h * prime) & mask
    return f"fnv1a64:{h:016x}"


def surface_tokens(surfaces: list[dict[str, Any]]) -> list[str]:
    return [str(s.get("surface")) for s in surfaces if isinstance(s, dict)]


def evidence_nonempty(surface: dict[str, Any]) -> bool:
    refs = surface.get("evidence_refs") or []
    return any(isinstance(ref, str) and ref.strip() for ref in refs)


def evaluate_surface(surface: dict[str, Any]) -> dict[str, bool]:
    """Re-derives the per-dimension pass/fail, mirroring the Rust evaluator."""
    conformant = surface.get("authoring_path") == "declarative_recipe_builder"
    labels = surface.get("safety_labels") or []
    return {
        "builder_parity": conformant and evidence_nonempty(surface),
        "parameter_review": surface.get("parameters_reviewed") is True
        and surface.get("secret_references_safe") is True,
        "dry_run_explain_coverage": surface.get("side_effect_preview_shown") is True
        and surface.get("predicted_effects_disclosed") is True,
        "run_history_integrity": surface.get("run_history_durable") is True
        and surface.get("run_history_redaction_safe") is True
        and surface.get("rerun_under_current_policy") is True,
        "macro_scope_safety": surface.get("macro_scope_declared") is True
        and surface.get("macro_fails_closed_on_mismatch") is True,
        "label_reuse": surface.get("reuses_controlled_labels") is True
        and isinstance(labels, list)
        and len(labels) > 0,
    }


def claim_state(certified: bool, freshness: str) -> str:
    if not certified:
        return "blocked"
    if freshness == "stale":
        return "narrowed_below_stable"
    return "shareable"


def check_packet_block(repo_root: Path, packet: dict[str, Any], findings: list[Finding]) -> None:
    if packet.get("record_kind") != EXPECTED_RECORD_KIND:
        findings.append(
            Finding(
                "record_kind_mismatch",
                f"packet.record_kind must be {EXPECTED_RECORD_KIND}",
                detail={"record_kind": packet.get("record_kind")},
            )
        )
    if packet.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(
            Finding("schema_version_mismatch", f"packet.schema_version must be {EXPECTED_SCHEMA_VERSION}")
        )
    for ref_field in ("packet_id", "generated_at"):
        if not str(packet.get(ref_field, "")).strip():
            findings.append(Finding("identity_missing", f"packet.{ref_field} must be non-empty"))
    for ref_field in ("certification_schema_ref", "contract_baseline_schema_ref", "doc_ref"):
        ref = packet.get(ref_field)
        if not isinstance(ref, str) or not (repo_root / ref).exists():
            findings.append(
                Finding(
                    "schema_ref_missing",
                    f"packet.{ref_field} must point at an existing path",
                    detail={ref_field: ref},
                )
            )
    if packet.get("promotion_state") != "stable":
        findings.append(
            Finding(
                "promotion_not_stable",
                "packet.promotion_state must be stable",
                detail={"promotion_state": packet.get("promotion_state")},
            )
        )
    if packet.get("validation_findings"):
        findings.append(
            Finding(
                "validation_findings_present",
                "packet.validation_findings must be empty",
                detail={"count": len(packet.get("validation_findings", []))},
            )
        )
    surfaces = packet.get("surfaces") if isinstance(packet.get("surfaces"), list) else []
    if packet.get("surface_digest") != surface_digest(surface_tokens(surfaces)):
        findings.append(Finding("surface_digest_drift", "packet.surface_digest must match the surfaces"))


def check_surfaces(repo_root: Path, packet: dict[str, Any], findings: list[Finding]) -> None:
    surfaces = packet.get("surfaces")
    if not isinstance(surfaces, list) or not surfaces:
        findings.append(Finding("surfaces_missing", "packet.surfaces must be a non-empty array"))
        return
    present: dict[str, int] = {}
    for surface in surfaces:
        if isinstance(surface, dict):
            token = str(surface.get("surface"))
            present[token] = present.get(token, 0) + 1
    for token in REQUIRED_SURFACES:
        count = present.get(token, 0)
        if count == 0:
            findings.append(Finding("missing_surface", "a required automation surface is absent", subject=token))
        elif count > 1:
            findings.append(Finding("duplicate_surface", "an automation surface is declared more than once", subject=token))

    for surface in surfaces:
        if isinstance(surface, dict):
            check_surface(repo_root, surface, findings)


def check_surface(repo_root: Path, surface: dict[str, Any], findings: list[Finding]) -> None:
    token = str(surface.get("surface") or "<unknown>")

    for ref in surface.get("evidence_refs") or []:
        if not isinstance(ref, str) or not (repo_root / ref).exists():
            findings.append(
                Finding(
                    "evidence_ref_missing",
                    "a surface cites an evidence ref that does not exist on disk",
                    subject=token,
                    detail={"evidence_ref": ref},
                )
            )

    outcomes = evaluate_surface(surface)
    conformant = surface.get("authoring_path") == "declarative_recipe_builder"
    for dimension, passed in outcomes.items():
        if passed:
            continue
        if dimension == "builder_parity":
            if not conformant:
                findings.append(Finding("ad_hoc_authoring", "a surface authors automation outside the declarative recipe builder", subject=token))
            else:
                findings.append(Finding("missing_builder_evidence", "a surface cites no upstream builder proof", subject=token))
            continue
        code, message = DIMENSION_FINDING[dimension]
        findings.append(Finding(code, message, subject=token))

    if not evidence_nonempty(surface):
        findings.append(Finding("missing_evidence_ref", "a surface cites no upstream automation proof", subject=token))

    age = surface.get("proof_age_days")
    window = surface.get("freshness_window_days")
    if isinstance(age, int) and isinstance(window, int):
        expected_freshness = "stale" if age > window else "current"
        if surface.get("freshness_state") != expected_freshness:
            findings.append(Finding("surface_freshness_drift", "a surface freshness state disagrees with proof age", subject=token))
        if expected_freshness == "stale":
            findings.append(Finding("surface_evidence_stale", "a surface proof has aged past its window", subject=token))
    else:
        expected_freshness = None
        findings.append(Finding("surface_freshness_drift", "a surface has non-integer proof age/window", subject=token))

    certified_expected = all(outcomes.values())
    if surface.get("certified") is not certified_expected:
        findings.append(Finding("surface_certification_drift", "a surface certified flag disagrees with the derivation", subject=token))

    if surface.get("presents_as_shareable") is True:
        if not certified_expected:
            findings.append(Finding("shareable_claim_unproven", "a surface presents as shareable without full proof", subject=token))
        elif expected_freshness == "stale":
            findings.append(Finding("shareable_claim_narrowed", "a surface presents as shareable on aged proof", subject=token))

    if expected_freshness is not None:
        expected_claim = claim_state(certified_expected, expected_freshness)
        if surface.get("claim_state") != expected_claim:
            findings.append(Finding("surface_claim_state_drift", "a surface claim state disagrees with the derivation", subject=token))

    stored = surface.get("dimension_outcomes")
    if not isinstance(stored, list) or len(stored) != len(DIMENSIONS):
        findings.append(Finding("dimension_outcome_drift", "a surface stored the wrong number of dimension outcomes", subject=token))
    else:
        for index, dimension in enumerate(DIMENSIONS):
            row = stored[index]
            if not isinstance(row, dict) or row.get("dimension") != dimension:
                findings.append(Finding("dimension_outcome_drift", "a surface dimension outcome is out of order", subject=token))
            elif row.get("passed") is not outcomes[dimension]:
                findings.append(Finding("dimension_outcome_drift", "a surface dimension outcome disagrees with the derivation", subject=f"{token}:{dimension}"))


def derive_index(surfaces: list[dict[str, Any]]) -> dict[str, Any]:
    all_current = all(s.get("freshness_state") == "current" for s in surfaces if isinstance(s, dict))
    all_certified = bool(surfaces) and all(s.get("certified") is True for s in surfaces if isinstance(s, dict))
    shareable = [str(s.get("surface")) for s in surfaces if isinstance(s, dict) and s.get("claim_state") == "shareable"]
    narrowed = [str(s.get("surface")) for s in surfaces if isinstance(s, dict) and s.get("claim_state") == "narrowed_below_stable"]
    blocked = [str(s.get("surface")) for s in surfaces if isinstance(s, dict) and s.get("claim_state") == "blocked"]
    return {
        "all_surfaces_current": all_current,
        "all_surfaces_certified": all_certified,
        "shareable_surfaces": shareable,
        "narrowed_surfaces": narrowed,
        "blocked_surfaces": blocked,
    }


def check_certification_index(packet: dict[str, Any], findings: list[Finding]) -> None:
    index = packet.get("certification_index")
    if not isinstance(index, dict):
        findings.append(Finding("certification_index_missing", "packet.certification_index must be an object"))
        return
    if not str(index.get("certification_ref", "")).strip():
        findings.append(Finding("certification_index_missing", "certification_index.certification_ref must be non-empty"))
    if not str(index.get("certification_summary", "")).strip():
        findings.append(Finding("certification_index_drift", "certification_index.certification_summary must be non-empty"))
    surfaces = packet.get("surfaces") if isinstance(packet.get("surfaces"), list) else []
    expected = derive_index(surfaces)
    if index.get("all_surfaces_current") is not expected["all_surfaces_current"]:
        findings.append(Finding("certification_index_drift", "certification_index.all_surfaces_current disagrees with the surfaces"))
    if index.get("all_surfaces_certified") is not expected["all_surfaces_certified"]:
        findings.append(Finding("certification_index_drift", "certification_index.all_surfaces_certified disagrees with the surfaces"))
    for key in ("shareable_surfaces", "narrowed_surfaces", "blocked_surfaces"):
        if list(index.get(key) or []) != expected[key]:
            findings.append(Finding("certification_index_drift", f"certification_index.{key} disagrees with the surfaces"))


def check_support_export(packet: dict[str, Any], export: dict[str, Any], findings: list[Finding]) -> None:
    if export.get("record_kind") != EXPECTED_SUPPORT_RECORD_KIND:
        findings.append(Finding("support_record_kind_mismatch", f"support_export.record_kind must be {EXPECTED_SUPPORT_RECORD_KIND}"))
    if export.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(Finding("support_schema_version_mismatch", "support_export.schema_version mismatch"))
    if not str(export.get("export_id", "")).strip() or not str(export.get("exported_at", "")).strip():
        findings.append(Finding("support_identity_missing", "support_export must carry an id and timestamp"))
    if export.get("packet_id_ref") != packet.get("packet_id"):
        findings.append(Finding("support_packet_ref_mismatch", "support_export.packet_id_ref must quote the packet id"))
    if export.get("packet") != packet:
        findings.append(Finding("support_packet_drift", "support_export.packet must equal the checked-in packet"))


def check_evidence_join(packet: dict[str, Any], view: dict[str, Any], surface: str, findings: list[Finding]) -> None:
    label = f"{surface}_join"
    if view.get("record_kind") != EXPECTED_EVIDENCE_RECORD_KIND:
        findings.append(Finding("evidence_record_kind_mismatch", f"{label}.record_kind must be {EXPECTED_EVIDENCE_RECORD_KIND}"))
    if view.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(Finding("evidence_schema_version_mismatch", f"{label}.schema_version mismatch"))
    if view.get("surface") != surface:
        findings.append(Finding("evidence_surface_mismatch", f"{label}.surface must be {surface}"))
    if view.get("packet_id_ref") != packet.get("packet_id"):
        findings.append(Finding("evidence_packet_ref_mismatch", f"{label}.packet_id_ref must quote the packet id"))
    if view.get("surface_digest") != packet.get("surface_digest"):
        findings.append(Finding("evidence_surface_digest_drift", f"{label}.surface_digest must match the packet"))
    if view.get("certification_index") != packet.get("certification_index"):
        findings.append(Finding("evidence_index_drift", f"{label}.certification_index must match the packet"))

    surfaces = packet.get("surfaces") if isinstance(packet.get("surfaces"), list) else []
    rows = view.get("surface_rows") if isinstance(view.get("surface_rows"), list) else []
    if len(rows) != len(surfaces):
        findings.append(Finding("evidence_surface_row_drift", f"{label} must carry one surface row per surface"))
    for row in rows:
        if not isinstance(row, dict):
            continue
        if not str(row.get("authoring_path", "")).strip() or not str(row.get("explanation", "")).strip() or not str(row.get("claim_summary", "")).strip():
            findings.append(Finding("evidence_flattens_provenance", f"{label} surface row drops authoring path, claim, or explanation", subject=row.get("surface")))


def check_cli_headless(packet: dict[str, Any], view: dict[str, Any], findings: list[Finding]) -> None:
    if view.get("record_kind") != EXPECTED_CLI_RECORD_KIND:
        findings.append(Finding("cli_record_kind_mismatch", f"cli_headless.record_kind must be {EXPECTED_CLI_RECORD_KIND}"))
    if view.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(Finding("cli_schema_version_mismatch", "cli_headless.schema_version mismatch"))
    if view.get("packet_id_ref") != packet.get("packet_id"):
        findings.append(Finding("cli_packet_ref_mismatch", "cli_headless.packet_id_ref must quote the packet id"))
    if view.get("surface_digest") != packet.get("surface_digest"):
        findings.append(Finding("cli_surface_digest_drift", "cli_headless.surface_digest must match the packet"))
    if view.get("promotion_state") != packet.get("promotion_state"):
        findings.append(Finding("cli_promotion_drift", "cli_headless.promotion_state must match the packet"))
    surfaces = packet.get("surfaces") if isinstance(packet.get("surfaces"), list) else []
    rows = view.get("surface_rows")
    if not isinstance(rows, list) or len(rows) != len(surfaces):
        findings.append(Finding("cli_surface_row_drift", "cli_headless.surface_rows must carry one row per surface"))
        return
    for row in rows:
        if not isinstance(row, dict):
            continue
        if not (isinstance(row.get("evidence_ref_count"), int) and row.get("evidence_ref_count") > 0) or not str(row.get("explanation", "")).strip():
            findings.append(Finding("cli_surface_unexplained", "a CLI/headless surface row cites no evidence or is unexplained", subject=row.get("surface")))


def check_doc(repo_root: Path, findings: list[Finding]) -> None:
    doc = repo_root / DOC_REL
    if not doc.exists():
        findings.append(Finding("doc_missing", f"missing companion doc: {DOC_REL}"))
        return
    body = doc.read_text(encoding="utf-8")
    for backlink in DOC_BACKLINKS:
        if backlink not in body:
            findings.append(
                Finding("doc_missing_backlink", "companion doc must back-link the canonical artifacts and gate", detail={"backlink": backlink})
            )


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    packet = ensure_dict(load_json(repo_root / PACKET_REL), "packet")
    export = ensure_dict(load_json(repo_root / SUPPORT_EXPORT_REL), "support_export")
    ai_evidence = ensure_dict(load_json(repo_root / AI_EVIDENCE_REL), "ai_evidence")
    incident = ensure_dict(load_json(repo_root / INCIDENT_PACKET_REL), "incident_packet")
    view = ensure_dict(load_json(repo_root / CLI_HEADLESS_REL), "cli_headless")
    for schema_rel in (CERTIFICATION_SCHEMA_REL, CONTRACT_BASELINE_SCHEMA_REL):
        if not (repo_root / schema_rel).exists():
            raise SystemExit(f"missing required input: {schema_rel}")

    findings: list[Finding] = []
    check_packet_block(repo_root, packet, findings)
    check_surfaces(repo_root, packet, findings)
    check_certification_index(packet, findings)
    check_support_export(packet, export, findings)
    check_evidence_join(packet, ai_evidence, "ai_evidence", findings)
    check_evidence_join(packet, incident, "incident_packet", findings)
    check_cli_headless(packet, view, findings)
    check_doc(repo_root, findings)

    if args.format == "json":
        print(json.dumps({"findings": [f.as_dict() for f in findings]}, indent=2))
    else:
        if not findings:
            print("m5 automation certification: clean")
        else:
            for finding in findings:
                location = finding.subject or "packet"
                print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
