#!/usr/bin/env python3
"""Generate and validate the beta reference-workspace report.

The report is derived from the reference-workspace register, workspace
packets, and current harness result rows. It is also a publication gate:
claim-manifest archetype rows may not publish a stronger support class than
the corresponding reference-workspace report row.
"""

from __future__ import annotations

import argparse
import dataclasses
import datetime as dt
import json
import re
import subprocess
import sys
from pathlib import Path
from typing import Any


DEFAULT_REGISTER_REL = "artifacts/compat/m3/reference_workspace_register.yaml"
DEFAULT_REFERENCE_ROWS_REL = "artifacts/compat/reference_workspace_rows.yaml"
DEFAULT_CLAIMED_SURFACE_REGISTER_REL = (
    "artifacts/milestones/m3/claimed_surface_register.json"
)
DEFAULT_CLAIM_MANIFEST_REL = "artifacts/release/m3/claim_manifest.json"
DEFAULT_REPORT_JSON_REL = "artifacts/compat/m3/reference_workspace_report.json"
DEFAULT_REPORT_MD_REL = "artifacts/compat/m3/reference_workspace_report.md"
DEFAULT_DOC_MD_REL = "docs/compat/m3/reference_workspace_report.md"
DEFAULT_BADGES_REL = "artifacts/compat/m3/reference_workspace_badges.json"
DEFAULT_CAPTURE_REL = (
    "artifacts/compat/m3/captures/reference_workspace_report_validation_capture.json"
)

REPORT_ROW_PREFIX = "reference_workspace_report_row"
MISSING_BETA_HARNESS = "missing_beta_harness"
MISSING_WORKSPACE_PACKET = "missing_workspace_packet"

SUPPORT_STRICTNESS = {
    "certified": 0,
    "supported": 1,
    "limited": 2,
    "experimental": 3,
    "community": 3,
    "preview": 3,
    "retest_pending": 4,
    "evidence_stale": 5,
    "unsupported": 6,
}


@dataclasses.dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    remediation: str
    ref: str | None = None
    details: dict[str, Any] = dataclasses.field(default_factory=dict)

    def as_report(self) -> dict[str, Any]:
        payload = dataclasses.asdict(self)
        if payload["ref"] is None:
            payload.pop("ref")
        if not payload["details"]:
            payload.pop("details")
        return payload


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--register", default=DEFAULT_REGISTER_REL)
    parser.add_argument("--reference-rows", default=DEFAULT_REFERENCE_ROWS_REL)
    parser.add_argument(
        "--claimed-surface-register",
        default=DEFAULT_CLAIMED_SURFACE_REGISTER_REL,
    )
    parser.add_argument("--claim-manifest", default=DEFAULT_CLAIM_MANIFEST_REL)
    parser.add_argument("--report-json", default=DEFAULT_REPORT_JSON_REL)
    parser.add_argument("--report-md", default=DEFAULT_REPORT_MD_REL)
    parser.add_argument("--doc-md", default=DEFAULT_DOC_MD_REL)
    parser.add_argument("--badges", default=DEFAULT_BADGES_REL)
    parser.add_argument("--capture", default=DEFAULT_CAPTURE_REL)
    parser.add_argument(
        "--check",
        action="store_true",
        help="Fail if checked-in report, doc, badges, or capture would change.",
    )
    return parser.parse_args()


def render_yaml_as_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing YAML file: {path}")
    ruby = subprocess.run(
        [
            "ruby",
            "-rjson",
            "-ryaml",
            "-e",
            (
                "require 'date';"
                " payload = YAML.safe_load(File.read(ARGV[0]),"
                " permitted_classes: [Date, Time], aliases: false);"
                " STDOUT.write(JSON.generate(payload))"
            ),
            str(path),
        ],
        capture_output=True,
        text=True,
    )
    if ruby.returncode != 0:
        stderr = ruby.stderr.strip() or "unknown Ruby/Psych failure"
        raise SystemExit(f"failed to parse YAML at {path}: {stderr}")
    try:
        return json.loads(ruby.stdout)
    except json.JSONDecodeError as exc:
        raise SystemExit(f"Ruby/Psych emitted invalid JSON for {path}: {exc}") from exc


def load_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing JSON file: {path}")
    return json.loads(path.read_text(encoding="utf-8"))


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be an object/mapping")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a list/array")
    return value


