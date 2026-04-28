# Decoration Precedence Contract

This document freezes the shared status-icon, badge, and decoration
contract for shells, tables, tabs, cards, editor layers, review rows,
and support/export projections. Its purpose is to stop surfaces from
stacking local badges with conflicting meaning or hiding high-stakes
state behind lower-priority chrome.

Companion artifacts:

- [`/artifacts/ux/status_icon_legend.yaml`](../../artifacts/ux/status_icon_legend.yaml)
  publishes the machine-readable legend and precedence bands.
- [`/fixtures/ux/decoration_cases/`](../../fixtures/ux/decoration_cases/)
  contains worked conflict cases reviewers can compare against
  rendered tables, tabs, cards, badge lists, editor layers, and review
  rows.

This contract composes with, and does not replace:

- `.t2/docs/Aureline_UI_UX_Spec_Document.md` Appendix BA for the
  status-icon legend and editor decoration precedence seed.
- [`/docs/design/component_state_taxonomy.md`](../design/component_state_taxonomy.md)
  for `locked`, `degraded`, `pending`, and `warning_error` component
  states.
- [`/docs/design/design_token_component_state_vocabulary.md`](../design/design_token_component_state_vocabulary.md)
  for high-contrast and forced-colors state conveyance.
- [`/docs/ux/status_bar_contract.md`](./status_bar_contract.md),
  [`/docs/ux/status_strip_family_contract.md`](./status_strip_family_contract.md),
  [`/docs/ux/editor_gutter_contract.md`](./editor_gutter_contract.md),
  [`/docs/ux/editor_inline_assist_contract.md`](./editor_inline_assist_contract.md),
  [`/docs/ux/tabs_editor_groups_contract.md`](./tabs_editor_groups_contract.md),
  [`/docs/ux/tree_row_contract.md`](./tree_row_contract.md), and
  [`/docs/ux/collection_view_contract.md`](./collection_view_contract.md)
  for surface-specific anatomy and overflow behavior.
- [`/docs/language/completion_and_inline_hint_contract.md`](../language/completion_and_inline_hint_contract.md)
  and
  [`/docs/language/diagnostics_and_code_action_contract.md`](../language/diagnostics_and_code_action_contract.md)
  for inline metadata, diagnostic convergence, and detail-sheet
  requirements.
- [`/schemas/editor/inline_assist.schema.json`](../../schemas/editor/inline_assist.schema.json)
  and
  [`/fixtures/editor/inline_assist_cases/`](../../fixtures/editor/inline_assist_cases/)
  for editor-inline element records and precedence fixtures.
- [`/docs/security/safe_preview_trust_classes.md`](../security/safe_preview_trust_classes.md)
  for trust-class, owner, origin, representation, snapshot, and
  suspicious-content chrome.

If this document conflicts with those upstream sources, the upstream
owner wins and this document, the legend artifact, and fixtures must be
updated in the same change.

## Scope

Frozen here:

- one shared legend for trust, lifecycle, support class, freshness,
  imported or snapshot state, generated state, danger or severity, and
  blocked or degraded conditions;
- one visual precedence ladder for rows, tabs, cards, badge lists,
  review rows, and object headers;
- one editor decoration precedence mapping that preserves the source
  inline and gutter priority bands;
- compact and overflow rules that identify the primary visible cue,
  the secondary summary affordance, and the facts that remain in
  accessible names and detail views; and
- no-hue-only requirements for high contrast, forced colors, reduced
  motion, compact density, screen readers, screenshots, and exports.

Out of scope: final icon artwork, per-brand marketplace marks, color
tokens, animation curves, renderer implementation, or schema generation
for the eventual Rust UI types.

## Shared Decoration Record

Every badge-bearing surface resolves state through a shared decoration
record before rendering. A surface may choose local layout, but it must
not reinterpret the state family or invent a private priority.

