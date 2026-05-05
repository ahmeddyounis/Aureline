# Authority class matrix and canonical writer rules

This document freezes the authority-class matrix Aureline uses to
answer two questions for every state-bearing surface:

1. **Who is the canonical writer for this state?**
2. **What are consumers allowed to project vs. mutate?**

It exists so the shell, editor, knowledge workers, execution
surfaces, policy/trust gates, and provider adapters cannot each
invent their own notion of truth or silently treat cached, derived,
or imported data as canonical authority.

The document is normative. If it disagrees with the PRD, Technical
Architecture Document, Technical Design Document, UI / UX Spec, or
Design System Style Guide, those source documents win and this
document must be updated in the same change.

## Companion artifacts

- [`/artifacts/runtime/authority_classes.yaml`](../../artifacts/runtime/authority_classes.yaml)
  — machine-readable authority-class matrix with canonical owners,
  allowed writers, consumer rules, staleness gates, and projection
  versus mutation boundaries.
- [`/fixtures/runtime/authority_examples/`](../../fixtures/runtime/authority_examples/)
  — short fixtures exercising canonical-writer rules, “overlay is
  evidence not authority” behavior, and stale-data mutation gating.
- [`/schemas/runtime/subscription_envelope.schema.json`](../../schemas/runtime/subscription_envelope.schema.json)
  — boundary schema that carries `authority_class`, `derivation_class`,
  `freshness`, `completeness`, and provenance (`producer_refs`) for
  every reactive snapshot/delta.
- [`/docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`](../adr/0005-subscription-envelope-and-invalidation-semantics.md)
  — the reactive subscription envelope and invalidation semantics this
  matrix binds to.

## Scope and authority

This contract does not introduce a second state engine or new runtime
enforcement layer. It freezes the *shared meaning* of authority so
future implementations do not drift.

In scope:

- the closed set of authority classes (`authority_class`);
- per-class canonical owner and allowed writers;
- projection vs mutation boundaries per class;
- staleness requirements and mutation gating rules; and
- consumer obligations for subscriptions, diagnostics, support bundles,
  and release evidence.

Out of scope:

- runtime enforcement code (guards, admission checks, or automatic
  remediation); and
- full state-store or replication implementation details.

## Shared vocabulary

These authority classes are closed by ADR-0005 and the runtime schemas
that re-export it. Adding a new class is a breaking change that MUST
update the ADR, the schemas, and the artifact matrix in the same
change.

- `workspace_vfs` — workspace roots, filesystem identity, watcher
  health, and save coordination.
- `buffer_editor` — in-memory editor buffers, selections, dirty state,
  undo groups, and recovery journals.
- `derived_knowledge` — indexes and computations derived from
  canonical workspace/buffer inputs (search, graph, language services).
- `execution` — task/debug/terminal/notebook/pipeline session truth and
  control surfaces.
- `policy_entitlement` — trust, identity, policy epochs, capability
  envelopes, quotas, and approval tickets.
- `provider_overlay` — provider and managed-service overlays
  (review/CI/companion/status feeds) that inform but do not overwrite
  local canonical truth.

These authority classes are distinct from the *storage* authority
classes in [`/docs/state/profile_and_state_map.md`](../state/profile_and_state_map.md).
Storage authority answers “who owns this durable blob on disk”; this
matrix answers “who is the canonical writer for this runtime truth
surface”.

## Authority-class matrix (summary)

| `authority_class` | Canonical owner | Allowed writers (summary) | Consumer rule (summary) |
|---|---|---|---|
| `workspace_vfs` | workspace runtime + VFS | VFS adapters + approved workspace commands | project; never invent root/path/trust authority |
| `buffer_editor` | editor engine | editor command graph + recovery replay | read by subscription; mutate through commands/journal |
| `derived_knowledge` | knowledge workers | producing workers from canonical inputs only | freshness/completeness/provenance mandatory |
| `execution` | execution runtimes | execution services only | control/state through typed subscriptions, never hidden polling |
| `policy_entitlement` | trust/policy/identity services | policy + auth services only | stale policy is visible before mutating actions remain enabled |
| `provider_overlay` | provider/managed adapters | provider adapters within scoped contracts | overlays remain labeled secondary; never silently replace canonical truth |

