# Editor Document State Cases

Worked fixtures for
[`/docs/ux/editor_document_state_contract.md`](../../../docs/ux/editor_document_state_contract.md)
using
[`/schemas/editor/document_state_badge.schema.json`](../../../schemas/editor/document_state_badge.schema.json).

- [`recovered_read_only_snapshot.json`](./recovered_read_only_snapshot.json)
  covers a recovered snapshot opened read-only with provenance and
  writable-source actions.
- [`generated_stale_output.json`](./generated_stale_output.json)
  covers stale generated output promoted from preview to a durable tab
  with source, compare, and regenerate actions.
- [`compare_dirty_source.json`](./compare_dirty_source.json)
  covers a compare view whose source side is dirty and whose compare
  basis must not hide unsaved source authority.
