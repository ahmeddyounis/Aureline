# Reactive-state certification proof packet

The canonical reactive-state certification packet is implemented in
[`crates/aureline-reactive-state/src/m5_reactive_certification/mod.rs`](../../crates/aureline-reactive-state/src/m5_reactive_certification/mod.rs)
and serialized to
[`artifacts/state/m5-reactive-proof-packet.json`](./m5-reactive-proof-packet.json).

It is the checked-in truth source for:

- the reviewer contract in
  [`docs/state/m5-reactive-certification.md`](../../docs/state/m5-reactive-certification.md)
- the boundary schema at
  [`schemas/state/m5-reactive-certification.schema.json`](../../schemas/state/m5-reactive-certification.schema.json)
- fixture replay in
  [`crates/aureline-reactive-state/tests/m5_reactive_certification.rs`](../../crates/aureline-reactive-state/tests/m5_reactive_certification.rs)
- the fixture corpus under
  [`fixtures/state/m5-reactive-certification/`](../../fixtures/state/m5-reactive-certification/)

## What the packet certifies

For each claimed M5 surface profile — shell, search, graph, AI, review, and
support — the packet proves the five required reactive-state dimensions
(authority class, epoch parity, invalidation behavior, stale-state labeling, and
safe-action narrowing) and stamps the verdict the narrowing engine reaches.

A profile is `certified` only when every dimension is `current`. Partial evidence
narrows the claim to `beta`; stale evidence narrows it to `preview`; missing
evidence withholds the claim. The certification only narrows — it never widens a
claim, and a profile absent from the packet is uncertified rather than green.

## Certified rows

| Row | Profile | Claimed | Effective | Verdict |
| --- | --- | --- | --- | --- |
| `cert.reactive.shell` | shell | `stable` | `stable` | `certified` |
| `cert.reactive.search` | search | `stable` | `stable` | `certified` |
| `cert.reactive.graph` | graph | `beta` | `beta` | `certified` |
| `cert.reactive.ai` | ai | `beta` | `beta` | `certified` |
| `cert.reactive.review` | review | `beta` | `beta` | `certified` |
| `cert.reactive.support` | support | `beta` | `beta` | `certified` |

## Automatic narrowing rules

| Trigger evidence | Maturity floor |
| --- | --- |
| `partial` | `beta` |
| `stale` | `preview` |
| `missing` | `withdrawn` |

## Failure and recovery drills

One drill per profile injects a failure into a backing dimension, narrows or
withholds the claim, then recovers to `certified` after the evidence is
refreshed. The drills cover an epoch lag (shell → preview), partial invalidation
coverage (search → beta), stale labeling (graph → preview), a rolled policy
epoch (AI → preview), missing authority evidence (review → withheld), and an
unavailable capture provider (support → preview).

## Publication bindings

Every binding ingests the same packet id (`state.m5_reactive_certification.v1`)
and preserves the per-row verdict, effective maturity, and narrowing tokens
verbatim:

- `release_shiproom` — holds promotion for any narrowed or withheld release-scope
  profile.
- `support_export` — re-exports the verdict and narrowing tokens with no raw
  payloads or ambient authority.
- `docs` — quotes the certified dimensions, freshness rules, and verdicts.
- `help` — reuses the same verdict vocabulary in the reactive-state explainer.
