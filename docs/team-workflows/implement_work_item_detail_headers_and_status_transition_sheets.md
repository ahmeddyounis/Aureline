# Work-item detail headers and status-transition sheets

Status: implemented (M05-983, batch B116)

This lane narrows two components frozen in the
[M5 work-item component matrix](m5_work_item_component_matrix.md) — the
`work_item_detail_header` and the `status_transition_sheet` — into one implemented,
export-safe packet with two co-equal control vectors. Together they make the durable
work-item detail surface and every publish-capable transition explicit about identity,
provider boundary, side effects, permission scope, and publish-later continuity.

- Boundary schema: [`schemas/ui/m5-work-item-detail-header-status-transition-controls.schema.json`](../../schemas/ui/m5-work-item-detail-header-status-transition-controls.schema.json)
- Per-component contracts: [`schemas/ui/m5-work-item-detail-header.schema.json`](../../schemas/ui/m5-work-item-detail-header.schema.json), [`schemas/ui/m5-status-transition-sheet.schema.json`](../../schemas/ui/m5-status-transition-sheet.schema.json)
- Rust module: `crates/aureline-provider/src/implement_work_item_detail_headers_and_status_transition_sheets_with_provider_boundary_side_effect_permission_scope_and_confirm_export_cancel_truth`
- Headless emitter: `cargo run -p aureline-provider --bin aureline_work_item_detail_header_status_transition_primitive -- <subcommand>`
- Release proof: [`artifacts/release/m5-work-item-detail-header-status-transition-proof/`](../../artifacts/release/m5-work-item-detail-header-status-transition-proof/)
- Scenario fixtures: [`fixtures/ui/m5-work-item-detail-header-status-transition-controls/`](../../fixtures/ui/m5-work-item-detail-header-status-transition-controls/)

## Work-item detail header

A `DetailHeader` states the provider/project space, canonical id, title, work-item
kind, current state, assignee/owner, and always offers an open-external escape hatch
(`open_external` and `copy_canonical_id` are mandatory actions). Its **write scope** and
**freshness class** are both *derived* from the frozen provider authority and
local-versus-provider state — never asserted:

| provider authority | derived `write_scope` |
| --- | --- |
| policy-pinned | `policy_blocked_write` (policy-block note required) |
| mirrored read-only / imported snapshot | `read_only_mirror` |
| local draft / unlinked local | `local_draft_only` |
| provider-owned | `provider_writable` |

| condition | derived `freshness_class` |
| --- | --- |
| freshness unknown | `unknown_freshness` |
| local draft / unlinked local | `local_only` |
| imported snapshot | `stale_snapshot` |
| provider-backed, reference current | `live_synced` |
| provider-backed, reference out of date | `stale_snapshot` |

A local draft or unlinked local item is **not provider-backed**; declaring
`claims_provider_backed` on it fails with
`local_draft_misrepresented_as_provider_backed` — this is the teeth behind the
acceptance criterion that detail surfaces preserve local draft state and do not imply
external mutation before confirmation. Any non-writable scope requires a
`write_scope_note`, and any non-live freshness requires a `freshness_note`.

## Status-transition sheet

A `StatusTransitionSheet` previews, before any publish, what will change
(`comment_mutation`, `state_mutation`, `assignment_mutation`, `link_mutation`, or
`field_mutation`), the linked branch/review context, the notification side effects, and
the permission scope that can authorize the change. It reuses the frozen transition
effects and derives its **publish class**:

| condition | derived `publish_class` | publishes externally |
| --- | --- | --- |
| policy-blocked | `policy_blocked_transition` (policy-block note required) | no |
| blocked transition effect | `blocked_needs_permission` | no |
| local-only transition effect | `local_draft_only` | no |
| open-in-provider effect | `opens_in_provider` | yes |
| publish-now / comment / status effect | `publishes_to_provider` | yes |

`implies_external_mutation` must equal the derived `publishes_externally`; a local-only
transition that claims external mutation (or an external transition that denies it)
fails with `external_mutation_misrepresented` — the teeth behind the acceptance
criterion that a local transition never implies external mutation before confirmation.
Every sheet must preview what will change (`side_effect_preview_label`), name who can
authorize it (`permission_scope` + `permission_scope_note`), and offer
confirm/export/cancel behavior (`confirm`, `export_packet`, and `cancel` are mandatory)
with a metadata-safe `export_fallback_note` — the teeth behind the acceptance criterion
that users can see what will change, who can authorize it, and what fallback exists when
publish cannot proceed. An externally-publishing transition additionally requires a
`notification_side_effect_note`.

## Coverage and guardrails

The canonical packet covers every header write scope, every header freshness class,
every transition publish class, every transition mutation kind, and every permission
scope class; validation enforces that coverage. Generic ticket/task wording is rejected
(`generic_ticket_wording_used`), and raw work-item bodies, pasted paths, credentials,
and private endpoints never cross the export boundary
(`raw_boundary_material_in_export`). Stale proof automatically narrows the lane.
