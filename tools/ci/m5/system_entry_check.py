#!/usr/bin/env python3
"""System-open and file-association intake CI gate.

This gate enforces that the checked-in system-entry intake report stays fresh
and clean across the six required OS-initiated intake kinds (file, folder,
workspace, review_link, patch_bundle, provider_return). It reads:

- the report fixture at ``fixtures/platform/m5-system-entry/report.json``;
- the support-export fixture at
  ``fixtures/platform/m5-system-entry/support_export.json``;
- the four per-incident case exports under
  ``fixtures/platform/m5-system-entry/cases/``;
- the boundary schema at ``schemas/platform/m5-system-entry.schema.json``; and
- (when present) the published markdown at
  ``artifacts/platform/m5-system-open-and-file-association.md`` and the
  companion doc at ``docs/m5/system-open-and-file-association.md``.

For the report the gate verifies that:

- the report covers every required intake kind;
- at least one intake provably reuses the in-product project-entry path, and an
  ``entry_flow_resolved`` intake records that reuse while a routed intake names
  its reviewed surface;
- every intake carries a literal target, a canonical target, an active-profile
  owner, a channel/build owner, a trust checkpoint, the canonical in-product
  command, a non-empty continuity note, a non-empty degraded-state vocabulary,
  at least one platform, a downgrade rule, and
  ``registered_on_system_entry_harness = true``;
- any scope wider than a plain local read is gated behind an explicit
  interstitial that names an interstitial ref;
- any non-exact target offers at least one recovery action;
- no marketed intake carries stale evidence;
- no intake carries any blocking finding, so the distinct failure classes (a
  silent scope widen, a silent provider mutation, a coerced verb, a wrong-target
  dead-end, a silent loss on an unavailable path, an unsafe policy block, a
  bypassed trust evaluation, and a hidden channel owner) are all caught;
- the report cross-links the native-desktop matrix, install-topology,
  project-entry contract, entry interstitials, handoff-review, and auth-recovery
  packets;
- the support-export wrapper quotes the report id, every intake id, and every
  descriptor revision; and
- the four incident case exports (wrong_association, moved_target, mixed_root,
  policy_blocked) exist, each carries a non-exact availability with at least one
  recovery action, and the published markdown and companion doc back-link the
  canonical schema, fixtures, and CLI gate.

Exit codes:

- ``0`` -- report is clean (all kinds covered, parity present, no blockers).
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

REPORT_REL = Path("fixtures/platform/m5-system-entry/report.json")
SUPPORT_EXPORT_REL = Path("fixtures/platform/m5-system-entry/support_export.json")
CASES_DIR_REL = Path("fixtures/platform/m5-system-entry/cases")
SCHEMA_REL = Path("schemas/platform/m5-system-entry.schema.json")
MARKDOWN_REL = Path("artifacts/platform/m5-system-open-and-file-association.md")
DOC_REL = Path("docs/m5/system-open-and-file-association.md")

REQUIRED_KINDS = (
    "file",
    "folder",
    "workspace",
    "review_link",
    "patch_bundle",
    "provider_return",
)

CROSS_LINK_FIELDS = (
    "native_desktop_matrix_ref",
    "install_topology_ref",
    "project_entry_contract_ref",
    "entry_interstitial_ref",
    "handoff_review_ref",
    "auth_recovery_ref",
)

# (intake_id, case_label) for the four required incident exports.
REQUIRED_CASES = (
    ("intake:case.wrong_association", "wrong_association"),
    ("intake:case.moved_target", "moved_target"),
    ("intake:case.mixed_root", "mixed_root"),
    ("intake:case.policy_blocked", "policy_blocked"),
)

EXPECTED_RECORD_KIND_REPORT = "shell_m5_system_entry_report_record"
EXPECTED_RECORD_KIND_ROW = "shell_m5_system_entry_intake_record"
EXPECTED_RECORD_KIND_SUPPORT = "shell_m5_system_entry_support_export_record"
EXPECTED_RECORD_KIND_CASE = "shell_m5_system_entry_case_export_record"
EXPECTED_SHARED_CONTRACT_REF = "shell:m5_system_entry:v1"
EXPECTED_SCHEMA_VERSION = 1

NON_EXACT_AVAILABILITY = (
    "wrong_association",
    "moved_target",
    "mixed_root",
    "blocked_by_policy",
    "missing_or_unmounted",
)

DOC_BACKLINKS = (
    "artifacts/platform/m5-system-open-and-file-association.md",
    "fixtures/platform/m5-system-entry/report.json",
    "schemas/platform/m5-system-entry.schema.json",
    "tools/ci/m5/system_entry_check.py",
)


@dataclass
class Finding:
    """One blocking finding emitted by the gate."""

    code: str
    message: str
    intake_id: str | None = None
    detail: dict[str, Any] = field(default_factory=dict)

    def as_dict(self) -> dict[str, Any]:
        out: dict[str, Any] = {"code": self.code, "message": self.message}
        if self.intake_id is not None:
            out["intake_id"] = self.intake_id
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


def is_nonempty_str(value: Any) -> bool:
    return isinstance(value, str) and value.strip() != ""


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
    if report.get("report_clean") is not True:
        findings.append(Finding("report_not_clean", "report.report_clean must be true"))


def check_cross_links(report: dict[str, Any], findings: list[Finding]) -> None:
    cross_links = report.get("cross_links")
    if not isinstance(cross_links, dict):
        findings.append(Finding("cross_links_missing", "report.cross_links must be an object"))
        return
    for field_name in CROSS_LINK_FIELDS:
        if not is_nonempty_str(cross_links.get(field_name)):
            findings.append(
                Finding("cross_link_missing", "report.cross_links field is empty", detail={"field": field_name})
            )


def check_required_coverage(report: dict[str, Any], findings: list[Finding]) -> None:
    entries = ensure_list(report.get("entries", []), "report.entries")
    present_kinds = {
        ensure_dict(entry.get("descriptor", {}), "entry.descriptor").get("intake_kind")
        for entry in entries
    }
    for kind in REQUIRED_KINDS:
        if kind not in present_kinds:
            findings.append(
                Finding("missing_required_kind", "report is missing a required intake kind", detail={"kind": kind})
            )
    any_parity = any(
        entry.get("parity_outcome", {}).get("reuses_project_entry_path") is True
        for entry in entries
    )
    if not any_parity:
        findings.append(
            Finding("no_project_entry_parity", "no intake provably reuses the in-product project-entry path")
        )


def check_entry(entry: dict[str, Any], findings: list[Finding]) -> None:
    if entry.get("record_kind") != EXPECTED_RECORD_KIND_ROW:
        findings.append(
            Finding(
                "row_record_kind_mismatch",
                f"entry.record_kind must be {EXPECTED_RECORD_KIND_ROW}",
                detail={"record_kind": entry.get("record_kind")},
            )
        )
    descriptor = ensure_dict(entry.get("descriptor", {}), "entry.descriptor")
    intake_id = descriptor.get("intake_id")

    for field_name in (
        "literal_target_ref",
        "canonical_target_ref",
        "active_profile_owner_ref",
        "channel_build_owner_ref",
        "trust_checkpoint_ref",
        "canonical_command_ref",
        "continuity_note",
        "downgrade_rule_ref",
        "descriptor_revision_ref",
    ):
        if not is_nonempty_str(descriptor.get(field_name)):
            findings.append(
                Finding("descriptor_field_missing", "descriptor field is empty", intake_id=intake_id, detail={"field": field_name})
            )

    vocab = descriptor.get("degraded_state_vocabulary")
    if not isinstance(vocab, list) or not any(is_nonempty_str(v) for v in vocab):
        findings.append(Finding("missing_degraded_state_vocabulary", "descriptor.degraded_state_vocabulary must be non-empty", intake_id=intake_id))

    platforms = descriptor.get("claimed_platforms")
    if not isinstance(platforms, list) or not platforms:
        findings.append(Finding("missing_claimed_platforms", "descriptor.claimed_platforms must be non-empty", intake_id=intake_id))

    if descriptor.get("registered_on_system_entry_harness") is not True:
        findings.append(
            Finding("intake_not_on_harness", "descriptor.registered_on_system_entry_harness must be true", intake_id=intake_id)
        )

    if descriptor.get("marketed") and descriptor.get("evidence_freshness") == "stale":
        findings.append(
            Finding("stale_evidence_on_marketed_intake", "marketed intake carries stale evidence", intake_id=intake_id)
        )

    # Scope discipline: anything wider than a plain local read must be gated.
    scope_class = descriptor.get("scope_class")
    if scope_class and scope_class != "plain_local_read":
        if descriptor.get("requires_explicit_interstitial") is not True:
            findings.append(
                Finding("scope_widen_not_gated", "scope wider than plain_local_read must require an interstitial", intake_id=intake_id, detail={"scope_class": scope_class})
            )
        if not is_nonempty_str(descriptor.get("interstitial_ref")):
            findings.append(
                Finding("missing_interstitial", "a gated intake must name an interstitial ref", intake_id=intake_id, detail={"scope_class": scope_class})
            )

    # Parity: entry-flow intakes must record reuse; routed intakes must name a
    # reviewed surface.
    parity_class = descriptor.get("parity_class")
    parity_outcome = ensure_dict(entry.get("parity_outcome", {}), "entry.parity_outcome")
    if parity_class == "entry_flow_resolved":
        if parity_outcome.get("reuses_project_entry_path") is not True:
            findings.append(
                Finding("verb_coercion", "entry_flow_resolved intake does not reuse the canonical resolution", intake_id=intake_id)
            )
    elif parity_class in ("routed_to_review_surface", "routed_to_auth_recovery"):
        if not is_nonempty_str(descriptor.get("routed_surface_ref")):
            findings.append(
                Finding("missing_routed_surface", "routed intake must name a reviewed surface", intake_id=intake_id)
            )

    # Recovery: a non-exact target must offer a recovery action.
    availability = descriptor.get("availability")
    if availability in NON_EXACT_AVAILABILITY:
        recovery = descriptor.get("recovery_actions")
        if not isinstance(recovery, list) or not recovery:
            findings.append(
                Finding("missing_recovery_action", "non-exact target must offer a recovery action", intake_id=intake_id, detail={"availability": availability})
            )

    for blocker in ensure_list(entry.get("blocking_findings", []), "entry.blocking_findings"):
        findings.append(
            Finding(
                "blocking_finding_present",
                "intake carries a blocking finding",
                intake_id=intake_id,
                detail={"class": blocker.get("class")},
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
    if report.get("report_id") not in case_set:
        findings.append(
            Finding("support_missing_report_id", "support_export.case_ids must quote the report id", detail={"report_id": report.get("report_id")})
        )
    for entry in ensure_list(report.get("entries", []), "report.entries"):
        descriptor = ensure_dict(entry.get("descriptor", {}), "entry.descriptor")
        intake_id = descriptor.get("intake_id")
        revision = descriptor.get("descriptor_revision_ref")
        if intake_id not in case_set:
            findings.append(Finding("support_missing_intake_id", "support_export.case_ids must quote every intake id", intake_id=intake_id))
        if revision not in case_set:
            findings.append(
                Finding("support_missing_descriptor_revision", "support_export.case_ids must quote every descriptor revision", intake_id=intake_id, detail={"descriptor_revision_ref": revision})
            )


def check_case_exports(repo_root: Path, findings: list[Finding]) -> None:
    for intake_id, label in REQUIRED_CASES:
        path = repo_root / CASES_DIR_REL / f"{label}.json"
        if not path.exists():
            findings.append(Finding("case_export_missing", "missing required incident case export", detail={"case": label}))
            continue
        export = ensure_dict(load_json(path), f"case[{label}]")
        if export.get("record_kind") != EXPECTED_RECORD_KIND_CASE:
            findings.append(Finding("case_record_kind_mismatch", "case export record_kind mismatch", detail={"case": label, "record_kind": export.get("record_kind")}))
        if export.get("case_label") != label:
            findings.append(Finding("case_label_mismatch", "case export label mismatch", detail={"case": label, "case_label": export.get("case_label")}))
        if export.get("availability") not in NON_EXACT_AVAILABILITY:
            findings.append(Finding("case_availability_not_degraded", "case export must carry a non-exact availability", detail={"case": label, "availability": export.get("availability")}))
        recovery = export.get("recovery_actions")
        if not isinstance(recovery, list) or not recovery:
            findings.append(Finding("case_missing_recovery", "case export must offer a recovery action", detail={"case": label}))
        intake = ensure_dict(export.get("intake", {}), f"case[{label}].intake")
        descriptor = ensure_dict(intake.get("descriptor", {}), f"case[{label}].intake.descriptor")
        if descriptor.get("intake_id") != intake_id:
            findings.append(Finding("case_intake_id_mismatch", "case export intake id mismatch", detail={"case": label, "intake_id": descriptor.get("intake_id")}))


def check_publications(repo_root: Path, findings: list[Finding]) -> None:
    markdown = repo_root / MARKDOWN_REL
    if not markdown.exists():
        findings.append(Finding("published_markdown_missing", f"missing published markdown: {MARKDOWN_REL}"))
    doc = repo_root / DOC_REL
    if not doc.exists():
        findings.append(Finding("published_doc_missing", f"missing companion doc: {DOC_REL}"))
        return
    body = doc.read_text(encoding="utf-8")
    for kind in REQUIRED_KINDS:
        if kind not in body:
            findings.append(Finding("doc_missing_kind", "companion doc must quote every required intake kind", detail={"kind": kind}))
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

    findings: list[Finding] = []
    check_report_envelope(report, findings)
    check_cross_links(report, findings)
    check_required_coverage(report, findings)
    for entry in ensure_list(report.get("entries", []), "report.entries"):
        check_entry(ensure_dict(entry, "entry"), findings)
    check_support_export(report, export, findings)
    check_case_exports(repo_root, findings)
    check_publications(repo_root, findings)

    if args.format == "json":
        print(json.dumps({"findings": [f.as_dict() for f in findings]}, indent=2))
    else:
        if not findings:
            print("m5 system-entry intake: clean")
        else:
            for finding in findings:
                location = finding.intake_id or "report"
                print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
