#!/usr/bin/env python3
"""Validate and project the stable claim-publication manifest."""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path


SUPPORT_RANK = {
    "certified": 4,
    "supported": 3,
    "limited": 2,
    "unsupported": 0,
}

CLAIM_RANK = {
    "certified": 4,
    "supported": 3,
    "limited": 2,
    "retest_pending": 1,
    "unsupported": 0,
}

REQUIRED_SURFACES = {
    "release_notes",
    "website_docs",
    "enterprise_evaluation_packet",
    "product_badges",
    "help_about",
    "service_health",
    "cli_inspection",
    "support_export",
    "public_proof_packet",
}


def required_reasons(entry: dict) -> set[str]:
    """Return narrowing reasons implied by linked report refs."""
    reasons: set[str] = set()
    for report in entry.get("linked_report_refs", []):
        family = report.get("report_family")
        state = report.get("evidence_state")
        if family == "reference_workspace_report" and state == "stale":
            reasons.add("reference_workspace_stale")
        elif family == "reference_workspace_report" and state == "missing":
            reasons.add("reference_workspace_missing")
        elif family == "compatibility_report" and state == "dropped":
            reasons.add("compatibility_dropped")
        elif state == "stale":
            reasons.add("evidence_stale")
        elif state == "missing":
            reasons.add("evidence_missing")
        elif state == "dropped":
            reasons.add("compatibility_dropped")
        if not report.get("owner_signed", False):
            reasons.add("owner_signoff_missing")
    return reasons


def entry_has_current_certified_report(entry: dict) -> bool:
    """Return true when a Certified entry has current signed reference proof."""
    return any(
        report.get("report_family") == "reference_workspace_report"
        and report.get("support_class") == "certified"
        and report.get("evidence_state") == "current"
        and report.get("owner_signed") is True
        for report in entry.get("linked_report_refs", [])
    )


