#!/usr/bin/env python3
"""M5 imported-theme mapping & rollback report gate.

This gate enforces that the checked-in M5 imported-theme report stays fresh,
honest, and clean: every imported theme surfaces explicit mapping quality and
unresolved-slot counts before users trust the result, no row claims parity it
cannot back, every imported visual customization is reversible, and the
migration-center, support/export, compatibility, release/public-truth, and
sync/import surfaces share the same report object. It reads:

- the report fixture at ``fixtures/ux/m5/theme-import-corpus/report.json``;
- the support-export fixture at
  ``fixtures/ux/m5/theme-import-corpus/support_export.json``;
- the boundary schema at ``schemas/ux/m5-theme-import-report.schema.json``; and
- the published report at
  ``artifacts/ux/m5/theme-import-reports/m5_theme_import_report.md`` and the
  companion doc at ``docs/m5/theme-import-and-rollback.md``.

For the report the gate verifies that:

- the record envelope (record kind, schema version, shared contract ref, source
  schema ref) is correct and the source schema file exists on disk;
- the report-level invariant flags are all true and imports preview before
  apply;
- the outcome summary, aggregate-token summary, and ecosystem coverage are
  recomputed and match the rows;
- every row's mapping-summary counts sum to its total and its syntax-coverage
  counts and percent are internally consistent;
- every row carries source provenance, a parity note, an explicit unresolved
  count disclosed with listed slots, a compatibility note when parity is not
  full, and a reversible rollback ref (a row that wrote durable state may never
  carry ``rollback_unavailable_denied``);
- no row claims full parity (``claimed_with_report``) without zero unresolved
  mappings, zero blocked honesty checks, full syntax coverage, and a translated
  headline mapping state;
- a report with escalation rows (blocked / rolled back / policy denied / review
  required) carries export and support refs, and the report declares
  migration-center, compatibility, and release/public-truth refs;
- the support-export wrapper quotes the report id and every row id, source-theme
  identifier, checkpoint ref, and rollback ref; and
- the published report and companion doc exist and the doc back-links the
  canonical schema, fixtures, artifact, and gate.

Exit codes:

- ``0`` -- report is clean.
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

REPORT_REL = Path("fixtures/ux/m5/theme-import-corpus/report.json")
SUPPORT_EXPORT_REL = Path("fixtures/ux/m5/theme-import-corpus/support_export.json")
SCHEMA_REL = Path("schemas/ux/m5-theme-import-report.schema.json")
MARKDOWN_REL = Path("artifacts/ux/m5/theme-import-reports/m5_theme_import_report.md")
DOC_REL = Path("docs/m5/theme-import-and-rollback.md")

EXPECTED_RECORD_KIND_REPORT = "shell_m5_theme_import_report_record"
EXPECTED_RECORD_KIND_ROW = "shell_m5_theme_import_report_row_record"
EXPECTED_RECORD_KIND_SUPPORT = "shell_m5_theme_import_report_support_export_record"
EXPECTED_SHARED_CONTRACT_REF = "shell:m5_theme_import_report:v1"
EXPECTED_SCHEMA_VERSION = 1

DURABLE_OUTCOMES = {"applied", "applied_with_warnings", "rolled_back"}
ESCALATION_OUTCOMES = {"blocked", "rolled_back", "policy_denied", "review_required"}

INVARIANT_FLAGS = (
    "every_import_reversible",
    "no_overclaimed_parity",
    "unresolved_counts_disclosed",
    "no_raw_theme_content",
)

DOC_BACKLINKS = (
    "artifacts/ux/m5/theme-import-reports/m5_theme_import_report.md",
    "fixtures/ux/m5/theme-import-corpus/report.json",
    "schemas/ux/m5-theme-import-report.schema.json",
    "tools/ci/m5/theme_import_report_check.py",
)


@dataclass
class Finding:
    """One blocking finding emitted by the gate."""

    code: str
    message: str
    row_id: str | None = None
    detail: dict[str, Any] = field(default_factory=dict)

    def as_dict(self) -> dict[str, Any]:
        out: dict[str, Any] = {"code": self.code, "message": self.message}
        if self.row_id is not None:
            out["row_id"] = self.row_id
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


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a JSON array")
    return value


def expected_percent(total: int, translated: int) -> int:
    return 100 if total == 0 else (translated * 100) // total


def check_report_envelope(repo_root: Path, report: dict[str, Any], findings: list[Finding]) -> None:
    if report.get("record_kind") != EXPECTED_RECORD_KIND_REPORT:
        findings.append(
            Finding(
                "report_record_kind_mismatch",
                f"report.record_kind must be {EXPECTED_RECORD_KIND_REPORT}",
                detail={"record_kind": report.get("record_kind")},
            )
        )
    if report.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(
            Finding(
                "report_schema_version_mismatch",
                f"report.schema_version must be {EXPECTED_SCHEMA_VERSION}",
                detail={"schema_version": report.get("schema_version")},
            )
        )
    if report.get("shared_contract_ref") != EXPECTED_SHARED_CONTRACT_REF:
        findings.append(
            Finding(
                "report_shared_contract_ref_mismatch",
                f"report.shared_contract_ref must be {EXPECTED_SHARED_CONTRACT_REF}",
                detail={"shared_contract_ref": report.get("shared_contract_ref")},
            )
        )
    source_schema_ref = report.get("source_schema_ref")
    if not isinstance(source_schema_ref, str) or not (repo_root / source_schema_ref).exists():
        findings.append(
            Finding(
                "source_schema_ref_missing",
                "report.source_schema_ref must point at an existing schema",
                detail={"source_schema_ref": source_schema_ref},
            )
        )
    if report.get("preview_before_apply") is not True:
        findings.append(
            Finding(
                "preview_before_apply_missing",
                "report.preview_before_apply must be true",
            )
        )
    for flag in INVARIANT_FLAGS:
        if report.get(flag) is not True:
            findings.append(
                Finding(
                    "invariant_flag_not_true",
                    f"report.{flag} must be true",
                    detail={"flag": flag, "value": report.get(flag)},
                )
            )
    for ref_field in ("migration_center_refs", "compatibility_report_refs", "release_truth_refs"):
        refs = report.get(ref_field)
        if not isinstance(refs, list) or not refs:
            findings.append(
                Finding(
                    "routing_ref_missing",
                    f"report.{ref_field} must be a non-empty array",
                    detail={"field": ref_field},
                )
            )


def check_summaries(report: dict[str, Any], rows: list[dict[str, Any]], findings: list[Finding]) -> None:
    outcome_counts = {
        "preview_ready": 0,
        "applied": 0,
        "applied_with_warnings": 0,
        "blocked": 0,
        "rolled_back": 0,
        "cancelled": 0,
        "policy_denied": 0,
        "review_required": 0,
    }
    total_source = total_translated = total_unresolved = total_blocked = 0
    ecosystems: list[str] = []
    for row in rows:
        outcome = row.get("import_outcome")
        if outcome in outcome_counts:
            outcome_counts[outcome] += 1
        summary = row.get("mapping_summary", {})
        total_source += int(summary.get("total_source_slot_count", 0))
        total_translated += int(summary.get("translated_slot_count", 0))
        total_unresolved += int(summary.get("unresolved_mapping_count", 0))
        total_blocked += int(summary.get("blocked_honesty_count", 0))
        eco = row.get("source_tool", {}).get("source_ecosystem")
        if eco is not None and eco not in ecosystems:
            ecosystems.append(eco)

    expected_outcome = dict(outcome_counts, total_rows=len(rows))
    if report.get("outcome_summary") != expected_outcome:
        findings.append(
            Finding(
                "outcome_summary_stale",
                "report.outcome_summary does not match the rows",
                detail={"expected": expected_outcome, "declared": report.get("outcome_summary")},
            )
        )
    expected_tokens = {
        "total_source_slots": total_source,
        "total_translated_slots": total_translated,
        "total_unresolved_slots": total_unresolved,
        "total_blocked_slots": total_blocked,
    }
    if report.get("aggregate_tokens") != expected_tokens:
        findings.append(
            Finding(
                "aggregate_tokens_stale",
                "report.aggregate_tokens does not match the rows",
                detail={"expected": expected_tokens, "declared": report.get("aggregate_tokens")},
            )
        )
    if report.get("ecosystem_coverage") != ecosystems:
        findings.append(
            Finding(
                "ecosystem_coverage_stale",
                "report.ecosystem_coverage does not match the rows",
                detail={"expected": ecosystems, "declared": report.get("ecosystem_coverage")},
            )
        )


def check_row(row: dict[str, Any], findings: list[Finding]) -> None:
    row_id = row.get("row_id") or "<unknown>"

    if row.get("record_kind") != EXPECTED_RECORD_KIND_ROW:
        findings.append(
            Finding(
                "row_record_kind_mismatch",
                f"row.record_kind must be {EXPECTED_RECORD_KIND_ROW}",
                row_id=row_id,
                detail={"record_kind": row.get("record_kind")},
            )
        )

    source_tool = row.get("source_tool", {})
    for field_name in ("source_ecosystem", "source_tool_name", "source_tool_version", "source_theme_identifier"):
        if not str(source_tool.get(field_name, "")).strip():
            findings.append(
                Finding(
                    "source_provenance_missing",
                    "row must carry full source provenance",
                    row_id=row_id,
                    detail={"field": field_name},
                )
            )

    summary = row.get("mapping_summary", {})
    parts = (
        int(summary.get("translated_slot_count", 0))
        + int(summary.get("substituted_with_fallback_count", 0))
        + int(summary.get("unsupported_slot_count", 0))
        + int(summary.get("unresolved_mapping_count", 0))
        + int(summary.get("blocked_honesty_count", 0))
    )
    if parts != int(summary.get("total_source_slot_count", -1)):
        findings.append(
            Finding(
                "mapping_summary_inconsistent",
                "row mapping-summary counts do not sum to the total",
                row_id=row_id,
                detail={"sum": parts, "total": summary.get("total_source_slot_count")},
            )
        )

    syntax = row.get("syntax_coverage", {})
    s_total = int(syntax.get("total_source_scope_count", 0))
    s_translated = int(syntax.get("translated_scope_count", 0))
    s_parts = (
        s_translated
        + int(syntax.get("substituted_scope_count", 0))
        + int(syntax.get("unresolved_scope_count", 0))
        + int(syntax.get("blocked_scope_count", 0))
    )
    if s_parts > s_total or syntax.get("coverage_percent") != expected_percent(s_total, s_translated):
        findings.append(
            Finding(
                "syntax_coverage_inconsistent",
                "row syntax-coverage counts or percent are inconsistent",
                row_id=row_id,
                detail={
                    "parts": s_parts,
                    "total": s_total,
                    "coverage_percent": syntax.get("coverage_percent"),
                    "expected_percent": expected_percent(s_total, s_translated),
                },
            )
        )

    if not str(row.get("parity_note", "")).strip():
        findings.append(Finding("parity_note_missing", "row must carry a parity note", row_id=row_id))

    unresolved = int(summary.get("unresolved_mapping_count", 0))
    if unresolved > 0 and not row.get("unresolved_slots"):
        findings.append(
            Finding(
                "unresolved_count_hidden",
                "row with a non-zero unresolved count discloses no unresolved slots",
                row_id=row_id,
                detail={"unresolved_mapping_count": unresolved},
            )
        )

    parity = row.get("parity_claim_state")
    if parity != "claimed_with_report" and row.get("compatibility_note") in (None, ""):
        findings.append(
            Finding(
                "compatibility_note_missing",
                "a non-full-parity row must disclose a compatibility note",
                row_id=row_id,
                detail={"parity_claim_state": parity},
            )
        )
    if parity == "claimed_with_report":
        full_mapping = (
            unresolved == 0
            and int(summary.get("blocked_honesty_count", 0)) == 0
            and int(summary.get("unsupported_slot_count", 0)) == 0
            and int(summary.get("substituted_with_fallback_count", 0)) == 0
            and int(summary.get("translated_slot_count", 0)) == int(summary.get("total_source_slot_count", -1))
        )
        full_syntax = (
            int(syntax.get("unresolved_scope_count", 0)) == 0
            and int(syntax.get("blocked_scope_count", 0)) == 0
            and int(syntax.get("substituted_scope_count", 0)) == 0
            and s_translated == s_total
        )
        if not (full_mapping and full_syntax and row.get("primary_mapping_state") == "translated"):
            findings.append(
                Finding(
                    "parity_overclaimed",
                    "row claims full parity it cannot back",
                    row_id=row_id,
                )
            )

    rollback = row.get("rollback", {})
    rollback_ref = str(rollback.get("rollback_ref", "")).strip()
    rollback_class = rollback.get("rollback_path_class")
    if not rollback_ref:
        findings.append(Finding("rollback_ref_missing", "row must carry a rollback ref", row_id=row_id))
    durable = bool(row.get("mutates_durable_state")) or row.get("import_outcome") in DURABLE_OUTCOMES
    if durable and rollback_class == "rollback_unavailable_denied":
        findings.append(
            Finding(
                "rollback_path_missing",
                "a row that wrote durable state must carry a reversible rollback path",
                row_id=row_id,
                detail={"rollback_path_class": rollback_class},
            )
        )

    if not row.get("docs_help_refs"):
        findings.append(Finding("docs_help_ref_missing", "row must carry a docs/help ref", row_id=row_id))


def check_escalation_refs(report: dict[str, Any], rows: list[dict[str, Any]], findings: list[Finding]) -> None:
    has_escalation = any(row.get("import_outcome") in ESCALATION_OUTCOMES for row in rows)
    if has_escalation and (not report.get("export_refs") or not report.get("support_packet_refs")):
        findings.append(
            Finding(
                "escalation_refs_missing",
                "a report with escalation rows must carry export and support refs",
            )
        )


def check_support_export(report: dict[str, Any], export: dict[str, Any], findings: list[Finding]) -> None:
    if export.get("record_kind") != EXPECTED_RECORD_KIND_SUPPORT:
        findings.append(
            Finding(
                "support_record_kind_mismatch",
                f"support_export.record_kind must be {EXPECTED_RECORD_KIND_SUPPORT}",
                detail={"record_kind": export.get("record_kind")},
            )
        )
    case_ids = export.get("case_ids")
    if not isinstance(case_ids, list):
        findings.append(Finding("support_case_ids_missing", "support_export.case_ids must be an array"))
        return
    case_set = set(case_ids)
    report_id = report.get("report_id")
    if report_id not in case_set:
        findings.append(
            Finding("support_missing_report_id", "support_export.case_ids must quote the report id")
        )
    for row in ensure_list(report.get("rows", []), "report.rows"):
        row_id = row.get("row_id")
        provenance = row.get("source_tool", {}).get("source_theme_identifier")
        rollback_ref = row.get("rollback", {}).get("rollback_ref")
        checkpoint = row.get("rollback", {}).get("checkpoint_ref")
        if row_id not in case_set:
            findings.append(Finding("support_missing_row_id", "case_ids must quote every row id", row_id=row_id))
        if provenance not in case_set:
            findings.append(
                Finding("support_missing_provenance", "case_ids must quote every source-theme id", row_id=row_id)
            )
        if rollback_ref not in case_set:
            findings.append(
                Finding("support_missing_rollback_ref", "case_ids must quote every rollback ref", row_id=row_id)
            )
        if checkpoint is not None and checkpoint not in case_set:
            findings.append(
                Finding("support_missing_checkpoint", "case_ids must quote every checkpoint ref", row_id=row_id)
            )


def check_publications(repo_root: Path, findings: list[Finding]) -> None:
    markdown = repo_root / MARKDOWN_REL
    if not markdown.exists():
        findings.append(Finding("published_markdown_missing", f"missing published markdown: {MARKDOWN_REL}"))
    doc = repo_root / DOC_REL
    if not doc.exists():
        findings.append(Finding("published_doc_missing", f"missing companion doc: {DOC_REL}"))
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

    report = ensure_dict(load_json(repo_root / REPORT_REL), "report")
    export = ensure_dict(load_json(repo_root / SUPPORT_EXPORT_REL), "support_export")
    if not (repo_root / SCHEMA_REL).exists():
        raise SystemExit(f"missing required input: {SCHEMA_REL}")

    rows = ensure_list(report.get("rows", []), "report.rows")

    findings: list[Finding] = []
    check_report_envelope(repo_root, report, findings)
    if not rows:
        findings.append(Finding("rows_empty", "report.rows must be non-empty"))
    check_summaries(report, rows, findings)
    for row in rows:
        check_row(ensure_dict(row, "row"), findings)
    check_escalation_refs(report, rows, findings)
    check_support_export(report, export, findings)
    check_publications(repo_root, findings)

    if args.format == "json":
        print(json.dumps({"findings": [f.as_dict() for f in findings]}, indent=2))
    else:
        if not findings:
            print("m5 imported-theme report: clean")
        else:
            for finding in findings:
                location = finding.row_id or "report"
                print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
