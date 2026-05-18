# Completion, Snippet, and Quick-Fix Beta Fixtures

These fixtures exercise the editor assist beta contracts:

- [`schemas/editor/assist_source.schema.json`](../../../schemas/editor/assist_source.schema.json)
- [`schemas/assist/completion_item.schema.json`](../../../schemas/assist/completion_item.schema.json)
- [`schemas/editor/code_action_preview.schema.json`](../../../schemas/editor/code_action_preview.schema.json)

Coverage:

- deterministic language completion, cached fallback, snippet-origin suggestions, and AI inline assist keep separate source-label classes;
- snippet sessions expose placeholder position, exit routes, IME posture, and cursor recovery state; and
- broad or tainted quick fixes route through preview, approval, rollback, or promotion before apply.
