# Stabilize command discoverability records, alias/deprecation propagation, and query-session privacy

This lane makes command discoverability a stable product contract instead of a
set of per-surface hints. The canonical runtime owner is
`aureline_commands::stabilize_command_discoverability_records_alias_history`.

The packet derives one governed discoverability record per protected command
from the canonical command registry and binds:

- one stable command identity, title, summary, examples, tags, origin class,
  lifecycle state, alias/deprecation map, replacement route, accessibility
  labels, shortcut narration hints, automation-support posture, and docs/help
  anchor;
- one privacy-bounded query-session policy for command discovery, with local
  history posture, explicit retention, clear/disable controls, provider classes,
  held-modifier intent, and explicit local-only versus governed-sync posture;
- one per-surface parity row proving the command palette, keybinding help,
  docs/help, onboarding hints, voice hints, CLI help, and support export all
  resolve against the same discoverability source instead of private copies.

## Contract

The checked packet is export-safe: refs, state tokens, booleans, and bounded
counts only. Raw query text, raw command arguments, paths, URLs, credentials,
and provider payloads stay outside the support boundary.

Stable-line commands are blocked if they are missing:

- a discoverability record id;
- a current alias/deprecation map;
- at least one docs/help anchor;
- a declared automation-support posture;
- required discoverability surface coverage; or
- a local-first, clearable query-session policy.

## Checked artifacts

- `artifacts/commands/m4/stabilize_command_discoverability_records_alias_history/support_export.json`
- `artifacts/commands/m4/stabilize_command_discoverability_records_alias_history/summary.md`
- `fixtures/commands/m4/stabilize_command_discoverability_records_alias_history/discoverability_support_packet.json`
- `schemas/commands/discoverability-record.schema.json`

## Verification

```sh
cargo test -p aureline-commands stabilize_command_discoverability_records_alias_history
```
