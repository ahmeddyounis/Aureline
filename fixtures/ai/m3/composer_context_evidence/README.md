# AI composer context evidence fixtures

This directory holds reviewer fixtures for the composer/context/evidence beta
support packet. The canonical checked export is
`artifacts/ai/m3/composer_context_evidence_beta_support_export.json`.

The fixture set exercises:

- visible `included`, `pinned`, `omitted`, `stale`, and `trimmed` context rows;
- post-run evidence with approval, rollback, mutation journal, route, spend,
  and tool-call lineage refs;
- a promotable retrieval inspector packet consumed by AI context;
- surface projections that preserve the same operator-truth refs.

Verify and print the canonical export with:

```sh
cargo run -q -p aureline-ai --example dump_composer_context_evidence_beta
```
