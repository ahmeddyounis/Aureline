# Managed-workspace lifecycle truth

This document describes the managed-workspace lifecycle truth lane implemented by
`aureline-remote`. It makes the managed-workspace lifecycle a first-class
reviewed concept on the M5 remote and companion flows that provision, warm,
suspend, resume, reconnect, reprovision, expire, or hand off to a
browser/companion surface.

It reuses the canonical managed-workspace lifecycle vocabulary frozen at
[`artifacts/runtime/managed_workspace_lifecycle.yaml`](../../artifacts/runtime/managed_workspace_lifecycle.yaml)
rather than inventing a second continuity language for companion or preview
surfaces.

## Scope

Frozen here:

- a `ManagedWorkspaceLifecyclePage` proof packet consumed by desktop, preview,
  companion, incident, and support/export surfaces;
- one `LifecycleRecord` per required lifecycle state: `provision`, `warm`,
  `ready`, `suspended`, `resumed`, `reconnecting`, `rebuild_required`,
  `recreate_required`, `expired`, and `local_safe_continuation`;
- typed persistence class, template/image provenance, continuity class,
  recovery options, expiry posture, local-safe-continuation availability, and
  caveat history on every record;
- a non-inheriting disposition gate that narrows, flags, or withholds the
  published lifecycle claim instead of implying exact continuity when the
  runtime changed materially.

Out of scope:

- provisioning backends, billing, quotas, fleet orchestration, or broad
  control-plane UX;
- tunnel backends, browser automation, remote-agent startup, or real process
  attach;
- a second product-specific lifecycle vocabulary for companion or preview
  surfaces;
- raw URLs, hostnames, ports, image bytes, credentials, or support-bundle
  bodies.

## Lifecycle truth rules

1. **State is explicit and attributable.** Every record names its lifecycle
   state, the prior state it transitioned from, and a typed transition reason.
   Silent transitions are non-conforming.
2. **Continuity is never overclaimed.** A resume or reprovision may claim
   `exact_continuity` only when the persistence class, template/image
   provenance, and target identity all stayed the same. Claiming exact
   continuity over a material change is a `continuity_overclaim` defect that
   narrows the published claim.
3. **Material changes carry caveats.** When the persistence class, provenance,
   or target identity changed, the record carries a non-empty typed caveat
   history (for example `image_changed`, `persistence_class_changed`,
   `target_identity_changed`, `scratch_state_discarded`) that survives resume
   and reprovision.
4. **Outages degrade to attributable local-safe continuation.** Reconnecting,
   rebuild-required, recreate-required, expired, and local-safe-continuation
   states must offer local-safe continuation and at least one recovery option.
   A control-plane outage or expiry therefore degrades to an explicit local-safe
   state rather than a generic loss-of-context failure.
5. **Identity stays stable across surfaces.** Every record declares a stable
   opaque `target_identity_ref` and reaches every required surface (`desktop`,
   `preview_route`, `companion_handoff`, `incident_packet`, `support_export`) so
   the lifecycle event a user sees is identical to the one logs, incident
   packets, and support exports quote.

## Dispositions

| Disposition | When |
|-------------|------|
| `truthful` | All truthfulness conditions hold; the lifecycle claim is published. |
| `narrowed` | A continuity, caveat, or local-safe-continuation condition narrowed the claim; the narrowing is published explicitly. |
| `flagged` | A coverage gap (missing required state or surface) flags the record for review. |
| `withheld` | Raw private material was exposed or the target identity was undeclared; the claim is withheld. |

## Guardrail

> Do not let a managed resume path imply exact continuity when the backing
> image, template, persistence class, or target identity changed materially.

The `continuity_overclaim` rule enforces this directly in the audit.

## Artifacts

- Crate model:
  [`/crates/aureline-remote/src/managed_workspace_lifecycle/mod.rs`](../../crates/aureline-remote/src/managed_workspace_lifecycle/mod.rs)
- Schema:
  [`/schemas/remote/managed_workspace_lifecycle.schema.json`](../../schemas/remote/managed_workspace_lifecycle.schema.json)
- Artifact summary:
  [`/artifacts/remote/managed_workspace_lifecycle.md`](../../artifacts/remote/managed_workspace_lifecycle.md)
- Fixtures:
  [`/fixtures/remote/managed_workspace_lifecycle/`](../../fixtures/remote/managed_workspace_lifecycle/)

## Verification

```sh
cargo test -p aureline-remote managed_workspace_lifecycle
```

Regenerate the fixtures from the seeded packet:

```sh
cargo run -q -p aureline-remote --example dump_managed_workspace_lifecycle_fixtures -- page
```
