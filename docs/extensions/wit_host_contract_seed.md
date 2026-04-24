# Wasm / WIT extension-host contract seed

This document is the narrative companion to ADR-0019
([`docs/adr/0019-wasm-wit-extension-host-and-capability-worlds.md`](../adr/0019-wasm-wit-extension-host-and-capability-worlds.md)).
It names the reserved capability worlds, the host-family binding,
the host / guest negotiation packet, and the compatibility-bridge
scope in a reviewer-friendly form. The ADR is authoritative when
the narrative and the ADR disagree; this document MUST be updated
in the same change that lands any ADR-0019 successor.

The seed is deliberately narrow. It does **not** land the extension
runtime, nor the SDK, nor the full world library, nor the
compatibility-bridge profile set beyond shape. Its job is to make
the reserved vocabulary usable by the manifest, permission-inspector,
registry, and install-review lanes before the runtime ADR closes.

The normative interfaces live as WIT files under
[`/wit/aureline/`](../../wit/aureline/). The capability-world registry
that binds WIT worlds to host families, lifecycle rows, and permission-
scope projections lives at
[`/artifacts/extensions/capability_worlds.yaml`](../../artifacts/extensions/capability_worlds.yaml).
The typed host-negotiation packet lives at
[`/schemas/extensions/host_negotiation.schema.json`](../../schemas/extensions/host_negotiation.schema.json).

## What this seed freezes

1. A **capability-world naming scheme**:
   `aureline:<world-slug>@<world-semver>` under the reserved
   `aureline` WIT namespace. Slugs are never reused after retirement.
2. Five **seeded worlds** — `editor-read`, `workspace-read`,
   `diff-apply-preview`, `terminal-observe`, `network-egress` — each
   with a WIT seed under `/wit/aureline/`. Additional worlds are
   additive-minor with a `world_vocabulary_version` bump.
3. A **host-family binding shape** for the six ADR-0012 families
   (`wasm_component_model`, `wasm_core_module`,
   `external_host_process`, `helper_binary`, `remote_side_component`,
   `compatibility_bridge`). The binding names what a manifest's
   `host_contract_identity_ref` resolves to per family.
4. A **host-negotiation packet** that every session records: declared
   worlds, offered worlds, negotiated worlds, ABI ranges, budget
   declarations, narrowing reasons, unsupported-world decisions,
   trust-state ref, identity-mode ref, execution-context ref, policy-
   pack constraint refs, and compatibility-bridge binding ref.
5. A **capability-narrowing contract**: admin policy packs (ADR-0012),
   host context (identity mode, trust state, execution-context posture,
   freshness floor), capability-lifecycle rows (ADR-0011), and
   compatibility-bridge translation each narrow, never widen.
6. A **budget-declaration vocabulary** (wall-clock, memory, network
   egress, filesystem read / write, subscription fanout, terminal
   observe) with enumerated classes. Quantitative ceilings per class
   land in the successor ADR.
7. A **compatibility-bridge scope envelope**: a bridge profile is a
   typed row with `honoured_worlds`, `translated_worlds`, and
   `refused_worlds`. A bridge never widens a world, never mints an
   `aureline:`-namespace world identity, and never substitutes
   ADR-0012 publisher continuity, signatures, or digests.
8. A **denial posture**: every failure resolves to a typed reason and
   a repair affordance. Silent fallback to a generic "unavailable"
   chip is forbidden.

## Capability-world naming

Reserved identity pattern:

```
aureline:<world-slug>@<MAJOR.MINOR.PATCH>
```

- `world-slug` is kebab-case, short, decision-focused.
  Reserved slugs at this seed: `editor-read`, `workspace-read`,
  `diff-apply-preview`, `terminal-observe`, `network-egress`.
- `world-semver` uses standard semantic-versioning rules:
  additive fields / records / resources / functions bump `MINOR`;
  repurposing any named item bumps `MAJOR` and requires a new
  decision row.

Third-party capability worlds are permitted. They MUST carry
their own namespace (`<publisher-id>:<world-slug>@<semver>`) and
MAY NOT mint identities under the reserved `aureline` namespace.

## Seeded worlds

Each world below has a normative WIT seed at
`/wit/aureline/<slug>.wit`. The ADR reserves the world; the WIT
file is the contract. The narrative here is a reviewer-facing
summary.

### `aureline:editor-read@0.1`

Read-only access to the editor buffer(s) exposed to the extension.
Surface: cursor position, selection, text in a bounded range, and
syntax-scope hints for the active editor.

Permission-scope projection (ADR-0012):
`ui_command_contribute`, `subscription_subscribe`.

Write, apply, or mutate? Out of scope; those route through
`aureline:diff-apply-preview`.

### `aureline:workspace-read@0.1`

Read-only workspace traversal. Surface: list declared-prefix paths,
read declared-prefix bytes up to a bounded size, read workspace
settings through the ADR-0008 resolver.

Permission-scope projection (ADR-0012):
`filesystem_read`, `workspace_settings_read`, `subscription_subscribe`.

This world binds filesystem-touching calls to the ADR-0006 VFS
path-identity contract. Extensions never crawl disk directly.