def validate_manifest(manifest: dict) -> list[str]:
    """Validate the manifest and return all findings."""
    findings: list[str] = []
    if manifest.get("schema_version") != 1:
        findings.append("unsupported schema_version")
    if manifest.get("record_kind") != "claim_publication_manifest":
        findings.append("unsupported record_kind")

    manifest_id = manifest.get("manifest_id")
    entry_ranks: dict[str, int] = {}
    surface_rows = 0
    downgraded = 0
    unsupported = 0
    certified_current = 0
    overclaims = 0

    seen_entries: set[str] = set()
    for entry in manifest.get("entries", []):
        entry_id = entry.get("entry_id", "")
        if not entry_id:
            findings.append("entry missing entry_id")
            continue
        if entry_id in seen_entries:
            findings.append(f"{entry_id}: duplicate entry_id")
        seen_entries.add(entry_id)

        declared_rank = SUPPORT_RANK.get(entry.get("declared_support_class"), -1)
        effective_rank = CLAIM_RANK.get(entry.get("effective_claim"), -1)
        entry_ranks[entry_id] = effective_rank
        if effective_rank > declared_rank:
            findings.append(f"{entry_id}: effective claim is wider than declared support class")
        if effective_rank < declared_rank:
            downgraded += 1
        if entry.get("effective_claim") == "unsupported":
            unsupported += 1
        if entry.get("declared_support_class") == "certified":
            if entry_has_current_certified_report(entry):
                certified_current += 1
            else:
                findings.append(f"{entry_id}: Certified entry lacks a current signed reference report")

        reasons = required_reasons(entry)
        active = set(entry.get("active_narrowing_reasons", []))
        missing = reasons - active
        if missing:
            findings.append(f"{entry_id}: missing narrowing reasons {sorted(missing)}")
        if reasons and effective_rank >= declared_rank:
            findings.append(f"{entry_id}: narrowing evidence did not downgrade the claim")
        if not reasons and active:
            findings.append(f"{entry_id}: active narrowing reasons without narrowing evidence")

        report_refs = {report.get("report_ref") for report in entry.get("linked_report_refs", [])}
        surfaces = [row.get("surface_id") for row in entry.get("surface_projections", [])]
        if set(surfaces) != REQUIRED_SURFACES:
            findings.append(f"{entry_id}: surface coverage mismatch")
        if len(surfaces) != len(set(surfaces)):
            findings.append(f"{entry_id}: duplicate surface projection")

        for projection in entry.get("surface_projections", []):
            surface_rows += 1
            if projection.get("source_manifest_ref") != manifest_id:
                findings.append(f"{entry_id}: projection uses wrong manifest ref")
            if CLAIM_RANK.get(projection.get("rendered_claim"), -1) > effective_rank:
                overclaims += 1
                findings.append(
                    f"{entry_id}: {projection.get('destination_ref')} renders wider than effective claim"
                )
            for report_ref in projection.get("linked_report_refs", []):
                if report_ref not in report_refs:
                    findings.append(f"{entry_id}: projection cites unknown report ref {report_ref}")

    for filter_row in manifest.get("evaluation_filters", []):
        if filter_row.get("public_ceiling_manifest_ref") != manifest_id:
            findings.append(f"{filter_row.get('filter_id')}: wrong public ceiling manifest")
        max_rank = CLAIM_RANK.get(filter_row.get("max_effective_claim"), -1)
        widest_included_public_claim = None
        for entry_ref in filter_row.get("included_entry_refs", []):
            if entry_ref not in entry_ranks:
                findings.append(f"{filter_row.get('filter_id')}: unknown entry {entry_ref}")
            else:
                widest_included_public_claim = max(
                    widest_included_public_claim or 0, entry_ranks[entry_ref]
                )
        if (
            widest_included_public_claim is not None
            and max_rank > widest_included_public_claim
        ):
            findings.append(f"{filter_row.get('filter_id')}: widens public filter ceiling")

    summary = manifest.get("summary", {})
    expected_summary = {
        "total_entries": len(manifest.get("entries", [])),
        "certified_entries_current": certified_current,
        "entries_downgraded": downgraded,
        "unsupported_entries": unsupported,
        "surface_projection_rows": surface_rows,
        "overclaiming_surface_rows": overclaims,
    }
    if summary != expected_summary:
        findings.append(f"summary mismatch: expected {expected_summary}, got {summary}")

    expected_decision = (
        "hold"
        if overclaims
        else "proceed_with_downgrades"
        if downgraded
        else "proceed"
    )
    if manifest.get("publication", {}).get("decision") != expected_decision:
        findings.append(f"publication decision should be {expected_decision}")

    return findings


def projection(manifest: dict) -> dict:
    """Return the surface-safe projection emitted by this tool."""
    return {
        "manifest_id": manifest["manifest_id"],
        "as_of": manifest["as_of"],
        "publication_decision": manifest["publication"]["decision"],
        "entries": [
            {
                "entry_id": entry["entry_id"],
                "declared_support_class": entry["declared_support_class"],
                "effective_claim": entry["effective_claim"],
                "active_narrowing_reasons": entry["active_narrowing_reasons"],
                "surface_ids": [
                    row["surface_id"] for row in entry.get("surface_projections", [])
                ],
                "report_refs": [
                    report["report_ref"] for report in entry.get("linked_report_refs", [])
                ],
            }
            for entry in manifest.get("entries", [])
        ],
    }


def main() -> int:
    """CLI entry point."""
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "manifest",
        nargs="?",
        default="artifacts/release/stable/claim-publication-manifest/manifest.json",
        help="Path to the claim-publication manifest.",
    )
    parser.add_argument(
        "--projection",
        action="store_true",
        help="Print the surface projection when validation succeeds.",
    )
    args = parser.parse_args()

    manifest_path = Path(args.manifest)
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    findings = validate_manifest(manifest)
    if findings:
        for finding in findings:
            print(f"claim-publication validation failed: {finding}", file=sys.stderr)
        return 1

    output = {"status": "pass", "manifest": str(manifest_path)}
    if args.projection:
        output["projection"] = projection(manifest)
    print(json.dumps(output, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
