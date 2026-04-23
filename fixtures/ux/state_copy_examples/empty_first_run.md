# Fixture: empty state — first-run Start Center

## Scenario

Fresh install on a previously-unused device. The Start Center
lands in the no-prior-profile path with no recent-work rows and
no pending restore. The surface is **empty by design**: the user
has not yet chosen an entry verb. Copy, chrome, and focus MUST
NOT imply that a workspace, index, or session is ready.

## Row bindings

- **Empty-state row.** `empty:no_work_started` per
  [`failure_tier_matrix.yaml`](../../../artifacts/ux/failure_tier_matrix.yaml)
  § `empty_state_rows`.
- **Startup-state intersection.** `startup_state:first_run` per
  [`entry_restore_truth_audit.md`](../../../docs/ux/entry_restore_truth_audit.md)
  §6.1 and the copy-review row
  `ux:startup-state:first_run`.
- **User-impact label class.** `no_durable_work_yet`.
- **Truth class.** No row is rendered; the surface is the Start
  Center chrome, not a truth row.
- **Degraded-state token.** None (first-run is not a degraded
  posture).

## Required axes rendered

- **`area_purpose`.** "Start Center: open, clone, import,
  restore, or pick from recent work."
- **`emptiness_cause`.** `no_work_started` — named verbatim in
  the surface's empty-state copy as *no workspace yet*.
- **`next_best_action`.** Distinct Open / Clone / Import /
  Restore / Recent work entry verbs (the `set_up_later` and
  `open_minimal` hooks are advertised verbatim when the user
  declines the opt-in; see
  `startup_state_copy_review.yaml#copy_review_rows[0]`).

## Preserved / narrowed / next-safe action

- **Preserved.** Keyboard-reachable chrome (menu bar, command
  palette, activity rail, breadcrumbs); local-only paths;
  `set_up_later` / `open_minimal` hooks.
- **Narrowed.** `no_durable_edits_yet` — the user has not yet
  reached a surface that would persist work. No banner claims
  an index is ready.
- **Next-safe action hooks.** `review_archetype_match`,
  `set_up_later`, `open_minimal`.

## Placement

- **Failure tier.** None (empty by design).
- **Delivery surface.** Owning surface's primary content plane
  (Start Center body).
- **Interruptibility tier.** `tier_ambient`.
- **Focus.** First focusable element is the entry verb row, not
  a marketing banner.

## Recovery / support / measurement

- **Recovery-ladder rung.** `rung.restricted_mode_fallback` (if
  `continue_in_restricted_mode` is chosen).
- **Support-packet family.** `first_run_evidence`.
- **Journey-trace class.** `startup_to_first_useful_chrome`,
  `startup_to_first_paint`.
- **Protected metrics.** `ff.warm_start_to_first_paint`,
  `ff.first_paint`.

## Accessibility

- Entry-verb row order: Open, Clone, Import, Restore, Recent
  work — announced in this order by assistive technology; no
  verb collapses into "Get started".
- Focus order: entry verbs → settings / opt-in → help.
- Reduced-motion and high-contrast themes carry the same rows.

## Forbidden copy on this path

- "Workspace ready"
- "All extensions loaded"
- "Index complete"
- "Get started"
- "Nothing here yet"
- "Start your journey"

## Expected observable outcomes

- Rendering this fixture MUST NOT mint a private state token,
  collapse entry verbs, or imply that any readiness chrome is
  satisfied.
- The row id (`empty:no_work_started`) is preserved on any
  support export and on any later timeline / event-lineage
  emission.
- `overclaims_readiness = false` is asserted on the rendered
  empty view.

## Fixture fields (seed)

```yaml
__fixture__:
  name: empty_first_run
  taxonomy_row: empty:no_work_started
  startup_state_intersection: startup_state:first_run
  doc_section: docs/ux/state_and_recovery_taxonomy.md#5
  matrix_row_ref: empty:no_work_started
running_build_identity_ref: build-identity-seed-state-empty-first-run
overclaims_readiness: false
```
