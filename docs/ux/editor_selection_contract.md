# Editor selection, caret, and scope-status contract

This document freezes Aureline's source-editor selection model before
multi-cursor, rename, search, snippets, modal editing, quick fixes, and
refactor previews grow separate vocabularies for the same state.

The editor selection model is not the dense-collection multi-select
model. Dense collections select objects or rows for batch action.
Editors select source ranges and insertion points for text mutation,
navigation, search, assist, and status reporting. Both models share the
global focus and activation principles, but their state names and scope
terms are intentionally separate.

Machine-readable companions:

- [`/schemas/editor/selection_state.schema.json`](../../schemas/editor/selection_state.schema.json)
  defines the boundary record for editor caret, selection, multi-cursor,
  column-selection, IME, scope-status, persistence, and exported status
  state.
- [`/fixtures/editor/selection_cases/`](../../fixtures/editor/selection_cases/)
  contains worked cases for primary selection replace, multi-cursor IME
  narrowing, column and line selection, structural/modal/snippet flows,
  explicit scope widening, and read-only/generated compare surfaces.

This contract composes with, and does not replace:

- [`/docs/ux/editor_anatomy_contract.md`](./editor_anatomy_contract.md)
  for text-viewport ownership, status-layer placement, source-column
  stability, and large-file or degraded editor state.
- [`/docs/editor/refactor_and_replace_transaction_contract.md`](../editor/refactor_and_replace_transaction_contract.md)
  for replace, rename, refactor, imported patch, preview, checkpoint,
  and rollback packets that consume editor-derived scope.
- [`/docs/language/completion_and_inline_hint_contract.md`](../language/completion_and_inline_hint_contract.md)
  for completion, signature help, snippets, and inline assistance that
  must stay honest around multi-cursor and IME state.
- [`/docs/language/diagnostics_and_code_action_contract.md`](../language/diagnostics_and_code_action_contract.md)
  for quick-fix and fix-all review when a code action consumes editor
  selection or widens beyond it.
- [`/docs/ux/modal_editing_status_contract.md`](./modal_editing_status_contract.md)
  and
  [`/docs/commands/sequence_and_modal_discoverability_contract.md`](../commands/sequence_and_modal_discoverability_contract.md)
  for mode, operator, macro, and sequence status that changes selection
  or command scope.
- [`/docs/accessibility/a11y_ime_packet_template.md`](../accessibility/a11y_ime_packet_template.md)
  and [`/docs/i18n/locale_input_readiness.md`](../i18n/locale_input_readiness.md)
  for IME, screen-reader, keyboard, bidi, dead-key, AltGr, and exported
  input evidence.
- [`/docs/ux/selection_and_scope_contract.md`](./selection_and_scope_contract.md)
  for dense-collection row selection. This editor contract must not
  borrow row-selection terms such as loaded rows, selected items, or
  custom identity sets for source text.

If this document conflicts with the PRD, technical architecture,
technical design, UI/UX spec, editor anatomy, refactor transaction,
modal editing, command, language, accessibility, or collection-selection
contracts, the upstream owner wins and this document, schema, and
fixtures must update in the same change.

## Scope

Frozen here:

- canonical names for primary caret, selection anchor, active selection,
  secondary cursors, multi-cursor groups, column selections, line
  selections, structural selections, semantic selections, and IME
  composition ranges;
- scope-status language for cursor count, selection extent, visible
  matches, all matching ranges, selection-derived write scope, and
  replace/refactor/apply scope;
- persistence rules for undo groups, quick fixes, previews, snippet
  insertion, modal commands, compare mode, generated surfaces, read-only
  buffers, and exported support/status records;
- keyboard, pointer, accessibility, and exported-status parity over the
  same state names; and
- explicit widening rules for commands that move from active selection
  to file, visible matches, all matching ranges, or workspace scope.

Out of scope:

- implementing the editor engine, structural-edit engine, modal-editing
  engine, search engine, snippet engine, or renderer;
- final visual styling for selection fills, carets, overlays, or status
  items; and
- defining every language-specific structural-selection behavior.

## Canonical State Names

