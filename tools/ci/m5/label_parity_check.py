#!/usr/bin/env python3
"""M5 automation-label cross-surface parity gate.

This gate enforces that automation safety labels stay consistent wherever a
claimed M5 command is surfaced or exported. The canonical packet binds every
claimed command to one source label set and one projection per surface (command
palette row, recipe builder, macro recorder, docs/help, CLI/headless inspect,
support export, and release/public-truth), lists the full frozen vocabulary, and
promotes to ``stable`` with no findings. For every command row the gate checks
that:

- the command projects its labels to every required surface;
- each surface projects the same stable-id label set as the command source;
- each projected label keeps its canonical stable id token and canonical display
  token (no surface-local synonyms);
- no effect-disclosure (side-effect) label is dropped on any surface; and
- the stable ids are declared to survive localization, export, and downgrade.

A dropped surface, a drifted surface label set, a synonym display token, a
drifted stable id, a dropped side-effect label, a stable id that does not survive
localization/export/downgrade, a label outside the vocabulary, or a violated
invariant *blocks stable*. The gate also checks the vocabulary block matches the
frozen, ordered label set; that the support export and CLI/headless view are
present and consistent; and that every mutation fixture except
``label_parity_stable`` reproduces a ``blocks_stable`` state with the expected
finding kinds.

The typed Rust consumer mints the same packet, so
``cargo test -p aureline-runtime --test m5_label_parity`` enforces the same
invariants and that the fixtures and artifacts are bit-for-bit derivable from the
seed.

Exit codes:

- ``0`` -- gate is clean.
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

ARTIFACT_DIR = Path("artifacts/m5/automation/label-parity")
PACKET_REL = ARTIFACT_DIR / "packet.json"
SUPPORT_EXPORT_REL = ARTIFACT_DIR / "support_export.json"
CLI_HEADLESS_REL = ARTIFACT_DIR / "cli_headless.json"
COMPACT_REL = ARTIFACT_DIR / "compact.txt"

SCHEMA_REL = Path("schemas/automation/automation-labels.schema.json")
BASELINE_SCHEMA_REL = Path("schemas/automation/automation-contract-baseline.schema.json")
DOC_REL = Path("docs/m5/automation-safety-labels.md")

FIXTURE_DIR = Path("fixtures/automation/m5/label-parity")

EXPECTED_RECORD_KIND = "m5_automation_label_parity_packet"
EXPECTED_SUPPORT_RECORD_KIND = "m5_automation_label_parity_support_export"
EXPECTED_CLI_RECORD_KIND = "m5_automation_label_parity_cli_headless"
EXPECTED_SCHEMA_VERSION = 1

# Frozen, ordered controlled-automation-label vocabulary: (stable id, display token, kind).
CANONICAL_LABELS: list[tuple[str, str, str]] = [
    ("macro_safe", "Macro-safe", "admissibility_cue"),
    ("recipe_safe", "Recipe-safe", "admissibility_cue"),
    ("headless_safe", "Headless-safe", "admissibility_cue"),
    ("ui_only", "UI-only", "admissibility_cue"),
    ("approval_required", "Approval required", "admissibility_cue"),
    ("writes_files", "Writes files", "effect_disclosure"),
    ("runs_process", "Runs process", "effect_disclosure"),
    ("network_call", "Network call", "effect_disclosure"),
    ("remote_mutation", "Remote mutation", "effect_disclosure"),
]
DISPLAY_TOKENS = {stable: display for stable, display, _ in CANONICAL_LABELS}
LABEL_KINDS = {stable: kind for stable, _, kind in CANONICAL_LABELS}
VOCABULARY_TOKENS = [stable for stable, _, _ in CANONICAL_LABELS]
EFFECT_LABELS = {stable for stable, _, kind in CANONICAL_LABELS if kind == "effect_disclosure"}

REQUIRED_SURFACES = [
    "command_palette_row",
    "recipe_builder",
    "macro_recorder",
    "docs_help",
    "cli_headless_inspect",
    "support_export",
    "release_public_truth",
]

REQUIRED_INVARIANTS = [
    "all_surfaces_project_from_one_label_source",
    "no_surface_invents_synonyms",
    "effect_disclosure_labels_never_dropped",
    "stable_ids_survive_localization_export_downgrade",
    "vocabulary_is_closed_and_frozen",
    "every_claimed_command_projects_to_every_surface",
]

MUTATION_FIXTURES = [
    "label_parity_stable.json",
    "missing_surface_projection_blocks_stable.json",
    "surface_label_drift_blocks_stable.json",
    "synonym_display_token_blocks_stable.json",
    "effect_disclosure_dropped_blocks_stable.json",
    "stable_id_not_preserved_blocks_stable.json",
    "invariant_violated_blocks_stable.json",
]

DOC_BACKLINKS = (
    "schemas/automation/automation-labels.schema.json",
    "schemas/automation/automation-contract-baseline.schema.json",
    "artifacts/m5/automation/label-parity/",
    "fixtures/automation/m5/label-parity/",
    "tools/ci/m5/label_parity_check.py",
)


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


def projected_tokens(projection: dict[str, Any]) -> list[str]:
    return [
        label.get("stable_id_token")
        for label in projection.get("projected_labels") or []
        if isinstance(label, dict)
    ]


def check_projection(
    verb: str,
    source_labels: set[str],
    projection: dict[str, Any],
    findings: list[Finding],
) -> None:
    surface = projection.get("surface", "<unknown>")
    subject = f"{verb}:{surface}"
    tokens = projected_tokens(projection)
    token_set = set(tokens)

    if token_set != source_labels:
        findings.append(
            Finding(
                "surface_label_set_drift",
                "a surface projects a label set that differs from the command source",
                subject=subject,
                detail={"source": sorted(source_labels), "surface": sorted(token_set)},
            )
        )

    dropped_effect = sorted((source_labels & EFFECT_LABELS) - token_set)
    if dropped_effect:
        findings.append(
            Finding(
                "effect_disclosure_dropped",
                "a surface dropped an effect-disclosure (side-effect) label",
                subject=subject,
                detail={"dropped": dropped_effect},
            )
        )

    for label in projection.get("projected_labels") or []:
        if not isinstance(label, dict):
            continue
        label_id = label.get("label_id")
        stable = label.get("stable_id_token")
        display = label.get("display_token")
        if label_id not in VOCABULARY_TOKENS:
            findings.append(
                Finding("label_outside_vocabulary", "a surface renders a label outside the vocabulary", subject=subject, detail={"label_id": label_id})
            )
            continue
        if stable != label_id:
            findings.append(
                Finding("stable_id_token_drift", "a surface renders a label with a drifted stable id", subject=subject, detail={"label_id": label_id, "stable_id_token": stable})
            )
        if display != DISPLAY_TOKENS.get(label_id):
            findings.append(
                Finding("synonym_display_token", "a surface renders a label with a synonym display token", subject=subject, detail={"label_id": label_id, "display_token": display})
            )
        if label.get("label_kind") != LABEL_KINDS.get(label_id):
            findings.append(
                Finding("label_kind_drift", "a surface renders a label with the wrong kind", subject=subject, detail={"label_id": label_id})
            )

    if not (
        projection.get("preserves_stable_ids_on_localization") is True
        and projection.get("preserves_stable_ids_on_export") is True
        and projection.get("preserves_stable_ids_on_downgrade") is True
    ):
        findings.append(
            Finding("stable_id_not_preserved_across_states", "a surface does not preserve stable ids across states", subject=subject)
        )


def check_command_row(row: dict[str, Any], findings: list[Finding]) -> None:
    verb = row.get("canonical_verb", "<unknown>")
    if not row.get("command_id") or not row.get("command_revision_ref") or not verb:
        findings.append(Finding("command_missing_identity", "a command row is missing its identity", subject=verb))
    source_labels = set(row.get("source_labels") or [])
    projections = {
        p.get("surface"): p
        for p in row.get("surface_projections") or []
        if isinstance(p, dict)
    }
    for surface in REQUIRED_SURFACES:
        projection = projections.get(surface)
        if projection is None:
            findings.append(
                Finding("missing_surface_projection", "a command does not project to a required surface", subject=f"{verb}:{surface}")
            )
            continue
        check_projection(verb, source_labels, projection, findings)


def check_vocabulary(vocabulary: Any, findings: list[Finding]) -> None:
    if not isinstance(vocabulary, list) or len(vocabulary) != len(CANONICAL_LABELS):
        findings.append(Finding("vocabulary_coverage_incomplete", "the vocabulary block must list the full frozen label set"))
        return
    for index, (stable, display, kind) in enumerate(CANONICAL_LABELS):
        row = vocabulary[index]
        if not isinstance(row, dict):
            findings.append(Finding("vocabulary_coverage_incomplete", "a vocabulary row is malformed", subject=stable))
            continue
        if row.get("label_id") != stable or row.get("display_token") != display or row.get("label_kind") != kind:
            findings.append(
                Finding("vocabulary_coverage_incomplete", "a vocabulary row drifted from the frozen set", subject=stable, detail={"row": row})
            )


def check_packet(packet: dict[str, Any], findings: list[Finding]) -> None:
    if packet.get("record_kind") != EXPECTED_RECORD_KIND:
        findings.append(Finding("packet_record_kind", f"packet record_kind must be {EXPECTED_RECORD_KIND}"))
    if packet.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(Finding("packet_schema_version", f"packet schema_version must be {EXPECTED_SCHEMA_VERSION}"))

    check_vocabulary(packet.get("vocabulary"), findings)

    rows = packet.get("command_rows")
    if not isinstance(rows, list) or not rows:
        findings.append(Finding("command_rows_missing", "command_rows must be a non-empty list"))
        rows = []
    seen_verbs = [r.get("canonical_verb") for r in rows if isinstance(r, dict)]
    if len(seen_verbs) != len(set(seen_verbs)):
        findings.append(Finding("duplicate_command", "a command appears more than once"))
    # The union of all command source labels must cover the full vocabulary.
    union: set[str] = set()
    for row in rows:
        if isinstance(row, dict):
            union |= set(row.get("source_labels") or [])
            check_command_row(row, findings)
    missing = [token for token in VOCABULARY_TOKENS if token not in union]
    if missing:
        findings.append(Finding("vocabulary_not_exercised", "no command exercises some labels", detail={"missing": missing}))

    if not packet.get("reused_contract_refs"):
        findings.append(Finding("reused_contract_ref_missing", "the packet cites no reused contract refs"))

    invariants = packet.get("invariants")
    if not isinstance(invariants, dict):
        findings.append(Finding("invariants_missing", "invariants must be an object"))
        invariants = {}
    for name in REQUIRED_INVARIANTS:
        if invariants.get(name) is not True:
            findings.append(Finding("invariant_violated", "a freeze invariant is not true", subject=name))

    if packet.get("promotion_state") != "stable":
        findings.append(Finding("packet_not_stable", f"packet promotion_state must be stable, got {packet.get('promotion_state')}"))
    if packet.get("validation_findings"):
        findings.append(Finding("packet_has_findings", "a stable packet must carry no validation findings"))

    digest = packet.get("packet_digest", "")
    if not isinstance(digest, str) or not digest.startswith("fnv1a64:"):
        findings.append(Finding("packet_digest", "packet packet_digest must be an fnv1a64 digest"))


def check_support_export(export: dict[str, Any], packet: dict[str, Any], findings: list[Finding]) -> None:
    if export.get("record_kind") != EXPECTED_SUPPORT_RECORD_KIND:
        findings.append(Finding("support_record_kind", f"support export record_kind must be {EXPECTED_SUPPORT_RECORD_KIND}"))
    if export.get("packet_id") != packet.get("packet_id"):
        findings.append(Finding("support_packet_id", "support export packet_id must match the packet"))
    if export.get("packet_digest") != packet.get("packet_digest"):
        findings.append(Finding("support_digest", "support export packet_digest must match the packet"))
    rows = export.get("command_rows")
    packet_rows = packet.get("command_rows") or []
    if not isinstance(rows, list) or len(rows) != len(packet_rows):
        findings.append(Finding("support_command_rows", "support export must carry one row per command"))
    tokens = export.get("vocabulary_tokens")
    if not isinstance(tokens, list) or set(tokens) != set(VOCABULARY_TOKENS):
        findings.append(Finding("support_vocabulary_tokens", "support export must carry the full vocabulary tokens"))


def check_cli_headless(view: dict[str, Any], packet: dict[str, Any], findings: list[Finding]) -> None:
    if view.get("record_kind") != EXPECTED_CLI_RECORD_KIND:
        findings.append(Finding("cli_record_kind", f"cli/headless record_kind must be {EXPECTED_CLI_RECORD_KIND}"))
    lines = view.get("command_lines")
    packet_rows = packet.get("command_rows") or []
    if not isinstance(lines, list) or len(lines) != len(packet_rows):
        findings.append(Finding("cli_command_lines", "cli/headless view must explain every command"))


def check_mutation_fixtures(root: Path, findings: list[Finding]) -> None:
    for file_name in MUTATION_FIXTURES:
        path = root / FIXTURE_DIR / file_name
        if not path.exists():
            findings.append(Finding("missing_mutation_fixture", "a mutation fixture is missing", subject=file_name))
            continue
        payload = ensure_dict(load_json(path), str(path))
        expect = payload.get("expect", {})
        promotion = expect.get("promotion_state")
        if file_name == "label_parity_stable.json":
            if promotion != "stable" or expect.get("is_stable") is not True:
                findings.append(Finding("mutation_fixture_not_stable", "stable fixture must promote stable", subject=file_name))
        else:
            if promotion != "blocks_stable" or expect.get("is_stable") is not False:
                findings.append(Finding("mutation_fixture_not_blocking", "a mutation fixture must block stable", subject=file_name))
            if not expect.get("expected_finding_kinds"):
                findings.append(Finding("mutation_fixture_no_findings", "a blocking fixture must list expected finding kinds", subject=file_name))


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

    for schema_rel in (SCHEMA_REL, BASELINE_SCHEMA_REL):
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
    check_cli_headless(cli, packet, findings)

    if not (root / COMPACT_REL).exists():
        findings.append(Finding("compact_missing", "compact.txt is missing", subject=str(COMPACT_REL)))

    check_mutation_fixtures(root, findings)
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
            sys.stdout.write("M5 automation-label parity: OK (clean)\n")
        else:
            sys.stdout.write(f"M5 automation-label parity: {len(findings)} finding(s)\n")
            for finding in findings:
                subject = f" [{finding.subject}]" if finding.subject else ""
                sys.stdout.write(f"  - {finding.code}{subject}: {finding.message}\n")

    return 1 if findings else 0


if __name__ == "__main__":
    raise SystemExit(main())
