#!/usr/bin/env python3
"""M3 bounded voice-preview and privacy CI gate.

This gate enforces that the checked-in bounded voice-command and
dictation preview surface stays honest. It reads:

- the page fixture at
  ``fixtures/ux/m3/voice_preview_and_privacy/page.json``;
- the support-export fixture at
  ``fixtures/ux/m3/voice_preview_and_privacy/support_export.json``;
- the boundary schemas at
  ``schemas/ux/voice_session_state.schema.json`` and
  ``schemas/ux/voice_command_resolution.schema.json``; and
- (when present) the published markdown at
  ``artifacts/ux/m3/voice_preview_beta.md`` and the companion doc at
  ``docs/ux/m3/voice_preview_beta.md``.

The gate verifies that:

- the page is internally clean: ``page_clean`` is true, the invariant
  manifest is all-true, and no row carries a blocking finding;
- the page exercises both a claimed beta/preview row and a Labs
  unadvertised row, and both command mode and dictation mode;
- every claimed row makes command and dictation modes explicit, is
  keyboard reachable and screen-reader narratable, carries a mic-state
  pill, uses an explicit (non-always-on) default activation, and keeps
  background listening off unless an explicit wake-phrase opt-in exists;
- a mic pill that reports active capture shows the persistent
  capture-active indicator;
- every high-impact resolution keeps ``preview_required`` true and keeps
  every no-bypass guard true; every canonical resolution binds a command
  id; every disabled resolution carries a typed disabled reason;
- every unavailable state offers a keyboard fallback;
- every Labs/unadvertised row stays suppressed (no active capture, no
  spoken resolutions, background listening off);
- the support-export wrapper excludes raw audio/transcript bytes and
  quotes the page id and every row id; and
- the published markdown and companion doc back-link the canonical
  schemas, fixtures, artifact, and this gate.

Exit codes:

- ``0`` -- surface is clean (no findings).
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

PAGE_FIXTURE = "fixtures/ux/m3/voice_preview_and_privacy/page.json"
SUPPORT_FIXTURE = "fixtures/ux/m3/voice_preview_and_privacy/support_export.json"
SESSION_SCHEMA = "schemas/ux/voice_session_state.schema.json"
RESOLUTION_SCHEMA = "schemas/ux/voice_command_resolution.schema.json"
PUBLISHED_MARKDOWN = "artifacts/ux/m3/voice_preview_beta.md"
COMPANION_DOC = "docs/ux/m3/voice_preview_beta.md"
CONTRACT_DOC = "docs/ux/voice_and_dictation_contract.md"

HIGH_IMPACT_SCOPES = {
    "recoverable_durable_mutation",
    "destructive_bulk_mutation",
    "irreversible_publish",
}


@dataclass
class Findings:
    items: list[str] = field(default_factory=list)

    def add(self, message: str) -> None:
        self.items.append(message)

    def ok(self) -> bool:
        return not self.items


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".", help="Repository root.")
    return parser.parse_args()


def load_json(root: Path, rel: str) -> Any:
    path = root / rel
    if not path.exists():
        raise FileNotFoundError(rel)
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def check_resolution(row_id: str, resolution: dict[str, Any], findings: Findings) -> None:
    rid = resolution.get("resolution_id", "<unknown>")
    scope = resolution.get("capability_scope_class")
    if scope in HIGH_IMPACT_SCOPES:
        if not resolution.get("preview_required"):
            findings.add(f"{row_id}/{rid}: high-impact resolution skips preview")
        guards = resolution.get("no_bypass_guards", {})
        for name, value in guards.items():
            if value is not True:
                findings.add(f"{row_id}/{rid}: no-bypass guard {name} is not true")
        if not resolution.get("canonical_command_id"):
            findings.add(f"{row_id}/{rid}: high-impact resolution missing command id")
    if resolution.get("resolution_class") == "resolves_to_canonical_command_id":
        if not resolution.get("canonical_command_id"):
            findings.add(f"{row_id}/{rid}: canonical resolution missing command id")
    if resolution.get("enablement_decision_class") in {
        "disabled_with_reason",
        "hidden_with_reason",
    }:
        if not resolution.get("disabled_reason_code"):
            findings.add(f"{row_id}/{rid}: disabled resolution missing disabled reason")


def check_claimed_row(row: dict[str, Any], findings: Findings) -> None:
    row_id = row.get("row_id", "<unknown>")
    for flag in (
        "command_mode_explicit",
        "dictation_mode_explicit",
        "keyboard_reachable",
        "screen_reader_narratable",
    ):
        if not row.get(flag):
            findings.add(f"{row_id}: claimed row missing {flag}")
    if row.get("mic_pill") is None:
        findings.add(f"{row_id}: claimed row missing mic-state pill")
    activation = row.get("default_activation_class")
    if activation not in {"push_to_talk_held", "push_to_talk_toggle", "manual_command_activation"}:
        findings.add(f"{row_id}: claimed row uses non-explicit default activation {activation}")
    background = row.get("background_listening_state")
    if background == "on_user_opted_in" and activation != "wake_phrase_continuous_user_opted_in":
        findings.add(f"{row_id}: background listening on without wake-phrase opt-in")


def check_labs_row(row: dict[str, Any], findings: Findings) -> None:
    row_id = row.get("row_id", "<unknown>")
    pill = row.get("mic_pill")
    if pill is not None and pill.get("capture_active"):
        findings.add(f"{row_id}: Labs row reports active capture")
    if row.get("command_resolutions"):
        findings.add(f"{row_id}: Labs row exposes spoken-command resolutions")
    if row.get("background_listening_state") == "on_user_opted_in":
        findings.add(f"{row_id}: Labs row enables background listening")


def check_row(row: dict[str, Any], findings: Findings) -> None:
    row_id = row.get("row_id", "<unknown>")
    if row.get("blocking_findings"):
        findings.add(f"{row_id}: row carries blocking findings")
    posture = row.get("claim_posture")
    if posture in {"claimed_beta", "claimed_preview"}:
        check_claimed_row(row, findings)
    elif posture == "labs_unadvertised":
        check_labs_row(row, findings)
    else:
        findings.add(f"{row_id}: unknown claim posture {posture}")

    pill = row.get("mic_pill")
    if pill is not None and pill.get("capture_active"):
        if pill.get("mic_indicator_class") != "persistent_indicator_visible_capture_active":
            findings.add(f"{row_id}: mic indicator hidden during active capture")

    privacy = row.get("provider_privacy_row", {})
    if not privacy.get("provider_or_local_engine_label_ref"):
        findings.add(f"{row_id}: provider/privacy state not disclosed")
    unavailable = privacy.get("unavailable_reason") is not None or row.get("unavailable_banner")
    if unavailable:
        if not privacy.get("keyboard_fallback_available"):
            findings.add(f"{row_id}: unavailable row offers no keyboard fallback")
        if not privacy.get("keyboard_fallback_command_id"):
            findings.add(f"{row_id}: unavailable row missing keyboard fallback command")

    for resolution in row.get("command_resolutions", []):
        check_resolution(row_id, resolution, findings)


def check_page(page: dict[str, Any], findings: Findings) -> None:
    if page.get("record_kind") != "shell_voice_preview_beta_page_record":
        findings.add("page record_kind is not shell_voice_preview_beta_page_record")
    if page.get("shared_contract_ref") != "shell:voice_preview_beta:v1":
        findings.add("page shared_contract_ref mismatch")
    if not page.get("page_clean"):
        findings.add("page_clean is not true")

    invariants = page.get("invariants", {})
    if not invariants or any(value is not True for value in invariants.values()):
        findings.add("invariant manifest is not all-true")

    rows = page.get("rows", [])
    if not rows:
        findings.add("page has no rows")

    postures = {row.get("claim_posture") for row in rows}
    if not ({"claimed_beta", "claimed_preview"} & postures):
        findings.add("page has no claimed beta/preview row")
    if "labs_unadvertised" not in postures:
        findings.add("page has no Labs/unadvertised row")

    modes = {
        row.get("mic_pill", {}).get("voice_mode_class")
        for row in rows
        if row.get("mic_pill")
    }
    if "command_mode_active" not in modes:
        findings.add("page never exercises command mode")
    if "dictation_mode_active" not in modes:
        findings.add("page never exercises dictation mode")

    for row in rows:
        check_row(row, findings)


def check_support_export(
    support: dict[str, Any], page: dict[str, Any], findings: Findings
) -> None:
    if support.get("record_kind") != "shell_voice_preview_beta_support_export_record":
        findings.add("support export record_kind mismatch")
    if support.get("raw_audio_or_transcript_bytes_excluded") is not True:
        findings.add("support export does not exclude raw audio/transcript bytes")
    case_ids = set(support.get("case_ids", []))
    if page.get("page_id") not in case_ids:
        findings.add("support export does not quote the page id")
    for row in page.get("rows", []):
        if row.get("row_id") not in case_ids:
            findings.add(f"support export does not quote row {row.get('row_id')}")


def check_docs(root: Path, findings: Findings) -> None:
    for rel, required in (
        (COMPANION_DOC, True),
        (PUBLISHED_MARKDOWN, False),
    ):
        path = root / rel
        if not path.exists():
            if required:
                findings.add(f"missing companion doc {rel}")
            continue
        body = path.read_text(encoding="utf-8")
        for token in (
            SESSION_SCHEMA,
            RESOLUTION_SCHEMA,
            PAGE_FIXTURE if rel == COMPANION_DOC else "voice_preview",
        ):
            if token not in body:
                findings.add(f"{rel} does not reference {token}")
        if rel == COMPANION_DOC:
            for token in (CONTRACT_DOC, "tools/ci/m3/voice_preview_check.py"):
                if token not in body:
                    findings.add(f"{rel} does not reference {token}")


def main() -> int:
    args = parse_args()
    root = Path(args.repo_root).resolve()
    findings = Findings()

    try:
        page = load_json(root, PAGE_FIXTURE)
        support = load_json(root, SUPPORT_FIXTURE)
        load_json(root, SESSION_SCHEMA)
        load_json(root, RESOLUTION_SCHEMA)
    except FileNotFoundError as missing:
        print(f"error: missing required input file {missing}", file=sys.stderr)
        return 2
    except json.JSONDecodeError as err:
        print(f"error: invalid JSON: {err}", file=sys.stderr)
        return 2

    check_page(page, findings)
    check_support_export(support, page, findings)
    check_docs(root, findings)

    if findings.ok():
        print("voice_preview_check: ok")
        return 0

    for item in findings.items:
        print(f"finding: {item}", file=sys.stderr)
    print(f"voice_preview_check: {len(findings.items)} finding(s)", file=sys.stderr)
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