Every editor view exposes one `editor_selection_state_record`. The
record is window-local view state over shared buffer truth: dirty state,
save authority, and mutation journals are still owned by the editor and
buffer contracts.

| State name | Meaning | Required behavior |
|---|---|---|
| `primary_caret` | The insertion point that owns command focus, caret-announcement position, and primary result placement. | Exactly one exists in an active editor view, even when many carets are visible. |
| `selection_anchor` | The stable endpoint from which a range or structural extension began. | Persists until the selection collapses, the user resets it, the buffer identity changes, or an operation records an explicit rebase. |
| `active_selection` | The range, line set, column rectangle, semantic node, or structural unit currently addressed by selection-scoped commands. | May be empty when the state is caret-only; must name its extent and origin when non-empty. |
| `secondary_cursors` | Additional insertion points or ranges in the same editor view. | Never called multi-select; commands must state whether they affect all carets or only the primary caret. |
| `column_selection` | A rectangular or virtual-column-backed text selection across multiple lines. | Remains distinct from line selection and from multiple independent ranges. |
| `line_selection` | Whole logical lines selected for line-level commands. | Must name whether trailing line endings are included before delete, copy, move, or replace. |
| `structural_selection` | A syntax-backed selection such as enclosing expression, block, string, parameter, or node. | Must disclose when it falls back to text because syntax state is unavailable. |
| `semantic_selection` | A provider-backed symbol, reference family, or language object selection. | Must carry provider freshness and scope truth before rename/refactor/apply can consume it. |
| `ime_composition_range` | Preedit or marked text owned by an active input method. | Has priority over shortcuts, modal operators, snippets, and multi-caret mirroring until it commits or cancels. |

Non-conforming aliases:

- Do not use `multi-select` for editor carets or source ranges.
- Do not call a column selection a generic selection set.
- Do not call an active row, table cell, palette result, or collection
  checkbox an editor selection.
- Do not let a status string say only `12 selected` for editor state; it
  must say `12 carets`, `12 ranges`, `column selection`, `line
  selection`, or the equivalent editor term.

## Position And Range Rules

Internal editor positions are source positions, not protocol positions.
Every exported state may include line and grapheme-column summaries for
humans, but mutation engines must bind through internal buffer positions
or opaque position refs.

Rules:

1. Caret movement, deletion, selection, multi-cursor edits, and range
   extension follow grapheme-aware behavior.
2. Protocol offsets such as UTF-16 LSP positions are translated at the
   protocol boundary and must not become the editor's source of truth.
3. The primary caret is the focus position for keyboard and assistive
   technology. It is not necessarily the first visible caret.
4. A range has an anchor endpoint and an active endpoint. Reversing a
   selection changes direction but must not erase the anchor without a
   reset event.
5. Selection state survives scroll, viewport virtualization, decoration
   churn, semantic refresh, and minimap/overview navigation by buffer
   position, not by rendered pixel or DOM row.
6. A selection rebase caused by edit, undo, redo, external reload,
   compare remap, or provider remap must emit a reason class. Silent
   movement to a different logical target is non-conforming.

## Multi-Cursor And Column Selection

Multi-cursor state is an editor insertion model. It is not collection
multi-select.

Required behavior:

- The status layer exposes cursor count once more than one caret exists.
- Commands state whether they apply to `primary_caret_only`,
  `all_carets`, `all_selected_ranges`, `column_selection`, or
  `line_selection`.
- Grouped text edits created from multi-cursor or column selection form
  one named undo group unless an operation explicitly enters preview or
  review.
- Partial application must name skipped carets or ranges before commit
  when protected, generated, read-only, IME, semantic, or structural
  constraints prevent uniform behavior.
- Column selection must report its line span, start column, end column,
  virtual-space policy, and ragged-line behavior. It may materialize
  per-line ranges for mutation, but the user-facing mode remains
  `column_selection`.
- Line selection must report line count and whether command semantics
  include final line endings. A whole-line delete, duplicate, or move
  may not masquerade as an arbitrary character range.

## IME Composition Rules

IME composition owns text input while active.

Rules:

