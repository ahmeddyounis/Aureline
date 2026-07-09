# Relation strips and sync-pending pills

Status: implemented (M05-982, batch B116)

This lane narrows two components frozen in the
[M5 work-item component matrix](m5_work_item_component_matrix.md) — the
`relation_strip` and the `sync_pending_pill` — into one implemented, export-safe
packet with two co-equal control vectors. Together they keep code / review / test /
incident context and unsent local state compact but truthful in list and side-rail
surfaces.

- Boundary schema: [`schemas/ui/m5-relation-strip-sync-pending-controls.schema.json`](../../schemas/ui/m5-relation-strip-sync-pending-controls.schema.json)
- Per-component contracts: [`schemas/ui/m5-relation-strip.schema.json`](../../schemas/ui/m5-relation-strip.schema.json), [`schemas/ui/m5-sync-pending-pill.schema.json`](../../schemas/ui/m5-sync-pending-pill.schema.json)
- Rust module: `crates/aureline-provider/src/implement_relation_strips_and_sync_pending_pills_with_linked_context_stale_labeling_and_retry_or_export_continuity`
- Headless emitter: `cargo run -p aureline-provider --bin aureline_relation_strip_sync_pending_primitive -- <subcommand>`
- Release proof: [`artifacts/release/m5-relation-strip-sync-pending-proof/`](../../artifacts/release/m5-relation-strip-sync-pending-proof/)
- Scenario fixtures: [`fixtures/ui/m5-relation-strip-sync-pending-controls/`](../../fixtures/ui/m5-relation-strip-sync-pending-controls/)

## Relation strip

A `RelationStrip` summarizes the linked engineering context attached to a work item,
naming **each** linked context by kind and reference rather than collapsing several
links into a single vague `Linked` label. It reuses the frozen relation kinds
(`linked_branch`, `linked_pull_request`, `linked_review`, `linked_test_run`,
`linked_incident`, `unmapped_relation`).

Each relation carries a **derived** health class — never asserted:

| condition | derived `health_class` | note required |
| --- | --- | --- |
| unmapped relation kind | `unmapped` | yes |
| target not reachable | `broken` | yes |
| reachable but out of date | `stale` | yes |
| reachable and current | `current` | no |

Every relation offers metadata-safe copy/open actions (`copy_reference` and
`open_relation` are mandatory). A strip that sets
`collapses_into_generic_linked_label` or that reuses the same reference label for two
relations fails validation with `relations_collapsed_into_vague_label` — this is the
teeth behind the acceptance criterion that relation strips no longer collapse multiple
linked contexts into a vague `Linked` label.

## Sync-pending pill

A `SyncPendingPill` discloses what local change is pending (`pending_comment`,
`pending_transition`, `pending_link`, `pending_field_edit`, or `pending_create`), the
last sync attempt, and a retry-or-export recovery action. It reuses the frozen
local-versus-provider states and derives its sync-recovery class:

| condition | derived `recovery_class` | consequences |
| --- | --- | --- |
| policy-blocked | `policy_blocked` | policy-block note required; never confirmed |
| synced with provider | `provider_confirmed` | the only state that may claim confirmed |
| publish failed | `recoverable_failure` | retry-or-export action + last attempt required |
| provider offline (unsynced) | `offline_held` | retry-or-export action + last attempt required |
| any other unsynced state | `pending_publish` | retry-or-export action + last attempt required |

A pending, failed, or offline-held pill must read **visibly differently** from a
provider-confirmed state (`distinct_from_confirmed_style`) and can never claim
`claims_provider_confirmed`; misrepresenting either fails with
`sync_state_misrepresented` / `not_visibly_distinct_from_confirmed`. A recoverable pill
must offer at least one of `retry_publish` / `export_packet`, so the change stays
recoverable when publish fails or the provider is offline — the teeth behind the
acceptance criterion that sync-pending state remains recoverable.

## Coverage and guardrails

The canonical packet covers every relation health class, every sync-recovery class,
and every pending change type; validation enforces that coverage. Generic ticket/task
wording is rejected (`generic_ticket_wording_used`), and raw work-item bodies, pasted
paths, credentials, and private endpoints never cross the export boundary
(`raw_boundary_material_in_export`). Stale proof automatically narrows the lane.
