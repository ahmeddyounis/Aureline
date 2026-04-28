# Editor Inline Assist Cases

These JSON fixtures exercise the inline assist contract in
[`docs/ux/editor_inline_assist_contract.md`](../../../docs/ux/editor_inline_assist_contract.md)
and validate against
[`schemas/editor/inline_assist.schema.json`](../../../schemas/editor/inline_assist.schema.json).

Each fixture is an `inline_assist_case_record` with:

- one or more `inline_assist_element_record` payloads;
- expected precedence, density, and suppression resolution;
- stale, approximate, partial, cached, blocked, or exact truth labels;
- ghost-text attribution and preview distinction; and
- keyboard/accessibility routes for every visible or hidden element.

Coverage:

| Fixture | Primary coverage |
| --- | --- |
| `dense_line_precedence.json` | Dense line with diagnostic, current frame, review, coverage, code lens, inlay hint, AI ghost text, and quick action competition. |
| `stale_semantics_downgraded_hints.json` | Stale semantic graph with downgraded inlay hints and code lenses. |
| `generated_read_only_actions.json` | Generated read-only file that blocks or redirects unsafe inline actions. |
| `partial_confidence_hints.json` | Partial-index and low-confidence hint suppression without hiding coverage/test truth. |
