#!/usr/bin/env python3
"""M5 command-parity and discoverability audit CI gate.

This gate enforces that the checked-in M5 command-parity audit stays
fresh and clean across the six required discoverability channels:
command palette, keybinding help, help search, onboarding/tour,
CLI/headless help, and AI automation. It reads:

- the audit fixture at ``fixtures/ux/m5/command-parity/report.json``;
- the support-export fixture at
  ``fixtures/ux/m5/command-parity/support_export.json``;
- the boundary schema at
  ``schemas/commands/m5-command-descriptor-diff.schema.json``; and
- (when present) the published markdown at
  ``artifacts/ux/m5/discoverability-audits/m5_command_parity_audit.md``
  and the companion doc at ``docs/ux/m5/command_parity_audit.md``.

For the audit the gate verifies that:

- the audit covers all six required channels and at least one row
  claims each channel;
- every registered command has a projection for every required channel;
- every command carries a canonical help anchor, non-empty search
  metadata, and ``promoted_to_stable_graph = true``;
- every high-risk command (non-trivial preview class or non-trivial
  capability scope) requires a typed disabled reason and has no
  ``unknown_high_risk_gap`` projection;
- no row carries any blocking finding (so pointer-only islands and any
  descriptor/projection drift are caught);
- the support-export wrapper quotes every command id and descriptor
  revision the audit exposes; and
- the published markdown audit and the companion doc are present and
  back-link the canonical schema, fixtures, and CLI gate.

Exit codes:

- ``0`` -- audit is clean (all six channels claimed, no blockers).
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

REPORT_REL = Path("fixtures/ux/m5/command-parity/report.json")
SUPPORT_EXPORT_REL = Path("fixtures/ux/m5/command-parity/support_export.json")
COMPACT_REL = Path("fixtures/ux/m5/command-parity/compact.txt")
SCHEMA_REL = Path("schemas/commands/m5-command-descriptor-diff.schema.json")
MARKDOWN_REL = Path("artifacts/ux/m5/discoverability-audits/m5_command_parity_audit.md")
DOC_REL = Path("docs/ux/m5/command_parity_audit.md")

REQUIRED_CHANNELS = (
    "command_palette",
    "keybinding_help",
    "help_search",
    "onboarding_tour",
    "cli_headless",
    "ai_automation",
)

EXPECTED_RECORD_KIND_REPORT = "shell_m5_command_parity_audit_report_record"
EXPECTED_RECORD_KIND_ROW = "shell_m5_command_parity_row_record"
EXPECTED_RECORD_KIND_SUPPORT = "shell_m5_command_parity_support_export_record"
EXPECTED_SHARED_CONTRACT_REF = "shell:m5_command_parity:v1"
EXPECTED_SCHEMA_VERSION = 1

HIGH_RISK_PREVIEW_CLASSES = {
    "structured_diff_preview",
    "destructive_bulk_mutation_preview",
    "policy_authoring_or_waiver_preview",
    "irreversible_publish_preview",
}
HIGH_RISK_CAPABILITY_SCOPES = {
    "recoverable_durable_mutation",
    "destructive_bulk_mutation",
    "irreversible_publish",
}

DOC_BACKLINKS = (
    "artifacts/ux/m5/discoverability-audits/m5_command_parity_audit.md",
    "fixtures/ux/m5/command-parity/report.json",
    "schemas/commands/m5-command-descriptor-diff.schema.json",
    "tools/ci/m5/command_parity_check.py",
)


@dataclass
class Finding:
    """One blocking finding emitted by the gate."""

    code: str
    message: str
    command_id: str | None = None
    channel: str | None = None
    detail: dict[str, Any] = field(default_factory=dict)

    def as_dict(self) -> dict[str, Any]:
        out: dict[str, Any] = {"code": self.code, "message": self.message}
        if self.command_id is not None:
            out["command_id"] = self.command_id
        if self.channel is not None:
            out["channel"] = self.channel
        if self.detail:
            out["detail"] = self.detail
        return out


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--repo-root",
        default=".",
        help="Path to the repository root (default: cwd).",
    )
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


def descriptor_high_risk(descriptor: dict[str, Any]) -> bool:
    preview = descriptor.get("preview_class")
    capability = descriptor.get("capability_scope_class")
    return preview in HIGH_RISK_PREVIEW_CLASSES or capability in HIGH_RISK_CAPABILITY_SCOPES


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
    declared = report.get("required_channels")
    if declared != list(REQUIRED_CHANNELS):
        findings.append(
            Finding(
                "required_channels_mismatch",
                "required_channels must equal the canonical channel list",
                detail={"required": list(REQUIRED_CHANNELS), "declared": declared},
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


def check_required_channels_claimed(report: dict[str, Any], findings: list[Finding]) -> None:
    rows = ensure_list(report.get("rows", []), "report.rows")
    for required in REQUIRED_CHANNELS:
        any_claimed = False
        for row in rows:
            for projection in ensure_list(row.get("channels", []), "row.channels"):
                if (
                    projection.get("channel") == required
                    and projection.get("coverage_status") == "claimed"
                ):
                    any_claimed = True
                    break
            if any_claimed:
                break
        if not any_claimed:
            findings.append(
                Finding(
                    "required_channel_not_claimed",
                    "no claimed row for required channel",
                    channel=required,
                )
            )


def check_row(row: dict[str, Any], findings: list[Finding]) -> None:
    descriptor = ensure_dict(row.get("descriptor", {}), "row.descriptor")
    command_id = descriptor.get("command_id")
    if not isinstance(command_id, str) or not command_id.strip():
        findings.append(Finding("missing_command_id", "descriptor.command_id must be non-empty"))
        return

    if row.get("record_kind") != EXPECTED_RECORD_KIND_ROW:
        findings.append(
            Finding(
                "row_record_kind_mismatch",
                f"row.record_kind must be {EXPECTED_RECORD_KIND_ROW}",
                command_id=command_id,
                detail={"record_kind": row.get("record_kind")},
            )
        )

    revision = descriptor.get("descriptor_revision_ref")
    if not isinstance(revision, str) or not revision.strip():
        findings.append(
            Finding(
                "missing_descriptor_revision_ref",
                "descriptor.descriptor_revision_ref must be non-empty",
                command_id=command_id,
            )
        )

    anchor = descriptor.get("help_anchor_ref")
    if not isinstance(anchor, str) or not anchor.strip():
        findings.append(
            Finding(
                "descriptor_missing_help_anchor",
                "descriptor.help_anchor_ref must be non-empty",
                command_id=command_id,
            )
        )

    keywords = descriptor.get("search_keywords")
    if not isinstance(keywords, list) or not any(
        isinstance(k, str) and k.strip() for k in keywords
    ):
        findings.append(
            Finding(
                "missing_search_metadata",
                "descriptor.search_keywords must contain at least one non-empty keyword",
                command_id=command_id,
            )
        )

    if descriptor.get("promoted_to_stable_graph") is not True:
        findings.append(
            Finding(
                "command_not_promoted",
                "descriptor.promoted_to_stable_graph must be true",
                command_id=command_id,
            )
        )

    high_risk = descriptor_high_risk(descriptor)
    if high_risk and descriptor.get("disabled_reason_mode") != "typed_reason_required_when_unavailable":
        findings.append(
            Finding(
                "missing_disabled_reason_mode",
                "high-risk command must require a typed disabled reason",
                command_id=command_id,
            )
        )

    # Every required channel must be projected.
    channels = ensure_list(row.get("channels", []), "row.channels")
    present = {projection.get("channel") for projection in channels}
    for required in REQUIRED_CHANNELS:
        if required not in present:
            findings.append(
                Finding(
                    "missing_required_channel",
                    "command is missing a required channel projection",
                    command_id=command_id,
                    channel=required,
                )
            )

    for projection in channels:
        channel = projection.get("channel")
        status = projection.get("coverage_status")
        if status == "custom_pane_only":
            findings.append(
                Finding(
                    "pointer_only_affordance",
                    "command is reachable only through its own pane",
                    command_id=command_id,
                    channel=channel,
                )
            )
        if status == "unknown_high_risk_gap" and high_risk:
            findings.append(
                Finding(
                    "unknown_high_risk_gap",
                    "high-risk command has an unknown projection on a required channel",
                    command_id=command_id,
                    channel=channel,
                )
            )

    # Any blocking finding the Rust validator emitted is a gate failure.
    for blocker in ensure_list(row.get("blocking_findings", []), "row.blocking_findings"):
        findings.append(
            Finding(
                "blocking_finding_present",
                "row carries a blocking finding",
                command_id=command_id,
                channel=blocker.get("channel"),
                detail={"class": blocker.get("class")},
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
        findings.append(
            Finding("support_case_ids_missing", "support_export.case_ids must be an array")
        )
        return
    case_set = set(case_ids)
    report_id = report.get("report_id")
    if report_id not in case_set:
        findings.append(
            Finding(
                "support_missing_report_id",
                "support_export.case_ids must quote the report id",
                detail={"report_id": report_id},
            )
        )
    for row in ensure_list(report.get("rows", []), "report.rows"):
        descriptor = ensure_dict(row.get("descriptor", {}), "row.descriptor")
        command_id = descriptor.get("command_id")
        revision = descriptor.get("descriptor_revision_ref")
        if command_id not in case_set:
            findings.append(
                Finding(
                    "support_missing_command_id",
                    "support_export.case_ids must quote every command id",
                    command_id=command_id,
                )
            )
        if revision not in case_set:
            findings.append(
                Finding(
                    "support_missing_descriptor_revision",
                    "support_export.case_ids must quote every descriptor revision",
                    command_id=command_id,
                    detail={"descriptor_revision_ref": revision},
                )
            )


def check_publications(repo_root: Path, findings: list[Finding]) -> None:
    markdown = repo_root / MARKDOWN_REL
    if not markdown.exists():
        findings.append(
            Finding("published_markdown_missing", f"missing published markdown: {MARKDOWN_REL}")
        )
    doc = repo_root / DOC_REL
    if not doc.exists():
        findings.append(Finding("published_doc_missing", f"missing companion doc: {DOC_REL}"))
        return
    body = doc.read_text(encoding="utf-8")
    for channel in REQUIRED_CHANNELS:
        if channel not in body:
            findings.append(
                Finding(
                    "doc_missing_channel",
                    "companion doc must quote every required channel",
                    channel=channel,
                )
            )
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
    # The schema is required to exist so the contract stays discoverable.
    if not (repo_root / SCHEMA_REL).exists():
        raise SystemExit(f"missing required input: {SCHEMA_REL}")

    findings: list[Finding] = []
    check_report_envelope(report, findings)
    check_required_channels_claimed(report, findings)
    for row in ensure_list(report.get("rows", []), "report.rows"):
        check_row(ensure_dict(row, "row"), findings)
    check_support_export(report, export, findings)
    check_publications(repo_root, findings)

    if args.format == "json":
        print(json.dumps({"findings": [f.as_dict() for f in findings]}, indent=2))
    else:
        if not findings:
            print("m5 command-parity audit: clean")
        else:
            for finding in findings:
                location = finding.command_id or "report"
                if finding.channel:
                    location = f"{location} / {finding.channel}"
                print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
