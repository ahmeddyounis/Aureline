#!/usr/bin/env python3
"""M5 macro-recorder session and replay first-consumers gate.

This gate enforces that the checked-in macro-recorder session and replay object
stays honest across its first M5 automation consumers. The canonical packet binds
all six first-consumer entrypoints (notebook, task/test/debug, request/API, package,
incident, and the AI assistant) to a seeded panel, keeps every freeze invariant
true, and promotes to ``stable`` with no findings. For every session the gate checks
that:

- the replay action recomputed from the session's repository-import state and the
  observed current-replay blockers matches the projected replay resolution, and the
  no-blocker pairing holds (admissible-in-declared-scope pairs with exactly
  ``[no_blocker_present]``; any other class cites a non-no-blocker entry);
- a repository-imported macro always resolves to the imported-blocked replay class
  (repository content never defines an executable macro), and a session that
  captured an unsupported command fails closed;
- a saved macro carries no unsupported command (unsupported commands block save);
- a macro that crosses files carries the cross-scope promotion blocker and is not
  promotable-as-UI-only (promotion to a recipe is explicit);
- the macro is profile-local by default and projects only ``macro_safe`` /
  ``ui_only`` labels; and
- every captured command id and digest is opaque (never a raw value).

A dropped entrypoint, an empty panel, a replay that implies stale context, an
unsupported command that does not block save, a repository-imported macro, a
non-explicit cross-scope promotion, an ambient or managed-only capture, an
inconsistent replay-resolution projection, a non-profile-local default, a raw
secret, or a violated invariant *blocks stable*. The gate also checks the support
export, CLI/headless view, and compact projection are present and consistent (and
that the resolved replay resolutions ride along in the support export); that the
worked-example fixtures (the export round-trip, the cross-scope promotion, the
unsupported-command block, and the fail-closed replay) exist and carry the expected
shape; and that every mutation fixture except ``macro_recorder_stable`` reproduces a
``blocks_stable`` state.

The typed Rust consumer mints the same packet, so
``cargo test -p aureline-runtime --test m5_macro_recorder`` enforces the same
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

ARTIFACT_DIR = Path("artifacts/m5/automation/macro-recorder")
PACKET_REL = ARTIFACT_DIR / "packet.json"
SUPPORT_EXPORT_REL = ARTIFACT_DIR / "support_export.json"
CLI_HEADLESS_REL = ARTIFACT_DIR / "cli_headless.json"
COMPACT_REL = ARTIFACT_DIR / "compact.txt"

SCHEMA_REL = Path("schemas/automation/macro-recorder.schema.json")
SESSION_SCHEMA_REL = Path("schemas/automation/macro-session.schema.json")
DOC_REL = Path("docs/m5/macro-recorder-and-replay.md")

FIXTURE_DIR = Path("fixtures/automation/m5/macro-recorder")

EXPECTED_RECORD_KIND = "m5_macro_recorder_first_consumers_packet"
EXPECTED_SUPPORT_RECORD_KIND = "m5_macro_recorder_first_consumers_support_export"
EXPECTED_CLI_RECORD_KIND = "m5_macro_recorder_first_consumers_cli_headless"
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
    "every_entrypoint_binds_a_session_panel",
    "every_session_declares_target_and_storage_scope",
    "recorded_macros_are_profile_local_by_default",
    "unsupported_commands_are_flagged_and_block_save",
    "replay_fails_closed_when_context_or_scope_no_longer_matches",
    "repository_content_never_defines_an_executable_macro",
    "promotion_to_recipe_is_explicit_when_macro_crosses_scope",
    "macro_sessions_capture_ui_or_editor_state_only",
    "macro_sessions_never_use_the_managed_only_channel",
]

NO_BLOCKER = "no_blocker_present"
RECONCILABLE_BLOCKERS = {"active_context_reconcilable"}

# Canonical MacroReplayBlocker::ALL order, mirroring the Rust enum.
CANONICAL_BLOCKER_ORDER = [
    "no_blocker_present",
    "active_context_reconcilable",
    "target_scope_no_longer_matches",
    "active_document_or_selection_drift",
    "supported_command_set_changed",
    "unsupported_command_captured",
    "profile_scope_mismatch",
    "promotion_required_crosses_scope",
    "kill_switch_engaged",
    "replay_disabled_by_policy",
    "macro_revision_retired",
    "imported_from_repository_content",
]

FAIL_CLOSED_BLOCKERS = {
    blocker
    for blocker in CANONICAL_BLOCKER_ORDER
    if blocker != NO_BLOCKER and blocker not in RECONCILABLE_BLOCKERS
}

BLOCKER_TO_REPLAY = {
    "no_blocker_present": "macro_replay_admissible_in_declared_scope",
    "active_context_reconcilable": "macro_replay_admissible_after_scope_reconciliation",
    "target_scope_no_longer_matches": "macro_replay_blocked_target_scope_mismatch",
    "active_document_or_selection_drift": "macro_replay_blocked_active_context_drift",
    "supported_command_set_changed": "macro_replay_blocked_supported_command_set_changed",
    "unsupported_command_captured": "macro_replay_blocked_unsupported_command_captured",
    "profile_scope_mismatch": "macro_replay_blocked_profile_scope_mismatch",
    "promotion_required_crosses_scope": "macro_replay_blocked_promotion_required_crosses_scope",
    "kill_switch_engaged": "macro_replay_blocked_kill_switch_engaged",
    "replay_disabled_by_policy": "macro_replay_blocked_disabled_by_policy",
    "macro_revision_retired": "macro_replay_blocked_revision_retired",
    "imported_from_repository_content": "macro_replay_blocked_imported_from_repository_content",
}

ADMISSIBLE_REPLAY = {
    "macro_replay_admissible_in_declared_scope",
    "macro_replay_admissible_after_scope_reconciliation",
}
ADMISSIBLE_IN_SCOPE = "macro_replay_admissible_in_declared_scope"
BLOCKED_IMPORTED = "macro_replay_blocked_imported_from_repository_content"

CROSS_SCOPE = {"multi_file_scope", "workspace_scope"}
LOCAL_STORAGE = {"user_scope_local_only", "workspace_scope_local_only"}
MACRO_LABELS = {"macro_safe", "ui_only"}
SAVED_DISPOSITIONS = {"saved_as_profile_local_macro", "saved_and_promoted_to_recipe"}

WORKED_EXAMPLE_FIXTURES = {
    "macro_session_export_roundtrip.json": "macro_session_export_record",
    "cross_scope_macro_requires_promotion.json": "macro_cross_scope_promotion_demonstration",
    "unsupported_command_blocks_save.json": "macro_unsupported_command_demonstration",
    "replay_fails_closed_on_context_mismatch.json": "macro_replay_fail_closed_demonstration",
}

MUTATION_FIXTURES = [
    "macro_recorder_stable.json",
    "missing_entrypoint_blocks_stable.json",
    "replay_implies_stale_context_blocks_stable.json",
    "repository_content_defines_macro_blocks_stable.json",
    "unsupported_command_not_blocked_blocks_stable.json",
    "promotion_not_explicit_blocks_stable.json",
    "profile_local_default_violated_blocks_stable.json",
    "ambient_or_managed_only_capture_blocks_stable.json",
    "replay_resolution_projection_inconsistent_blocks_stable.json",
    "raw_secret_material_in_session_blocks_stable.json",
    "invariant_violated_blocks_stable.json",
]

DOC_BACKLINKS = (
    "schemas/automation/macro-recorder.schema.json",
    "schemas/automation/macro-session.schema.json",
    "artifacts/m5/automation/macro-recorder/",
    "fixtures/automation/m5/macro-recorder/",
    "tools/ci/m5/macro_recorder_check.py",
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


def derive_replay(imported: bool, blockers: list[str]) -> str:
    if imported:
        return BLOCKED_IMPORTED
    fail_closed: str | None = None
    reconcilable: str | None = None
    for candidate in CANONICAL_BLOCKER_ORDER:
        if candidate not in blockers:
            continue
        if candidate in FAIL_CLOSED_BLOCKERS and fail_closed is None:
            fail_closed = candidate
        elif candidate in RECONCILABLE_BLOCKERS and reconcilable is None:
            reconcilable = candidate
    if fail_closed is not None:
        return BLOCKER_TO_REPLAY[fail_closed]
    if reconcilable is not None:
        return BLOCKER_TO_REPLAY[reconcilable]
    return BLOCKER_TO_REPLAY[NO_BLOCKER]


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


def session_has_unsupported(session: dict[str, Any]) -> bool:
    for command in session.get("captured_commands") or []:
        if isinstance(command, dict) and command.get("support_class") != "supported_ui_or_editor_state":
            return True
    return False


def check_session(
    entrypoint: str,
    index: int,
    session: dict[str, Any],
    resolution: dict[str, Any] | None,
    findings: list[Finding],
) -> None:
    session_id = session.get("session_id", f"#{index}")
    subject = f"{entrypoint}:{session_id}"
    imported = bool(session.get("imported_from_repository_content"))
    blockers = [b for b in session.get("current_replay_blockers") or [] if isinstance(b, str)]
    resolved = derive_replay(imported, blockers)
    scope = session.get("declared_target_scope_class")
    disposition = session.get("disposition_class")
    has_unsupported = session_has_unsupported(session)

    # Replay must resolve current context, never implying stale authority.
    has_no_blocker = NO_BLOCKER in blockers
    if resolved == ADMISSIBLE_IN_SCOPE:
        pairing_ok = blockers == [NO_BLOCKER]
    else:
        pairing_ok = not has_no_blocker
    if not pairing_ok:
        findings.append(
            Finding(
                "replay_implies_stale_context",
                "a replay resolution implies stale context",
                subject=subject,
                detail={"resolved": resolved, "blockers": blockers},
            )
        )

    # Repository content never defines an executable macro.
    if imported:
        findings.append(
            Finding(
                "repository_content_defines_macro",
                "a macro was imported from repository content",
                subject=subject,
            )
        )

    # A session that captured an unsupported command must fail closed.
    if has_unsupported and resolved in ADMISSIBLE_REPLAY:
        findings.append(
            Finding(
                "replay_not_fail_closed_on_context_mismatch",
                "an unsupported-command session does not fail closed",
                subject=subject,
            )
        )

    # A saved macro carries no unsupported command.
    if has_unsupported and disposition in SAVED_DISPOSITIONS:
        findings.append(
            Finding(
                "unsupported_command_not_blocked",
                "a saved macro carries an unsupported command",
                subject=subject,
            )
        )

    # Promotion to a recipe must be explicit when a macro crosses scope.
    crosses = scope in CROSS_SCOPE
    blocked_for_promotion = "promotion_required_crosses_scope" in blockers
    affordance_ok = (not crosses) or session.get("promotion_affordance_class") != "not_promotable_ui_only"
    if crosses != blocked_for_promotion or not affordance_ok:
        findings.append(
            Finding(
                "promotion_not_explicit_for_cross_scope",
                "a cross-scope macro lacks an explicit promotion path",
                subject=subject,
            )
        )

    # The macro must be profile-local by default and never repository-defined.
    if imported or session.get("storage_scope_class") not in LOCAL_STORAGE:
        findings.append(
            Finding(
                "profile_local_default_violated",
                "a macro is not profile-local by default",
                subject=subject,
                detail={"storage": session.get("storage_scope_class")},
            )
        )

    # The session must project only macro_safe / ui_only labels.
    labels = session.get("projected_safety_labels") or []
    if not labels or any(label not in MACRO_LABELS for label in labels):
        findings.append(
            Finding(
                "ambient_or_managed_only_capture",
                "a session projects a label outside macro_safe / ui_only",
                subject=subject,
                detail={"labels": labels},
            )
        )

    # The save-or-discard disposition must match the minted manifest.
    mints = disposition in SAVED_DISPOSITIONS
    if mints != (session.get("resulting_macro_manifest_ref") is not None):
        findings.append(
            Finding(
                "replay_resolution_projection_inconsistent",
                "the disposition disagrees with the minted manifest",
                subject=subject,
            )
        )

    # No raw secret may appear in a macro session.
    for command in session.get("captured_commands") or []:
        if not isinstance(command, dict):
            continue
        command_id = command.get("command_id", "")
        digest_hex = (command.get("state_digest") or {}).get("digest_hex", "")
        if not reference_is_opaque(command_id) or not reference_is_opaque(digest_hex):
            findings.append(
                Finding(
                    "raw_secret_material_in_session",
                    "a captured command reference is not opaque",
                    subject=subject,
                )
            )
            break

    # The projected replay resolution must quote the recomputed resolution.
    if resolution is not None:
        if (
            resolution.get("replay_action_class") != resolved
            or resolution.get("admissible") is not (resolved in ADMISSIBLE_REPLAY)
            or resolution.get("declared_target_scope_class") != scope
            or resolution.get("declares_target_scope") is not True
            or resolution.get("refuses_on_context_mismatch") is not True
            or resolution.get("reresolves_supported_command_set") is not True
        ):
            findings.append(
                Finding(
                    "replay_resolution_projection_inconsistent",
                    "the projected replay resolution disagrees with the session",
                    subject=subject,
                    detail={"row": resolution.get("replay_action_class"), "recomputed": resolved},
                )
            )


def check_binding(binding: dict[str, Any], findings: list[Finding]) -> None:
    entrypoint = binding.get("entrypoint", "<unknown>")
    sessions = binding.get("sessions")
    resolutions = binding.get("replay_resolutions")
    if not isinstance(sessions, list) or not sessions:
        findings.append(Finding("entrypoint_panel_empty", "a binding previews no sessions", subject=entrypoint))
        return
    if not isinstance(resolutions, list) or len(resolutions) != len(sessions):
        findings.append(
            Finding(
                "replay_resolution_projection_inconsistent",
                "the panel projects a different replay-resolution count",
                subject=entrypoint,
                detail={
                    "resolutions": len(resolutions) if isinstance(resolutions, list) else None,
                    "sessions": len(sessions),
                },
            )
        )
        resolutions = resolutions if isinstance(resolutions, list) else []

    for index, session in enumerate(sessions):
        if not isinstance(session, dict):
            continue
        resolution = (
            resolutions[index]
            if index < len(resolutions) and isinstance(resolutions[index], dict)
            else None
        )
        check_session(entrypoint, index, session, resolution, findings)


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
    total_sessions = sum(
        len(b.get("sessions") or [])
        for b in packet.get("consumer_bindings") or []
        if isinstance(b, dict)
    )
    resolutions = export.get("replay_resolutions")
    if not isinstance(resolutions, list) or len(resolutions) != total_sessions:
        findings.append(
            Finding("support_replay_resolutions", "support export must carry one replay resolution per session")
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
        if file_name == "macro_session_export_roundtrip.json":
            session = payload.get("session")
            if not isinstance(session, dict) or not session.get("session_id"):
                findings.append(
                    Finding("export_missing_session", "the export must preserve the session and its id", subject=file_name)
                )
            if payload.get("replay_resolution", {}).get("declares_target_scope") is not True:
                findings.append(
                    Finding("export_no_scope_declaration", "the export resolution must declare its target scope", subject=file_name)
                )
        if file_name == "cross_scope_macro_requires_promotion.json":
            if (
                payload.get("crosses_scope") is not True
                or payload.get("replay_admissible") is not False
                or payload.get("replay_fails_closed_pending_promotion") is not True
                or payload.get("replay_and_scope_preserved") is not True
            ):
                findings.append(
                    Finding("cross_scope_not_promoted", "a cross-scope macro must fail closed pending promotion", subject=file_name)
                )
        if file_name == "unsupported_command_blocks_save.json":
            if (
                payload.get("has_unsupported_command") is not True
                or payload.get("save_admissible") is not False
                or payload.get("replay_fails_closed") is not True
                or payload.get("minted_no_manifest") is not True
            ):
                findings.append(
                    Finding("unsupported_not_blocked", "an unsupported command must block save and mint no macro", subject=file_name)
                )
        if file_name == "replay_fails_closed_on_context_mismatch.json":
            if (
                payload.get("admissible") is not False
                or payload.get("fails_closed") is not True
                or payload.get("refuses_on_context_mismatch") is not True
            ):
                findings.append(
                    Finding("replay_not_fail_closed", "replay must fail closed on a context mismatch", subject=file_name)
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
        if file_name == "macro_recorder_stable.json":
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
            sys.stdout.write("M5 macro-recorder first consumers: OK (clean)\n")
        else:
            sys.stdout.write(f"M5 macro-recorder first consumers: {len(findings)} finding(s)\n")
            for finding in findings:
                subject = f" [{finding.subject}]" if finding.subject else ""
                sys.stdout.write(f"  - {finding.code}{subject}: {finding.message}\n")

    return 1 if findings else 0


if __name__ == "__main__":
    raise SystemExit(main())
