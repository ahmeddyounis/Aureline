#!/usr/bin/env python3
"""M3 docs / public-truth freshness gate.

This gate enforces that every claimed M3 beta row in the governed
claim manifest stays inspectable through fresh, vocabulary-consistent
docs, Help/About, release, and release-notes surfaces. It reads:

- the M3 source map at ``artifacts/ci/m3_docs_truth_source_map.yaml``;
- the governed claim manifest at
  ``artifacts/release/m3/claim_manifest.json``;
- the M3 compatibility report at
  ``artifacts/compat/m3/compatibility_report.json``;
- the release-notes draft at
  ``artifacts/release/m3/release_notes_draft.md``; and
- the consuming docs / Help / release surfaces the source map pins.

For each enforced row the gate verifies that:

- the manifest as_of falls inside the source map's review-window
  floor relative to today;
- each consuming surface declared in the source map exists on disk
  and quotes every required token verbatim;
- each manifest vocabulary the source map names still appears in the
  claim manifest's ``vocabularies`` block (so retired vocabularies
  surface before a docs publication);
- the release-notes draft back-links the manifest by ``manifest_id``
  and the compatibility report by ``report_id`` and ``as_of``; and
- every enforced manifest row and compatibility row is cited verbatim
  by ``row_id`` in the release-notes draft before beta publication.

The gate also regenerates ``artifacts/docs/m3/docs_truth_report.md``
on every run; ``--check`` fails CI when the on-disk report would
drift from the upstream truth.
"""

from __future__ import annotations

import datetime as dt
import json
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any

if __package__ in (None, ""):
    HERE = Path(__file__).resolve().parent
    sys.path.insert(0, str(HERE.parent.parent))
    from ci.m3._common import (  # type: ignore
        Finding,
        artifact_ref_exists,
        base_argument_parser,
        ensure_dict,
        ensure_list,
        ensure_str,
        index_compat_rows,
        index_manifest_rows,
        load_claim_manifest,
        load_compatibility_report,
        load_source_map,
        normalize_generated_text,
        now_iso_z,
        parse_iso_date,
        resolve_today,
        write_if_changed,
    )
else:
    from ._common import (
        Finding,
        artifact_ref_exists,
        base_argument_parser,
        ensure_dict,
        ensure_list,
        ensure_str,
        index_compat_rows,
        index_manifest_rows,
        load_claim_manifest,
        load_compatibility_report,
        load_source_map,
        normalize_generated_text,
        now_iso_z,
        parse_iso_date,
        resolve_today,
        write_if_changed,
    )


DEFAULT_REPORT_REL = "artifacts/docs/m3/docs_truth_report.md"
DEFAULT_CAPTURE_REL = (
    "artifacts/docs/m3/captures/m3_docs_freshness_validation_capture.json"
)


@dataclass
class RowResult:
    row_id: str
    claim_family: str
    freshness_badge: str
    evidence_date: str
    review_window_days: int
    days_since_evidence: int
    passed_checks: list[str] = field(default_factory=list)
    failed_checks: list[dict[str, Any]] = field(default_factory=list)
    cited_in_release_notes: bool = False

    def as_report(self) -> dict[str, Any]:
        return asdict(self)


@dataclass
class SurfaceResult:
    surface_ref: str
    purpose: str
    exists: bool
    missing_tokens: list[str] = field(default_factory=list)
    passed_checks: list[str] = field(default_factory=list)
    failed_checks: list[dict[str, Any]] = field(default_factory=list)

    def as_report(self) -> dict[str, Any]:
        return asdict(self)


def fail_row(
    row: RowResult,
    findings: list[Finding],
    check_id: str,
    message: str,
    remediation: str,
    *,
    details: dict[str, Any] | None = None,
) -> None:
    entry: dict[str, Any] = {"check_id": check_id, "message": message}
    if details:
        entry["details"] = details
    row.failed_checks.append(entry)
    findings.append(
        Finding(
            severity="error",
            check_id=check_id,
            message=f"{row.row_id}: {message}",
            remediation=remediation,
            ref=row.row_id,
            details=details or {},
        )
    )


