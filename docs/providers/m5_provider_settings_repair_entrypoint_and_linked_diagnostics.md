# M5 Provider-Settings Repair-Entrypoint Row & Linked Diagnostics

Status: shipped (M05-920, batch B108)

This primitive ships the reusable **provider-settings repair-entrypoint row** so a user can tell,
from the row alone, *which* boundary actually failed, *where* the repair entrypoint is, *what*
diagnostics the row links to, and — above all — that repairing the boundary never loses queued
work, breaks cached-read continuity, drops the reviewed export path, or forces a blind re-entry of
credentials.

It builds on the frozen provider-account / mapping / offline-capture component matrix
(M05-916) and the three row-state lanes that narrow it (M05-917 provider-account rows, M05-918
mapping/sync rows, M05-919 offline-capture/privacy rows). Those lanes resolve *what state* a row is
in; this lane closes the acceptance-criteria gap: it wires each row to the **real repair
entrypoint** and the **diagnostics that explain the failure**, so provider settings stop feeling
like an isolated sidebar and stale sessions / broken mappings stop collapsing into "retry-login"
folklore.

Source of truth: `crates/aureline-provider/src/ship_provider_settings_repair_entrypoints_and_linked_diagnostics_so_network_egress_auth_compatibility_boundaries_stay_explicit_across_claimed_m5_provider_surfaces`.

## Resolver

`resolve_provider_repair_entrypoint` takes one row's failed boundary, its account connection
state, whether queued drafts and a cached read exist, and whether a reviewed policy-escalation
route is available, and returns one `M5ResolvedProviderRepairEntrypoint`.

### Boundary → posture → entrypoint (one-to-one)

| Boundary class | Repair posture | Repair entrypoint | Boundary-specific diagnostic |
| --- | --- | --- | --- |
| `network_egress_blocked` | `network_egress_repair_row` | `open_network_egress_diagnostics` | `network_egress_diagnostic` |
| `auth_stale_session` | `reauth_session_row` | `open_reauth_handoff` | `auth_session_diagnostic` |
| `auth_scope_limited` | `widen_scope_row` | `open_scope_review` | `auth_session_diagnostic` |
| `mapping_broken` | `remap_target_row` | `open_mapping_repair` | `provider_compatibility_diagnostic` |
| `provider_incompatible` | `compatibility_review_row` | `open_compatibility_report` | `provider_compatibility_diagnostic` |
| `policy_blocked` | `policy_blocked_row` | `open_policy_review` | `auth_session_diagnostic` |

Every row additionally links to the `support_bundle_diagnostic` and the
`export_redaction_diagnostic`, so no repair row is ever divorced from the support and export
surfaces that explain and evidence the failure.

### Continuity guarantees (always all four)

Every resolved repair carries all four `M5RepairContinuityGuarantee`s:
`preserves_queued_drafts`, `preserves_cached_read_continuity`, `preserves_reviewed_export_path`,
and `no_blind_credential_reentry`. The auth boundary is repaired through the reviewed reauth
handoff (browser / device-code), never a blind credential prompt.

### Actions

Reveal, open-linked-diagnostics, and export-repair-evidence are always offered. A self-serve
`open_repair_entrypoint` is offered for every non-policy-blocked boundary; a `policy_blocked`
boundary offers only a reviewed `request_policy_escalation` (and must carry an escalation route —
otherwise the resolver returns `policy_blocked_without_escalation_route`).

## Hard invariants

Every consumer row asserts (all `false`): `isolates_settings_from_diagnostics`,
`loses_queued_work`, `requires_blind_credential_reentry`, `breaks_cached_read_continuity`,
`breaks_reviewed_export_path`.

## Consumers

The same repair grammar is bound on five claimed provider-settings surfaces: the provider-account
row, the project/board mapping row, the sync-behavior row, the privacy/redaction row, and the
provider status bar.

## Artifacts

- Schema: `schemas/ui/m5-provider-settings-repair-entrypoint-row.schema.json`
- Support export: `artifacts/release/m5-provider-settings-repair-entrypoint-row-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-provider-settings-repair-entrypoint-row-proof/matrix.csv`
- Design report: `artifacts/design/m5-provider-settings-repair-entrypoint-row.md`
- Narrowed fixtures: `fixtures/ui/m5-provider-settings-repair-entrypoint-row/`

Regenerate every artifact from truth with the headless emitter:

```sh
cargo run -q -p aureline-provider --bin aureline_provider_repair_entrypoint_row_primitive -- support-export
cargo run -q -p aureline-provider --bin aureline_provider_repair_entrypoint_row_primitive -- csv
cargo run -q -p aureline-provider --bin aureline_provider_repair_entrypoint_row_primitive -- report
cargo run -q -p aureline-provider --bin aureline_provider_repair_entrypoint_row_primitive -- validate
```

## Source contracts

Beyond this primitive's own schema and doc plus the M05-916 component matrix, each row binds the
real diagnostic and continuity contracts it links to:
`schemas/network/network_remediation_card.schema.json`,
`schemas/auth/reauth_requirement.schema.json`,
`schemas/providers/provider_sync_health_view.schema.json`,
`schemas/support/support_bundle.schema.json`,
`schemas/support/export_redaction_profile.schema.json`, and
`schemas/providers/offline_handoff_packet.schema.json`.
