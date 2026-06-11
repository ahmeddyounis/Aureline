# First-useful-work entry routes for M5 depth lanes

The stable v1 shell established a safe first-run contract: every surface can be
entered, skipped, revisited, and repaired from the main shell **without hidden
setup work or account-first dead ends**. This page describes how that contract
is carried forward into the new M5 depth lanes so retention features feel
native instead of like sidecar tools with separate onboarding rituals.

The canonical truth for this lane is the seeded packet produced by
[`aureline_shell::m5_entry_routes`](../../crates/aureline-shell/src/m5_entry_routes/mod.rs).
Later dashboards, docs/help surfaces, release-center views, and support exports
should ingest that packet instead of cloning status text.

## Covered lanes

Every major M5 depth lane has an in-product entry route with setup-later
posture, an explicit local-core fallback, and a current first-useful-work
measurement:

| Lane | First useful work (before any optional setup) | What is *not* yet done |
|---|---|---|
| Notebook | Open the notebook and inspect cells/outputs | No kernel started |
| Request workspace | Inspect request definitions and saved responses | No request sent |
| Database workspace | Inspect connection definitions and cached schema | No database connected |
| Profiler / trace capture | Inspect a previously captured profile or trace | No trace captured |
| Framework pack | Browse the local pack catalog | No framework pack installed |
| Docs / local browser | Read bundled and cached docs | No browser auth completed |
| Preview routes | Inspect preview route definitions | No preview route exposed |
| Companion handoff | Review the handoff packet locally | No companion joined |
| Managed sync | Inspect local sync state | No sync joined |
| Offboarding | Review the plan and export preview | No offboarding action committed |

## Invariants

The packet is validated against the safe first-run invariants
(`validate_m5_entry_routes_packet`). For every lane that claims local-first
continuity:

- a **local-core fallback** is exposed (`local_core_fallback`);
- the **`set_up_later`** action is always present;
- **no hidden prerequisite** gates basic open, inspect, or learnability flows —
  browser auth, provider attachment, kernel execution, and managed sync are all
  `false`;
- optional managed or provider-backed enrichments are **suggestions, never
  blockers** (`mandatory` is always `false`);
- at least one **deferred-action statement** explains what Aureline has not yet
  done, so setup stays reviewable; and
- **first useful work is reachable before any optional setup**
  (`reached_before_optional_setup`), and its task-success instrumentation
  records completion, fallback, abandonment, and repair-required so partner
  studies can measure real switching friction.

The records carry no credential bodies, raw provider payloads, file paths, or
project content (`no_raw_sensitive_user_content`).

## Inspecting and regenerating

```sh
# Inspect the packet, routes, coverage, or run the validators.
cargo run -q -p aureline-shell --bin aureline_shell_m5_entry_routes -- packet
cargo run -q -p aureline-shell --bin aureline_shell_m5_entry_routes -- coverage
cargo run -q -p aureline-shell --bin aureline_shell_m5_entry_routes -- validate

# Regenerate the published markdown artifact.
cargo run -q -p aureline-shell --bin aureline_shell_m5_entry_routes -- markdown > \
  artifacts/ux/m5/first-useful-work-packets/m5_entry_routes_packet.md
```

The checked-in fixtures under `fixtures/ux/m5/entry-and-resume/` are bit-for-bit
equal to the seeded packet output.
