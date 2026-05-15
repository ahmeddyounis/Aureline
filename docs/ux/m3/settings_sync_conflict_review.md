# Settings sync conflict review (beta)

This page describes the beta-grade settings sync and device-registry
conflict review projection that lives in `aureline-settings`. It
builds on the alpha conflict packet documented in
[`docs/settings/sync_conflict_review_alpha.md`](../../settings/sync_conflict_review_alpha.md)
and the M1 device-registry seed in
[`docs/settings/m1_sync_and_device_seed.md`](../../settings/m1_sync_and_device_seed.md),
so user-facing review surfaces, headless CLI inspection, and support
exports all report the same sync truth.

## Contract surface

The beta projection ships four record kinds, all under the shared
contract ref `settings:sync_beta:v1`:

- `sync_conflict_review_beta_row` — one row carrying the local and
  remote device records, the typed sync state class
  (`local_authoritative` / `synced` / `imported` / `stale` /
  `disabled_device`), the alpha conflict class, the offered and
  recommended resolution paths, the last-writer breadcrumb, the
  winning scope and value preview, the policy-lock source ref (when
  active), the rollback-and-retry decision, and the alpha review
  surface for handoff to the existing review affordance.
- `sync_conflict_review_beta_page` — rows in deterministic order
  with a `state_summary` (counts per sync state, rollback-required
  rows, policy-locked rows, auto-resolvable rows) and a separate
  `disabled_devices` block so the registry posture is visible
  without pivoting to another surface.
- `sync_conflict_review_beta_support_export` — wraps the beta page
  plus the alpha conflict packets the rows were derived from. The
  `source_packet_ref` on every row matches the packet at the same
  `packet_id`, so a support reviewer and the user pivot through one
  shared lineage.
- `RollbackDecision` (inline on every row) — names whether a
  rollback checkpoint or an approval ticket is required before apply,
  whether retry is available, and the human-readable explanation
  the review affordance renders verbatim.

The reviewer-facing landing page is this document. The first
runtime consumer is
[`crates/aureline-settings/src/bin/aureline_settings_inspect.rs`](../../../crates/aureline-settings/src/bin/aureline_settings_inspect.rs).

## Acceptance posture

The beta projection delivers the M3 sync and device-registry
acceptance gates:

- **Distinguishable sync states** — every row carries a typed
  `sync_state` from the closed `local_authoritative` / `synced` /
  `imported` / `stale` / `disabled_device` vocabulary. Stale and
  disabled-device rows are forced onto the `keep_local` path so an
  older bundle or a paused / revoked / forgotten peer can never
  silently overwrite the local effective value.
- **Support-export explains the winner without leaking secrets** —
  the support export carries the winning scope, the winning source
  label, the last-writer breadcrumb (device id, actor class,
  revision ref, mutation-journal ref), and a redaction-aware value
  preview. Redaction classes
  (`redact_value_preserve_shape` / `redact_to_class_label` /
  `exclude_from_export`) are honoured by the row preview and counted
  in the support-export envelope's `redacted_value_count`.
- **Rollback and retry paths cover widening or unexpected
  overwrite** — every row's `rollback_decision` names whether the
  recommended path requires a checkpoint or an approval ticket
  before apply, and the `retry_state` token names the condition
  under which retry becomes available
  (`retry_when_fresh_bundle_arrives`,
  `retry_when_remote_device_active`,
  `retry_when_local_device_active`,
  `checkpoint_required_before_apply`, or `no_retry_required`).

## Headless consumers

The beta projection is exercised through the same
`aureline_settings_inspect` binary the alpha inspector ships:

```sh
cargo run -q -p aureline-settings --bin aureline_settings_inspect -- sync-beta-review
cargo run -q -p aureline-settings --bin aureline_settings_inspect -- sync-beta-support-export
```

Both subcommands emit a deterministic three-row page (stale,
policy-locked high risk, disabled-device) so the rollback /
retry / disabled-device copy is testable from CI without launching
the live shell.

## Fixtures

Protected fixtures live under
[`fixtures/settings/sync_beta/`](../../../fixtures/settings/sync_beta/):

- `review_page.json` — beta page covering stale, policy-locked
  high-risk, and disabled-device rows, with the aggregate state
  summary and the registry block for the paused device.
- `support_export.json` — support export wrapper that quotes both
  the beta page and the alpha conflict packets the rows were built
  from.

## Verification

```sh
cargo test -p aureline-settings
cargo test -p aureline-settings --test sync_beta_fixtures
```

The integration test
[`crates/aureline-settings/tests/sync_beta_fixtures.rs`](../../../crates/aureline-settings/tests/sync_beta_fixtures.rs)
replays the fixtures through the Rust types and asserts the
state-summary counters, the rollback decision, the disabled-device
registry block, and the shared `source_packet_ref` parity between
page rows and the embedded conflict packets.
