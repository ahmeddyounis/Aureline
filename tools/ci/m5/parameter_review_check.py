#!/usr/bin/env python3
"""M5 parameter-review first-consumers gate.

This gate enforces that the checked-in parameter-review object stays honest
across its first M5 automation consumers. The canonical packet binds all six
first-consumer entrypoints (notebook, task/test/debug, request/API, package,
incident, and the AI assistant) to a seeded sheet, keeps every freeze invariant
true, and promotes to ``stable`` with no findings. For every binding the gate
checks that:

- every reviewed parameter is typed and carries an explicit source layer (never
  ``unspecified_generic_control``);
- a secret-bearing value (a ``secret_reference`` field) is held as a reference,
  not a raw value, and carries a secret-bearing redaction class; and a non-secret
  field never smuggles a broker handle;
- a chosen save scope is within the parameter's allowed set; and
- the frozen ``parameter_review_sheet_record`` projection stays aligned with the
  live parameters (same row count, same verdict / inspection-kind / requiredness /
  sensitivity, and the recomputed unresolved-required count).

A dropped entrypoint, an empty sheet, an untyped or ambiguous-source parameter, a
raw secret, a disallowed save scope, an inconsistent projection, or a violated
invariant *blocks stable*. The gate also checks the support export, CLI/headless
view, and compact projection are present and consistent; that the worked-example
fixtures (sheet export, secret-reference sheet, and rerun demonstration) exist and
carry the expected shape; and that every mutation fixture except
``parameter_review_stable`` reproduces a ``blocks_stable`` state.

The typed Rust consumer mints the same packet, so
``cargo test -p aureline-runtime --test m5_parameter_review`` enforces the same
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

ARTIFACT_DIR = Path("artifacts/m5/automation/parameter-review")
PACKET_REL = ARTIFACT_DIR / "packet.json"
SUPPORT_EXPORT_REL = ARTIFACT_DIR / "support_export.json"
CLI_HEADLESS_REL = ARTIFACT_DIR / "cli_headless.json"
COMPACT_REL = ARTIFACT_DIR / "compact.txt"

SCHEMA_REL = Path("schemas/automation/parameter-review.schema.json")
SHEET_SCHEMA_REL = Path("schemas/automation/recipe-builder.schema.json")
DOC_REL = Path("docs/m5/parameter-review-and-secret-references.md")

FIXTURE_DIR = Path("fixtures/automation/m5/parameter-review")

EXPECTED_RECORD_KIND = "m5_parameter_review_first_consumers_packet"
EXPECTED_SUPPORT_RECORD_KIND = "m5_parameter_review_first_consumers_support_export"
EXPECTED_CLI_RECORD_KIND = "m5_parameter_review_first_consumers_cli_headless"
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
    "every_parameter_is_typed",
    "source_layer_is_explicit_for_every_parameter",
    "default_or_override_state_is_visible",
    "secret_values_are_references_not_raw",
    "save_to_scope_is_explicit_and_allowed",
    "verdicts_reuse_the_frozen_vocabulary",
    "provenance_and_redaction_survive_export_import",
]

SECRET_BEARING_REDACTION_CLASSES = {
    "redaction_required_with_secret_broker_handles",
    "signing_evidence_only",
    "operator_only_restricted",
}

# Every source layer maps to a frozen argument-inspection kind, except the
# inadmissible ambiguous control.
SOURCE_LAYER_INSPECTION_KIND = {
    "descriptor_default": "default_from_descriptor_argument_ref",
    "workspace_saved": "typed_argument_slot_ref",
    "user_saved": "typed_argument_slot_ref",
    "recipe_supplied": "automation_recipe_supplied_argument_ref",
    "selection_backed": "selection_backed_argument_ref",
    "focused_context_backed": "focused_context_backed_argument_ref",
    "ai_proposed": "ai_proposed_argument_ref",
    "policy_pinned": "policy_pinned_argument_ref",
    "secret_broker": "credential_handle_argument_ref",
}

WORKED_EXAMPLE_FIXTURES = {
    "sheet_export_roundtrip.json": "parameter_review_export_record",
    "secret_reference_held_sheet.json": "parameter_review_sheet_record",
    "rerun_preserves_provenance.json": "parameter_review_rerun_demonstration",
}

MUTATION_FIXTURES = [
    "parameter_review_stable.json",
    "missing_entrypoint_blocks_stable.json",
    "raw_secret_blocks_stable.json",
    "save_scope_not_allowed_blocks_stable.json",
    "source_layer_unspecified_blocks_stable.json",
    "sheet_projection_inconsistent_blocks_stable.json",
    "invariant_violated_blocks_stable.json",
]

DOC_BACKLINKS = (
    "schemas/automation/parameter-review.schema.json",
    "schemas/automation/recipe-builder.schema.json",
    "artifacts/m5/automation/parameter-review/",
    "fixtures/automation/m5/parameter-review/",
    "tools/ci/m5/parameter_review_check.py",
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


def compute_verdict_class(parameter: dict[str, Any]) -> str:
    """Recompute the derived review verdict, mirroring the typed Rust consumer."""
    validation = parameter.get("validation") or {}
    if validation.get("satisfied") is not True:
        return "blocked"
    value_state = parameter.get("value_state")
    if value_state == "policy_pinned":
        return "policy_pinned"
    if value_state == "awaiting_input":
        return "needs_input"
    if parameter.get("secret_reference") is not None:
        return "sensitive_held_for_review"
    return "resolved"


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


def check_parameter(
    entrypoint: str,
    index: int,
    parameter: dict[str, Any],
    row: dict[str, Any] | None,
    findings: list[Finding],
) -> bool:
    """Validate one reviewed parameter; returns whether it is unresolved-required."""
    name = parameter.get("parameter_name", f"#{index}")
    subject = f"{entrypoint}:{name}"
    field_type = parameter.get("field_type")
    source_layer = parameter.get("source_layer")
    value_state = parameter.get("value_state")
    sensitivity = parameter.get("sensitivity_class")
    secret_reference = parameter.get("secret_reference")
    chosen_scope = parameter.get("chosen_save_scope")
    available_scopes = parameter.get("available_save_scopes") or []

    if not field_type:
        findings.append(Finding("parameter_untyped", "a parameter has no field type", subject=subject))

    if source_layer not in SOURCE_LAYER_INSPECTION_KIND:
        findings.append(
            Finding(
                "source_layer_unspecified",
                "a parameter hides in a generic control with no source layer",
                subject=subject,
                detail={"source_layer": source_layer},
            )
        )

    # Secret-reference posture: a secret value is a reference, never raw.
    is_secret = field_type == "secret_reference"
    if is_secret:
        if secret_reference is None:
            if value_state != "awaiting_input":
                findings.append(
                    Finding(
                        "secret_value_not_referenced",
                        "a secret-bearing value is not held as a reference",
                        subject=subject,
                    )
                )
        else:
            handle = secret_reference.get("broker_handle_ref") if isinstance(secret_reference, dict) else None
            ref_class = secret_reference.get("redaction_class") if isinstance(secret_reference, dict) else None
            if not handle:
                findings.append(
                    Finding("secret_handle_missing", "a secret reference carries no broker handle", subject=subject)
                )
            if ref_class not in SECRET_BEARING_REDACTION_CLASSES or sensitivity not in SECRET_BEARING_REDACTION_CLASSES:
                findings.append(
                    Finding(
                        "secret_value_not_referenced",
                        "a secret reference does not carry a secret-bearing redaction class",
                        subject=subject,
                        detail={"sensitivity_class": sensitivity, "redaction_class": ref_class},
                    )
                )
    elif secret_reference is not None:
        findings.append(
            Finding(
                "secret_value_not_referenced",
                "a non-secret field smuggles a broker handle",
                subject=subject,
            )
        )

    # Save scope must be explicit and allowed.
    if chosen_scope not in available_scopes:
        findings.append(
            Finding(
                "save_scope_not_allowed",
                "a chosen save scope is outside its allowed set",
                subject=subject,
                detail={"chosen_save_scope": chosen_scope, "available_save_scopes": available_scopes},
            )
        )

    # The frozen row must quote the same verdict truth.
    if row is not None:
        expected_kind = SOURCE_LAYER_INSPECTION_KIND.get(source_layer, "typed_argument_slot_ref")
        if (
            row.get("parameter_name") != name
            or row.get("inspection_kind") != expected_kind
            or row.get("verdict_class") != compute_verdict_class(parameter)
            or row.get("required") != parameter.get("required")
            or row.get("sensitivity_class") != sensitivity
        ):
            findings.append(
                Finding(
                    "sheet_projection_inconsistent",
                    "the projected row disagrees with the reviewed parameter",
                    subject=subject,
                )
            )

    required = bool(parameter.get("required"))
    return required and compute_verdict_class(parameter) == "needs_input"


def check_binding(binding: dict[str, Any], findings: list[Finding]) -> None:
    entrypoint = binding.get("entrypoint", "<unknown>")
    parameters = binding.get("reviewed_parameters")
    sheet = binding.get("sheet_record")
    if not isinstance(sheet, dict):
        findings.append(Finding("binding_missing_sheet", "a binding has no sheet record", subject=entrypoint))
        return
    if not isinstance(parameters, list) or not parameters:
        findings.append(Finding("entrypoint_sheet_empty", "a binding reviews no parameters", subject=entrypoint))
        return

    rows = sheet.get("rows") or []
    if len(rows) != len(parameters):
        findings.append(
            Finding(
                "sheet_projection_inconsistent",
                "the frozen sheet projects a different parameter count",
                subject=entrypoint,
                detail={"rows": len(rows), "parameters": len(parameters)},
            )
        )

    unresolved = 0
    for index, parameter in enumerate(parameters):
        if not isinstance(parameter, dict):
            continue
        row = rows[index] if index < len(rows) else None
        if check_parameter(entrypoint, index, parameter, row, findings):
            unresolved += 1

    if sheet.get("unresolved_required_count") != unresolved:
        findings.append(
            Finding(
                "sheet_projection_inconsistent",
                "the sheet unresolved-required count disagrees with the parameters",
                subject=entrypoint,
                detail={"reported": sheet.get("unresolved_required_count"), "recomputed": unresolved},
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
        rows = rows if isinstance(rows, list) else []
    # A support export must never carry a raw value: secret rows expose only the flag.
    for consumer in rows:
        if not isinstance(consumer, dict):
            continue
        for parameter_row in consumer.get("parameter_rows") or []:
            if not isinstance(parameter_row, dict):
                continue
            if parameter_row.get("field_type") == "secret_reference" and not parameter_row.get(
                "held_as_secret_reference"
            ):
                findings.append(
                    Finding(
                        "support_raw_secret",
                        "a support-export secret row is not flagged as a reference",
                        subject=parameter_row.get("parameter_name"),
                    )
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
        if file_name == "rerun_preserves_provenance.json":
            if (
                payload.get("source_layers_preserved") is not True
                or payload.get("redaction_preserved") is not True
                or payload.get("provenance_preserved") is not True
            ):
                findings.append(
                    Finding(
                        "rerun_not_preserving",
                        "rerun demonstration must preserve provenance and redaction",
                        subject=file_name,
                    )
                )
        if file_name == "secret_reference_held_sheet.json":
            rows = payload.get("rows") or []
            if not any(isinstance(r, dict) and r.get("verdict_class") == "sensitive_held_for_review" for r in rows):
                findings.append(
                    Finding(
                        "secret_sheet_not_held",
                        "the secret-reference sheet must hold a value for review",
                        subject=file_name,
                    )
                )
        if file_name == "sheet_export_roundtrip.json":
            builder = payload.get("builder")
            if not isinstance(builder, dict) or not builder.get("parameters"):
                findings.append(
                    Finding("export_missing_provenance", "sheet export must preserve parameters", subject=file_name)
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
        if file_name == "parameter_review_stable.json":
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

    for schema_rel in (SCHEMA_REL, SHEET_SCHEMA_REL):
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
            sys.stdout.write("M5 parameter review first consumers: OK (clean)\n")
        else:
            sys.stdout.write(f"M5 parameter review first consumers: {len(findings)} finding(s)\n")
            for finding in findings:
                subject = f" [{finding.subject}]" if finding.subject else ""
                sys.stdout.write(f"  - {finding.code}{subject}: {finding.message}\n")

    return 1 if findings else 0


if __name__ == "__main__":
    raise SystemExit(main())
