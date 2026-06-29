# Descriptor / badge matrix

Claimed M5 release, ecosystem, docs, and companion surfaces all render the same
public-truth state: where an artifact came from, how fresh the evidence behind it is,
what support class it qualifies for, and which client scope it runs in. Before this lane
that vocabulary was split across local enums, prose, and ad hoc badges, so a surface
could quietly claim more than its evidence supports. This document is the contract for the
one shared **descriptor and badge runtime** those surfaces consume.

The `m5_descriptor_badge` module in `aureline-release` owns the model and publishes the one
matrix packet every consumer reads. The machine-readable inventory lives at
[`artifacts/public-truth/m5-descriptor-badge-matrix.json`](../../artifacts/public-truth/m5-descriptor-badge-matrix.json)
(human governance matrix:
[`artifacts/public-truth/m5-descriptor-badge-governance.md`](../../artifacts/public-truth/m5-descriptor-badge-governance.md)),
the release parity proof at
[`artifacts/release/m5-descriptor-parity-proof/descriptor-badge-matrix.json`](../../artifacts/release/m5-descriptor-parity-proof/descriptor-badge-matrix.json),
and the packet schema is
[`schemas/provenance/m5-descriptor-badge-matrix.schema.json`](../../schemas/provenance/m5-descriptor-badge-matrix.schema.json).
The matrix is **derived** from the same checked-in descriptor proofs, so a consumer's
qualification can never claim more than the descriptor proof under it supports.

## Four shared descriptor objects

Each descriptor family is a reusable object with a frozen value vocabulary, a badge family,
and an explanation drawer. Each object names its source-of-truth schema, the proof packet
that keeps it current, the owner accountable for it, its first consumer, and a freshness
state. The standalone descriptor objects are checked in under
[`artifacts/public-truth/descriptors/`](../../artifacts/public-truth/descriptors/).

| Descriptor | Schema | First consumer | Value vocabulary |
|------------|--------|----------------|------------------|
| `provenance` | [`m5-provenance-descriptor`](../../schemas/provenance/m5-provenance-descriptor.schema.json) | `help_about` | `first_party_signed`, `vendor`, `community`, `mirror`, `offline_bundle`, `side_loaded`, `not_provided` |
| `freshness` | [`m5-freshness-descriptor`](../../schemas/provenance/m5-freshness-descriptor.schema.json) | `release_center` | `current`, `stale`, `expired`, `missing` |
| `qualification` | [`m5-qualification-descriptor`](../../schemas/provenance/m5-qualification-descriptor.schema.json) | `release_center` | `stable`, `beta`, `preview`, `experimental`, `deprecated`, `unavailable` |
| `client_scope` | [`m5-client-scope-descriptor`](../../schemas/provenance/m5-client-scope-descriptor.schema.json) | `companion_handoff` | `desktop_full`, `companion_scoped`, `mobile_companion`, `embedded_panel`, `browser_reference`, `handoff_only` |

Each descriptor family maps 1:1 to a badge family (`provenance_badge`, `freshness_badge`,
`qualification_badge`, `client_scope_badge`) whose member badges are exactly the
descriptor's value tokens, so a badge always resolves to a value rather than to
hand-authored copy. The weaker provenance origins — `mirror`, `offline_bundle`,
`side_loaded`, and `not_provided` — are first-class tokens: a weaker or absent origin can
never disappear into omission.

The badge families' user-facing labels and explanation-drawer bodies — `Official`,
`Mirrored`, `Side-loaded`, `Signature verified`, `Attestation available`, `Not provided`,
`Partial`, `Certified`, `Supported`, `Limited`, `Experimental`, `Retest pending`,
`Evidence stale`, and the rest — resolve through the shared
[badge vocabulary](m5-badge-vocabulary.md), which is where the
`explanation_drawer_message_id` each descriptor carries is rendered. The badge vocabulary is
the one place those drawers are written, so every consumer surface reads the same expansion
text and export-safe ids.

## Downgrade rules