### `aureline:diff-apply-preview@0.1`

Propose a typed text-edit diff against the active editor or a
workspace file. Application is host-side only and requires the
ADR-0012 `filesystem_write` scope plus the host approval
affordance.

Permission-scope projection (ADR-0012):
`filesystem_write` (for the apply path), `ui_command_contribute`,
`subscription_subscribe`.

The world surface distinguishes `preview` (required, always
available when declared) from `apply` (gated by approval, trust
state, and admin policy). An extension MUST NOT rely on `apply`
firing.

### `aureline:terminal-observe@0.1`

Subscribe to an existing terminal's observable surface: line
output, status, exit code. Launching a terminal, running a
repo-owned recipe, or injecting input is out of scope
(ADR-0018 binds those to `terminal_repo_recipe_launch` and
`terminal_manual_open`; only the host affordance fires).

Permission-scope projection (ADR-0012):
`subscription_subscribe`.

### `aureline:network-egress@0.1`

Outbound network egress, bounded to a declared host allow-list.
The host applies the ADR-0012 `egress_host_narrowing` policy-pack
constraint at admission.

Permission-scope projection (ADR-0012):
`network_egress`.

## Host-family binding

Manifests quote a `host_contract_family` (ADR-0012) plus a
`host_contract_identity_ref`. This seed binds each family to a
typed row:

- `wasm_component_model` → WIT package id (e.g.
  `aureline:worlds@0.1`), world-version, component-model ABI
  range, one or more `capability_world_ref` entries.
- `wasm_core_module` → `host_imports_ref` naming a curated subset
  of the negotiated capability-world surface, core-module ABI
  range. WIT is not native to core modules; the host-side adapter
  projects the named worlds onto imports.
- `external_host_process` → `process_contract_id`, one or more
  `capability_world_ref` entries exposed as an RPC surface, a
  declared launch class.
- `helper_binary` → `helper_binary_contract_id`, one or more
  `capability_world_ref` entries (narrowed to a subset), a
  kill-switch and short-lived window contract (successor-ADR
  concrete).
- `remote_side_component` → `remote_side_component_contract_id`,
  `capability_world_ref` set scoped to `remote_agent` client
  scope, inherits the remote's identity-mode envelope.
- `compatibility_bridge` → `bridge_profile_id`,
  `bridge_translation_ref` (honoured / translated / refused per
  world).

## Host-negotiation packet

Every extension session resolves through one host-negotiation
packet before any capability function fires. The schema at
`/schemas/extensions/host_negotiation.schema.json` is normative;
fixture at
`/fixtures/extensions/host_negotiation_examples/declared_vs_negotiated_example.yaml`
demonstrates a worked example.

Declared vs offered vs negotiated:

- **Declared** worlds come from the manifest (ADR-0012).
- **Offered** worlds are what the host is prepared to surface
  for this session, after reading identity mode, trust state,
  admin policy, and lifecycle rows.
- **Negotiated** worlds are the intersection of declared and
  offered. The host admits the intersection; any declared world
  not in the offered set records an `unsupported_world_decision`
  with a typed repair affordance.

Widening is denied. A host that admitted a world not present in
both declared and offered sets violates the ADR-0012
`effective_permission_widening_attempted` denial reason.

## Budget declarations

Each budget class is enumerated; a budget not declared falls to
the host default (successor-ADR concrete). Policy packs and trust
state may narrow further; they never widen.

| Budget class                      | Members                                                                          |
|-----------------------------------|----------------------------------------------------------------------------------|
| `wall_clock_budget_class`         | `short_bounded`, `medium_bounded`, `long_bounded`, `streaming_bounded`           |
| `memory_budget_class`             | `small_fixed`, `medium_fixed`, `streaming_bounded`                               |
| `network_egress_budget_class`     | `none`, `metered_per_session`, `metered_per_invocation`                          |
| `filesystem_read_budget_class`    | `none`, `declared_prefix_only`, `declared_prefix_metered`                        |
| `filesystem_write_budget_class`   | `none`, `declared_prefix_preview_only`, `declared_prefix_metered`                |
| `subscription_fanout_budget_class`| `none`, `bounded_per_session`, `bounded_per_subscription`                        |
| `terminal_observe_budget_class`   | `none`, `attached_sessions_only`, `attached_sessions_metered`                    |

## Capability narrowing

Orthogonal axes. Each may narrow; none may widen.

1. **Admin policy packs** — ADR-0012 `allow_list`, `deny_list`,
   `permission_floor`, `version_pin`, `version_floor`,
   `mirror_rule`, `emergency_disable`, `signed_continuity_required`,
   `managed_only_narrowing`, `egress_host_narrowing`,
   `freshness_floor_narrowing`.
2. **Host context** — identity mode (ADR-0001), trust state
   (ADR-0018), execution-context posture (ADR-0009), freshness
   floor (ADR-0011).
3. **Capability-lifecycle row** — ADR-0011 readiness / support /
   channel / freshness / client-scope markers; a world whose row
   renders `degraded_by_trust`, `degraded_by_policy`, or
   `retired` narrows accordingly.