The detailed per-class contract is the machine-readable matrix at
[`/artifacts/runtime/authority_classes.yaml`](../../artifacts/runtime/authority_classes.yaml).

## Per-class contract

### `workspace_vfs` — Workspace / VFS authority

- **Canonical owner:** workspace runtime + VFS layer (including save
  coordination and filesystem identity).
- **Allowed writers:**
  - VFS adapters (watchers, remote mounts, provider-backed filesystem
    bridges) that emit identity and change events.
  - Workspace/VFS command handlers that mediate root add/remove, trust
    boundary transitions as *workspace-scoped facts*, and save/rename
    mutations.
  - Recovery replay that rehydrates durable workspace/VFS state and then
    republishes it through the same subscription graph.
- **Projection boundary:** consumers may cache projections (file trees,
  breadcrumbs, quick-open lists) but MUST NOT mint new path identity,
  canonical object identity, or trust state. Any computed alias MUST be
  represented as an alias of a canonical identity, not as a second
  mutable object.
- **Mutation boundary:** all filesystem mutations route through the
  canonical workspace/VFS command path (and its mutation journal
  entries); direct ad-hoc file IO from UI surfaces is non-conforming.
- **Staleness and gating:** if watcher health, save tokens, or identity
  lineage are degraded, the surface MUST label the degradation before
  offering mutations that assume exact VFS truth (rename/move/delete,
  destructive refactors, bulk apply).

### `buffer_editor` — Buffer / editor authority

- **Canonical owner:** editor engine (buffer store + undo/redo + dirty
  journal + view-state handles).
- **Allowed writers:**
  - editor command graph and input pipeline (keyboard, IME, structured
    edits, refactor apply);
  - recovery replay that replays the buffer journal into the same buffer
    engine on restore.
- **Projection boundary:** other surfaces may render projections (search
  highlights, review overlays, AI suggestions) but MUST treat them as
  overlays anchored onto buffer identities and revisions.
- **Mutation boundary:** every text mutation flows through an editor
  command that issues a journal entry and republishes a buffer delta;
  no consumer may “patch the buffer” out-of-band.
- **Staleness and gating:** any mutation-capable affordance computed
  from a cached buffer snapshot (for example, a quick fix derived from
  a stale diagnostics snapshot) MUST re-check against the live buffer
  before offering an “apply” action, or disable the affordance with a
  stale/derived explanation.

### `derived_knowledge` — Derived knowledge authority

- **Canonical owner:** search/index/graph/language workers.
- **Allowed writers:** producing workers MAY publish only derivations
  computed from named canonical inputs (workspace/VFS identity, buffer
  revisions, policy epoch), and MUST carry provenance (`producer_refs`
  plus input digests) for derived frames.
- **Projection boundary:** derived outputs are consumable as *evidence*
  and navigation aids (hits, references, diagnostics, graph edges), but
  they do not become the canonical source for mutating operations.
- **Mutation boundary:** derived lanes do not directly mutate workspace,
  buffers, or policy. Any “apply” action is a request routed to the
  canonical owner (`workspace_vfs`/`buffer_editor`) through commands.
- **Staleness and gating:** derived data MUST render freshness and
  completeness; stale/partial derived data must not keep mutation-
  capable affordances enabled unless a refresh/re-validate step succeeds
  first.

### `execution` — Execution authority

- **Canonical owner:** task/debug/terminal/notebook/pipeline runtimes.
- **Allowed writers:** execution services only (session hosts, runners,
  debug adapters, notebook kernel controllers) for their own session
  state.
