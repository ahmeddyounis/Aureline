# Service-health continuity drill report — M3 beta

Schema version: 1
As of: 2026-05-19
Owner: @ahmeddyounis
Backup waiver: single-maintainer-backup

This report is the reviewer-facing summary of the service-health
continuity drill corpus. It is the proof packet that release engineering
and partner review read alongside the beta claim manifest, the
deployment-profile continuity packet, and the failover-continuity banner
contract before promoting any service-health row to beta.

## Why this corpus exists

Service-health is the single chrome surface that tells a user "is the
product still usable right now". A cosmetic panel that paints generic
"service degraded" copy under every outage class would let a hosted
marketplace outage imply the whole product is broken, let a stale cache
masquerade as current online truth after a restart, and let a contract
mismatch in the release-channel manifest collapse into a generic yellow
chip indistinguishable from a slow AI provider.

The corpus prevents that by minting one drill per failure class. Each
drill builds a `ServiceHealthAggregator` from a deterministic set of
probe readings, projects the cards across the closed contract-state
vocabulary, and asserts the overall rollup the surface must land on. The
drills are pinned bit-for-bit on disk so a regression in contract-state
wording, freshness, affected-workflow mapping, or the local-continuity
rollup fails the corpus instead of shipping silently.

## How surfaces read the corpus

Every surface that displays service-health truth reads the same
aggregator record:

- **Desktop shell** — service-health pane, About card, diagnostics
  inspector, status chip.
- **CLI / headless inspect** — `aureline_shell_service_health_inspect`
  for the seeded packet; `aureline_shell_service_health_continuity_corpus`
  for the continuity drills.
- **Support exports** — `ServiceHealthAggregator::render_plaintext()` is
  the plaintext block the support export embeds.
- **Release-truth packet / matrix** —
  `artifacts/release/m3/service_health_contract_matrix.json` quotes the
  drill ids and the on-disk fixture refs verbatim so a beta-promoted row
  can be cross-referenced to its qualifying drills.

If any surface diverges from the aggregator record (different copy,
different state token, different last-checked age) it is a contract
violation, not a presentation choice.

## Drill index

The corpus contains one drill per failure class. Each drill exercises
one or more service families and pins the overall rollup the surface
must land on.

| Drill id | Plane | Overall state | Overall continuity | Honesty marker | Fixture |
| -------- | ----- | -------------- | ------------------ | -------------- | ------- |
| `single_service_outage` | Single service | `unavailable` | `local_safe` | present | `fixtures/ops/m3/service_health_continuity/single_service_outage.json` |
| `control_plane_unavailable` | Control plane | `contract_mismatch` | `local_safe` | present | `fixtures/ops/m3/service_health_continuity/control_plane_unavailable.json` |
| `data_plane_unavailable` | Data plane | `unavailable` | `local_safe_read_only` | present | `fixtures/ops/m3/service_health_continuity/data_plane_unavailable.json` |
| `mirror_only_fallback` | Mirror fallback | `local_only` | `local_safe` | present | `fixtures/ops/m3/service_health_continuity/mirror_only_fallback.json` |
| `stale_cache` | Stale cache | `stale` | `local_safe` | present | `fixtures/ops/m3/service_health_continuity/stale_cache.json` |
| `contract_mismatch` | Contract mismatch | `contract_mismatch` | `local_safe` | present | `fixtures/ops/m3/service_health_continuity/contract_mismatch.json` |
| `policy_block` | Policy block | `policy_blocked` | `local_safe` | present | `fixtures/ops/m3/service_health_continuity/policy_block.json` |
| `auth_loss` | Auth / license loss | `unavailable` | `local_safe_read_only` | present | `fixtures/ops/m3/service_health_continuity/auth_loss.json` |
| `recovery_after_restart` | Recovery | `ready` | `local_safe` | present | `fixtures/ops/m3/service_health_continuity/recovery_after_restart.json` |

## Distinguishing control plane from data plane

Control-plane and data-plane impairment stay distinct in product and
docs.