4. **Compatibility-bridge translation** — a bridge profile
   declares `honoured_worlds`, `translated_worlds`, and
   `refused_worlds`; a refused world denies with
   `compatibility_bridge_required_not_present` (ADR-0012) or
   `host_negotiation_compatibility_bridge_refused_world`.

## Compatibility-bridge scope

Bridges exist so Aureline can adopt existing extension ecosystems
without inventing a parallel runtime. The bridge profile binds a
foreign ecosystem to the reserved capability-world set. This seed
names the shape; the profile set lands in the successor ADR.

Reserved profile fields: `bridge_profile_id`,
`bridge_translation_class`, `honoured_worlds`, `translated_worlds`,
`refused_worlds`, `budget_translation_rules`, `schema_version`.

Reserved `bridge_translation_class` members:
`vs_code_compatible`, `language_server_protocol_only`,
`debug_adapter_protocol_only`, `generic_helper_wrapper`,
`bespoke_profile`. Adding a class is additive-minor; the
successor ADR selects which classes ship at first beta.

A bridge MAY NOT:

- Widen a world beyond its declared Aureline surface.
- Mint a new world identity under the `aureline:` namespace.
- Substitute the ADR-0012 publisher-continuity, signature, or
  digest invariants of the underlying artifact.
- Bypass the ADR-0018 trust-state packet or the ADR-0008 admin-
  policy narrowing ceiling.

The bridge's scope is ecosystem translation, not trust
substitution.

## Unsupported-world behavior

A declared world the host does not implement at the negotiated
ABI range (or that is retired, or that a compatibility bridge
refuses) records a typed `unsupported_world_decision`: the
declared world ref, the typed reason, a nullable successor world
ref, a human-legible repair affordance label, and the audit-event
ref.

Install-review and permission-inspector surfaces MUST render the
repair affordance. Silent drop is non-conforming.

## Negotiation audit events

The eventual extension-host crate emits a typed audit stream on
`extension_host_negotiation`. Reserved events at this seed:

- `host_negotiation_opened`
- `host_negotiation_worlds_narrowed`
- `host_negotiation_world_denied`
- `host_negotiation_budget_narrowed`
- `host_negotiation_abi_mismatch`
- `host_negotiation_world_vocabulary_mismatch`
- `host_negotiation_compatibility_bridge_applied`
- `host_negotiation_compatibility_bridge_refused_world`
- `host_negotiation_completed`
- `host_negotiation_retired_world_denied`
- `host_negotiation_trust_state_denied`
- `host_negotiation_policy_pack_denied`
- `host_negotiation_remote_side_component_denied`
- `host_negotiation_helper_binary_launch_denied`

Raw component bytes, raw helper-binary invocation bodies, raw
bridge-shim payloads, raw process launch bodies, raw signing-key
material, and raw policy-bundle bytes never appear on any event.

## Denial reasons

In addition to the ADR-0012 set, the seed reserves:

- `host_negotiation_world_unknown_to_host`
- `host_negotiation_world_retired`
- `host_negotiation_host_abi_range_mismatch`
- `host_negotiation_guest_abi_range_mismatch`
- `host_negotiation_world_vocabulary_version_unknown`
- `host_negotiation_budget_declaration_unacceptable`
- `host_negotiation_compatibility_bridge_refused_world`
- `host_negotiation_compatibility_bridge_profile_unbound`
- `host_negotiation_remote_side_component_client_scope_mismatch`
- `host_negotiation_helper_binary_launch_refused`
- `host_negotiation_trust_state_denies_world`
- `host_negotiation_policy_pack_denies_world`
- `host_negotiation_disclosure_incomplete`

## Consumer expectations

The downstream surfaces below MUST read this seed rather than
invent extension-host-shaped fields:

- **Extension manifest** (ADR-0012) — `host_contract_family` and
  `host_contract_identity_ref` resolve through the host-family
  binding above; `compatibility_bridge_notes` resolves through a
  bridge-profile row.
- **Install / update review sheet** — projects the negotiated-
  capability-worlds set alongside the declared-vs-effective
  permission diff. A surface that hides the negotiated set denies
  with `review_disclosure_incomplete`.
- **Permission inspector** — per-world authority rendered per
  axis (identity mode, trust state, admin policy, lifecycle row,
  bridge translation).
- **Support export** — carries negotiation id, world refs, budget
  class labels, and narrowing-reason labels only. Raw payload
  bytes forbidden.
- **Mutation-journal entry / save manifest / claim manifest** —
  same fields as support export; no raw bytes.
- **Eventual SDK seed** — binds to each WIT world; stability-
  window labels align with the world-semver axis (successor-ADR
  concrete).
- **Registry index and mirror adapter** — index on world refs;
  mirrors preserve the world set without rewriting it.

## Open questions

Tracked in ADR-0019 §Open questions and the `decision_history`
of `D-0024`. They block the `Proposed` → `Accepted` promotion.
Later lanes MUST NOT assume a resolution silently; if the
successor ADR lands a choice that contradicts a lane's current
behaviour, that lane takes the narrower posture until it is
updated.