- **Projection boundary:** consumers render execution state and artifacts
  via typed subscriptions, not by scraping logs, polling hidden side
  channels, or reinterpreting provider overlays as local truth.
- **Mutation boundary:** control actions (stop, retry, attach, send
  signal, queue run) route through execution commands that re-check
  current policy entitlements and publish their updates through
  subscriptions.
- **Staleness and gating:** if execution truth is stale (session ended,
  reconnect pending, policy revoked), control affordances MUST degrade
  to inspect-only with an explicit reason.

### `policy_entitlement` — Policy / entitlement authority

- **Canonical owner:** trust, identity, policy, and network services
  (local and/or managed, per deployment profile).
- **Allowed writers:** policy and auth services only. UI surfaces may
  request refreshes, but they do not manufacture approvals, grants, or
  capability envelopes.
- **Projection boundary:** consumers render effective policy snapshots
  as projections that always cite source and epoch. Policy overlays
  never silently widen entitlement scope.
- **Mutation boundary:** granting or changing entitlements requires an
  explicit policy action (sign-in, ticket acceptance, admin bundle
  update) that produces a new policy epoch and is auditable.
- **Staleness and gating:** stale policy/entitlement state MUST be
  visible *before* mutation-capable affordances remain enabled. Any
  action requiring policy approval or network egress either refreshes
  policy first or stays disabled with a clear stale/reauth explanation.

### `provider_overlay` — Provider / overlay authority

- **Canonical owner:** provider adapters (code host, CI, companion, and
  managed-service connectors) for provider-owned metadata only.
- **Allowed writers:** provider adapters within scoped contracts (typed
  RPC, identity-bound routes, policy-scoped credentials).
- **Projection boundary:** provider data is always labeled as an overlay
  with its own freshness and scope. Overlays may annotate local truth
  (for example, CI status on a commit) but do not overwrite it.
- **Mutation boundary:** provider mutations (comment, approve, merge,
  rerun) require the same command governance, preview/approval posture,
  and evidence capture as equivalent local commands. A provider overlay
  may not bypass local review, trust, or policy gates.
- **Staleness and gating:** when provider data is cached/stale/unavailable,
  surfaces keep local-safe work usable but downgrade provider control
  affordances to “refresh required” or “inspect-only”.

## Cross-cutting rules

### Subscription and projection discipline

1. Every consumer that renders state beyond its own canonical owner
   MUST do so through typed subscriptions that carry `authority_class`,
   `derivation_class`, `freshness`, `completeness`, and provenance
   (`producer_refs`).
2. A consumer MUST NOT replace canonical truth with a derived,
   imported, or provider overlay view without making the downgrade
   visible.
3. A consumer that cannot maintain sequence continuity (snapshot/delta)
   MUST request a resync or declare itself stale; it must not silently
   continue under false “current” assumptions.

### Command mutations and canonical writers

1. Mutations are issued as commands routed to the canonical owner for
   the target authority class. Consumers do not “write their own state”
   and then reconcile later.
2. Any mutation path that cannot consult the current canonical owner
   MUST fail closed or degrade to inspect-only unless an explicit waiver
   exists for that surface and action class.

### Diagnostics, support bundles, and release evidence

1. Diagnostics surfaces MUST carry authority and freshness labels for
   every derived or provider-owned claim they surface.
2. Support bundles and evidence packets MUST be able to trace any
   state-bearing surface back to:
   - a canonical `authority_class`;
   - a declared projection rule (authoritative vs derived); and
   - the staleness state at capture time.

## Change management

- Adding a new authority class is breaking: update ADR-0005, the
  re-exporting runtime schemas, this document, and
  [`authority_classes.yaml`](../../artifacts/runtime/authority_classes.yaml)
  in the same change.
- Repurposing an existing authority class is breaking and requires a
  new decision row; additive clarification belongs in this contract and
  the artifact matrix.
