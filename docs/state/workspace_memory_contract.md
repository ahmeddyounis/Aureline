# Workspace memory, continuity, and handoff artifact contract

This document freezes the product-wide vocabulary for workspace memory
and continuity state. It gives restore, resume, sync, support export,
browser/mobile handoff, work-item handoff, and incident handoff one
shared answer to four questions:

- what class of state is this;
- where may it travel by default;
- what label must users see when the state affects expectations; and
- what live authority must stay local-only or be replaced by a safe
  placeholder.

The machine-readable boundary lives at:

- [`/schemas/state/state_class.schema.json`](../../schemas/state/state_class.schema.json)

The companion matrix lives at:

- [`/artifacts/state/workspace_memory_matrix.yaml`](../../artifacts/state/workspace_memory_matrix.yaml)

This contract composes with:

- [`/docs/state/profile_and_state_map.md`](./profile_and_state_map.md)
- [`/docs/state/migration_and_restore_playbook.md`](./migration_and_restore_playbook.md)
- [`/docs/state/state_object_inventory.md`](./state_object_inventory.md)
- [`/docs/ai/memory_and_reconciliation_contract.md`](../ai/memory_and_reconciliation_contract.md)
- [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
- [`/docs/support/object_handoff_packet.md`](../support/object_handoff_packet.md)
- [`/docs/work_items/work_item_contract.md`](../work_items/work_item_contract.md)
- [`/docs/ops/incident_workspace_contract.md`](../ops/incident_workspace_contract.md)
- [`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md)
- [`/docs/vcs/git_state_and_worktree_contract.md`](../vcs/git_state_and_worktree_contract.md)
- [`/docs/runtime/execution_context_vocabulary.md`](../runtime/execution_context_vocabulary.md)

Where this document disagrees with those specialized contracts, the
specialized contract wins and this document plus its schema and matrix
MUST be updated in the same change. Where a downstream restore,
export, sync, support, companion, work-item, or incident surface mints
a local replacement for the vocabulary below, this document wins and
the surface is non-conforming.

## Scope

Frozen here:

- five workspace-memory state classes:
  `restorable_local_state`, `user_portable_state`,
  `workspace_shared_state`, `machine_local_ephemeral_state`, and
  `handoff_artifact_state`;
- one default-portability vocabulary every state-class row uses;
- one visible-state-label vocabulary for `local_only`, `synced`,
  `imported`, `provider_linked`, `stale`, and `machine_local` states;
- one continuity-flow vocabulary for resume, restore, support export,
  browser handoff, mobile handoff, work-item handoff, and incident
  handoff;
- one resume-context requirement set for branch/worktree, remote/local
  boundary, execution profile, pending recovery state, and missing
  dependency placeholder behavior; and
- one excluded-live-authority rule for secrets, live tokens, delegated
  approvals, machine-unique handles, and similar authority-bearing
  material.

Out of scope:

- sync engines, cross-device transfer execution, provider adapters, or
  support upload;
- final UI copy;
- concrete store layout beyond existing state-object inventory rows;
- changing the specialized schemas this contract cites.

## 1. State classes

Every restore, export, handoff, or continuity packet that carries
workspace memory MUST classify each carried row into exactly one
`state_class_id`.

| State class | Meaning | Examples | Default portability |
|---|---|---|---|
| `restorable_local_state` | Durable local state held so the user can resume work on the same installation without silent loss. | session restore manifest, dirty-buffer recovery refs, local history checkpoint refs, terminal transcript metadata, pane layout snapshot, recent-work row metadata | `local_restore_only` |
| `user_portable_state` | User-owned preferences or notes that may move across machines through profile export/import or opt-in sync. | settings, keybindings, snippets, themes, user-pinned memory, explicit AI preset choices, portable profile metadata | `portable_by_default` |
| `workspace_shared_state` | Workspace-authored or repo-owned durable state intended to be reviewable with the project. | workspace manifest, worksets, task and launch config, workspace extension recommendations, workspace AI policy refs, template lineage metadata | `workspace_shared_by_default` |
| `machine_local_ephemeral_state` | Machine-bound, session-bound, or regenerable state that must not be used as portable truth. | render caches, index shards, in-flight turn scratch, temporary command overrides, display bindings, local absolute paths, PTY handles, debug-session handles | `machine_local_only` |
| `handoff_artifact_state` | A deliberately assembled packet that transfers selected context to a support, browser, mobile, work-item, incident, or external-provider flow. | support bundle manifest row, browser handoff packet, offline work-item packet, incident evidence handoff bundle, object issue handoff packet, mobile companion triage snapshot | `explicit_handoff_only` |

Rules:

1. A row MUST NOT appear in more than one state class. When a handoff
   packet references local restore state, the packet row is
   `handoff_artifact_state`; the referenced local body remains
   `restorable_local_state`.
2. `machine_local_ephemeral_state` can be summarized or referenced only
   when the receiving flow understands it as evidence or placeholder
   context. It MUST NOT become portable truth.
3. `handoff_artifact_state` is never implicit sync. It exists only
   after a user, policy, or governed workflow explicitly assembles it.
4. `workspace_shared_state` travels through the workspace/repository
   path, not through a user-profile sync lane.

## 2. Default portability postures

Every state-class record declares one `default_portability_posture`.

| Posture | Rule |
|---|---|
| `portable_by_default` | May travel in a user-initiated profile export/import or opt-in sync when the row carries no excluded material. |
| `portable_with_review` | May travel only after preview because translation, redaction, or policy narrowing may affect meaning. |
| `workspace_shared_by_default` | May travel with workspace files or workspace export. User-profile sync MUST NOT silently include it. |
| `local_restore_only` | May be used for same-install resume/restore. Exporting it outside the machine requires a governed handoff or support export rule. |
| `machine_local_only` | Never synced or portable by default. A machine-binding addendum or support packet may include metadata only when explicitly allowed. |
| `explicit_handoff_only` | Only assembled into a named handoff/support/export packet with destination, redaction, and omitted-class disclosure. |
| `excluded_from_portable_and_handoff` | Not portable and not handoffable through this contract. A separate governed contract is required before it can cross the boundary. |

The default posture is a floor. Admin policy may narrow it; a surface
may not widen it silently.

## 3. Visible state labels

Surfaces MUST show the following labels when the distinction changes a
reasonable user expectation about durability, sharing, authority,
freshness, or machine dependence:

| Label | Use when | Required behavior |
|---|---|---|
| `local_only` | no synced, provider, or portable copy exists | Do not imply another device or provider can recover it. |
| `synced` | a governed profile or managed sync lane has accepted the row | Show sync source/freshness and conflict posture when relevant. |
| `imported` | the row came from an import, support packet, offline packet, or handoff snapshot | Preserve source, import time, and whether there is a live refresh path. |
| `provider_linked` | the row is attached to a code host, issue tracker, CI, docs portal, identity provider, or managed control plane | Show actor/scope/freshness or link health; never collapse to generic connected state. |
| `stale` | the row is outside its freshness window, its authority changed, or its snapshot epoch no longer guarantees current truth | Keep it inspectable, but narrow or block unsafe actions. |
| `machine_local` | the row depends on this machine, display topology, local path, credential store, PTY/debug handle, or runtime process | Do not offer sync or cross-machine restore as if exact. |

Rules:

1. Labels compose. A row may be both `imported` and `stale`, or
   `provider_linked` and `local_only` when a local draft references a
   provider object but has not been accepted.
2. A surface MAY hide a label only when the label has no material
   effect on user expectations for that surface.
3. `synced` never implies provider authority. `provider_linked` never
   implies synced user state.

## 4. Excluded live authority

The following material is excluded from portable and handoff artifacts
under this contract:

- raw secrets, API keys, passwords, OAuth refresh tokens, bearer
  tokens, SSH private key material, mTLS key material, cookies, and
  vault plaintext;
- live delegated approvals, unspent approval tickets, browser session
  cookies, live callback tokens, and runtime grants;
- machine-unique handles such as OS keychain item ids, PTY handles,
  process ids, debug-session ids, file descriptors, display ids, and
  device-local trust-store handles;
- raw provider payloads, raw URLs, raw absolute paths, raw command
  lines, raw logs, raw terminal scrollback, raw model/tool payloads,
  or raw source content unless the receiving contract explicitly allows
  them under preview and redaction; and
- live remote-session bindings, live kernel handles, live browser
  runtime sessions, and mutable connected-provider authority.

Allowed substitutes:

- opaque refs;
- redacted summaries;
- hashes, counts, or time windows;
- source/freshness labels;
- by-reference managed ids when the destination is authorized to
  resolve them; and
- placeholders that ask the user to reconnect, reauthenticate, open in
  provider, install a dependency, or continue local-only.

A separately governed contract MAY admit one of these classes, but it
must name the authority, destination, redaction profile, expiry, and
approval path. This document alone never grants that authority.

## 5. Continuity flows

Each continuity flow declares what it can carry and what it must leave
behind.

| Flow | May carry | Must remain local-only or be represented by placeholder/evidence | Required disclosure |
|---|---|---|---|
| `resume` | `restorable_local_state`, selected `machine_local_ephemeral_state` metadata, and unresolved recovery refs for the same installation | raw secrets, live tokens, live approvals, process/PTY/debug/kernel handles, provider mutation authority | branch/worktree, target boundary, execution profile, recovery state, missing dependencies |
| `restore` | `user_portable_state`, `workspace_shared_state` after review, and `restorable_local_state` only as provenance or same-install restore | live authority and machine-unique handles | fidelity label, source, exclusions, placeholders, preserved prior artifacts |
| `support_export` | selected `handoff_artifact_state`, redacted `restorable_local_state` metadata, state-class inventory, logs/traces by policy | raw code/content, secrets, live authority, unreviewed machine-local bodies | redaction profile, included/excluded classes, storage mode, destination class |
| `browser_handoff` | a typed `handoff_artifact_state` packet with destination class, reason, object identity, return anchor, and selected draft/evidence refs | browser cookies, raw URLs from arbitrary callers, unspent approvals, local process handles | reason for leaving product scope, actor/scope, freshness, return path |
| `mobile_handoff` | read-only or narrowly scoped captured context, approval request refs, offline triage snapshots, selected summaries | raw lock-screen payloads, provider tokens, local path bodies, unadmitted mutations | capture time, drain state, destination, replay posture |
| `work_item_handoff` | work-item identity, requested transition, branch/worktree/review refs, validation summary, local draft/offline packet refs | raw tracker bodies, raw comments, raw provider tokens, assumed provider acceptance | provider authority/freshness, local draft vs queued vs accepted, export route |
| `incident_handoff` | incident id, timeline refs, runbook/action refs, evidence bundle refs, target/boundary/freshness summaries | raw logs by default, approval bodies, live console/browser sessions, secret-bearing payloads | actor role, destination class, mutating-step authority, redaction and retention posture |

Rules:

1. A flow MUST name its `continuity_flow_id` and every carried
   `state_class_id`.
2. A flow MUST list intentional exclusions. Omitting live authority is
   expected; hiding the omission is non-conforming.
3. A handoff flow MUST disclose whether the packet is `saved_local_only`,
   `ready_for_browser_handoff`, `captured_pending_drain`,
   `attached_by_reference`, `provider_accepted`, or
   `provider_rejected`.
4. Imported handoff artifacts remain `imported` until the receiving
   authority refreshes or accepts them.

## 6. Resume context requirements

Any resume claim stronger than "open clean" MUST carry these context
blocks or explain why the block is unavailable.

### Branch and worktree context

Required fields:

- workspace identity ref;
- repository/root identity refs;
- active worktree ref and worktree role;
- current branch, detached state, or unknown-state marker;
- base/head refs as opaque ids where available;
- dirty-state summary and pending operation summary;
- sparse/partial clone or workset scope when applicable; and
- provider overlay link and freshness when provider state affects the
  resume claim.

Resume MUST NOT silently switch worktrees, infer a branch from provider
state, or treat a stale provider overlay as local Git truth.

### Remote/local boundary context

Required fields:

- target class and route class;
- host-boundary cue stack;
- reachability/freshness state;
- remote authority or managed-control-plane posture where applicable;
- local fallback or direct attach path when available; and
- stale/unavailable reason when a boundary cannot be reattached.

Resume MUST preserve local editing continuity where possible while
rendering remote, managed, browser, and provider boundaries honestly.

### Execution profile context

Required fields:

- execution profile ref;
- target environment or devcontainer/profile ref;
- toolchain/runtime fingerprint refs;
- policy epoch and workspace-trust state;
- environment-variable and credential-handle posture; and
- last successful run/debug/test context as evidence, not automatic
  authority.

Resume MUST NOT rerun commands, restart debug sessions, or restore live
kernels as authority without an explicit user action or governed
reconnect path.

### Pending recovery state

Required fields:

- recovery journal refs for dirty buffers and local history;
- checkpoint refs and restore-provenance refs;
- deferred intent, publish-later, or offline handoff refs;
- repair/recovery ladder rung where recovery is not complete;
- blocked or manual-review failure states; and
- intentional exclusions for secrets/live handles.

Resume MUST create or cite an attributable checkpoint when it replays
content or state.

### Missing dependency placeholders

When an extension, remote target, provider, kernel, runtime, credential
store, docs pack, or display topology is missing, resume MUST:

1. preserve the surrounding layout or object identity when safe;
2. render a placeholder with the missing dependency class and source;
3. keep local-only alternatives visible where they exist;
4. offer only safe actions: close/remove, install, reconnect,
   reauthenticate, open in provider, export diagnostics, or continue
   local-only;
5. avoid rerunning commands or spending approvals automatically; and
6. label the result `layout_only`, `evidence_only`, or
   `manual_review` rather than `exact`.

## 7. Matrix and schema rules

The YAML matrix is the seed machine-readable registry. It contains:

- closed vocabularies;
- one row per state class;
- one row per continuity flow;
- resume-context blocks; and
- exclusion rules.

The JSON Schema validates these record shapes for tools that convert
YAML to JSON. Adding a new state class, posture, flow, visible label,
handoff delivery state, or excluded-material class is additive only
when the value has a new meaning and the schema version is bumped.
Repurposing a value is breaking and requires a governance decision.

## 8. Conformance checklist

A restore, export, or handoff flow conforms when it can answer:

- Which `state_class_id` is carried?
- Which `default_portability_posture` applied before policy narrowing?
- Which visible labels materially affect user expectations?
- Which live-authority classes were intentionally excluded?
- Is this exact, compatible, layout-only, evidence-only, local-only,
  pending drain, provider-accepted, or provider-rejected?
- What branch/worktree, boundary, execution profile, and recovery refs
  explain resume?
- What safe placeholder is shown when the exact dependency is missing?

If any answer requires new vocabulary, the matrix and schema are
extended first.
