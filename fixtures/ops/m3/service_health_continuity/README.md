# service-health continuity drill corpus

These fixtures back the M3 continuity-proof lane for the service-health
surface. They are read by every surface that displays service-health
truth — desktop shell, About, diagnostics, CLI/headless inspect, support
exports, and the release-truth packet — so a single regression in
contract-state wording, freshness, affected-workflow mapping, or the
local-continuity rollup fails the corpus instead of shipping silently.

Each fixture is a complete aggregator record validated against
`schemas/ops/service_health_card.schema.json`. The fixtures are minted
by the `aureline_shell_service_health_continuity_corpus` emitter from
the in-code corpus at
`crates/aureline-shell/src/service_health/continuity_corpus.rs`, and the
fixture-replay test at
`crates/aureline-shell/tests/service_health_continuity_fixtures.rs`
asserts the disk content matches the in-code projection bit-for-bit.

## Drills

| Drill id | Plane | Overall state | Overall continuity | What it proves |
| -------- | ----- | -------------- | ------------------ | -------------- |
| `single_service_outage` | Single service | `unavailable` | `local_safe` | A hosted-only outage (marketplace) cannot drag overall local continuity below `local_safe`. |
| `control_plane_unavailable` | Control plane | `contract_mismatch` | `local_safe` | Release-channel, license, marketplace, and status feed impaired while data-plane editing, sync, and AI assist stay current. |
| `data_plane_unavailable` | Data plane | `unavailable` | `local_safe_read_only` | Workspace sync (remote-required boundary) and the hosted remote runtime are offline. Local edits keep working; external writes pause. |
| `mirror_only_fallback` | Mirror fallback | `local_only` | `local_safe` | Marketplace and docs primary paths unreachable; the cached mirrors keep both families usable. Cards carry the `fallback_mode:mirror_only` detail token. |
| `stale_cache` | Stale cache | `stale` | `local_safe` | Release channel, docs, and status feed cards are past their review window; they MUST NOT render as current ready truth. |
| `contract_mismatch` | Contract mismatch | `contract_mismatch` | `local_safe` | Release-channel returned an off-schema manifest; the result is held until the contract clears. Distinct from generic `degraded`. |
| `policy_block` | Policy block | `policy_blocked` | `local_safe` | Telemetry upload and AI assist gated by workspace policy. Distinct from `unavailable`. |
| `auth_loss` | Auth / license loss | `unavailable` | `local_safe_read_only` | License broker unreachable cascades into workspace-sync push pause. |
| `recovery_after_restart` | Recovery | `ready` | `local_safe` | Post-restart probes replace cached "down" state. The AI card is honestly `never_checked` because the post-restart probe has not yet returned. |

## Refreshing the corpus

```sh
cargo run -q -p aureline-shell \
  --bin aureline_shell_service_health_continuity_corpus -- emit-fixtures \
  fixtures/ops/m3/service_health_continuity
```

The replay test in
`crates/aureline-shell/tests/service_health_continuity_fixtures.rs`
fails if the on-disk JSON drifts from the in-code corpus, so refreshing
the fixtures is mandatory whenever the corpus changes.
