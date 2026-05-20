# Service-health claim qualification

This doc names what every marketed beta service family MUST produce
before a service-health card can ship behind it. It binds the
service-health beta contract
(`docs/ops/m3/service_health_contract_beta.md`) to the checked-in
continuity corpus (`fixtures/ops/m3/service_health_continuity/`), the
reviewer-facing report
(`artifacts/ops/m3/service_health_continuity_report.md`), and the
release-truth contract matrix
(`artifacts/release/m3/service_health_contract_matrix.json`).

The rules below are normative for the M3 beta-exit service-health truth
lane.

## Required evidence per marketed service family

Every claimed beta service family — `language_services`, `ai_assist`,
`sync`, `license_entitlement`, `telemetry`, `marketplace`,
`remote_runtime`, `release_channel`, `docs_knowledge`, `status_feed` —
MUST publish all of the following before its card appears in the
service-health pane, About, diagnostics, CLI/headless inspect, or a
support export:

1. **A row in the contract matrix.** Every family MUST resolve to a
   `service_family_rows` entry in
   [`artifacts/release/m3/service_health_contract_matrix.json`](../../../artifacts/release/m3/service_health_contract_matrix.json)
   that pins its `boundary_class`, the plane classes it can fail on, and
   the `qualifying_drill_ids` that hold the row honest. A family with no
   matrix row is not currently proved and MUST NOT paint a card.
2. **Current degraded-state evidence.** Each family MUST be exercised by
   at least one drill in the corpus that lands the family in a non-`ready`
   contract state, or — for families that cannot themselves go down (a
   `local_only` boundary such as `language_services`) — by drills proving
   that unrelated outages do not silently downgrade it. The drills are
   minted from the in-code corpus at
   [`crates/aureline-shell/src/service_health/continuity_corpus.rs`](../../../crates/aureline-shell/src/service_health/continuity_corpus.rs)
   and pinned bit-for-bit on disk under
   `fixtures/ops/m3/service_health_continuity/`.
3. **Boundary-correct continuity proof.** The family's drills MUST prove
   the local-continuity invariant for its `boundary_class`:
   - `hosted` and `vendor_provider` families (`marketplace`,
     `remote_runtime`, `telemetry`, `status_feed`, `ai_assist`) MUST NOT
     drag `overall_local_continuity` below `local_safe` when they alone
     are impaired.
   - `local_with_remote_required` families (`sync`) MUST drag
     `overall_local_continuity` to `local_safe_read_only` (or worse) so
     external writes are visibly paused.
   - `local_with_remote_optional` families (`license_entitlement`,
     `release_channel`, `docs_knowledge`) MUST keep local work safe while
     surfacing the impaired card.
4. **Stale-status ban.** Any card whose `contract_state` is `stale`, or
   whose `last_checked_age` is `stale`, `very_stale`, or `never_checked`,
   MUST light the honesty marker. Cached or aged cards cannot masquerade
   as current online truth after restart, reconnect, or offline
   transition. This is exercised by the `stale_cache` and
   `recovery_after_restart` drills.
5. **Cross-surface parity.** The family's card MUST render identically
   across the desktop shell, About, diagnostics, the headless emitter
   (`aureline_shell_service_health_continuity_corpus`), the seeded
   inspector (`aureline_shell_service_health_inspect`), and the
   support-export plaintext block. Divergent copy, state tokens, or
   last-checked age between surfaces is a contract violation, not a
   presentation choice.

## Plane separation is mandatory

Control-plane and data-plane impairment MUST stay distinct in product,
docs, and support export.

- **Control-plane** families (`release_channel`, `license_entitlement`,
  `marketplace`, `telemetry`, `status_feed`, `docs_knowledge`) surface
  their cards as `unavailable`, `stale`, `contract_mismatch`, or
  `policy_blocked` without dragging overall local continuity below
  `local_safe`. Editing, sync, and language services keep reading
  `ready`. Proven by `control_plane_unavailable`, `stale_cache`,
  `contract_mismatch`, and `policy_block`.
- **Data-plane** families (`sync`, `remote_runtime`) surface their cards
  as `local_only` or `unavailable`, drag overall continuity to
  `local_safe_read_only`, and quote the workflow tokens the user
  actually feels (`workspace_sync`, `remote_shell`). Proven by
  `data_plane_unavailable` and `auth_loss`.

