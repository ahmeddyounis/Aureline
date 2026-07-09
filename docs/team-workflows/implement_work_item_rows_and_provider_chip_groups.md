# Work-item rows and provider chip groups (M05-981)

This lane implements two components frozen in the
[M5 work-item component matrix](m5_work_item_component_matrix.md) — the
`work_item_row` and the `provider_chip_group` — into one export-safe packet with
two co-equal control vectors. Together they make list-level work-item identity and
provider authority explicit **before** a user opens detail or publish flows.

- Crate module:
  `crates/aureline-provider/src/implement_work_item_rows_and_provider_chip_groups_with_canonical_id_owner_state_freshness_and_write_scope_truth/`
- Boundary schema:
  [`schemas/ui/m5-work-item-row-provider-chip-controls.schema.json`](../../schemas/ui/m5-work-item-row-provider-chip-controls.schema.json)
- Release proof:
  `artifacts/release/m5-work-item-row-provider-chip-proof/`
- Scenario fixtures:
  `fixtures/ui/m5-work-item-row-provider-chip-controls/`
- Headless emitter:
  `cargo run -p aureline-provider --bin aureline_work_item_row_provider_chip_primitive -- <subcommand>`

## Goal

Make list-level work-item identity and provider authority explicit before users
open detail or publish flows.

## Work-item row

A `WorkItemRow` always names its canonical id, title, work-item kind,
assignee/owner, priority or severity, and linked-change count, and offers
keyboard-complete default actions. Its **state-authority class is derived**, never
asserted, from the provider authority and the local-versus-provider state:

| Condition | Derived class |
| --- | --- |
| provider authority is `policy_pinned` | `blocked_capability` |
| local state is queued / deferred / failed / conflict-held | `publish_pending` |
| authority is `local_draft` / `unlinked_local`, or state is `local_only_draft` | `local_only_draft` |
| authority is `mirrored_read_only` / `imported_snapshot` | `snapshot_only` |
| otherwise (`provider_owned` + `synced_with_provider`) | `provider_authoritative` |

Only a `provider_authoritative` row may claim provider-authoritative state, so a
local-only draft or a policy-blocked item can never read as provider-authoritative
in a list surface (validation: `state_authority_misrepresented`). The canonical id
is always visible and copyable (`canonical_id_not_copyable`) and `copy_canonical_id`
and `open_detail` are mandatory default actions (`default_actions_incomplete`).

Non-synced rows must carry a local-versus-provider state note; publish-pending and
blocked rows must carry their respective notes; rows with linked changes must name
the linked-change context. Generic ticket/task wording that conceals ownership or
queued state is rejected (`generic_ticket_wording_used`).

## Provider chip group

A `ProviderChipGroup` always names the provider, the project or space it is scoped
to, the tenant/org cue where relevant, and an explicit write posture. The five
postures are the ones a user must be able to tell apart directly:

- `read_only` — a read-only binding; nothing writes back.
- `comment_link` — a comment-link (limited) connection.
- `full_edit` — a full-edit connection.
- `offline_capture` — a local offline-capture; changes are captured locally, not
  published.
- `policy_blocked` — capability is blocked by policy.

The posture is checked against the provider authority
(`chip_posture_misrepresented`): `policy_blocked` requires a `policy_pinned`
authority, `offline_capture` requires a local authority, `comment_link` /
`full_edit` require a `provider_owned` authority, and `read_only` requires a
mirrored, imported, or provider-owned authority. The chip's `is_writable` flag must
match the derived truth (`chip_writability_misrepresented`), and the read-only,
offline-capture, and policy-block notes are required for their postures.

## Acceptance criteria coverage

- **Distinguish provider-authoritative from local-only or blocked directly in
  list surfaces**: the derived `state_authority_class` and the
  `claims_provider_authoritative` cross-check enforce it per row; the seed proves
  all five classes, and the trust review pins
  `local_or_blocked_never_reads_as_provider_authoritative`.
- **Canonical IDs remain visible and copyable**: every row requires a non-empty
  `canonical_id` with `canonical_id_copyable = true` and a `copy_canonical_id`
  default action; the consumer projection pins `canonical_id_copyable_everywhere`.

## Regenerating artifacts

```sh
cargo run -p aureline-provider --bin aureline_work_item_row_provider_chip_primitive -- support-export > artifacts/release/m5-work-item-row-provider-chip-proof/support_export.json
cargo run -p aureline-provider --bin aureline_work_item_row_provider_chip_primitive -- report      > artifacts/release/m5-work-item-row-provider-chip-proof/summary.md
cargo run -p aureline-provider --bin aureline_work_item_row_provider_chip_primitive -- csv         > artifacts/release/m5-work-item-row-provider-chip-proof/matrix.csv
cargo run -p aureline-provider --bin aureline_work_item_row_provider_chip_primitive -- fixture-work-item-row-local-only        > fixtures/ui/m5-work-item-row-provider-chip-controls/work_item_row_local_only.json
cargo run -p aureline-provider --bin aureline_work_item_row_provider_chip_primitive -- fixture-provider-chip-offline-capture   > fixtures/ui/m5-work-item-row-provider-chip-controls/provider_chip_offline_capture.json
```
