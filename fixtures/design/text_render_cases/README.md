# Text-render and typography contract fixtures

Worked fixtures for the typography and text overflow/copy-honesty
contract frozen in:

- [`/docs/design/typography_text_contract.md`](../../../docs/design/typography_text_contract.md)

and validated by:

- [`/schemas/design/text_role.schema.json`](../../../schemas/design/text_role.schema.json)

These cases exist to keep launch-critical surfaces aligned on:

- which **text role** is in use (display/title/body/supporting/caption/
  code/terminal/dense_metric);
- which **typography scale row** (`type.*`) and **typography role**
  (`type.role.*`) the slot consumes; and
- which overflow behavior and recovery route applies when the visible
  string is truncated or clamped.

Cases are intentionally small and do not embed screenshots, raw assets,
or large payloads. The `example_text` fields are illustrative snippets
used to reason about overflow and bidi behavior; they are not intended
as product copy.

