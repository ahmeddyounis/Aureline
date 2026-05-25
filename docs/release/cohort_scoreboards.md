# Cohort scoreboards and signoff loops

This document is the reviewer-facing companion for the cohort scoreboards packet:

- [`/artifacts/release/cohort_scoreboards.json`](../../artifacts/release/cohort_scoreboards.json)
- schema: [`/schemas/release/cohort_scoreboards.schema.json`](../../schemas/release/cohort_scoreboards.schema.json)
- proof packet:
  [`/artifacts/release/m4/cohort_scoreboards_proof_packet.md`](../../artifacts/release/m4/cohort_scoreboards_proof_packet.md)

The packet is the **canonical truth** for the design-partner, certified-archetype,
and stable-cohort scoreboards. It finalizes one qualification-and-publication model
for those scoreboards instead of side spreadsheets, stale badges, or optimistic
launch language. Downstream docs, Help/About surfaces, shiproom panels, release
packets, and support exports MUST ingest the packet by `scoreboard_id` and render
`effective_label` rather than restating the status in prose.

## Why this packet exists

The stable claim manifest decides the single lifecycle label each public subject
publishes. The stable qualification matrix and proof index ground the surfaces
that are meant to ship at the stable cutline. Cohort scoreboards add the human
signoff-loop control beside those gates: a design-partner, certified-archetype, or
stable-cohort row must not widen a public claim merely because a neighbouring row
is green. If a row's packet is stale, its metric fails, its waiver lapses, or its
required signoffs are incomplete, its effective label narrows below the cutline
before publication.

## Scoreboard rows

Each `row` is one `(scoreboard, public claim)` binding. It names:

- the scoreboard lane (`design_partner`, `certified_archetype`, `stable_cohort`)
  and the subject family it governs;
- whether it belongs to the release-blocking scoreboard set;
- the stable claim manifest entry it backs (`claim_ref`) and the canonical
  lifecycle label that entry publishes (`claim_label`);
- the `scoreboard_packet`, with its freshness SLO and recorded `slo_state`;
- the measurable `metrics` the row must clear, each with a `threshold` and a
  `measured` value (or `null` when no measurement has been captured);
- the required human `signoff_loop` (cadence, packet, and required roles);
- any waiver, the owner sign-off, active gap reasons, and the `effective_label`
  product surfaces render.

The lifecycle vocabulary is shared with the stable claim matrix:
`lts`, `stable`, `beta`, `preview`, and `withdrawn`.

## The launch cutline

The cutline fixes the boundary between a row that renders as Stable or LTS and one
narrowed below it:

```text
lts > stable   |   beta > preview > withdrawn   (below the cutline)
```

A row renders at or above the cutline only when it carries a captured scoreboard
packet within its freshness SLO, every metric clears its threshold, the row owner
has signed, every required signoff is complete, no waiver it relies on has expired,
and its backing public claim is itself at or above the cutline. Otherwise the
packet narrows the row to `beta`, `preview`, or `withdrawn`.

## Packet-freshness SLO {#packet-freshness-slo}

Each `scoreboard_packet` carries:

- `target_max_age_days` — the maximum age before the packet is stale;
- `warn_within_days` — the remaining-days threshold for `due_for_refresh`;
- `slo_register_ref` — this section, the source of the packet freshness rule.

The packet uses a 90-day target with a 30-day warning window for cohort scoreboard
packets. The CI gate recomputes each packet's state from `captured_at` against the
packet `as_of` date and fails when the declared state is fresher than the clock
allows or when a signed-off row rides a breached packet.

## Scoreboard states

- `signed_off` — the row has current proof, passing metrics, an owner sign-off, and
  a complete signoff loop, and renders the public claim label.
- `signed_off_on_waiver` — the row renders the claim label only because an active,
  unexpired waiver covers a recorded gap.
- `narrowed_unbacked` — required evidence, an owner sign-off, a required signoff, or
  a metric is incomplete or below threshold.
- `narrowed_claim_narrowed` — the backing public claim is itself below the cutline,
  so the row inherits that ceiling.