| Field | Required content | Non-conforming collapse |
| --- | --- | --- |
| `object_ref` | Stable object, row, tab, card, line, range, or review item ref. | Inferring identity from the visible label alone. |
| `surface_family` | `table_row`, `tab`, `card`, `badge_list`, `editor_gutter`, `editor_inline`, `review_row`, `status_item`, or `support_export`. | A private surface family with no mapped rules. |
| `state_families` | Independent state facts from the legend artifact. | Combining support, lifecycle, trust, freshness, and severity into one vague badge. |
| `primary_visual_state` | The highest-precedence visible fact after conflict resolution. | Letting a lower-priority decorative cue occupy the only visible slot. |
| `secondary_visible_states` | Additional states that must remain visible beside the primary cue. | Stacking all low-risk badges until the high-risk cue is crowded out. |
| `collapsed_summary` | Counted summary for lower-priority states that lost visible space. | Dropping generated, stale, imported, or support truth without a details path. |
| `accessible_name_parts` | Ordered phrases for primary, secondary, and collapsed facts. | Icon-only names, hue-only meaning, or summary text that omits the losing states. |
| `detail_sections` | Drill-in sections preserving every state family and source ref. | A screenshot-only argument with no inspectable source records. |

The record is semantic, not decorative. If the visible surface and the
record disagree, the visible surface is wrong.

## State Families

Each family is independent. Surfaces may condense presentation, but
they may not merge families into a single ambiguous label.

| Family | Answers | Examples | Required preservation |
| --- | --- | --- | --- |
| `trust` | Is this object or surface safe to trust, execute, preview, copy, or mutate? | restricted workspace, policy locked, suspicious content, active preview trust class | Always present in accessible name and detail view when non-default; visible at commit or trust-decision points. |
| `blocked_degraded` | Is the workflow blocked, read-only, narrowed, partial, or running through fallback? | blocked, read-only degraded, policy blocked, provider degraded, partial | Visible when it changes the next safe action. |
| `danger_severity` | How severe is the finding or consequence? | danger, error, warning, info, security critical | High severity is visible and outranks lifecycle, support, freshness, generated, and ambient cues. |
| `review_conflict` | Does review, diff, conflict, or comment state affect interpretation? | merge conflict, changed hunk, review thread, comment anchor | Visible on review-bearing rows unless outranked; always preserved in detail. |
| `execution_test` | Does execution, debug, test, task, or coverage state affect the object? | current frame, breakpoint, running task, failed test, coverage miss | Editor lanes and review rows preserve owner refs and suppression reasons. |
| `inline_metadata` | Is advisory inline metadata competing for editor space? | code lens, inlay hint, type hint, reference count | Suppresses first when a higher-priority decoration consumes the slot. |
| `lifecycle` | What readiness posture has the owner declared? | labs, preview, beta, stable, deprecated, retired | Preserved as its own axis; not a substitute for support or freshness. |
| `support_class` | What help or SLA posture applies? | no support, best effort, community, standard, extended, operator only | Visible when weaker than the surrounding surface claim or when it affects a decision. |
| `freshness` | How current is the evidence? | authoritative live, warm cached, degraded cached, stale, unverified | Visible when not authoritative live. |
| `imported_snapshot` | Is the object imported, captured, static, mirrored, or metadata-only? | imported scanner result, captured snapshot, static preview, provider overlay | Preserved in accessible names, support exports, and mutation reviews. |
| `generated` | Is the object generated, derived, or safe to edit only through lineage? | generated read-only, generated edit-blocked, generated editable with lineage | Visible for editing, refactor, review, and export contexts. |
| `ambient` | Does the cue merely aid scanning without changing consequence? | favorite, recent, decorative grouping, ordinary file type | Collapses first and may not displace consequence-bearing state. |

## Visual Precedence Ladder

The same object must resolve to the same dominant meaning in a table
row, tab, card, badge list, review row, or support export. The visual
budget may differ, but the highest applicable band below wins the
primary slot.

