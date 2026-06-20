# Reactive-state truth certification

This document describes the reactive-state certification lane for claimed M5
surface profiles. The canonical packet is implemented in
[`crates/aureline-reactive-state/src/m5_reactive_certification/mod.rs`](../../crates/aureline-reactive-state/src/m5_reactive_certification/mod.rs)
and serialized to
[`artifacts/state/m5-reactive-proof-packet.json`](../../artifacts/state/m5-reactive-proof-packet.json).

It composes the reactive-state packets already frozen in this batch:

- the cross-surface subscription contract at
  [`artifacts/state/cross_surface_subscription.json`](../../artifacts/state/cross_surface_subscription.json),
- the gated reactive-truth cues at
  [`artifacts/state/reactive_truth_surfaces.json`](../../artifacts/state/reactive_truth_surfaces.json),
- the materialized-view policy at
  [`artifacts/state/materialized_view_policy.json`](../../artifacts/state/materialized_view_policy.json),
- the lagging-consumer recovery flows at
  [`artifacts/state/reactive_recovery.json`](../../artifacts/state/reactive_recovery.json),
- the command and mutation-journal publication parity at
  [`artifacts/state/reactive_command_parity.json`](../../artifacts/state/reactive_command_parity.json),
- the canonical reactive-governance matrix and narrowing engine at
  [`artifacts/state/m5_reactive_governance.json`](../../artifacts/state/m5_reactive_governance.json).

## Why this exists

Those packets prove the *mechanisms* of reactive-state truth. What they leave
implicit is whether any given product profile has actually proven all of them.
Without one certification lane, a `stable` or `beta` claim for the shell,
search, graph, AI, review, or support surface could outrun its evidence — staying
green because it worked once on a happy-path fixture while the reactive-state
evidence behind it is stale or incomplete.

This lane closes that loophole. It turns reactive-state truth into a
promotion-grade claim per claimed profile and narrows the claim automatically
when the backing evidence goes partial, stale, or missing.

## The certified dimensions

Every claimed profile must prove five reactive-state dimensions. A profile may
not present derived state as product truth unless all five are canonical and
testable:

- **`authority_class`** — the surface declares which authority owns its truth and
  whether it is authoritative or a derived projection.
- **`epoch_parity`** — the surface reads the shared authoritative epoch for its
  authority class and narrows rather than presenting a parallel epoch as truth.
- **`invalidation_behavior`** — the surface honors its invalidation reasons and
  recovers lagging consumers without offering stale exact-truth actions.
- **`stale_state_labeling`** — the surface labels warming, cached, stale, partial,
  and coalesced state instead of implying exact current truth.
- **`safe_action_narrowing`** — the surface narrows the actions it offers under
  degraded state instead of offering stale exact-truth affordances.

## The narrowing engine

Each dimension carries an `evidence_state`. One engine —
`certify_row_outcome` — folds the per-dimension evidence into a single verdict
and an effective maturity floor. It is the only place the downgrade rule lives;
the rows, the drills, the fixtures, and the freshness rules all read it.

| Evidence state | Maturity floor | Effect on the claim |
| --- | --- | --- |
| `current` | none | the claim holds at its claimed maturity |
| `partial` | `beta` | the claim narrows to at most beta |
| `stale` | `preview` | the claim narrows to at most preview |
| `missing` | `withdrawn` | the claim is withheld; promotion fails |
| `not_applicable` | none | the dimension does not constrain the claim |

The effective maturity is the worst (narrowest) of the claimed maturity and
every triggered floor. The verdict follows:

- **`certified`** — the effective maturity equals the claimed maturity.
- **`narrowed`** — the effective maturity is below the claimed maturity but the
  claim still holds (beta or preview).
- **`withheld`** — a required dimension is missing, so the claim is withdrawn.

The certification only ever narrows. It never promotes a profile above its
claimed maturity, and a profile absent from the packet is uncertified rather than
implicitly green.

## Certified profiles

| Profile | Claimed maturity | Backing surfaces |
| --- | --- | --- |
| `shell` | `stable` | workspace tree, activity center |
| `search` | `stable` | search results |
| `graph` | `beta` | graph neighborhood |
| `ai` | `beta` | AI context panel |
| `review` | `beta` | review workspace overlay |
| `support` | `beta` | support reactive-state export |

In the checked-in packet every dimension is `current`, so every profile is
`certified` at its claimed maturity.

## Failure and recovery drills

Each profile carries one failure / recovery drill. A drill injects a failure
into one dimension, observes the degraded evidence, watches the claim narrow or
withhold, refreshes the evidence, and recovers to `certified`. The degraded
posture is computed from the same engine the rows use, so a drill can never
disagree with the certification. The drill set covers an epoch lag, partial
invalidation coverage, stale labeling, a rolled policy epoch, missing authority
evidence (withheld), and an unavailable capture provider.

## One packet for every surface

Release/shiproom, support export, docs, and help all bind to this packet rather
than re-deriving reactive-state staleness. Each binding preserves the per-row
verdict, effective maturity, and narrowing tokens verbatim, and narrows in
lockstep with the packet, so the product tells one consistent story about its
reactive-state guarantees.

## Regeneration

The proof packet and fixtures are projections of the seeded packet:

```bash
cargo run -q -p aureline-reactive-state --example dump_m5_reactive_certification -- packet \
  | python3 -c 'import json,sys; print(json.dumps(json.load(sys.stdin), indent=2, sort_keys=True))' \
  > artifacts/state/m5-reactive-proof-packet.json
```

The fixture corpus under
[`fixtures/state/m5-reactive-certification/`](../../fixtures/state/m5-reactive-certification/)
is generated the same way from the `fixtures` mode and split one file per
fixture. The replay gate in
[`crates/aureline-reactive-state/tests/m5_reactive_certification.rs`](../../crates/aureline-reactive-state/tests/m5_reactive_certification.rs)
fails CI if the artifact or fixtures drift from the seeded packet.
