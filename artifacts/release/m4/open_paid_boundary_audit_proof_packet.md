# Open/paid boundary audit — proof packet

Reviewer-facing proof packet for the governance-fact layer that audits, for stable
launch, where the open-source core ends and the paid/managed tier begins, the
licensing posture, the build provenance, and the contribution policy as one canonical
audit, binds every audited subject to a public claim ceiling and an attestation
packet, and narrows any row whose packet is stale, whose required control is
unsatisfied, whose owner sign-off is missing, or whose waiver expired before the row
can widen release, docs, Help/About, or support-export language.

Canonical machine source (do not clone status text from this packet — ingest the JSON):

- Audit: [`/artifacts/release/open_paid_boundary_audit.json`](../open_paid_boundary_audit.json)
- Schema: [`/schemas/release/open_paid_boundary_audit.schema.json`](../../../schemas/release/open_paid_boundary_audit.schema.json)
- Companion doc: [`/docs/release/open_paid_boundary_audit.md`](../../../docs/release/open_paid_boundary_audit.md)
- Validator: `ci/check_open_paid_boundary_audit.py`
- Validation capture:
  [`/artifacts/release/captures/open_paid_boundary_audit_validation_capture.json`](../captures/open_paid_boundary_audit_validation_capture.json)
- Typed consumer: `aureline_release::open_paid_boundary_audit`

This audit is registered under the stable proof index through the
`stable_proof_index_ref` it carries and the `proof_index_ref` each row's attestation
packet carries (`artifacts/release/stable_proof_index.json#proof:*`), so a launch
reviewer reaches the open/paid boundary, licensing, provenance, and contribution-policy
attestations from the same proof index that grounds the launch-blocking requirements,
rather than from a side spreadsheet.

## What this audit proves

1. **Each audited subject binds a public claim to an attestation packet and required
   controls.** Every row names its domain (`open_paid_boundary`, `licensing`,
   `provenance`, `contribution_policy`), the stable claim manifest entry it backs
   (`claim_ref`, `claim_label`), its `attestation_packet` and freshness SLO, and the
   `audit_controls` whose every member must be satisfied. The audit reuses the stable
   claim level vocabulary rather than minting per-row labels, so docs, Help/About,
   shiproom, the release center, and support exports render one label per row.

2. **A row attests only when every gate is clean.** A row may render at or above the
   cutline (`attested` or `attested_on_waiver`) only when it carries a captured
   within-freshness-SLO packet, every required control is satisfied, the row owner has
   signed, any waiver it relies on is unexpired, and the public claim it backs is itself
   at or above the cutline. The typed model and the CI gate both enforce this.

3. **The audit ingests the stable claim manifest as a hard ceiling.** The CI gate reads
   the stable claim manifest named by `claim_manifest_ref` and fails when a row's
   `claim_label` is not the label that manifest publishes for the entry named by
   `claim_ref`, when a row names an entry the manifest does not carry, or when a row
   renders wider than the public claim's canonical label. A row's effective label can
   never outrun the public claim it backs.

4. **The packet-freshness, waiver-expiry, and control stop rules narrow rows before
   promotion.** Each packet carries a freshness SLO and a recorded `slo_state`. The CI
   gate recomputes the freshness state and the waiver-expiry state against the audit
   `as_of` date, failing when a declared state overstates the clock, when an attested
   row rides a stale packet or an expired waiver, when a required control is
   unsatisfied, or when an owner sign-off is missing under a Stable claim.

5. **The four domains and the release-blocking set stay covered.** The gate fails if any
   of `open_paid_boundary`, `licensing`, `provenance`, or `contribution_policy` has no
   row, if a declared release-blocking subject has no covering row, if a release-blocking
   row is not declared, or if an `entry_id` or `subject_ref` repeats.

6. **The publication verdict is recomputed, not asserted.** The gate recomputes the
   `hold`/`proceed` decision and the blocking rule/entry sets from the firing stop rules
   and fails on any drift. With `--require-proceed` it exits non-zero on `hold`, so
   shiproom and release tooling fail promotion directly from this artifact.

## Current snapshot (as of 2026-05-24)

The checked-in audit holds promotion. Of eight audit rows across four public claims,
three attest and back Stable claims cleanly (the open-core boundary matrix, the core
SPDX/license inventory, and the build-provenance row — the last on an active waiver).
Five rows are narrowed below the cutline:

- the **managed-to-open offboarding** row inherits the ceiling from a public claim
  already published beta;
- the **licensing redistribution** row narrowed to beta because a copyleft-bundling
  control is unsatisfied;
- the **regulated-environment provenance** row inherits the ceiling from a public claim
  already published beta;
- the **contribution DCO/CLA** row narrowed to beta because its attestation packet
  breached its freshness SLO; and
- the **maintainer-governance** row narrowed to beta because the waiver it relied on
  expired.

Three of those — the unsatisfied control, the breached packet, and the expired waiver —
back claims still published Stable, so they fire three blocking stop rules and hold the
`release.shiproom.open_paid_boundary_audit` gate. The audit narrows the optimistic
Stable rows automatically instead of letting them ride; promotion clears once the
copyleft-bundling control is satisfied, the DCO/CLA packet is refreshed, and the
maintainer-governance waiver is renewed (or those public claims are formally narrowed).

## How to re-verify

```
python3 ci/check_open_paid_boundary_audit.py --repo-root . --check
cargo test -p aureline-release
```

The first command revalidates the audit, recomputes the freshness/waiver automations
and the control checks against `as_of`, runs the negative drills and fixture cases, and
writes the validation capture. The second runs the typed contract tests that bind the
model to the checked-in audit. Add `--require-proceed` to the gate to turn the recorded
`hold` into a non-zero exit for shiproom use.