| Rank | Band | Wins when | Visual rule |
| ---: | --- | --- | --- |
| 1 | `trust_or_policy_critical` | Trust, policy, credential, suspicious-content, active-content, or authority state changes whether an action is safe. | Primary label must pair icon and text. Other states move to secondary or summary. |
| 2 | `blocked_or_high_severity` | A workflow is blocked, data may be lost, mutation is denied, or a high-severity finding exists. | Primary label must name the blocker or severity and expose the narrowest details route. |
| 3 | `diagnostic_or_debug` | Current-frame, diagnostic, parse, runtime, or execution truth is the highest editor/review concern. | Editor lanes follow inline/gutter precedence; rows use a visible severity or current-frame cue. |
| 4 | `merge_diff_review` | Conflict, diff, review thread, comment anchor, or change-state truth is present. | Visible when the row is review-bearing; lower bands summarize. |
| 5 | `test_coverage_execution` | Test, run, benchmark, coverage, or task execution state affects interpretation. | Visible unless displaced by ranks 1-4; detail view keeps all run refs. |
| 6 | `degraded_partial_readonly` | The object remains usable through a narrowed, partial, stale, read-only, or fallback path. | Visible if it changes available actions; otherwise summary plus accessible name. |
| 7 | `lifecycle_support` | Lifecycle or support class is weaker, narrower, or more time-bound than normal. | Render as separate axes; never collapse into "available" or "unsupported" alone. |
| 8 | `freshness_import_snapshot` | Evidence is cached, stale, unverified, imported, captured, provider-backed, static, or metadata-only. | Visible when mutation, review, support, or trust depends on it; otherwise counted summary. |
| 9 | `generated_derived` | Generated or derived lineage changes edit, refactor, or export behavior. | Visible for edit/review surfaces; summary allowed only when generated-safe actions are unavailable. |
| 10 | `code_lens_or_inlay` | Advisory code lenses, inlay hints, type hints, or reference counts compete for editor space. | Hidden before consequence-bearing decorations and recorded with a suppression reason. |
| 11 | `ambient_decorative` | The cue does not change consequence, trust, authority, or action availability. | Collapses first and may be omitted when budget is constrained. |

When multiple states share the same highest band, the owning subsystem
selects one primary label by consequence:

1. unsafe or untrusted beats blocked;
2. blocked beats degraded;
3. destructive or data-loss risk beats ordinary error;
4. error beats warning;
5. current exact live beats stale, imported, or remapped evidence; and
6. source-owned current truth beats provider overlay when both describe
   the same object and no higher trust or policy rule says otherwise.

The losing same-band states remain either visible as secondary badges or
listed in the summary affordance and detail view.

## Surface Projection Rules

| Surface | Primary slot | Secondary slot | Summary affordance | Detail requirement |
| --- | --- | --- | --- | --- |
| Table or dense row | One leading state cell or badge with text at normal density. | At most two short badges when they change action availability. | Counted `more states` row action or inline summary chip. | Detail row or inspector lists every state family and source ref. |
| Tab | One compact cue beside the label, plus structural cues for dirty, pinned, active, and blocked states. | Dirty, blocked, read-only, shared, and generated cues remain recoverable in overflow. | Overflow row names every hidden state, not only the tab title. | Full title, accessible name, tooltip/focus popover, and overflow row preserve all states. |
| Card | Header badge uses the highest band; body may include a status strip for secondary facts. | Lifecycle/support and freshness may appear as a small axis group when space allows. | Expandable details or footer summary. | Details include source refs, freshness, trust, support, and action consequences. |
| Badge list | Ordered by precedence, not by arrival time or provider order. | Independent axes use separate badges until budget is exhausted. | Lower bands collapse into a counted summary with family names. | Expanding the summary restores the ordered badge list. |
| Editor gutter | Breakpoint or execution lane, diagnostics, change markers, fold controls, then supplemental cues. | Same-line conflicts use lane placement before adding inline text. | Lower-priority lanes may aggregate to hover/focus detail. | Keyboard detail exposes hidden diagnostics, review anchors, coverage, and generated/freshness cues. |
| Editor inline | `diagnostic_or_debug`, `merge_diff_review`, `test_coverage_execution`, `code_lens_or_inlay`, then `decorative_qualifier`. | Hints may downgrade but may not obscure caret, selection, or readable code. | Suppressed hints carry `higher_priority_decoration` or a narrower reason. | Detail view records every hidden hint and suppression reason. |
| Review row | Severity, trust, blocked state, and conflict state lead; freshness/import/imported evidence follows. | Source, support, generated, or snapshot badges remain visible if they affect apply. | Collapsed evidence summary names counts by family. | Review detail preserves every evidence source and divergence. |
| Support export | No visual budget limit; emit ordered facts and source refs. | Not applicable. | Not applicable. | Export preserves the same dominant state and all collapsed states. |

