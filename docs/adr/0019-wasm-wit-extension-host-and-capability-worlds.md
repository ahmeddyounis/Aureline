# ADR 0019 — Wasm / WIT extension-host, capability-world, and compatibility-bridge ADR seed

- **Decision id:** D-0024 (see `artifacts/governance/decision_index.yaml#D-0024`)
- **Status:** Proposed — this is an ADR seed. The host-family binding, the initial WIT capability-world set, the host / guest negotiation vocabulary, the capability-narrowing rules, the budget-declaration vocabulary, the unsupported-world denial posture, and the compatibility-bridge scope below reserve shape and record fields so the extension-runtime, SDK, registry-mirror, install-review, and permission-inspector lanes at a later milestone cannot invent them ad hoc. Full freeze lands in a successor ADR once the open questions in §Open questions are closed.
- **Decision date:** pending
- **Freeze deadline:** 2027-01-31
- **Owner:** `@ahmeddyounis`
- **Backup owner:** `null` (covered by waiver `single-maintainer-backup` in `artifacts/governance/ownership_matrix.yaml#waivers`)
- **Forum:** compatibility_ecosystem_review (co-required with security_trust_review because the capability-world contracts carry the permission-scope projection invariants the trust-review remit already owns, and with architecture_council because the host-family binding rides the RPC / subscription / execution-context contracts)
- **Related requirement ids:** none

## Context

ADR-0012 (`docs/adr/0012-extension-manifest-permission-publisher-policy.md`)
froze the extension-manifest, effective-permission, publisher-continuity,
and policy-pack vocabulary without committing to any particular
extension-host runtime. It reserved a `host_contract_family` enum with
six members — `wasm_component_model`, `wasm_core_module`,
`external_host_process`, `helper_binary`, `remote_side_component`,
`compatibility_bridge` — and a `host_contract_identity_ref` field that
manifests quote to name the ABI they were written against, but it did
**not** name any actual ABI, capability-world, or negotiation contract.
The successor ADR now under preparation (the runtime ADR that closes
`D-0018`) cannot close its open questions on sandbox model and full
WIT world layout until the worlds themselves exist as typed interface
seeds rather than prose.

