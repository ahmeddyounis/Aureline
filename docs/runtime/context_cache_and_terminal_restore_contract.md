# Execution-context cache and terminal-restore contract

This contract freezes two derived-state artifact families before the
resolver cache, terminal restore store, Project Doctor integration, or
support-export packager are implemented:

- execution-context cache entries and their compare / reset records;
- terminal restore metadata and its compare / reset records.

Both families exist to explain runtime decisions. They do not own user
settings, workspace manifests, trust approvals, policy authority,
credentials, terminal preferences, local history, dirty-buffer journals,
or support evidence. A reset of either family is scoped, previewed, and
independent of the other unless a reviewed repair explicitly selects
both.

If this contract disagrees with the PRD, Technical Architecture
Document, Technical Design Document, UI / UX Spec, or Design System
Style Guide, those source documents win and this contract plus its
schemas update in the same change.

## Companion Artifacts

- [`/schemas/runtime/execution_context_cache_entry.schema.json`](../../schemas/runtime/execution_context_cache_entry.schema.json)
  - boundary schema for `execution_context_cache_entry_record`,
  `execution_context_cache_comparison_record`,
  `execution_context_cache_reset_preview_record`, and
  `execution_context_cache_audit_event_record`.
- [`/schemas/runtime/terminal_restore_metadata.schema.json`](../../schemas/runtime/terminal_restore_metadata.schema.json)
  - boundary schema for `terminal_restore_metadata_record`,
  `terminal_restore_comparison_record`,
  `terminal_restore_reset_preview_record`, and
  `terminal_restore_audit_event_record`.
- [`/fixtures/runtime/context_cache_cases/`](../../fixtures/runtime/context_cache_cases/)
  - worked fixtures for wrong interpreter, wrong shell, wrong target,
  stale environment, and blocked restore flows.
