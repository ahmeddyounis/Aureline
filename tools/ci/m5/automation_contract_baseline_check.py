#!/usr/bin/env python3
"""M5 automation contract-baseline gate.

This gate enforces that the checked-in automation contract baseline stays honest:
the canonical packet binds all six automation object families (recipe builder,
parameter review, dry-run/explain, run history, macro recorder, and safety
labels), reuses the whole controlled-automation-label safety-label vocabulary,
keeps every freeze invariant true, and promotes to ``stable`` with no findings. A
baseline that drops a family, declares a family with no schema / evidence hook /
consumer surface / state vocabulary, ships an incomplete or miscategorized
safety-label set, drops its reused-contract refs, or violates an invariant
*blocks stable*. The gate also checks that:

- the support export, CLI/headless view, safety-label manifest, and compact
  projection are present and consistent with the packet;
- the worked-example recipe-macro fixtures exist and carry the expected record
  kinds; and
- every baseline mutation fixture except ``baseline_stable`` reproduces a
  ``blocks_stable`` promotion state.

The typed Rust consumer mints the same packet, so
``cargo test -p aureline-runtime --test m5_automation_contract_baseline`` enforces
the same invariants and that the fixtures and artifacts are bit-for-bit derivable
from the seed.

Exit codes:

- ``0`` -- baseline is clean.
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

ARTIFACT_DIR = Path("artifacts/m5/automation/automation-contract-baseline")
PACKET_REL = ARTIFACT_DIR / "packet.json"
SUPPORT_EXPORT_REL = ARTIFACT_DIR / "support_export.json"
CLI_HEADLESS_REL = ARTIFACT_DIR / "cli_headless.json"
SAFETY_LABEL_MANIFEST_REL = ARTIFACT_DIR / "safety_label_manifest.json"
COMPACT_REL = ARTIFACT_DIR / "compact.txt"

BASELINE_SCHEMA_REL = Path("schemas/automation/automation-contract-baseline.schema.json")
RECIPE_BUILDER_SCHEMA_REL = Path("schemas/automation/recipe-builder.schema.json")
MACRO_SESSION_SCHEMA_REL = Path("schemas/automation/macro-session.schema.json")
DOC_REL = Path("docs/m5/recipe-builder-and-macro-contract.md")

RECIPE_MACRO_FIXTURE_DIR = Path("fixtures/automation/m5/recipe-macro")
BASELINE_FIXTURE_DIR = Path("fixtures/automation/m5/automation-contract-baseline")

EXPECTED_RECORD_KIND = "m5_automation_contract_baseline_packet"
EXPECTED_SUPPORT_RECORD_KIND = "m5_automation_contract_baseline_support_export"
EXPECTED_CLI_RECORD_KIND = "m5_automation_contract_baseline_cli_headless"
EXPECTED_MANIFEST_RECORD_KIND = "automation_safety_label_manifest_record"
EXPECTED_SCHEMA_VERSION = 1

REQUIRED_FAMILIES = [
    "recipe_builder",
    "parameter_review",
    "dry_run_explain",
    "run_history",
    "macro_recorder",
    "safety_labels",
]

ADMISSIBILITY_CUES = ["macro_safe", "recipe_safe", "headless_safe", "ui_only", "approval_required"]
EFFECT_DISCLOSURES = ["writes_files", "runs_process", "network_call", "remote_mutation"]
REQUIRED_LABELS = ADMISSIBILITY_CUES + EFFECT_DISCLOSURES

REQUIRED_INVARIANTS = [
    "recipe_builder_emits_declarative_manifests_only",
    "macro_recorder_constrained_to_ui_or_editor_state",
    "dry_run_explain_required_before_irreversible_apply",
    "parameter_review_resolves_provenance_before_apply",
    "one_safety_label_vocabulary_reused_across_surfaces",
    "safety_labels_project_from_existing_axes_not_minted",
    "run_history_reuses_the_canonical_run_record",
    "no_hidden_ui_shortcut_widens_automation_authority",
    "reruns_reresolve_current_context_never_replay_stale_authority",
]

RECIPE_MACRO_FIXTURES = {
    "recipe_builder_session_preview_ready.json": "recipe_builder_session_record",
    "recipe_builder_session_blocked.json": "recipe_builder_session_record",
    "parameter_review_sheet.json": "parameter_review_sheet_record",
    "dry_run_explain_packet.json": "dry_run_explain_packet_record",
    "macro_session_stopped_promotable.json": "macro_session_record",
    "macro_session_discarded.json": "macro_session_record",
}

BASELINE_FIXTURES = [
    "baseline_stable.json",
    "missing_object_family_blocks_stable.json",
    "family_missing_evidence_hook_blocks_stable.json",
    "family_missing_consumer_surface_blocks_stable.json",
    "safety_label_set_incomplete_blocks_stable.json",
    "safety_label_miscategorized_blocks_stable.json",
    "invariant_violated_blocks_stable.json",
]

DOC_BACKLINKS = (
    "schemas/automation/recipe-builder.schema.json",
    "schemas/automation/macro-session.schema.json",
    "schemas/automation/automation-contract-baseline.schema.json",
    "artifacts/m5/automation/automation-contract-baseline/",
    "fixtures/automation/m5/recipe-macro/",
    "tools/ci/m5/automation_contract_baseline_check.py",
)

CONTROLLED_LABEL_AXIS = "schemas/automation/automation-manifest.schema.json"


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
    parser.add_argument("--format", choices=("text", "json"), default="text")
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


def check_packet(packet: dict[str, Any], findings: list[Finding]) -> None:
    if packet.get("record_kind") != EXPECTED_RECORD_KIND:
        findings.append(Finding("packet_record_kind", f"packet record_kind must be {EXPECTED_RECORD_KIND}"))
    if packet.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(Finding("packet_schema_version", f"packet schema_version must be {EXPECTED_SCHEMA_VERSION}"))

    families = packet.get("object_families")
    if not isinstance(families, list):
        findings.append(Finding("families_missing", "object_families must be a list"))
        families = []
    seen_families = [f.get("family") for f in families if isinstance(f, dict)]
    for required in REQUIRED_FAMILIES:
        if required not in seen_families:
            findings.append(Finding("missing_object_family", "a required object family is absent", subject=required))
    if len(seen_families) != len(set(seen_families)):
        findings.append(Finding("duplicate_object_family", "an object family is bound more than once"))

    for binding in families:
        if not isinstance(binding, dict):
            continue
        name = binding.get("family", "<unknown>")
        if not binding.get("schema_ref"):
            findings.append(Finding("family_missing_schema_ref", "a family cites no boundary schema", subject=name))
        if not binding.get("evidence_hook_refs"):
            findings.append(Finding("family_missing_evidence_hook", "a family cites no evidence hook", subject=name))
        if not binding.get("consumer_surfaces"):
            findings.append(Finding("family_missing_consumer_surface", "a family names no consumer surface", subject=name))
        if not binding.get("state_vocabulary"):
            findings.append(Finding("family_missing_state_vocabulary", "a family declares no state vocabulary", subject=name))
        if not binding.get("doc_anchor"):
            findings.append(Finding("family_missing_doc_anchor", "a family declares no doc anchor", subject=name))

    check_safety_labels(packet.get("safety_labels"), findings, source="packet")

    if not packet.get("reused_contract_refs"):
        findings.append(Finding("reused_contract_ref_missing", "the baseline cites no reused contract refs"))

    invariants = packet.get("invariants")
    if not isinstance(invariants, dict):
        findings.append(Finding("invariants_missing", "invariants must be an object"))
        invariants = {}
    for name in REQUIRED_INVARIANTS:
        if invariants.get(name) is not True:
            findings.append(Finding("invariant_violated", "a freeze invariant is not true", subject=name))

    promotion = packet.get("promotion_state")
    if promotion != "stable":
        findings.append(Finding("packet_not_stable", f"packet promotion_state must be stable, got {promotion}"))
    if packet.get("validation_findings"):
        findings.append(Finding("packet_has_findings", "a stable packet must carry no validation findings"))

    digest = packet.get("baseline_digest", "")
    if not isinstance(digest, str) or not digest.startswith("fnv1a64:"):
        findings.append(Finding("packet_digest", "packet baseline_digest must be an fnv1a64 digest"))


def check_safety_labels(labels: Any, findings: list[Finding], *, source: str) -> None:
    if not isinstance(labels, list):
        findings.append(Finding("safety_label_set_incomplete", f"{source} safety_labels must be a list"))
        return
    seen = {}
    for label in labels:
        if not isinstance(label, dict):
            continue
        seen[label.get("label_id")] = label
    for required in REQUIRED_LABELS:
        if required not in seen:
            findings.append(Finding("safety_label_set_incomplete", f"{source} is missing a safety label", subject=required))
            continue
        label = seen[required]
        expected_kind = "admissibility_cue" if required in ADMISSIBILITY_CUES else "effect_disclosure"
        if label.get("label_kind") != expected_kind:
            findings.append(
                Finding(
                    "safety_label_miscategorized",
                    f"{source} safety label categorized as {label.get('label_kind')} but must be {expected_kind}",
                    subject=required,
                )
            )
        axis = label.get("source_axis_ref", "")
        if CONTROLLED_LABEL_AXIS not in axis:
            findings.append(
                Finding(
                    "safety_label_axis_minted",
                    f"{source} safety label must project from the controlled-automation-label axis",
                    subject=required,
                )
            )


def check_support_export(export: dict[str, Any], packet: dict[str, Any], findings: list[Finding]) -> None:
    if export.get("record_kind") != EXPECTED_SUPPORT_RECORD_KIND:
        findings.append(Finding("support_record_kind", f"support export record_kind must be {EXPECTED_SUPPORT_RECORD_KIND}"))
    if export.get("packet_id") != packet.get("packet_id"):
        findings.append(Finding("support_packet_id", "support export packet_id must match the packet"))
    if export.get("baseline_digest") != packet.get("baseline_digest"):
        findings.append(Finding("support_digest", "support export baseline_digest must match the packet"))
    rows = export.get("family_rows")
    if not isinstance(rows, list) or len(rows) != len(REQUIRED_FAMILIES):
        findings.append(Finding("support_family_rows", "support export must carry one row per family"))
    check_safety_labels(export.get("safety_labels"), findings, source="support export")


def check_cli_headless(view: dict[str, Any], findings: list[Finding]) -> None:
    if view.get("record_kind") != EXPECTED_CLI_RECORD_KIND:
        findings.append(Finding("cli_record_kind", f"cli/headless record_kind must be {EXPECTED_CLI_RECORD_KIND}"))
    if not isinstance(view.get("family_lines"), list) or len(view.get("family_lines", [])) != len(REQUIRED_FAMILIES):
        findings.append(Finding("cli_family_lines", "cli/headless view must explain every family"))
    if not isinstance(view.get("safety_label_lines"), list) or len(view.get("safety_label_lines", [])) != len(REQUIRED_LABELS):
        findings.append(Finding("cli_label_lines", "cli/headless view must explain every safety label"))


def check_safety_label_manifest(manifest: dict[str, Any], findings: list[Finding]) -> None:
    if manifest.get("record_kind") != EXPECTED_MANIFEST_RECORD_KIND:
        findings.append(Finding("manifest_record_kind", f"safety-label manifest record_kind must be {EXPECTED_MANIFEST_RECORD_KIND}"))
    check_safety_labels(manifest.get("labels"), findings, source="safety-label manifest")


def check_recipe_macro_fixtures(root: Path, findings: list[Finding]) -> None:
    for file_name, record_kind in RECIPE_MACRO_FIXTURES.items():
        path = root / RECIPE_MACRO_FIXTURE_DIR / file_name
        if not path.exists():
            findings.append(Finding("missing_recipe_macro_fixture", "a worked-example fixture is missing", subject=file_name))
            continue
        payload = ensure_dict(load_json(path), str(path))
        if payload.get("record_kind") != record_kind:
            findings.append(
                Finding(
                    "recipe_macro_fixture_record_kind",
                    f"worked-example fixture record_kind must be {record_kind}",
                    subject=file_name,
                )
            )


def check_baseline_fixtures(root: Path, findings: list[Finding]) -> None:
    for file_name in BASELINE_FIXTURES:
        path = root / BASELINE_FIXTURE_DIR / file_name
        if not path.exists():
            findings.append(Finding("missing_baseline_fixture", "a baseline mutation fixture is missing", subject=file_name))
            continue
        payload = ensure_dict(load_json(path), str(path))
        expect = payload.get("expect", {})
        promotion = expect.get("promotion_state")
        if file_name == "baseline_stable.json":
            if promotion != "stable" or expect.get("is_stable") is not True:
                findings.append(Finding("baseline_fixture_not_stable", "baseline_stable fixture must promote stable", subject=file_name))
        else:
            if promotion != "blocks_stable" or expect.get("is_stable") is not False:
                findings.append(Finding("baseline_fixture_not_blocking", "a mutation fixture must block stable", subject=file_name))
            if not expect.get("expected_finding_kinds"):
                findings.append(Finding("baseline_fixture_no_findings", "a blocking fixture must list expected finding kinds", subject=file_name))


def check_doc(root: Path, findings: list[Finding]) -> None:
    path = root / DOC_REL
    if not path.exists():
        findings.append(Finding("doc_missing", "the reviewer contract doc is missing", subject=str(DOC_REL)))
        return
    body = path.read_text(encoding="utf-8")
    for backlink in DOC_BACKLINKS:
        if backlink not in body:
            findings.append(Finding("doc_backlink_missing", "the doc must backlink the companion artifact", subject=backlink))


def run(root: Path) -> list[Finding]:
    findings: list[Finding] = []

    for schema_rel in (BASELINE_SCHEMA_REL, RECIPE_BUILDER_SCHEMA_REL, MACRO_SESSION_SCHEMA_REL):
        schema_path = root / schema_rel
        if not schema_path.exists():
            findings.append(Finding("schema_missing", "a boundary schema is missing", subject=str(schema_rel)))
        else:
            ensure_dict(load_json(schema_path), str(schema_path))

    packet = ensure_dict(load_json(root / PACKET_REL), str(PACKET_REL))
    check_packet(packet, findings)

    support = ensure_dict(load_json(root / SUPPORT_EXPORT_REL), str(SUPPORT_EXPORT_REL))
    check_support_export(support, packet, findings)

    cli = ensure_dict(load_json(root / CLI_HEADLESS_REL), str(CLI_HEADLESS_REL))
    check_cli_headless(cli, findings)

    manifest = ensure_dict(load_json(root / SAFETY_LABEL_MANIFEST_REL), str(SAFETY_LABEL_MANIFEST_REL))
    check_safety_label_manifest(manifest, findings)

    if not (root / COMPACT_REL).exists():
        findings.append(Finding("compact_missing", "compact.txt is missing", subject=str(COMPACT_REL)))

    check_recipe_macro_fixtures(root, findings)
    check_baseline_fixtures(root, findings)
    check_doc(root, findings)

    return findings


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    findings = run(repo_root)

    if args.format == "json":
        sys.stdout.write(json.dumps({"findings": [f.as_dict() for f in findings]}, indent=2, sort_keys=True) + "\n")
    else:
        if not findings:
            sys.stdout.write("M5 automation contract baseline: OK (clean)\n")
        else:
            sys.stdout.write(f"M5 automation contract baseline: {len(findings)} finding(s)\n")
            for finding in findings:
                subject = f" [{finding.subject}]" if finding.subject else ""
                sys.stdout.write(f"  - {finding.code}{subject}: {finding.message}\n")

    return 1 if findings else 0


if __name__ == "__main__":
    raise SystemExit(main())
