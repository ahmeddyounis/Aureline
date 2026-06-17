# Crash-loop recovery screens and crash intake

When startup, reopen, or a supervised host crashes repeatedly, Aureline stops silently retrying and
shows a **crash-loop recovery screen**: a product surface with distinct, named recovery actions and an
exact-build-aware crash intake. The contract and fail-closed gate are owned by the `aureline-support`
crate (`m5_crash_intake_and_recovery`); the canonical packet is checked in at
`artifacts/support/m5/m5-crash-intake-and-recovery.json` and validated against
`schemas/support/m5-crash-intake.schema.json`.

## What every screen shows

- **Distinct, named recovery actions — never a generic "try again".** Every screen offers the five core
  actions — **Restore**, **Open without restore**, **Safe mode**, **Open logs**, and **Report issue** —
  plus a targeted **Disable recently changed extension** or **Disable recently changed profile** action
  for each suspected change. Each action declares whether it **reruns** the session (only Restore does)
  and that it never **discards** user-owned state, plus its bounded blast radius, so you can pick a safe
  path without guessing which action reruns or discards work.
- **A visible, copyable exact-build id and crash-envelope id.** The crash-envelope id and the build id
  are always shown and copyable, so you can quote them to support.
- **Honest fidelity labels.** The build-identity fidelity (`exact_build`, `approximate_build`,
  `unresolved_build`) and symbolication fidelity (`resolved`, `partially_resolved`, `stale_symbol_map`,
  `unresolved`) are shown plainly. A screen never implies an exact build or resolved symbolication when
  only approximate or unresolved data exists.
- **Restore provenance and install / advisory state.** The screen names the restore-provenance class
  (including `restore_downgraded`) and any active advisory or extension quarantine.
- **Typed intake modes, with local-save first-class.** The same surface offers **local save**, **team
  share**, and **formal support handoff**, each with its redaction posture. Local save is always offered
  and never less prominent than a send mode.

## The recovery / intake gate

A fail-closed gate decides how each screen may be presented. The published presentation is the weakest
of three ceilings:

| Input | Ceiling |
| --- | --- |
| Exact build + resolved symbols | `exact_ready` |
| Approximate / unresolved build, or stale / partial / unresolved symbols | `narrowed` |
| Downgraded restore, or active advisory / extension quarantine | `narrowed` |
| Selected send mode would carry content that cannot leave the machine | `send_blocked` |

So an approximate build, a stale symbol map, a downgraded restore, an active advisory, or unsafe content
can never read as a clean "exact, ready to send" screen. Two invariants hold regardless of presentation:

1. **Recovery actions stay distinct, bounded, and non-destructive.** No action collapses into a generic
   affordance, no action discards user-owned state, and no factory-reset / delete-state action is ever
   offered.
2. **Local-save is never out-shouted by a send mode.** A screen whose local-save mode is less prominent
   than a team-share or formal-support send fails the gate.

## The scenarios this corpus proves

| Screen | Build / symbols | Restore / advisory | Status | Presentation |
| --- | --- | --- | --- | --- |
| `exact-build-local-save` | exact / resolved | exact / clean | exact_ready | **exact_ready** |
| `repeated-crash-loop` | exact / resolved | compatible / advisory | advisory_narrowed | **narrowed** |
| `stale-symbol-map` | approximate / stale | exact / clean | fidelity_narrowed | **narrowed** |
| `quarantined-extension` | exact / resolved | exact / quarantine | advisory_narrowed | **narrowed** |
| `restore-downgrade` | exact / resolved | downgraded / clean | fidelity_narrowed | **narrowed** |
| `send-blocked-unsafe-intake` | unresolved / unresolved | none / clean | send_blocked | **send_blocked** |

- **Exact-build local save** is a clean, full-fidelity screen with local save as the primary path.
- **A repeated crash loop** after a recent extension and profile change offers distinct, reversible
  disable choices beside the core actions; an active advisory narrows the screen.
- **A stale symbol map** with an approximate build never implies exact identity or resolved frames.
- **A quarantined extension** is offered as a bounded, reversible disable next to the core actions.
- **A downgraded restore** is labeled as downgraded, never implied to be exact.
- **An unsafe intake** (unresolved identity staged for formal support with retained-local-only content)
  blocks the send before anything leaves and keeps local save as the primary path.

## One vocabulary across surfaces

The active crash-recovery screen, the Support Center, the CLI / headless recovery path
(`aureline support crash recover`), the issue-report / crash-intake packet, and the support export all
bind to this one registry. Each preserves the same recovery / intake vocabulary, object ids, and
exact-build lineage, keeps local-save first-class, and narrows with the gate.

## Regenerating this packet

This packet is checked in alongside the registry it documents. When the registry changes, update the
packet, schema, reviewer artifact, and fixtures together, then re-run the gate:

```sh
cargo test -p aureline-support m5_crash_intake_and_recovery
cargo run -p aureline-support --example dump_m5_crash_intake_and_recovery
```
