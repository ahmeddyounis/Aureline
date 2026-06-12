# M5 coexistence-and-fleet-rollout fixtures

Fixture corpus for the `m5_coexistence_and_fleet_rollout` packet. These fixtures pin the seeded
coexistence lanes, rollout rings, mirror imports, and troubleshooting drills so a change to the
typed model, the gate, or the checked-in packet is caught against frozen evidence.

- Canonical packet: `artifacts/install/m5/m5-coexistence-and-fleet-rollout.json`
- Schema: `schemas/install/m5-coexistence-and-fleet-rollout.schema.json`
- Typed model: `crates/aureline-install/src/m5_coexistence_and_fleet_rollout/`
- Governance matrix the lanes are pinned to:
  `artifacts/install/m5/m5-install-and-portability-governance.json`

## Files

- `corpus_manifest.json` — indexes the five coexistence lanes, four rollout rings, two mirror
  imports, and four drills, recording what each proves (install mode, channel, state-root
  separation, marker ownership, handler precedence, governance lane, and published label).
- `drill_wrong_target_launch.json` — a deep link launches the preview install instead of stable;
  the wrong-target launch is detected against the declared precedence.
- `drill_handler_takeover.json` — a sibling install grabs a file association last-writer-wins; the
  takeover is detected against the managed-policy declared precedence.
- `drill_stale_mirror.json` — an air-gap import serves a stale package; the staleness is detected
  and its support is held at retest-pending.
- `drill_managed_package_drift.json` — a managed install drifts from its policy-pinned build; the
  drift is detected and its support stays withheld.

## What the corpus proves

- **Coexistence stays explicit.** Five install families that can land on one machine — stable broad,
  preview side-by-side, portable, managed, and mirror/offline — each disclose state-root separation,
  import choice, update-marker ownership, and per-surface handler precedence rather than assuming one
  connected happy path.
- **The gate narrows in every direction.** `stable_broad` admits verified, `preview_side_by_side`
  narrows to bounded, `portable` to retest-pending, and `managed` and `mirror_offline` are withheld.
  Every published label equals the recomputed gate ceiling and never exceeds the governance lane the
  family is pinned to.
- **No last-writer-wins ownership.** A `last_writer_wins` handler withholds a lane; the seeded lanes
  use `sole_owner`, `precedence_declared`, or `user_arbitrated`.
- **Ring and mirror evidence exists.** Canary, pilot, broad, and LTS rings publish posture, rollback,
  and evidence freshness; the detached-signature path is proven by the managed-mirror import while
  the checksum-only, stale air-gap import is held at retest-pending.
- **Incidents are reproducible.** The four drills replay wrong-target-launch, handler-takeover,
  stale-mirror, and managed-package-drift incidents, and each is detected.

The fixtures carry typed states and opaque refs only — no credential bodies, raw provider payloads,
or workspace contents.
