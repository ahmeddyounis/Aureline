# Execution-context cache and terminal-restore fixture cases

Worked cases for
[`/docs/runtime/context_cache_and_terminal_restore_contract.md`](../../../docs/runtime/context_cache_and_terminal_restore_contract.md).

Schemas:

- [`/schemas/runtime/execution_context_cache_entry.schema.json`](../../../schemas/runtime/execution_context_cache_entry.schema.json)
- [`/schemas/runtime/terminal_restore_metadata.schema.json`](../../../schemas/runtime/terminal_restore_metadata.schema.json)

## Index

| Fixture | Schema | Record kind | Coverage |
|---|---|---|---|
| `wrong_interpreter_cache_entry.json` | execution-context cache | `execution_context_cache_entry_record` | rejected Python task cache entry after the toolchain fingerprint changed |
| `wrong_shell_terminal_restore_metadata.json` | terminal restore metadata | `terminal_restore_metadata_record` | restored terminal metadata records a shell-family mismatch and stays inspect-only |
| `wrong_target_cache_comparison.json` | execution-context cache | `execution_context_cache_comparison_record` | cached local target compared with observed remote target and blocked until target selection |
| `stale_env_cache_reset_preview.json` | execution-context cache | `execution_context_cache_reset_preview_record` | stale environment capsule reset preview preserves settings, history, and terminal restore metadata |
| `blocked_restore_flow_terminal_restore_metadata.json` | terminal restore metadata | `terminal_restore_metadata_record` | policy/trust blocked restore remains metadata-only and cannot rerun or replay privileged actions |

The fixture set carries only opaque refs, hashes, counts, class labels,
and reviewable sentences. Raw terminal bodies, command lines,
environment bodies, paths, URLs, clipboard bytes, and secrets are not
present.