1. Preedit text, marked text, candidate navigation, dead keys, AltGr,
   emoji input, and platform composition shortcuts are routed to the
   input method before editor shortcuts, modal operators, snippets, or
   macro capture.
2. While composition is active, commands that cannot safely mirror the
   composition across secondary cursors must narrow to
   `primary_caret_only`, block, or wait. They must say which posture was
   chosen.
3. The composition range is rendered in the text viewport and announced
   through accessibility state without committing bytes prematurely.
4. Accepting a completion, snippet placeholder, quick fix, modal
   operator, or refactor while composition is active requires one of:
   commit composition first, cancel composition, apply only at the
   primary caret, or block with a visible explanation.
5. A macro must not record raw composition keystrokes as modal commands.
   It may record the committed text only when the command layer receives
   an explicit text insertion event.

## Scope-Status Vocabulary

The status layer, command palette, accessibility tree, support export,
search/replace preview, quick-fix review, and refactor/apply surfaces
must use the same scope terms.

| Scope term | Meaning | Example status language |
|---|---|---|
| `primary_caret_only` | The next operation affects only the primary caret or its active range. | `Primary caret: line 84, col 17` |
| `active_selection` | The next operation affects the non-empty active selection. | `Selection: 3 lines` |
| `all_carets` | The next operation applies once at every visible/known caret. | `12 carets` |
| `all_selected_ranges` | The next operation applies to multiple non-column ranges. | `4 selected ranges` |
| `column_selection` | The next operation applies to a rectangular selection. | `Column selection: 8 lines, cols 12-20` |
| `line_selection` | The next operation applies to complete logical lines. | `Line selection: 5 lines` |
| `visible_editor_matches` | Search/replace currently targets matches rendered or admitted in the editor viewport. | `18 visible matches` |
| `all_matching_in_file` | Search/replace targets all matches in the current file. | `142 matches in file` |
| `all_matching_in_workspace` | Search/replace targets all matches across the resolved workspace scope. | `2,341 workspace matches` |
| `semantic_scope` | Rename/refactor targets a provider-backed symbol or graph scope. | `Rename preview: current workset graph` |
| `read_only_or_generated` | The active surface can inspect/copy but cannot mutate without a reviewed path. | `Read-only generated file: selection can copy only` |

Count terms are editor-specific:

- `caret_count`
- `selected_range_count`
- `selected_line_count`
- `selected_grapheme_count`
- `visible_match_count`
- `all_matching_count`
- `skipped_range_count`
- `blocked_range_count`

Every count carries truth state: `exact`, `approximate`, `partial`,
`stale`, `provider_limited`, or `unknown`. A status item must not merge
visible matches, all matching ranges, selected ranges, skipped ranges,
and blocked ranges into one number.

## Command Scope And Widening

Selection-derived commands start narrow and may widen only through an
explicit rule.

Allowed starting scopes:

- caret-only command starts at `primary_caret_only`;
- selection command starts at `active_selection`, `column_selection`,
  `line_selection`, or `all_selected_ranges`;
- multi-cursor command starts at `all_carets` only when the command
  declares multi-caret support;
- find/replace starts at active selection when the search control was
  opened from a selection and the user keeps selection scope enabled;
- rename/refactor starts at `semantic_scope` only after a semantic
  selection or provider lookup has proved the target.

Widening rules:

1. A command may widen from `active_selection` to
   `all_matching_in_file` only after the UI says the selected range is
   no longer the full target.
2. A command may widen from file scope to `all_matching_in_workspace`
   only through a preview or review surface that names match count,
   excluded scopes, blocked/protected/generated counts, undo group, and
   checkpoint posture.
3. A quick fix, fix-all, organize imports, format, AI apply, imported
   patch apply, rename, or refactor may not infer wider scope from the
   presence of a selection. The preview or code-action packet must name
   the target scope.
4. Modal operators may multiply scope by count, text object, range, or
   replay only while pending state is visible. If the operation crosses
   files, settings, workspace, run-capable commands, or policy
   boundaries, it routes to review or blocks.
