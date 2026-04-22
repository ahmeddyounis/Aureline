#!/usr/bin/env python3
"""Generate a reusable command-surface parity diff report from the seed corpus."""

from __future__ import annotations

import argparse
import json
import sys
from collections import Counter
from dataclasses import dataclass
from pathlib import Path
from typing import Any

SEED_REL = "artifacts/commands/command_parity_seed.yaml"


@dataclass
class Finding:
    category: str
    command_id: str
    surface_family: str
    detail: str


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--seed", default=SEED_REL)
    parser.add_argument("--format", choices=("markdown", "json"), default="markdown")
    parser.add_argument("--write-report", default=None)
    parser.add_argument(
        "--strict",
        action="store_true",
        help="Exit non-zero when actionable findings are present.",
    )
    return parser.parse_args()


def read_jsonish(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as fh:
        return json.load(fh)


def load_descriptor(repo_root: Path, rel: str) -> dict[str, Any]:
    payload = read_jsonish(repo_root / rel)
    if payload.get("record_kind") != "command_descriptor_record":
        raise SystemExit(f"{rel} is not a command_descriptor_record")
    return payload


def descriptor_ui_slot_hint(
    descriptor: dict[str, Any], ui_slot_class: str | None
) -> dict[str, Any] | None:
    if ui_slot_class is None:
        return None
    for hint in descriptor.get("ui_slot_hints", []):
        if hint["ui_slot_class"] == ui_slot_class:
            return hint
    return None


def is_high_risk(seed: dict[str, Any], descriptor: dict[str, Any]) -> bool:
    high_risk_scopes = set(seed["report_contract"]["high_risk_scope_classes"])
    return (
        descriptor["capability_scope_class"] in high_risk_scopes
        or descriptor["preview_class"] != "no_preview_required"
    )


def canonical_aliases(descriptor: dict[str, Any]) -> set[str]:
    return {row["alias_id"] for row in descriptor.get("aliases", [])}


def surface_defaults(seed: dict[str, Any]) -> dict[str, str]:
    defaults: dict[str, str] = {}
    for row in seed["report_contract"]["surface_families"]:
        defaults[row["surface_family"]] = row["default_authority_class"]
    return defaults


def sort_strings(values: list[str]) -> list[str]:
    return sorted(values)


def compare_claim(
    descriptor: dict[str, Any],
    claim: dict[str, Any],
    authority_defaults: dict[str, str],
) -> list[Finding]:
    command_id = descriptor["command_id"]
    surface = claim["surface_family"]
    findings: list[Finding] = []

    if claim["projected_command_id"] != command_id:
        findings.append(
            Finding(
                "command_id_drift",
                command_id,
                surface,
                f"surface carries {claim['projected_command_id']} instead of {command_id}",
            )
        )

    if claim["projected_label_ref"] != descriptor["primary_label_ref"]:
        findings.append(
            Finding(
                "label_or_alias_drift",
                command_id,
                surface,
                "surface label ref does not match the canonical primary_label_ref",
            )
        )

    declared_aliases = canonical_aliases(descriptor)
    exposed_aliases = set(claim.get("exposed_alias_ids", []))
    undeclared_aliases = sort_strings(list(exposed_aliases - declared_aliases))
    if undeclared_aliases:
        findings.append(
            Finding(
                "label_or_alias_drift",
                command_id,
                surface,
                "surface exposes undeclared aliases: " + ", ".join(undeclared_aliases),
            )
        )

    hidden_aliases = sort_strings(claim.get("hidden_alias_ids", []))
    if hidden_aliases:
        findings.append(
            Finding(
                "surface_specific_hidden_alias",
                command_id,
                surface,
                "surface hides alias ids from parity consumers: " + ", ".join(hidden_aliases),
            )
        )

    if claim.get("docs_help_anchor_ref") is None:
        findings.append(
            Finding(
                "missing_help_docs_anchor",
                command_id,
                surface,
                "surface cannot point back to the canonical docs/help anchor",
            )
        )
    elif claim["docs_help_anchor_ref"] != descriptor["docs_help_anchor_ref"]:
        findings.append(
            Finding(
                "missing_help_docs_anchor",
                command_id,
                surface,
                "surface points at a docs/help anchor that differs from the canonical descriptor",
            )
        )

    if claim["preview_class"] != descriptor["preview_class"]:
        findings.append(
            Finding(
                "mismatched_preview_posture",
                command_id,
                surface,
                f"surface reports {claim['preview_class']} but descriptor requires {descriptor['preview_class']}",
            )
        )

    expected_authority = authority_defaults.get(surface)
    if expected_authority is not None and claim["authority_class"] != expected_authority:
        findings.append(
            Finding(
                "authority_class_drift",
                command_id,
                surface,
                f"surface reports authority {claim['authority_class']} but the surface family default is {expected_authority}",
            )
        )

    descriptor_result = descriptor["result_contract"]["result_contract_class"]
    if claim["result_contract_class"] != descriptor_result:
        findings.append(
            Finding(
                "result_contract_drift",
                command_id,
                surface,
                "surface reports a different result_contract_class than the descriptor",
            )
        )

    canonical_evidence = sort_strings(descriptor["result_contract"]["evidence_ref_class_required"])
    projected_evidence = sort_strings(claim.get("evidence_ref_classes", []))
    if projected_evidence != canonical_evidence:
        findings.append(
            Finding(
                "result_contract_drift",
                command_id,
                surface,
                "surface evidence refs do not match the descriptor-owned result contract",
            )
        )

    enablement = claim["enablement"]
    if enablement["disabled_reason_mode"] == "missing":
        findings.append(
            Finding(
                "missing_disabled_reason",
                command_id,
                surface,
                "surface drops typed disabled-reason disclosure",
            )
        )

    hint = descriptor_ui_slot_hint(descriptor, claim.get("descriptor_ui_slot_class"))
    if hint is not None:
        if enablement["contextual_filter_class_ref"] != hint["contextual_filter_class_ref"]:
            findings.append(
                Finding(
                    "enablement_rule_drift",
                    command_id,
                    surface,
                    "surface contextual filter differs from the descriptor-owned ui_slot_hint",
                )
            )
        if sort_strings(enablement.get("menu_path_refs", [])) != sort_strings(hint["menu_path_refs"]):
            findings.append(
                Finding(
                    "enablement_rule_drift",
                    command_id,
                    surface,
                    "surface menu path differs from the descriptor-owned ui_slot_hint",
                )
            )

    return findings


def analyze_seed(repo_root: Path, seed: dict[str, Any]) -> dict[str, Any]:
    authority_defaults = surface_defaults(seed)
    commands_report: list[dict[str, Any]] = []
    all_findings: list[Finding] = []

    for command_seed in seed["commands"]:
        descriptor = load_descriptor(repo_root, command_seed["descriptor_ref"])
        command_findings: list[Finding] = []
        rows: list[dict[str, Any]] = []
        high_risk = is_high_risk(seed, descriptor)

        for claim in command_seed["surface_matrix"]:
            status = claim["coverage_status"]
            surface = claim["surface_family"]
            row_findings: list[Finding] = []
            if status == "claimed":
                row_findings = compare_claim(descriptor, claim, authority_defaults)
                command_findings.extend(row_findings)
            elif status == "unknown_gap" and high_risk:
                row_findings = [
                    Finding(
                        "unknown_high_risk_gap",
                        descriptor["command_id"],
                        surface,
                        "high-risk command has no claim or explicit narrowing for this surface family",
                    )
                ]
                command_findings.extend(row_findings)

            rows.append(
                {
                    "surface_family": surface,
                    "coverage_status": status,
                    "slot_key": claim.get("slot_key"),
                    "preview_class": claim.get("preview_class"),
                    "authority_class": claim.get("authority_class"),
                    "result_contract_class": claim.get("result_contract_class"),
                    "finding_categories": [finding.category for finding in row_findings],
                    "narrowing_reason_code": claim.get("narrowing_reason_code"),
                }
            )

        all_findings.extend(command_findings)
        commands_report.append(
            {
                "command_id": descriptor["command_id"],
                "canonical_verb": descriptor["canonical_verb"],
                "descriptor_ref": command_seed["descriptor_ref"],
                "high_risk": high_risk,
                "notes": command_seed.get("notes"),
                "rows": rows,
                "findings": [
                    {
                        "category": finding.category,
                        "surface_family": finding.surface_family,
                        "detail": finding.detail,
                    }
                    for finding in command_findings
                ],
            }
        )

    category_counts = Counter(finding.category for finding in all_findings)
    claimed_rows = sum(
        1
        for command in commands_report
        for row in command["rows"]
        if row["coverage_status"] == "claimed"
    )
    narrowed_rows = sum(
        1
        for command in commands_report
        for row in command["rows"]
        if row["coverage_status"] == "explicitly_narrowed"
    )
    unknown_rows = sum(
        1
        for command in commands_report
        for row in command["rows"]
        if row["coverage_status"] == "unknown_gap"
    )

    return {
        "seed_id": seed["seed_id"],
        "title": seed["title"],
        "comparison_axes": seed["report_contract"]["comparison_axes"],
        "surface_families": seed["report_contract"]["surface_families"],
        "failure_categories": seed["report_contract"]["failure_categories"],
        "summary": {
            "command_count": len(commands_report),
            "claimed_surface_rows": claimed_rows,
            "explicitly_narrowed_rows": narrowed_rows,
            "unknown_gap_rows": unknown_rows,
            "actionable_finding_count": len(all_findings),
            "category_counts": dict(sorted(category_counts.items())),
        },
        "commands": commands_report,
        "findings": [
            {
                "category": finding.category,
                "command_id": finding.command_id,
                "surface_family": finding.surface_family,
                "detail": finding.detail,
            }
            for finding in all_findings
        ],
    }


def format_surface_table(rows: list[dict[str, Any]]) -> list[str]:
    lines = [
        "| Surface | Status | Slot | Preview | Authority | Result | Findings |",
        "|---|---|---|---|---|---|---|",
    ]
    for row in rows:
        slot = row["slot_key"] or "-"
        preview = row["preview_class"] or "-"
        authority = row["authority_class"] or "-"
        result = row["result_contract_class"] or "-"
        findings = ", ".join(row["finding_categories"]) if row["finding_categories"] else "-"
        status = row["coverage_status"]
        if status == "explicitly_narrowed" and row.get("narrowing_reason_code"):
            status = f"explicitly_narrowed ({row['narrowing_reason_code']})"
        lines.append(
            f"| `{row['surface_family']}` | `{status}` | `{slot}` | `{preview}` | `{authority}` | `{result}` | {findings} |"
        )
    return lines


def render_markdown(analysis: dict[str, Any]) -> str:
    lines: list[str] = [
        "# Command parity diff seed report",
        "",
        "This report is emitted from `artifacts/commands/command_parity_seed.yaml` by `tools/commands/parity_diff_seed.py`. It is a seed-only parity corpus for launch-bearing command surfaces, not a claim that live runtime surfaces already ship.",
        "",
        "## Report contract",
        "",
        "### Surface families",
        "",
        "| Surface family | Meaning | Default authority |",
        "|---|---|---|",
    ]
    for row in analysis["surface_families"]:
        lines.append(
            f"| `{row['surface_family']}` | {row['description']} | `{row['default_authority_class']}` |"
        )

    lines.extend(
        [
            "",
            "### Comparison axes",
            "",
            "| Axis | Canonical source |",
            "|---|---|",
        ]
    )
    axis_explanations = {
        "stable_command_id": "Descriptor `command_id` projected onto every claimed surface row.",
        "label_or_alias": "Descriptor `primary_label_ref`, `canonical_verb`, and declared alias ids.",
        "enablement_rules": "Descriptor `ui_slot_hints`, `palette_visibility`, `client_scopes`, and typed disabled-reason disclosure.",
        "preview_posture": "Descriptor `preview_class` carried through palette, menu/button, help, CLI, and AI routes.",
        "authority_class": "Surface-family default authority lane (`user_initiated_local` or `ai_initiated`) without widening command semantics.",
        "result_contract": "Descriptor `result_contract_class` and `evidence_ref_class_required`.",
    }
    for axis in analysis["comparison_axes"]:
        lines.append(f"| `{axis}` | {axis_explanations.get(axis, 'Seed-defined axis.')} |")

    lines.extend(
        [
            "",
            "### Failure categories",
            "",
            "| Category | Severity | Trigger |",
            "|---|---|---|",
        ]
    )
    for row in analysis["failure_categories"]:
        lines.append(
            f"| `{row['category']}` | `{row['severity']}` | {row['description']} |"
        )

    summary = analysis["summary"]
    lines.extend(
        [
            "",
            "## Seed summary",
            "",
            f"- Seed id: `{analysis['seed_id']}`",
            f"- Commands: `{summary['command_count']}`",
            f"- Claimed surface rows: `{summary['claimed_surface_rows']}`",
            f"- Explicit narrowings: `{summary['explicitly_narrowed_rows']}`",
            f"- Unknown gaps: `{summary['unknown_gap_rows']}`",
            f"- Actionable findings: `{summary['actionable_finding_count']}`",
            "",
            "### Findings by category",
            "",
            "| Category | Count |",
            "|---|---|",
        ]
    )
    if summary["category_counts"]:
        for category, count in summary["category_counts"].items():
            lines.append(f"| `{category}` | `{count}` |")
    else:
        lines.append("| `none` | `0` |")

    lines.extend(
        [
            "",
            "## Actionable findings",
            "",
            "| Category | Command | Surface | Detail |",
            "|---|---|---|---|",
        ]
    )
    if analysis["findings"]:
        for finding in analysis["findings"]:
            lines.append(
                f"| `{finding['category']}` | `{finding['command_id']}` | `{finding['surface_family']}` | {finding['detail']} |"
            )
    else:
        lines.append("| `none` | - | - | No actionable findings. |")

    for command in analysis["commands"]:
        risk_label = "high-risk" if command["high_risk"] else "standard-risk"
        lines.extend(
            [
                "",
                f"## {command['command_id']}",
                "",
                f"- Descriptor: `{command['descriptor_ref']}`",
                f"- Canonical verb: `{command['canonical_verb']}`",
                f"- Risk class: `{risk_label}`",
                f"- Seed notes: {command['notes']}",
                "",
            ]
        )
        lines.extend(format_surface_table(command["rows"]))
        lines.append("")
        lines.append("### Findings")
        lines.append("")
        if command["findings"]:
            for finding in command["findings"]:
                lines.append(
                    f"- `{finding['category']}` on `{finding['surface_family']}`: {finding['detail']}"
                )
        else:
            lines.append("- None.")

    lines.extend(
        [
            "",
            "## CI reuse",
            "",
            "- Use `--format json` for machine-readable summaries.",
            "- Use `--strict` to make actionable findings fail the invocation with exit code `1`.",
            "- Keep the seed corpus synthetic until runtime surface exports land; later CI can swap the seed rows for generated surface captures without changing the report structure.",
        ]
    )
    return "\n".join(lines) + "\n"


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    seed = read_jsonish(repo_root / args.seed)
    analysis = analyze_seed(repo_root, seed)
    if args.format == "json":
        rendered = json.dumps(analysis, indent=2, sort_keys=True) + "\n"
    else:
        rendered = render_markdown(analysis)

    if args.write_report:
        out_path = repo_root / args.write_report
        out_path.parent.mkdir(parents=True, exist_ok=True)
        out_path.write_text(rendered, encoding="utf-8")
    else:
        sys.stdout.write(rendered)

    if args.strict and analysis["summary"]["actionable_finding_count"] > 0:
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
