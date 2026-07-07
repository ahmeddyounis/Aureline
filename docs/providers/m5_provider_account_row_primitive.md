# M5 Provider-Account-Row Primitive

One reusable M5 provider primitive — the **provider-account row** — so a user can tell,
from the row alone, whether Aureline can currently **read**, **write**, or only **inspect a
cached read** of provider state before any live mutation is attempted.

This lane (`M05-917`) narrows the `provider_account_row` family of the frozen
[provider-account / offline-capture component matrix](m5_provider_account_offline_capture_component_matrix.md)
into one reusable resolver plus a per-surface parity matrix. It reuses that matrix's frozen
vocabulary verbatim (provider identity class, account connection state, tenant scope,
effective write scope, surface family, deployment line, consumer surface, accessibility
route, qualification class, downgrade trigger) and mints only what the matrix left implicit
about the account row itself.

## Resolver

`resolve_provider_account_row` takes one account's state and produces one
`M5ResolvedProviderAccountRow`:

- **Row posture** is derived one-to-one from the frozen `M5AccountConnectionState`, so the
  six governed states never collapse into one generic "connected" chip:
  `not_configured_row`, `signed_in_row`, `limited_scope_row`, `stale_session_row`,
  `offline_cached_read_row`, `policy_blocked_row`.
- **Access capability** is derived from the connection state and the effective write scope,
  so a user can tell whether Aureline can read live, write, or only inspect a cached read:
  - `not_configured` / `policy_blocked` → `no_access`
  - `stale_session` / `offline_cached_read` → `cached_inspect_only` (never reads as live)
  - `signed_in` → `can_read_and_write` / `can_read_write_limited` / `can_read_only_live`
    depending on the write scope
  - `limited_scope` → capped to `can_read_write_limited` / `can_read_only_live`
- **Bounded actions**: `reveal_scope` and `export_row` are always offered; `sign_in_account`
  when nothing is configured; `retry_auth` when a configured account is degraded or its
  session needs refresh; `remove_account` whenever an account is configured.

### Acceptance criteria encoded

- **Read / write / inspect from the row alone** — `can_read_live`, `can_write`, and
  `only_inspect_cached` are derived and exported on every resolved row; the
  `access_capability_coverage_unproven` lint requires a write-capable, a read-only-live, and
  a cached-inspect-only worked case.
- **Retry / re-auth / remove preserve local drafts and support/export continuity** —
  `preserves_local_drafts` and `preserves_support_export_continuity` are always `true` and
  `requires_blind_credential_reentry` is always `false`; the `draft_continuity_unproven`
  lint enforces it across every worked case.
- **Six states never collapse** — the `connection_state_coverage_unproven` lint requires
  every one of the six connection states to be exercised, and the
  `collapses_states_into_generic_connected` hard invariant must be `false` on every row.

## Parity matrix

`M5ProviderAccountRowPacket` binds one row per claimed provider surface consumer —
`account_settings_panel`, `provider_status_bar`, `connection_picker`,
`headless_cli_accounts`, and `support_account_export` — to the shared account-row anatomy,
the same identity/connection/scope/freshness vocabulary, the derived posture and access
capability, the bounded actions, the export fields, and the non-visual accessibility routes,
so the connection / scope / freshness / access vocabulary stays identical across desktop,
headless/export, and support consumers.

Each row carries four hard invariants (all `false`): it never masks its connection state or
write scope, never collapses the six connection states into one generic connected label,
never renders a cached-only read with live certainty, and never forces blind credential
re-entry.

## Sources of truth

- Schema: `schemas/ui/m5-provider-account-row.schema.json`
- Frozen matrix: `schemas/ui/m5-provider-account-offline-capture-component-matrix.schema.json`
- Real binds: `schemas/providers/connected_account_record.schema.json`,
  `schemas/providers/provider_account_scope.schema.json`
- Support export: `artifacts/release/m5-provider-account-row-primitive-proof/support_export.json`
- Design report: `artifacts/design/m5-provider-account-row-primitive.md`
- Narrowed fixtures: `fixtures/ui/m5-provider-account-row-primitive/`

## Emitter

The headless emitter is the only mint-from-truth path:

```sh
cargo run -q -p aureline-provider --bin aureline_provider_account_row_primitive -- support-export
cargo run -q -p aureline-provider --bin aureline_provider_account_row_primitive -- report
cargo run -q -p aureline-provider --bin aureline_provider_account_row_primitive -- csv
cargo run -q -p aureline-provider --bin aureline_provider_account_row_primitive -- validate
```