def fail_surface(
    surface: SurfaceResult,
    findings: list[Finding],
    check_id: str,
    message: str,
    remediation: str,
    *,
    details: dict[str, Any] | None = None,
) -> None:
    entry: dict[str, Any] = {"check_id": check_id, "message": message}
    if details:
        entry["details"] = details
    surface.failed_checks.append(entry)
    findings.append(
        Finding(
            severity="error",
            check_id=check_id,
            message=f"{surface.surface_ref}: {message}",
            remediation=remediation,
            ref=surface.surface_ref,
            details=details or {},
        )
    )


def check_consuming_surfaces(
    repo_root: Path,
    surfaces: list[dict[str, Any]],
    findings: list[Finding],
) -> list[SurfaceResult]:
    results: list[SurfaceResult] = []
    for raw in surfaces:
        entry = ensure_dict(raw, "consuming_surfaces[]")
        surface_ref = ensure_str(
            entry.get("surface_ref"), "consuming_surfaces[].surface_ref"
        )
        purpose = ensure_str(
            entry.get("purpose"), f"{surface_ref}.purpose"
        )
        must_contain = [
            ensure_str(t, f"{surface_ref}.must_contain_tokens[]")
            for t in ensure_list(
                entry.get("must_contain_tokens", []),
                f"{surface_ref}.must_contain_tokens",
            )
        ]
        surface_path = repo_root / surface_ref
        result = SurfaceResult(
            surface_ref=surface_ref,
            purpose=purpose,
            exists=surface_path.exists(),
        )
        if not result.exists:
            fail_surface(
                result,
                findings,
                "consuming_surface.missing",
                f"consuming surface does not exist on disk: {surface_ref}",
                "Restore the surface or fix the source map path.",
            )
            results.append(result)
            continue
        text = surface_path.read_text(encoding="utf-8")
        missing = [t for t in must_contain if t not in text]
        result.missing_tokens = missing
        if missing:
            fail_surface(
                result,
                findings,
                "consuming_surface.required_tokens_missing",
                (
                    f"consuming surface is missing required tokens: "
                    f"{sorted(missing)}"
                ),
                (
                    "Restore the required tokens or update the source "
                    "map's must_contain_tokens list in the same change "
                    "set."
                ),
                details={"missing_tokens": sorted(missing)},
            )
        else:
            result.passed_checks.append("consuming_surface.tokens_present")
        results.append(result)
    return results


def check_manifest_vocabularies(
    manifest: dict[str, Any],
    required: list[str],
    findings: list[Finding],
) -> list[str]:
    vocab = ensure_dict(
        manifest.get("vocabularies"), "claim_manifest.vocabularies"
    )
    missing: list[str] = []
    for name in required:
        if name not in vocab:
            missing.append(name)
            findings.append(
                Finding(
                    severity="error",
                    check_id="manifest_vocabulary.missing",
                    message=(
                        f"claim manifest does not publish required "
                        f"vocabulary {name!r}"
                    ),
                    remediation=(
                        "Restore the vocabulary in the claim manifest or "
                        "drop the entry from required_manifest_vocabularies "
                        "in the same change set."
                    ),
                    ref=name,
                )
            )
    return missing


