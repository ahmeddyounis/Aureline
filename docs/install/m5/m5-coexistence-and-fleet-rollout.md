# M5 coexistence and fleet rollout

The M5 coexistence-and-fleet-rollout packet is the single inspectable object that keeps stable,
preview, portable, mirror/offline, and managed installs from **colliding over user state or
launches**, and proves the **canary-to-LTS rollout rings** are evidence-backed before publication.
Release engineers, support engineers, and enterprise operators read one object instead of assuming
one connected happy path or one opaque managed default.

- Typed model: `aureline-install` crate, module `m5_coexistence_and_fleet_rollout`
  ([`M5CoexistenceFleetRollout`]).
- Canonical packet: `artifacts/install/m5/m5-coexistence-and-fleet-rollout.json`
- Schema: `schemas/install/m5-coexistence-and-fleet-rollout.schema.json`
- Reviewer artifact: `artifacts/install/m5/m5-coexistence-and-fleet-rollout.md`
- Fixtures: `fixtures/install/m5/m5-coexistence-and-fleet-rollout/`

## What the packet records

One `CoexistenceLaneRow` covers one M5 side-by-side install family — the families that can land on
one machine at the same time:

| Family | Covers |
| --- | --- |
| `stable_broad` | The stable-channel broad install, the first-party local baseline |
| `preview_side_by_side` | A preview install running beside stable |
| `portable` | A portable install carrying its own durable state root |
| `managed` | An organization-managed, policy-controlled fleet install |
| `mirror_offline` | A mirror or air-gap install provisioned from offline media |

Each lane records, for that install family:

- **state-root separation** — `isolated`, `bounded_namespaced`, `import_linked`, or `colliding`;
- **import choice** — the first-run decision: `fresh_state`, `copy_from_sibling`, `decline_import`,
  `mirror_import`, or `link_shared`;
- **update-marker ownership** — `exclusive_owned`, `scoped_shared`, or `contested_last_writer`;
- **handler precedence** — one record per `file_association`, `deep_link`, and `protocol_handler`
  surface: `sole_owner`, `precedence_declared`, `user_arbitrated`, or `last_writer_wins`;
- **install verification** — `signed_verified` / `platform_trusted` / `self_signed` / `unverified`.

## The coexistence gate

The support label an install family may publish (`published_support`) is the weakest ceiling
implied by its observed states. It is computed, never asserted by hand:

```
published_support = declared_support
    .min(install_verification ceiling)
    .min(state_root_separation ceiling)
    .min(update_marker_ownership ceiling)
    .min(worst handler_precedence ceiling)
    .min(governs_assurance)             // the governance lane's own published label
```

So an unverified binary, a shared or colliding state root, a contested update marker, a
last-writer-wins handler, or a governance lane that was itself narrowed all lower or withhold the
published support automatically. The recorded `narrow_reasons` and `recovery_path` are recomputed
the same way and validated to match.

This is the guardrail the lane exists to enforce: **a new M5 channel cannot rely on
last-writer-wins handler ownership or undocumented state-root sharing.** Each family is pinned to
the canonical governance lane it draws verification truth from (`governs_lane`), and
`governs_assurance` is validated against the embedded `m5_install_and_portability_governance`
matrix, so an install family can never publish support beyond the lane the governance gate already
narrowed.

## Ring evidence

Each `RolloutRingRow` publishes ring evidence for one of the `canary`, `pilot`, `broad`, and `lts`
rings: its rollout `posture` (`promoted` / `soaking` / `held` / `rolled_back`), the `rollback_state`
it can fall back to, and how fresh its `evidence_freshness` is. A ring's published support is the
weakest ceiling those imply, so a held, soaking, or rolled-back ring, an expired or missing rollback
target, or stale evidence narrows the ring before publication.

## Mirror and air-gap import review

Each `MirrorImportRow` reviews one mirror or air-gap import and records its detached-signature
verification (`detached_signature_verified` / `checksum_only` / `unverified`), package freshness, and
review state, so an offline or managed-fleet profile cannot import an unverified or stale package as
if it were trusted. At least one import must prove the detached-signature path.

## Troubleshooting drills

Each `RolloutDrill` replays one rollout or coexistence incident and proves the object detects it, so
a failure is visible before publication rather than after a wrong-target launch:

| Incident | What it replays |
| --- | --- |
| `wrong_target_launch` | A launch routed to the wrong install target |
| `handler_takeover` | A sibling install took over a handler last-writer-wins |
| `stale_mirror` | A mirror or air-gap import served stale state |
| `managed_package_drift` | A managed package drifted from its policy-pinned build |

## Consumer surfaces

Release center, Help/About, support export, diagnostics, CLI, and admin docs each bind to this one
packet via a `RolloutConsumerBinding`, ingest it, preserve its published support and recovery paths
verbatim, and narrow with it. An install family narrowed here cannot read as supported on a release
evidence row, an About panel, a CLI status line, an admin doc badge, or a support export. The export
projection and support-export wrapper carry typed states and opaque refs only — no credential bodies,
raw provider payloads, or workspace contents.

[`M5CoexistenceFleetRollout`]: ../../../crates/aureline-install/src/m5_coexistence_and_fleet_rollout/mod.rs
