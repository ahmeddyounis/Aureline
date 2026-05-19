# service-health card fixtures

These fixtures pin the service-health aggregator and contract-state card
output every shell, About, CLI/headless inspect, diagnostics, and support
export surface reads. They are the cross-tool boundary for the
`service_health_contract_beta` review.

Every aggregator fixture validates against
`schemas/ops/service_health_card.schema.json`. The vocabulary fixture
validates against `schemas/ops/service_contract_state.schema.json`.

## Cases

- `seeded_aggregator.json` — the deterministic seeded packet minted by
  `crates/aureline-shell/src/service_health/seed.rs::seeded_aggregator()`
  and emitted by `aureline_shell_service_health_inspect aggregator`.
  Covers every contract-state token in the closed vocabulary so a single
  packet exercises the whole surface.

- `all_ready_aggregator.json` — the safe baseline. Both cards are
  `ready`; the aggregator MUST NOT light an honesty marker and MUST
  report `local_safe` overall. Used as the negative drill: a healthy
  aggregator must not fabricate yellow chrome.

- `hosted_outage_keeps_local_safe.json` — the core acceptance criterion
  ("a single failed service cannot silently flip the whole product into
  broken or unavailable messaging when local work remains safe"). The
  marketplace card is `unavailable` but its boundary is `hosted`, so
  overall local-continuity stays at `local_safe`. The chrome must still
  paint the marketplace card as unavailable.

- `sync_outage_downgrades_to_read_only.json` — the inverse: workspace
  sync has `local_with_remote_required` boundary, so a `local_only`
  contract state correctly drags overall continuity down to
  `local_safe_read_only`. Edits are still safe; only external writes
  pause.

- `contract_state_vocabulary.json` — the closed contract-state
  vocabulary itself (`ready`, `degraded`, `local_only`, `stale`,
  `contract_mismatch`, `policy_blocked`, `unavailable`). Surfaces MUST
  quote one of these tokens; rendering an open-ended token outside this
  set is a contract violation.

## Refresh

To regenerate `seeded_aggregator.json` after changing the seeded probe
set:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_service_health_inspect -- aggregator \
  > fixtures/ops/m3/service_health_cards/seeded_aggregator.json
```

The `service_health_aggregator_seeded_fixture_matches_code` test in
`crates/aureline-shell/tests/service_health_card_fixtures.rs` enforces
that the fixture on disk matches the in-code seeded aggregator so the
two cannot drift silently.
