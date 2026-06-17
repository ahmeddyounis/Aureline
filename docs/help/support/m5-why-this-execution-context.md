# Why this execution context?

Every run-capable surface in Aureline carries a persistent **environment/target status strip** that shows
where the next run happens — the active interpreter, SDK, shell, container, and remote target — and a
**one-step "Why this execution context?"** entrypoint that opens the inspectable answer. The status strip
registry is the one authoritative contract for that supportability UX: it makes "where does this run, and
why?" answerable from the surface you are actually using, not only from a hidden diagnostics page.

- Typed model + gate: `aureline-runtime` crate, `m5_environment_status_strips`
- Packet: `artifacts/support/m5/m5-execution-context-explainability.json`
- Reviewer artifact: `artifacts/support/m5/m5-execution-context-explainability.md`
- Schema: `schemas/runtime/m5-environment-status-strip.schema.json`
- Fixtures: `fixtures/runtime/m5/m5-environment-status-strips/`
- Shiproom review packet:
  `artifacts/shiproom/m5-execution-context-explainability-review-packet/execution_context_explainability_review_packet.md`

## Why this packet exists

M5 adds many more run-capable surfaces — run, test, debug, notebook, request, database, preview,
pipeline, and incident. Each one resolves an execution context, but until now that truth lived behind its
own route. A user about to run something could not always see which interpreter, SDK, shell, container, or
remote target would be used, and a stale or blocked environment only became visible *after* the run
failed.

This packet projects the execution-context resolver truth — the
[`ExecutionContext`](../../runtime/execution_context_seed.md) object, its resolver, and its per-field
explanations — to the locus of work on every run-capable surface. It does not invent a new execution
context: it references the resolved context and renders a status strip plus a one-step explainability
entry beside it.

## The status strip

Each run-capable surface carries exactly one strip. A strip shows:

- a subset of the **context facets** — `interpreter`, `sdk`, `shell`, `container`, `remote_target` — each
  with a human-readable value label and a **freshness** state (`fresh`, `stale`, `unknown`);
- the overall **status** of the context (`resolved`, `stale`, `blocked`, `remote_drift`, `conflicting`);
- a one-step `explain_entrypoint` that opens the inspectable "Why this execution context?" answer; and
- the equivalent **CLI / headless object** id, so the same answer is reachable without the desktop UI.

A strip always carries the explain entry and the CLI-equivalent object — even when the environment is
blocked — so a blocked user can still ask where the run would happen and why.

## The fail-closed status gate

A strip must never present a generic "current target" chip that hides a differing or blocked execution
context. Its published **presentation** is therefore the weaker of two ceilings:

- **Status ceiling** — a `resolved` context can present cleanly; a `stale`, `remote_drift`, or
  `conflicting` context **flags** the strip; a `blocked` environment caps it at **blocked**.
- **Facet-freshness ceiling** — a `fresh` facet can present cleanly; a `stale` or `unknown` facet
  **flags** the strip.

The three published decisions are `resolved`, `flagged`, and `blocked`. When the gate flags or blocks a
strip it records the headline reasons (`stale_context`, `blocked_environment`, `remote_drift`,
`conflicting_context`) and the resolution path (`refresh_target`, `reconnect_remote`, `resolve_conflict`,
`unblock_environment`, or `none`). A blocked environment is the hardest state, so it points at an unblock
before a remote, conflict, or refresh path. A flagged or blocked strip always names its resolution path, a
caveat, and the stale-or-blocked field driving the downgrade; a blocked strip warns before the downstream
run failure; and a cleanly resolved strip must be whole — every shown facet current, status resolved,
nothing flagging it.

The recorded presentation, reasons, resolution path, and blocked-before-run flag are recomputed and
validated against the gate, so a clean chip can never be asserted by hand over a degraded context
(`M5EnvironmentStatusStrips::validate()`).

## One execution-context truth across surfaces

Five consumer surfaces bind to this one registry: the desktop shell, the Support Center, the support
export, the issue-report packet, and the CLI / headless reference. Each binding must ingest the registry,
preserve its status vocabulary and object ids verbatim, and narrow with it — so the same status truth and
object ids appear across desktop, Support Center, support packets, and CLI, and a strip flagged or blocked
here cannot read as a clean current-target chip on a downstream surface.

This registry is a supportability surface for explaining where a run happens; it does not widen the
execution context beyond what the authority and policy rows already allow.
