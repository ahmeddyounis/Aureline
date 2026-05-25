# Final go/no-go rehearsal — proof packet

Reviewer-facing proof packet for the final go/no-go rehearsal that, for stable launch,
rehearses the release train end-to-end behind an explicit launch cutline, exception
packets, and rollback checkpoints as one canonical rehearsal, binds every rehearsal
stage to a public claim ceiling and a rehearsal packet, and narrows any stage whose
packet is stale, whose rollback checkpoint is unverified, whose owner sign-off is
missing, or whose exception packet expired before the stage can return Go or widen
release, docs, Help/About, or support-export language.

Canonical machine source (do not clone status text from this packet — ingest the JSON):

- Rehearsal: [`/artifacts/release/go_no_go_rehearsal.json`](../go_no_go_rehearsal.json)
- Schema: [`/schemas/release/go_no_go_rehearsal.schema.json`](../../../schemas/release/go_no_go_rehearsal.schema.json)
- Companion doc: [`/docs/release/go_no_go_rehearsal.md`](../../../docs/release/go_no_go_rehearsal.md)
- Validator: `ci/check_go_no_go_rehearsal.py`
- Validation capture:
  [`/artifacts/release/captures/go_no_go_rehearsal_validation_capture.json`](../captures/go_no_go_rehearsal_validation_capture.json)
- Typed consumer: `aureline_release::go_no_go_rehearsal`

This rehearsal is registered under the stable proof index through the
`stable_proof_index_ref` it carries and the `proof_index_ref` each row's rehearsal
packet carries (`artifacts/release/stable_proof_index.json#proof:*`), so a launch
reviewer reaches the cutline-signoff, promotion-step, rollback-checkpoint, and
exception-review evidence from the same proof index that grounds the launch-blocking
requirements, rather than from a side spreadsheet.

## What this rehearsal proves

1. **Each rehearsal stage binds a public claim to a rehearsal packet and rollback
   checkpoints.** Every row names its stage kind (`cutline_review`, `promotion_step`,
   `rollback_checkpoint`, `exception_review`), the stable claim manifest entry it backs
   (`claim_ref`, `claim_label`), its `rehearsal_packet` and freshness SLO, and the
   `rollback_checkpoints` whose every member must be verified to a restore point. The
   rehearsal reuses the stable claim level vocabulary rather than minting per-row labels,
   so docs, Help/About, shiproom, the release center, and support exports render one
   verdict per stage.

2. **A stage returns Go only when every gate is clean.** A stage may render at or above
   the cutline (`go_rehearsed` or `go_on_exception`) only when it carries a captured
   within-freshness-SLO rehearsal packet, every required rollback checkpoint is verified,
   the stage owner has signed, any exception packet it relies on is unexpired, and the
   public claim it backs is itself at or above the cutline. The typed model and the CI
   gate both enforce this.

3. **The rehearsal ingests the stable claim manifest as a hard ceiling.** The CI gate
   reads the stable claim manifest named by `claim_manifest_ref` and fails when a row's
   `claim_label` is not the label that manifest publishes for the entry named by
   `claim_ref`, when a row names an entry the manifest does not carry, or when a row
   renders wider than the public claim's canonical label. A stage's effective verdict can
   never outrun the public claim it backs.

4. **The packet-freshness, exception-expiry, and rollback-checkpoint stop rules narrow
   stages before the go/no-go.** Each packet carries a freshness SLO and a recorded
   `slo_state`. The CI gate recomputes the freshness state and the exception-expiry state
   against the rehearsal `as_of` date, failing when a declared state overstates the clock,
   when a Go stage rides a stale packet or an expired exception packet, when a required
   rollback checkpoint is unverified, or when an owner sign-off is missing under a Stable
   claim.

5. **The four stage kinds and the release-blocking set stay covered.** The gate fails if
   any of `cutline_review`, `promotion_step`, `rollback_checkpoint`, or `exception_review`
   has no row, if a declared release-blocking stage has no covering row, if a
   release-blocking row is not declared, or if an `entry_id` or `subject_ref` repeats.

6. **The go/no-go verdict is recomputed, not asserted.** The gate recomputes the
   `hold`/`proceed` decision and the blocking rule/entry sets from the firing stop rules
   and fails on any drift. With `--require-proceed` it exits non-zero on `hold`, so
   shiproom and release tooling fail promotion directly from this artifact.

## Current snapshot (as of 2026-05-24)

The checked-in rehearsal holds the go/no-go. Of eight rehearsal stages across four public
claims, three return Go and back Stable claims cleanly (the explicit cutline signoff, the
promotion publish-step dry run, and the live-hardware rollback checkpoint — the last on an
active exception packet). Five stages are narrowed below the cutline:

- the **export and offboarding** dry run inherits the ceiling from a public claim already
  published beta;
- the **rollback state-integrity** drill narrowed to a No-Go at beta because its
  history-integrity rollback checkpoint is unverified;
- the **regulated-environment exception review** inherits the ceiling from a public claim
  already published beta;
- the **promotion roll-forward recovery** rehearsal narrowed to a No-Go at beta because
  its rehearsal packet breached its freshness SLO; and
- the **open exception-packet review** narrowed to a No-Go at beta because the exception
  packet it relied on expired.

Three of those — the unverified checkpoint, the breached packet, and the expired exception
packet — back claims still published Stable, so they fire three blocking stop rules and
hold the `release.shiproom.go_no_go_rehearsal` gate. The rehearsal narrows the optimistic
Stable stages automatically instead of letting them ride; the go/no-go clears once the
history-integrity checkpoint is verified, the roll-forward rehearsal is re-run, and the
readiness-lane exception packet is renewed (or those public claims are formally narrowed).

## How to re-verify

```
python3 ci/check_go_no_go_rehearsal.py --repo-root . --check
cargo test -p aureline-release
```

The first command revalidates the rehearsal, recomputes the freshness/exception-expiry
automations and the rollback-checkpoint checks against `as_of`, runs the negative drills
and fixture cases, and writes the validation capture. The second runs the typed contract
tests that bind the model to the checked-in rehearsal. Add `--require-proceed` to the gate
to turn the recorded `hold` into a non-zero exit for shiproom use.
