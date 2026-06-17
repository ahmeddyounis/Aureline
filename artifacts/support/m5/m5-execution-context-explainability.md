# Execution-context explainability — reviewer artifact

Human-readable companion to the governed packet at
`artifacts/support/m5/m5-execution-context-explainability.json`. The full contract and gate semantics
live in `docs/help/support/m5-why-this-execution-context.md`; the typed model lives in the
`aureline-runtime` crate (`m5_environment_status_strips`).

This registry gives every run-capable M5 surface a **persistent environment/target status strip** and a
**one-step "Why this execution context?"** entry. Each strip projects the execution-context resolver truth
by reference — the active interpreter / SDK / shell / container / remote target — and a fail-closed status
gate flags or blocks any strip whose context is stale, blocked, drifted, or conflicting rather than
letting it present a clean current-target chip. The blocked or stale state becomes visible at the locus of
work, before the downstream run failure.

## Surface roll-up (as of 2026-06-16)

| Surface | Status | Presentation | Resolution | Shown facets |
| --- | --- | --- | --- | --- |
| `run` | resolved | **resolved** | none | interpreter, shell |
| `test` | resolved | **resolved** | none | interpreter, sdk |
| `debug` | stale | **flagged** | refresh_target | interpreter, shell |
| `notebook` | stale | **flagged** | refresh_target | interpreter |
| `request` | remote_drift | **flagged** | reconnect_remote | remote_target |
| `database` | blocked | **blocked** | unblock_environment | remote_target, sdk |
| `preview` | conflicting | **flagged** | resolve_conflict | container, remote_target |
| `pipeline` | resolved | **resolved** | none | container, shell |
| `incident` | stale | **flagged** | refresh_target | interpreter, remote_target |

Three strips resolve cleanly (`run`, `test`, `pipeline`), proving the gate is not a blanket flag; five
flag on a stale, drifted, or conflicting context; and one (`database`) blocks and warns before the
downstream statement runs.

## Per-surface notes

### run

Resolves cleanly: interpreter and shell are current. The explain entry and CLI object both project the
same resolved context.

### test

Resolves cleanly on a current interpreter and SDK; the explain entry names which toolchain runs the suite.

### debug

Flags a stale interpreter and offers a refresh; the explain entry shows the recorded versus current
interpreter before the session attaches.

### notebook

Cannot confirm the kernel's freshness (`unknown`), so it flags rather than claiming a current kernel;
refresh re-resolves the kernel against the selected interpreter.

### request

Flags a drifted remote target and offers a reconnect; the explain entry shows the recorded versus active
endpoint so the request never silently hits a different target.

### database

Blocked and warns before any statement runs; the explain entry names the blocked connection so the blocked
state is visible before the downstream failure.

### preview

Surfaces a conflict between the devcontainer and the remote target rather than picking one silently; the
explain entry shows both contexts so the user can reconcile them.

### pipeline

Resolves cleanly on a current devcontainer and shell; the explain entry names the container each pipeline
stage runs in.

### incident

Flags the captured context as stale relative to the live environment, so a historical context never reads
as the current one; refresh re-resolves against today's environment.

## Sign-off gate

Promotion of the status-strip registry holds unless all of the following are true on the current packet
(`M5EnvironmentStatusStrips::validate()` returns no violations):

1. Every run-capable surface carries exactly one strip; none is missing or duplicated.
2. Every strip shows at least one execution-context facet and carries its one-step explain entry and its
   CLI / headless equivalent object — even when blocked — so it can never collapse into a generic chip.
3. Every strip's `presentation`, `downgrade_reasons`, `resolution_path`, and `blocked_before_run` flag
   equal the recomputed fail-closed gate — a stale, blocked, drifted, or conflicting context flags or
   blocks the strip automatically.
4. No flagged or blocked strip is silent: it names its resolution path, a caveat, and the stale-or-blocked
   field; a blocked strip warns before the downstream run failure.
5. The five consumer bindings (desktop-shell, support-center, support-export, issue-report-packet,
   cli-headless) are all present and reuse this packet's status vocabulary and object ids.

## Regenerating this packet

This packet is checked in alongside the registry it reviews. When the status-strip registry changes,
update the packet, schema, reviewer artifact, and fixtures together, then re-run the gate:

```sh
cargo test -p aureline-runtime m5_environment_status_strips
cargo run -p aureline-runtime --example dump_m5_environment_status_strips
```
