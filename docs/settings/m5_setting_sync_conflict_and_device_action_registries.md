# M5 sync-conflict and device-action registries

This lane is the sync / conflict engine implement lane over the frozen
[M5 settings-governance matrix](./m5_settings_resolver_contract.md) (the `sync_scope` family). It turns the
*sync-conflict-packet* grammar (how a sync scope bundle, session, and conflict packet declare which field
diverged, the local and remote revisions, the field-level keep-local / keep-synced options, the compare surface,
and the blocked-state reason a conflict class carries) and the *device-action-record* grammar (how a device
action ledger records the actor, timestamp, transport and policy state, capability dependency, attribution, and
last revision for a pause / resume / revoke / forget / token-rotation action) into registry resolvers that
produce export-safe, honest projections, so the sync-session, import-apply, outage-recovery, device-review, docs,
CLI, and support surfaces resolve one canonical configuration truth instead of a per-conflict, last-writer-wins
path. The conflict packet and the device-action ledger are separated in runtime and serialized state: the
conflict class, field path, local / remote revisions, keep-local option, keep-synced option, compare reference,
and blocked-state reason live on the conflict packet, while the actor, action timestamp, transport state, policy
state, capability dependency, attribution reference, and last ledger revision live on the device-action record,
and sync never silently overwrites locked, machine-only, or stale-local authoritative state because a downstream
flow found that path easier.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_setting_sync_conflict_and_device_action_registries` (the authoritative validator).
- **Combined schema:**
  `schemas/config/m5-setting-sync-conflict-and-device-action-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/config/m5-sync-conflict-packet.schema.json`](../../schemas/config/m5-sync-conflict-packet.schema.json)
  and
  [`schemas/settings/sync_device_record.schema.json`](../../schemas/settings/sync_device_record.schema.json)
  as its canonical domain contracts.
- **Checked proof:**
  `artifacts/release/m5-setting-sync-conflict-and-device-action-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`).
- **Narrowed fixtures:** `fixtures/config/m5-setting-sync-conflict-and-device-action-registries/`
  (`sync_conflict_beta_narrowed.json`, `device_action_preview_narrowed.json`).

## Two registries

1. **Sync-conflict packet** (`resolve_sync_conflict_packet_entry`) — publishes one conflict packet per conflict:
   the conflict class and canonical class mode, the field path, the local and remote revisions, the field-level
   keep-local option, the keep-synced option, the compare reference, and the blocked-state reason. A clean entry
   names a canonical registry token, a classified conflict class, and a settings-governance role, covers the
   canonical / accessible / audit resolution forms, publishes a complete packet, keeps its resolution field-aware,
   and preserves local authoritative state before a protected (policy-locked / machine-only / stale-remote)
   conflict applies. Otherwise it degrades honestly — a resolution that collapses into last-writer-wins (or a
   protected conflict that silently overwrote local state) degrades to
   `conflict_silently_overwrites_or_hides_field_resolution`.
2. **Device-action record** (`resolve_device_action_record_entry`) — keeps a pause / resume / revoke / forget /
   token-rotation action reconstructable. A clean entry names a classified device-action class and provides the
   complete actor / action-timestamp / transport-state / policy-state / capability-dependency /
   attribution-reference / last-ledger-revision device-action-record object; a record that would hide a revoke /
   forget cause without disclosing its reason or leave a degraded-transport action without disclosing that local
   state stays authoritative degrades to `device_action_ledger_hides_attribution_or_drops_reconstruction`.

## Per-entry sync-conflict reference

The conflict class carries its canonical class mode, and the resolver publishes the full conflict packet, so the
registry — never a hand-copied, last-writer-wins assumption — is the single source of truth.
`sync_conflict_packet_is_complete` rejects a packet missing any field, `conflict_does_not_silently_overwrite`
rejects a collapsed resolution or an overwritten protected conflict, and
`device_action_ledger_stays_reconstructable` rejects a record that has hidden its attribution or dropped its
revoke cause.

| conflict class | class mode | field path | keep local | keep synced | blocked reason | compare reference |
| --- | --- | --- | --- | --- | --- | --- |
| same-key divergent | same_key_divergent_conflict | `editor.font-size` | `keep-local.font-size-14` | `keep-synced.font-size-16` | `blocked.none-review-and-choose` | `compare.field-diff-0007` |
| policy-locked | policy_locked_conflict | `security.telemetry-optin` | `keep-local.policy-locked-off` | `keep-synced.blocked-by-policy` | `blocked.policy-lock-holds-local` | `compare.field-diff-0011` |
| machine-only | machine_only_conflict | `runtime.gpu-adapter` | `keep-local.machine-only-adapter` | `keep-synced.not-portable` | `blocked.machine-only-stays-local` | `compare.field-diff-0013` |

A collapsed last-writer-wins resolution degrades to `conflict_silently_overwrites_or_hides_field_resolution`, an
incomplete packet degrades to `sync_conflict_packet_incomplete`, and a hidden device-action ledger degrades to
`device_action_ledger_hides_attribution_or_drops_reconstruction`, so a collapsed resolution, an incomplete
packet, or a hidden ledger can never turn release evidence green.

## Acceptance criteria (proven by resolved examples)

- **Local durable state remains authoritative when sync transport, encryption, policy, or provider state
  degrades.** Clean conflict entries cover the canonical same-key-divergent / policy-locked / missing-capability /
  machine-only / delete-versus-modify / stale-remote classes and the first sync-session / import-apply /
  outage-recovery / device-review / support flows, a packet-incomplete example degrades, and no clean conflict
  entry published an incomplete packet.
- **Users can review and resolve conflicts at the field level with keep-local / keep-synced / compare behavior
  and explicit blocked-state reasons.** A field-collapse / overwrite example and an unbound example degrade, a
  clean field-aware conflict entry is present, and no clean entry collapsed into last-writer-wins — so a protected
  conflict can never silently overwrite locked, machine-only, or stale-local state because a downstream flow found
  that path easier.
- **Regression suites fail when sync / import routes silently overwrite locked, machine-only, or stale-local
  configuration state.** Clean device-action entries cover the pause / resume / revoke / forget / rotate classes
  with full resolution-form coverage while providing the complete record object, and a record that would hide a
  revoke / forget cause or leave a degraded-transport action without disclosing its local-authority posture
  degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_setting_sync_conflict_and_device_action_registries -- support-export
cargo run -p aureline-ui --example dump_m5_setting_sync_conflict_and_device_action_registries -- csv
cargo run -p aureline-ui --example dump_m5_setting_sync_conflict_and_device_action_registries -- report
cargo run -p aureline-ui --example dump_m5_setting_sync_conflict_and_device_action_registries -- conflict-table
cargo run -p aureline-ui --example dump_m5_setting_sync_conflict_and_device_action_registries -- fixture-sync-conflict-beta-narrowed
cargo run -p aureline-ui --example dump_m5_setting_sync_conflict_and_device_action_registries -- fixture-device-action-preview-narrowed
```
