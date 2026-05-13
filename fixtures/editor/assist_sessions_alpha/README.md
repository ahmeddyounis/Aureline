# Editor Assist Sessions Alpha Fixtures

These fixtures exercise the editor-owned assist contract in
[`schemas/assist/completion_item.schema.json`](../../../schemas/assist/completion_item.schema.json).

Coverage:

- completion items from language-server, lexical fallback, and snippet sources;
- signature-help source labeling and non-blocking IME-safe placement;
- visible snippet-session state, placeholder traversal, escape handling, and pass-through behavior for unrelated keys; and
- combined assist surface counts for editor, support, and CLI consumers.
