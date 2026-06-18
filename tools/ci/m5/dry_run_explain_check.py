#!/usr/bin/env python3
"""M5 dry-run/explain first-consumers gate.

This gate enforces that the checked-in dry-run/explain preview object stays honest
across its first M5 automation consumers. The canonical packet binds all six
first-consumer entrypoints (notebook, task/test/debug, request/API, package,
incident, and the AI assistant) to a seeded preview, keeps every freeze invariant
true, and promotes to ``stable`` with no findings. For every binding the gate
checks that:

- every previewed action declares its side-effect class, and a ``predicted_write``
  action declares at least one write while a ``read_only_inspection`` declares
  none, stays reversible and idempotent, names no mutating destination, and has no
  blocking blocker (so a mutation cannot hide as read-only);
- the aggregate outcome and the safety-label union recomputed from the live
  actions and posture match the frozen ``dry_run_explain_packet_record`` and the
  binding's denormalized fields; and
- the frozen packet projects one step per live action, and each step quotes the
  same canonical verb, reversibility, and derived safety labels as its action.

A dropped entrypoint, an empty preview, an undeclared predicted write, a mutating
action mislabeled read-only, an inconsistent outcome or label projection, or a
violated invariant *blocks stable*. The gate also checks the support export,
CLI/headless view, and compact projection are present and consistent (and that the
attributable run-history rows ride along in the support export); that the
worked-example fixtures (preview export, blocked preview, and the survival
demonstration) exist and carry the expected shape; and that every mutation fixture
except ``dry_run_explain_stable`` reproduces a ``blocks_stable`` state.

The typed Rust consumer mints the same packet, so
``cargo test -p aureline-runtime --test m5_dry_run_explain`` enforces the same
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

ARTIFACT_DIR = Path("artifacts/m5/automation/dry-run-explain")
PACKET_REL = ARTIFACT_DIR / "packet.json"
SUPPORT_EXPORT_REL = ARTIFACT_DIR / "support_export.json"
CLI_HEADLESS_REL = ARTIFACT_DIR / "cli_headless.json"
COMPACT_REL = ARTIFACT_DIR / "compact.txt"

SCHEMA_REL = Path("schemas/automation/dry-run-explain.schema.json")
PACKET_SCHEMA_REL = Path("schemas/automation/recipe-builder.schema.json")
DOC_REL = Path("docs/m5/dry-run-and-explain.md")

FIXTURE_DIR = Path("fixtures/automation/m5/side-effect-preview")

EXPECTED_RECORD_KIND = "m5_dry_run_explain_first_consumers_packet"
EXPECTED_SUPPORT_RECORD_KIND = "m5_dry_run_explain_first_consumers_support_export"
EXPECTED_CLI_RECORD_KIND = "m5_dry_run_explain_first_consumers_cli_headless"
EXPECTED_SCHEMA_VERSION = 1

REQUIRED_ENTRYPOINTS = [
    "notebook",
    "task_test_debug",
    "request_api",
    "package",
    "incident",
    "ai_assistant",
]

REQUIRED_INVARIANTS = [
    "every_entrypoint_binds_a_preview",
    "predicted_writes_are_explicit_before_apply",
    "process_network_remote_actions_are_labeled",
    "trust_and_policy_blockers_are_visible",
    "artifact_destinations_are_named",
    "idempotence_hints_are_present",
    "outcome_and_labels_reuse_the_frozen_vocabulary",
    "preview_survives_export_history_and_support",
]

# Canonical safety-label order, mirroring AutomationSafetyLabelId::ALL.
CANONICAL_LABEL_ORDER = [
    "macro_safe",
    "recipe_safe",
    "headless_safe",
    "ui_only",
    "approval_required",
    "writes_files",
    "runs_process",
    "network_call",
    "remote_mutation",
]

# Each side-effect class projects its frozen safety label (read-only projects none).
SIDE_EFFECT_LABEL = {
    "predicted_write": "writes_files",
    "process_launch": "runs_process",
    "network_call": "network_call",
    "remote_mutation": "remote_mutation",
}

MUTATING_DESTINATION_CLASSES = {
    "workspace_file",
    "device_local_path",
    "remote_target",
    "external_registry",
}

NO_SAFE_PREVIEW = "no_safe_preview"
APPROVAL_REQUIRED_POSTURE = "approval_required_before_apply"

WORKED_EXAMPLE_FIXTURES = {
    "preview_export_roundtrip.json": "dry_run_explain_export_record",
    "blocked_preview_packet.json": "dry_run_explain_packet_record",
    "preview_survives_history_and_support.json": "dry_run_preview_survival_demonstration",
}

MUTATION_FIXTURES = [
    "dry_run_explain_stable.json",
    "missing_entrypoint_blocks_stable.json",
    "predicted_write_not_declared_blocks_stable.json",
    "mutating_action_mislabeled_read_only_blocks_stable.json",
    "outcome_projection_inconsistent_blocks_stable.json",
    "safety_label_projection_inconsistent_blocks_stable.json",
    "invariant_violated_blocks_stable.json",
]

DOC_BACKLINKS = (
    "schemas/automation/dry-run-explain.schema.json",
    "schemas/automation/recipe-builder.schema.json",
    "artifacts/m5/automation/dry-run-explain/",
    "fixtures/automation/m5/side-effect-preview/",
    "tools/ci/m5/dry_run_explain_check.py",
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


def canonical_labels(labels: set[str]) -> list[str]:
    return [label for label in CANONICAL_LABEL_ORDER if label in labels]


def action_requires_approval(action: dict[str, Any]) -> bool:
    for blocker in action.get("trust_policy_blockers") or []:
        if isinstance(blocker, dict) and blocker.get("blocking") and blocker.get(
            "blocker_class"
        ) == "approval_required_gate":
            return True
    return False


def action_has_blocking_denial(action: dict[str, Any]) -> bool:
    for blocker in action.get("trust_policy_blockers") or []:
        if (
            isinstance(blocker, dict)
            and blocker.get("blocking")
            and blocker.get("blocker_class") != "approval_required_gate"
        ):
            return True
    return False


def action_has_blocking_blocker(action: dict[str, Any]) -> bool:
    return any(
        isinstance(b, dict) and b.get("blocking")
        for b in action.get("trust_policy_blockers") or []
    )


def action_projected_labels(action: dict[str, Any]) -> list[str]:
    labels: set[str] = set()
    label = SIDE_EFFECT_LABEL.get(action.get("side_effect_class"))
    if label:
        labels.add(label)
    if action_requires_approval(action):
        labels.add("approval_required")
    return canonical_labels(labels)


def recompute_outcome(binding: dict[str, Any]) -> str:
    actions = binding.get("previewed_actions") or []
    if any(action_has_blocking_denial(a) for a in actions if isinstance(a, dict)):
        return "would_be_denied_at_gate"
    if binding.get("preview_posture_class") == NO_SAFE_PREVIEW:
        return "no_safe_preview"
    needs_approval = binding.get("approval_posture_class") == APPROVAL_REQUIRED_POSTURE or any(
        action_requires_approval(a) for a in actions if isinstance(a, dict)
    )
    if needs_approval:
        return "would_apply_under_approval"
    return "would_apply"


def recompute_labels(binding: dict[str, Any]) -> list[str]:
    labels: set[str] = set(binding.get("portability_labels") or [])
    for action in binding.get("previewed_actions") or []:
        if isinstance(action, dict):
            labels.update(action_projected_labels(action))
    return canonical_labels(labels)


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


def check_action(
    entrypoint: str,
    index: int,
    action: dict[str, Any],
    step: dict[str, Any] | None,
    findings: list[Finding],
) -> None:
    step_id = action.get("step_id", f"#{index}")
    subject = f"{entrypoint}:{step_id}"
    side_effect = action.get("side_effect_class")
    writes = action.get("predicted_writes") or []

    # A predicted write must declare what it writes.
    if side_effect == "predicted_write" and not writes:
        findings.append(
            Finding(
                "predicted_write_not_declared",
                "a predicted-write action declares no write",
                subject=subject,
            )
        )

    # A mutating action must not hide as a read-only inspection.
    if side_effect == "read_only_inspection":
        mutating_dest = any(
            isinstance(d, dict) and d.get("destination_class") in MUTATING_DESTINATION_CLASSES
            for d in action.get("artifact_destinations") or []
        )
        if (
            writes
            or not action.get("reversible")
            or action.get("idempotence_class") != "idempotent"
            or mutating_dest
            or action_has_blocking_blocker(action)
        ):
            findings.append(
                Finding(
                    "mutating_action_mislabeled_read_only",
                    "an action is labeled read-only but declares a side effect",
                    subject=subject,
                )
            )

    # The frozen step must quote the same projection as the live action.
    if step is not None:
        if (
            step.get("step_id") != step_id
            or step.get("canonical_verb") != action.get("canonical_verb")
            or step.get("reversible") != action.get("reversible")
            or step.get("projected_safety_labels") != action_projected_labels(action)
        ):
            findings.append(
                Finding(
                    "outcome_projection_inconsistent",
                    "the projected step disagrees with the action",
                    subject=subject,
                )
            )


def check_binding(binding: dict[str, Any], findings: list[Finding]) -> None:
    entrypoint = binding.get("entrypoint", "<unknown>")
    actions = binding.get("previewed_actions")
    packet_record = binding.get("packet_record")
    if not isinstance(packet_record, dict):
        findings.append(Finding("binding_missing_packet", "a binding has no packet record", subject=entrypoint))
        return
    if not isinstance(actions, list) or not actions:
        findings.append(Finding("entrypoint_preview_empty", "a binding previews no actions", subject=entrypoint))
        return

    steps = packet_record.get("step_explanations") or []
    if len(steps) != len(actions):
        findings.append(
            Finding(
                "outcome_projection_inconsistent",
                "the frozen packet projects a different step count",
                subject=entrypoint,
                detail={"steps": len(steps), "actions": len(actions)},
            )
        )

    recomputed_outcome = recompute_outcome(binding)
    if (
        packet_record.get("dry_run_outcome_class") != recomputed_outcome
        or binding.get("dry_run_outcome_class") != recomputed_outcome
    ):
        findings.append(
            Finding(
                "outcome_projection_inconsistent",
                "the frozen outcome disagrees with the live actions",
                subject=entrypoint,
                detail={
                    "frozen": packet_record.get("dry_run_outcome_class"),
                    "recomputed": recomputed_outcome,
                },
            )
        )

    recomputed_labels = recompute_labels(binding)
    if (
        packet_record.get("aggregate_safety_labels") != recomputed_labels
        or binding.get("aggregate_safety_labels") != recomputed_labels
    ):
        findings.append(
            Finding(
                "safety_label_projection_inconsistent",
                "the frozen safety-label union disagrees with the live actions",
                subject=entrypoint,
                detail={
                    "frozen": packet_record.get("aggregate_safety_labels"),
                    "recomputed": recomputed_labels,
                },
            )
        )

    for index, action in enumerate(actions):
        if not isinstance(action, dict):
            continue
        step = steps[index] if index < len(steps) else None
        check_action(entrypoint, index, action, step, findings)


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
    history = export.get("run_history_rows")
    if not isinstance(history, list) or len(history) != len(REQUIRED_ENTRYPOINTS):
        findings.append(
            Finding("support_run_history_rows", "support export must carry one run-history row per entrypoint")
        )


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
        if file_name == "preview_survives_history_and_support.json":
            if (
                payload.get("outcome_preserved") is not True
                or payload.get("digest_preserved") is not True
                or payload.get("side_effects_preserved") is not True
            ):
                findings.append(
                    Finding(
                        "survival_not_preserved",
                        "survival demonstration must preserve the outcome, digest, and side effects",
                        subject=file_name,
                    )
                )
        if file_name == "blocked_preview_packet.json":
            if payload.get("dry_run_outcome_class") != "would_be_denied_at_gate":
                findings.append(
                    Finding(
                        "blocked_preview_not_denied",
                        "the blocked preview must keep its denying gate visible",
                        subject=file_name,
                    )
                )
        if file_name == "preview_export_roundtrip.json":
            preview = payload.get("preview")
            if not isinstance(preview, dict) or not preview.get("actions"):
                findings.append(
                    Finding("export_missing_actions", "preview export must preserve actions", subject=file_name)
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
        if file_name == "dry_run_explain_stable.json":
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

    for schema_rel in (SCHEMA_REL, PACKET_SCHEMA_REL):
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
            sys.stdout.write("M5 dry-run/explain first consumers: OK (clean)\n")
        else:
            sys.stdout.write(f"M5 dry-run/explain first consumers: {len(findings)} finding(s)\n")
            for finding in findings:
                subject = f" [{finding.subject}]" if finding.subject else ""
                sys.stdout.write(f"  - {finding.code}{subject}: {finding.message}\n")

    return 1 if findings else 0


if __name__ == "__main__":
    raise SystemExit(main())
