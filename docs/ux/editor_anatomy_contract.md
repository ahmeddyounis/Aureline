# Editor Anatomy Contract

This contract freezes the editor stack before tabs, gutters, source
viewports, inline assists, overlays, and status signals land as separate
implementations. The goal is to keep source text authoritative and
stable while every editor accessory declares where it lives, what it is,
and what it may not imply.

Companion artifacts:

- [`/schemas/ux/editor_layer.schema.json`](../../schemas/ux/editor_layer.schema.json)
  defines the machine-readable layer catalog and editor-stack case
  records.
- [`/fixtures/ux/editor_layer_cases/`](../../fixtures/ux/editor_layer_cases/)
  contains worked cases for canonical layer ownership, stable text
  column behavior, state placement, and large-file degradation.
- [`/docs/ux/editor_document_state_contract.md`](./editor_document_state_contract.md)
  owns the document-state badge vocabulary used by tabs, document
  headers, context rows, status surfaces, compare sheets, preview
  sheets, support bundles, accessibility labels, and docs screenshots.
- [`/schemas/editor/document_state_badge.schema.json`](../../schemas/editor/document_state_badge.schema.json)
  and
  [`/fixtures/editor/document_state_cases/`](../../fixtures/editor/document_state_cases/)
  publish the machine-readable document-state badge contract and worked
  cases.
- [`/docs/ux/editor_gutter_contract.md`](./editor_gutter_contract.md)
  owns detailed gutter lane admission, precedence, hit-target,
  accessibility-label, narrow-width fallback, and no-jitter rules.
- [`/schemas/ux/editor_gutter_lane.schema.json`](../../schemas/ux/editor_gutter_lane.schema.json)
  and [`/fixtures/ux/gutter_cases/`](../../fixtures/ux/gutter_cases/)
  publish the gutter-specific machine-readable contract and worked
  cases.
- [`/docs/ux/editor_viewport_summary_contract.md`](./editor_viewport_summary_contract.md)
  owns detailed minimap, overview-ruler, fold-summary, search-hit tick,
  and diagnostic-summary admission, suppression, and fallback rules.
- [`/schemas/editor/viewport_summary.schema.json`](../../schemas/editor/viewport_summary.schema.json)
  and
  [`/fixtures/editor/viewport_summary_cases/`](../../fixtures/editor/viewport_summary_cases/)
  publish the viewport-summary machine-readable contract and worked
  cases.
- [`/docs/ux/editor_inline_assist_contract.md`](./editor_inline_assist_contract.md)
  owns detailed inline decoration, code-lens, inlay-hint, ghost-text,
  inline-value, inline quick-action, precedence, density, and
  suppression rules.
- [`/schemas/editor/inline_assist.schema.json`](../../schemas/editor/inline_assist.schema.json)
  and
  [`/fixtures/editor/inline_assist_cases/`](../../fixtures/editor/inline_assist_cases/)
  publish the inline-assist machine-readable contract and worked cases.

Normative sources projected here:

