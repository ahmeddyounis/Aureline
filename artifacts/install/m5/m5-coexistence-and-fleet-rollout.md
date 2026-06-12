# M5 coexistence and fleet rollout — reviewer artifact

Human-readable companion to the packet at
`artifacts/install/m5/m5-coexistence-and-fleet-rollout.json`. The full contract and gate semantics
live in `docs/install/m5/m5-coexistence-and-fleet-rollout.md`; the typed model lives in the
`aureline-install` crate (`m5_coexistence_and_fleet_rollout`).

This artifact freezes one coexistence lane per M5 side-by-side install family and publishes, for
each, **only the support label its coexistence evidence actually backs**. An unverified binary, a
shared or colliding state root, a contested update marker, a last-writer-wins handler, or a
governance lane that was itself narrowed automatically lowers or withholds the published support
before it reaches release center, About, CLI, diagnostics, support export, or admin docs.

## Coexistence roll-up (as of 2026-06-12)

| Family | Mode | Channel | State root | Marker | Worst handler | Governs lane | Declared | Published | Recovery |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `stable_broad` | system | stable_broad | isolated | exclusive_owned | sole_owner | desktop_stable (verified) | verified | **verified** | none |
| `preview_side_by_side` | user | preview_early_access | isolated | exclusive_owned | precedence_declared | desktop_preview (bounded) | verified | **bounded** | follow_governance_recovery |
| `portable` | portable | stable_pinned | isolated | scoped_shared | user_arbitrated | portable_install (retest_pending) | verified | **retest_pending** | follow_governance_recovery |
| `managed` | managed | managed_pinned | bounded_namespaced | exclusive_owned | precedence_declared | managed_fleet (withheld) | verified | **withheld** | withhold_claim |
| `mirror_offline` | managed | managed_pinned | isolated | exclusive_owned | sole_owner | managed_fleet (withheld) | verified | **withheld** | withhold_claim |

One family admits at full trust (`stable_broad`), proving the gate is not a blanket downgrade; one
narrows to bounded, one to retest-pending, and two are withheld. Every published label equals the
gate's recomputed ceiling and never exceeds the governance lane the family is pinned to. The
`mirror_offline` lane proves the gate withholds on governance alone — every coexistence input is
whole, but its managed-fleet governance lane is withheld.

## Ring evidence (canary → LTS)

| Ring | Rolls out | Posture | Rollback | Evidence | Published |
| --- | --- | --- | --- | --- | --- |
| `canary` | stable_broad | soaking | available | current | **bounded** |
| `pilot` | stable_broad | promoted | available | current | **verified** |
| `broad` | stable_broad | held | available_bounded | aging | **retest_pending** |
| `lts` | stable_broad | promoted | available | current | **verified** |

## Mirror / air-gap import review

| Import | Source | Signature | Freshness | Review | Published |
| --- | --- | --- | --- | --- | --- |
| `managed-mirror` | managed_mirror | detached_signature_verified | current | reviewed | **verified** |
| `air-gap-media` | air_gap_media | checksum_only | stale | pending_review | **retest_pending** |

## What the coexistence object proves

- **Installs do not collide over state.** State-root separation, import choice, and update-marker
  ownership are recorded for every family. The `managed` lane's namespaced root and the `portable`
  lane's scoped marker are disclosed and narrow the published label rather than reading as isolated.
- **No last-writer-wins handler ownership.** Handler precedence is recorded per file-association,
  deep-link, and protocol-handler surface. A `last_writer_wins` handler withholds the lane; the
  seeded lanes use `sole_owner`, `precedence_declared`, or `user_arbitrated` and narrow accordingly.
- **Ring evidence exists for canary, pilot, broad, and LTS.** Each ring publishes posture, rollback,
  and evidence freshness, and only the support its evidence backs.
- **Mirror and air-gap imports are verified.** The detached-signature path is proven by the
  `managed-mirror` import; the checksum-only, stale, pending `air-gap-media` import is held at
  retest-pending.
- **Incidents are reproducible.** Drills replay wrong-target-launch, handler-takeover, stale-mirror,
  and managed-package-drift incidents and each is detected.
- **One object, every surface.** Release center, About, support export, diagnostics, CLI, and admin
  docs each bind to this packet, ingest it, and narrow with it, so a narrowed family cannot read as
  supported downstream.

## Consumer surfaces

Release center, Help/About, support export, diagnostics, CLI, and admin docs each bind to this one
packet, preserve its labels and recovery paths, and narrow with it. The export projection and
support-export wrapper carry typed states and opaque refs only — no credential bodies, raw provider
payloads, or workspace contents.
