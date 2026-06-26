# M5 Accessibility-Bridge, Live-Announcement, Focus-Return, and Non-Visual Dynamic-Surface Matrix

This document is the contract for the frozen M5 matrix that names the canonical
Aureline assistive-technology object model for custom-rendered dynamic surfaces.
The matrix is the single M5 source of truth for whether a claimed dynamic surface
may publish a screen-reader-complete or keyboard-complete claim: shell, editor,
terminal, notebook, data, review, help, and presentation surfaces ingest the
checked-in packet rather than maintaining per-surface ad hoc assistive behavior.

- Record kind: `freeze_m5_accessibility_bridge_live_announcement_focus_return_and_non_visual_dynamic_surface_matrix`
- Schema: [`schemas/a11y/m5-dynamic-surface-a11y.schema.json`](../../schemas/a11y/m5-dynamic-surface-a11y.schema.json)
- Canonical support export: [`artifacts/a11y/m5-dynamic-surfaces/support_export.json`](../../artifacts/a11y/m5-dynamic-surfaces/support_export.json)
- Governance summary artifact: [`artifacts/a11y/m5-dynamic-a11y-governance.md`](../../artifacts/a11y/m5-dynamic-a11y-governance.md)
- Fixtures: [`fixtures/a11y/m5-dynamic-surfaces/`](../../fixtures/a11y/m5-dynamic-surfaces/)
- Producer: `aureline_shell::freeze_the_m5_accessibility_bridge_live_announcement_focus_return_and_non_visual_dynamic_surface_matrix::current_stable_m5_dynamic_surface_a11y_matrix_export`
- Headless emitter: `aureline_shell_m5_dynamic_surface_a11y_matrix`

## Governed objects

| Object | Qualification | Owner | State vocabularies | Source contract |
| --- | --- | --- | --- | --- |
| `accessibility_surface_descriptor` | Stable | Accessibility owner | semantic_role_class / non_visual_fidelity / bridge_state | [`docs/accessibility/accessibility_tree_contract.md`](../accessibility/accessibility_tree_contract.md) |
| `screen_reader_label_model` | Stable | Accessibility owner | semantic_role_class / fallback_durability / bridge_state | [`docs/accessibility/accessibility_tree_contract.md`](../accessibility/accessibility_tree_contract.md) |
| `live_announcement_class` | Stable | Accessibility owner | announcement_politeness / coalescing_strategy / fallback_durability | [`docs/accessibility/screen_reader_and_live_region_contract.md`](../accessibility/screen_reader_and_live_region_contract.md) |
| `focus_return_contract` | Stable | Accessibility owner | focus_return_disposition / fallback_durability | [`docs/accessibility/focus_zoom_and_pointer_independence_contract.md`](../accessibility/focus_zoom_and_pointer_independence_contract.md) |
| `dense_surface_non_visual_summary` | Stable | Accessibility owner | non_visual_fidelity / coalescing_strategy / semantic_role_class | [`docs/accessibility/collection_announcement_contract.md`](../accessibility/collection_announcement_contract.md) |
| `bridge_diagnostics_packet` | Beta | Accessibility platform owner | bridge_state / non_visual_fidelity | [`docs/accessibility/m1_shell_bridge.md`](../accessibility/m1_shell_bridge.md) |

Each object row binds a qualification class to its required fields, the controlled
state vocabularies it carries, the concrete vocabulary tokens it admits, its
evidence requirement, the assistive-tech proof packet refs that keep it current,
its downgrade triggers, its rollback posture, its source contracts, and the
consumer surfaces that must project its qualification truth. An object kind's
required state vocabularies must appear in `state_vocabularies`, and a declared
vocabulary must carry concrete tokens while an undeclared vocabulary must carry
none — so the matrix is exact about which assistive-tech truth each object speaks.

## Controlled vocabulary

The matrix freezes one self-describing `vocabulary_set` block, mapped onto the
canonical tokens already owned by the screen-reader/live-region, accessibility-tree,
focus/zoom/pointer-independence, and collection-announcement contracts rather than
minting parallel tokens:

- **Announcement politeness** — `polite`, `assertive`, `silent`. Mirrors the
  `live_region_channel` vocabulary; `assertive` is reserved for safety-critical
  state and `silent` is a disclosed non-announcement, never a dropped meaning.
- **Coalescing strategy** — `none`, `dedupe_same_meaning`,
  `last_meaning_wins_with_count`, `start_and_terminal_only`, `focused_surface_only`.
  Live regions coalesce rather than spam.
- **Fallback durability** — `immediate`, `coalesced`, `on_focus`,
  `durable_surface_only`, `not_delivered_silent`. `durable_surface_only` is the
  durable fallback; a blocking state is delivered `immediate` and never depends on
  a transient live region alone.