A surface that collapses the two planes into one chrome label fails the
fixture-replay test.

## Cut-vs-downgrade rules

If a family cannot produce the evidence above, do **not** paper it over
with generic "service degraded" or "best effort" wording. Either:

- **Cut the card.** Remove the family from the marketed service-health
  list until its drills and matrix row exist. Update the service-health
  pane, About, diagnostics, CLI/headless inspect, support exports, and
  the report in the same change.
- **Downgrade the claim.** Surface the narrower truth the evidence
  supports — for example, render `local_only` with a
  `fallback_mode:mirror_only` detail token instead of claiming a live
  primary path, or render `stale` with the honesty marker instead of
  `ready` when the probe is past its review window.

A claim that cannot be cut or honestly narrowed is a release blocker.

## Prohibited renderings

The following renderings are non-conforming and the corpus replay test
(`crates/aureline-shell/tests/service_health_continuity_fixtures.rs`)
holds them out:

- **Generic outage copy.** `state_explanation` MUST NOT contain
  "service down", "service degraded", "broken", "error occurred",
  "we hit an error", or "something went wrong". Surfaces quote the
  closed contract-state vocabulary instead.
- **`contract_mismatch` collapsed into `degraded`.** A schema-mismatched
  remote response surfaces as `contract_mismatch`, with results held
  until the contract clears.
- **`policy_blocked` collapsed into `unavailable`.** A policy-disabled
  service surfaces as `policy_blocked` and explains why local work
  continues to be safe.
- **`local_only` mirror fallback collapsed into `unavailable`.** A
  mirror serving while the primary path is unreachable surfaces as
  `local_only` with the `fallback_mode:mirror_only` detail token and
  `local_safe` continuity.
- **Aged probe painted as fresh.** A `stale` / `very_stale` /
  `never_checked` probe MUST light the honesty marker; the chrome cannot
  reuse a pre-restart "ready" cache.

## Vocabulary reuse

Product, docs, support, and the release-truth matrix MUST quote the same
closed vocabulary for the tested scenarios, all re-exported from
[`crates/aureline-shell/src/service_health/aggregator.rs`](../../../crates/aureline-shell/src/service_health/aggregator.rs)
and pinned by the schemas:

- `service_family`, `boundary_class`, `contract_state`,
  `local_continuity`, `affected_workflows`, `last_checked_age` —
  re-exported from
  [`schemas/ops/service_health_card.schema.json`](../../../schemas/ops/service_health_card.schema.json).
- The closed contract-state token set — re-exported from
  [`schemas/ops/service_contract_state.schema.json`](../../../schemas/ops/service_contract_state.schema.json).

Surfaces that mint parallel wording are non-conforming and the corpus
replay test will fail when the vocabulary diverges.

## Verification

Run the corpus replay test:

```bash
cargo test -p aureline-shell --test service_health_continuity_fixtures
cargo test -p aureline-shell --lib service_health::continuity_corpus
```

The replay test loads every fixture under
`fixtures/ops/m3/service_health_continuity/`, asserts each aggregator
matches the in-code projection byte-for-byte, and asserts every pinned
`overall_contract_state`, `overall_local_continuity`, and
`honesty_marker_present` holds, plus the stale-status ban, the boundary
continuity invariants, and the generic-copy ban.

## Refresh trigger

Refresh this doc, the contract, the corpus, the matrix, and the report
in the same change when any of these change:

- The corpus module at
  `crates/aureline-shell/src/service_health/continuity_corpus.rs`.
- The aggregator module at
  `crates/aureline-shell/src/service_health/aggregator.rs`.
- The schemas under `schemas/ops/service_health_card.schema.json` or
  `schemas/ops/service_contract_state.schema.json`.
- The beta contract at `docs/ops/m3/service_health_contract_beta.md`.
- The marketed service-family list (a family is added, narrowed,
  downgraded, or cut).

After editing the corpus, re-emit the fixtures:

```sh
cargo run -q -p aureline-shell \
  --bin aureline_shell_service_health_continuity_corpus -- emit-fixtures \
  fixtures/ops/m3/service_health_continuity
```
