# Contextual Teaching / Migration Bridge Fixtures

Worked examples for
[`/docs/ux/contextual_teaching_contract.md`](../../../docs/ux/contextual_teaching_contract.md)
validated by
[`/schemas/ux/teaching_surface.schema.json`](../../../schemas/ux/teaching_surface.schema.json).

The fixtures use only opaque ids, stable vocabulary, short
privacy-safe labels, monotonic placeholder timestamps, and typed policy
context. They do not carry raw URLs, raw absolute paths, raw source
bodies, raw prompt/completion text, or credential material.

## Index

| Fixture | Surface kind | Exercises |
|---|---|---|
| `contextual_tip_symbol_search.json` | `contextual_tip_card` | Nearby tip with command, docs, and file refs plus reversible snooze. |
| `keymap_shimmed_format_document.json` | `migration_bridge_card` | Keymap import in `shimmed` state with native command and rollback. |
| `settings_partial_terminal_shell.json` | `migration_bridge_card` | Settings import in `partial` state with omitted behavior still visible. |
| `snippet_native_user_snippets.json` | `migration_bridge_card` | Snippet import in `native` state with exact native mapping. |
| `task_config_bridge_npm_script.json` | `migration_bridge_card` | Task config import in `bridge` state through a translator bridge. |
| `extension_bridge_unsupported_debugger_protocol.json` | `migration_bridge_card` | Extension bridge in `unsupported` state without hiding the gap. |
| `why_unavailable_share_preview_policy.json` | `why_unavailable_explainer` | Policy-owned unavailable explanation with local fallback. |
| `source_language_fallback_glossary.json` | `source_language_fallback_surface` | Locale fallback preserving canonical docs and message ids. |
