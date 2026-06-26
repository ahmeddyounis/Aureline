#!/usr/bin/env python3
"""M5 content-design certification gate.

This gate enforces that the checked-in content-design certification packet stays
fresh, honest, and clean: every governed M5 wording object (safety-critical
strings, controlled glossary, action labels, error/recovery copy, AI copy
guardrails, count/scope language, content-ops metadata, and commercial-boundary
wording) carries a current content-truth row; each row's green/yellow/red status
is the derived auto-narrowed value (never an asserted one); a marketed (green)
row cannot keep a Stable wording claim while its proof is stale, unverified, or
unbacked, its wording drifted, or its content-ops metadata is missing; and a
disclosed narrowing is backed by a reason and, where required, an active waiver.
It reads:

- the packet fixture at
  ``fixtures/release/m5-content-design-certification/packet.json``;
- the dashboard fixture at
  ``fixtures/release/m5-content-design-certification/dashboard.json``;
- the support-export fixture at
  ``fixtures/release/m5-content-design-certification/support_export.json``;
- the boundary schema at
  ``schemas/release/m5-content-design-certification.schema.json``; and
- the published report, dashboard, and companion doc.

The typed Rust consumer mints the same packet, so ``cargo test -p aureline-shell``
enforces the same structural invariants and that the fixtures are bit-for-bit
equal to the seed.

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

PACKET_REL = Path("fixtures/release/m5-content-design-certification/packet.json")
DASHBOARD_REL = Path("fixtures/release/m5-content-design-certification/dashboard.json")
SUPPORT_EXPORT_REL = Path("fixtures/release/m5-content-design-certification/support_export.json")
SCHEMA_REL = Path("schemas/release/m5-content-design-certification.schema.json")
MARKDOWN_REL = Path("artifacts/release/m5-content-design-certification/m5_content_design_certification.md")
DASHBOARD_ARTIFACT_REL = Path("artifacts/content/m5-content-truth-dashboard.json")
DOC_REL = Path("docs/release/m5-content-design-certification.md")

EXPECTED_RECORD_KIND_PACKET = "shell_m5_content_design_certification_packet_record"
EXPECTED_RECORD_KIND_DASHBOARD = "shell_m5_content_truth_dashboard_record"
EXPECTED_RECORD_KIND_SUPPORT = "shell_m5_content_design_certification_support_export_record"
EXPECTED_SHARED_CONTRACT_REF = "shell:m5_content_design_certification:v1"
EXPECTED_SCHEMA_VERSION = 1

REQUIRED_OBJECT_KINDS = [
    "safety_critical_ui_string",
    "glossary_term",
    "action_label_pattern",
    "error_recovery_block",
    "ai_copy_guardrail",
    "count_scope_phrase_set",
    "content_ops_artifact",
    "commercial_boundary_wording",
]

NARROWING_FRESHNESS = {"cached", "warming", "stale", "unverified"}

DOC_BACKLINKS = (
    "artifacts/release/m5-content-design-certification/m5_content_design_certification.md",
    "artifacts/content/m5-content-truth-dashboard.json",
    "fixtures/release/m5-content-design-certification/packet.json",
    "schemas/release/m5-content-design-certification.schema.json",
    "tools/ci/m5/content_design_certification_check.py",
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
    parser.add_argument("--format", choices=("text", "json"), default="text", help="Output format.")
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


def is_stable(row: dict[str, Any]) -> bool:
    return row.get("matrix_qualification") == "stable"


def has_active_waiver(row: dict[str, Any]) -> bool:
    return isinstance(row.get("active_waiver"), dict)


def has_hard_blocker(row: dict[str, Any]) -> bool:
    if row.get("copy_parity") == "undisclosed_drift":
        return True
    if row.get("object_kind") == "content_ops_artifact" and row.get("metadata_state") == "missing":
        return True
    if is_stable(row):
        if row.get("proof_freshness") == "unverified":
            return True
        if not row.get("proof_packet_refs"):
            return True
        if row.get("proof_freshness") == "stale" and not has_active_waiver(row):
            return True
    return False


def has_narrowing(row: dict[str, Any]) -> bool:
    if not is_stable(row):
        return True
    if row.get("proof_freshness") in NARROWING_FRESHNESS:
        return True
    if row.get("copy_parity") == "disclosed_drift":
        return True
    if row.get("metadata_state") in {"partial", "missing"}:
        return True
    return False


def recompute_status(row: dict[str, Any]) -> str:
    if has_hard_blocker(row):
        return "red"
    if has_narrowing(row):
        return "yellow"
    return "green"


def requires_waiver(row: dict[str, Any]) -> bool:
    if row.get("copy_parity") == "disclosed_drift":
        return True
    return is_stable(row) and row.get("proof_freshness") == "stale"


def has_reason(row: dict[str, Any]) -> bool:
    return bool(str(row.get("narrowing_reason") or "").strip())


def check_envelope(repo_root: Path, packet: dict[str, Any], findings: list[Finding]) -> None:
    if packet.get("record_kind") != EXPECTED_RECORD_KIND_PACKET:
        findings.append(Finding("packet_record_kind_mismatch", f"record_kind must be {EXPECTED_RECORD_KIND_PACKET}"))
    if packet.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(Finding("packet_schema_version_mismatch", f"schema_version must be {EXPECTED_SCHEMA_VERSION}"))
    if packet.get("shared_contract_ref") != EXPECTED_SHARED_CONTRACT_REF:
        findings.append(Finding("packet_shared_contract_ref_mismatch", f"shared_contract_ref must be {EXPECTED_SHARED_CONTRACT_REF}"))
    schema_ref = packet.get("source_schema_ref")
    if not isinstance(schema_ref, str) or not (repo_root / schema_ref).exists():
        findings.append(Finding("source_schema_ref_missing", "source_schema_ref must point at an existing schema", detail={"ref": schema_ref}))
    if not str(packet.get("matrix_packet_ref", "")).strip():
        findings.append(Finding("matrix_packet_ref_missing", "matrix_packet_ref must be non-empty"))
    if not str(packet.get("build_identity_ref", "")).strip():
        findings.append(Finding("build_identity_ref_missing", "build_identity_ref must be non-empty"))
    if packet.get("report_clean") is not True:
        findings.append(Finding("packet_not_clean", "report_clean must be true"))
    if packet.get("all_rows_publishable") is not True:
        findings.append(Finding("rows_not_publishable", "all_rows_publishable must be true"))
    if packet.get("blocking_findings"):
        findings.append(Finding("blocking_findings_present", "blocking_findings must be empty", detail={"count": len(packet.get("blocking_findings", []))}))
    if not packet.get("public_truth_refs"):
        findings.append(Finding("public_truth_refs_missing", "public_truth_refs must be a non-empty array"))


def check_coverage(packet: dict[str, Any], rows: list[dict[str, Any]], findings: list[Finding]) -> None:
    present = [row.get("object_kind") for row in rows]
    for kind in REQUIRED_OBJECT_KINDS:
        if kind not in present:
            findings.append(Finding("object_kind_missing", "a governed wording object has no certification row", subject=kind))
    expected_covered = sorted(set(present))
    if packet.get("covered_object_kinds") != expected_covered:
        findings.append(Finding("coverage_stale", "covered_object_kinds does not match the rows", detail={"expected": expected_covered, "declared": packet.get("covered_object_kinds")}))

    statuses = [recompute_status(row) for row in rows]
    expected_counts = {
        "row_count": len(rows),
        "green_row_count": statuses.count("green"),
        "yellow_row_count": statuses.count("yellow"),
        "red_row_count": statuses.count("red"),
    }
    for key, value in expected_counts.items():
        if packet.get(key) != value:
            findings.append(Finding("status_count_stale", f"{key} does not match the rows", detail={"expected": value, "declared": packet.get(key)}))


def check_row(packet: dict[str, Any], row: dict[str, Any], findings: list[Finding]) -> None:
    kind = row.get("object_kind") or "<unknown>"
    generated_at = str(packet.get("generated_at") or "")

    if is_stable(row) and not row.get("proof_packet_refs"):
        findings.append(Finding("row_missing_proof", "a stable row must cite proof packets", subject=kind))
    if row.get("copy_parity") == "undisclosed_drift":
        findings.append(Finding("undisclosed_copy_drift", "a row must not hide a wording drift", subject=kind))
    if kind == "content_ops_artifact" and row.get("metadata_state") == "missing":
        findings.append(Finding("missing_content_ops_metadata", "the content-ops object must keep its metadata", subject=kind))
    if is_stable(row) and row.get("proof_freshness") == "stale" and not has_active_waiver(row):
        findings.append(Finding("stale_proof_without_waiver", "a stable row must not claim current wording on stale proof", subject=kind))
    if is_stable(row) and row.get("proof_freshness") == "unverified":
        findings.append(Finding("unverified_proof_on_stable_row", "a stable row must not claim current wording on unverified proof", subject=kind))

    derived = recompute_status(row)
    if row.get("derived_status") != derived:
        findings.append(Finding("row_status_stale", "derived_status must match the recomputed status", subject=kind, detail={"declared": row.get("derived_status"), "derived": derived}))
    if derived != "green" and not has_reason(row):
        findings.append(Finding("narrowed_row_without_reason", "a narrowed/blocked row must disclose why", subject=kind))
    if requires_waiver(row) and not has_hard_blocker(row) and not has_active_waiver(row):
        findings.append(Finding("narrowed_row_without_waiver", "a waiver-requiring narrowing must carry an active waiver", subject=kind))

    waiver = row.get("active_waiver")
    if isinstance(waiver, dict):
        if waiver.get("object_kind") != kind:
            findings.append(Finding("waiver_object_mismatch", "an attached waiver must point at the row's object", subject=kind))
        if str(waiver.get("expires_at") or "") <= generated_at:
            findings.append(Finding("waiver_expired", "an attached waiver must still be active", subject=kind, detail={"expires_at": waiver.get("expires_at"), "generated_at": generated_at}))


def check_dashboard(packet: dict[str, Any], dashboard: dict[str, Any], findings: list[Finding]) -> None:
    if dashboard.get("record_kind") != EXPECTED_RECORD_KIND_DASHBOARD:
        findings.append(Finding("dashboard_record_kind_mismatch", f"dashboard.record_kind must be {EXPECTED_RECORD_KIND_DASHBOARD}"))
    if dashboard.get("source_packet_ref") != packet.get("packet_id"):
        findings.append(Finding("dashboard_source_mismatch", "dashboard.source_packet_ref must equal the packet id"))
    for key in ("green_row_count", "yellow_row_count", "red_row_count", "all_rows_publishable"):
        if dashboard.get(key) != packet.get(key):
            findings.append(Finding("dashboard_count_stale", f"dashboard.{key} does not match the packet", detail={"packet": packet.get(key), "dashboard": dashboard.get(key)}))
    packet_rows = {row.get("object_kind"): row for row in packet.get("rows", [])}
    for drow in dashboard.get("rows", []):
        kind = drow.get("object_kind")
        prow = packet_rows.get(kind)
        if prow is None:
            findings.append(Finding("dashboard_extra_row", "dashboard row has no matching packet row", subject=kind))
            continue
        if drow.get("status") != prow.get("derived_status"):
            findings.append(Finding("dashboard_status_stale", "dashboard status must match the packet row", subject=kind))


def check_support_export(packet: dict[str, Any], export: dict[str, Any], findings: list[Finding]) -> None:
    if export.get("record_kind") != EXPECTED_RECORD_KIND_SUPPORT:
        findings.append(Finding("support_record_kind_mismatch", f"support_export.record_kind must be {EXPECTED_RECORD_KIND_SUPPORT}"))
    case_ids = export.get("case_ids")
    if not isinstance(case_ids, list):
        findings.append(Finding("support_case_ids_missing", "support_export.case_ids must be an array"))
        return
    case_set = set(case_ids)
    if packet.get("packet_id") not in case_set:
        findings.append(Finding("support_missing_packet_id", "case_ids must quote the packet id"))
    if packet.get("matrix_packet_ref") not in case_set:
        findings.append(Finding("support_missing_matrix_ref", "case_ids must quote the matrix packet ref"))
    for row in packet.get("rows", []):
        if row.get("object_kind") not in case_set:
            findings.append(Finding("support_missing_object_kind", "case_ids must quote every object kind", subject=row.get("object_kind")))
        waiver = row.get("active_waiver")
        if isinstance(waiver, dict) and waiver.get("waiver_id") not in case_set:
            findings.append(Finding("support_missing_waiver_id", "case_ids must quote every active waiver id", subject=row.get("object_kind")))


def check_publications(repo_root: Path, findings: list[Finding]) -> None:
    if not (repo_root / MARKDOWN_REL).exists():
        findings.append(Finding("published_markdown_missing", f"missing published markdown: {MARKDOWN_REL}"))
    if not (repo_root / DASHBOARD_ARTIFACT_REL).exists():
        findings.append(Finding("published_dashboard_missing", f"missing published dashboard: {DASHBOARD_ARTIFACT_REL}"))
    doc = repo_root / DOC_REL
    if not doc.exists():
        findings.append(Finding("published_doc_missing", f"missing companion doc: {DOC_REL}"))
        return
    body = doc.read_text(encoding="utf-8")
    for backlink in DOC_BACKLINKS:
        if backlink not in body:
            findings.append(Finding("doc_missing_backlink", "companion doc must back-link the canonical artifacts and gate", detail={"backlink": backlink}))


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    packet = ensure_dict(load_json(repo_root / PACKET_REL), "packet")
    dashboard = ensure_dict(load_json(repo_root / DASHBOARD_REL), "dashboard")
    export = ensure_dict(load_json(repo_root / SUPPORT_EXPORT_REL), "support_export")
    if not (repo_root / SCHEMA_REL).exists():
        raise SystemExit(f"missing required input: {SCHEMA_REL}")

    rows = packet.get("rows", [])
    if not isinstance(rows, list) or not rows:
        raise SystemExit("packet.rows must be a non-empty array")

    findings: list[Finding] = []
    check_envelope(repo_root, packet, findings)
    check_coverage(packet, rows, findings)
    for row in rows:
        check_row(packet, ensure_dict(row, "row"), findings)
    check_dashboard(packet, dashboard, findings)
    check_support_export(packet, export, findings)
    check_publications(repo_root, findings)

    if args.format == "json":
        print(json.dumps({"findings": [f.as_dict() for f in findings]}, indent=2))
    else:
        if not findings:
            print("m5 content-design certification: clean")
        else:
            for finding in findings:
                location = finding.subject or "packet"
                print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
