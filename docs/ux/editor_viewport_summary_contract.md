# Editor Viewport Summary Contract

This contract freezes the optional editor summary surfaces that help a
user scan or jump through a file without turning those surfaces into a
second source of truth. Minimap, overview-ruler, fold-summary,
search-hit tick, and diagnostic-summary projections are allowed only
when they remain faithful to the source viewport, gutter, folding
model, diagnostics, search, review, and degraded-state contracts.

Companion artifacts:

- [`/schemas/editor/viewport_summary.schema.json`](../../schemas/editor/viewport_summary.schema.json)
  defines the machine-readable surface and case records.
- [`/fixtures/editor/viewport_summary_cases/`](../../fixtures/editor/viewport_summary_cases/)
  contains worked cases for fold-heavy files, large-file mode,
  degraded semantics, and narrow editor groups.

This contract composes with, and does not replace:

- [`/docs/ux/editor_anatomy_contract.md`](./editor_anatomy_contract.md)
  for editor layer ownership and the source-viewport truth boundary.
- [`/docs/ux/editor_gutter_contract.md`](./editor_gutter_contract.md)
  for gutter lane admission, precedence, hit targets, and no-jitter
  behavior.
- [`/docs/ux/decoration_precedence_contract.md`](./decoration_precedence_contract.md)
  for cross-surface decoration precedence and detail preservation.
- [`/docs/ux/view_freshness_contract.md`](./view_freshness_contract.md)
  for live, partial, stale, snapshot, and approximate disclosure.
- [`/docs/ux/degraded_mode_pattern.md`](./degraded_mode_pattern.md)
  for degraded-state labels, preserved capability disclosure, and
  recovery paths.
- [`/docs/language/diagnostics_and_code_action_contract.md`](../language/diagnostics_and_code_action_contract.md)
  for diagnostic source, freshness, semantic-layer, and code-action
  truth.
- [`/docs/search/search_readiness_vocabulary.md`](../search/search_readiness_vocabulary.md)
  for search readiness, partial scope, hidden-scope, and stale-result
  wording.
- [`/docs/adr/0003-buffer-undo-large-file.md`](../adr/0003-buffer-undo-large-file.md)
  for large-file mode switch conditions and reduced-capability rules.

Normative sources projected here:

- `.t2/docs/Aureline_Technical_Design_Document.md` section 7.1.15
  says orientation aids accelerate editing but must not become a
  hidden truth model.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` sections 11.1 through
  11.7 and 11.15 define editor anatomy, gutter behavior, large-file
  mode, constrained-file state, folding, minimap, and overview-ruler
  expectations.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` section 10,
  section 12.7, section 15.3, and Appendix BK define reduced-motion,
  adaptive-width, gutter, fold-summary, and minimap-disabled guidance.

If this document conflicts with those upstream sources, the upstream
owner wins and this document, schema, and fixtures must update in the
same change.

## 1. Boundary

Viewport summaries are projections over canonical editor state. They do
not own source bytes, search truth, diagnostic truth, review anchors,
debug truth, fold ranges, generated-lineage truth, policy state, or
write authority.

Frozen here:

- which summary surfaces may appear and what they may claim;
- source-authority requirements for every marker;
- suppression and downgrade rules for large-file, mixed zoom,
  accessibility, reduced motion, low-confidence semantics, degraded
  runtime, and constrained-width editor groups;
- folded-region, off-screen diagnostic, search-hit, and review-marker
  discoverability rules;
- fallback language when a summary is hidden, stale, partial, or
  unavailable; and
- schema and fixture invariants used by renderer, accessibility,
  support/export, and contract tests.

Out of scope: final minimap renderer, exact pixel scaling, color
tokens, scrollbar implementation, pointer drag physics, search engine
implementation, diagnostics engine implementation, and fold-range
generation.

## 2. Core Rule

The source viewport remains the canonical route to the file. A summary
surface may speed up scanning, but it must never be the only way to
discover or act on a warning, error, fold, conflict, review marker,
search hit, breakpoint, trust warning, or generated/read-only state.

Every summary marker must answer:

1. Which canonical source produced it.
2. Whether that source is live exact, partial, stale, approximate, or
   unavailable.
3. Which canonical route exposes the full detail without relying on the
   summary.
4. Whether the marker is visible, compact, partial, stale, hidden with
   replacement, or suppressed.
5. Which keyboard and assistive-technology path reaches the same
   information.

If any answer is missing, the marker is not admitted.

## 3. Summary Surfaces

