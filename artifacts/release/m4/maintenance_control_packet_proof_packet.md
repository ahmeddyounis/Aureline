# Maintenance-control packet — proof packet

Reviewer-facing proof packet for the gated maintenance-control packet for the release
line's hotfix, backport, correction-train, and support-window lanes.

Canonical machine source (do not clone status text from this packet — ingest the
JSON):

- Packet: [`/artifacts/release/maintenance_control_packet.json`](../maintenance_control_packet.json)
- Schema: [`/schemas/release/maintenance_control_packet.schema.json`](../../../schemas/release/maintenance_control_packet.schema.json)
- Companion doc: [`/docs/release/maintenance_control_packet.md`](../../../docs/release/maintenance_control_packet.md)
- Validator: `ci/check_maintenance_control_packet.py`
- Validation capture:
  [`/artifacts/release/captures/maintenance_control_packet_validation_capture.json`](../captures/maintenance_control_packet_validation_capture.json)
- Typed consumer: `aureline_release::maintenance_control_packet`

## What this packet proves

1. **Each maintenance lane binds a support window and a control packet to a public
   claim.** Every row binds one lane (`lane_kind`, `lane_ref`) to the support window it
   commits to (`support_window`), the control packet that proves the lane is staffed
   (`control_packet`), the shared correction-train packet form it rides
   (`correction_packet_ref`), the waiver that holds it provisionally (`waiver`), and the
   public claim whose lifecycle label it backs (`claim_ref`, `claim_label`). The packet
   reuses the stable claim level vocabulary rather than minting per-lane labels, so
   docs, Help/About, the release center, and support exports render one label per lane.

2. **The packet ingests the stable claim manifest as a hard ceiling.** The CI gate
   reads the stable claim manifest named by `claim_manifest_ref` and fails when a row's
   `claim_label` is not the label that manifest publishes for the entry named by
   `claim_ref`, when a row names an entry the manifest does not carry, or when a control
   is backed wider than the public claim's canonical label. A lane's controlled label
   can never outrun the public claim it backs.

3. **The packet-freshness, waiver-expiry, and support-window-expiry automations narrow
   ungoverned lanes before publication.** Each row's control packet carries a freshness
   SLO and a recorded `slo_state`; each support window carries an `end_of_support_date`
   and a `support_posture`. The CI gate recomputes the freshness state, the waiver-
   expiry state, and the support-window-expiry state against the packet `as_of` date and
   fails when a declared state overstates the clock, when a governed lane rides a stale
   packet or an expired support window, or when a lane that lost its waiver still claims
   governance.

4. **The four maintenance kinds and the release-blocking lane set stay covered.** The
   gate fails if any of `hotfix`, `backport`, `correction_train`, or `support_window`
   has no row, if a declared release-blocking lane has no covering row, if a
   release-blocking row is not declared, or if a `lane_ref` repeats.

5. **The publication verdict is recomputed, not asserted.** The gate recomputes the
   `hold`/`proceed` decision and the blocking rule/lane sets from the firing control
   rules and fails on any drift. With `--require-proceed` it exits non-zero on `hold`,
   so shiproom and release tooling block maintenance publication directly from this
   artifact.

## Current snapshot (as of 2026-05-21)

The checked-in packet holds publication. Of nine lanes across five public claims, two
governed lanes back Stable claims cleanly (the provider hotfix lane and the
repair/rollback correction-train lane, the latter on an active waiver). Seven lanes are
narrowed below the cutline:

- the **repair/rollback support window** narrowed to beta because its control packet
  breached its freshness SLO;
- the **provider support window** narrowed to beta because its end-of-support date
  passed without renewal;
- the **repair/rollback hotfix lane** narrowed to beta because its provisional waiver
  expired;
- the **export** and **localization** backport lanes and the **regulated** support
  window inherit ceilings from public claims already narrowed upstream (beta, preview,
  beta); and
- the advisory (non-release-blocking) **regulated correction-train** lane is unbacked
  for an incomplete support window and a missing owner sign-off under a beta claim.

Three of those — the breached support-window packet, the expired provider window, and
the expired hotfix-lane waiver — back claims still published Stable, so they fire three
blocking control rules and hold the `maintenance_control_packet_publication` gate. The
packet narrows the optimistic Stable maintenance promises automatically instead of
letting them ride; publication clears once the support-window packet is refreshed, the
provider window is renewed, and the hotfix-lane rehearsal lands (or those public claims
are formally narrowed).

## How to re-verify

```
python3 ci/check_maintenance_control_packet.py --repo-root . --check
cargo test -p aureline-release
```

The first command revalidates the packet, recomputes the freshness/waiver/support-
window automations against `as_of`, runs the negative drills and fixture cases, and
writes the validation capture. The second runs the typed contract tests that bind the
model to the checked-in packet, the frozen capture, and the negative fixtures. Add
`--require-proceed` to the gate to turn the recorded `hold` into a non-zero exit for
shiproom use.