- [`/schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json)
  and
  [`/docs/runtime/execution_context_vocabulary.md`](./execution_context_vocabulary.md)
  - canonical execution-context vocabulary. The cache schema reuses
  target, toolchain, surface, trust, policy, activation, provenance,
  and degraded-state terms from this family.
- [`/docs/runtime/environment_capsule_contract.md`](./environment_capsule_contract.md)
  - environment-capsule and environment-diff contract. Cache entries
  quote capsule id, hash, drift state, and compatibility-fingerprint
  refs; they do not embed capsule bodies.
- [`/docs/execution/context_inspector_packet.md`](../execution/context_inspector_packet.md)
  - snapshot and diff packet. Cache entries may cite snapshot refs
  when Project Doctor, support export, or an inspector needs to compare
  cached and observed context truth.
- [`/docs/execution/terminal_truth_contract.md`](../execution/terminal_truth_contract.md)
  and
  [`/schemas/terminal/session_restore_metadata.schema.json`](../../schemas/terminal/session_restore_metadata.schema.json)
  - terminal protocol, clipboard, and restore vocabulary. The runtime
  terminal-restore schema narrows that seed into resettable metadata
  artifacts without minting command replay authority.
- [`/docs/runtime/storage_classes_and_gc.md`](./storage_classes_and_gc.md)
  and
  [`/docs/state/config_and_state_path_map.md`](../state/config_and_state_path_map.md)
  - storage-class and path semantics. Execution-context cache is
  disposable derived state; terminal restore metadata is user-owned
  recovery state.
- [`/docs/support/project_doctor_packet.md`](../support/project_doctor_packet.md)
  and
  [`/docs/support/repair_transaction_contract.md`](../support/repair_transaction_contract.md)
  - Doctor findings and repair transactions. Doctor may recommend
  cache reset or terminal restore reset, but the mutation still goes
  through a preview.

Normative source anchors projected here include the PRD requirements
for context caches keyed on workspace, target, profile, policy version,
and toolchain fingerprint; terminal restore metadata that never
auto-reruns commands; Project Doctor diagnosis of wrong interpreters,
wrong targets, environment drift, and cache incompatibility; and
storage-class rules separating disposable derived state from
user-owned recovery state.

## Scope

Frozen at this revision:

- the execution-context cache entry as a derived artifact whose key is
  the tuple of workspace, target, profile, policy, environment capsule,
  and toolchain fingerprints;
- a comparison record that explains stale, wrong-target, wrong-
  toolchain, schema-drift, and redaction-limited cache decisions;
- a reset-preview record that clears only selected derived cache rows
  and preserves user-authored durable state, terminal restore metadata,
  policy/trust state, and credentials;
- terminal restore metadata as bounded recovery metadata with explicit
  retention, redaction, shell / target / cwd hints, and restore
  authority state;
- a terminal restore comparison record for target drift, shell mismatch,
  stale execution context, redaction limits, and policy blockers;
- a terminal restore reset-preview record that clears selected restore
  metadata without changing terminal preferences, local history,
  dirty-buffer journals, execution authority, or support evidence; and
- audit-event vocabularies for writes, compares, stale marking, reset
  preview / apply, restore blocking, fresh-session minting, and export.

Out of scope:

- implementing a cache engine, PTY broker, terminal multiplexer,
  scrollback store, shell-integration injector, Project Doctor probe
  runner, or repair transaction executor;
- final UI copy, button placement, or CLI syntax for reset and compare;
- storing raw terminal transcripts, raw command lines, raw environment
  bodies, raw paths, raw URLs, or raw secret bytes in these artifacts;
  and
- replay, automation rerun, or privileged action restoration.

## Execution-context Cache Entry

An `execution_context_cache_entry_record` is **derived state**. It is a
memoized explanation of a resolver decision, not an authority record.
It may accelerate future resolution, but it may not silently widen
target, trust, policy, or toolchain authority.

Every entry carries:

- `cache_key` - opaque ref and hash over the normalized key material;
- `derived_input_fingerprints` - workspace id plus target, profile,
  policy, environment capsule, and toolchain fingerprints;
- `cache_provenance` - resolver version, compute time, execution-
  context/provenance/snapshot refs where available, evidence refs,
  Doctor finding refs, and repair candidate refs;
- `cache_status` - current, stale, rejected, corrupted, blocked, or
  rebuild-pending state with a typed stale reason;
- `reset_contract` - scope and safety class for reset, with invariant
  booleans that preserve user-authored state and execution authority;
  and
- `export_safe_summary` - class labels, hashes, opaque refs, and counts
  only.

The cache key MUST be derived from all of:

| Key layer | Minimum fields |
|---|---|
| Workspace | `workspace_id` and scope ref |
| Target | `target_class`, canonical target id, requested target ref, materialized instance ref, target identity hash |
| Profile | active profile id, profile revision ref, profile-layer digest |
| Policy | identity mode, trust state, policy epoch, policy bundle ref, approval-ticket ref when present |
| Environment capsule | capsule id, capsule hash, drift state, capsule schema version, compatibility-fingerprint ref |
| Toolchain | toolchain class/id, resolved version, activation strategy, executable digest, wrapper-chain digest |

If any layer is unknown, redaction-limited, stale, or corrupted, the
entry may still be written for evidence, but its `context_cache_state`
MUST narrow to a non-current value and consumers MUST NOT treat it as a
launch authority.

### Cache States

`hit_current` is admitted only when every fingerprint still matches the
latest authoritative input and the compatible version range admits the
running application.

`stale_safe_to_compare` means the entry may appear in inspectors,
support exports, and Doctor explanations, but it may not launch work
without a re-resolve.

`rejected_wrong_target` and `rejected_wrong_toolchain` fail closed:
they may point at repair or selection actions, but they do not silently
fall back to another target or executable.

`corrupted_reset_candidate` means support and Doctor may offer a reset
preview. It does not authorize deleting broader state roots.

### Comparison

An `execution_context_cache_comparison_record` explains why a cached
entry differs from the observed context. It is the object Project
Doctor, support export, CLI, and inspectors share when they need to say
"wrong interpreter", "wrong target", "stale capsule", "policy epoch
changed", or "schema drift" without ad hoc prose.

Each mismatch names:

- the layer (`target`, `profile`, `policy`, `environment_capsule`,
  `toolchain`, `schema`, `aureline_version`, or `provenance`);
- severity (`changed_rebuild_allowed`, `changed_compare_only`,
  `changed_block_until_user_selects`, `redaction_limited`, or
  `unknown_requires_probe`);
- cached and observed refs; and
- a short reviewable explanation.

The recommended action is closed: use cache, rebuild, compare only,
clear a single entry, refresh the environment capsule, select target,
run Project Doctor, or block execution.

## Cache Reset

A cache reset is a previewed mutation over **disposable derived state**.
It must be narrow enough to name selected cache entry refs and broad
enough to be useful only when the preview says so.

Cache reset MUST:

1. Preserve user-authored settings, workspace manifests, profile
   library entries, local history, dirty-buffer journals, terminal
   restore metadata, terminal scrollback, trust approvals, policy
   bundles, credential handles, and support evidence.
2. Declare `will_delete_user_authored_state = false` and
   `will_change_execution_authority = false`.
3. Rebuild only from authoritative resolver inputs, a recomputed
   environment capsule, a target probe, a policy refresh, or an
   explicit user selection.
4. Emit an audit event for preview and apply.
5. Leave affected launch surfaces visibly stale, rebuilding, or blocked
   until a fresh resolver output is available.

Cache reset MUST NOT:

- delete broad state roots blindly;
- reset profile settings, terminal preferences, or trust state;
- silently choose a new target or executable after clearing a stale
  entry;
- delete support evidence that referenced the old entry; or
- use cache deletion as proof that a toolchain, policy decision, or
  external side effect happened.

## Terminal Restore Metadata

A `terminal_restore_metadata_record` is **user-owned recovery state** or
session-only evidence, depending on `storage_scope_class`. It protects
continuity after restart, crash recovery, reconnect, or import, but it
does not own live PTY authority.

Every record carries:

- workspace/profile/session refs and storage scope;
- retention timestamp or null when retention is governed by a
  class-specific rule;
- terminal restore level and restore decision;
- bounded scrollback metadata with hash refs and counts;
- shell / target / cwd / title hints as refs;
- restore authority invariants;
- typed blockers; and
- a reset contract and export-safe summary.

Restored metadata may restore:

- pane placement, title, cwd hints, shell-family hints, target labels,
  prompt / command marker counts, transcript hashes, and bounded
  rendered transcript references when policy allows;
- inspect-only evidence for support and Doctor; and
- a "start fresh session" handoff when the user explicitly asks to
  regain write authority.

Restored metadata MUST NOT:

- auto-rerun commands;
- imply replay of privileged actions, approval tickets, credential
  use, remote clipboard bridges, or shell input;
- claim live PTY continuity when the original PTY is gone;
- hide target, shell, policy, or context drift; or
- expose raw scrollback, commands, escape payloads, URLs, paths,
  environment bodies, clipboard bytes, or secrets by default.

## Restore Authority

The load-bearing restore fields are:

- `restore_authority_state_class`
- `auto_rerun_forbidden`
- `privileged_action_replay_forbidden`
- `input_admission_forbidden_until_fresh_session`
- `clipboard_bridge_admission_forbidden_until_fresh_session`
- `fresh_terminal_session_record_ref`
- `fresh_command_dispatch_descriptor_ref`

`auto_rerun_forbidden` and `privileged_action_replay_forbidden` are
always `true`.

For `restored_inspect_only`, `restored_with_hints_inspect_only`, and
`restore_blocked_no_authority`, input and clipboard bridge admission
are forbidden and fresh-session refs are null.

For `fresh_session_minted_with_explicit_user_initiation`, the record
MUST cite both a fresh terminal-session record and a fresh command-
dispatch descriptor. That fresh session has its own authority. It never
inherits authority from the restored metadata.

## Terminal Restore Reset

Terminal restore reset clears selected restore metadata. It is not a
generic cache reset and it is not a settings reset.

Terminal restore reset MUST:

1. Preserve terminal preferences, terminal profile settings, user
   settings, profile library entries, local history, dirty-buffer
   journals, trust approvals, policy bundles, credential handles, and
   support evidence.
2. Declare `will_delete_user_authored_state = false` and
   `will_change_execution_authority = false`.
3. State whether execution-context cache entries are preserved. The
   default is to preserve them; a combined repair must preview both
   artifact families explicitly.
4. Leave restored panes as unavailable or inspect-only until a fresh
   live session is explicitly created.

Terminal restore reset MUST NOT:

- clear all terminal preferences to "fix" a restore issue;
- delete local history or dirty-buffer journals;
- make a restored pane live;
- silently rebind a remote target; or
- delete support evidence referenced by a case, export, or incident.

## Corruption And Stale Handling

Corruption and stale state are diagnosis states, not broad-delete
instructions.

| Artifact | Stale or corruption signal | Required posture |
|---|---|---|
| Execution-context cache | policy epoch changed, target changed, capsule hash changed, toolchain fingerprint changed, schema drift, digest mismatch, parse failure | compare or mark stale; rebuild from authoritative inputs; offer scoped reset only after preview |
| Terminal restore metadata | target identity changed, missing root, shell-family mismatch, stale context cache ref, policy/trust denial, transcript hash mismatch, parse failure | render inspect-only or blocked; offer fresh session or scoped reset; never replay |
| Support export | redaction prevents full comparison | mark redaction-limited; do not request broad state deletion |
| Project Doctor | cache or restore metadata inconsistent | emit finding with evidence refs and repair candidates; mutation remains in repair preview |

Support and Project Doctor flows MUST be able to identify the affected
artifact family and selected refs. Asking the user to delete broad state
roots is non-conforming unless a separate export-before-reset flow names
every affected durable class.

## Export And Redaction

Both schemas default to metadata, hashes, opaque refs, class labels,
counts, and short reviewable sentences.

The following are forbidden by default:

- raw environment bodies;
- raw command lines;
- raw PTY bytes;
- raw scrollback bodies;
- raw escape-sequence payloads;
- raw clipboard bytes;
- raw URLs or file paths;
- raw credential or secret values; and
- raw user identifiers beyond opaque refs.

`broadened_capture` requires an approval-ticket ref and still does not
turn a restored transcript into replay authority.

## Worked Case Set

The fixture corpus covers:

| Case | Artifact | Purpose |
|---|---|---|
| `wrong_interpreter_cache_entry.json` | cache entry | Python task cache entry rejected because the current toolchain fingerprint no longer matches the workspace expectation |
| `wrong_shell_terminal_restore_metadata.json` | terminal restore metadata | restored pane records shell-family mismatch as inspect-only evidence |
| `wrong_target_cache_comparison.json` | cache comparison | cached local target differs from observed remote target and execution is blocked until the user selects target |
| `stale_env_cache_reset_preview.json` | cache reset preview | stale environment capsule can reset a single derived entry without deleting settings, history, or terminal restore metadata |
| `blocked_restore_flow_terminal_restore_metadata.json` | terminal restore metadata | policy/trust blocks restore; metadata stays inspect-only and cannot replay privileged action or command rerun |

These fixtures are pre-implementation artifacts. They freeze the
contract shape and reset semantics; they do not claim a live cache
engine, terminal host, or Doctor runner exists.