- `narrowed_stale` — the scoreboard packet breached its freshness SLO (or is
  missing).
- `narrowed_waiver_expired` — a waiver the row relied on has expired.

## Gap reasons and stop rules

The closed gap-reason vocabulary is:

- `claim_label_narrowed`
- `scoreboard_evidence_incomplete`
- `scoreboard_packet_freshness_breached`
- `scoreboard_packet_missing`
- `waiver_expired`
- `owner_signoff_missing`
- `required_signoff_missing`
- `score_below_threshold`

Every reason has a stop `rule` watching for it. The `claim_label_narrowed` rule is
non-blocking because the stable claim manifest already narrowed the upstream claim.
The remaining reasons block promotion when they fire under a Stable or LTS public
claim: they indicate a scoreboard row that could be read as Stable but does not
have the proof, metrics, or signoffs to carry that label.

## Coverage

`release_blocking_scoreboard_refs` is the closed set of scoreboards the release
line must cover. The gate fails when:

- a declared release-blocking scoreboard has no row;
- a release-blocking row is not declared;
- a `scoreboard_id` appears on more than one row;
- any of the three lanes has no row.

This keeps a scoreboard from quietly dropping out of release control.

## Publication verdict

The `publication` block records the shiproom verdict for this packet. It is `hold`
when any blocking rule fires and `proceed` otherwise. The gate recomputes the
decision, `blocking_rule_ids`, `blocking_scoreboard_ids`, and summary counts and
fails on any drift.

At this revision the packet holds publication. The design-partner alpha-evidence
row has a breached packet, the legacy remote-SSH archetype has a metric below
threshold, and the extension-author cohort row relied on an expired waiver. All
three sit under claims still published Stable, so the packet narrows them below the
cutline and blocks promotion until their packets, metrics, or waivers are fixed or
the upstream public claims are narrowed.

## CI gate

Run:

```sh
python3 ci/check_cohort_scoreboards.py --repo-root .
```

The gate fails when closed vocabularies or the cutline drift; when a signed-off row
carries active gap reasons, stale proof, a failing metric, a missing owner
sign-off, an incomplete signoff loop, or an expired waiver; when a narrowed row
does not drop below the cutline; when a row renders wider than its public claim;
when the claim label disagrees with the stable claim manifest; when freshness or
waiver-expiry arithmetic is overstated; when coverage drops; when publication or
summary fields drift; or when referenced artifacts are missing. It also runs
negative drills and fixture cases under
[`/fixtures/release/cohort_scoreboards/`](../../fixtures/release/cohort_scoreboards/)
and writes
[`/artifacts/release/captures/cohort_scoreboards_validation_capture.json`](../../artifacts/release/captures/cohort_scoreboards_validation_capture.json).

Shiproom and release tooling can fail promotion directly from this artifact:

```sh
python3 ci/check_cohort_scoreboards.py --repo-root . --require-proceed
```

This exits with code 2 when the recomputed publication verdict is `hold`, distinct
from an invalid artifact failure.

The typed Rust consumer
(`aureline_release::finalize_design_partner_certified_archetype_and_stable_cohort::current_cohort_scoreboards`)
reads the same packet and exposes `support_export_projection()` for Help/About and
support export consumers, so `cargo test -p aureline-release` enforces the
structural invariants without a separate build step.

## Update rules

1. Capture or refresh the scoreboard packet first, then point the row at the
   packet, proof-index row, evidence refs, metrics, signoff loop, and owner
   sign-off.
2. Set `scoreboard_state`, `active_gap_reasons`, `slo_state`, and `effective_label`
   to the honest posture. A row with a stale packet, a failing metric, an expired
   waiver, an incomplete signoff loop, or a narrowed backing claim must display
   below the cutline.
3. Recompute the `publication` and `summary` blocks, run
   `python3 ci/check_cohort_scoreboards.py --repo-root . --check`, and commit the
   regenerated validation capture with the packet.
4. If the evidence supports only a narrower label, narrow the row and packet rather
   than preserving optimistic Stable wording.