def ensure_str(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value.strip():
        raise SystemExit(f"{label} must be a non-empty string")
    return value.strip()


def strip_fragment(ref: str) -> str:
    return ref.split("#", 1)[0].strip()


def parse_date(value: str) -> dt.date:
    return dt.date.fromisoformat(value)


def add_days(date_value: str, days: int) -> str:
    return (parse_date(date_value) + dt.timedelta(days=days)).isoformat()


def today_utc() -> str:
    return dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def index_reference_rows(reference_rows: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {
        ensure_str(row.get("archetype_row_id"), "archetype_row_id"): row
        for row in ensure_list(reference_rows.get("archetype_rows"), "reference_rows.archetype_rows")
        if isinstance(row, dict)
    }


def index_claimed_archetypes(register: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {
        ensure_str(row.get("archetype_row_ref"), "archetype_row_ref"): row
        for row in ensure_list(register.get("claimed_archetype_rows"), "claimed_archetype_rows")
        if isinstance(row, dict)
    }


def index_registered_workspaces(register: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {
        ensure_str(row.get("archetype_row_ref"), "archetype_row_ref"): row
        for row in ensure_list(register.get("reference_workspaces"), "register.reference_workspaces")
        if isinstance(row, dict)
    }


def read_optional_yaml(repo_root: Path, ref: str | None) -> dict[str, Any] | None:
    if not ref:
        return None
    path = repo_root / strip_fragment(ref)
    if not path.exists():
        return None
    return ensure_dict(render_yaml_as_json(path), ref)


def descriptor_for(repo_root: Path, ref: str | None) -> dict[str, Any] | None:
    if not ref:
        return None
    path = repo_root / strip_fragment(ref)
    if not path.exists():
        return None
    return ensure_dict(load_json(path), ref)


def toolchain_from(row: dict[str, Any], packet: dict[str, Any] | None, descriptor: dict[str, Any] | None) -> dict[str, Any]:
    if packet is not None:
        packet_toolchain = ensure_dict(packet.get("toolchain_manifest"), "packet.toolchain_manifest")
        exact_versions = ensure_dict(packet_toolchain.get("exact_versions"), "packet.toolchain_manifest.exact_versions")
        mode_coverage = ensure_dict(packet_toolchain.get("mode_coverage"), "packet.toolchain_manifest.mode_coverage")
        return {
            "manifest_id": packet_toolchain.get("manifest_id"),
            "exact_versions": exact_versions,
            "os_arch_coverage_targets": ensure_list(
                packet_toolchain.get("os_arch_coverage_targets"),
                "packet.toolchain_manifest.os_arch_coverage_targets",
            ),
            "supported_modes": ensure_list(mode_coverage.get("supported"), "packet.mode_coverage.supported"),
            "not_claimed_modes": ensure_list(mode_coverage.get("not_claimed"), "packet.mode_coverage.not_claimed"),
            "version_source_ref": packet_toolchain.get("version_source_ref"),
        }
    if row.get("toolchain_manifest"):
        row_toolchain = ensure_dict(row.get("toolchain_manifest"), "register.toolchain_manifest")
        return {
            "manifest_id": row_toolchain.get("manifest_id"),
            "exact_versions": ensure_dict(row_toolchain.get("exact_versions"), "register.exact_versions"),
            "os_arch_coverage_targets": ensure_list(row_toolchain.get("os_arch_coverage_targets"), "register.os_arch_coverage_targets"),
            "supported_modes": ensure_list(row_toolchain.get("supported_modes"), "register.supported_modes"),
            "not_claimed_modes": ensure_list(row_toolchain.get("not_claimed_modes"), "register.not_claimed_modes"),
            "version_source_ref": row.get("workspace_descriptor_ref"),
        }
    if descriptor is not None and descriptor.get("declared_toolchain"):
        return {
            "manifest_id": None,
            "exact_versions": ensure_dict(descriptor.get("declared_toolchain"), "descriptor.declared_toolchain"),
            "os_arch_coverage_targets": [],
            "supported_modes": [],
            "not_claimed_modes": [],
            "version_source_ref": "descriptor.declared_toolchain",
        }
    return {
        "manifest_id": None,
        "exact_versions": {},
        "os_arch_coverage_targets": [],
        "supported_modes": [],
        "not_claimed_modes": [],
        "version_source_ref": None,
    }


def workflow_results(harness: dict[str, Any] | None) -> list[dict[str, Any]]:
    if harness is None:
        return []
    rows: list[dict[str, Any]] = []
    for entry in ensure_list(harness.get("harness_entries"), "harness.harness_entries"):
        entry = ensure_dict(entry, "harness.harness_entries[]")
        rows.append(
            {
                "harness_entry_id": ensure_str(entry.get("harness_entry_id"), "harness_entry_id"),
                "workflow_id": ensure_str(entry.get("workflow_id"), "workflow_id"),
                "workflow_class": ensure_str(entry.get("workflow_class"), "workflow_class"),
                "runner": ensure_dict(entry.get("command_shape"), "command_shape").get("runner"),
                "action": ensure_dict(entry.get("command_shape"), "command_shape").get("action"),
                "expected_outcome": ensure_str(entry.get("expected_outcome"), "expected_outcome"),
                "latest_result": ensure_str(entry.get("latest_result"), "latest_result"),
                "fixture_refs": [
                    ensure_str(ref, "fixture_refs[]")
                    for ref in ensure_list(entry.get("fixture_refs"), "fixture_refs")
                ],
                "evidence_outputs": [
                    ensure_str(ref, "evidence_outputs[]")
                    for ref in ensure_list(entry.get("evidence_outputs"), "evidence_outputs")
                ],
            }
        )
    return rows


def count_results(results: list[dict[str, Any]]) -> dict[str, int]:
    counts = {"pass": 0, "fail": 0, "blocked": 0, "not_run": 0}
    for row in results:
        result = row["latest_result"]
        counts[result] = counts.get(result, 0) + 1
    return counts


def derive_support(
    declared: str,
    results: list[dict[str, Any]],
    packet_present: bool,
    harness_present: bool,
) -> tuple[str, str, list[str]]:
    reasons: list[str] = []
    if not packet_present:
        reasons.append(MISSING_WORKSPACE_PACKET)
    if not harness_present:
        reasons.append(MISSING_BETA_HARNESS)
    if not results:
        reasons.append("no_current_workflow_results")
        return "retest_pending", "retest_pending", reasons

    counts = count_results(results)
    if counts.get("fail", 0) > 0:
        reasons.append("failing_workflow_results")
        return "limited", "failing", reasons
    if counts.get("blocked", 0) > 0:
        reasons.append("blocked_workflow_results")
        return "limited", "blocked", reasons
    if counts.get("not_run", 0) > 0:
        reasons.append("not_run_workflow_results")
        return "retest_pending", "retest_pending", reasons
    return declared, "current", reasons


def stricter_support_class(left: str, right: str) -> str:
    """Return the support class that publishes the narrower claim."""

    if SUPPORT_STRICTNESS.get(right, 99) > SUPPORT_STRICTNESS.get(left, 99):
        return right
    return left


def apply_freshness_window(
    effective_support: str,
    evidence_state: str,
    reasons: list[str],
    as_of: str,
    generated_at: str,
    review_window_days: int,
) -> tuple[str, str, str, list[str]]:
    """Downgrade report state when its evidence date ages past the review window."""

    age_days = (parse_date(generated_at[:10]) - parse_date(as_of)).days
    report_state = "current"
    if age_days > review_window_days * 2:
        report_state = "stale"
        evidence_state = "evidence_stale"
        effective_support = stricter_support_class(
            effective_support,
            "evidence_stale",
        )
        reasons.append(
            f"evidence_stale:age_days={age_days}:window={review_window_days}"
        )
    elif age_days > review_window_days:
        report_state = "review_due"
        if evidence_state == "current":
            evidence_state = "retest_pending"
        effective_support = stricter_support_class(
            effective_support,
            "retest_pending",
        )
        reasons.append(
            f"retest_pending:age_days={age_days}:window={review_window_days}"
        )
    return effective_support, evidence_state, report_state, reasons


def row_id_for(register_row_ref: str, archetype_row_ref: str) -> str:
    suffix = register_row_ref.split(":", 1)[1] if ":" in register_row_ref else archetype_row_ref.split(":", 1)[-1]
    return f"{REPORT_ROW_PREFIX}:{suffix}"


def compose_report(
    repo_root: Path,
    register: dict[str, Any],
    reference_rows: dict[str, Any],
    claimed_register: dict[str, Any],
    generated_at: str,
) -> dict[str, Any]:
    as_of = ensure_str(register.get("as_of"), "register.as_of")
    reference_index = index_reference_rows(reference_rows)
    claimed_archetypes = index_claimed_archetypes(claimed_register)
    workspace_index = index_registered_workspaces(register)
    rows: list[dict[str, Any]] = []

    for archetype_ref, claimed in sorted(claimed_archetypes.items()):
        reference_row = reference_index.get(archetype_ref, {})
        workspace_row = workspace_index.get(archetype_ref)
        register_row_ref = (
            ensure_str(workspace_row.get("register_row_id"), "register_row_id")
            if workspace_row is not None
            else f"unmaterialized_reference_workspace:{archetype_ref.split(':', 1)[1]}"
        )
        reference_workspace_id = None
        descriptor_ref = None
        packet_ref = None
        harness_ref = None
        scorecard_ref = None
        workflow_bundle_ref = None
        declared_support = ensure_str(claimed.get("current_support_class"), f"{archetype_ref}.current_support_class")

        if workspace_row is not None:
            reference_workspace_id = ensure_str(workspace_row.get("reference_workspace_id"), "reference_workspace_id")
            descriptor_ref = ensure_str(workspace_row.get("workspace_descriptor_ref"), "workspace_descriptor_ref")
            packet_ref = ensure_str(workspace_row.get("workspace_packet_ref"), "workspace_packet_ref")
            harness_ref = ensure_str(workspace_row.get("harness_ref"), "harness_ref")
            scorecard_ref = ensure_str(workspace_row.get("archetype_scorecard_ref"), "archetype_scorecard_ref")
            workflow_bundle_ref = workspace_row.get("workflow_bundle_ref")
            declared_support = ensure_str(workspace_row.get("declared_support_class"), "declared_support_class")
        else:
            refs = ensure_list(reference_row.get("reference_workspace_refs", []), f"{archetype_ref}.reference_workspace_refs")
            material_refs = [ref for ref in refs if isinstance(ref, str) and ref.startswith("refws.")]
            reference_workspace_id = material_refs[0] if material_refs else None
            for candidate in (
                f"fixtures/workspaces/reference/{archetype_ref.split(':', 1)[1]}.json",
                None,
            ):
                if candidate and (repo_root / candidate).exists():
                    descriptor_ref = candidate

        if descriptor_ref is None and reference_workspace_id:
            for candidate in (repo_root / "fixtures/workspaces/reference").glob("*.json"):
                descriptor = load_json(candidate)
                if descriptor.get("reference_workspace_id") == reference_workspace_id:
                    descriptor_ref = str(candidate.relative_to(repo_root))
                    break
        if scorecard_ref is None:
            candidate_scorecard = (
                "artifacts/compat/m3/archetype_scorecards/"
                f"{archetype_ref.split(':', 1)[1]}.md"
            )
            if (repo_root / candidate_scorecard).exists():
                scorecard_ref = candidate_scorecard

        packet = read_optional_yaml(repo_root, packet_ref)
        harness = read_optional_yaml(repo_root, harness_ref)
        descriptor = descriptor_for(repo_root, descriptor_ref)
        results = workflow_results(harness)
        effective_support, evidence_state, reasons = derive_support(
            declared_support,
            results,
            packet_present=packet is not None,
            harness_present=harness is not None,
        )
        review_window_days = 21
        effective_support, evidence_state, report_state, reasons = apply_freshness_window(
            effective_support,
            evidence_state,
            reasons,
            as_of,
            generated_at,
            review_window_days,
        )
        counts = count_results(results)
        row = {
            "report_row_id": row_id_for(register_row_ref, archetype_ref),
            "register_row_ref": register_row_ref,
            "reference_workspace_id": reference_workspace_id,
            "title": workspace_row.get("title") if workspace_row else ensure_str(claimed.get("public_label"), f"{archetype_ref}.public_label"),
            "archetype_row_ref": archetype_ref,
            "beta_archetype_ref": ensure_str(claimed.get("archetype_surface_id"), f"{archetype_ref}.archetype_surface_id"),
            "public_label": ensure_str(claimed.get("public_label"), f"{archetype_ref}.public_label"),
            "workspace_descriptor_ref": descriptor_ref,
            "workspace_packet_ref": packet_ref,
            "harness_ref": harness_ref,
            "archetype_scorecard_ref": scorecard_ref,
            "workflow_bundle_ref": workflow_bundle_ref,
            "representative_stack": reference_row.get("representative_stack"),
            "core_workflows": reference_row.get("core_workflows", []),
            "toolchain_manifest": toolchain_from(workspace_row or {}, packet, descriptor),
            "platform_matrix": (
                reference_row.get("minimum_matrix_dimensions", {}).get("platform_dimensions", [])
                or claimed.get("minimum_platform_matrix", [])
            ),
            "mode_matrix": (
                reference_row.get("minimum_matrix_dimensions", {}).get("required_modes", [])
                or claimed.get("minimum_mode_matrix", [])
            ),
            "owner": workspace_row.get("owner") if workspace_row else {
                "owner_dri": register.get("owner"),
                "evidence_owner_ref": "lane:benchmark_lab",
                "publication_owner_ref": "lane:release_evidence",
                "backup_owner_ref_or_waiver": "waiver:single-maintainer-backup",
            },
            "privacy_license": workspace_row.get("privacy_license") if workspace_row else {
                "source_class": "synthetic",
                "privacy_class": "public_synthetic",
                "privacy_decision": "admit_public",
                "license_status": "synthetic_no_real_content",
            },
            "support_class": {
                "declared": declared_support,
                "effective": effective_support,
                "target_at_beta_exit": claimed.get("target_support_class_at_beta_exit"),
                "target_at_stable": claimed.get("target_support_class_at_stable"),
                "downgrade_reasons": reasons,
            },
            "freshness": {
                "report_state": report_state,
                "evidence_state": evidence_state,
                "as_of": as_of,
                "review_window_days": review_window_days,
                "expires_on": add_days(as_of, review_window_days),
                "stale_after": add_days(as_of, review_window_days * 2),
                "badge_label": label_for(effective_support),
            },
            "workflow_result_counts": counts,
            "workflow_results": results,
            "consumer_refs": workspace_row.get("consumer_refs") if workspace_row else {},
            "claim_manifest_refs": [
                f"artifacts/release/m3/claim_manifest.json#{claimed.get('archetype_surface_id')}",
                f"artifacts/release/m3/claim_manifest.md#{claimed.get('archetype_surface_id')}",
            ],
        }
        rows.append(row)

    support_counts: dict[str, int] = {}
    workflow_totals = {"pass": 0, "fail": 0, "blocked": 0, "not_run": 0}
    for row in rows:
        support = row["support_class"]["effective"]
        support_counts[support] = support_counts.get(support, 0) + 1
        for key, value in row["workflow_result_counts"].items():
            workflow_totals[key] = workflow_totals.get(key, 0) + value

    return {
        "schema_version": 1,
        "record_kind": "m3_reference_workspace_report",
        "report_id": "reference_workspace_report:m3.beta",
        "report_revision": 1,
        "report_state": "draft",
        "release_channel_scope": "beta",
        "as_of": as_of,
        "generated_at": generated_at,
        "owner": ensure_str(register.get("owner"), "register.owner"),
        "backup_owner": None,
        "backup_waiver": "single-maintainer-backup",
        "source_refs": {
            "reference_workspace_register": DEFAULT_REGISTER_REL,
            "reference_workspace_rows": DEFAULT_REFERENCE_ROWS_REL,
            "claimed_surface_register": DEFAULT_CLAIMED_SURFACE_REGISTER_REL,
            "claim_manifest": DEFAULT_CLAIM_MANIFEST_REL,
        },
        "consuming_surfaces": [
            "artifacts/release/m3/claim_manifest.json",
            "artifacts/release/m3/claim_manifest.md",
            "artifacts/compat/m3/archetype_scorecards/scorecard_index.yaml",
            "docs/help/m3/release_truth_surfaces.md",
            "docs/release/m3/reference_workspace_claim_integration.md",
            "docs/release/release_evidence_packet_template.md",
            "docs/partners/m3/design_partner_beta_pack.md",
            "artifacts/benchmarks/m3/publication_packet/partner_packet.md",
            "docs/support/support_center_concept.md",
        ],
        "vocabularies": {
            "support_class": list(SUPPORT_STRICTNESS),
            "evidence_state": ["current", "retest_pending", "failing", "blocked", "evidence_stale"],
            "workflow_result": ["pass", "fail", "blocked", "not_run"],
        },
        "summary": {
            "workspace_count": len(rows),
            "materialized_workspace_count": sum(1 for row in rows if row["workspace_packet_ref"]),
            "support_class_counts": support_counts,
            "workflow_result_counts": workflow_totals,
            "publication_gate_state": "blocked_widening"
            if any(row["support_class"]["effective"] in {"retest_pending", "evidence_stale", "unsupported"} for row in rows)
            else "passed",
            "claim_widening_rule": "Claims and badges may not render a stronger support class than the matching report row.",
        },
        "rows": rows,
        "notes": (
            "Generated from the beta reference-workspace register and current harness rows. "
            "Rows with missing or not-run workflow evidence downgrade automatically before "
            "the claim manifest, badges, release evidence, Help/About, service health, "
            "partner packets, or support exports may publish them."
        ),
    }


def label_for(support_class: str) -> str:
    labels = {
        "certified": "Certified",
        "supported": "Supported",
        "limited": "Limited",
        "experimental": "Experimental",
        "community": "Community",
        "retest_pending": "Retest pending",
        "evidence_stale": "Evidence stale",
        "unsupported": "Unsupported",
    }
    return labels.get(support_class, support_class)


def compose_badges(report: dict[str, Any], generated_at: str) -> dict[str, Any]:
    rows = []
    for row in report["rows"]:
        effective = row["support_class"]["effective"]
        rows.append(
            {
                "badge_id": f"reference_workspace_badge:{row['archetype_row_ref'].split(':', 1)[1]}",
                "register_row_ref": row["register_row_ref"],
                "reference_workspace_id": row["reference_workspace_id"],
                "archetype_row_ref": row["archetype_row_ref"],
                "beta_archetype_ref": row["beta_archetype_ref"],
                "label": label_for(effective),
                "support_class": effective,
                "freshness_state": row["freshness"]["evidence_state"],
                "report_row_ref": row["report_row_id"],
                "report_ref": f"{DEFAULT_REPORT_JSON_REL}#{row['report_row_id']}",
                "scorecard_ref": row["archetype_scorecard_ref"],
                "claim_manifest_refs": row["claim_manifest_refs"],
                "surface_projection": {
                    "help_about": "status_badge_only",
                    "service_health": "status_badge_only",
                    "support_export": "packet_reference_only",
                    "release_packet": "packet_reference_only",
                    "partner_packet": "packet_reference_only",
                },
                "widening_blocked": effective in {"retest_pending", "evidence_stale", "unsupported"},
            }
        )
    return {
        "schema_version": 1,
        "record_kind": "m3_reference_workspace_badges",
        "source_report_ref": DEFAULT_REPORT_JSON_REL,
        "as_of": report["as_of"],
        "generated_at": generated_at,
        "rows": rows,
        "notes": "Badge projections mirror the reference-workspace report; they are not hand-authored status labels.",
    }


def render_markdown(report: dict[str, Any], *, docs_variant: bool = False) -> str:
    lines: list[str] = []
    title = "Beta Reference-Workspace Report"
    lines.append(f"# {title}")
    lines.append("")
    lines.append(
        "This report is generated from the beta reference-workspace register, "
        "workspace packets, and current harness result rows. It is the source "
        "for archetype support classes, Help/About and service-health badges, "
        "release evidence, partner packets, and support exports."
    )
    lines.append("")
    lines.append("## Metadata")
    lines.append("")
    lines.append(f"- Report id: `{report['report_id']}`")
    lines.append(f"- Release channel scope: `{report['release_channel_scope']}`")
    lines.append(f"- As of: `{report['as_of']}`")
    lines.append(f"- Generated at: `{report['generated_at']}`")
    lines.append(f"- Owner: {report['owner']}")
    lines.append(f"- Publication gate: `{report['summary']['publication_gate_state']}`")
    lines.append("")
    lines.append("## Source Artifacts")
    lines.append("")
    for label, ref in report["source_refs"].items():
        lines.append(f"- `{label}`: `{ref}`")
    lines.append("")
    lines.append("## Summary")
    lines.append("")
    lines.append(
        "| Workspace | Reference row | Reference id | Support | Freshness | Results |"
    )
    lines.append("|---|---|---|---|---|---|")
    for row in report["rows"]:
        counts = row["workflow_result_counts"]
        result_text = ", ".join(f"{key}={counts.get(key, 0)}" for key in ("pass", "fail", "blocked", "not_run"))
        lines.append(
            f"| {row['public_label']} | `{row['register_row_ref']}` | "
            f"`{row['reference_workspace_id']}` | "
            f"{label_for(row['support_class']['effective'])} "
            f"(`{row['support_class']['declared']}` -> `{row['support_class']['effective']}`) | "
            f"`{row['freshness']['evidence_state']}` | {result_text} |"
        )
    lines.append("")
    lines.append("## Workspace Rows")
    lines.append("")
    for row in report["rows"]:
        lines.append(f"### {row['public_label']}")
        lines.append("")
        lines.append(f"- Report row: `{row['report_row_id']}`")
        lines.append(f"- Archetype row: `{row['archetype_row_ref']}`")
        lines.append(f"- Beta archetype: `{row['beta_archetype_ref']}`")
        lines.append(f"- Reference workspace id: `{row['reference_workspace_id']}`")
        if row["workspace_descriptor_ref"]:
            lines.append(f"- Workspace descriptor: `{row['workspace_descriptor_ref']}`")
        if row["workspace_packet_ref"]:
            lines.append(f"- Workspace packet: `{row['workspace_packet_ref']}`")
        if row["harness_ref"]:
            lines.append(f"- Harness: `{row['harness_ref']}`")
        toolchain = row["toolchain_manifest"]["exact_versions"]
        if toolchain:
            pins = ", ".join(f"`{name} {version}`" for name, version in toolchain.items())
            lines.append(f"- Toolchain pins: {pins}")
        if row["platform_matrix"]:
            lines.append(
                "- Platform coverage: "
                + ", ".join(f"`{item}`" for item in row["platform_matrix"])
            )
        if row["mode_matrix"]:
            lines.append(
                "- Mode scope: " + ", ".join(f"`{item}`" for item in row["mode_matrix"])
            )
        support = row["support_class"]
        lines.append(
            f"- Support class: declared `{support['declared']}`, effective `{support['effective']}`"
        )
        if support["downgrade_reasons"]:
            lines.append(
                "- Downgrade reasons: "
                + ", ".join(f"`{reason}`" for reason in support["downgrade_reasons"])
            )
        lines.append(
            f"- Freshness: `{row['freshness']['evidence_state']}`, expires `{row['freshness']['expires_on']}`, stale after `{row['freshness']['stale_after']}`"
        )
        if row["workflow_results"]:
            lines.append("")
            lines.append("| Workflow | Class | Latest result | Runner | Evidence outputs |")
            lines.append("|---|---|---|---|---|")
            for workflow in row["workflow_results"]:
                outputs = ", ".join(f"`{item}`" for item in workflow["evidence_outputs"])
                lines.append(
                    f"| `{workflow['workflow_id']}` | `{workflow['workflow_class']}` | "
                    f"`{workflow['latest_result']}` | `{workflow['runner']}` | {outputs} |"
                )
        else:
            lines.append("- Workflow results: no current beta harness row is materialized.")
        lines.append("")
    lines.append("## Claim Integration")
    lines.append("")
    lines.append(
        "Claim publication checks compare each beta archetype row in "
        "`artifacts/release/m3/claim_manifest.json` against this report. A "
        "claim row fails publication if its effective support class is greener "
        "than the matching reference-workspace row. Badge projections in "
        "`artifacts/compat/m3/reference_workspace_badges.json` carry the same "
        "support and freshness labels for docs, Help/About, service health, "
        "release packets, partner packets, and support exports."
    )
    lines.append("")
    lines.append("## How To Refresh")
    lines.append("")
    lines.append("```sh")
    lines.append("python3 ci/check_m3_reference_workspace_report.py --repo-root .")
    lines.append("```")
    lines.append("")
    lines.append(
        "Use `--check` in CI to fail when the checked-in report, docs copy, "
        "badge projection, or validation capture would drift."
    )
    lines.append("")
    if docs_variant:
        lines.append(
            "The artifact copy lives at `artifacts/compat/m3/reference_workspace_report.md`."
        )
        lines.append("")
    return "\n".join(lines)


def validate_claim_manifest(report: dict[str, Any], manifest: dict[str, Any], findings: list[Finding]) -> None:
    report_by_archetype = {row["archetype_row_ref"]: row for row in report["rows"]}
    for row in ensure_list(manifest.get("rows"), "claim_manifest.rows"):
        row = ensure_dict(row, "claim_manifest.rows[]")
        if row.get("row_kind") != "beta_archetype_binding":
            continue
        archetype_ref = row.get("archetype_row_ref")
        if not isinstance(archetype_ref, str):
            continue
        report_row = report_by_archetype.get(archetype_ref)
        if report_row is None:
            findings.append(
                Finding(
                    "error",
                    "claim_manifest.reference_workspace_report.missing",
                    "claim manifest beta archetype row has no reference-workspace report row",
                    "Add a report row or downgrade/remove the beta archetype claim.",
                    ref=row.get("row_id"),
                )
            )
            continue
        claim_support = ensure_str(
            ensure_dict(row.get("support"), "claim_manifest.row.support").get("effective"),
            "claim_manifest.row.support.effective",
        )
        report_support = ensure_str(
            ensure_dict(report_row.get("support_class"), "report_row.support_class").get("effective"),
            "report_row.support_class.effective",
        )
        if SUPPORT_STRICTNESS.get(claim_support, 99) < SUPPORT_STRICTNESS.get(report_support, 99):
            findings.append(
                Finding(
                    "error",
                    "claim_manifest.support_widens_reference_report",
                    (
                        f"claim support {claim_support!r} is greener than "
                        f"reference-workspace report support {report_support!r}"
                    ),
                    "Regenerate scorecards and the claim manifest from the current reference-workspace report, or narrow the claim row.",
                    ref=row.get("row_id"),
                    details={
                        "archetype_row_ref": archetype_ref,
                        "report_row_ref": report_row["report_row_id"],
                    },
                )
            )


_GENERATED_AT_RE = re.compile(
    r'"generated_at":\s*"[^"]*"|\*\*Generated at:\*\*\s*`[^`]*`|Generated at: `[^`]*`'
)


def normalize(text: str) -> str:
    return _GENERATED_AT_RE.sub("__generated_at__", text)


def write_or_check(path: Path, content: str, check: bool) -> bool:
    existing = path.read_text(encoding="utf-8") if path.exists() else None
    changed = existing is None or normalize(existing) != normalize(content)
    if not check:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content, encoding="utf-8")
    return changed


def write_capture(path: Path, findings: list[Finding], generated_at: str, check: bool) -> bool:
    payload = {
        "schema_version": 1,
        "record_kind": "m3_reference_workspace_report_validation_capture",
        "status": "pass" if not any(f.severity == "error" for f in findings) else "fail",
        "generated_at": generated_at,
        "report_json_ref": DEFAULT_REPORT_JSON_REL,
        "report_md_ref": DEFAULT_REPORT_MD_REL,
        "doc_md_ref": DEFAULT_DOC_MD_REL,
        "badges_ref": DEFAULT_BADGES_REL,
        "summary": {
            "errors": sum(1 for f in findings if f.severity == "error"),
            "warnings": sum(1 for f in findings if f.severity == "warning"),
        },
        "findings": [f.as_report() for f in findings],
    }
    return write_or_check(path, json.dumps(payload, indent=2, sort_keys=True) + "\n", check)


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    generated_at = today_utc()

    register = ensure_dict(render_yaml_as_json(repo_root / args.register), "register")
    reference_rows = ensure_dict(render_yaml_as_json(repo_root / args.reference_rows), "reference_rows")
    claimed_register = ensure_dict(load_json(repo_root / args.claimed_surface_register), "claimed_surface_register")

    report = compose_report(repo_root, register, reference_rows, claimed_register, generated_at)
    badges = compose_badges(report, generated_at)

    findings: list[Finding] = []
    manifest_path = repo_root / args.claim_manifest
    if manifest_path.exists():
        validate_claim_manifest(report, ensure_dict(load_json(manifest_path), "claim_manifest"), findings)

    report_json = json.dumps(report, indent=2, sort_keys=True) + "\n"
    badges_json = json.dumps(badges, indent=2, sort_keys=True) + "\n"
    report_md = render_markdown(report)
    doc_md = render_markdown(report, docs_variant=True)

    changed = {
        args.report_json: write_or_check(repo_root / args.report_json, report_json, args.check),
        args.report_md: write_or_check(repo_root / args.report_md, report_md, args.check),
        args.doc_md: write_or_check(repo_root / args.doc_md, doc_md, args.check),
        args.badges: write_or_check(repo_root / args.badges, badges_json, args.check),
    }
    if args.check and any(changed.values()):
        findings.append(
            Finding(
                "error",
                "reference_workspace_report.stale",
                "checked-in reference-workspace report artifacts are stale",
                "Run `python3 ci/check_m3_reference_workspace_report.py --repo-root .` and commit the regenerated artifacts.",
                details={key: value for key, value in changed.items()},
            )
        )

    capture_changed = write_capture(repo_root / args.capture, findings, generated_at, args.check)
    if args.check and capture_changed:
        findings.append(
            Finding(
                "error",
                "reference_workspace_report.capture_stale",
                "checked-in validation capture is stale",
                "Run the reference-workspace report generator and commit the capture.",
            )
        )
        write_capture(repo_root / args.capture, findings, generated_at, args.check)

    errors = [item for item in findings if item.severity == "error"]
    if errors:
        for item in errors:
            ref = f" ({item.ref})" if item.ref else ""
            print(f"ERROR [{item.check_id}]{ref}: {item.message}", file=sys.stderr)
            print(f"  remediation: {item.remediation}", file=sys.stderr)
        return 1
    print("M3 reference-workspace report generated and validated")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
