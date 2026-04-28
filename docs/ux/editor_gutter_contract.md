# Editor Gutter Contract

This contract freezes the editor gutter before breakpoints, diagnostics,
folding, blame, test, coverage, and collaboration cues become separate
feature implementations. The goal is to keep the gutter predictable:
every signal either belongs in a known lane with known precedence or is
rejected as noise.

Companion artifacts:

- [`/schemas/ux/editor_gutter_lane.schema.json`](../../schemas/ux/editor_gutter_lane.schema.json)
  defines the machine-readable lane catalog and line-conflict case
  records.
- [`/fixtures/ux/gutter_cases/`](../../fixtures/ux/gutter_cases/)
  contains worked cases for dense conflicts, collapsed folds, debug stop
  state, and narrow-width fallback.

Normative sources projected here:

- `.t2/docs/Aureline_UI_UX_Spec_Document.md` section 11.1 defines the
  recommended gutter lane order, keyboard reachability, no-jitter rule,
  and high-contrast / screen-reader requirements.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` Appendix ES defines the
  editor anatomy and gutter lane precedence seed.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` section 15.1 and
  `.t2/docs/Aureline_UI_UX_Spec_Document.md` section 6.3 define compact
  icon-control hit target floors.
- [`/docs/ux/editor_anatomy_contract.md`](./editor_anatomy_contract.md)
  owns the canonical editor-layer boundary and stable text-column rule.
- [`/docs/ux/decoration_precedence_contract.md`](./decoration_precedence_contract.md)
  owns cross-surface decoration precedence and gutter/inline projection.
- [`/docs/language/diagnostics_and_code_action_contract.md`](../language/diagnostics_and_code_action_contract.md)
  owns diagnostic clustering, severity convergence, freshness, and
  semantic-layer truth.
- [`/docs/execution/debug_truth_contract.md`](../execution/debug_truth_contract.md)
  owns debug session, stopped-state, breakpoint, and frame truth.

If this document conflicts with those upstream sources, the upstream
owner wins and this document, schema, and fixtures must update in the
same change.

## 1. Boundary

The gutter is an editor-layer projection. It does not own source bytes,
diagnostic truth, debug truth, VCS truth, review truth, coverage truth,
test truth, collaboration presence, or folding ranges. It owns only
lane placement, conflict resolution, hit targets, accessibility labels,
and no-jitter layout obligations for the left-of-text editor chrome.

Frozen here:

- the canonical gutter lanes and their order;
- signal admission rules for placing or rejecting future gutter cues;
- same-line and same-lane precedence, including breakpoint versus
  diagnostic, fold versus execution, and high-severity versus low-value
  decoration conflicts;
- pointer hit-target, hover, keyboard-equivalent, and accessibility
  label rules; and
- no-jitter behavior when lines gain or lose gutter signals.

Out of scope: final renderer implementation, exact glyph artwork,
debugger implementation, coverage implementation, VCS blame
implementation, and concrete keyboard shortcut defaults.

## 2. Canonical Lanes

The gutter reserves lanes at the viewport level. A marker on one line
MUST NOT add per-line padding or move that line's source text start.

| Order | Lane | Owns | Must not own |
| ---: | --- | --- | --- |
| 0 | `line_number` | Numeric line position, current-line orientation, and stable digit reserve. | Breakpoint toggles, diagnostic severity, or semantic truth. |
| 1 | `execution_debug` | Breakpoints, logpoints, disabled breakpoints, current frame, stopped frame, and debug execution cues. | Diagnostic severity or fold disclosure. |
| 2 | `diagnostic` | Diagnostic severity markers, clustered problem counts, stale/imported diagnostic disclosure, and line-local diagnostic detail entry. | Breakpoint hit targets, code action application, or provider truth. |
| 3 | `change_review` | VCS change bars, merge/conflict markers, review anchors, comment anchors, and change-family summaries. | Diagnostic severity or current-frame state. |
| 4 | `fold_control` | Fold availability, collapsed fold affordances, and hidden-range summaries. | The only disclosure for hidden diagnostics, conflicts, or execution state. |
| 5 | `supplemental` | Blame, coverage, test, collaboration, generated-lineage, freshness, and other low-urgency context cues. | Crowding out execution, diagnostics, changes, folds, or text-column stability. |