| Surface | Primary job | Allowed claims | Must never claim |
| --- | --- | --- | --- |
| `minimap` | Provide a coarse scaled preview of the open buffer and optional marker overlays. | Approximate document shape, current viewport position, and admitted marker families. | Full source fidelity, semantic completeness, exact diagnostics, or hidden critical state. |
| `overview_ruler` | Provide a thin marker rail for high-signal positions in the whole buffer. | Admitted diagnostics, search hits, review/conflict markers, breakpoints, folded regions, and current viewport position. | Marker truth not backed by a canonical source or a complete list of all issues when sources are partial. |
| `fold_summary` | Summarize a collapsed range in or beside the source viewport. | Hidden line count, structure label when available, and hidden high-signal state from canonical sources. | That folded content is clean unless diagnostics, search, review, conflict, trust, and change state were checked for the folded range. |
| `search_hit_ticks` | Show current find/search matches as positional ticks. | The active search-session result set, with scope, count, and freshness labels. | Workspace-wide completeness when the query ran only in-file, partial, chunked, or stale. |
| `diagnostic_summary` | Summarize off-screen or hidden diagnostic state. | Counts and severities from diagnostic-cluster records with freshness and semantic-layer labels. | A new diagnostic source, a fix-safe claim, or current exact semantics when diagnostics are stale, imported, or partial. |

Future summary surfaces must reuse this contract or update it in the
same change. A private "mini overview" with local marker vocabulary is
non-conforming.

## 4. Source Authority

Every summary marker references one canonical source family:

| Marker family | Canonical source | Required fallback route |
| --- | --- | --- |
| Diagnostics | diagnostic cluster or task/build/test diagnostic record | Problems/detail route, next/previous problem, and line detail. |
| Search hits | active search or find session result set | Find widget, search panel/result list, next/previous match. |
| Review comments | review anchor or review workspace object | Review thread detail and line detail. |
| Merge/conflict/change | VCS or review/conflict authority | Diff/hunk/conflict detail and line detail. |
| Breakpoints/current frame | debug session or breakpoint authority | Debug detail and breakpoint list/command. |
| Folded range | folding model over the current buffer snapshot | Fold/unfold commands and line detail. |
| Trust or policy warning | trust/policy authority | Policy/trust detail, command diagnostics, and status/context route. |
| Generated/read-only/freshness | generated-lineage, buffer posture, or freshness record | Context reason strip, status detail, and support/export row. |

Rules:

1. A marker without a non-null canonical source ref is rejected.
2. A marker whose source is stale, partial, imported, approximate, or
   low confidence may render only with that truth label.
3. A marker whose source no longer maps to the current buffer snapshot
   must move to `stale` or `hidden_with_replacement`; it may not drift
   to a nearby line silently.
4. Counts are derived from source records. Summary surfaces do not
   invent counts by sampling pixels or visible rows.
5. Raw source text, raw file paths, raw provider payloads, and
   credential material do not cross this boundary.

## 5. Appearance Decision

A summary surface may appear only when all gates below pass.

| Gate | Pass condition | Required failure behavior |
| --- | --- | --- |
| Source authority | Every marker has a canonical source ref and admitted freshness. | Reject unbacked markers; show replacement route if the surface itself hides. |
| Core route parity | The source viewport, gutter/detail, Problems/search/review/debug route, or command palette exposes the same critical information. | Hide the summary until replacement route exists. |
| Layout budget | The editor group can reserve the surface without hiding source text, required context, or recovery-critical status. | Hide minimap first, then compact overview; preserve source viewport. |
| Accessibility posture | Meaning survives screen reader, high contrast, forced colors, keyboard-only, and reduced-motion modes. | Move to text/detail/status routes and name the reason. |
| Performance budget | Rendering and updating the summary does not consume protected typing, cursor, selection, or scroll budget. | Static snapshot, coalesced markers, or hidden-with-replacement. |
| Freshness and confidence | The source can label current, partial, stale, approximate, or unavailable honestly. | Downgrade or suppress exact-looking markers. |
| User/admin setting | User preference, workspace policy, or restricted mode allows the surface. | Hide with controlled fallback language and keep canonical routes. |

The appearance decision is per editor view. A split editor may show the
overview ruler in one group and hide the minimap in another if width,
zoom, or policy differs, but both groups must share the same canonical
source records and truth labels.

## 6. Suppression and Downgrade Matrix