## Collapse and Summary Rules

1. A surface may render only one primary visual state, but the backing
   record keeps every applicable family.
2. `trust`, `blocked_degraded`, and high `danger_severity` states may
   collapse only into a visible summary that names the family, such as
   `Blocked plus 3 states`; they may not disappear into a generic dot.
3. `lifecycle` and `support_class` are separate axes. `Preview` plus
   `No support` is not `Experimental`; `Stable` plus stale evidence is
   not `Ready`.
4. `freshness` and `imported_snapshot` are separate axes. A stale live
   provider result is not the same as an imported snapshot.
5. `generated` and `read_only` are separate axes. A generated object may
   be editable through lineage, and a read-only object may be authored
   source blocked by policy or provider state.
6. Same-band conflicts must preserve the losing facts in the accessible
   name and details. A compact row that says `Policy blocked` still
   names `generated`, `stale snapshot`, and `no support` when those
   facts also apply.
7. Summary counts group by family and state, not by badge count. Three
   stale provider rows behind one object become `stale evidence` with a
   count in details, not three identical stale chips.
8. Screenshots, exports, and support packets use the same family names
   as the interactive surface so review does not depend on color or
   layout.

## No-Hue-Only and Accessibility Rules

Color may reinforce a cue, but it never carries state alone.

Mandatory non-color channels:

- `trust`, `policy`, `blocked`, `danger`, `error`, `read_only`,
  `degraded`, `stale`, `imported_snapshot`, and `generated` states need
  text, shape, icon, border, position, or structural placement in
  addition to hue.
- High-severity and trust-critical cues require visible text at normal
  density. Icon-only treatment is allowed only for repeated compact
  chrome when the accessible name, tooltip or focus popover, and detail
  route name the state.
- Compact summaries must expose a keyboard-reachable expansion or
  details route. Pointer-only hover does not satisfy this contract.
- Forced-colors and high-contrast modes keep borders, shapes, glyphs,
  and labels. A token that disappears when color is removed is
  non-conforming.
- Reduced-motion mode replaces pulsing, shimmer, animated counters, or
  progress loops with static labels, reserved layout width, and
  text-based progress or state.
- Screen-reader names list the primary state first, then required
  secondary states, then the summary count, for example:
  `Policy blocked, generated file, stale snapshot, no support`.
- Export and screenshot-safe labels use the same state words as the
  source surface. A screenshot of a compact tab must still reveal the
  dominant state without relying on red/green meaning.

## Detail View Requirements

Any surface that collapses one or more state families must expose a
detail view with these sections when applicable:

- `primary_state_reason` - why the winning band won;
- `trust_and_policy` - trust class, policy source, authority, and owner
  or origin refs;
- `blockers_and_degradation` - blocked, read-only, partial, fallback,
  or degraded path plus recovery action;
- `severity_and_review` - diagnostic, review, conflict, or security
  severity sources;
- `lifecycle_and_support` - lifecycle, channel, support class, and
  narrowing reason;
- `freshness_and_snapshot` - live/cached/stale/imported/snapshot state,
  timestamp, epoch, and source;
- `generated_and_lineage` - generated or derived source refs, edit
  posture, and regeneration route; and
- `hidden_by_precedence` - every state hidden, collapsed, or suppressed
  by visual budget.

Details must be reachable by keyboard, command palette, or the owning
row/tab/card action. Details available only through hover or raw logs do
not satisfy the contract.

## Fixture Use

The fixture corpus under
[`/fixtures/ux/decoration_cases/`](../../fixtures/ux/decoration_cases/)
is the reviewer baseline. A future implementation can render any case
as a table row, tab, card, badge list, editor decoration, or review row
and compare the emitted primary state, collapsed states, accessible
name, and detail sections against the fixture.

A visual review is non-conforming when it passes only because a
screenshot appears plausible while the fixture's state families,
precedence decision, or accessibility assertions are missing.
