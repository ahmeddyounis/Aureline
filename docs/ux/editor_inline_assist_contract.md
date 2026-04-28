# Editor Inline Assist Contract

This contract freezes the text-viewport model for inline decorations,
code lenses, inlay hints, ghost text, inline values, and inline quick
actions. The goal is to let diagnostics, debug, review, test, coverage,
language, AI, and generated-source features share one honest inline
surface instead of competing with private precedence and stale-truth
rules.

Companion artifacts:

- [`/schemas/editor/inline_assist.schema.json`](../../schemas/editor/inline_assist.schema.json)
  defines machine-readable inline-assist element and scenario-case
  records.
- [`/fixtures/editor/inline_assist_cases/`](../../fixtures/editor/inline_assist_cases/)
  contains worked cases for dense lines, stale semantic truth,
  generated read-only files, and partial-confidence hints.

Normative sources projected here:

- `.t2/docs/Aureline_UI_UX_Spec_Document.md` sections 11.2 through
  11.4 define the inline assist layer, accessory role disclosure,
  no-obscuring rules, and decoration precedence.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` sections 18.4
  and 18.5 define visual priority, inlay-hint/code-lens treatment, and
  separate toggles.
- `.t2/docs/Aureline_Technical_Design_Document.md` sections 7.1.13
  and 8.62 define typing-loop assist truth, AI ghost-text distinction,
  and stale/imported/approximate diagnostic labeling.
- [`/docs/ux/editor_anatomy_contract.md`](./editor_anatomy_contract.md)
  owns the editor layer boundary and stable text-column rule.
- [`/docs/ux/decoration_precedence_contract.md`](./decoration_precedence_contract.md)
  owns cross-surface decoration precedence and collapsed-state
  preservation.
- [`/docs/language/completion_and_inline_hint_contract.md`](../language/completion_and_inline_hint_contract.md)
  owns language completion, code-lens, and inlay-hint provenance,
  density, suppression, confidence, and side-effect vocabulary.
- [`/docs/language/diagnostics_and_code_action_contract.md`](../language/diagnostics_and_code_action_contract.md)
  owns diagnostic freshness, semantic-layer state, and code-action
  review posture.
- [`/docs/execution/debug_truth_contract.md`](../execution/debug_truth_contract.md)
  and [`/docs/execution/test_truth_contract.md`](../execution/test_truth_contract.md)
  own debug/current-frame, inline runtime value, test, and coverage
  truth.

If this document conflicts with those upstream sources, the upstream
owner wins and this document, schema, and fixtures must update in the
same change.

## 1. Boundary

The inline assist layer projects facts and affordances into or near the
text viewport. It does not own source bytes, diagnostic truth, debug
truth, review truth, test truth, coverage truth, language-provider
truth, AI evidence, generated lineage, policy, or command execution.

Frozen here:

- the decoration classes admitted into the inline assist layer;
- the required content posture for each inline element: committed
  content, annotation, preview, or action affordance;
- the precedence ladder used when multiple inline elements compete for
  one line, range, or insertion point;
- density reduction and suppression rules for large-file,
  low-confidence, partial-index, high-zoom, reduced-motion, generated,
  and read-only states;
- ghost-text and AI proposal distinction from committed source; and
- stale, approximate, partial, cached, and blocked truth labels.

Out of scope: final renderer implementation, exact colors, animation
curves, provider engines, AI ranking, debugger evaluation, test
execution, coverage collection, and concrete command shortcut defaults.

## 2. Decoration Classes

Every inline element must declare exactly one `inline_decoration_class`
and exactly one `content_posture_class`.

| Decoration class | Content posture | Primary owner | Notes |
| --- | --- | --- | --- |
| `diagnostic_decoration` | `annotation` or `action_affordance` | diagnostics contract | Squiggle, underline, faded span, or grouped marker for exact, approximate, stale, imported, or clustered findings. |
| `debug_current_frame` | `annotation` | debug contract | Current-frame or stopped-line state. It may pair with gutter state but is never displaced by hints. |
| `merge_conflict_marker` | `annotation` or `action_affordance` | VCS conflict contract | Conflict, unresolved side, or conflict-resolution entry point. |
| `diff_review_marker` | `annotation` or `action_affordance` | review / diff contracts | Review comments, changed hunk state, or diff-side qualifiers. |
| `test_cue` | `annotation` or `action_affordance` | test contract | Test status, inline test entry, or run-state marker. |
| `coverage_cue` | `annotation` | coverage / execution artifact truth | Covered, missed, partial, stale, or imported coverage truth. |
| `inline_value` | `annotation` | debug/runtime truth | Runtime value shown beside code. It must name exact, remapped, stale, or approximate evaluation posture. |
| `inline_quick_action` | `action_affordance` | command graph plus owning provider | Quick fix, explain, apply, rerun, open review, or source/generator review entry. |
| `ghost_text_ai` | `preview` | AI provider / policy / evidence contracts | AI proposal text. It is never committed content until explicitly accepted. |
| `ghost_text_deterministic` | `preview` | language/snippet/local provider | Deterministic preview text such as snippet or completion preview. |
| `code_lens` | `action_affordance` or `annotation` | language / graph / test / review owner | Reference counts, run links, docs links, or reviewable actions. |
| `inlay_type_hint` | `annotation` | language provider | Type metadata. |
| `inlay_parameter_hint` | `annotation` | language provider | Parameter-name metadata. |
| `inlay_return_hint` | `annotation` | language provider | Return-type or result metadata. |
| `freshness_generated_qualifier` | `annotation` | provider, generated-lineage, or freshness owner | Stale, generated, partial, read-only, imported, or approximate qualifiers. |
| `ambient_decoration` | `annotation` | renderer policy | Non-consequence-bearing scan aid. It collapses first and may not hide other state. |

Rules:

1. Inline elements are not allowed to imply committed source unless
   `content_posture_class` is `committed_content`. The inline assist
   layer normally emits annotations, previews, and action affordances;
   committed content belongs to the text viewport.
2. Any element with side effects chooses `action_affordance`, cites a
   command or detail route, names its side-effect class, and exposes a
   keyboard path.
3. Inlay hints, code lenses, ghost text, inline values, and qualifiers
   may not obscure source text, selection, caret, IME composition, or
   active search match.
4. A diagnostic, current frame, merge conflict, or review marker may
   open detail or fix routes, but the action route is separate from
   source content and from lower-priority hinting.

## 3. Precedence Ladder

Precedence answers which inline fact can remain visible when there is
not enough inline budget. It does not rewrite source truth.

| Rank | `precedence_band_class` | Wins over |
| ---: | --- | --- |
| 1 | `diagnostic_or_current_frame` | Every lower band. |
| 2 | `merge_conflict_diff_review` | Test, coverage, inline values, actions, ghost text, lenses, hints, qualifiers, and ambient decoration. |
| 3 | `test_coverage_execution` | Inline values, actions, ghost text, lenses, hints, qualifiers, and ambient decoration. |
| 4 | `inline_value_observation` | Actions, ghost text, lenses, hints, qualifiers, and ambient decoration. |
| 5 | `inline_action_affordance` | Ghost text, lenses, hints, qualifiers, and ambient decoration. |
| 6 | `ghost_text_preview` | Code lenses, inlay hints, qualifiers, and ambient decoration when the preview remains safe and visually distinct. |
| 7 | `code_lens_or_inlay` | Qualifiers and ambient decoration. |
| 8 | `decorative_qualifier` | Ambient decoration. |
| 9 | `ambient` | Nothing. It collapses first. |

Rules:

1. Nothing in this contract outranks diagnostics, current-frame state,
   or merge/conflict state except source text, selection, caret, and
   IME composition owned by the text viewport.
2. A same-line diagnostic and current frame can both remain visible
   because they use separate execution and diagnostic channels where
   available. If one primary label must win, current execution state
   owns the leading inline/gutter cue and the diagnostic remains
   visible or detail-preserved.
3. Merge/conflict and active review state outrank convenience metadata.
4. Test, run, and coverage cues outrank advisory code lenses and inlay
   hints.
5. Inline quick actions do not become higher priority merely because
   they are clickable. Their owning diagnostic, review, test, or policy
   state determines urgency; the action itself must yield to the
   visible truth marker.
6. Lower-priority elements hidden by precedence must carry
   `higher_priority_decoration`, the competing band, and a keyboard
   detail route that lists the hidden facts.

## 4. Density Reduction and Suppression

Inline assist defaults to `auto` density. The renderer may reduce or
suppress inline elements to protect readability, typing budget, and
truthfulness.

Allowed density modes:

| `density_mode_class` | Meaning |
| --- | --- |
| `off` | Hide non-essential inline assist. |
| `compact` | Keep only consequence-bearing or selected-line assist. |
| `standard` | Default inline metadata density. |
| `rich` | Expanded lenses, hints, inline values, and action affordances. |
| `auto` | Adapt by file size, zoom, certainty, provider, and mode posture. |

Required suppression or downgrade triggers:

| Trigger | Required behavior |
| --- | --- |
| `large_file_mode` | Suppress advisory lenses, inlay hints, ghost text, and ambient qualifiers before degrading input latency. |
| `low_confidence` | Downgrade or suppress hints that fall below the display floor. |
| `partial_index` | Label as partial or suppress semantic hints that cannot name admitted scope. |
| `high_zoom` | Reduce density and move crowded detail to keyboard-reachable line detail. |
| `reduced_motion` | Remove shimmer, pulsing, animated counters, and other motion-only attention cues. |
| `generated_file` | Hide direct mutation affordances unless they route to source/generator review. |
| `read_only_surface` | Hide or block write actions and keep inspect/copy/detail routes. |
| `higher_priority_decoration` | Hide the lower-priority inline element and preserve it in details. |
| `typing_budget_protection` | Suppress low-priority or late-arriving assist rather than moving text or delaying keystrokes. |

Rules:

1. Reduction is an explicit state. The element record names the trigger,
   visibility state, summary, and equivalent detail route.
2. Stale, cached, approximate, partial, limited, or blocked semantics
   may not render as `visible_primary`.
3. Generated and read-only surfaces may keep read-only annotations, but
   they must block or redirect unsafe inline quick actions.
4. Suppression never deletes truth. The detail route and support/export
   packet keep the hidden element, source, freshness, and reason.

## 5. Ghost Text and Preview Text

Ghost text is preview, not source. It remains visually and behaviorally
distinct until explicit acceptance commits an edit through the normal
undo path.

Rules:

1. `ghost_text_ai` must carry `source_actor_class: ai_provider`, an
   actor label, policy/trust context, and accept/reject keyboard routes.
2. AI ghost text must use preview styling and an attribution channel
   such as source label, preview role, ghost-text render class, or
   accessible name. It may not reuse committed-source styling.
3. Deterministic preview text must still declare that it is preview,
   cite its provider or snippet source, and expose accept/reject or
   dismiss routes.
4. Ghost text suppresses before diagnostics, current-frame state,
   merge/conflict state, review state, test/coverage cues, and active
   inline values.
5. Ghost text with generated-file, read-only, policy-blocked, stale, or
   partial-index constraints must downgrade, block, or route through a
   preview/review surface instead of showing a normal inline accept.

## 6. Stale and Approximate Language

Inline assist may present partial truth only when it says so.

Allowed truth labels:

| `truth_display_class` | Required meaning |
| --- | --- |
| `live_exact_label` | Current exact truth for the admitted scope. |
| `partial_scope_label` | Current only for a narrower loaded scope or workset. |
| `approximate_label` | Useful but not exact, remapped, heuristic, or runtime-observed without current semantic proof. |
| `stale_label` | Older than the active epoch or target floor. |
| `cached_label` | Warm cached result below exact-current truth. |
| `limited_fallback_label` | Syntax/text/fallback result without semantic proof. |
| `blocked_label` | Known hint or action exists but cannot be used. |
| `not_semantic_label` | No semantic truth is being claimed. |

Rules:

1. Partial-index, stale, cached, fallback, approximate, imported, and
   blocked providers may not emit `live_exact_label`.
2. Inline diagnostic decorations must distinguish exact, approximate,
   stale, imported, and clustered states in the accessible name and
   detail route.
3. Inline values from debug/runtime observation must say whether the
   frame mapping and value are exact, remapped, stale, or approximate.
4. Coverage and test cues from older runs remain useful only when they
   name the run freshness or target mismatch.
5. Support exports use the same labels as the interactive surface so a
   screenshot is never the only explanation.

## 7. Accessibility and Keyboard Parity

Every inline element must expose `keyboard_access`. Pointer hover may
enrich a surface, but it is not a complete route.

Required parity:

- annotations open line detail, diagnostic detail, debug detail, review
  detail, test/coverage detail, or provider detail from keyboard;
- action affordances cite a command id and resolve through the same
  command graph as palette, menu, shortcut, and automation routes;
- ghost text exposes accept, reject/dismiss, and attribution detail
  routes;
- hidden or suppressed elements remain reachable through line detail or
  owning surface detail;
- accessible names include the content posture first, then source,
  truth label, actionability, and suppression reason when present; and
- high-contrast, forced-colors, reduced-motion, screen-reader, and
  screenshot-safe presentations do not rely on hue alone.

## 8. Fixture Acceptance

The fixture corpus under
[`/fixtures/editor/inline_assist_cases/`](../../fixtures/editor/inline_assist_cases/)
is the reviewer baseline. A future renderer or language/debug/test/AI
provider can render any case and compare:

- input element classes and content posture;
- precedence winners and hidden-by-precedence records;
- density mode and suppression reasons;
- stale, approximate, partial, cached, blocked, or exact labels;
- ghost-text source attribution and preview distinction; and
- keyboard/accessibility routes for every visible, hidden, blocked, or
  suppressed element.

A review is non-conforming when it passes only because the screen looks
plausible while the record lacks source attribution, truth labels,
suppression reasons, or keyboard parity.
