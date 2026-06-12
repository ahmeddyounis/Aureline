# Managed-workspace lifecycle proof packet

Canonical artifact summary for the managed-workspace lifecycle truth lane on the
M5 remote and companion flows. The checked-in implementation, schema, fixtures,
and this summary are canonical; later product/help/support surfaces consume them
instead of re-describing lifecycle state manually.

- **Crate model:** `crates/aureline-remote/src/managed_workspace_lifecycle/mod.rs`
- **Schema:** `schemas/remote/managed_workspace_lifecycle.schema.json`
- **Fixtures:** `fixtures/remote/managed_workspace_lifecycle/`
- **Doc:** `docs/remote/managed_workspace_lifecycle.md`
- **Contract ref:** `remote:managed_workspace_lifecycle:v1`

## What the packet proves

The `ManagedWorkspaceLifecyclePage` makes the managed-workspace lifecycle a
first-class reviewed concept on the lanes that provision, warm, suspend, resume,
reconnect, reprovision, expire, or hand off to a browser/companion surface. It
embeds one `LifecycleRecord` per required lifecycle state and derives a per-state
disposition plus an overall disposition.

## Lifecycle states (closed vocabulary, in order)

| State | Meaning |
|-------|---------|
| `provision` | Control plane is allocating the workspace from a template/image; no runtime reachable yet. |
| `warm` | Workspace is booting and warming caches; runtime exists but not ready. |
| `ready` | Workspace is reachable and ready for interactive work. |
| `suspended` | Paused by idle window or request; persistent volume survives. |
| `resumed` | Resumed from suspension; exact continuity only when nothing changed materially. |
| `reconnecting` | Connection dropped and is being re-established; local-safe continuation applies. |
| `rebuild_required` | Successor image or drift makes the prior runtime non-resumable in place. |
| `recreate_required` | Identity or backing volume cannot be recovered; a new workspace must be minted. |
| `expired` | An idle/hibernation/hard-deadline window elapsed; the runtime is gone. |
| `local_safe_continuation` | Control plane unreachable; work continues against a local-safe mirror with explicit caveats. |

## Per-record truth fields

Each record exposes — before the user loses context — the transition reason,
persistence class (and whether it changed), template and image provenance (and
whether provenance changed), a stable opaque target-identity ref (and whether
identity was preserved), the claimed continuity class, the recovery options, the
expiry class and an opaque expiry-timing ref, whether local-safe continuation is
available, and the typed caveat history that survives resume and reprovision.
Every record names the consuming surfaces it reaches: `desktop`,
`preview_route`, `companion_handoff`, `incident_packet`, `support_export`.

## Disposition gate

A record publishes `truthful` only when all of the following hold; otherwise it
is `narrowed`, `flagged`, or `withheld` with a typed narrow reason:

1. No raw private material is exposed (`raw_private_material_excluded: true`) —
   else `withheld` immediately.
2. A stable target-identity ref is declared — else `withheld`.
3. The record reaches every required consuming surface — else `flagged`.
4. A claimed `exact_continuity` is not contradicted by a changed persistence
   class, changed template/image provenance, or changed target identity — else
   `narrowed` (`continuity_overclaim`).
5. Any material change carries a non-empty caveat history — else `narrowed`
   (`caveat_history_missing`).
6. Outage and expiry states (`reconnecting`, `rebuild_required`,
   `recreate_required`, `expired`, `local_safe_continuation`) offer local-safe
   continuation and at least one recovery option — else `narrowed`
   (`local_safe_continuation_unavailable`).

A missing required state flags the packet (`required_state_missing`) without
forcing any individually-clean state below `truthful`.

## Guardrail

A managed resume path may never imply exact continuity when the backing image,
template, persistence class, or target identity changed materially. The
`continuity_overclaim` rule enforces this directly: claiming `exact_continuity`
over a material change is a defect that narrows the published claim.

## Export safety

The packet carries record-kind tags, schema-version integers, closed-vocabulary
tokens, plain-language summary sentences, and opaque refs only. Raw credentials,
raw private keys, raw image bytes, raw endpoint URLs, and raw PII never appear on
any record.

## Verification

```sh
cargo test -p aureline-remote managed_workspace_lifecycle
cargo run -q -p aureline-remote --example dump_managed_workspace_lifecycle_fixtures -- page
```