5. Pointer affordances such as gutter clicks, drag selections,
   minimap/overview jumps, and context menus may not silently replace
   a selected range with file or workspace scope.
6. A command that narrows because of IME, read-only, generated,
   compare, protected, or large-file state must say what was skipped or
   blocked before execution when a write would otherwise occur.

Forbidden labels:

- `Replace all` with no scope.
- `Fix all` with no target class.
- `Apply to selection` when the command will also touch imports,
  sibling files, generated outputs, settings, or workspace metadata.
- `All selected` for editor carets; use `all_carets`,
  `all_selected_ranges`, `column_selection`, or `line_selection`.

## Persistence Across Workflows

Selection state is not a file mutation by itself, but many workflows
depend on it.

| Workflow | Required persistence |
|---|---|
| Undo/redo | Text mutations restore the post-undo caret/selection state recorded by the undo group. Selection-only moves may coalesce separately, but grouped multi-cursor or column edits remain attributable. |
| Quick fixes and code actions | Preview records cite the active selection or semantic selection they consumed, then preserve the prior selection for cancel and focus return unless the user accepts a new range. |
| Replace previews | Step-through preview preserves primary caret, active selection, and match scope; workspace replace mints a refactor/replace preview packet rather than mutating directly. |
| Snippet insertion | Snippet state records placeholder index/count, primary placeholder, multi-cursor compatibility, and exit route. Leaving snippet mode restores or intentionally replaces selection state with a visible reason. |
| Modal commands | Pending operators, counts, macro capture, register routes, and text objects must quote the same selection scope used by the command graph. |
| Compare mode | Source/target/base/result roles preserve independent view selection; applying to result or source must name the role and review path. |
| Read-only or generated surfaces | Selection, cursoring, copy, find, and inspect remain available where textual; write commands are blocked or route to a reviewed source/generator path. |
| Collaboration/presence | Remote cursors and follow state are presence, not local editor selection. Shared control may drive local selection only while the control grant says so. |
| Restore/reopen | Session restore may preserve caret, selection, scroll, and snippet/modal state when buffer identity and version are compatible; otherwise it must explain reset or remap. |

## Accessibility And Export Parity

Keyboard, pointer, screen-reader, status-bar, command-palette,
automation, support export, and screenshot evidence must name the same
state.

Required labels:

- primary caret position;
- cursor count when more than one caret exists;
- active selection extent and class;
- anchor when range extension is active or inspectable;
- column-selection or line-selection mode when active;
- IME composition state when active;
- visible/all-matching scope when search or replace is active;
- skipped and blocked ranges when a write cannot apply uniformly;
- read-only/generated/compare posture when it changes command safety;
- scope-widening review state when a command no longer targets only the
  selection.

Assistive technology must be able to determine:

- where keyboard input goes now;
- which editor ranges or carets a command will affect;
- whether a pending command will affect the primary caret, all carets,
  selected ranges, column selection, line selection, current file, or
  workspace;
- how to clear the selection or collapse to the primary caret;
- how to exit IME composition, snippet mode, modal pending state, or
  column selection; and
- where focus returns after preview, quick fix, review, or cancel.

Exported records must avoid raw source text unless a separately reviewed
evidence packet permits it. Default editor selection exports use opaque
buffer ids, opaque position/range refs, line/column summaries, status
labels, count truth, command ids, and policy/trust refs.

## Fixture Acceptance

A seeded editor-selection fixture conforms only when it proves:

- exactly one primary caret exists;
- active selection, selection anchor, secondary cursors, column
  selection, line selection, structural selection, semantic selection,
  and IME composition state use the canonical names above;
- status language distinguishes cursor count, selected ranges, line
  extent, column extent, visible matches, all matching ranges, skipped
  ranges, and blocked ranges;
- commands that widen beyond selection name the wider scope and review
  requirement before execution;
- keyboard, pointer, accessibility, status-bar, command-palette, and
  support-export labels derive from the same state record;
- multi-cursor and column-selection cases do not use dense-collection
  multi-select vocabulary; and
- replace, refactor, quick-fix, snippet, modal, compare, generated, and
  read-only cases can consume the record without inventing new scope
  words.
