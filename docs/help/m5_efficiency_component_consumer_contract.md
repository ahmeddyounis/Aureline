# M5 adaptive-efficiency component consumer contract (M05-1066)

This is the consumer-adoption lane over the frozen M5 adaptive-efficiency
component matrix. It proves that the eight reusable adaptive-efficiency
component families are reused as **primitives** across every M5 surface that
claims adaptive efficiency, rather than being reinvented as per-surface
low-power prose.

- Boundary schema: `schemas/ui/m5-efficiency-component-consumer.schema.json`
- Release proof: `artifacts/release/m5-efficiency-component-consumer-proof/`
- Fixtures: `fixtures/ui/m5-efficiency-component-consumers/`
- Frozen matrix: `artifacts/release/m5-efficiency-components-proof/support_export.json`

## The eight canonical component families

Every consumer row points back to exactly one frozen family and the one
canonical controls contract (schema + doc + release-proof artifact) its family
group belongs to:

| Family | Controls lane |
| --- | --- |
| `power_state_indicator` | `power_throttle` |
| `throttled_subsystem_row` | `power_throttle` |
| `background_work_row` | `background_work` |
| `background_work_banner` | `background_work` |
| `per_workspace_override_sheet` | `override_policy` |
| `override_policy_note_row` | `override_policy` |
| `resume_summary_card` | `resume_continuity` |
| `stale_result_continuity_note` | `resume_continuity` |

A surface may not fork the controls lane for a family: every consumer of a
family reuses the same lane's canonical schema, doc, and release-proof
artifact.

## The five claimed consumer classes

1. **`shell_status_activity`** — shell status bar, activity center, background-work tray.
2. **`work_content_surface`** — notebook, preview, pipeline, graph explorer. These
   are the surfaces whose work actually slows or pauses under pressure, so their
   rows must preserve the slowed-versus-paused and what-still-works truth.
3. **`docs_browser_companion`** — docs/browser handoff and companion-adjacent surfaces.
4. **`incident_diagnostics`** — incident console and diagnostics panel.
5. **`support_export_help`** — support/export replay and the Help/About reference
   surface (the AC2 lane).

## Preserved truth pillars (one vocabulary everywhere)

Every consumer keeps the identical controlled label families — the track
invariant — rather than surface-local phrasing:

`source_of_change`, `active_efficiency_state`, `slowed_versus_paused_work`,
`what_still_works`, `override_availability`, `policy_owner`,
`resumed_work_backlog`, `stale_result_continuity`, `next_safe_action`.

Every consumer also keeps the frozen `M5EfficiencyWorkDisposition` vocabulary
visible — `running_full`, `slowed`, `paused`, `policy_blocked`,
`override_available`, `override_blocked`, `resuming`, `stale_result_shown`,
`not_evaluated` — so the same constrained *or recovered* state renders with one
vocabulary and one component family across every claimed consumer (AC1).

## Narrowing is disclosed, never silent

A consumer may narrow authority (`read_only`, `inspect_only`, `override_gated`,
`export_only`, `policy_blocked`) but never rename or drop governed state. A
narrower consumer discloses the reduction with a reduced-capability banner whose
`capability_state` matches its `authority_mode`, and carries a desktop /
companion / browser / support note whenever it punts to another surface. The
banner label must be precise: a label that collapses to a generic phrase (`low
power`, `power saver`, `battery saver`, `throttled`, `slowed down`, …) is
rejected.

## Guardrails (all false on every row)

- `collapses_pressure_sources_into_generic_warning` — battery saver, thermal
  pressure, user-selected low-power mode, and policy cap are never collapsed into
  one generic warning.
- `hides_paused_work_behind_toast_only` — paused work is never hidden behind
  toast-only messaging.
- `presents_override_available_when_policy_blocks` — an override is never shown as
  available when policy blocks it.
- `clears_stale_context_on_resume` — stale-result context is never cleared merely
  because background work resumed.

## Acceptance criteria

- **AC1** — The same constrained or recovered state renders with one vocabulary
  and component family across every claimed M5 consumer. Enforced by the label,
  work-disposition, canonical-family, and controls-lane-stability checks.
- **AC2** — Help/support/export consumers no longer need bespoke prose to explain
  adaptive-efficiency behavior for different M5 lanes. Enforced by the docs/help
  reference check: the Help/About surface references the canonical families
  rather than cloning local efficiency vocabulary.

Regenerate the release proof and fixtures with:

```
GEN_EFFICIENCY_CONSUMER_ARTIFACTS=1 cargo test -p aureline-shell generate_artifacts
```