The `line_number` lane is structural. It remains visible whenever line
numbers are enabled and width permits; it is not allowed to win over
execution or diagnostic state by consuming arbitrary extra space.

The `diagnostic` and `change_review` lanes MAY render as a combined
compact visual lane at narrow widths, but the backing record still
resolves them separately. If only one visible subslot is available,
diagnostic error or warning state wins over ordinary change bars; merge
conflict or active review state remains in the accessible name and
detail view.

## 3. Signal Admission

Every gutter signal is admitted through the lane catalog before it can
render.

| Signal family | Default lane | Admission rule |
| --- | --- | --- |
| `line_number`, `current_line` | `line_number` | Accepted when line orientation is enabled. Current-line cues must not overpower diagnostics or selection. |
| `breakpoint_enabled`, `breakpoint_disabled`, `conditional_breakpoint`, `logpoint`, `debug_current_frame`, `debug_paused_frame` | `execution_debug` | Accepted only with a debug or breakpoint authority ref and a command or detail route when actionable. |
| `diagnostic_error`, `diagnostic_warning`, `diagnostic_info`, `diagnostic_hint` | `diagnostic` | Accepted only from a diagnostic cluster or producer ref. Severity and freshness remain inspectable. |
| `git_added`, `git_modified`, `git_deleted`, `merge_conflict`, `review_thread` | `change_review` | Accepted only from VCS, diff, conflict, or review authority. Conflict and active review outrank ordinary change bars. |
| `fold_available`, `fold_collapsed`, `fold_hidden_range_summary` | `fold_control` | Accepted only from the folding/range model. Collapsed folds must summarize hidden high-severity state. |
| `coverage_pass`, `coverage_miss`, `test_pass`, `test_fail`, `blame_author`, `collaboration_presence`, `generated_lineage`, `freshness_stale` | `supplemental` | Accepted only when detail survives collapse. These cues collapse before primary lanes are harmed. |
| `ambient_line_decoration` | none | Rejected unless it can be restated as one of the families above and adds user value beyond ornament. |

A new signal that cannot cite a default lane, source authority, visual
priority, keyboard/detail route when actionable, and no-jitter behavior
MUST be rejected from the gutter. It may use another editor layer only
if that layer's contract admits it.

## 4. Precedence

Lane order resolves cross-family conflicts. Same-lane conflicts resolve
by consequence.

### 4.1 Breakpoint Versus Diagnostic

Breakpoints and diagnostics do not compete for one slot. A breakpoint,
logpoint, or current-frame cue occupies `execution_debug`; diagnostics
occupy `diagnostic`.

Rules:

1. A current debug frame wins the primary execution slot over an
   ordinary breakpoint on the same line.
2. A breakpoint remains actionable when a diagnostic is present; the
   diagnostic remains perceivable and inspectable.
3. If the gutter collapses lanes for width, the compact representation
   must still announce both facts, for example `Current frame,
   breakpoint enabled, error`.
4. Code actions associated with diagnostics do not run from the
   breakpoint target. They open through diagnostic detail, quick fix, or
   command palette routes.

### 4.2 Fold Versus Execution

Fold controls do not cover execution cues. When a line has a fold
affordance and an execution/debug signal:

1. `execution_debug` remains the primary interactive lane.
2. `fold_control` remains visible when width permits.
3. If width does not permit a separate fold hit target, the fold state
   moves to the line detail route and keyboard fold command while its
   collapsed/expanded state remains in the accessible name.
4. A collapsed fold that hides breakpoints, diagnostics, conflicts, or
   review anchors must summarize those hidden states before any
   supplemental cue appears.

### 4.3 High-Severity Versus Low-Value Decoration

Diagnostic errors, warnings, current-frame state, breakpoints, merge
conflicts, active review anchors, and collapsed-fold state outrank
supplemental cues.

Rules:

1. Error beats warning, warning beats info, and info beats hint inside
   the diagnostic lane.
2. Merge conflict or active review beats ordinary added/modified/deleted
   bars inside the change/review lane.
3. Failed tests and coverage misses beat blame, collaboration, generated
   qualifiers, and freshness notes inside the supplemental lane.
4. Blame, coverage, tests, collaboration, generated-lineage, and
   freshness cues collapse before they change text layout or hide
   execution, diagnostic, change, or fold state.
5. Collapsed lower-priority cues must name their family in detail. A
   generic dot or unlabelled count is non-conforming.

