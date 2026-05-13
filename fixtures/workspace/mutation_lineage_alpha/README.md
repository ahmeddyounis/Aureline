# Mutation-Lineage Alpha Fixtures

These fixtures exercise the export-safe projection defined in
[`/schemas/workspace/mutation_journal_alpha.schema.json`](../../../schemas/workspace/mutation_journal_alpha.schema.json).

The packet cites mutation-journal entry or group ids and generated-artifact
lineage refs rather than raw file bodies, raw diffs, raw prompts/responses, or
secret material. It proves the protected formatter, lockfile, build-output,
preview-regeneration, and AI-apply paths can share one support-export-safe
lineage envelope.

## Cases

- [`protected_mutation_lineage_packet.json`](./protected_mutation_lineage_packet.json)