- **Non-visual fidelity** — `full_accessible`, `degraded_accessible`,
  `summary_only`, `inspect_only`, `unsupported_blocked`, `not_applicable`. Mirrors
  the accessibility-tree `support_state` vocabulary; a surface never overstates its
  non-visual coverage.
- **Bridge state** — `bridged_active`, `partial`, `stale`, `unavailable`.
  `bridged_active` is the proven, connected OS accessibility bridge; `partial`,
  `stale`, and `unavailable` are the disclosed narrowing states that auto-narrow a
  claimed surface.
- **Focus-return disposition** — `returned_exact`,
  `returned_nearest_safe_ancestor`, `returned_current_batch_or_detail_owner`,
  `returned_placeholder_announced`, `focus_loss_denied`,
  `focus_not_applicable_non_interactive`. Mirrors the `focus_return_state`
  vocabulary; every disposition returns focus to a real owner, so focus never
  teleports or vanishes.
- **Semantic role class** — `landmark_region`, `structure_group`,
  `interactive_control`, `text_document`, `status_region`, `live_log_region`,
  `data_grid_cell`, `notebook_cell`. Groups the accessibility-tree node taxonomy
  into the broad structural classes a dynamic surface must speak.

The `vocabulary_set` block must match these canonical token lists exactly; any
drift fails validation with `vocabulary_set_drift`.

## Track invariant

Non-visual truth stays first-class. The `conformance_review` block encodes the lane
invariants as hard flags — all must hold for the matrix to validate:

- `custom_surfaces_expose_semantic_structure` — custom-rendered surfaces expose
  semantic structure, not visual-only state.
- `focus_never_teleports_or_vanishes_on_async_update` — focus never teleports to an
  unrelated surface or vanishes on an async update.
- `live_regions_coalesce_rather_than_spam` — live regions coalesce rather than spam.
- `dynamic_state_changes_announce_meaning_not_repaint_noise` — dynamic state changes
  announce meaning, not repaint noise.
- `no_visual_only_state_or_pointer_hover_dependence` — no surface depends on
  visual-only state or pointer hover.
- `dense_surfaces_expose_non_visual_summaries` — dense surfaces expose non-visual
  summaries.
- `durable_fallbacks_present_for_blocking_states` — durable fallbacks are present for
  blocking states.
- `bridge_degradation_disclosed_not_hidden` — bridge degradation is disclosed, never
  hidden.
- `one_bridge_aware_contract_not_per_surface_adhoc` — every surface resolves to one
  bridge-aware contract, not per-surface ad hoc behavior.
- `claimed_rows_auto_narrow_when_bridge_or_proof_stale`,
  `downgrade_narrows_instead_of_hides`, and
  `stale_or_underqualified_blocks_promotion`.

## Consumer projection and release posture

`consumer_projection` binds every claimed dynamic surface to the shared object
model: shell, editor, terminal, notebook, data grid, review, help, presentation,
support export, and AI surfaces all read the same packet, and unqualified surfaces
are visibly labeled when not covered. The `release_posture` block binds the
supporting release packet (`evidence:dynamic-surface-a11y-release-packet:m5`) and the
mirror/offline packet (`evidence:dynamic-surface-a11y-mirror-offline-packet:m5`),
requires support/export and mirror/offline parity for every object, and — via
`stable_promotion_blocks_without_mapped_proof` — blocks Stable promotion of any
claimed dynamic surface that lacks a mapped assistive-tech proof row or current
matrix entry.

## Downgrade and freshness

`proof_freshness` carries the SLO (168 hours) and the last-refresh timestamp; when
proof goes stale `auto_narrow_on_stale` narrows the affected object. The supported
downgrade triggers are `proof_stale`, `bridge_unavailable`, `bridge_partial_or_stale`,
`focus_teleported`, `focus_lost`, `live_region_spam`, `announcement_meaning_lost`,
`non_visual_fidelity_lost`, `label_or_role_drift`, `pointer_or_hover_dependence`,
`policy_blocked`, and `upstream_dependency_narrowed`. The
[fixtures](../../fixtures/a11y/m5-dynamic-surfaces/) show a held bridge-diagnostics
packet (after the OS bridge goes unavailable) and a preview-narrowed dense-surface
non-visual summary; both remain valid packets because narrowing is explicit, not
hidden.

Stable promotion of any claimed M5 dynamic surface that maps to a governed object
fails while that object lacks a current matrix entry and mapped proof packet:
`current_stable_m5_dynamic_surface_a11y_matrix_export` revalidates the checked-in
packet, and a missing object, drifted vocabulary, missing proof ref, or unsatisfied
conformance invariant blocks the packet.

## Boundary

Raw provider payloads, credentials, secret material, screenshots, and untranslated
free-text prose never cross this boundary. The packet carries only metadata,
qualification truth, controlled-vocabulary tokens, and contract references.
