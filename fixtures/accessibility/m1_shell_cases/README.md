# Shell Accessibility Tree Cases

Seed fixtures for the shell-facing accessibility bridge that maps core shell
zones, Start Center actions, command palette search, embedded docs/help boundary
chrome, and degraded placeholder tool panels into the accessibility-tree
contract.

| Fixture | Purpose |
|---|---|
| `shell_start_center_placeholders.json` | Start Center action list + docs/help boundary chrome + terminal placeholder downgrade posture. |
| `shell_command_palette_overlay.json` | Command palette overlay with focused searchbox, selected result row, and redacted file-result downgrade posture. |

These fixtures are export-safe: they avoid raw file paths, raw terminal bytes,
credentials, and user identifiers.

