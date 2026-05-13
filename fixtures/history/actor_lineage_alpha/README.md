# Local-History Actor-Lineage Alpha Fixtures

These fixtures exercise the export-safe projection defined in
[`/schemas/history/local_history_alpha.schema.json`](../../../schemas/history/local_history_alpha.schema.json).

The packet deliberately carries local-history entry ids, mutation ids,
checkpoint ids, and support refs rather than raw file snapshots or
content-addressed body refs. It proves that protected rows can
distinguish typing, import, Git mutation, formatter, AI apply, review
apply, and restore-checkpoint classes while keeping recovery and support
exports metadata-only by default.

## Cases

- [`protected_actor_lineage_packet.json`](./protected_actor_lineage_packet.json)