- `.t2/docs/Aureline_UI_UX_Spec_Document.md` sections 11.1 through
  11.7 and Appendix ES define editor principles, stacked layer
  ownership, gutter lane order, inline metadata precedence, large-file
  mode, and constrained file-state badges.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` sections 11.5
  and 11.6 define the shared rendering pipeline, decoration engine,
  large-file controller, and editor subsystem outputs.
- `.t2/docs/Aureline_Technical_Design_Document.md` section 7.1.15
  defines orientation-aid behavior and the rule that gutter,
  minimap/overview, and fold markers cannot become a second hidden truth
  model.
- [`/docs/ux/tabs_editor_groups_contract.md`](./tabs_editor_groups_contract.md)
  owns tab and editor-group identity, compare fallback, restored state,
  generated/read-only tab posture, live-preview truth, and overflow.
- [`/docs/ux/editor_document_state_contract.md`](./editor_document_state_contract.md)
  owns the reusable state names, badge classes, canonical recovery
  actions, and screenshot / accessibility labels for dirty, pinned,
  compare, recovered, generated, imported, read-only, mirrored,
  policy-locked, live-preview, conflict, and stale document states.
- [`/docs/ux/decoration_precedence_contract.md`](./decoration_precedence_contract.md)
  owns shared decoration precedence for editor gutter and inline
  projection.
- [`/docs/ux/editor_gutter_contract.md`](./editor_gutter_contract.md)
  owns the detailed gutter lane contract underneath this layer-level
  assignment.
- [`/docs/ux/editor_inline_assist_contract.md`](./editor_inline_assist_contract.md)
  owns the detailed inline-assist contract underneath this layer-level
  assignment.
- [`/docs/ux/editor_selection_contract.md`](./editor_selection_contract.md)
  owns primary caret, active selection, selection anchor, multi-cursor,
  column-selection, line-selection, IME-composition, and editor
  scope-status language projected through the text viewport and status
  layer.
- [`/docs/language/diagnostics_and_code_action_contract.md`](../language/diagnostics_and_code_action_contract.md)
  and
  [`/docs/language/completion_and_inline_hint_contract.md`](../language/completion_and_inline_hint_contract.md)
  own diagnostic, code-action, completion, and inline-hint provenance.
- [`/docs/ux/degraded_mode_pattern.md`](./degraded_mode_pattern.md)
  owns degraded-state slots, preserved-versus-reduced capability
  disclosure, last-failure visibility, and support-export continuity.

If this document conflicts with those upstream sources, the upstream
owner wins and this document, schema, and fixtures must update in the
same change.

## 1. Boundary

The editor stack is a rendering and interaction contract over existing
workspace, buffer, language, review, preview, and shell authority. It
does not own source truth, save authority, provider truth, policy, or
command execution.

Frozen here:

- the seven canonical editor layers and their responsibilities;
- the required content / annotation / action-affordance classification
  for every launch-bearing editor accessory;
- the stable text column rule that prevents transient state from moving
  source text horizontally;
- placement rules for compare, restored, generated, read-only,
  live-preview, degraded, and large-file states; and
- layer downgrade rules for large-file and degraded cases.

Out of scope: final renderer implementation, visual styling, icon
artwork, font metrics, concrete editor widgets, and live command
execution.

## 2. Canonical Layers

Every editor accessory belongs to exactly one canonical layer. A feature
may project summary state into another layer only through the
state-placement rules below; it may not claim ownership in two layers.

| Layer | Owns | Must never own |
| --- | --- | --- |
| `document_header` | Tab-facing document identity, dirty state, pin state, compare role, restored/recovered state, and top-level constrained-editing badges. | Source bytes, save authority, policy truth, or the only visible explanation for conflict/read-only state. |
| `context_layer` | Breadcrumbs, root identity, symbol ancestry, source-of-truth relation, generated/managed/read-only reason strips, and immediate editor context. | Replace the document title as the sole identity source or hide state that changes write safety. |
| `gutter` | Line numbers, breakpoints, fold controls, diagnostic/change lanes, current-line cues, and optional overview/minimap marker projections. | Become a dense toolbar, shift the text column per line, or make hover-only actions the only route to core workflows. |
| `text_viewport` | Rendered source text, selections, carets, multi-cursor/column-selection state, and inline composition/preedit text. | Move horizontally because diagnostics, badges, hints, overlays, or status items appear. |
| `inline_assist_layer` | Code lenses, inlay hints, inline quick actions, ghost text, and other inline metadata that helps interpretation or proposes a bounded action. | Obscure source text, impersonate committed content, consume typing budget, or silently apply broader edits. |
| `transient_knowledge_layer` | Hover cards, peek, docs, signature help, parameter help, and short-lived contextual knowledge surfaces anchored to source. | Trap focus, become the only route to critical instructions, or rewrite editor identity/state while open. |
| `status_layer` | Encoding, line endings, language mode, selection/cursor stats, semantic/execution mode, degraded capability, and compact ambient status. | Duplicate high-urgency blockers that require stronger treatment or become the only visible constrained-state disclosure. |

Layer order is conceptual, not a z-index license. Any rendered z-index
or compositor band must still respect the ownership table above.

## 3. Accessory Role Classes

Every accessory declares one of three role classes:

| Role | Meaning | Must not do |
| --- | --- | --- |
| `content` | The user is seeing source text, an active text selection/caret, or composition state that belongs in the text editing stream. | Masquerade as generated assistance, stale provider output, or read-only annotation. |
| `annotation` | The accessory explains state, provenance, risk, freshness, position, or advisory metadata without directly invoking a command. | Look like committed source, hide stronger state, or become the only path to an action. |
| `action_affordance` | The accessory can invoke a command, review surface, toggle, navigation, or apply path. | Appear as ordinary source text or run without command identity, keyboard access, and consequence disclosure. |

An accessory that both explains and acts chooses `action_affordance` and
must carry the annotation in its accessible name, tooltip/focus detail,
or adjacent text. For example, a breakpoint marker toggles a breakpoint
and is therefore an action affordance; a diagnostic marker is an
annotation unless the marker itself is the command target.

## 4. Accessory Assignment Catalog

The catalog below is the minimum launch-bearing set. Additional
accessories must extend the same schema vocabulary before broad
implementation.

| Accessory | Layer | Role | Contract |
| --- | --- | --- | --- |
| Tab label, dirty marker, pin marker | `document_header` | `annotation` | Show document identity and working-set state without owning buffer truth. |
| Compare role marker | `document_header` | `annotation` | Name source/target/base/result role in text or structure, not icon alone. |
| Restored/recovered marker | `document_header` | `annotation` | Name restore posture and keep safe restore actions reachable. |
| Header close/unpin controls | `document_header` | `action_affordance` | Route through the editor-group command contract and preserve dirty/read-only meaning. |
| Breadcrumb path, root identity, symbol ancestry | `context_layer` | `annotation` | Preserve navigation context and source-of-truth relation. |
| Generated/source relation strip | `context_layer` | `annotation` | Name canonical source, generator, or lineage relation before write actions appear. |
| Read-only/degraded reason strip | `context_layer` | `annotation` | Name why writes or semantics are narrowed and what still works. |
| Line numbers and current-line cue | `gutter` | `annotation` | Orient the user without changing per-line text start. |
| Breakpoint/debug stop marker | `gutter` | `action_affordance` | Provide reliable hit targets and keyboard commands. |
| Fold control | `gutter` | `action_affordance` | Collapse structure without hiding diagnostics/conflicts/trust warnings. |
| Diagnostic/change marker | `gutter` | `annotation` | Follow precedence and remain inspectable when lanes collapse. |
| Supplemental marker: blame, coverage, tests, collaboration | `gutter` | `annotation` | Collapse before diagnostics, breakpoints, or text-column stability are harmed. |
| Overview ruler/minimap marker projection | `gutter` | `annotation` | Mirror gutter/decoration meaning; never become the sole critical cue. |
| Source text | `text_viewport` | `content` | Render the source or current buffer truth exactly for the admitted representation. |
| Selection, caret, multi-cursor, column-selection indicator | `text_viewport` | `content` | Preserve editing semantics, focus, and grouped undo meaning. |
| IME marked text / composition | `text_viewport` | `content` | Stay anchored to text without committing bytes prematurely. |
| Code lens | `inline_assist_layer` | `action_affordance` | Show command/source and suppress before higher-priority state. |
| Inlay hint | `inline_assist_layer` | `annotation` | Label as hint metadata and never impersonate source. |
| Inline quick action | `inline_assist_layer` | `action_affordance` | Name preview/apply consequence and never bypass review rules. |
| Ghost text | `inline_assist_layer` | `annotation` | Render as advisory proposal with accept/reject routes outside source truth. |
| Hover card and docs popup | `transient_knowledge_layer` | `annotation` | Preserve provider/freshness and be dismissible without losing context. |
| Peek panel | `transient_knowledge_layer` | `action_affordance` | Preserve source anchor, focus return, and keyboard navigation. |
| Signature help / parameter help | `transient_knowledge_layer` | `annotation` | Keep active parameter visible and degrade stale provider data honestly. |
| Encoding, line endings, language mode | `status_layer` | `annotation` | Report buffer/display state without conflicting with header/context cues. |
| Selection/cursor stats | `status_layer` | `annotation` | Report scope without stealing focus. |
| Semantic/execution/degraded mode item | `status_layer` | `annotation` | Name current capability/freshness and link to details. |

## 5. Stable Text Column

The stable text column is the x-position where source text starts for a
given editor viewport after planned chrome reservations are applied. It
is part of the typing contract.

Rules:

1. Diagnostics, hints, code lenses, ghost text, hover/peek surfaces,
   generated/read-only badges, live-preview state, degraded-state
   labels, status changes, typing, cursoring, and selection changes
   MUST NOT move the source text column.
2. Gutter lanes reserve width at the viewport level. A marker appearing
   on one line must not change that line's text start independently of
   other lines.
3. Line-number width may change only when the document crosses a
   configured digit-reserve threshold or an explicit renderer policy
   says the reserve changed. It may not bounce per visible range.
4. Entering large-file, read-only, generated, restored, live-preview, or
   degraded state may narrow accessories, disable layers, or show
   reason strips, but those state transitions must preserve the source
   text column unless accompanied by an explicit user layout change,
   font/zoom/density change, viewport resize, or line-number reserve
   threshold change.
5. Inline assistance that cannot fit without moving source text must
   suppress, wrap into a non-source lane, or move to a transient detail
   surface. It must not push committed source horizontally.
6. Any permitted text-column recompute must emit the reason class so
   renderer tests, accessibility review, and support export can
   distinguish intentional layout changes from jitter.

Forbidden behaviors:

- adding a diagnostic badge that increases left padding for only the
  affected lines;
- toggling inlay hints that changes the start x-position of source text;
- placing a hover, signature-help, or peek surface in normal flow;
- resizing the gutter every time a breakpoint, fold, blame, or coverage
  marker appears; and
- hiding text-column motion behind animation during typing.

## 6. Required State Placement

State that changes editing, review, restore, or source-of-truth meaning
must surface in the editor stack. Icon-only state is non-conforming.

| State | Required layers | Allowed additional layers | Forbidden placement |
| --- | --- | --- | --- |
| `compare` | `document_header`, `context_layer` | `status_layer`, `transient_knowledge_layer` | Tiny diff icon with no source/target/base/result role. |
| `restored` | `document_header`, `context_layer` | `status_layer` | Only a toast or restore log after the editor opens. |
| `generated` | `document_header`, `context_layer` | `gutter`, `status_layer`, `transient_knowledge_layer` | Styling generated text like ordinary editable source with no lineage/source pointer. |
| `read_only` | `document_header`, `context_layer` | `status_layer`, `command_palette` | Disabled write commands with no reason. |
| `live_preview` | `document_header`, `context_layer` | `status_layer`, `transient_knowledge_layer` | Green dot or preview badge that implies editable source or live readiness after restore. |
| `degraded` | `context_layer`, `status_layer` | `document_header`, `gutter`, `transient_knowledge_layer` | Generic `Service degraded` label that does not name the narrowed capability. |
| `large_file` | `document_header`, `context_layer`, `status_layer` | `gutter`, `inline_assist_layer`, `transient_knowledge_layer` | Silently disabling semantic features or pretending full diagnostics were computed. |

The document header carries compact state because tabs and restored
working-set routes need it. The context layer carries the explanation
because the user must not infer safety from icons. The status layer
reports compact capability/freshness details, but it cannot be the only
place where constrained editing is disclosed.

## 7. Large-File and Degraded Downgrades

Large-file and degraded cases narrow the stack by layer. They must name
what remains available, what changed, and where the user can inspect the
reason.

| Layer | Large-file / degraded behavior | Disclosure |
| --- | --- | --- |
| `document_header` | Remains visible. Adds compact `Large file`, `Read-only`, `Generated`, `Recovered`, or `Degraded` labels when those facts affect editing or restore. | Header label plus full title/overflow detail. |
| `context_layer` | Remains visible. Shows reason strip with trigger, source-of-truth relation, preserved capabilities, and safe next action. | Breadcrumb/focus row, command palette details, support export. |
| `gutter` | Line numbers remain where feasible. Breakpoint, diagnostics, fold, blame, coverage, test, and collaboration lanes may narrow, aggregate, or hide by precedence. | Gutter detail command names hidden lanes and suppression reasons. |
| `text_viewport` | Source text, scroll, selection, copy, and cursoring remain stable where the representation is textual. Editing may be constrained or read-only. | Source column remains stable; read-only/constrained state appears in header/context/status. |
| `inline_assist_layer` | Code lenses, inlay hints, ghost text, and inline quick actions suppress first under large-file, low-resource, high-zoom, read-only, generated, or stale semantic state. | Suppression reasons are inspectable and exported. |
| `transient_knowledge_layer` | Hovers, peek, docs, and signature help downgrade to text-only, cached, stale, or unavailable states before blocking typing. | Transient surface shows provider/freshness and an immediate fallback route. |
| `status_layer` | Remains visible unless the entire shell is in a recovery mode that replaces it with a stronger banner. Prioritizes degraded/read-only/large-file capability before ambient metadata. | Status item has keyboard-reachable details and support-export fields. |

Layer disappearance is allowed only when a replacement route exists and
the replacement names the hidden capability. For example, hiding blame
markers in a large file is acceptable if the gutter detail or status
detail says blame is hidden for large-file budget reasons.

## 8. Fixture and Schema Requirements

Records using
[`/schemas/ux/editor_layer.schema.json`](../../schemas/ux/editor_layer.schema.json)
must satisfy these invariants:

1. Every accessory assignment has exactly one owning layer and one role
   class.
2. Every non-content accessory that renders near text carries a
   `must_not_impersonate_content` or equivalent forbidden-outcome proof.
3. Stable text-column records set `column_shift_px = 0` for transient
   diagnostics, hints, overlays, status changes, typing, cursoring,
   compare/restored/generated/read-only/live-preview/degraded badges,
   and large-file entry unless an explicit permitted reflow reason is
   recorded.
4. Compare, restored, generated, read-only, live-preview, degraded, and
   large-file states name their required layers and avoid icon-only
   placement.
5. Large-file and degraded fixtures list the layer adjustments:
   narrowed, downgraded, hidden-with-replacement, or unchanged, plus the
   disclosure surface.

## 9. Acceptance Checklist

A reviewer can accept an editor-stack implementation or fixture when:

1. Every launch-bearing editor accessory maps to one canonical layer and
   one role class.
2. Source text, carets, selections, and IME composition stay in the
   `text_viewport`; annotations and action affordances do not
   impersonate committed source.
3. Transient diagnostics, hints, badges, hovers, peeks, overlays, and
   status changes do not move the stable text column.
4. Compare, restored, generated, read-only, live-preview, degraded, and
   large-file state are visible in the required layers with text or
   structural cues, not icon-only.
5. Large-file and degraded states say which layers narrow, disappear, or
   downgrade; what still works; what is reduced; and how to inspect the
   reason.
6. Any hidden gutter lane, inline hint, or transient provider state has
   a suppression/downgrade reason and a keyboard-reachable details path.
