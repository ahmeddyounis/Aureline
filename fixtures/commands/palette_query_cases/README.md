# Command Palette Query-Session Cases

Worked cases for the command palette query-session contract in
[`docs/commands/palette_query_session_contract.md`](../../../docs/commands/palette_query_session_contract.md).

Each case contains one `palette_query_session` record and, where relevant,
`history_entries` and `recent_query_sets`. The top-level case wrapper is not
itself a boundary schema; records inside the wrapper cite
[`schemas/commands/palette_query_session.schema.json`](../../../schemas/commands/palette_query_session.schema.json).

| Fixture | Purpose |
| --- | --- |
| [`lexical_then_semantic_stream.yaml`](./lexical_then_semantic_stream.yaml) | Recent and lexical command rows appear first, semantic rows stream later, and the held split/open-alt modifier stays stable. |
| [`bounded_local_history_clear.yaml`](./bounded_local_history_clear.yaml) | Local-first recent-query history is bounded, clearable, and deletes raw/hash material according to typed rules. |
| [`privacy_safe_export.yaml`](./privacy_safe_export.yaml) | Support export cites command/query refs and redacted material while raw query text remains local-only or absent. |
