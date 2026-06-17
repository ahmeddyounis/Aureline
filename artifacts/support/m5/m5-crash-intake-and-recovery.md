# Crash-loop recovery and crash intake — reviewer artifact

Human-readable companion to the governed packet at
`artifacts/support/m5/m5-crash-intake-and-recovery.json`. The full contract and gate semantics live in
`docs/help/support/m5-crash-intake-and-recovery.md`; the typed model lives in the `aureline-support`
crate (`m5_crash_intake_and_recovery`).

This registry turns crash loops and issue-report entry into explicit, exact-build-aware product flows.
Every crash-loop scenario gets a **recovery screen** that offers distinct, named recovery actions
(Restore, Open without restore, Safe mode, Disable recently changed extension, Disable recently changed
profile, Open logs, Report issue — never a generic "try again"), each labeled with whether it reruns or
discards state and its bounded blast radius, bound to an exact-build-aware crash intake: the visible
crash-envelope id, the copyable exact-build id and its identity fidelity, the symbolication fidelity, the
restore-provenance class, the install / advisory state, the redaction posture, and the local-save /
team-share / formal-support intake modes. A fail-closed gate narrows or blocks any screen whose intake
would overclaim an exact build or resolved symbolication, ride on a downgraded restore or an active
advisory, or carry content that cannot safely leave the machine — and forbids any screen from making
local-save look secondary to a send mode.

## Screen roll-up (as of 2026-06-17)

| Screen | Build / symbols | Restore / advisory | Status | Presentation | Actions |
| --- | --- | --- | --- | --- | --- |
| `exact-build-local-save` | exact_build / resolved | exact_restore / clean | exact_ready | **exact_ready** | 5 |
| `repeated-crash-loop` | exact_build / resolved | compatible_restore / advisory_active | advisory_narrowed | **narrowed** | 7 |
| `stale-symbol-map` | approximate_build / stale_symbol_map | exact_restore / clean | fidelity_narrowed | **narrowed** | 5 |
| `quarantined-extension` | exact_build / resolved | exact_restore / extension_quarantine_active | advisory_narrowed | **narrowed** | 6 |
| `restore-downgrade` | exact_build / resolved | restore_downgraded / clean | fidelity_narrowed | **narrowed** | 5 |
| `send-blocked-unsafe-intake` | unresolved_build / unresolved | no_restore_attempted / clean | send_blocked | **send_blocked** | 5 |

One screen presents as fully exact-ready (proving the gate is not a blanket flag), four narrow on a
fidelity downgrade or an active advisory, and one blocks an unsafe intake. The five core actions are
present on every screen; both disable-action classes are exercised; and the local-save path is
first-class on all six.

## The cases this corpus proves

### Exact-build local save — `exact-build-local-save`

Exact build, resolved symbols, an exact restore, and a clean install: a full-fidelity recovery screen.
The five core actions are each distinct and labeled (only Restore reruns the session), and local save is
the primary intake path.

### Repeated crash loop with suspects — `repeated-crash-loop`

A reopen crash loop after a recent extension and profile change. Beside the five core actions, the screen
offers a reversible **Disable recently changed extension** and a reversible **Disable recently changed
profile**, each targeting a named suspect and preserving user-owned state. An active advisory narrows the
screen; the team-share intake stays send-safe with a redacted summary, and local save stays co-equal.

### Stale symbol map — `stale-symbol-map`

The symbol map does not match this build and the build identity is only approximate. The screen labels
the build identity and symbolication as approximate and stale, never claims exact / resolved, and narrows
with `approximate_build_identity` and `stale_or_partial_symbolication`.

### Quarantined extension — `quarantined-extension`

A recently changed extension is quarantined as the suspected cause. The screen narrows with
`extension_quarantine_active` and offers a bounded, reversible **Disable recently changed extension**
targeting the quarantined extension, with local save primary.

### Restore downgrade — `restore-downgrade`

Restore replay failed and the recorded session was downgraded to a weaker restore. The screen labels the
restore provenance as downgraded, never implies an exact restore, and narrows with
`restore_provenance_downgraded`.

### Unsafe intake blocks the send — `send-blocked-unsafe-intake`

The build identity and symbols are unresolved, and the staged formal-support handoff would carry
retained-local-only content. The gate blocks the send (`intake_send_blocked_unsafe_content`), warns
before any packet leaves, and keeps local save as the primary path. The unresolved build and symbols are
labeled as such, never implied to be exact / resolved.

## Sign-off gate

Promotion of the crash-intake-and-recovery registry holds unless all of the following are true on the
current packet (`M5CrashIntakeAndRecovery::validate()` returns no violations):

1. Every screen offers the five distinct core recovery actions, plus a targeted disable action for each
   surfaced suspect change; no action collapses into a generic affordance, discards user-owned state, or
   is a destructive factory reset.
2. Every screen shows a visible, copyable exact-build id and crash-envelope id, and carries its one-step
   "Why this crash, on which build?" explain entry plus the CLI / headless equivalent object.
3. Every screen's `intake_status`, `presentation`, `downgrade_reasons`, exact-build and
   resolved-symbolication claims, `local_save_first_class` attestation, and `blocked_before_send` flag
   equal the recomputed fail-closed gate — so a screen can never imply exact build / resolved
   symbolication for approximate / unresolved data.
4. The local-save intake mode is offered, enabled, and at least as prominent as every send mode on every
   screen.
5. No raw secret bodies, raw dumps, or raw payloads are carried (`raw_material_excluded`).
6. The five consumer bindings (crash-recovery screen, Support Center, CLI / headless, issue-report
   packet, support export) are all present and reuse this packet's vocabulary, object ids, and
   exact-build lineage, each keeping local-save first-class.

## Regenerating this packet

This packet is checked in alongside the registry it reviews. When the registry changes, update the
packet, schema, reviewer artifact, and fixtures together, then re-run the gate:

```sh
cargo test -p aureline-support m5_crash_intake_and_recovery
cargo run -p aureline-support --example dump_m5_crash_intake_and_recovery
```