| Trigger | Minimap | Overview ruler | Fold summaries | Search-hit ticks | Diagnostic summaries |
| --- | --- | --- | --- | --- | --- |
| Large-file mode | Hidden or static document-shape only. | May remain with viewport-bounded or diagnostics-only markers. | Remain for collapsed ranges when folding is available; otherwise move to line detail. | Chunked or partial with count truth. | Partial/unavailable labels required; no full-diagnostics claim. |
| Mixed zoom, high zoom, or scale transition | Hide before source text, context, or status is clipped. | Compact or detail-only. | Keep visible text summary if it fits; otherwise line detail. | Detail route and next/previous match remain. | Status/detail route remains. |
| Screen reader, keyboard-only, high contrast, forced colors | Optional visual surface may hide. | Marker rail may render only if non-color channels and labels survive. | Must expose accessible name and commands. | Find/search routes announce count and current match. | Problems/detail/status routes announce counts. |
| Reduced motion, power saver, thermal, or protect-core runtime | Disable live animation and continuous repaint. | Static/coalesced markers only. | No animated fold transitions required. | Coalesced updates with freshness. | Coalesced updates with freshness. |
| Low-confidence or stale semantics | Semantic overlays hidden or labeled. | Current exact markers can remain; stale semantic markers label or hide. | Hidden-state counts disclose unknown or stale semantic coverage. | Textual search may remain exact; semantic search labels partial/approximate. | Stale/imported/partial diagnostic labels required. |
| Constrained-width editor group | Hidden first. | Compact; aggregate lower-priority families. | Fold command/detail route remains; pointer target may move to detail. | Find widget/search panel remains canonical. | Problems/status/line detail remains canonical. |
| Restricted mode or policy block | Hide markers that would imply unavailable authority. | Trust/policy markers may remain if backed and labeled. | Fold summaries must preserve trust/policy hidden state. | Search scope labels policy omissions. | Policy-hidden diagnostics disclose omitted scope when known. |
| Provider unavailable | Hide provider-owned markers. | Show only source-owned current markers. | Mark unknown provider-derived hidden state. | Search hits unavailable unless local source has results. | Diagnostics unavailable/stale with repair route. |

Suppression is not failure by itself. It is conforming when the backing
record names the trigger, says what still works, names what changed, and
points to the replacement route.

## 7. Folded Regions

Fold summaries are the highest-obligation summary in this contract
because they hide source text in the canonical viewport.

Rules:

1. A collapsed range must expose hidden line count.
2. If structure labels are available, the label must cite the folding
   model or symbol/breadcrumb source and inherit its freshness.
3. Hidden diagnostics, conflicts, trust/policy warnings, review
   anchors, breakpoints, search hits, and staged hunks must be named in
   the fold summary or in an adjacent detail route.
4. Error and blocking states outrank warnings, search hits, ordinary
   changes, blame, coverage, or ambient metadata.
5. The fold summary may aggregate lower-priority facts, but the detail
   route preserves counts by family.
6. A stale fold model may keep the range collapsed only if the summary
   says the fold model is stale and exposes an expand/recompute route.
7. Unfolding to reveal a hidden diagnostic, search hit, or review
   marker must preserve back/forward and focus continuity.

Forbidden behaviors:

- a folded range with hidden diagnostics appearing visually clean;
- hiding an error count because the minimap or overview ruler also has
  an error tick;
- using hover as the only way to discover hidden state;
- dropping hidden review or trust markers because the fold was created
  before those markers arrived; and
- reusing an old fold summary after the buffer snapshot changed without
  a stale label.

## 8. Off-Screen Diagnostics

Diagnostic summaries may appear in the overview ruler, minimap, status
layer, fold summary, or line detail. They remain projections of
diagnostic-cluster truth.

Rules:

1. The Problems/detail route and next/previous diagnostic commands are
   the canonical discovery path. Summary ticks are accelerators.
2. Off-screen diagnostic counts distinguish severity, freshness, and
   scope: current, stale, imported, partial, superseded, or unavailable.
3. A diagnostic in a folded range contributes to both the fold summary
   and the diagnostic summary, but both quote the same diagnostic
   cluster ref and must not disagree on severity or freshness.
4. A diagnostic outside the loaded or policy-visible scope must be
   disclosed as omitted/hidden when the source can know it; unknown
   scope must be labeled unknown rather than inferred clean.
5. Summary activation opens diagnostic detail or reveals the line; it
   does not run a code action directly.

## 9. Search and Review Markers

Search-hit ticks are tied to one active search or find session. Review
markers are tied to review anchors. Both can coexist with diagnostics
and folds only as projections.

Rules:

1. A search tick carries search-session ref, query scope, match count
   truth, and freshness. In-file find, workspace search, and semantic
   search do not share one unlabeled tick set.
2. Chunked, partial, stale, or approximate search may show ticks only
   with that label and a route to the result list or query status.
3. Review markers carry anchor freshness: exact, remapped, stale,
   unmapped, imported, or policy-hidden.
4. If a folded region contains both search hits and review markers,
   the fold detail groups them by family and names counts separately.
5. Summary activation must not skip review permissions, apply fixes, or
   mutate state. It opens review/search detail or reveals the source.

## 10. Interaction and Focus

