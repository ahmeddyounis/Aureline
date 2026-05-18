# Editor Assist and Quick-Fix Beta Contract

This contract defines the beta boundary for completion, signature help,
snippet sessions, AI inline assist, and quick-fix previews.

Machine-readable companions:

- [`/schemas/editor/assist_source.schema.json`](../../schemas/editor/assist_source.schema.json)
- [`/schemas/assist/completion_item.schema.json`](../../schemas/assist/completion_item.schema.json)
- [`/schemas/editor/code_action_preview.schema.json`](../../schemas/editor/code_action_preview.schema.json)
- [`/fixtures/editor/completion_snippet_and_quickfix_beta/`](../../fixtures/editor/completion_snippet_and_quickfix_beta/)

## Source Labels

Assist providers use a stable `source_label_class`:

| Class | Meaning | Direct-apply posture |
|---|---|---|
| `deterministic_language` | Current language-service or semantic-provider result | allowed only when side effects stay local and undoable |
| `cached_fallback` | Cached, lexical, syntax, or limited fallback | visible downgrade; no hidden semantic claim |
| `snippet_origin` | Built-in, profile, workspace, or extension snippet pack | starts a visible snippet session |
| `ai_inline_assist` | AI-authored inline assist or proposal | visually distinct; never presented as deterministic completion |
| `project_graph` | Graph-backed project truth | scope and freshness remain visible |
| `framework_provider` | Framework, schema, or generated-source provider | generated/protected scope remains visible |
| `tool_adapter` | Formatter, linter, build, test, or other structured tool adapter | side effects follow the adapter action packet |

The same label projection is used by compact UI rows, telemetry tokens,
support exports, and docs references. Raw insert text, model output, logs,
paths, URLs, and secrets are not part of the projection.

## Snippet Sessions

Snippet records expose:

- active placeholder index and total placeholder count;
- next, previous, exit, and cancel command refs;
- multi-cursor compatibility and primary-caret fallback when composition cannot apply to every caret;
- IME posture: `no_composition`, `composition_active_pass_through`, `composition_primary_caret_only`, or `composition_blocked`; and
- cursor posture for rapid movement recovery.

When IME composition is active, snippet traversal does not capture `Tab`.
The key passes through to normal composition handling unless composition has
ended. If multi-cursor composition cannot be coherent, the session must reduce
to one visible primary caret or block with a visible explanation.

## Quick-Fix Preview

Editor quick-fix previews are projected from language code-action records and
add one editor-specific gate: evidence trust.

`tainted_terminal_output`, `tainted_build_log`, `tainted_runtime_log`,
`tainted_imported_diagnostic`, and `unknown_untrusted` cannot direct-apply.
They require a preview or approval path until a user or trusted parser emits a
promotion ref. Multi-file, generated/protected, dependency/configuration, or
policy-scoped changes require preview plus an attributable rollback or grouped
undo route before mutation.