def check_release_notes(
    repo_root: Path,
    release_notes_ref: str,
    manifest: dict[str, Any],
    compat_report: dict[str, Any],
    enforced_rows: list[dict[str, Any]],
    enforced_compat_rows: list[dict[str, Any]],
    requirements: dict[str, Any],
    findings: list[Finding],
    forced_strip_manifest_id: bool = False,
) -> dict[str, Any]:
    result: dict[str, Any] = {
        "release_notes_ref": release_notes_ref,
        "exists": False,
        "passed_checks": [],
        "failed_checks": [],
        "row_citations": {},
        "compat_row_citations": {},
    }
    path = repo_root / release_notes_ref
    if not path.exists():
        findings.append(
            Finding(
                severity="error",
                check_id="release_notes.draft_missing",
                message=(
                    f"release-notes draft does not exist on disk: "
                    f"{release_notes_ref}"
                ),
                remediation=(
                    "Create the release-notes draft so it back-links the "
                    "current claim-manifest and compatibility rows before "
                    "beta publication."
                ),
                ref=release_notes_ref,
            )
        )
        result["failed_checks"].append(
            {"check_id": "release_notes.draft_missing"}
        )
        return result
    text = path.read_text(encoding="utf-8")
    if forced_strip_manifest_id:
        text = text.replace(
            ensure_str(manifest.get("manifest_id"), "manifest_id"),
            "__force_drill_manifest_id_stripped__",
        )
    result["exists"] = True

    manifest_id = ensure_str(manifest.get("manifest_id"), "manifest_id")
    manifest_as_of = ensure_str(manifest.get("as_of"), "manifest.as_of")
    compat_report_id = ensure_str(
        compat_report.get("report_id"), "compatibility_report.report_id"
    )

    if requirements.get("must_contain_manifest_id", True):
        if manifest_id in text:
            result["passed_checks"].append("manifest_id_present")
        else:
            findings.append(
                Finding(
                    severity="error",
                    check_id="release_notes.manifest_backlink_missing",
                    message=(
                        f"release-notes draft does not back-link the claim "
                        f"manifest by manifest_id {manifest_id!r}"
                    ),
                    remediation=(
                        "Cite the current manifest_id in the release-notes "
                        "draft so reviewers can resolve the row truth."
                    ),
                    ref=release_notes_ref,
                )
            )
            result["failed_checks"].append(
                {"check_id": "release_notes.manifest_backlink_missing"}
            )

    if requirements.get("must_contain_compat_report_id", True):
        if compat_report_id in text:
            result["passed_checks"].append("compat_report_id_present")
        else:
            findings.append(
                Finding(
                    severity="error",
                    check_id="release_notes.compat_report_backlink_missing",
                    message=(
                        f"release-notes draft does not back-link the M3 "
                        f"compatibility report by report_id "
                        f"{compat_report_id!r}"
                    ),
                    remediation=(
                        "Cite the current compatibility-report id in the "
                        "release-notes draft so reviewers can resolve the "
                        "row truth."
                    ),
                    ref=release_notes_ref,
                )
            )
            result["failed_checks"].append(
                {"check_id": "release_notes.compat_report_backlink_missing"}
            )

    if requirements.get("must_contain_manifest_as_of", True):
        if manifest_as_of in text:
            result["passed_checks"].append("manifest_as_of_present")
        else:
            findings.append(
                Finding(
                    severity="error",
                    check_id="release_notes.manifest_as_of_missing",
                    message=(
                        f"release-notes draft does not name the manifest "
                        f"as_of date {manifest_as_of!r}"
                    ),
                    remediation=(
                        "Cite the current manifest as_of in the release-"
                        "notes draft so the publication is dated."
                    ),
                    ref=release_notes_ref,
                )
            )
            result["failed_checks"].append(
                {"check_id": "release_notes.manifest_as_of_missing"}
            )

    for entry in enforced_rows:
        row_id = ensure_str(entry.get("manifest_row_id"), "enforced_rows[]")
        must_cite = bool(entry.get("must_appear_in_release_notes", True))
        cited = row_id in text
        result["row_citations"][row_id] = cited
        if must_cite and not cited:
            findings.append(
                Finding(
                    severity="error",
                    check_id="release_notes.row_citation_missing",
                    message=(
                        f"release-notes draft does not cite manifest row "
                        f"{row_id!r}"
                    ),
                    remediation=(
                        "Add a release-note line that quotes the row_id "
                        "and a short row headline."
                    ),
                    ref=row_id,
                )
            )
            result["failed_checks"].append(
                {
                    "check_id": "release_notes.row_citation_missing",
                    "row_id": row_id,
                }
            )

    for entry in enforced_compat_rows:
        compat_id = ensure_str(
            entry.get("compat_row_id"), "enforced_compat_rows[]"
        )
        cited = compat_id in text
        result["compat_row_citations"][compat_id] = cited
        if not cited:
            findings.append(
                Finding(
                    severity="error",
                    check_id="release_notes.compat_row_citation_missing",
                    message=(
                        f"release-notes draft does not cite compat row "
                        f"{compat_id!r}"
                    ),
                    remediation=(
                        "Add a release-note line that quotes the compat "
                        "row_id and a short skew-window summary."
                    ),
                    ref=compat_id,
                )
            )
            result["failed_checks"].append(
                {
                    "check_id": "release_notes.compat_row_citation_missing",
                    "compat_row_id": compat_id,
                }
            )

    return result


