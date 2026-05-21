# Stable claim matrix, launch cutline, qualification rows, and shiproom stop rules

This document is the reviewer-facing companion for the gated stable claim
matrix:

- [`/artifacts/release/stable_claim_matrix.json`](../../artifacts/release/stable_claim_matrix.json)
- schema: [`/schemas/release/stable_claim_matrix.schema.json`](../../schemas/release/stable_claim_matrix.schema.json)

The matrix is the **canonical truth** for which surfaces may publish as Stable.
It freezes one publication and qualification model so the stable line exits on a
single record instead of side spreadsheets, stale badges, or optimistic launch
language. Downstream dashboards, docs, Help/About surfaces, release packets, and
support exports MUST ingest this matrix by `claim_id` rather than cloning its
status text.

It is the stable-line analog of the upstream beta cutline packet
([`/docs/milestones/m3/cutline_packet.md`](../milestones/m3/cutline_packet.md)),
which froze *what must be green before any stable lane is allowed to start*. Each
row here names the upstream cutline rows it inherits from in `cutline_row_refs`.
It does not re-mint the assurance-claim vocabulary; the assurance-claim matrix
([`/docs/release/assurance_claim_matrix.md`](./assurance_claim_matrix.md))
remains the contract for claim *language*, and this matrix decides which
surfaces have *earned* a stable claim and what happens when they have not.

## The launch cutline

The cutline fixes the boundary between "claimed Stable" and "narrowed below
Stable". The level vocabulary, strongest to weakest, is:

```
lts > stable | beta > preview > withdrawn
```

- **At or above the cutline:** `lts`, `stable`. A row at these levels publishes
  as a stable claim.
- **Below the cutline:** `beta`, `preview`, `withdrawn`. A row that has not
  earned its claim drops here.

A row's `claimed_level` is the level it is put forward as (always at or above the
cutline). Its `effective_level` is the level it actually holds after narrowing.
The effective level may never be **wider** (stronger) than the claimed level —
narrowing is always admissible, widening is forbidden.

## Qualification rows

Every row carries a qualification row: the proof refs that back it
(`evidence_refs`), the stable proof-index entry it is registered under
(`proof_index_ref`), when the evidence was captured (`captured_at`), the
freshness window after which it stops being claim-bearing
(`freshness_window_days`), an optional `waiver`, and an `owner_signoff`.

The `qualification_state` is the verdict for that row:

- `qualified` — full, current proof with owner sign-off; holds the claimed
  level.
- `provisional_on_waiver` — holds the claimed level only because an active,
  unexpired waiver covers a recorded gap.
- `not_qualified` — required proof is absent; the row must narrow.
- `evidence_stale` — proof exists but its freshness window expired; the row must
  narrow.
- `waiver_expired` — the row relied on a waiver that has expired; the row must
  narrow.

A row whose state forces narrowing MUST drop below the cutline and name at least
one active downgrade reason. A row that holds a stable claim MUST have current,
proof-backed, owner-signed qualification with **no** active downgrade reason. No
stable claim widens without a fresh packet, a downgrade rule, and an owner
sign-off.

## Downgrade reasons

The closed reason vocabulary (mirrored in the schema and the typed model) is:

- `qualification_evidence_missing`
- `qualification_evidence_stale`
- `waiver_expired`
- `freshness_window_exceeded`
- `owner_signoff_missing`
- `compatibility_row_degraded`
- `blocking_defect_open`

## Shiproom stop rules

Each stop rule names one downgrade reason as its `trigger_reason`, the claimed
levels it watches (`applies_to_levels`), a `default_action`, and whether it
`blocks_promotion`. A rule **fires** when any claimed row in its watch set
carries its trigger reason. Every downgrade reason has a stop rule watching for
it, so a narrowing reason can never fire without a corresponding promotion gate.

The default-action vocabulary is `hold_promotion`, `narrow_claim`,
`refresh_evidence_packet`, `staff_correction_lane`, `block_milestone_close`.

## Promotion verdict

The `promotion` block records the verdict for the stable train. It is `hold`
when any blocking stop rule fires and `proceed` otherwise. The `blocking_rule_ids`
and `blocking_claim_ids` enumerate the firing rules and the rows that triggered
them. The gate recomputes all three and fails on any drift, so the verdict can
never overstate readiness.

At this revision the matrix carries three surfaces narrowed below the cutline
(missing, stale, and waiver-expired qualification), so four blocking stop rules
fire and the stable train is held. That is the honest posture: the repository is
pre-implementation and most surfaces have not yet earned a stable claim.

## CI gate

Run:

```sh
python3 ci/check_stable_claim_matrix.py --repo-root .
```

The gate fails when a closed vocabulary or the cutline drifts, when a row that is
not stable-qualified does not narrow below the cutline, when a held stable claim
carries an active downgrade reason or lacks evidence or owner sign-off, when a
row overstates its posture against the `as_of` date (holds a claim on an expired
waiver or stale evidence), when the promotion verdict or blocking sets disagree
with the firing stop rules, when the summary counts drift, or when a referenced
artifact does not exist. It also runs negative drills proving the narrowing,
stop-rule, waiver-expiry, and promotion rejections all fire, and writes a
validation capture to
[`/artifacts/release/captures/stable_claim_matrix_validation_capture.json`](../../artifacts/release/captures/stable_claim_matrix_validation_capture.json).

Shiproom and release tooling can fail promotion directly from this artifact:

```sh
python3 ci/check_stable_claim_matrix.py --repo-root . --require-proceed
```

This exits non-zero (code 2) whenever the recomputed promotion verdict is
`hold`, distinct from an invalid-artifact failure (code 1).

The typed Rust consumer
(`aureline_release::stable_claim_matrix::current_stable_claim_matrix`) reads the
same matrix and runs the same structural cross-check, and exposes a
redaction-safe `support_export_projection()` for Help/About and support surfaces,
so `cargo test -p aureline-release` enforces these invariants without a cargo
build in CI.

## Update rules

1. Land qualification evidence and waivers first; point each row's
   `evidence_refs` and `proof_index_ref` at the canonical packets.
2. Set each row's `qualification_state`, `active_downgrade_reasons`, and
   `effective_level` to the honest posture. A row that is not qualified narrows
   below the cutline.
3. Recompute the `promotion` block and `summary`, then run
   `python3 ci/check_stable_claim_matrix.py --repo-root .` and commit the
   regenerated capture in the same change set.
4. If delivery proves a narrower stable claim than planned, narrow the claim and
   update the matrix — do not paper over the gap with prose.
