# Final go/no-go rehearsal with explicit cutline, exception packets, and rollback checkpoints

This document is the reviewer-facing companion for the final go/no-go rehearsal:

- [`/artifacts/release/go_no_go_rehearsal.json`](../../artifacts/release/go_no_go_rehearsal.json)
- schema: [`/schemas/release/go_no_go_rehearsal.schema.json`](../../schemas/release/go_no_go_rehearsal.schema.json)
- proof packet:
  [`/artifacts/release/m4/go_no_go_rehearsal_proof_packet.md`](../../artifacts/release/m4/go_no_go_rehearsal_proof_packet.md)

The rehearsal is the **canonical truth** for whether the release train was actually
exercised before the go/no-go: the explicit launch cutline signed off, the promotion
publish step dry-run, each rollback checkpoint verified to a restore point, and every open
exception packet reviewed. It locks one rehearsal-and-publication model for those stages
instead of side spreadsheets, stale badges, or optimistic launch language. Downstream docs,
Help/About surfaces, shiproom panels, release packets, and support exports MUST ingest the
rehearsal by `entry_id` and render `effective_label` rather than restating the status in
prose.

## Why this rehearsal exists

The stable claim manifest decides the single lifecycle label each public subject publishes.
The stable proof index and version windows ground the requirements and interface surfaces
that are meant to ship at the stable cutline. This rehearsal adds the launch-rehearsal
control beside those gates: a go/no-go must not return Go merely because a neighbouring
stage is green. If a stage's rehearsal packet is stale, a required rollback checkpoint is
unverified, its owner sign-off is missing, or its exception packet lapses, its effective
label narrows below the cutline to a No-Go before publication.

## Rehearsal stage kinds

Each `row` belongs to one of four closed stage kinds:

- `cutline_review` — the explicit launch-cutline review and go/no-go signoff that fixes
  which surfaces sit at or above Stable versus narrowed below it;
- `promotion_step` — the promotion publish-step dry run and roll-forward recovery path,
  exercised against the live train;
- `rollback_checkpoint` — the rollback drills that restore a previous build or pre-update
  state from a verified rollback checkpoint;
- `exception_review` — the review that confirms every open launch exception packet is
  recorded, time-boxed, and owner-attributed.

## Rehearsal rows

Each `row` is one `(rehearsal stage, public claim)` binding. It names:

- the stage kind and the subject it rehearses (`subject_ref`, `subject_summary`);
- whether it belongs to the release-blocking rehearsal set;
- the stable claim manifest entry it backs (`claim_ref`) and the canonical lifecycle label
  that entry publishes (`claim_label`);
- the `rehearsal_packet`, with its freshness SLO and recorded `slo_state`;
- the required `rollback_checkpoints` the stage must verify, each with a `verified` flag and
  the `restore_point_ref` it checks;
- any exception packet, the owner sign-off, active gap reasons, and the `effective_label`
  product surfaces render.

The lifecycle vocabulary is shared with the stable claim matrix:
`lts`, `stable`, `beta`, `preview`, and `withdrawn`.

## The launch cutline

The cutline fixes the boundary between a stage that renders a Go at Stable or LTS and one
narrowed below it to a No-Go:

```text
lts > stable   |   beta > preview > withdrawn   (below the cutline)
```

A stage returns Go at or above the cutline only when it carries a captured rehearsal packet
within its freshness SLO, every required rollback checkpoint is verified, the stage owner
has signed, no exception packet it relies on has expired, and its backing public claim is
itself at or above the cutline. Otherwise the rehearsal narrows the stage to `beta`,
`preview`, or `withdrawn`.

## Packet-freshness SLO {#packet-freshness-slo}

Each `rehearsal_packet` carries:

- `target_max_age_days` — the maximum age before the packet is stale;
- `warn_within_days` — the remaining-days threshold for `due_for_refresh`;
- `slo_register_ref` — this section, the source of the packet freshness rule.

The rehearsal uses a 90-day target with a 30-day warning window for rehearsal packets. The
CI gate recomputes each packet's state from `captured_at` against the rehearsal `as_of` date
and fails when the declared state is fresher than the clock allows or when a Go stage rides
a breached packet.

## Rehearsal states

- `go_rehearsed` — the stage was rehearsed end-to-end with current proof, verified
  checkpoints, and an owner sign-off, and renders the public claim label.
