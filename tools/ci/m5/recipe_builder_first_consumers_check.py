#!/usr/bin/env python3
"""M5 recipe-builder first-consumers gate.

This gate enforces that the checked-in recipe-builder object stays honest across
its first M5 consumers. The canonical packet binds all six first-consumer
entrypoints (notebook, task/test/debug, request/API, package, incident, and the
AI assistant) to a seeded builder, keeps every freeze invariant true, and
promotes to ``stable`` with no findings. For every binding the gate checks that:

- the builder reuses command truth: its session record carries at least one step
  draft and every step keeps a non-empty command id, command revision, and
  canonical verb;
- the builder emits a declarative manifest (``manifest_target_schema_ref`` is the
  recipe-manifest schema), never an arbitrary script;
- a UI-only step keeps the builder ``blocked`` rather than silently preview-ready;
  and
- copy-CLI and open-docs cite the same command: each ``copy_cli`` line contains
  its step's canonical verb and each ``open_docs`` anchor ends with the slugified
  verb fragment.

A dropped entrypoint, an empty builder, a step missing its command identity, a
non-declarative manifest target, an unblocked UI-only step, broken CLI/docs
parity, or a violated invariant *blocks stable*. The gate also checks the support
export, CLI/headless view, and compact projection are present and consistent;
that the worked-example fixtures (builder export, blocked session, and reorder
demonstration) exist and carry the expected shape; and that every mutation
fixture except ``first_consumers_stable`` reproduces a ``blocks_stable`` state.

The typed Rust consumer mints the same packet, so
``cargo test -p aureline-runtime --test m5_recipe_builder_first_consumers``
enforces the same invariants and that the fixtures and artifacts are bit-for-bit
derivable from the seed.

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

ARTIFACT_DIR = Path("artifacts/m5/automation/recipe-builder-first-consumers")
PACKET_REL = ARTIFACT_DIR / "packet.json"
SUPPORT_EXPORT_REL = ARTIFACT_DIR / "support_export.json"
CLI_HEADLESS_REL = ARTIFACT_DIR / "cli_headless.json"
COMPACT_REL = ARTIFACT_DIR / "compact.txt"

SCHEMA_REL = Path("schemas/automation/recipe-builder-first-consumers.schema.json")
SESSION_SCHEMA_REL = Path("schemas/automation/recipe-builder.schema.json")
DOC_REL = Path("docs/m5/recipe-builder.md")

FIXTURE_DIR = Path("fixtures/automation/m5/recipe-builder")

EXPECTED_RECORD_KIND = "m5_recipe_builder_first_consumers_packet"
EXPECTED_SUPPORT_RECORD_KIND = "m5_recipe_builder_first_consumers_support_export"
EXPECTED_CLI_RECORD_KIND = "m5_recipe_builder_first_consumers_cli_headless"
EXPECTED_SCHEMA_VERSION = 1

RECIPE_MANIFEST_SCHEMA = "schemas/automation/recipe_manifest.schema.json"

REQUIRED_ENTRYPOINTS = [
    "notebook",
    "task_test_debug",
    "request_api",
    "package",
    "incident",
    "ai_assistant",
]

REQUIRED_INVARIANTS = [
    "builder_reuses_command_truth_not_private_form_state",
    "every_entrypoint_binds_the_canonical_builder",
    "steps_are_ordered_and_reorder_preserves_identity",
    "blocked_or_unresolved_steps_remain_visible",
    "copy_cli_and_open_docs_cite_the_same_command",
    "builder_emits_declarative_manifests_only",
    "builder_state_survives_export_import",
]

WORKED_EXAMPLE_FIXTURES = {
    "builder_export_roundtrip.json": "recipe_builder_export_record",
    "blocked_builder_session.json": "recipe_builder_session_record",
    "reorder_preserves_identity.json": "recipe_builder_reorder_demonstration",
}

MUTATION_FIXTURES = [
    "first_consumers_stable.json",
    "missing_entrypoint_blocks_stable.json",
    "non_declarative_manifest_blocks_stable.json",
    "ui_only_step_not_blocked_blocks_stable.json",
    "cli_docs_parity_broken_blocks_stable.json",
    "invariant_violated_blocks_stable.json",
]

DOC_BACKLINKS = (
    "schemas/automation/recipe-builder-first-consumers.schema.json",
    "schemas/automation/recipe-builder.schema.json",
    "artifacts/m5/automation/recipe-builder-first-consumers/",
    "fixtures/automation/m5/recipe-builder/",
    "tools/ci/m5/recipe_builder_first_consumers_check.py",
)


def slugify_verb(verb: str) -> str:
    return verb.replace(".", "-").replace("_", "-")


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


def check_binding(binding: dict[str, Any], findings: list[Finding]) -> None:
    entrypoint = binding.get("entrypoint", "<unknown>")
    session = binding.get("session_record")
    if not isinstance(session, dict):
        findings.append(Finding("binding_missing_session", "a binding has no session record", subject=entrypoint))
        return
    drafts = session.get("step_drafts")
    if not isinstance(drafts, list) or not drafts:
        findings.append(Finding("entrypoint_builder_empty", "a binding builds no steps", subject=entrypoint))
        return

    if session.get("manifest_target_schema_ref") != RECIPE_MANIFEST_SCHEMA:
        findings.append(
            Finding(
                "non_declarative_manifest_target",
                "a builder targets a non-declarative manifest schema",
                subject=entrypoint,
                detail={"manifest_target_schema_ref": session.get("manifest_target_schema_ref")},
            )
        )

    has_ui_only = any(
        isinstance(d, dict) and "ui_only" in (d.get("projected_safety_labels") or []) for d in drafts
    )
    if has_ui_only and binding.get("builder_state_class") != "blocked":
        findings.append(
            Finding(
                "ui_only_step_not_blocked",
                "a UI-only step is present but the builder is not blocked",
                subject=entrypoint,
                detail={"builder_state_class": binding.get("builder_state_class")},
            )
        )

    cli_lines = binding.get("copy_cli_lines") or []
    docs_anchors = binding.get("open_docs_anchors") or []
    if len(cli_lines) != len(drafts) or len(docs_anchors) != len(drafts):
        findings.append(
            Finding(
                "cli_docs_count_mismatch",
                "copy-CLI / open-docs lists are not aligned with the steps",
                subject=entrypoint,
            )
        )

    for index, draft in enumerate(drafts):
        if not isinstance(draft, dict):
            continue
        step_id = draft.get("step_id", f"#{index}")
        verb = draft.get("canonical_verb", "")
        if not draft.get("command_id") or not draft.get("command_revision_ref") or not verb:
            findings.append(
                Finding(
                    "step_missing_command_identity",
                    "a step is missing its command identity",
                    subject=f"{entrypoint}:{step_id}",
                )
            )
            continue
        cli = cli_lines[index] if index < len(cli_lines) else ""
        docs = docs_anchors[index] if index < len(docs_anchors) else ""
        parity = isinstance(cli, str) and verb in cli and isinstance(docs, str) and docs.endswith(
            "#" + slugify_verb(verb)
        )
        if not parity:
            findings.append(
                Finding(
                    "cli_docs_parity_broken",
                    "copy-CLI / open-docs do not cite the step's canonical verb",
                    subject=f"{entrypoint}:{step_id}",
                    detail={"canonical_verb": verb, "copy_cli": cli, "open_docs": docs},
                )
            )


def check_packet(packet: dict[str, Any], findings: list[Finding]) -> None:
    if packet.get("record_kind") != EXPECTED_RECORD_KIND:
        findings.append(Finding("packet_record_kind", f"packet record_kind must be {EXPECTED_RECORD_KIND}"))
    if packet.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(Finding("packet_schema_version", f"packet schema_version must be {EXPECTED_SCHEMA_VERSION}"))

    bindings = packet.get("consumer_bindings")
    if not isinstance(bindings, list):
        findings.append(Finding("bindings_missing", "consumer_bindings must be a list"))
        bindings = []
    seen = [b.get("entrypoint") for b in bindings if isinstance(b, dict)]
    for required in REQUIRED_ENTRYPOINTS:
        if required not in seen:
            findings.append(Finding("missing_entrypoint", "a required entrypoint is absent", subject=required))
    if len(seen) != len(set(seen)):
        findings.append(Finding("duplicate_entrypoint", "an entrypoint is bound more than once"))

    for binding in bindings:
        if isinstance(binding, dict):
            check_binding(binding, findings)

    if packet.get("recipe_manifest_schema_ref") != RECIPE_MANIFEST_SCHEMA:
        findings.append(Finding("packet_manifest_ref", "packet must cite the declarative recipe-manifest schema"))

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
        findings.append(
            Finding("packet_not_stable", f"packet promotion_state must be stable, got {packet.get('promotion_state')}")
        )
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
    rows = export.get("consumer_rows")
    if not isinstance(rows, list) or len(rows) != len(REQUIRED_ENTRYPOINTS):
        findings.append(Finding("support_consumer_rows", "support export must carry one row per entrypoint"))


def check_cli_headless(view: dict[str, Any], findings: list[Finding]) -> None:
    if view.get("record_kind") != EXPECTED_CLI_RECORD_KIND:
        findings.append(Finding("cli_record_kind", f"cli/headless record_kind must be {EXPECTED_CLI_RECORD_KIND}"))
    lines = view.get("consumer_lines")
    if not isinstance(lines, list) or len(lines) != len(REQUIRED_ENTRYPOINTS):
        findings.append(Finding("cli_consumer_lines", "cli/headless view must explain every entrypoint"))


def check_worked_examples(root: Path, findings: list[Finding]) -> None:
    for file_name, record_kind in WORKED_EXAMPLE_FIXTURES.items():
        path = root / FIXTURE_DIR / file_name
        if not path.exists():
            findings.append(Finding("missing_worked_example", "a worked-example fixture is missing", subject=file_name))
            continue
        payload = ensure_dict(load_json(path), str(path))
        if payload.get("record_kind") != record_kind:
            findings.append(
                Finding(
                    "worked_example_record_kind",
                    f"worked-example fixture record_kind must be {record_kind}",
                    subject=file_name,
                )
            )
        if file_name == "reorder_preserves_identity.json":
            if payload.get("orders_match") is not True or payload.get("step_identity_preserved") is not True:
                findings.append(
                    Finding("reorder_not_convergent", "reorder demonstration must show drag and keyboard converge", subject=file_name)
                )
        if file_name == "blocked_builder_session.json":
            if payload.get("builder_state_class") != "blocked":
                findings.append(
                    Finding("blocked_example_not_blocked", "blocked builder session must read as blocked", subject=file_name)
                )
        if file_name == "builder_export_roundtrip.json":
            builder = payload.get("builder")
            if not isinstance(builder, dict) or not builder.get("steps"):
                findings.append(
                    Finding("export_missing_provenance", "builder export must preserve step provenance", subject=file_name)
                )


def check_mutation_fixtures(root: Path, findings: list[Finding]) -> None:
    for file_name in MUTATION_FIXTURES:
        path = root / FIXTURE_DIR / file_name
        if not path.exists():
            findings.append(Finding("missing_mutation_fixture", "a mutation fixture is missing", subject=file_name))
            continue
        payload = ensure_dict(load_json(path), str(path))
        expect = payload.get("expect", {})
        promotion = expect.get("promotion_state")
        if file_name == "first_consumers_stable.json":
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

    for schema_rel in (SCHEMA_REL, SESSION_SCHEMA_REL):
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

    if not (root / COMPACT_REL).exists():
        findings.append(Finding("compact_missing", "compact.txt is missing", subject=str(COMPACT_REL)))

    check_worked_examples(root, findings)
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
            sys.stdout.write("M5 recipe-builder first consumers: OK (clean)\n")
        else:
            sys.stdout.write(f"M5 recipe-builder first consumers: {len(findings)} finding(s)\n")
            for finding in findings:
                subject = f" [{finding.subject}]" if finding.subject else ""
                sys.stdout.write(f"  - {finding.code}{subject}: {finding.message}\n")

    return 1 if findings else 0


if __name__ == "__main__":
    raise SystemExit(main())