def check_enforced_rows(
    manifest: dict[str, Any],
    enforced: list[dict[str, Any]],
    review_window_days_floor: int,
    today: dt.date,
    release_notes: dict[str, Any],
    findings: list[Finding],
) -> list[RowResult]:
    row_index = index_manifest_rows(manifest)
    valid_freshness = set(
        ensure_dict(
            manifest.get("vocabularies"), "claim_manifest.vocabularies"
        ).get("freshness_badge_class", [])
    )
    results: list[RowResult] = []
    for entry in enforced:
        row_id = ensure_str(entry.get("manifest_row_id"), "enforced_rows[]")
        manifest_row = row_index.get(row_id)
        if manifest_row is None:
            findings.append(
                Finding(
                    severity="error",
                    check_id="enforced_row.unknown_manifest_row",
                    message=(
                        f"enforced row {row_id!r} is not present in the "
                        f"claim manifest"
                    ),
                    remediation=(
                        "Add the row to the claim manifest or drop it from "
                        "the source map in the same change set."
                    ),
                    ref=row_id,
                )
            )
            continue
        freshness_block = ensure_dict(
            manifest_row.get("freshness"), f"{row_id}.freshness"
        )
        evidence_date_str = ensure_str(
            freshness_block.get("evidence_date"),
            f"{row_id}.freshness.evidence_date",
        )
        review_window_days = int(
            freshness_block.get("review_window_days", 21)
        )
        badge = ensure_str(
            freshness_block.get("badge_class"),
            f"{row_id}.freshness.badge_class",
        )
        claim_family = ensure_str(
            manifest_row.get("claim_family"), f"{row_id}.claim_family"
        )
        evidence_date = parse_iso_date(
            evidence_date_str, f"{row_id}.freshness.evidence_date"
        )
        days_since = (today - evidence_date).days
        result = RowResult(
            row_id=row_id,
            claim_family=claim_family,
            freshness_badge=badge,
            evidence_date=evidence_date_str,
            review_window_days=review_window_days,
            days_since_evidence=days_since,
            cited_in_release_notes=bool(
                release_notes.get("row_citations", {}).get(row_id)
            ),
        )
        # Floor-based freshness check.
        if days_since > review_window_days_floor:
            fail_row(
                result,
                findings,
                "enforced_row.evidence_outside_review_window_floor",
                (
                    f"evidence_date {evidence_date_str} is "
                    f"{days_since} days old, beyond the "
                    f"{review_window_days_floor}-day floor"
                ),
                (
                    "Refresh the upstream evidence and regenerate the "
                    "claim manifest in the same change set, or land a "
                    "decision row that loosens the floor."
                ),
                details={
                    "days_since_evidence": days_since,
                    "review_window_days_floor": review_window_days_floor,
                },
            )
        elif days_since > review_window_days:
            fail_row(
                result,
                findings,
                "enforced_row.evidence_outside_per_row_review_window",
                (
                    f"evidence_date {evidence_date_str} is "
                    f"{days_since} days old, beyond the per-row "
                    f"{review_window_days}-day window"
                ),
                (
                    "Refresh the upstream evidence or attach a waiver "
                    "before publishing the row."
                ),
                details={
                    "days_since_evidence": days_since,
                    "review_window_days": review_window_days,
                },
            )
        else:
            result.passed_checks.append("evidence_inside_review_window")
        # Badge floor.
        if badge in {"stale", "unverified"}:
            fail_row(
                result,
                findings,
                "enforced_row.freshness_badge_below_floor",
                (
                    f"freshness badge {badge!r} is below the warm_cached "
                    f"floor required for beta publication"
                ),
                (
                    "Refresh the evidence or narrow the row's effective "
                    "claim posture in the canonical seed."
                ),
                details={"badge_class": badge},
            )
        elif badge not in valid_freshness:
            fail_row(
                result,
                findings,
                "enforced_row.freshness_badge_unknown_token",
                (
                    f"freshness badge {badge!r} is not in the manifest "
                    f"freshness_badge_class vocabulary"
                ),
                (
                    "Restore the row's badge to a value still listed in "
                    "manifest.vocabularies.freshness_badge_class."
                ),
            )
        else:
            result.passed_checks.append("freshness_badge_above_floor")
        results.append(result)
    return results