The `.t2` source documents already commit the product to a
Wasm-first host with external-host escape hatches, a compatibility
bridge for pre-existing extension ecosystems, and a reviewer-visible
effective-permission summary that must quote named host contracts
(`.t2/docs/Aureline_Technical_Architecture_Document.md` — "Wasm-first
contracts map to WIT/component-model worlds"; "Extension host <->
platform SDK | manifest SDK range, permission vocabulary version, WIT
ABI version"; "AD-008 | Extension runtime | Wasm capability sandbox
+ isolated external hosts"). Without a typed-interface seed, every
downstream surface — the manifest's `host_contract_identity_ref`,
the review sheet's `host_contract_and_artifact_transport` field, the
permission inspector's per-scope capability projection, the registry
index's ABI-range row, the SDK's bindgen target, the
`host_contract_family_unsupported_on_target` denial reason, the
`compatibility_bridge_required_not_present` denial reason, and the
eventual runtime sandbox contract — would either have to invent its
own world identity scheme or defer to hand-maintained prose. Both
options leave ADR-0012's `host_contract_identity_ref` field as a
free-form string and strand ADR-0012's `compatibility_bridge_notes`
field with no profile set to quote.

This ADR rides alongside ADR-0001 (the `managed_admin_surface`
client-scope gate applies to any world whose capability contributes
to admin-facing surfaces), ADR-0004 (host / guest negotiation
packets and capability-world manifests cross RPC as typed payloads;
raw Wasm component bytes, raw external-host launch bodies, raw
helper-binary invocation bodies, and raw bridge-shim payloads never
do), ADR-0005 (per-world capability views ride the shared
subscription envelope with authority class `derived_knowledge`),
ADR-0006 (filesystem-touching worlds bind to the VFS path-identity
contract; they never crawl disk directly), ADR-0007 (any world that
requests credential material projects through the secret broker's
handle classes only), ADR-0008 (admin-policy narrowing is an
orthogonal ceiling over declared worlds), ADR-0009 (worlds that
reach the shell / command / task surface bind to the
execution-context object model), ADR-0010 (worlds that reach
connected providers quote the grant-resolution vocabulary), ADR-0011
(capability-world rows carry the five-axis lifecycle markers; a
degraded world reads `degraded_by_trust` / `degraded_by_policy`
without inventing a world-local posture), ADR-0012 (host-family
binding, artifact-transport family, and permission-scope vocabulary
are already frozen — this ADR binds them to named worlds),
ADR-0015 (remote-agent-scoped worlds surface through the
native-reserved embedded-surface boundary), ADR-0016 (terminal
worlds route through the command-dispatch entry point), and
ADR-0018 (every world resolves one shared `trust_state` packet
before admitting a capability invocation — a handle issued under
`trusted` MUST NOT survive downgrade to `restricted`).

The extension runtime itself does not land at this milestone. What
this seed reserves is the **capability-world identity scheme**, the
**host / guest negotiation vocabulary**, and the **initial WIT
seeds** for five small worlds — editor read, workspace read,
diff / apply preview, terminal observe, and network egress — so
the successor ADR has concrete interfaces to compose against rather
than prose. The broader world library (debug, notebook kernel, AI
apply, extension-to-extension dependency, identity gate) lands in
the successor ADR; this seed reserves the binding surface.

Runtime sandbox enforcement, the full world library, the
compatibility-bridge profile set (beyond naming the bridge scope),
the SDK bindgen target, the helper-binary launch contract, and the
remote-side component attach contract are explicitly out of scope
here. They land with the successor ADR (`D-0018` and `D-0024`
together) after the open questions below close.

## Decision

Aureline reserves five record families — **host-family row**,
**capability-world row**, **host-negotiation packet**,
**world-admission decision record**, and **budget-declaration
record** — plus a fixed naming scheme for WIT packages, an initial
set of typed interface seeds for five worlds, a frozen negotiation
vocabulary, and a compatibility-bridge scope envelope. Every vocabulary
named below is opened as an enumerable set whose initial members are
frozen by this seed and whose additions are additive-minor with a
schema-version bump. Where this seed names a WIT interface, the
interface is a seed — additive field, record, resource, or function
additions are minor with a world-version bump; repurposing any named
item is breaking and requires a new decision row.

The intent is deliberately narrower than the successor ADR. This seed
freezes **shape, names, and invariants**, not the sandbox model, nor
the full world library, nor the helper-binary invocation body.

### Host-family binding

Every host family reserved by ADR-0012 maps to exactly one binding
row here. The rows are not a re-freeze of the ADR-0012 vocabulary;
they are the typed contract every later lane reads when resolving a
manifest's `host_contract_family` + `host_contract_identity_ref`
pair.

| Host family                   | Binding kind                                    | Carries                                                                                                                                                                               |
|-------------------------------|-------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `wasm_component_model`        | WIT package + world, component model 0.2.x      | `capability_world_ref` (one or more), `wit_package_id`, `world_version`, `component_model_abi_range`, declared budgets, declared permission-scope projection, runtime-posture marker. |
| `wasm_core_module`            | WIT-free core module, host-imported functions   | `host_imports_ref` (named subset of the negotiated capability-world surface), `core_module_abi_range`, declared budgets, permission-scope projection, runtime-posture marker.         |
| `external_host_process`       | Out-of-process host, typed RPC over the ADR-0004 envelope | `process_contract_id`, `capability_world_ref` set (same world set exposed as an RPC surface), declared launch class, budgets, permission-scope projection, runtime-posture marker.    |
| `helper_binary`               | Short-lived, kill-switch-capable helper         | `helper_binary_contract_id`, `capability_world_ref` set (narrowed to a subset), budgets, permission-scope projection, runtime-posture marker.                                         |
| `remote_side_component`       | Component executing on an attached remote agent | `remote_side_component_contract_id`, `capability_world_ref` set scoped to `remote_agent`, client-scope projection per ADR-0011, budgets, permission-scope projection.                 |
| `compatibility_bridge`        | Shim that translates a foreign ecosystem        | `bridge_profile_id`, `bridge_translation_ref` (declares which capability worlds are `honoured`, `translated`, or `refused`), budgets, permission-scope projection, runtime-posture marker. |

Host-family rows cross the RPC boundary as typed payloads. Raw
component bytes, raw module bytes, raw process launch bodies, raw
helper-binary invocation bodies, and raw bridge-shim packages never
do.

### Capability-world naming scheme

Every Aureline-authored capability world carries a stable, opaque
identity under the `aureline` WIT namespace. Names are reserved;
manifests quote them, the registry indexes them, and the review
surfaces render them. The scheme is:

```
aureline:<world-slug>@<world-semver>
```

- `world-slug` is a short, kebab-case label (`editor-read`,
  `workspace-read`, `diff-apply-preview`, `terminal-observe`,
  `network-egress`). Slugs are never reused after retirement.
- `world-semver` follows semantic-versioning: `MAJOR.MINOR.PATCH`.
  Minor bumps are additive; major bumps are breaking and require a
  new decision row.

The seed reserves five world slugs. Additional slugs are
additive-minor with a `world_vocabulary_version` bump.

- `aureline:editor-read@0.1` — read-only access to the active
  editor buffer(s) exposed to the extension: cursor position,
  selection, text in a bounded range, and syntax-scope hints.
- `aureline:workspace-read@0.1` — read-only workspace traversal:
  listing declared-prefix paths, reading declared-prefix bytes,
  reading workspace settings through the ADR-0008 resolver.
- `aureline:diff-apply-preview@0.1` — propose a typed text-edit
  diff against the active editor or a workspace file; preview
  only, application requires the host-side approval affordance and
  the ADR-0012 `filesystem_write` scope.
- `aureline:terminal-observe@0.1` — subscribe to an existing
  terminal's observable surface (line output, status, exit code);
  launching a new terminal or running a repo-owned recipe is out
  of scope for this world (ADR-0018 binds that to
  `terminal_repo_recipe_launch`).
- `aureline:network-egress@0.1` — outbound network egress, bounded
  to a declared host allow-list; the host applies the ADR-0012
  `egress_host_narrowing` policy-pack constraint at admission.

Third-party capability worlds are permitted; they MUST carry their
own namespace (`<publisher-id>:<world-slug>@<world-semver>`) and
MAY NOT mint identities under the reserved `aureline` namespace.

### Initial WIT package seeds

The WIT package seeds live at `wit/aureline/*.wit`. They are the
boundary contract every later SDK binding, every later runtime
import surface, and every later host adapter reads. Breaking a
named interface requires a new decision row; additive fields,
records, resources, and functions are minor with a world-version
bump.

A package `aureline:worlds@0.1` (root) reserves the top-level
world names and imports the per-capability packages. The per-
capability packages are:

- `aureline:editor-read@0.1` at `wit/aureline/editor-read.wit`.
- `aureline:workspace-read@0.1` at `wit/aureline/workspace-read.wit`.
- `aureline:diff-apply-preview@0.1` at `wit/aureline/diff-apply-preview.wit`.
- `aureline:terminal-observe@0.1` at `wit/aureline/terminal-observe.wit`.
- `aureline:network-egress@0.1` at `wit/aureline/network-egress.wit`.

The WIT files themselves are the normative contract. The narrative
companion lives at `docs/extensions/wit_host_contract_seed.md`.

### Host-negotiation packet

Every extension session — an install, an activation, an
invocation, a remote-attach handshake, a bridge-shim exchange —
resolves through one typed **host-negotiation packet** before any
capability function fires. The packet is recorded; the decision is
typed; widening is denied.

Reserved fields:

| Field                              | Notes                                                                                                                                                          |
|------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `negotiation_id`                   | Opaque, stable id for the negotiation (safe to log).                                                                                                           |
| `extension_identity_ref`           | Ref to the ADR-0012 manifest row.                                                                                                                              |
| `extension_version`                | Version declared at negotiation time.                                                                                                                          |
| `host_contract_family`             | One of the ADR-0012 frozen set; read from the manifest.                                                                                                        |
| `declared_capability_worlds`       | Ordered list of `capability_world_ref` entries the manifest declares.                                                                                          |
| `offered_capability_worlds`        | Ordered list of worlds the host offers for this session (may be narrower than declared — per ADR-0018 `trust_state` or ADR-0008 admin policy).                 |
| `negotiated_capability_worlds`     | Ordered list of worlds admitted (always a subset of both declared and offered).                                                                                |
| `host_abi_range`                   | Declared host-side ABI range (e.g. component-model 0.2.x, core-module linking model).                                                                          |
| `guest_abi_range`                  | Declared guest-side ABI range (carried by the manifest).                                                                                                       |
| `permission_vocabulary_version`    | ADR-0012 permission-vocabulary version used to project capability scopes.                                                                                      |
| `world_vocabulary_version`         | World-vocabulary version used to resolve world slugs.                                                                                                          |
| `budget_declarations`              | List of `budget_declaration` entries (see below).                                                                                                              |
| `narrowing_reasons`                | List of typed reasons for every world that was declared but not admitted (see §Narrowing reasons).                                                             |
| `unsupported_world_decisions`      | List of `unsupported_world_decision` entries for any declared world the host does not implement at this ABI range (see §Unsupported worlds).                   |
| `trust_state_ref`                  | Ref to the ADR-0018 `trust_decision_record` that gates the negotiation.                                                                                        |
| `identity_mode_ref`                | Ref to the ADR-0001 identity-mode envelope.                                                                                                                    |
| `execution_context_ref`            | Ref to the ADR-0009 execution-context id when any admitted world binds to the shell / task surface; null otherwise.                                            |
| `policy_pack_constraint_refs`      | Ordered list of ADR-0012 policy-pack constraint rows that narrowed the negotiation.                                                                            |
| `compatibility_bridge_binding_ref` | Ref to the compatibility-bridge binding row when `host_contract_family = compatibility_bridge`; null otherwise.                                                |
| `audit_event_refs`                 | Ordered list of host-negotiation audit events emitted for this negotiation.                                                                                    |
| `redaction_class`                  | Redaction posture (per ADR-0012 `redaction_class` enum).                                                                                                       |
| `computed_at`                      | Monotonic timestamp.                                                                                                                                           |
| `schema_version`                   | Integer; additive-minor on additions.                                                                                                                          |

### Budget declarations

Every negotiation declares, at a minimum, the following budget
classes. A budget not declared is treated as "host default"
(applied narrowly; the successor ADR binds exact defaults).
Budgets never widen at admission; policy packs and `trust_state`
may narrow them further.

- `wall_clock_budget_class` — `short_bounded` | `medium_bounded`
  | `long_bounded` | `streaming_bounded`.
- `memory_budget_class` — `small_fixed` | `medium_fixed` |
  `streaming_bounded`.
- `network_egress_budget_class` — `none` | `metered_per_session`
  | `metered_per_invocation`.
- `filesystem_read_budget_class` — `none` | `declared_prefix_only`
  | `declared_prefix_metered`.
- `filesystem_write_budget_class` — `none` |
  `declared_prefix_preview_only` | `declared_prefix_metered`.
- `subscription_fanout_budget_class` — `none` | `bounded_per_session`
  | `bounded_per_subscription`.
- `terminal_observe_budget_class` — `none` |
  `attached_sessions_only` | `attached_sessions_metered`.

Each budget carries the class plus a reason label. The successor
ADR binds the quantitative ceiling per class; this seed reserves
the vocabulary so later rows cannot mint world-local budget axes.

### Narrowing reasons

Every world declared-but-not-admitted carries one typed
narrowing reason. Reserved values:

- `workspace_trust_restricted` — `trust_state` admitted a
  restricted posture (ADR-0018).
- `admin_policy_deny_list` — an ADR-0012 `deny_list` constraint
  named the world.
- `admin_policy_permission_floor` — a `permission_floor`
  constraint narrowed a required scope below the world's floor.
- `admin_policy_egress_host_narrowing` — an
  `egress_host_narrowing` constraint narrowed the egress set past
  the world's requirement.
- `capability_lifecycle_degraded` — the world's ADR-0011
  lifecycle row is `degraded_by_trust`, `degraded_by_policy`, or
  `retired`.
- `world_vocabulary_version_unknown` — host and guest disagree on
  the world vocabulary version; negotiation denies rather than
  widens.
- `host_abi_range_mismatch` — host and guest ABI ranges do not
  overlap.
- `guest_abi_range_mismatch` — the guest declares an ABI range
  the host does not support at this build.
- `compatibility_bridge_profile_unbound` — a bridge host family
  declared a world the bridge profile does not carry.
- `budget_declaration_unacceptable` — a declared budget class
  exceeds the host's ceiling for this identity mode or trust
  state.

Narrowing reasons are additive-minor.

### Unsupported-world behavior

When a declared world is not implemented at the host ABI range —
for any of the reasons above, or because the world has been
retired — the host emits an **unsupported-world decision record**
and denies admission. The record is typed and carries enough
information that install-review and permission-inspector surfaces
can render a specific repair affordance rather than a generic
"unavailable" chip.

Reserved fields per record:

| Field                              | Notes                                                                                                      |
|------------------------------------|------------------------------------------------------------------------------------------------------------|
| `declared_world_ref`               | The world the manifest declared.                                                                           |
| `unsupported_reason`               | One of the narrowing reasons above, plus `world_retired`, `world_not_shipped_on_host`, `bridge_refused`.   |
| `successor_world_ref`              | Nullable; ref to a successor world the host does carry.                                                    |
| `repair_affordance_label`          | Human-legible repair label (e.g. "grant trust", "lift admin deny-list", "upgrade extension ABI").          |
| `audit_event_ref`                  | Ref to the audit event recording the denial.                                                               |

A host that silently drops an unsupported-world declaration is
non-conforming; admission denies with a typed reason and the
review surface renders the repair affordance.

### Capability narrowing

Capability narrowing resolves through the same four orthogonal
axes ADR-0012 already froze for permission narrowing, applied
per-world rather than per-scope:

1. **Admin policy packs** — `deny_list`, `permission_floor`,
   `egress_host_narrowing`, `emergency_disable`.
2. **Host context** — identity mode, trust state, execution-
   context posture, freshness floor.
3. **Capability-lifecycle row** — readiness / support / channel /
   freshness / client-scope axes from ADR-0011.
4. **Compatibility-bridge translation** — `honoured`,
   `translated`, or `refused` per world on a bridge binding.

A world's negotiated shape is the intersection of its declared
shape and every axis above. Widening is denied with the ADR-0012
`effective_permission_widening_attempted` reason.

### Compatibility-bridge scope

A `compatibility_bridge` host family exists so Aureline can adopt
an existing extension ecosystem without inventing a parallel
runtime for it. This seed reserves the shape of a bridge profile
without naming any particular foreign ecosystem; the successor
ADR names the profile set.

Reserved fields per `bridge_profile_row`:

| Field                              | Notes                                                                                                                       |
|------------------------------------|-----------------------------------------------------------------------------------------------------------------------------|
| `bridge_profile_id`                | Opaque profile id.                                                                                                          |
| `bridge_translation_class`         | One of `vs_code_compatible`, `language_server_protocol_only`, `debug_adapter_protocol_only`, `generic_helper_wrapper`, `bespoke_profile` (successor-ADR additive). |
| `honoured_worlds`                  | Worlds the bridge exposes to the foreign extension verbatim.                                                                |
| `translated_worlds`                | Worlds the bridge exposes under a translation layer (e.g. the bridge maps a foreign API onto `aureline:editor-read`).       |
| `refused_worlds`                   | Worlds the bridge does not expose; any manifest declaring one denies with `compatibility_bridge_required_not_present`.      |
| `budget_translation_rules`         | Optional typed rules mapping foreign budget vocabulary onto Aureline's budget classes.                                      |
| `schema_version`                   | Integer; additive-minor on additions.                                                                                       |

A bridge MAY NOT widen a world beyond Aureline's declared surface,
MAY NOT mint a new Aureline-namespace world identity, and MAY NOT
drop the ADR-0012 publisher-continuity, signature, or digest
invariants of the underlying artifact. The bridge's scope is
ecosystem translation, not trust substitution.

### Audit events reserved

The eventual extension-host crate emits a typed audit stream on
`extension_host_negotiation`. The seed reserves at minimum the
following audit-event ids; additional events are additive-minor.

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

Raw component bytes, raw module bytes, raw helper-binary invocation
bodies, raw bridge-shim payloads, raw process launch bodies, raw
signing-key material, and raw policy-bundle bytes never appear on
any of these events.

### Process-boundary constraints

1. Host-family rows, capability-world rows, host-negotiation
   packets, and world-admission decision records cross the RPC
   boundary as typed payloads (ADR-0004). Raw Wasm component
   bytes, raw core module bytes, raw external-host launch bodies,
   raw helper-binary invocation bodies, raw bridge-shim payloads,
   raw signing-key material, and raw policy-bundle bytes never
   cross.
2. The extension-host registry is authoritative in the host
   process; the eventual extension runtime reads capability-world
   rows only through the shared subscription envelope (ADR-0005)
   with authority class `derived_knowledge` and a declared
   freshness hint.
3. A credential handle a world requests projects under ADR-0007
   handle classes only; raw secret bytes never cross the world
   boundary.
4. A world bound to the shell / task surface resolves the
   ADR-0009 execution-context id before any capability fires; raw
   command lines and env bodies never cross the world boundary.
5. Remote-agent attach surfaces remote-scoped capability worlds
   whose `client_scope` (ADR-0011) includes `remote_agent`; a host
   surface renders a `client_scope_restricted_dependency` marker
   when the remote agent does not surface a world the host would.
6. Every world resolves the ADR-0018 `trust_state` packet before
   admitting any capability invocation; a handle issued under
   `trusted` MUST NOT survive downgrade to `restricted`.
7. Crash dumps and core files MUST NOT inherit unresolved
   host-negotiation packets; a crash discards the packet rather
   than persisting a partial set.
8. Mutation-journal entries, save manifests, claim manifests, and
   support bundles carry negotiation ids, world refs, budget
   class labels, and narrowing reason labels only; they MUST NOT
   embed raw component bytes, raw helper-binary invocation bodies,
   raw bridge-shim payloads, raw signing-key material, or raw
   policy-bundle bytes.

### Denial posture

Failures in host negotiation fail closed. Denial is typed,
visible, auditable, and repairable. The following denial reasons
are reserved by this seed in addition to the ADR-0012 set;
additional reasons are additive-minor.

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

Silent downgrade to a generic "not available" chip is forbidden;
every denial emits the corresponding audit event with its typed
reason and a repair affordance.

### Schema-of-record posture

The WIT files at `wit/aureline/*.wit` are the source of truth for
the capability-world interfaces; the host-negotiation schema at
`schemas/extensions/host_negotiation.schema.json` is the
cross-tool boundary every non-owning surface reads at this
milestone; and the capability-world registry at
`artifacts/extensions/capability_worlds.yaml` binds worlds to host
families, lifecycle rows, and permission-scope projections.
Rust types in an eventual extension-host crate will carry the same
record shape once the successor ADR lands.

Adding a world slug, a negotiation audit event, a narrowing
reason, a budget class, a denial reason, or an additive field is
additive-minor with a schema / world-vocabulary bump. Repurposing
any member is breaking and requires a new decision row. No
external IDL or code-generator toolchain at this milestone beyond
the WIT files themselves; this mirrors ADR 0004 through ADR 0012.

## Consequences

- **Reserved:** the capability-world naming scheme
  (`aureline:<slug>@<semver>`), five seeded world slugs
  (`editor-read`, `workspace-read`, `diff-apply-preview`,
  `terminal-observe`, `network-egress`), the host-family binding
  shape for all six ADR-0012 families, the host-negotiation packet
  field set, the capability-narrowing axis set, the compatibility-
  bridge profile shape, the budget-declaration vocabulary, the
  narrowing-reason vocabulary, the unsupported-world decision
  shape, the host-negotiation audit-event ids, and the denial-
  reason set for host-negotiation failures. Every later lane reads
  these rather than invent its own.
- **Reserved:** the initial WIT interface seeds at
  `wit/aureline/*.wit`. The seed interfaces are the normative
  contract; additive changes are minor, breaking changes require
  a new decision row.
- **Reserved:** process-boundary constraints. Raw Wasm component
  bytes, raw core module bytes, raw external-host launch bodies,
  raw helper-binary invocation bodies, raw bridge-shim payloads,
  raw signing-key material, and raw policy-bundle bytes never
  cross RPC. Host-negotiation records cross as typed payloads.
- **Reserved:** the schema-of-record posture. The WIT files are
  the source of truth for interfaces; the JSON Schema export at
  `schemas/extensions/host_negotiation.schema.json` is the cross-
  tool boundary at this milestone; the registry at
  `artifacts/extensions/capability_worlds.yaml` binds rows; no
  external IDL at this milestone beyond WIT.
- **Permitted:** later additive-minor additions to any enumerated
  set (new world slugs, new budget classes, new narrowing
  reasons, new audit events, new denial reasons) with a schema /
  world-vocabulary bump.
- **Permitted:** admin policy packs, trust-state narrowing,
  capability-lifecycle markers, and compatibility-bridge
  translation MAY each narrow a declared world further. None MAY
  widen.
- **Follow-up:** the successor ADR closes the open questions
  below (sandbox model per host family, the broader world
  library, the compatibility-bridge profile set, the helper-
  binary launch contract, the remote-side component attach
  contract, the SDK bindgen target, world-retirement chronology,
  and budget quantitative ceilings) and promotes this seed's
  `Proposed` status to `Accepted`.
- **Follow-up:** the install-review sheet, the permission
  inspector, the publisher-continuity packet, the mirror-
  continuity row, and the mutation-journal entry each project
  the negotiated-capability-worlds set alongside ADR-0012's
  declared-vs-effective permission diff. A surface that hides the
  negotiated world set denies with the ADR-0012
  `review_disclosure_incomplete` reason.
- **Follow-up:** ADR-0012's `host_contract_identity_ref` and
  `compatibility_bridge_notes` fields now quote the capability-
  world registry and the bridge-profile row respectively without
  re-freezing ADR-0012 vocabulary.
- **Ratifies:** the ADR-0001 identity-mode envelope gates
  `managed_admin_surface`-touching worlds; ADR-0004 typed
  payloads carry negotiation packets; ADR-0005 subscription
  authority `derived_knowledge` carries capability-world views;
  ADR-0006 VFS path identity applies to filesystem-touching
  worlds; ADR-0007 projection modes gate
  `secret_handle_use`-touching worlds; ADR-0008 admin-policy
  narrowing is the orthogonal ceiling; ADR-0009 execution-
  context resolution precedes shell / task-touching worlds;
  ADR-0010 grant-resolution reasons are quoted by provider-linked
  worlds; ADR-0011 capability-lifecycle and dependency-marker
  vocabulary projects per-world; ADR-0012 manifest, permission-
  scope, and denial vocabulary already frozen; ADR-0015 native-
  reserved embedded-surface boundary carries remote-side
  component worlds; ADR-0016 command-dispatch boundary routes
  terminal / task worlds; ADR-0018 trust-decision packet gates
  every world admission.

## Alternatives considered

- **Defer capability-world vocabulary until the runtime lands.**
  Rejected: ADR-0012's `host_contract_identity_ref` and
  `compatibility_bridge_notes` would stay free-form; the eventual
  install-review sheet would have to render opaque strings rather
  than named worlds; the SDK bindgen target would have no
  reserved identity scheme; the ADR-0012
  `host_contract_family_unsupported_on_target` and
  `compatibility_bridge_required_not_present` denial reasons
  would be unresolvable to a specific world. Per
  `D-0024.default_if_unresolved`, the extension-runtime lane
  freezes until this seed lands.
- **Pick a single host family (Wasm component model only) and
  drop the others.** Rejected: `.t2/docs/...Architecture...md`
  already commits the product to a Wasm-first host with external-
  host escape hatches, compatibility-bridge adoption, and remote-
  side components. Dropping any family forces that work into
  one-off per-lane fields.
- **Let each extension declare a free-form host ABI string.**
  Rejected: not machine-readable; host-ABI-range negotiation,
  unsupported-world denial, and compatibility-bridge translation
  cannot be computed against free-form strings.
- **One big "capabilities" enum rather than named worlds.**
  Rejected: conflates scope (a permission unit) with capability
  (a typed interface). ADR-0012 already owns the scope vocabulary;
  this seed owns the typed-interface vocabulary the scopes
  project onto.
- **External IDL + codegen (Protobuf, Cap'n Proto, Smithy).**
  Rejected: same reasoning as ADR 0004 through ADR 0012 — no
  second-language consumer yet beyond the WIT boundary itself,
  and WIT is the host-family-native IDL for `wasm_component_model`.
  The JSON Schema export reserves a clean integration point for
  the negotiation packet.
- **Define one universal world ("aureline:host@*") and let the
  manifest declare sub-capabilities via permission scopes only.**
  Rejected: collapses the typed interface into a permission scope;
  bridge translation, unsupported-world denial, and per-world
  lifecycle markers cannot be expressed cleanly. The seed keeps
  scopes orthogonal to worlds so ADR-0012 and this ADR each keep
  one clear remit.

The `D-0024` `freeze_lane` default-if-unresolved posture would
block the install-review, permission-inspector, SDK-seed, and
mirror-continuity lanes from closing the runtime ADR at the
first-beta milestone until a successor ADR lands. Accepting the
seed's `Proposed` status now — with its reserved vocabulary,
typed WIT seeds, and invariants — avoids that freeze by giving
the successor ADR concrete interfaces and records to compose
against.

## Open questions

These questions MUST be answered by the successor ADR before
this seed is promoted to `Accepted`. They are listed so no later
lane assumes a resolution silently.

1. **Sandbox model per host family.** What are the concrete
   sandbox guarantees for `wasm_component_model`,
   `wasm_core_module`, `external_host_process`, and
   `helper_binary` (memory isolation, time-bounded execution,
   network egress enforcement, filesystem path scoping, syscall
   surface, kill-switch contract)? The per-family answer binds
   the budget ceilings the successor ADR publishes.
2. **Broader world library.** Which additional worlds land in
   the successor ADR (candidates: `debug-session`,
   `notebook-kernel`, `ai-apply`, `extension-dependency`,
   `identity-gate`, `provider-connect`), and how does each
   project onto the ADR-0012 permission-scope vocabulary?
3. **Compatibility-bridge profile set.** Which concrete
   `bridge_translation_class` rows ship at the first beta
   (candidates: `vs_code_compatible`,
   `language_server_protocol_only`,
   `debug_adapter_protocol_only`, `generic_helper_wrapper`),
   and which declared permissions does each honour vs translate
   vs refuse?
4. **Helper-binary launch contract.** What is the typed launch
   contract for `helper_binary` hosts (kill-switch cadence, stdio
   isolation, short-lived window semantics, credential-handle
   projection, signature re-verification), and how is the
   helper's lifecycle row projected?
5. **Remote-side component attach contract.** What is the typed
   attach contract for `remote_side_component` hosts (remote-
   agent trust binding, client-scope projection, re-admission on
   remote trust rotation), and how does the negotiation packet
   inherit the remote's identity-mode envelope?
6. **SDK bindgen target.** How does the stable-surface SDK bind
   to each WIT seed (binding kind per ADR-0011 `sdk_or_api`
   client scope), and what stability-window labels apply to the
   world-semver axis vs the SDK-version axis?
7. **World-retirement chronology.** What is the retirement
   workflow for a named world (deprecation window, successor
   world binding, install-review repair affordance, registry-
   index projection), and how does it interact with ADR-0012's
   `extension_manifest_schema_version_bumped` audit event?
8. **Budget quantitative ceilings.** What are the concrete time,
   memory, egress, and subscription-fanout ceilings per budget
   class per identity mode and per trust state?
9. **Capability-inheritance across worlds.** When an extension's
   manifest declares `capability_inherit` against another
   extension, do the inherited capabilities project onto one
   shared negotiation packet or a nested packet? The ADR-0011
   dependency-marker rule applies; the nesting shape is open.
10. **Core-module world binding.** `wasm_core_module` hosts do
    not carry WIT worlds natively. How does a core module's
    `host_imports_ref` project onto the named world set (e.g.
    through a curated import-signature bundle, or a host-side
    adapter)?

Each question blocks the `Proposed` -> `Accepted` transition and
is tracked in the `decision_history` of `D-0024`.

## Source anchors

- `.t2/docs/Aureline_PRD.md` — "Users and admins can see the
  effective permission set after dependency resolution, not only
  the top-level extension manifest."
- `.t2/docs/Aureline_Technical_Architecture_Document.md` —
  "Wasm-first contracts map to WIT/component-model worlds."
- `.t2/docs/Aureline_Technical_Architecture_Document.md` —
  "Extension host <-> platform SDK | manifest SDK range,
  permission vocabulary version, WIT ABI version."
- `.t2/docs/Aureline_Technical_Architecture_Document.md` —
  "AD-008 | Extension runtime | Wasm capability sandbox +
  isolated external hosts."
- `.t2/docs/Aureline_Technical_Architecture_Document.md` —
  "canonical extension ID, version, artifact digest, signature
  fingerprint, declared-permissions digest, compatible host ABI,
  and source-registry class."
- `.t2/docs/Aureline_Technical_Design_Document.md` — "Install /
  update review sheet | runtime origin, bridge/native state,
  permission diff, helper/executable disclosures, trust-mode
  behavior, rollback path."
- `.t2/docs/Aureline_Milestones_Document.md` — "Capability
  manifest and effective permission view ... effective
  permission set after dependency resolution, runtime
  host/budget class, lifecycle state."

## Linked artifacts

- Decision register row:
  `artifacts/governance/decision_index.yaml#D-0024`
- RFC: none (the open-question option space runs down in the
  successor ADR).
- WIT capability-world seeds:
  - `wit/aureline/editor-read.wit`
  - `wit/aureline/workspace-read.wit`
  - `wit/aureline/diff-apply-preview.wit`
  - `wit/aureline/terminal-observe.wit`
  - `wit/aureline/network-egress.wit`
  - `wit/aureline/aureline.wit` (root package)
- Narrative companion:
  `docs/extensions/wit_host_contract_seed.md`
- Capability-world registry:
  `artifacts/extensions/capability_worlds.yaml`
- Host-negotiation boundary schema:
  `schemas/extensions/host_negotiation.schema.json`
- Worked host-negotiation fixture:
  `fixtures/extensions/host_negotiation_examples/declared_vs_negotiated_example.yaml`
- Extension manifest and effective-permission vocabulary this
  ADR binds host-family rows and capability-world rows to:
  `docs/adr/0012-extension-manifest-permission-publisher-policy.md`
- Trust-decision packet every admission resolves:
  `docs/adr/0018-workspace-trust-and-restricted-mode.md`
- Capability-lifecycle vocabulary each world row carries:
  `docs/adr/0011-capability-lifecycle-and-dependency-markers.md`
- Identity-mode envelope the negotiation packet inherits:
  `docs/adr/0001-identity-modes.md`
- RPC typed-payload rules for the negotiation packet:
  `docs/adr/0004-rpc-transport-and-schema-toolchain.md`
- Subscription envelope capability-world views ride:
  `docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`
- VFS path-identity rules filesystem-touching worlds bind to:
  `docs/adr/0006-vfs-save-cache-identity.md`
- Secret-broker handle classes `secret_handle_use`-touching
  worlds cite:
  `docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`
- Settings resolver the `workspace_settings_*`-touching worlds
  read through:
  `docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`
- Execution-context model shell / task-touching worlds bind to:
  `docs/adr/0009-execution-context-and-scope.md`
- Connected-provider vocabulary provider-linked worlds quote:
  `docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`
- Affected lanes:
  `governance_lane:compatibility_ecosystem_review`,
  `governance_lane:security_trust_review`,
  `governance_lane:architecture_council`,
  `governance_lane:docs_public_truth`,
  `governance_lane:support_export`,
  `governance_lane:governance_packets`.

## Supersession history

First acceptance (as a seed at `Status: Proposed`). A successor
ADR promotes this seed to `Accepted` once the open questions are
closed, and records the supersession in this section without
rewriting the body above.