- `go_on_exception` — the stage returns Go at the claim label only because an active,
  unexpired exception packet covers a recorded residual gap.
- `no_go_unrehearsed` — a required rollback checkpoint is unverified, the evidence is
  incomplete, or the owner sign-off is missing.
- `no_go_claim_narrowed` — the backing public claim is itself below the cutline, so the
  stage inherits that ceiling.
- `no_go_stale` — the rehearsal packet breached its freshness SLO (or is missing).
- `no_go_exception_expired` — an exception packet the stage relied on has expired.

## Gap reasons and stop rules

The closed gap-reason vocabulary is:

- `claim_label_narrowed`
- `rehearsal_evidence_incomplete`
- `rehearsal_packet_freshness_breached`
- `rehearsal_packet_missing`
- `exception_expired`
- `owner_signoff_missing`
- `rollback_checkpoint_unverified`

Every reason has a stop `rule` watching for it. The `claim_label_narrowed` rule is
non-blocking because the stable claim manifest already narrowed the upstream claim. The
remaining reasons hold the go/no-go when they fire under a Stable or LTS public claim: they
indicate a rehearsal stage that could be read as Go but does not have the proof, verified
checkpoints, or sign-off to carry that verdict.

## Coverage

`release_blocking_stage_refs` is the closed set of rehearsal stages the release line must
cover. The gate fails when:

- a declared release-blocking stage has no row;
- a release-blocking row is not declared;
- an `entry_id` or a `subject_ref` appears on more than one row;
- any of the four stage kinds has no row.

This keeps a rehearsal stage from quietly dropping out of release control.

## Go/no-go verdict

The `publication` block records the shiproom verdict for this rehearsal. It is `hold` when
any blocking rule fires and `proceed` otherwise. The gate recomputes the decision,
`blocking_rule_ids`, `blocking_entry_ids`, and summary counts and fails on any drift.

At this revision the rehearsal holds the go/no-go. The rollback state-integrity drill has an
unverified history-integrity checkpoint, the promotion roll-forward recovery rehearsal
breached its freshness SLO, and the open exception-packet review relied on an expired
exception packet. All three sit under claims still published Stable, so the rehearsal
narrows them below the cutline and blocks promotion until their checkpoints, packets, or
exception packets are fixed or the upstream public claims are narrowed.

## CI gate

Run:

```sh
python3 ci/check_go_no_go_rehearsal.py --repo-root .
```

The gate fails when closed vocabularies or the cutline drift; when a Go stage carries active
gap reasons, stale proof, an unverified checkpoint, or a missing owner sign-off; when a
No-Go stage does not drop below the cutline; when a stage renders wider than its public
claim; when the claim label disagrees with the stable claim manifest; when freshness or
exception-expiry arithmetic is overstated; when coverage drops; when publication or summary
fields drift; or when referenced artifacts are missing. It also runs negative drills and
fixture cases under
[`/fixtures/release/go_no_go_rehearsal/`](../../fixtures/release/go_no_go_rehearsal/)
and writes
[`/artifacts/release/captures/go_no_go_rehearsal_validation_capture.json`](../../artifacts/release/captures/go_no_go_rehearsal_validation_capture.json).

Shiproom and release tooling can fail promotion directly from this artifact:

```sh
python3 ci/check_go_no_go_rehearsal.py --repo-root . --require-proceed
```

This exits with code 2 when the recomputed go/no-go verdict is `hold`, distinct from an
invalid artifact failure.

The typed Rust consumer
(`aureline_release::go_no_go_rehearsal::current_go_no_go_rehearsal`) reads the same
rehearsal and exposes `support_export_projection()` for Help/About and support export
consumers, so `cargo test -p aureline-release` enforces the structural invariants without a
separate build step.

## Update rules

1. Capture or refresh the rehearsal packet first, then point the row at the packet,
   proof-index row, evidence refs, rollback checkpoints, and owner sign-off.
2. Set `rehearsal_state`, `active_gap_reasons`, `slo_state`, and `effective_label` to the
   honest posture. A stage with a stale packet, an unverified checkpoint, an expired
   exception packet, or a narrowed backing claim must display below the cutline.
3. Recompute the `publication` and `summary` blocks, run
   `python3 ci/check_go_no_go_rehearsal.py --repo-root . --check`, and commit the
   regenerated validation capture with the rehearsal.
4. If the rehearsal supports only a narrower verdict, narrow the stage and packet rather
   than preserving optimistic Go wording.
