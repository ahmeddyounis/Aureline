# M5 install-and-update diagnostics — reviewer artifact

Human-readable companion to the diagnostics packet at
`artifacts/install/m5/m5-install-diagnostics.json`. The full contract and gate semantics live in
`docs/install/m5/m5-install-diagnostics.md`; the typed model lives in the `aureline-install` crate
(`m5_install_diagnostics`).

This artifact freezes one inspectable diagnostics row per M5-added artifact family and publishes,
for each, **only the support label its evidence actually backs**. An unverified binary, a stale
verification, a missing rollback target, or a governance lane that was itself narrowed
automatically lowers or withholds the published support before it reaches desktop, CLI, About, or a
support export.

## Diagnostics roll-up (as of 2026-06-11)

| Artifact | Mode | Channel | Updater | Verification | Rollback | Governs lane | Declared | Published | Recovery |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `desktop_app` | system | stable_broad | first_party_auto | signed_verified / current | available | desktop_stable (verified) | verified | **verified** | none |
| `companion` | user | stable_broad | marketplace_host | platform_trusted / current | available_bounded | marketplace_companion (retest_pending) | bounded | **retest_pending** | follow_governance_recovery |
| `marketplace_helper` | marketplace | stable_broad | marketplace_host | self_signed / current | expired | marketplace_companion (retest_pending) | retest_pending | **retest_pending** | follow_governance_recovery |
| `local_model_runtime` | system | stable_broad | first_party_auto | signed_verified / stale | available | desktop_stable (verified) | verified | **retest_pending** | refresh_verification |
| `portable_export` | portable | stable_pinned | manual_user | signed_verified / current | missing | portable_install (retest_pending) | bounded | **withheld** | withhold_claim |

One artifact admits at full trust (`desktop_app`), proving the gate is not a blanket downgrade;
three narrow to retest-pending and one is withheld. Every published label equals the gate's
recomputed ceiling and never exceeds the governance lane the artifact is pinned to.

## What the diagnostics object proves

- **Topology stays inspectable per artifact.** Install mode, channel, updater owner, artifact /
  mutable-state / policy roots, verification state and freshness, and rollback target are recorded
  for every M5 artifact — not reduced to a single version string or channel chip.
- **Support never widens past the evidence.** The platform-trusted `companion` and self-signed
  `marketplace_helper` are held at retest-pending; the signed-but-stale `local_model_runtime` is
  narrowed only because its verification went stale; the rollback-less `portable_export` is
  withheld. Each is clamped to its governance lane.
- **Secrets are classified, not dumped.** The `companion` session credential store
  (`secret_bearing`) and the `marketplace_helper` policy manifest root (`machine_protected`) are
  named and redacted — no token values or machine-unique material appear.
- **Incidents are reproducible.** Drills replay root-mismatch, stale-verification,
  missing-rollback-target, and wrong-root-support incidents and each is detected.
- **One object, every surface.** Desktop, CLI, About, and support export each bind to this packet,
  ingest it, and narrow with it, so a narrowed artifact cannot read as supported downstream.

## Consumer surfaces

Desktop, CLI, About, and support export each bind to this one packet, preserve its labels and
recovery paths, and narrow with it. The export projection and support-export wrapper carry typed
states and opaque refs only — no credential bodies, raw provider payloads, or workspace contents.