- Control-plane drills (`control_plane_unavailable`, `stale_cache`,
  `contract_mismatch`, `policy_block`, parts of `single_service_outage`)
  surface their cards as `unavailable`, `stale`, `contract_mismatch`, or
  `policy_blocked` without dragging overall local continuity below
  `local_safe`. Editing, sync, and language services keep reading
  `ready`.
- Data-plane drills (`data_plane_unavailable`, `auth_loss`) surface
  their cards as `local_only` or `unavailable`, drag overall local
  continuity to `local_safe_read_only`, and quote the workflow tokens
  the user actually feels (`workspace_sync`, `remote_shell`).

Surfaces that collapse these two planes into one chrome label fail the
fixture-replay test.

## Stale-status ban

Cards whose contract state is `stale`, or whose probe last-checked age
is `stale`, `very_stale`, or `never_checked`, MUST light the honesty
marker. The fixture-replay test asserts this for every card in every
drill. Notably:

- The `stale_cache` drill carries three cards whose probes are >24h old.
  Each card surfaces as `stale` with `very_stale` age and the honesty
  marker; the chrome cannot paint them as current ready truth.
- The `recovery_after_restart` drill carries an AI assist card with
  `last_checked: null` (the post-restart probe has not yet returned).
  The aggregator surfaces `never_checked`, the honesty marker is on,
  and the chrome cannot reuse the pre-restart "ready" cache.

## Mirror-only fallback and offline guidance

The `mirror_only_fallback` drill proves that when a primary path is
unreachable but a cached mirror is serving, the card surfaces:

- `contract_state` = `local_only` (not `unavailable`);
- `local_continuity` = `local_safe`;
- `detail_tokens` include `fallback_mode:mirror_only`;
- `state_explanation` quotes the mirror keeping the workflow usable,
  not generic "service unavailable" copy.

This is the difference between a chrome that tells the user "your work
is safe; cached browse is keeping you going" and a chrome that lights a
generic yellow chip with no actionable guidance.

## Recovery after restart

The `recovery_after_restart` drill proves that fresh post-restart probes
REPLACE the cached "down" state instead of being added on top of it.
After restart, sync, marketplace, and the license broker are reachable
again; the cards surface `ready` with `fresh` last-checked age. The AI
assist card is honestly `never_checked` because the post-restart probe
has not yet returned. The honesty marker stays on for that one card
until the probe lands; the chrome cannot fabricate "all green" before
the probes return.

## Cross-surface parity

The aggregator record is the cross-tool boundary. Every drill is read
verbatim by:

- the desktop shell service-health pane, About card, and diagnostics
  inspector;
- the CLI / headless inspect bin
  (`aureline_shell_service_health_inspect`);
- the continuity emitter bin
  (`aureline_shell_service_health_continuity_corpus`);
- the support-export plaintext block;
- the release-truth contract matrix at
  `artifacts/release/m3/service_health_contract_matrix.json`.

If any surface diverges from the aggregator's tokens or labels it is a
contract violation, not a presentation choice.

## How to refresh the corpus

```sh
cargo run -q -p aureline-shell \
  --bin aureline_shell_service_health_continuity_corpus -- emit-fixtures \
  fixtures/ops/m3/service_health_continuity
```

The fixture-replay test
(`crates/aureline-shell/tests/service_health_continuity_fixtures.rs`)
will fail if the on-disk JSON drifts from the in-code corpus, so
refreshing the fixtures is mandatory whenever the corpus changes.

## Cross-references

- Corpus module:
  `crates/aureline-shell/src/service_health/continuity_corpus.rs`
- Aggregator module:
  `crates/aureline-shell/src/service_health/aggregator.rs`
- Aggregator schema: `schemas/ops/service_health_card.schema.json`
- Contract-state vocabulary schema:
  `schemas/ops/service_contract_state.schema.json`
- Beta contract doc:
  `docs/ops/m3/service_health_contract_beta.md`
- Claim qualification doc:
  `docs/ops/m3/service_health_claim_qualification.md`
- Contract matrix:
  `artifacts/release/m3/service_health_contract_matrix.json`
