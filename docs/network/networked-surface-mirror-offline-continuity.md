# Networked-surface mirror/offline continuity

This continuity layer is the **capstone** of the networked-surface
transport-governance lane. The
[transport-decision log](./networked-surface-transport-decision.md) emits one
inspectable decision per network-capable *action*; this layer answers the
question those decisions leave open for each claimed M5 *artifact family*: when
the primary route is stale, deferred, or policy-blocked, how does the family's
content resolve, and does local work keep going regardless?

The runtime owner is
`aureline_remote::networked_surface_mirror_offline_continuity`; the boundary
schema is
`schemas/network/networked_surface_mirror_offline_continuity.schema.json`.

No raw mirror URLs, raw hostnames, raw ports, raw credentials, raw bearer or
session tokens, raw cookie jars, raw private certificate bytes, raw SSH private
material, or raw mirror bodies cross the boundary — only closed-vocabulary
tokens, opaque refs, and plain-language summary sentences.

## Artifact families

The layer governs the mirror/offline continuity of five claimed M5 artifact
families, each carrying exactly one [`ContinuityRecord`]:

- `docs_pack` — documentation packs (bundled or fetched docs content),
- `registry` — extension / artifact registry reads,
- `model_pack` — AI model packs (downloadable weights and metadata),
- `request_workspace` — request/API client workspaces (collections, environments),
- `companion_handoff` — companion device handoff payloads.

## The five route-handling behaviors

Every record resolves its family to exactly one [`ContinuityRouteClass`], so
product surfaces and exported decision records agree on a single token rather
than guessing whether a family fell through to the public internet, served a
cached bundle, or was blocked:

- `mirror_route` — served from a declared signed mirror; deny rather than reach
  the public internet when the mirror is unavailable,
- `local_file_bundle` — served from a validated local bundle/cache with no live
  egress,
- `public_direct` — an **explicitly declared** public egress, never a silent
  fall-through from a confined profile,
- `blocked` — denied by policy, with a typed denial reason,
- `deferred` — queued for **idempotent** offline replay.

## Stale-mirror warnings

Each record carries a typed [`StaleMirrorWarningClass`] so a stale mirror
surfaces a warning instead of silently serving expired content:

- `none` — current,
- `stale_within_grace` — stale but inside the accepted grace window (advisory),
- `stale_beyond_grace` — stale past the window; the route **must block**,
- `mirror_root_mismatch` — the declared root does not match the served mirror;
  the route **must block**.

A blocking warning paired with a non-blocked route narrows the row to `beta`
(`stale_mirror_served_beyond_grace`).

## Explicit public-fallback rules

Each record declares a [`PublicFallbackRuleClass`] so the public-fallback rule
is never folklore:

- `no_public_fallback`, `explicit_public_direct_allowed`,
  `mirror_only_no_fallback`, `deny_all_no_fallback`.

The declared rule must be consistent with the route handling
(`mirror_route → mirror_only_no_fallback`,
`local_file_bundle → no_public_fallback`,
`public_direct → explicit_public_direct_allowed`,
`blocked → deny_all_no_fallback`, `deferred → no_public_fallback`); an
inconsistency narrows the row to `beta` (`fallback_rule_inconsistent`).

## Local-core continuity

Every record names the local-core workflow that stays usable when the family's
network route is denied or deferred (`local_core_workflow`) and asserts
`local_core_continuity_preserved`. Local editing, offline docs reading,
already-installed extensions and models, and local request-collection editing
all continue regardless of network availability.

## Stability conditions

The page qualifies `stable` only when **all** of the following hold for every
covered family:

1. Every required artifact family has a continuity record.
2. No raw private material is present on any record.
3. Every record resolved through the shared governance layer (`no_bypass: true`).
4. No mirror-only or deny-all profile permits a silent fall-through to the public
   internet.
5. Any deferred record queues only an idempotent action.
6. Every record preserves local-core continuity.
7. Every blocked record carries a typed denial reason.
8. Every record carries a non-empty trust-proof ref.
9. Every record's trust proof is fresh (or stale only within a grace window).
10. Every record's declared public-fallback rule matches its route handling.
11. No record serves a mirror whose stale-mirror warning is blocking instead of
    blocking the route.

## Hard guardrails — withdrawal conditions

Any one of these forces `withdrawn` immediately and cannot be overridden:

- a record with `raw_private_material_excluded: false`
  (`raw_private_material_exposed`),
- a record with `no_bypass: false` (`bypassed_shared_governance`),
- a mirror-only or deny-all profile that would silently reach the public
  internet (`silent_public_fallback_resolved`),
- a deferred record that queues a non-idempotent action
  (`non_idempotent_replay_queued`).

A missing required family narrows the packet to `preview`. A block without a
typed reason, a stale-beyond-window proof, an inconsistent fallback rule, a
stale-mirror served beyond grace, or a record that does not preserve local-core
continuity narrows the affected row to `beta`, which lets release and support
tooling detect and automatically narrow stale or under-qualified rows before
publication.

## CLI / support / product parity

Product, CLI/headless, and support exports all render a continuity record
through `render_fields()` over the single [`CONTINUITY_FIELD_NAMES`] catalog, so
the route-handling tokens a user reads in the UI are byte-for-byte the same
tokens CLI output and support packets quote. The
`dump_networked_surface_mirror_offline_continuity_fixtures` example's
`continuity-cli` subcommand is the headless rendering of this catalog.

## Truth paths

- Doc: `docs/network/networked-surface-mirror-offline-continuity.md`
- Artifact: `artifacts/network/networked-surface-mirror-offline-continuity.md`
- Schema:
  `schemas/network/networked_surface_mirror_offline_continuity.schema.json`
- Fixtures: `fixtures/network/networked_surface_mirror_offline_continuity/`
- Contract ref: `remote:networked_surface_mirror_offline_continuity:v1`

[`ContinuityRecord`]: ../../crates/aureline-remote/src/networked_surface_mirror_offline_continuity/mod.rs
[`ContinuityRouteClass`]: ../../crates/aureline-remote/src/networked_surface_mirror_offline_continuity/mod.rs
[`StaleMirrorWarningClass`]: ../../crates/aureline-remote/src/networked_surface_mirror_offline_continuity/mod.rs
[`PublicFallbackRuleClass`]: ../../crates/aureline-remote/src/networked_surface_mirror_offline_continuity/mod.rs
[`CONTINUITY_FIELD_NAMES`]: ../../crates/aureline-remote/src/networked_surface_mirror_offline_continuity/mod.rs
