#!/usr/bin/env python3
"""M5 live-appearance change & evidence-linkage gate.

This gate enforces that the checked-in M5 live-appearance evidence report stays
fresh, honest, and clean: every live OS theme / contrast / accent / text-scale
change either applies through the appearance-session model or carries an
*explicit* restart-or-reload posture, every screenshot and golden capture is
attributable to the exact build, theme package, and appearance session that
produced it, no claimed row survives by hiding a trust / severity / lifecycle /
focus cue, and no marketed axis passes on a single platform or with only a
static happy-path screenshot. It reads:

- the report fixture at
  ``fixtures/ux/m5/os-appearance-contrast-accent/report.json``;
- the support-export fixture at
  ``fixtures/ux/m5/os-appearance-contrast-accent/support_export.json``;
- the boundary schema at
  ``schemas/ux/m5-live-appearance-evidence.schema.json``; and
- the published report at
  ``artifacts/ux/m5/live-appearance-platform-labs/m5_live_appearance_evidence.md``
  and the companion doc at
  ``docs/m5/live-appearance-and-evidence-linkage.md``.

The typed Rust consumer mints the same report, so
``cargo test -p aureline-shell`` enforces the same structural invariants and
that the fixtures are bit-for-bit equal to the seed.

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

REPORT_REL = Path("fixtures/ux/m5/os-appearance-contrast-accent/report.json")
SUPPORT_EXPORT_REL = Path("fixtures/ux/m5/os-appearance-contrast-accent/support_export.json")
SCHEMA_REL = Path("schemas/ux/m5-live-appearance-evidence.schema.json")
MARKDOWN_REL = Path("artifacts/ux/m5/live-appearance-platform-labs/m5_live_appearance_evidence.md")
DOC_REL = Path("docs/m5/live-appearance-and-evidence-linkage.md")

EXPECTED_RECORD_KIND_REPORT = "shell_m5_live_appearance_evidence_report_record"
EXPECTED_RECORD_KIND_ROW = "shell_m5_live_appearance_evidence_row_record"
EXPECTED_RECORD_KIND_SUPPORT = "shell_m5_live_appearance_evidence_support_export_record"
EXPECTED_SHARED_CONTRACT_REF = "shell:m5_live_appearance_evidence:v1"
EXPECTED_SCHEMA_VERSION = 1

OS_SIGNAL_AXIS = {
    "system_theme_flip": "follow_system",
    "contrast_increased": "contrast",
    "forced_colors_enabled": "contrast",
    "accent_color_changed": "accent",
    "text_scale_increased": "text_scale",
    "reduced_motion_enabled": "reduced_motion",
}

REQUIRED_SURFACE_FAMILIES = {
    "notebook_cell_chrome",
    "result_grid_row",
    "profiler_panel",
    "trace_panel",
    "preview_route_badge",
    "docs_browser_pane",
    "companion_surface",
}

RELOAD_OR_RESTART_POSTURES = {"requires_surface_reload", "requires_app_restart"}
NARROWING_STATUSES = {
    "explicitly_narrowed",
    "not_applicable",
    "platform_omitted",
    "declared_capture_gap",
}
ATTRIBUTABLE_GOLDEN = {"matched", "diff_within_tolerance"}

ROUTING_REF_FIELDS = (
    "release_evidence_refs",
    "extension_inspection_refs",
    "sync_refs",
)

DOC_BACKLINKS = (
    "artifacts/ux/m5/live-appearance-platform-labs/m5_live_appearance_evidence.md",
    "fixtures/ux/m5/os-appearance-contrast-accent/report.json",
    "schemas/ux/m5-live-appearance-evidence.schema.json",
    "tools/ci/m5/live_appearance_evidence_check.py",
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


def projects_evidence(status: str) -> bool:
    return status == "qualified"


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
            Finding("report_schema_version_mismatch", f"report.schema_version must be {EXPECTED_SCHEMA_VERSION}")
        )
    if report.get("shared_contract_ref") != EXPECTED_SHARED_CONTRACT_REF:
        findings.append(
            Finding("report_shared_contract_ref_mismatch", f"report.shared_contract_ref must be {EXPECTED_SHARED_CONTRACT_REF}")
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
    if not str(report.get("build_identity_ref", "")).strip():
        findings.append(Finding("build_identity_ref_missing", "report.build_identity_ref must be non-empty"))
    if not str(report.get("release_channel_class", "")).strip():
        findings.append(Finding("release_channel_class_missing", "report.release_channel_class must be non-empty"))
    if report.get("live_change_demonstrated") is not True:
        findings.append(Finding("live_change_not_demonstrated", "report.live_change_demonstrated must be true"))
    if report.get("all_captures_build_attributed") is not True:
        findings.append(
            Finding("captures_not_build_attributed", "report.all_captures_build_attributed must be true")
        )
    if report.get("report_clean") is not True:
        findings.append(Finding("report_not_clean", "report.report_clean must be true"))
    if report.get("blocking_findings"):
        findings.append(
            Finding(
                "blocking_findings_present",
                "report.blocking_findings must be empty",
                detail={"count": len(report.get("blocking_findings", []))},
            )
        )
    for ref_field in ROUTING_REF_FIELDS:
        refs = report.get(ref_field)
        if not isinstance(refs, list) or not refs:
            findings.append(
                Finding("routing_ref_missing", f"report.{ref_field} must be a non-empty array", detail={"field": ref_field})
            )


def check_coverage(report: dict[str, Any], rows: list[dict[str, Any]], findings: list[Finding]) -> None:
    # Per-axis platform coverage over marketed (qualified) rows.
    axis_platforms: dict[str, set[str]] = {}
    axis_order: list[str] = []
    for row in rows:
        if not projects_evidence(row.get("qualification_status", "")):
            continue
        axis = row.get("changed_axis")
        platform = row.get("platform")
        if axis not in axis_platforms:
            axis_platforms[axis] = set()
            axis_order.append(axis)
        axis_platforms[axis].add(platform)

    declared = report.get("axis_platform_coverage")
    expected = [
        {"axis": axis, "platforms": sorted(axis_platforms[axis])}
        for axis in axis_order
    ]
    if declared != expected:
        findings.append(
            Finding(
                "axis_platform_coverage_stale",
                "report.axis_platform_coverage does not match the rows",
                detail={"expected": expected, "declared": declared},
            )
        )
    for axis, platforms in axis_platforms.items():
        if len(platforms) < 2:
            findings.append(
                Finding(
                    "single_platform_claim",
                    "a marketed appearance axis must be proven on at least two platforms",
                    detail={"axis": axis, "platforms": sorted(platforms)},
                )
            )

    # Surface coverage over qualified rows.
    covered = sorted({row.get("surface_family") for row in rows if projects_evidence(row.get("qualification_status", ""))})
    if report.get("covered_surface_families") != covered:
        findings.append(
            Finding(
                "surface_coverage_stale",
                "report.covered_surface_families does not match the qualified rows",
                detail={"expected": covered, "declared": report.get("covered_surface_families")},
            )
        )
    for family in sorted(REQUIRED_SURFACE_FAMILIES - set(covered)):
        findings.append(
            Finding("surface_family_uncovered", "a required surface family is exercised by no qualified row", detail={"surface_family": family})
        )

    # OS-signal coverage in first-seen order.
    signals: list[str] = []
    for row in rows:
        sig = row.get("os_signal")
        if sig not in signals:
            signals.append(sig)
    if report.get("os_signal_coverage") != signals:
        findings.append(
            Finding(
                "os_signal_coverage_stale",
                "report.os_signal_coverage does not match the rows",
                detail={"expected": signals, "declared": report.get("os_signal_coverage")},
            )
        )

    # Recomputed scalar summaries.
    expected_counts = {
        "row_count": len(rows),
        "marketed_row_count": sum(1 for r in rows if projects_evidence(r.get("qualification_status", ""))),
        "restart_or_reload_row_count": sum(1 for r in rows if r.get("apply_posture") in RELOAD_OR_RESTART_POSTURES),
    }
    for key, value in expected_counts.items():
        if report.get(key) != value:
            findings.append(
                Finding("summary_count_stale", f"report.{key} does not match the rows", detail={"expected": value, "declared": report.get(key)})
            )


def check_row(report: dict[str, Any], row: dict[str, Any], findings: list[Finding]) -> None:
    row_id = row.get("row_id") or "<unknown>"

    if row.get("record_kind") != EXPECTED_RECORD_KIND_ROW:
        findings.append(
            Finding("row_record_kind_mismatch", f"row.record_kind must be {EXPECTED_RECORD_KIND_ROW}", row_id=row_id)
        )

    status = row.get("qualification_status", "")
    signal = row.get("os_signal")
    axis = row.get("changed_axis")

    if axis != OS_SIGNAL_AXIS.get(signal):
        findings.append(
            Finding(
                "axis_signal_mismatch",
                "row.changed_axis must match the OS signal's axis",
                row_id=row_id,
                detail={"os_signal": signal, "changed_axis": axis, "expected_axis": OS_SIGNAL_AXIS.get(signal)},
            )
        )

    if row.get("apply_posture") in RELOAD_OR_RESTART_POSTURES and row.get("restart_or_reload_disclosed") is not True:
        findings.append(
            Finding("restart_reload_posture_undisclosed", "a reload/restart posture must be disclosed up front", row_id=row_id)
        )

    if status == "unqualified_local_appearance":
        findings.append(
            Finding("unqualified_local_appearance", "a change rendered outside the appearance-session model is never accepted", row_id=row_id)
        )

    if status in NARROWING_STATUSES and not str(row.get("narrowing_reason") or "").strip():
        findings.append(
            Finding("missing_narrowing_reason", "a narrowed/omitted/declared-gap row must carry a reason", row_id=row_id)
        )

    if not row.get("docs_help_refs"):
        findings.append(Finding("docs_help_ref_missing", "row must carry a docs/help ref", row_id=row_id))

    if projects_evidence(status):
        check_qualified_row(report, row, row_id, findings)


def check_qualified_row(report: dict[str, Any], row: dict[str, Any], row_id: str, findings: list[Finding]) -> None:
    evidence = row.get("evidence")
    if not isinstance(evidence, dict):
        findings.append(Finding("missing_evidence", "a qualified row must carry an evidence capture", row_id=row_id))
        return

    if not str(evidence.get("screenshot_ref", "")).strip() or not str(evidence.get("golden_baseline_ref", "")).strip():
        findings.append(Finding("missing_evidence", "a qualified row's capture must carry screenshot and golden refs", row_id=row_id))

    attribution = evidence.get("attribution", {})
    build_ref = attribution.get("build_identity_ref")
    if not str(build_ref or "").strip() or not str(attribution.get("release_channel_class") or "").strip() or build_ref != report.get("build_identity_ref"):
        findings.append(
            Finding(
                "build_attribution_mismatch",
                "a capture must be attributed to the report's exact build",
                row_id=row_id,
                detail={"capture_build": build_ref, "report_build": report.get("build_identity_ref")},
            )
        )

    for field_name in ("theme_package_ref", "theme_revision_ref", "appearance_session_ref", "checkpoint_ref"):
        if attribution.get(field_name) != row.get(field_name):
            findings.append(
                Finding("evidence_attribution_mismatch", "a capture must be attributed to the row's package/session/checkpoint", row_id=row_id, detail={"field": field_name})
            )
    if attribution.get("platform") != row.get("platform"):
        findings.append(Finding("evidence_attribution_mismatch", "capture platform must match the row", row_id=row_id, detail={"field": "platform"}))
    if attribution.get("os_signal") != row.get("os_signal"):
        findings.append(Finding("evidence_attribution_mismatch", "capture OS signal must match the row", row_id=row_id, detail={"field": "os_signal"}))

    if evidence.get("golden_match") not in ATTRIBUTABLE_GOLDEN:
        findings.append(
            Finding("golden_not_attributable", "a qualified row's golden capture must match a baseline", row_id=row_id, detail={"golden_match": evidence.get("golden_match")})
        )

    if evidence.get("freshness") != "fresh":
        findings.append(Finding("stale_evidence", "a marketed qualified row must carry fresh evidence", row_id=row_id))

    if row.get("apply_posture") == "applies_live" and evidence.get("capture_kind") != "live_transition":
        findings.append(
            Finding("static_evidence_only", "a live-applying change must be proven with a live-transition capture", row_id=row_id)
        )

    cues = row.get("cue_preservation")
    if not isinstance(cues, dict):
        findings.append(Finding("cue_hidden", "a qualified row must carry cue-preservation results", row_id=row_id, detail={"cue": "all"}))
        return

    salience = row.get("semantic_salience")
    salience_cue = {
        "trust_bearing": ("trust_cue", "trust"),
        "severity_bearing": ("severity_cue", "severity"),
        "lifecycle_bearing": ("lifecycle_cue", "lifecycle"),
    }
    for cue_field, token in [("trust_cue", "trust"), ("severity_cue", "severity"), ("lifecycle_cue", "lifecycle")]:
        if cues.get(cue_field) == "hidden":
            findings.append(Finding("cue_hidden", "a trust/severity/lifecycle cue is hidden under the live change", row_id=row_id, detail={"cue": token}))
    if salience in salience_cue:
        cue_field, token = salience_cue[salience]
        if cues.get(cue_field) != "present":
            findings.append(Finding("cue_hidden", "a high-salience surface must present its cue under the live change", row_id=row_id, detail={"cue": token}))

    if cues.get("focus_cue") != "visible_focus_ring":
        findings.append(Finding("focus_not_visible", "the focus ring must stay visible under the live change", row_id=row_id))
    if cues.get("state_semantics") != "preserved":
        findings.append(Finding("state_semantics_lost", "state meaning must be preserved under the live change", row_id=row_id))
    if cues.get("layout_integrity") != "intact":
        findings.append(Finding("layout_corrupted", "layout must stay intact under the live change", row_id=row_id))


def check_support_export(report: dict[str, Any], export: dict[str, Any], findings: list[Finding]) -> None:
    if export.get("record_kind") != EXPECTED_RECORD_KIND_SUPPORT:
        findings.append(
            Finding("support_record_kind_mismatch", f"support_export.record_kind must be {EXPECTED_RECORD_KIND_SUPPORT}")
        )
    case_ids = export.get("case_ids")
    if not isinstance(case_ids, list):
        findings.append(Finding("support_case_ids_missing", "support_export.case_ids must be an array"))
        return
    case_set = set(case_ids)
    if report.get("report_id") not in case_set:
        findings.append(Finding("support_missing_report_id", "case_ids must quote the report id"))
    if report.get("build_identity_ref") not in case_set:
        findings.append(Finding("support_missing_build_ref", "case_ids must quote the exact-build ref"))
    for row in ensure_list(report.get("rows", []), "report.rows"):
        row_id = row.get("row_id")
        for ref in (row.get("row_id"), row.get("appearance_session_ref"), row.get("checkpoint_ref"), row.get("theme_package_ref")):
            if ref not in case_set:
                findings.append(Finding("support_missing_row_ref", "case_ids must quote every row session/checkpoint/package ref", row_id=row_id))
        evidence = row.get("evidence")
        if isinstance(evidence, dict):
            for ref in (evidence.get("screenshot_ref"), evidence.get("golden_baseline_ref")):
                if ref not in case_set:
                    findings.append(Finding("support_missing_capture_ref", "case_ids must quote every screenshot and golden ref", row_id=row_id))


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
            findings.append(Finding("doc_missing_backlink", "companion doc must back-link the canonical artifacts and gate", detail={"backlink": backlink}))


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
    check_coverage(report, rows, findings)
    for row in rows:
        check_row(report, ensure_dict(row, "row"), findings)
    check_support_export(report, export, findings)
    check_publications(repo_root, findings)

    if args.format == "json":
        print(json.dumps({"findings": [f.as_dict() for f in findings]}, indent=2))
    else:
        if not findings:
            print("m5 live-appearance evidence: clean")
        else:
            for finding in findings:
                location = finding.row_id or "report"
                print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