def render_truth_report(
    *,
    manifest: dict[str, Any],
    compat_report: dict[str, Any],
    source_map_rel: str,
    enforced_rows: list[RowResult],
    surfaces: list[SurfaceResult],
    release_notes: dict[str, Any],
    findings: list[Finding],
    generated_at: str,
    today: dt.date,
    review_window_days_floor: int,
) -> str:
    lines: list[str] = []
    lines.append("# M3 docs / public-truth report")
    lines.append("")
    lines.append(
        "This file is generated by "
        "`tools/ci/m3/docs_freshness_gate.py` from the M3 docs-truth "
        "source map at `" + source_map_rel + "`. Do not hand-edit; "
        "refresh the upstream seeds and re-run the gate."
    )
    lines.append("")
    lines.append("## Run metadata")
    lines.append("")
    lines.append(f"- **Generated at:** `{generated_at}`")
    lines.append(f"- **Today:** `{today.isoformat()}`")
    lines.append(
        f"- **Manifest id:** `{manifest.get('manifest_id', '?')}`"
    )
    lines.append(
        f"- **Manifest as_of:** `{manifest.get('as_of', '?')}`"
    )
    lines.append(
        f"- **Manifest state:** `{manifest.get('manifest_state', '?')}`"
    )
    lines.append(
        f"- **Compatibility report id:** "
        f"`{compat_report.get('report_id', '?')}`"
    )
    lines.append(
        f"- **Compatibility report as_of:** "
        f"`{compat_report.get('as_of', '?')}`"
    )
    lines.append(
        f"- **Review window floor (days):** `{review_window_days_floor}`"
    )
    lines.append(
        f"- **Release-notes draft present:** "
        f"`{'yes' if release_notes.get('exists') else 'no'}`"
    )
    lines.append("")
    status = "PASS" if not [f for f in findings if f.severity == "error"] else "FAIL"
    lines.append(f"- **Gate status:** `{status}`")
    lines.append(
        f"- **Error findings:** "
        f"`{sum(1 for f in findings if f.severity == 'error')}`"
    )
    lines.append(
        f"- **Warning findings:** "
        f"`{sum(1 for f in findings if f.severity == 'warning')}`"
    )
    lines.append("")

    lines.append("## Enforced manifest rows")
    lines.append("")
    if not enforced_rows:
        lines.append("_No rows configured in the source map._")
    else:
        lines.append(
            "| Row | Family | Badge | Evidence date | Age (days) | "
            "Window | Release-notes cited |"
        )
        lines.append("|---|---|---|---|---:|---:|---|")
        for row in enforced_rows:
            lines.append(
                f"| `{row.row_id}` | `{row.claim_family}` | "
                f"`{row.freshness_badge}` | `{row.evidence_date}` | "
                f"{row.days_since_evidence} | {row.review_window_days} | "
                f"{'yes' if row.cited_in_release_notes else 'no'} |"
            )
    lines.append("")
    for row in enforced_rows:
        if not row.failed_checks:
            continue
        lines.append(f"### `{row.row_id}` — failed checks")
        lines.append("")
        for failed in row.failed_checks:
            check_id = failed.get("check_id", "?")
            message = failed.get("message", "")
            lines.append(f"- `{check_id}`: {message}")
        lines.append("")

    lines.append("## Consuming surfaces")
    lines.append("")
    if not surfaces:
        lines.append("_No consuming surfaces configured in the source map._")
    else:
        lines.append("| Surface | Purpose | Exists | Missing tokens |")
        lines.append("|---|---|---|---|")
        for surface in surfaces:
            missing = ", ".join(f"`{t}`" for t in surface.missing_tokens)
            if not missing:
                missing = "_(none)_"
            lines.append(
                f"| `{surface.surface_ref}` | {surface.purpose} | "
                f"{'yes' if surface.exists else 'no'} | {missing} |"
            )
    lines.append("")

    lines.append("## Release-notes draft")
    lines.append("")
    if not release_notes.get("exists"):
        lines.append(
            "_Release-notes draft is missing; beta publication is "
            "blocked until the draft exists with valid back-links._"
        )
    else:
        lines.append(
            "| Check | Status |\n|---|---|"
        )
        for passed in release_notes.get("passed_checks", []):
            lines.append(f"| `{passed}` | PASS |")
        for failed in release_notes.get("failed_checks", []):
            lines.append(
                f"| `{failed.get('check_id', '?')}` | FAIL |"
            )
        lines.append("")
        lines.append("### Manifest row citations")
        lines.append("")
        for row_id, cited in sorted(
            release_notes.get("row_citations", {}).items()
        ):
            lines.append(
                f"- `{row_id}`: " + ("cited" if cited else "**missing**")
            )
        if release_notes.get("compat_row_citations"):
            lines.append("")
            lines.append("### Compatibility row citations")
            lines.append("")
            for compat_id, cited in sorted(
                release_notes.get("compat_row_citations", {}).items()
            ):
                lines.append(
                    f"- `{compat_id}`: "
                    + ("cited" if cited else "**missing**")
                )
    lines.append("")

    lines.append("## Findings")
    lines.append("")
    if not findings:
        lines.append("_All checks pass; the gate is green._")
    else:
        for finding in findings:
            ref = f" ({finding.ref})" if finding.ref else ""
            lines.append(
                f"- `{finding.check_id}`{ref}: {finding.message}"
            )
            lines.append(f"  - **Remediation:** {finding.remediation}")
    lines.append("")

    lines.append("## How to refresh")
    lines.append("")
    lines.append("Run the gate locally to refresh this report:")
    lines.append("")
    lines.append("```")
    lines.append(
        "python3 tools/ci/m3/docs_freshness_gate.py --repo-root ."
    )
    lines.append("```")
    lines.append("")
    lines.append(
        "Use `--check` in CI to fail when the on-disk report or "
        "capture would drift from the upstream truth."
    )
    lines.append("")
    return "\n".join(lines)