Summary interactions are navigation aids. They must preserve editor
focus expectations.

1. Pointer clicks and drags in minimap or overview open or scroll the
   source viewport only when the source target is still resolvable.
2. Keyboard equivalents exist for next/previous marker family, reveal
   current summary marker, open summary detail, fold/unfold, and open
   Problems/search/review/debug detail.
3. After a summary jump, back/forward history includes the prior source
   location.
4. Focus returns to the source viewport or the explicit detail surface,
   never to a hidden summary wrapper.
5. Hover may enrich a marker label, but it is not the only route to
   inspect or act.
6. Summary surfaces do not run fix, refactor, approve, or write actions
   directly.

## 11. Accessibility and Motion

1. Color is never the only channel for marker family, severity,
   freshness, or hidden-state disclosure.
2. High-contrast and forced-colors modes preserve text, shape, border,
   position, glyph, or accessible-name channels.
3. Screen-reader flows announce primary state first, then required
   secondary states, then hidden or collapsed counts.
4. Reduced-motion mode removes animated minimap repaint, pulse, shimmer,
   smooth-scroll-only state changes, and moving counters. Static labels
   and reserved layout remain.
5. At high zoom, source text, context, and recovery-critical status win
   before minimap or overview surfaces.
6. Summary surfaces hidden for accessibility must say which canonical
   route still contains the information.

## 12. Fallback Language

Surfaces use controlled fallback language so product, docs, support,
and accessibility output agree.

| State | Required visible label | Required accessible/detail sentence |
| --- | --- | --- |
| Hidden by width | `Summary hidden` | `Editor summary is hidden because this editor group is narrow. Source view, line detail, Problems, and Search still contain the markers.` |
| Hidden by large-file mode | `Summary reduced` | `Large-file mode reduced overview rendering to keep editing responsive. Source view, search, and diagnostic detail remain available.` |
| Hidden for assistive technology | `Summary moved to details` | `Visual summary is hidden for the current accessibility posture. Use line detail, Problems, Search, or commands for the same markers.` |
| Stale | `Summary stale` | `Summary markers are from an older source epoch. Refresh or use the canonical detail before relying on exact positions.` |
| Partial | `Summary partial` | `Summary covers only the loaded or admitted scope. Omitted scope is listed in detail when known.` |
| Unavailable | `Summary unavailable` | `Summary cannot render because its source or renderer budget is unavailable. Canonical source routes remain available where the source exists.` |
| User disabled | `Summary disabled` | `Editor summary is disabled by setting or policy. Source view and canonical marker routes remain available.` |

These labels may be localized, but the state, reason, and canonical
route references must survive.

## 13. Export and Support

Support/export records preserve:

- editor view and buffer refs;
- summary surface visibility decisions;
- active suppression or downgrade reasons;
- source authority refs and freshness/completeness labels;
- marker counts by family and severity;
- fallback language class and replacement routes;
- accessibility posture that influenced the decision; and
- assertions that critical markers were not summary-only.

Exports must not include raw source text, raw file paths, raw provider
payloads, raw search queries when policy redacts them, or credentials.

## 14. Fixture and Schema Requirements

Records using
[`/schemas/editor/viewport_summary.schema.json`](../../schemas/editor/viewport_summary.schema.json)
must satisfy these invariants:

1. Every marker references one canonical source family and source ref.
2. Every surface declares an appearance decision, visibility class,
   suppression reasons, freshness, and fallback language.
3. Hidden, stale, partial, suppressed, or unavailable surfaces have a
   replacement route and a non-generic reason.
4. Fold summaries preserve hidden high-severity, search, review,
   conflict, trust, and change state.
5. Search-hit ticks preserve query scope and count truth.
6. Diagnostic summaries preserve diagnostic-cluster freshness and
   semantic-layer labels.
7. Accessibility, keyboard, and support/export projections preserve the
   same state names.
8. `critical_state_not_summary_only` is true for every case.

## 15. Acceptance Checklist

A reviewer can accept a viewport-summary implementation or fixture when:

1. Minimap, overview ruler, fold summary, search ticks, and diagnostic
   summaries all project from canonical source refs.
2. No warning, error, fold, search hit, review marker, breakpoint,
   conflict, trust warning, or generated/read-only state exists only in
   a viewport summary.
3. Large-file, mixed zoom, accessibility, reduced-motion, low-
   confidence, degraded, and constrained-width suppressions are explicit
   and route to replacement detail.
4. Folded regions disclose hidden high-signal state before lower-value
   metadata.
5. Stale, partial, imported, approximate, or unavailable source truth is
   labeled and cannot render as current exact state.
6. Pointer, keyboard, screen-reader, high-contrast, and support/export
   paths preserve the same facts.
