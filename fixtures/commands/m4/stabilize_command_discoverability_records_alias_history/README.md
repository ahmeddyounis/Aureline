# Stabilize command discoverability records, alias history, and query-session privacy

This fixture set exercises the canonical command discoverability support packet
owned by `aureline_commands::stabilize_command_discoverability_records_alias_history`.

`discoverability_support_packet.json` captures the clean protected-line case:

- one canonical discoverability record per protected command, including title,
  summary, examples, alias/deprecation rows, lifecycle, accessibility labels,
  shortcut narration, automation-support posture, and docs/help anchor;
- one local-first query-session policy with explicit retention, clear/disable
  controls, provider classes, and no raw query export; and
- one per-surface parity row covering command palette, keybinding help,
  docs/help, onboarding hints, voice hints, CLI help, and support export.

Verify the checked packet with:

```sh
cargo test -p aureline-commands stabilize_command_discoverability_records_alias_history
```