def write_capture(
    path: Path,
    *,
    findings: list[Finding],
    source_map_rel: str,
    manifest_ref: str,
    compat_ref: str,
    release_notes_ref: str,
    report_rel: str,
    enforced_rows: list[RowResult],
    surfaces: list[SurfaceResult],
    release_notes: dict[str, Any],
    generated_at: str,
    today: dt.date,
    forced_drill: dict[str, Any] | None,
    forced_drill_observed: dict[str, Any] | None,
    build_identity_ref: str,
) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    capture: dict[str, Any] = {
        "schema_version": 1,
        "capture_kind": "m3_docs_freshness_validation_capture",
        "captured_at": generated_at,
        "today": today.isoformat(),
        "source_map_ref": source_map_rel,
        "claim_manifest_ref": manifest_ref,
        "compatibility_report_ref": compat_ref,
        "release_notes_draft_ref": release_notes_ref,
        "docs_truth_report_ref": report_rel,
        "exact_build_identity_ref": build_identity_ref,
        "command": (
            "python3 tools/ci/m3/docs_freshness_gate.py --repo-root ."
        ),
        "status": "pass"
        if not [f for f in findings if f.severity == "error"]
        else "fail",
        "finding_counts": {
            "error": sum(1 for f in findings if f.severity == "error"),
            "warning": sum(
                1 for f in findings if f.severity == "warning"
            ),
        },
        "enforced_rows": [r.as_report() for r in enforced_rows],
        "consuming_surfaces": [s.as_report() for s in surfaces],
        "release_notes": release_notes,
        "findings": [f.as_report() for f in findings],
    }
    if forced_drill is not None:
        capture["forced_drill_replay"] = {
            "drill_id": forced_drill.get("drill_id"),
            "expected_check_id": forced_drill.get("expected_check_id"),
            "actionable_next_action": forced_drill.get(
                "actionable_next_action"
            ),
            "observed": forced_drill_observed,
        }
    path.write_text(
        json.dumps(capture, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )


def find_forced_drill(
    source_map: dict[str, Any],
    drill_id: str,
) -> dict[str, Any]:
    drills = ensure_list(
        source_map.get("failure_drills", []),
        "source_map.failure_drills",
    )
    for drill in drills:
        if (
            isinstance(drill, dict)
            and drill.get("drill_id") == drill_id
        ):
            return drill
    raise SystemExit(
        f"--force-drill {drill_id!r} does not match any "
        "failure_drills[].drill_id in the source map."
    )


def main() -> int:
    parser = base_argument_parser(description=__doc__)
    parser.add_argument(
        "--report", default=DEFAULT_REPORT_REL,
        help="Where to write the generated truth report markdown.",
    )
    parser.add_argument(
        "--capture", default=DEFAULT_CAPTURE_REL,
        help="Where to write the durable JSON capture.",
    )
    args = parser.parse_args()

    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(
            f"--repo-root does not look like a repository root: {repo_root}"
        )

    source_map = load_source_map(repo_root, args.source_map)
    today = resolve_today(args.today)

    manifest_ref = ensure_str(
        source_map.get("claim_manifest_ref"),
        "source_map.claim_manifest_ref",
    )
    compat_ref = ensure_str(
        source_map.get("compatibility_report_ref"),
        "source_map.compatibility_report_ref",
    )
    release_notes_ref = ensure_str(
        source_map.get("release_notes_draft_ref"),
        "source_map.release_notes_draft_ref",
    )
    review_window_days_floor = int(
        source_map.get("review_window_days_floor", 60)
    )

    manifest = load_claim_manifest(repo_root, manifest_ref)
    compat_report = load_compatibility_report(repo_root, compat_ref)

    findings: list[Finding] = []

    # Source-map same-change-set guard.
    source_as_of = ensure_str(source_map.get("as_of"), "source_map.as_of")
    manifest_as_of = ensure_str(manifest.get("as_of"), "manifest.as_of")
    compat_as_of = ensure_str(
        compat_report.get("as_of"), "compatibility_report.as_of"
    )
    if source_as_of != manifest_as_of:
        findings.append(
            Finding(
                severity="error",
                check_id="source_map.as_of_drift_against_manifest",
                message=(
                    f"source_map.as_of {source_as_of} drifts from "
                    f"claim_manifest.as_of {manifest_as_of}"
                ),
                remediation=(
                    "Refresh the source map in the same change set as the "
                    "regenerated claim manifest."
                ),
            )
        )
    if source_as_of != compat_as_of:
        findings.append(
            Finding(
                severity="error",
                check_id="source_map.as_of_drift_against_compat",
                message=(
                    f"source_map.as_of {source_as_of} drifts from "
                    f"compatibility_report.as_of {compat_as_of}"
                ),
                remediation=(
                    "Refresh the source map in the same change set as the "
                    "regenerated compatibility report."
                ),
            )
        )

    # Vocabularies.
    required_vocabs = [
        ensure_str(v, "required_manifest_vocabularies[]")
        for v in ensure_list(
            source_map.get("required_manifest_vocabularies", []),
            "source_map.required_manifest_vocabularies",
        )
    ]
    check_manifest_vocabularies(manifest, required_vocabs, findings)

    # Consuming surfaces.
    consuming = ensure_list(
        source_map.get("consuming_surfaces", []),
        "source_map.consuming_surfaces",
    )
    surfaces = check_consuming_surfaces(repo_root, consuming, findings)

    # Force-drill handling for release-notes manifest backlink.
    forced_drill: dict[str, Any] | None = None
    forced_drill_observed: dict[str, Any] | None = None
    forced_strip_manifest_id = False
    if args.force_drill:
        forced_drill = find_forced_drill(source_map, args.force_drill)
        target = forced_drill.get("target")
        if target != "release_notes_draft":
            raise SystemExit(
                f"--force-drill {args.force_drill!r} targets {target!r}; "
                "the freshness gate only replays release_notes_draft "
                "drills. Use tools/ci/m3/stale_example_checker.py for "
                "protected_example drills."
            )
        forced_input = ensure_dict(
            forced_drill.get("forced_input", {}),
            f"{args.force_drill}.forced_input",
        )
        forced_strip_manifest_id = bool(
            forced_input.get("strip_manifest_id")
        )

    enforced_row_entries = ensure_list(
        source_map.get("enforced_rows", []),
        "source_map.enforced_rows",
    )
    enforced_compat_entries = ensure_list(
        source_map.get("enforced_compat_rows", []),
        "source_map.enforced_compat_rows",
    )
    release_notes_reqs = ensure_dict(
        source_map.get("release_notes_requirements", {}),
        "source_map.release_notes_requirements",
    )

    # Release notes.
    release_notes_summary = check_release_notes(
        repo_root=repo_root,
        release_notes_ref=release_notes_ref,
        manifest=manifest,
        compat_report=compat_report,
        enforced_rows=enforced_row_entries,
        enforced_compat_rows=enforced_compat_entries,
        requirements=release_notes_reqs,
        findings=findings,
        forced_strip_manifest_id=forced_strip_manifest_id,
    )

    # Manifest rows (freshness, badge, citations).
    enforced_rows = check_enforced_rows(
        manifest=manifest,
        enforced=enforced_row_entries,
        review_window_days_floor=review_window_days_floor,
        today=today,
        release_notes=release_notes_summary,
        findings=findings,
    )

    # Compat-row presence in compat report.
    compat_index = index_compat_rows(compat_report)
    for entry in enforced_compat_entries:
        compat_id = ensure_str(
            entry.get("compat_row_id"), "enforced_compat_rows[]"
        )
        if compat_id not in compat_index:
            findings.append(
                Finding(
                    severity="error",
                    check_id="enforced_compat_row.unknown_row",
                    message=(
                        f"enforced compat row {compat_id!r} is not present "
                        f"in the M3 compatibility report"
                    ),
                    remediation=(
                        "Add the row to the compatibility matrix or drop "
                        "it from the source map in the same change set."
                    ),
                    ref=compat_id,
                )
            )

    generated_at = now_iso_z()
    report_text = render_truth_report(
        manifest=manifest,
        compat_report=compat_report,
        source_map_rel=args.source_map,
        enforced_rows=enforced_rows,
        surfaces=surfaces,
        release_notes=release_notes_summary,
        findings=findings,
        generated_at=generated_at,
        today=today,
        review_window_days_floor=review_window_days_floor,
    )

    report_changed = write_if_changed(
        repo_root / args.report, report_text, args.check
    )
    if args.check and report_changed:
        findings.append(
            Finding(
                severity="error",
                check_id="docs_truth_report.stale",
                message=(
                    "checked-in docs truth report is stale relative to "
                    "the source map or upstream truth"
                ),
                remediation=(
                    "Run `python3 tools/ci/m3/docs_freshness_gate.py "
                    "--repo-root .` and commit the regenerated report."
                ),
            )
        )

    if forced_drill is not None:
        observed_check_ids = [
            f.check_id for f in findings if f.severity == "error"
        ]
        forced_drill_observed = {
            "observed_failed_check_ids": observed_check_ids,
            "reproduced": forced_drill.get("expected_check_id")
            in observed_check_ids,
        }

    write_capture(
        repo_root / args.capture,
        findings=findings,
        source_map_rel=args.source_map,
        manifest_ref=manifest_ref,
        compat_ref=compat_ref,
        release_notes_ref=release_notes_ref,
        report_rel=args.report,
        enforced_rows=enforced_rows,
        surfaces=surfaces,
        release_notes=release_notes_summary,
        generated_at=generated_at,
        today=today,
        forced_drill=forced_drill,
        forced_drill_observed=forced_drill_observed,
        build_identity_ref=args.build_identity,
    )

    # Print summary.
    errors = [f for f in findings if f.severity == "error"]
    label = "m3-docs-truth"
    status = "PASS" if not errors else "FAIL"
    print(
        f"[{label}] {status} ({len(errors)} errors, "
        f"{len(findings) - len(errors)} warnings) — report: "
        f"{args.report}"
    )
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        print(
            f"[{label}] {prefix} {finding.check_id}: {finding.message}"
            f"{ref_suffix}"
        )
        print(f"[{label}]   remediation: {finding.remediation}")

    if forced_drill is not None:
        expected = forced_drill.get("expected_check_id")
        reproduced = forced_drill_observed["reproduced"]  # type: ignore[index]
        if reproduced:
            print(
                f"[{label}] forced drill "
                f"{forced_drill.get('drill_id')!r} reproduced "
                f"{expected!r}"
            )
            return 0
        print(
            f"[{label}] forced drill "
            f"{forced_drill.get('drill_id')!r} did NOT reproduce "
            f"{expected!r}; observed: "
            f"{forced_drill_observed['observed_failed_check_ids']}"  # type: ignore[index]
        )
        return 2

    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[m3-docs-truth] interrupted", file=sys.stderr)
        sys.exit(130)
