# Autosave replay case fixtures

These fixtures anchor the autosave journal, guided replay, crash
sentinel, and journal-reset contract in
[`/docs/reliability/autosave_journal_and_guided_replay_contract.md`](../../../docs/reliability/autosave_journal_and_guided_replay_contract.md).

Embedded records use:

- [`/schemas/recovery/autosave_journal_entry.schema.json`](../../../schemas/recovery/autosave_journal_entry.schema.json)
- [`/schemas/recovery/guided_replay_choice.schema.json`](../../../schemas/recovery/guided_replay_choice.schema.json)

Fixture ids, timestamps, object refs, workspace refs, and support refs
are opaque examples. They do not encode raw paths, raw file bodies,
credentials, or planning metadata.

## Index

| Fixture | Coverage |
|---|---|
| [`single_file_crash_restore.yaml`](./single_file_crash_restore.yaml) | Verified single-file dirty-buffer recovery, direct restore, inspect-only fallback, and journal retention. |
| [`multi_file_grouped_replay.yaml`](./multi_file_grouped_replay.yaml) | Multi-file crash group with one member requiring review, selected-member restore, and group support export. |
| [`journal_checksum_failure.yaml`](./journal_checksum_failure.yaml) | Checksum mismatch and truncated tail downgrade direct restore to inspect-only/evidence export. |
| [`repeated_crash_loop_safe_mode.yaml`](./repeated_crash_loop_safe_mode.yaml) | Repeated replay/startup failures stop automatic restore, enter safe mode, and quarantine a suspect extension. |
| [`read_only_generated_target_cases.yaml`](./read_only_generated_target_cases.yaml) | Read-only and generated targets block direct restore while preserving inspect/export and reset separation. |
