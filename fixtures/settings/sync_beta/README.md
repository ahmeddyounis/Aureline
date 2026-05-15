# Settings sync beta review fixtures

These fixtures pin the beta-grade settings sync conflict review
projection that lives in
[`crates/aureline-settings/src/sync/`](../../../crates/aureline-settings/src/sync).
Each file validates against the shared types in that module and
mirrors what the headless inspector bin emits, so the checked-in
JSON stays a literal projection of the resolver and the alpha
conflict packet — no hand-written drift.

| Fixture | Record kind | What it pins |
|---|---|---|
| [`review_page.json`](./review_page.json) | `sync_conflict_review_beta_page` | A page with one stale row, one policy-locked high-risk row, and one disabled-device row, plus the aggregate state summary and the disabled-device registry block. |
| [`support_export.json`](./support_export.json) | `sync_conflict_review_beta_support_export` | The same page wrapped in a support-export envelope, with the alpha conflict packets embedded by `source_packet_ref`, the redacted-value count, and the rollback-required count. |

Each fixture:

- carries opaque `dev-*` device ids and lineage cursors only (no
  hostnames, IP addresses, MAC addresses, serials, or filesystem
  paths);
- quotes the typed sync-state vocabulary (`local_authoritative`,
  `synced`, `imported`, `stale`, `disabled_device`) verbatim;
- carries a rollback decision that names whether a checkpoint or
  approval is required before any apply, and whether retry is
  available once the upstream condition clears;
- pivots from a page row to its alpha conflict packet through the
  shared `source_packet_ref`.

Regenerate after touching the sync module or the seed catalog:

```sh
cargo run -q -p aureline-settings --bin aureline_settings_inspect -- sync-beta-review \
  > fixtures/settings/sync_beta/review_page.json
cargo run -q -p aureline-settings --bin aureline_settings_inspect -- sync-beta-support-export \
  > fixtures/settings/sync_beta/support_export.json
```
