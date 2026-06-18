#!/usr/bin/env python3
"""M5 run-history / evidence-panel first-consumers gate.

This gate enforces that the checked-in run-history / evidence-panel object stays
honest across its first M5 automation consumers. The canonical packet binds all
six first-consumer entrypoints (notebook, task/test/debug, request/API, package,
incident, and the AI assistant) to a seeded panel, keeps every freeze invariant
true, and promotes to ``stable`` with no findings. For every entry the gate checks
that:

- the rerun action recomputed from the entry's automation layer, imported state,
  and observed current-policy blockers matches the projected evidence row, and the
  no-blocker pairing holds (admissible-no-revalidation pairs with exactly
  ``[no_blocker_present]``; any other class cites a non-no-blocker entry);
- an imported row always resolves to the imported-blocked rerun class, and a
  recorded macro never resolves to an extension/external or imported rerun class;
- the open-as-recipe affordance is admissible for the entry's layer (so a
  capability is never laundered into a recipe); and
- every secret reference is an opaque broker handle (never a raw value), and the
  retention/artifact-bundle posture is internally consistent.

A dropped entrypoint, an empty panel, a rerun that implies cached approval, an
imported row that offers rerun, a macro that offers external rerun, a laundered
capability, a raw secret, an inconsistent evidence-row projection, or a violated
invariant *blocks stable*. The gate also checks the support export, CLI/headless
view, and compact projection are present and consistent (and that the attributable
evidence rows ride along in the support export); that the worked-example fixtures
(the export round-trip, the imported row, and the survival demonstration) exist and
carry the expected shape; and that every mutation fixture except
``run_history_stable`` reproduces a ``blocks_stable`` state.

The typed Rust consumer mints the same packet, so
``cargo test -p aureline-runtime --test m5_run_history`` enforces the same
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

ARTIFACT_DIR = Path("artifacts/m5/automation/run-history")
PACKET_REL = ARTIFACT_DIR / "packet.json"
SUPPORT_EXPORT_REL = ARTIFACT_DIR / "support_export.json"
CLI_HEADLESS_REL = ARTIFACT_DIR / "cli_headless.json"
COMPACT_REL = ARTIFACT_DIR / "compact.txt"

SCHEMA_REL = Path("schemas/automation/run-history.schema.json")
ROW_SCHEMA_REL = Path("schemas/automation/run_history_row.schema.json")
DOC_REL = Path("docs/m5/automation-run-history.md")

FIXTURE_DIR = Path("fixtures/automation/m5/run-history-evidence")

EXPECTED_RECORD_KIND = "m5_run_history_first_consumers_packet"
EXPECTED_SUPPORT_RECORD_KIND = "m5_run_history_first_consumers_support_export"
EXPECTED_CLI_RECORD_KIND = "m5_run_history_first_consumers_cli_headless"
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
    "every_entrypoint_binds_a_panel",
    "every_entry_resolves_run_identity_and_layer",
    "rerun_resolves_current_policy_never_cached_approval",
    "current_policy_blockers_are_authoritative",
    "imported_records_never_offer_rerun",
    "macro_rows_never_offer_external_rerun",
    "open_as_recipe_never_launders_capability",
    "raw_secrets_never_appear_in_history",
    "history_reuses_canonical_run_record_and_row_schema",
]

NO_BLOCKER = "no_blocker_present"

REVALIDATION_BLOCKERS = {
    "environment_revalidation_required",
    "fresh_approval_required",
    "kill_switch_engaged",
    "managed_only_channel_unresolved",
}

# Canonical CurrentPolicyBlocker::ALL order, mirroring the Rust enum.
CANONICAL_BLOCKER_ORDER = [
    "no_blocker_present",
    "environment_revalidation_required",
    "fresh_approval_required",
    "kill_switch_engaged",
    "managed_only_channel_unresolved",
    "publisher_revoked",
    "capability_disabled_by_policy",
    "managed_only_template_retired",
    "recipe_revision_retired",
    "replay_window_expired",
    "descriptor_revision_retired",
    "environment_capsule_drift_detected",
    "macro_recording_only",
    "extension_or_external_runner_unavailable",
    "imported_record_no_dispatch_admissible",
]

DENIAL_BLOCKERS = {
    blocker
    for blocker in CANONICAL_BLOCKER_ORDER
    if blocker != NO_BLOCKER and blocker not in REVALIDATION_BLOCKERS
}

BLOCKER_TO_RERUN = {
    "no_blocker_present": "rerun_under_current_policy_admissible_no_revalidation_required",
    "environment_revalidation_required": "rerun_under_current_policy_admissible_after_environment_revalidation",
    "fresh_approval_required": "rerun_under_current_policy_admissible_after_fresh_approval",
    "kill_switch_engaged": "rerun_under_current_policy_admissible_after_kill_switch_clear",
    "managed_only_channel_unresolved": "rerun_under_current_policy_admissible_after_managed_channel_resolved",
    "publisher_revoked": "rerun_under_current_policy_blocked_publisher_revoked",
    "capability_disabled_by_policy": "rerun_under_current_policy_blocked_capability_disabled_by_policy",
    "managed_only_template_retired": "rerun_under_current_policy_blocked_managed_only_template_retired",
    "recipe_revision_retired": "rerun_under_current_policy_blocked_recipe_revision_retired",
    "replay_window_expired": "rerun_under_current_policy_blocked_replay_window_expired",
    "descriptor_revision_retired": "rerun_under_current_policy_blocked_descriptor_revision_retired",
    "environment_capsule_drift_detected": "rerun_under_current_policy_blocked_environment_capsule_drift_detected",
    "macro_recording_only": "rerun_under_current_policy_blocked_macro_recording_only",
    "extension_or_external_runner_unavailable": "rerun_under_current_policy_blocked_extension_or_external_runner_unavailable",
    "imported_record_no_dispatch_admissible": "rerun_under_current_policy_blocked_imported_record",
}

ADMISSIBLE_RERUN = {
    "rerun_under_current_policy_admissible_no_revalidation_required",
    "rerun_under_current_policy_admissible_after_environment_revalidation",
    "rerun_under_current_policy_admissible_after_fresh_approval",
    "rerun_under_current_policy_admissible_after_kill_switch_clear",
    "rerun_under_current_policy_admissible_after_managed_channel_resolved",
}

EXTENSION_OR_IMPORTED_RERUN = {
    "rerun_under_current_policy_blocked_extension_or_external_runner_unavailable",
    "rerun_under_current_policy_blocked_imported_record",
}

BLOCKED_IMPORTED = "rerun_under_current_policy_blocked_imported_record"
ADMISSIBLE_NO_REVALIDATION = "rerun_under_current_policy_admissible_no_revalidation_required"

LAYER_OPEN_AS_RECIPE = {
    "recorded_macro_layer": {
        "open_as_recipe_admissible_macro_promotable_to_declarative_recipe",
        "open_as_recipe_inadmissible_no_declarative_capability_path_admitted",
    },
    "declarative_recipe_layer": {"open_as_recipe_inadmissible_already_declarative_recipe"},
    "headless_safe_run_layer": {"open_as_recipe_inadmissible_already_declarative_recipe"},
    "managed_only_template_layer": {
        "open_as_recipe_inadmissible_already_managed_only_template"
    },
    "extension_or_external_automation_layer": {
        "open_as_recipe_admissible_extension_or_external_authored_as_declarative_recipe",
        "open_as_recipe_inadmissible_no_declarative_capability_path_admitted",
        "open_as_recipe_inadmissible_extension_or_external_authority_required",
    },
}

WINDOWED_RETENTION = {
    "retain_until_workspace_redaction_window",
    "retain_until_organization_audit_window",
    "retain_until_support_export_consumed",
    "retain_until_replay_window_expires",
}

WORKED_EXAMPLE_FIXTURES = {
    "run_history_export_roundtrip.json": "run_history_evidence_export_record",
    "imported_row_blocks_rerun.json": "run_history_evidence_row",
    "rerun_survives_history_and_support.json": "run_history_survival_demonstration",
}

MUTATION_FIXTURES = [
    "run_history_stable.json",
    "missing_entrypoint_blocks_stable.json",
    "rerun_implies_cached_approval_blocks_stable.json",
    "macro_offers_external_rerun_blocks_stable.json",
    "capability_laundered_into_recipe_blocks_stable.json",
    "raw_secret_material_in_history_blocks_stable.json",
    "evidence_row_projection_inconsistent_blocks_stable.json",
    "invariant_violated_blocks_stable.json",
]

DOC_BACKLINKS = (
    "schemas/automation/run-history.schema.json",
    "schemas/automation/run_history_row.schema.json",
    "artifacts/m5/automation/run-history/",
    "fixtures/automation/m5/run-history-evidence/",
    "tools/ci/m5/run_history_check.py",
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


def derive_rerun(imported: bool, blockers: list[str]) -> str:
    if imported:
        return BLOCKED_IMPORTED
    denial: str | None = None
    revalidation: str | None = None
    for candidate in CANONICAL_BLOCKER_ORDER:
        if candidate not in blockers:
            continue
        if candidate in DENIAL_BLOCKERS and denial is None:
            denial = candidate
        elif candidate in REVALIDATION_BLOCKERS and revalidation is None:
            revalidation = candidate
    if denial is not None:
        return BLOCKER_TO_RERUN[denial]
    if revalidation is not None:
        return BLOCKER_TO_RERUN[revalidation]
    return BLOCKER_TO_RERUN[NO_BLOCKER]


def reference_is_opaque(reference: str) -> bool:
    return (
        bool(reference)
        and "raw:" not in reference
        and "://" not in reference
        and not reference.startswith("/")
    )


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


def check_entry(
    entrypoint: str,
    index: int,
    entry: dict[str, Any],
    row: dict[str, Any] | None,
    findings: list[Finding],
) -> None:
    entry_id = entry.get("entry_id", f"#{index}")
    subject = f"{entrypoint}:{entry_id}"
    layer = entry.get("automation_layer")
    imported = bool(entry.get("imported"))
    blockers = [b for b in entry.get("current_policy_blockers") or [] if isinstance(b, str)]
    resolved = derive_rerun(imported, blockers)

    # Rerun must resolve current policy, never implying cached approval.
    has_no_blocker = NO_BLOCKER in blockers
    if resolved == ADMISSIBLE_NO_REVALIDATION:
        pairing_ok = blockers == [NO_BLOCKER]
    else:
        pairing_ok = not has_no_blocker
    if not pairing_ok:
        findings.append(
            Finding(
                "rerun_implies_cached_approval",
                "a rerun resolution implies cached approval",
                subject=subject,
                detail={"resolved": resolved, "blockers": blockers},
            )
        )

    # An imported row never offers a rerun.
    if imported and resolved != BLOCKED_IMPORTED:
        findings.append(
            Finding("imported_row_offers_rerun", "an imported row offers a rerun", subject=subject)
        )

    # A recorded macro never offers extension/external rerun.
    if layer == "recorded_macro_layer" and resolved in EXTENSION_OR_IMPORTED_RERUN:
        findings.append(
            Finding(
                "macro_offers_external_rerun",
                "a recorded macro offers extension/external rerun",
                subject=subject,
            )
        )

    # Open-as-recipe must not launder a capability into a recipe.
    open_as_recipe = entry.get("open_as_recipe_action_class")
    admissible = LAYER_OPEN_AS_RECIPE.get(layer, set())
    if open_as_recipe not in admissible:
        findings.append(
            Finding(
                "capability_laundered_into_recipe",
                "an open-as-recipe affordance is inadmissible for the layer",
                subject=subject,
                detail={"layer": layer, "open_as_recipe": open_as_recipe},
            )
        )

    # No raw secret may appear in a history row.
    for reference in entry.get("secret_reference_refs") or []:
        if not isinstance(reference, str) or not reference_is_opaque(reference):
            findings.append(
                Finding(
                    "raw_secret_material_in_history",
                    "a secret reference is not an opaque broker handle",
                    subject=subject,
                )
            )
            break

    # The retention / artifact-bundle posture must be consistent.
    windowed = entry.get("retention_class") in WINDOWED_RETENTION
    if windowed != (entry.get("retention_window_expires_at") is not None):
        findings.append(
            Finding("retention_posture_inconsistent", "the retention window posture is inconsistent", subject=subject)
        )
    bundle_available = entry.get("artifact_bundle_state") == "artifact_bundle_available"
    if bundle_available != (entry.get("artifact_bundle_ref") is not None):
        findings.append(
            Finding("retention_posture_inconsistent", "the artifact-bundle ref posture is inconsistent", subject=subject)
        )

    # The projected evidence row must quote the recomputed rerun resolution.
    if row is not None:
        if (
            row.get("rerun_action_class") != resolved
            or row.get("rerun_admissible") is not (resolved in ADMISSIBLE_RERUN)
            or row.get("run_identity") != entry.get("run_identity")
            or row.get("automation_layer") != layer
        ):
            findings.append(
                Finding(
                    "evidence_row_projection_inconsistent",
                    "the projected evidence row disagrees with the entry",
                    subject=subject,
                    detail={"row_rerun": row.get("rerun_action_class"), "recomputed": resolved},
                )
            )


def check_binding(binding: dict[str, Any], findings: list[Finding]) -> None:
    entrypoint = binding.get("entrypoint", "<unknown>")
    entries = binding.get("entries")
    rows = binding.get("evidence_rows")
    if not isinstance(entries, list) or not entries:
        findings.append(Finding("entrypoint_panel_empty", "a binding previews no entries", subject=entrypoint))
        return
    if not isinstance(rows, list) or len(rows) != len(entries):
        findings.append(
            Finding(
                "evidence_row_projection_inconsistent",
                "the panel projects a different evidence-row count",
                subject=entrypoint,
                detail={"rows": len(rows) if isinstance(rows, list) else None, "entries": len(entries)},
            )
        )
        rows = rows if isinstance(rows, list) else []

    for index, entry in enumerate(entries):
        if not isinstance(entry, dict):
            continue
        row = rows[index] if index < len(rows) and isinstance(rows[index], dict) else None
        check_entry(entrypoint, index, entry, row, findings)


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
    total_entries = sum(
        len(b.get("entries") or [])
        for b in packet.get("consumer_bindings") or []
        if isinstance(b, dict)
    )
    evidence = export.get("evidence_rows")
    if not isinstance(evidence, list) or len(evidence) != total_entries:
        findings.append(
            Finding("support_evidence_rows", "support export must carry one evidence row per entry")
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
        if file_name == "rerun_survives_history_and_support.json":
            if (
                payload.get("rerun_preserved") is not True
                or payload.get("digest_preserved") is not True
                or payload.get("rerun_resolution_is_fresh") is not True
                or payload.get("identity_and_rerun_preserved") is not True
            ):
                findings.append(
                    Finding(
                        "survival_not_preserved",
                        "survival demonstration must preserve identity and rerun freshly",
                        subject=file_name,
                    )
                )
        if file_name == "imported_row_blocks_rerun.json":
            if payload.get("rerun_action_class") != BLOCKED_IMPORTED or payload.get("imported") is not True:
                findings.append(
                    Finding(
                        "imported_row_not_blocked",
                        "the imported row must block rerun and stay imported",
                        subject=file_name,
                    )
                )
            if payload.get("rerun_admissible") is not False:
                findings.append(
                    Finding(
                        "imported_row_admissible",
                        "the imported row must report rerun is not admissible",
                        subject=file_name,
                    )
                )
        if file_name == "run_history_export_roundtrip.json":
            entry = payload.get("entry")
            if not isinstance(entry, dict) or not entry.get("run_identity"):
                findings.append(
                    Finding("export_missing_entry", "the export must preserve the entry and its identity", subject=file_name)
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
        if file_name == "run_history_stable.json":
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

    for schema_rel in (SCHEMA_REL, ROW_SCHEMA_REL):
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
            sys.stdout.write("M5 run-history first consumers: OK (clean)\n")
        else:
            sys.stdout.write(f"M5 run-history first consumers: {len(findings)} finding(s)\n")
            for finding in findings:
                subject = f" [{finding.subject}]" if finding.subject else ""
                sys.stdout.write(f"  - {finding.code}{subject}: {finding.message}\n")

    return 1 if findings else 0


if __name__ == "__main__":
    raise SystemExit(main())
