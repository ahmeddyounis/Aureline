# Fixture: empty state — filter-narrowed to zero in command palette

## Scenario

The user opens the command palette and types a query that
narrows the visible set to zero. The palette's empty state
MUST NOT collapse to a spinner, MUST NOT read as "No results",
and MUST preserve the filter chip so the user understands why
the set is empty.

## Row bindings

- **Empty-state row.** `empty:filter_narrowed_to_zero` per
  [`failure_tier_matrix.yaml`](../../../artifacts/ux/failure_tier_matrix.yaml)
  § `empty_state_rows`.
- **Startup-state intersection.** None.
- **User-impact label class.** `no_durable_work_yet` on the
  palette surface (the user has nothing to act on); the
  underlying workspace is unaffected.
- **Truth class.** `derived_indexed_truth` for the palette index.
- **Degraded-state token.** `Partial` when a partial_index is
  covering the query scope; otherwise no token.

## Required axes rendered

- **`area_purpose`.** "Command palette: commands, files, symbols,
  and recent actions."
- **`emptiness_cause`.** `filter_narrowed_to_zero` — the query
  plus the active scope filters exclude every row.
- **`next_best_action`.** `remove_from_recents` (clear recent
  filter), `review_archetype_match` (narrow to a different scope
  chip), `open_minimal` (dismiss the palette and return to the
  editor).

## Preserved / narrowed / next-safe action

- **Preserved.** Palette input remains editable; scope chips and
  recent history remain reachable; the editor behind the palette
  stays interactive on dismiss.
- **Narrowed.** The query + scope filter excludes every row.
  The palette names the **excluded scope** ("No commands match
  in *Scope: workspace*; showing 0 of 0 for this scope") rather
  than an unqualified "No results".
- **Next-safe action hooks.** `remove_from_recents`,
  `review_archetype_match`, `open_minimal`.

## Placement

- **Failure tier.** None (filter-narrowed empty by design).
- **Delivery surface.** Owning surface's primary content plane
  (palette body).
- **Interruptibility tier.** `tier_ambient`.
- **Focus.** Stays on the palette input; keyboard shortcut to
  clear the filter is announced.

## Recovery / support / measurement

- **Recovery-ladder rung.** `rung.none_required`.
- **Support-packet family.** `empty_state_audit_evidence`.
- **Journey-trace class.** `shell_open` (palette interactions).

## Accessibility

- Screen-reader announces the filter state ("0 results in
  *Scope: workspace* — clear filter to widen").
- Keyboard clears the filter via the scope-chip close affordance
  without leaving the palette.

## Forbidden copy on this path

- "No results"
- "Nothing found"
- "Try again"

## Expected observable outcomes

- The palette does not collapse to a spinner when the true state
  is filter-narrowed zero (per the forbidden loading-pattern
  `indeterminate_spinner_in_place_of_cause`).
- The row id (`empty:filter_narrowed_to_zero`) is preserved on
  later timeline / event-lineage emissions.
- `overclaims_readiness = false` is asserted on the rendered
  empty view.

## Fixture fields (seed)

```yaml
__fixture__:
  name: empty_filter_narrowed
  taxonomy_row: empty:filter_narrowed_to_zero
  doc_section: docs/ux/state_and_recovery_taxonomy.md#5
  matrix_row_ref: empty:filter_narrowed_to_zero
running_build_identity_ref: build-identity-seed-state-empty-filter
overclaims_readiness: false
```
