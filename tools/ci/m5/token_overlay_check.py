#!/usr/bin/env python3
"""M5 token-overlay round-trip portability audit CI gate.

This gate enforces that the checked-in M5 token-overlay portability audit stays
fresh and honest: appearance overrides are scope-explicit objects (with
provenance, portability flags, validation, and an explicit fallback chain), the
winning-versus-shadowed resolution for every token is inspectable, an
unsupported token survives the export / import / sync round trip as an inert or
downgraded entry with a disclosed downgrade note instead of being silently
dropped, rewritten, or treated as fully supported, and overlays stay structured
per scope instead of being flattened into opaque profile blobs. It reads:

- the audit fixture at
  ``fixtures/ux/m5/token-overlay-sync-import/report.json``;
- the support-export fixture at
  ``fixtures/ux/m5/token-overlay-sync-import/support_export.json``;
- the boundary schema at ``schemas/ux/token-overlay.schema.json``;
- the canonical per-token overlay-state schema at
  ``schemas/design/token_overlay.schema.json`` (referenced by the audit); and
- (when present) the published markdown at
  ``artifacts/ux/m5/token-overlay-roundtrip/m5_token_overlay_roundtrip_audit.md``
  and the companion doc at ``docs/m5/token-overlays-and-scope.md``.

For the audit the gate verifies that:

- every override entry is scope-explicit (an inherited entry resolves to the
  theme-package default; an overridden / deprecated / unmapped entry declares a
  real override scope), carries provenance and portability flags, and rides a
  non-empty fallback chain;
- a deprecated entry cites a replacement, an unmapped entry cites its source
  slot and resolves to ``inert_unresolved``, and an unmapped entry carries a
  disclosed downgrade (it is never treated as fully supported);
- every resolved token names exactly one winning scope -- the highest-precedence
  contributing scope -- and lists every shadowed entry;
- every overlay is structured and every entry's declared scope matches its
  overlay's scope;
- no round-trip stage drops or rewrites an entry, every traced entry survives
  with its scope preserved, and any downgrade that survives the round trip is
  disclosed;
- the report carries no blocking finding, is lossless, and preserves at least
  one unsupported entry as an inert or downgraded survivor;
- the support-export wrapper quotes the report id, the appearance-session ref,
  every overlay id, every entry id, every resolved token ref, the proof id, and
  every stage id; and
- the published markdown audit and the companion doc are present and back-link
  the canonical schemas, fixtures, and CLI gate.

Exit codes:

- ``0`` -- audit is clean (no blockers).
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

REPORT_REL = Path("fixtures/ux/m5/token-overlay-sync-import/report.json")
SUPPORT_EXPORT_REL = Path("fixtures/ux/m5/token-overlay-sync-import/support_export.json")
SCHEMA_REL = Path("schemas/ux/token-overlay.schema.json")
CANONICAL_SCHEMA_REL = Path("schemas/design/token_overlay.schema.json")
MARKDOWN_REL = Path(
    "artifacts/ux/m5/token-overlay-roundtrip/m5_token_overlay_roundtrip_audit.md"
)
DOC_REL = Path("docs/m5/token-overlays-and-scope.md")

EXPECTED_RECORD_KIND_REPORT = "shell_m5_token_overlay_portability_report_record"
EXPECTED_RECORD_KIND_OVERLAY = "shell_m5_token_overlay_scope_overlay_record"
EXPECTED_RECORD_KIND_ENTRY = "shell_m5_token_overlay_override_entry_record"
EXPECTED_RECORD_KIND_RESOLVED = "shell_m5_token_overlay_resolved_token_record"
EXPECTED_RECORD_KIND_STAGE = "shell_m5_token_overlay_round_trip_stage_record"
EXPECTED_RECORD_KIND_TRACE = "shell_m5_token_overlay_round_trip_entry_trace_record"
EXPECTED_RECORD_KIND_SUPPORT = "shell_m5_token_overlay_portability_support_export_record"
EXPECTED_SHARED_CONTRACT_REF = "shell:m5_token_overlays:v1"
EXPECTED_SCHEMA_VERSION = 1

# Mirrors OverrideScope::precedence_rank in
# crates/aureline-shell/src/token_overlays/mod.rs.
PRECEDENCE = {
    "theme_package_default": 0,
    "imported_theme": 10,
    "extension_contributed": 20,
    "user_global": 30,
    "profile": 40,
    "workspace": 50,
    "policy_managed": 100,
}


@dataclass
class Finding:
    """One blocking finding emitted by the gate."""

    code: str
    message: str
    subject_ref: str | None = None
    detail: dict[str, Any] = field(default_factory=dict)

    def as_dict(self) -> dict[str, Any]:
        out: dict[str, Any] = {"code": self.code, "message": self.message}
        if self.subject_ref is not None:
            out["subject_ref"] = self.subject_ref
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


def check_report_envelope(report: dict[str, Any], findings: list[Finding]) -> None:
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
    for ref_field in ("published_report_ref", "published_doc_ref"):
        ref = report.get(ref_field)
        if not isinstance(ref, str) or not ref.strip():
            findings.append(
                Finding(
                    "publication_ref_missing",
                    f"report.{ref_field} must be a non-empty string",
                    detail={ref_field: ref},
                )
            )
    if report.get("report_clean") is not True:
        findings.append(
            Finding(
                "report_not_clean",
                "report.report_clean must be true",
                detail={"report_clean": report.get("report_clean")},
            )
        )
    if report.get("round_trip_lossless") is not True:
        findings.append(
            Finding("round_trip_not_lossless", "report.round_trip_lossless must be true")
        )
    declared = ensure_list(report.get("blocking_findings", []), "report.blocking_findings")
    for blocker in declared:
        blocker = ensure_dict(blocker, "blocking_finding")
        findings.append(
            Finding(
                "declared_blocking_finding",
                "report carries a declared blocking finding",
                detail={"class": blocker.get("class")},
            )
        )
    if not ensure_list(report.get("overlays", []), "report.overlays"):
        findings.append(Finding("no_overlays", "report.overlays must be non-empty"))
    if not ensure_list(report.get("resolved_tokens", []), "report.resolved_tokens"):
        findings.append(Finding("no_resolved_tokens", "report.resolved_tokens must be non-empty"))


def check_entry(
    entry: dict[str, Any], overlay_scope: str | None, findings: list[Finding]
) -> None:
    entry_ref = entry.get("entry_id")
    if entry.get("record_kind") != EXPECTED_RECORD_KIND_ENTRY:
        findings.append(
            Finding(
                "entry_record_kind_mismatch",
                f"entry.record_kind must be {EXPECTED_RECORD_KIND_ENTRY}",
                subject_ref=entry_ref,
            )
        )
    scope = entry.get("declared_scope")
    value_state = entry.get("value_state")
    if scope != overlay_scope:
        findings.append(
            Finding(
                "overlay_scope_mismatch",
                "entry declared_scope must match its overlay scope",
                subject_ref=entry_ref,
                detail={"declared_scope": scope, "overlay_scope": overlay_scope},
            )
        )
    inherited = value_state == "inherited"
    scope_is_default = scope == "theme_package_default"
    if inherited != scope_is_default:
        findings.append(
            Finding(
                "entry_scope_not_explicit",
                "an override entry must declare a real scope; an inherited entry must be the theme default",
                subject_ref=entry_ref,
                detail={"value_state": value_state, "declared_scope": scope},
            )
        )
    if not isinstance(entry.get("explanation"), str) or not entry.get("explanation", "").strip():
        findings.append(
            Finding("entry_missing_explanation", "entry.explanation must be non-empty", subject_ref=entry_ref)
        )
    if not ensure_list(entry.get("fallback_chain", []), "entry.fallback_chain"):
        findings.append(
            Finding("entry_fallback_chain_empty", "entry.fallback_chain must be non-empty", subject_ref=entry_ref)
        )
    portability = ensure_dict(entry.get("portability", {}), "entry.portability")
    if "portability_class" not in portability:
        findings.append(
            Finding("entry_missing_portability_flags", "entry must carry portability flags", subject_ref=entry_ref)
        )
    if entry.get("provenance") is None:
        findings.append(
            Finding("entry_missing_provenance", "entry must carry provenance", subject_ref=entry_ref)
        )
    downgrade = entry.get("downgrade_class")
    if value_state == "deprecated" and not entry.get("deprecated_replacement_ref"):
        findings.append(
            Finding(
                "entry_deprecated_without_replacement",
                "a deprecated entry must cite a replacement",
                subject_ref=entry_ref,
            )
        )
    if value_state == "unmapped":
        if not entry.get("unmapped_source_slot_ref") or entry.get("validation_state") != "inert_unresolved":
            findings.append(
                Finding(
                    "entry_unmapped_without_placeholder",
                    "an unmapped entry must cite a source slot and resolve to inert_unresolved",
                    subject_ref=entry_ref,
                )
            )
        if downgrade in (None, "none"):
            findings.append(
                Finding(
                    "entry_unsupported_treated_as_supported",
                    "an unmapped entry must carry a disclosed downgrade",
                    subject_ref=entry_ref,
                )
            )


def check_overlay(overlay: dict[str, Any], findings: list[Finding]) -> None:
    overlay_ref = overlay.get("overlay_id")
    if overlay.get("record_kind") != EXPECTED_RECORD_KIND_OVERLAY:
        findings.append(
            Finding(
                "overlay_record_kind_mismatch",
                f"overlay.record_kind must be {EXPECTED_RECORD_KIND_OVERLAY}",
                subject_ref=overlay_ref,
            )
        )
    if overlay.get("structured") is not True:
        findings.append(
            Finding(
                "overlay_flattened_to_opaque_blob",
                "overlay.structured must be true (overlays are never opaque blobs)",
                subject_ref=overlay_ref,
            )
        )
    entries = ensure_list(overlay.get("entries", []), "overlay.entries")
    if not entries:
        findings.append(
            Finding("overlay_has_no_entries", "overlay.entries must be non-empty", subject_ref=overlay_ref)
        )
    for entry in entries:
        check_entry(ensure_dict(entry, "entry"), overlay.get("scope"), findings)


def collect_entries(report: dict[str, Any]) -> dict[str, dict[str, Any]]:
    out: dict[str, dict[str, Any]] = {}
    for overlay in ensure_list(report.get("overlays", []), "report.overlays"):
        overlay = ensure_dict(overlay, "overlay")
        for entry in ensure_list(overlay.get("entries", []), "overlay.entries"):
            entry = ensure_dict(entry, "entry")
            entry_id = entry.get("entry_id")
            if isinstance(entry_id, str):
                out[entry_id] = entry
    return out


def check_resolution(
    report: dict[str, Any], entries: dict[str, dict[str, Any]], findings: list[Finding]
) -> None:
    # Build the contributing scopes per token.
    by_token: dict[str, list[str]] = {}
    for entry in entries.values():
        by_token.setdefault(entry.get("token_ref"), []).append(entry.get("declared_scope"))

    for resolved in ensure_list(report.get("resolved_tokens", []), "report.resolved_tokens"):
        resolved = ensure_dict(resolved, "resolved_token")
        token_ref = resolved.get("token_ref")
        if resolved.get("record_kind") != EXPECTED_RECORD_KIND_RESOLVED:
            findings.append(
                Finding(
                    "resolved_record_kind_mismatch",
                    f"resolved.record_kind must be {EXPECTED_RECORD_KIND_RESOLVED}",
                    subject_ref=token_ref,
                )
            )
        contributing = by_token.get(token_ref, [])
        if not contributing:
            findings.append(
                Finding("resolved_token_no_winner", "resolved token has no contributing entry", subject_ref=token_ref)
            )
            continue
        expected = max(contributing, key=lambda s: PRECEDENCE.get(s, -1))
        if resolved.get("winning_scope") != expected:
            findings.append(
                Finding(
                    "resolved_token_wrong_winner",
                    "winning scope must be the highest-precedence contributing scope",
                    subject_ref=token_ref,
                    detail={"winning_scope": resolved.get("winning_scope"), "expected": expected},
                )
            )
        winning_entry_ref = resolved.get("winning_entry_ref")
        if winning_entry_ref not in entries:
            findings.append(
                Finding(
                    "resolved_winner_unresolved",
                    "winning_entry_ref does not resolve to a real entry",
                    subject_ref=token_ref,
                    detail={"winning_entry_ref": winning_entry_ref},
                )
            )
        shadowed = ensure_list(resolved.get("shadowed", []), "resolved.shadowed")
        if len(shadowed) + 1 != len(contributing):
            findings.append(
                Finding(
                    "resolved_token_shadowed_not_inspectable",
                    "resolved token must list every shadowed entry",
                    subject_ref=token_ref,
                    detail={"shadowed": len(shadowed), "contributing": len(contributing)},
                )
            )
        if not isinstance(resolved.get("precedence_explained"), str) or not resolved.get(
            "precedence_explained", ""
        ).strip():
            findings.append(
                Finding(
                    "resolved_token_no_explanation",
                    "resolved token must carry a precedence explanation",
                    subject_ref=token_ref,
                )
            )


def check_round_trip(report: dict[str, Any], findings: list[Finding]) -> None:
    proof = ensure_dict(report.get("round_trip", {}), "report.round_trip")
    stages = ensure_list(proof.get("stages", []), "round_trip.stages")
    traces = ensure_list(proof.get("entry_traces", []), "round_trip.entry_traces")
    if not stages:
        findings.append(Finding("no_round_trip_stages", "round_trip.stages must be non-empty"))
    if not traces:
        findings.append(Finding("no_round_trip_traces", "round_trip.entry_traces must be non-empty"))

    for stage in stages:
        stage = ensure_dict(stage, "stage")
        stage_ref = stage.get("stage_id")
        if stage.get("record_kind") != EXPECTED_RECORD_KIND_STAGE:
            findings.append(
                Finding(
                    "stage_record_kind_mismatch",
                    f"stage.record_kind must be {EXPECTED_RECORD_KIND_STAGE}",
                    subject_ref=stage_ref,
                )
            )
        if stage.get("dropped_count", 0) != 0:
            findings.append(
                Finding("round_trip_stage_dropped_entries", "a round-trip stage dropped entries", subject_ref=stage_ref)
            )
        if stage.get("rewritten_count", 0) != 0:
            findings.append(
                Finding("round_trip_stage_rewrote_entries", "a round-trip stage rewrote entries", subject_ref=stage_ref)
            )

    downgraded_survivors = 0
    for trace in traces:
        trace = ensure_dict(trace, "trace")
        entry_ref = trace.get("entry_ref")
        if trace.get("record_kind") != EXPECTED_RECORD_KIND_TRACE:
            findings.append(
                Finding(
                    "trace_record_kind_mismatch",
                    f"trace.record_kind must be {EXPECTED_RECORD_KIND_TRACE}",
                    subject_ref=entry_ref,
                )
            )
        disposition = trace.get("disposition")
        if disposition == "dropped":
            findings.append(
                Finding("round_trip_entry_dropped", "a traced entry was dropped", subject_ref=entry_ref)
            )
        if disposition == "rewritten":
            findings.append(
                Finding("round_trip_entry_rewritten", "a traced entry was rewritten", subject_ref=entry_ref)
            )
        if trace.get("origin_scope") != trace.get("final_scope"):
            findings.append(
                Finding("round_trip_scope_lost", "a traced entry lost its scope", subject_ref=entry_ref)
            )
        if disposition == "downgraded":
            if trace.get("downgrade_class") in (None, "none"):
                findings.append(
                    Finding(
                        "round_trip_downgrade_not_disclosed",
                        "a downgraded trace must disclose its downgrade",
                        subject_ref=entry_ref,
                    )
                )
            if trace.get("survived") is True:
                downgraded_survivors += 1

    if traces and downgraded_survivors == 0:
        findings.append(
            Finding(
                "no_unsupported_survivor",
                "the round trip must preserve at least one unsupported entry as an inert or downgraded survivor",
            )
        )


def check_support_export(
    report: dict[str, Any], export: dict[str, Any], findings: list[Finding]
) -> None:
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

    def require(ref: Any, code: str, message: str) -> None:
        if ref not in case_set:
            findings.append(Finding(code, message, detail={"ref": ref}))

    require(report.get("report_id"), "support_missing_report_id", "case_ids must quote the report id")
    require(
        report.get("appearance_session_ref"),
        "support_missing_session_ref",
        "case_ids must quote the appearance-session ref",
    )
    for overlay in ensure_list(report.get("overlays", []), "report.overlays"):
        overlay = ensure_dict(overlay, "overlay")
        require(overlay.get("overlay_id"), "support_missing_overlay_id", "case_ids must quote every overlay id")
        for entry in ensure_list(overlay.get("entries", []), "overlay.entries"):
            entry = ensure_dict(entry, "entry")
            require(entry.get("entry_id"), "support_missing_entry_id", "case_ids must quote every entry id")
    for resolved in ensure_list(report.get("resolved_tokens", []), "report.resolved_tokens"):
        resolved = ensure_dict(resolved, "resolved_token")
        require(resolved.get("token_ref"), "support_missing_token_ref", "case_ids must quote every resolved token")
    proof = ensure_dict(report.get("round_trip", {}), "report.round_trip")
    require(proof.get("proof_id"), "support_missing_proof_id", "case_ids must quote the proof id")
    for stage in ensure_list(proof.get("stages", []), "round_trip.stages"):
        stage = ensure_dict(stage, "stage")
        require(stage.get("stage_id"), "support_missing_stage_id", "case_ids must quote every stage id")


DOC_BACKLINKS = (
    "artifacts/ux/m5/token-overlay-roundtrip/m5_token_overlay_roundtrip_audit.md",
    "fixtures/ux/m5/token-overlay-sync-import/report.json",
    "schemas/ux/token-overlay.schema.json",
    "schemas/design/token_overlay.schema.json",
    "tools/ci/m5/token_overlay_check.py",
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
    for schema_rel in (SCHEMA_REL, CANONICAL_SCHEMA_REL):
        if not (repo_root / schema_rel).exists():
            raise SystemExit(f"missing required input: {schema_rel}")

    findings: list[Finding] = []
    check_report_envelope(report, findings)
    for overlay in ensure_list(report.get("overlays", []), "report.overlays"):
        check_overlay(ensure_dict(overlay, "overlay"), findings)
    entries = collect_entries(report)
    check_resolution(report, entries, findings)
    check_round_trip(report, findings)
    check_support_export(report, export, findings)
    check_publications(repo_root, findings)

    if args.format == "json":
        print(json.dumps({"findings": [f.as_dict() for f in findings]}, indent=2))
    else:
        if not findings:
            print("m5 token-overlay round-trip audit: clean")
        else:
            for finding in findings:
                location = finding.subject_ref or "report"
                print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