The frozen downgrade rules name how a non-authoritative descriptor value narrows or blocks
a claim. The rule set is the published downgrade vocabulary; the conformance review proves
it covers every non-authoritative origin, every non-current freshness state, and every
narrowed client scope.

| Trigger | Effect | Floor |
|---------|--------|-------|
| provenance below `first_party_signed` (`vendor`, `community`, `mirror`, `offline_bundle`, `side_loaded`) | `narrow` | `beta` |
| provenance `not_provided` | `block` | `unavailable` |
| freshness `stale` | `narrow` | `beta` |
| freshness `expired` / `missing` | `block` | `unavailable` |
| client scope below `desktop_full` | `narrow` | `beta` |

Stale or weaker evidence narrows a claim automatically; absent provenance and
expired/missing evidence block it. A narrowed client scope narrows the claim so it can
never imply authority or capability parity it lacks.

## Consumers bind the descriptors they render

Each claimed **public-truth consumer** declares the descriptor families it renders. The
matrix derives, per consumer, the exact coverage gaps, a gate decision, and an effective
qualification from those descriptors' freshness:

| Descriptor freshness on a bound family | Gap | Consumer gate | Effective qualification |
|----------------------------------------|-----|---------------|-------------------------|
| `current` | — | `governed` | the claimed class (e.g. `stable`) |
| `stale` | `proof_stale` | `narrowed` | floored at `beta` |
| `expired` | `proof_expired` | `blocked` | `unavailable` |
| `missing` | `proof_missing` | `blocked` | `unavailable` |
| family not governed at all | `descriptor_mapping_missing` | `blocked` | `unavailable` |

The narrowing is **deterministic**: a stale descriptor proof always narrows every consumer
that binds it below Stable, and a missing/expired or unmapped descriptor always blocks
them. Stable promotion therefore fails when a claimed consumer lacks a mapped descriptor
contract or a current proof — and the cause is always named in the consumer's `gaps`
rather than hidden.

## Auto-narrowing in practice

Two drills under
[`fixtures/public-truth/m5-badge-consumers/`](../../fixtures/public-truth/m5-badge-consumers/)
exercise the gate:

- **stale-proof drill** marks the `freshness` descriptor `stale`. Every consumer that binds
  `freshness` (release center, Help/About, certification, evaluation packs, support export,
  and companion handoff) auto-narrows to `beta`; consumers that do not bind it (marketplace,
  docs/help) stay `stable`.
- **missing-proof drill** marks the `client_scope` descriptor `missing`. Every consumer that
  binds `client_scope` (release center, marketplace, certification, support export, and
  companion handoff) is blocked from Stable promotion, with `proof_missing` named on the
  consumer.

The packet-level `release_gate` aggregates the per-consumer gates, so the release/public-
truth automation reads one `blocks_stable_promotion` signal plus the exact blocked,
narrowed, and governed consumer tokens.

## One runtime, every surface

The release center, Help/About, marketplace, docs/help, support exports, and companion
handoffs all consume *this* runtime rather than parallel badge or copy vocabularies — the
`disclosure` block records that every one of those surfaces reads the same machine-readable
descriptors. The packet carries metadata and refs only: no credential bodies and no raw
provider payloads.

The headless emitter is the only mint-from-truth path:

```sh
cargo run -q -p aureline-release --bin aureline_release_m5_descriptor_badge_matrix -- support-export
cargo run -q -p aureline-release --bin aureline_release_m5_descriptor_badge_matrix -- markdown
cargo run -q -p aureline-release --bin aureline_release_m5_descriptor_badge_matrix -- descriptor provenance
cargo run -q -p aureline-release --bin aureline_release_m5_descriptor_badge_matrix -- fixture-stale-proof-narrowed
cargo run -q -p aureline-release --bin aureline_release_m5_descriptor_badge_matrix -- fixture-missing-proof-blocked
cargo run -q -p aureline-release --bin aureline_release_m5_descriptor_badge_matrix -- validate
```
