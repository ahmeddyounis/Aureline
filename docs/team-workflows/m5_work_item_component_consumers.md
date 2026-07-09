# M5 work-item component consumers

**Status:** Stable (adoption lane over the frozen M5 work-item component matrix)

This lane proves the eight reusable M5 work-item components are adopted consistently across
every claimed M5 work-item surface, so the same provider, freshness, and offline-handoff
language survives outside the primary tracker view. It is the closing consumer lane of batch
B116, sitting on top of:

- the frozen matrix
  (`crate::freeze_the_m5_work_item_component_matrix`, schema
  `schemas/ui/m5-work-item-component-matrix.schema.json`), and
- the four sibling implement lanes that narrow the eight families into working primitives:
  - work-item row + provider-chip group →
    `schemas/ui/m5-work-item-row-provider-chip-controls.schema.json`
  - relation strip + sync-pending pill →
    `schemas/ui/m5-relation-strip-sync-pending-controls.schema.json`
  - work-item detail header + status-transition sheet →
    `schemas/ui/m5-work-item-detail-header-status-transition-controls.schema.json`
  - related-evidence card + offline-handoff-packet card →
    `schemas/ui/m5-related-evidence-offline-handoff-controls.schema.json`

## Consumers

Seven claimed M5 work-item consumers each adopt the shared components and point at the
canonical component schemas instead of re-wording facts in local prose:

| Consumer | Role |
| --- | --- |
| `inbox` | Issue Inbox |
| `detail` | Work-Item Detail |
| `review` | Review Workspace |
| `incident` | Incident Workspace |
| `help` | Help / Docs |
| `support` | Support / Export Desk |
| `export` | Offline Export Packet |

The `help`, `support`, and `export` consumers are held to a stronger check: every family they
adopt must reference the canonical component schema, so a help, support, or export surface can
never drift from the product truth.

## Shared descriptor vocabulary

Every binding keeps all six descriptors explicit — the track invariant for this lane:

`canonical_identity`, `provider_authority`, `local_versus_provider_state`,
`linked_engineering_context`, `side_effect_preview`, `publish_later_continuity`.

## Parity-health, narrowing, and commit honesty

A consumer renders a component under one parity-health mode. Full parity preserves the
descriptor vocabulary with no banner. Any weakened mode auto-narrows the claim and always
discloses a self-contained banner naming the exact reason, the preserved descriptors, and the
recovery action — never a generic "degraded" note:

| Parity-health mode | Narrowing reason | Recovery action | Queued/offline? |
| --- | --- | --- | --- |
| `full_parity` | — | — | no |
| `provider_scope_limited_narrowed` | `provider_scope_limited` | `reauthorize_for_full_scope` | no |
| `sync_pending_narrowed` | `sync_pending` | `publish_or_retry_queued_when_online` | yes |
| `offline_handoff_narrowed` | `offline_handoff_local_only` | `export_or_publish_handoff_packet` | yes |
| `linked_context_stale_narrowed` | `linked_context_stale` | `relink_or_refresh_context` | no |

A binding that reflects queued-local or offline-captured state (`sync_pending` or
`offline_handoff_local_only`) always narrows and never asserts provider-committed state, so a
locally-held change never masquerades as a published one.

## Guardrails (enforced by `validate`)

- Every one of the eight component families is adopted by at least two distinct consumers —
  proof that they are reusable components, not one tracker view plus isolated export objects.
- At least one worked binding proves a narrowed rendering with a self-contained banner, and at
  least one proves a full-parity rendering with no banner.
- At least one worked binding reflects queued-local or offline-captured state and never asserts
  commit; any such binding that claims commit fails validation.
- Generic ticket / task wording never conceals provider ownership, queued state, offline
  capture, or linked context.

## Artifacts

Minted only by `cargo run -p aureline-provider --bin aureline_work_item_component_consumers`:

- `artifacts/release/m5-work-item-component-consumer-proof/support_export.json`
- `artifacts/release/m5-work-item-component-consumer-proof/matrix.csv`
- `artifacts/release/m5-work-item-component-consumer-proof/report.md`
- `fixtures/ui/m5-work-item-component-consumers/incident_beta_narrowed.json`
- `fixtures/ui/m5-work-item-component-consumers/review_preview_narrowed.json`

The checked-in support export and fixtures are validated against the seed builder by the
inline tests, so the in-code matrix and the on-disk artifacts can never drift.