## 5. Interaction Contract

Every gutter action has a command identity. Hover may reveal detail, but
hover is never the only route to core workflows.

| Workflow | Pointer target | Keyboard equivalent | Detail / label requirement |
| --- | --- | --- | --- |
| Toggle breakpoint or logpoint | `execution_debug` lane target, minimum 28 logical px hit target. | Command ref for toggle/open breakpoint detail. | Accessible name includes line, breakpoint/logpoint state, and any condition/log message availability. |
| Inspect current frame | `execution_debug` lane target or line detail target. | Command ref for opening current frame / debug detail. | Accessible name names stopped state and mapped source confidence when known. |
| Inspect diagnostic | `diagnostic` lane target or line detail target. | Command ref for diagnostic detail / next problem. | Accessible name includes dominant severity, count, freshness, and whether hidden diagnostics exist. |
| Toggle fold | `fold_control` lane target when visible. | Fold/unfold command for current line or selected region. | Accessible name includes expanded/collapsed state and hidden high-severity summary. |
| Inspect blame, coverage, test, collaboration, generated, or freshness cue | Supplemental lane target when visible; otherwise line detail target. | Command ref for line detail or owning feature detail. | Accessible name names the cue family and collapse reason when hidden. |

Hit target rules:

1. Icon-only gutter actions MUST have at least a 28 logical px target in
   the action axis. Comfortable density SHOULD provide 36 logical px.
2. Pointer and keyboard activation resolve to the same command id and
   target snapshot.
3. The visible glyph may be smaller than the target, but the target must
   not overlap a different command without a deterministic focus order.
4. Hover/focus detail may enrich the label, but the accessible name must
   already state the primary meaning.
5. Color is never the only channel. High-contrast and forced-colors
   modes need shape, glyph, border, position, text, or structured
   accessible naming for breakpoint, folded, and diagnostic state.

## 6. No-Jitter Layout

The gutter reserves its width by viewport, not by line. Within a
viewport:

1. Adding or removing a breakpoint, diagnostic, fold affordance, change
   marker, blame cue, coverage cue, test cue, or collaboration cue MUST
   NOT change the source text x-position for only the affected line.
2. Lane width may change only for explicit renderer policy reasons:
   font/zoom change, density change, explicit user layout change,
   line-count digit-reserve threshold crossing, or viewport resize.
3. Line-number digit reserve changes are document/viewport decisions,
   not visible-range oscillations. Scrolling from line 99 to line 100
   must not cause repeated gutter bounce if the reserve threshold was
   already planned.
4. When width is constrained, lanes collapse by precedence instead of
   pushing source text. Supplemental cues collapse first, then ordinary
   change bars, then fold pointer targets with keyboard/detail fallback.
   Execution and diagnostic state remain perceivable.
5. Any permitted reflow must emit a reason class so layout tests can
   distinguish intentional recompute from jitter.

Forbidden behaviors:

- a diagnostic badge increasing left padding only on one line;
- a breakpoint appearing in the same visual target as a diagnostic
  quick fix without distinct command identity;
- a folded range hiding an error with no visible or accessible summary;
- a blame or coverage badge displacing a breakpoint, current frame,
  diagnostic error, merge conflict, or fold state;
- a hover-only fold, breakpoint, diagnostic, or blame workflow; and
- a compact gutter that keeps color dots but drops screen-reader labels.

## 7. Fixture and Schema Requirements

Records using
[`/schemas/ux/editor_gutter_lane.schema.json`](../../schemas/ux/editor_gutter_lane.schema.json)
must satisfy these invariants:

1. Every accepted signal resolves to exactly one canonical lane.
2. Every rejected signal declares a rejection reason.
3. Each line case includes expected lane resolution, collapsed signals,
   accessible-name parts, high-contrast non-color channels, and keyboard
   command refs for actionable states.
4. Breakpoint, folded, and diagnostic states remain perceivable in
   high-contrast and screen-reader flows.
5. Text-column measurements show `column_shift_px = 0` for signal
   additions and lane collapses.
6. Dense, folded, debug-stop, and narrow-width cases cover the
   precedence rules above.

Future implementations should render each fixture in normal, compact,
high-contrast, forced-colors, high-zoom, and screen-reader audit modes.
If a renderer cannot preserve these assertions, it must narrow the lane
set with disclosure rather than inventing local precedence.
