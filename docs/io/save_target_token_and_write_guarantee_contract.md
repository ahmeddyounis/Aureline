# Save-target token and write-guarantee contract

This contract makes save honesty inspectable before the full VFS,
formatter, refactor, AI apply, remote-root, and cloud-backed root
implementations depend on it. A save surface must be able to answer:

- which path the user opened;
- which canonical object would be replaced;
- which generation token is being compared;
- which write guarantee actually holds; and
- which conflict, review, checkpoint, or export fields explain the
  outcome later.

Machine-readable companions:

- [`/schemas/io/save_target_token.schema.json`](../../schemas/io/save_target_token.schema.json)
  defines the one-packet save-target token, compare-before-write, and
  support/export boundary.
- [`/schemas/io/save_guarantee_class.schema.json`](../../schemas/io/save_guarantee_class.schema.json)
  defines the guarantee-class vocabulary, required UI copy, required
  export fields, checkpoint posture, and default actions.
- [`/fixtures/io/save_target_cases/`](../../fixtures/io/save_target_cases/)
  contains reviewable YAML fixtures for local atomic save, cloud-backed
  in-place review, remote revision conflict, read-only mount,
  policy-blocked write, and missing capability fallback.

This contract composes with:

- [ADR 0006](../adr/0006-vfs-save-cache-identity.md) for filesystem
  identity layers, root capability flags, watcher health, and the base
  save pipeline.
- [`/schemas/filesystem/save_target_token.schema.json`](../../schemas/filesystem/save_target_token.schema.json)
  for the existing cross-surface filesystem identity vocabulary.
- [`/docs/execution/save_participant_and_fix_safety_contract.md`](../execution/save_participant_and_fix_safety_contract.md)
  for save participants, fix safety, compare-before-write phase
  ordering, and review triggers.
- [`/docs/ux/editor_external_change_contract.md`](../ux/editor_external_change_contract.md)
  for stale buffers, diff/rebase choices, and external-change review.
- [`/docs/reliability/local_history_contract.md`](../reliability/local_history_contract.md)
  and [`/docs/workspace/mutation_lineage_model.md`](../workspace/mutation_lineage_model.md)
  for checkpoint, rollback, and mutation lineage vocabulary.

If this contract disagrees with the authoritative product design docs
or ADR 0006, those sources win and this packet updates in the same
change.

## Scope

Frozen here:

- the save-target token packet emitted before explicit save, autosave,
  save-as, formatter save, refactor apply, AI apply, and remote apply;
- save guarantee classes and the minimum UI/export copy each class
  must provide;
- compare-before-write behavior for local, remote, cloud-backed, and
  degraded roots;
- conflict and wrong-target prevention fields that must be diagnosable
  from one saved packet; and
- fixture coverage for the minimum save-target cases.

Out of scope:

- implementing root adapters;
- final platform-specific durability calls;
- final fsync strategy;
- concrete UI layout;
- implementing formatter/refactor/AI participants; and
- replacing the VFS identity vocabulary.

## 1. Save-target token packet

Every write-like surface emits one `save_target_token_packet` before it
offers a write affordance that can mutate durable state. The packet is
both a user-facing truth object and a support/export object.

| Field | Required purpose |
|---|---|
| `packet_id` | Stable diagnostic handle for one staged save attempt. |
| `save_target_token.token_id` | Stable token pinned to the opened object and staged content. |
| `presentation_path` | The URI and display label the user opened. This path is never silently replaced by the canonical URI. |
| `logical_workspace_identity` | Workspace, root, logical URI, trust state, and policy scope for the object being edited. |
| `canonical_object_identity` | The real object the next write would target, including root class, canonical URI, normalization form, identity token, fallback identity tokens, and alias set. |
| `strongest_generation_token` | The strongest equality-comparable generation or revision token available at token allocation time. If no trustworthy token exists, the token records `kind = unavailable`, `strength = missing`, and the save guarantee becomes `capability_uncertain`. |
| `permission_snapshot` | Writable/read-only state, policy-constrained state, mode summary, policy decision ref, mount graph hash, and observation time. |
| `write_mode` | The write primitive the token authorizes: `atomic_replace`, `in_place_write`, `conditional_remote_write`, `blocked`, or `capability_uncertain`. |
| `save_guarantee_class` | The user-visible guarantee class from Section 2. |
| `rollback_checkpoint_posture` | The real recovery posture: no mutation, recovery journal, local history checkpoint, workspace checkpoint group, provider revision checkpoint, metadata-only checkpoint, or unavailable/blocking. |
| `compare_before_write` | The current compare strategy, pre-write checks, observed token, and actions for match, mismatch, or uncertainty. |
| `support_export` | Redaction-safe packet family, diagnostic summary, wrong-target prevention fields, conflict diagnosis, and the fields that must survive export. |

Rules:

1. A surface that cannot produce a `save_target_token_packet` may not
   render a normal save affordance. It may render review, save-as,
   export patch, or open details.
2. The packet must name both `presentation_path` and
   `canonical_object_identity` whenever the root can provide canonical
   identity. If they differ, the UI and export both carry
   `canonical_differs_from_presentation = true`.
3. The token is invalidated by canonical target drift, root or mount
   graph change, permission change, policy epoch change, review-ticket
   expiry, generation-token mismatch, or watcher/capability downgrade
   that makes the previous guarantee too strong.
4. Save participants, formatters, refactors, AI apply, and CLI/headless
   paths reuse this packet. They may add participant or mutation refs,
   but they may not invent a separate save identity model.
5. Autosave uses the same token and compare-before-write rules as
   explicit save. Autosave may skip deferred participants, but it may
   not use a weaker write guarantee than explicit save would allow.
6. Support exports must include enough fields to diagnose a conflict or
   wrong-target prevention without reading raw file contents.

## 2. Save Guarantee Classes

The guarantee class is what the user, CLI, support packet, and mutation
journal see. It is not a hidden adapter implementation detail.

| Class | Write mode | Required UI copy | Required export fields |
|---|---|---|---|
| `atomic_replace_preferred` | `atomic_replace` | `Save will atomically replace this target after Aureline confirms it has not changed.` | presentation path, canonical target, generation token, permission snapshot, write mode, checkpoint ref, compare result, wrong-target flag |
| `in_place_write_with_review` | `in_place_write` | `This target cannot be atomically replaced. Review the change before Aureline writes in place.` | all atomic fields plus review ref, checkpoint posture, root class, capability reason, watcher health |
| `remote_conditional_write` | `conditional_remote_write` | `Save uses a remote revision check. If the target changed remotely, Aureline stops before overwriting it.` | all atomic fields plus provider target ref, remote revision token, precondition result, route/session ref, reconnect or stale-token reason |
| `read_only_mount` | `blocked` | `This location is read-only. Aureline will not write here.` | presentation path, canonical target, permission snapshot, read-only reason, offered alternatives, no-write audit ref |
| `policy_blocked` | `blocked` | `Policy blocks writing to this target. Aureline will not write until policy admits the action or you choose another target.` | presentation path, canonical target, policy scope, policy decision ref, actor/surface ref, offered alternatives, no-write audit ref |
| `capability_uncertain` | `capability_uncertain` or `blocked` | `Aureline cannot prove this write would hit the intended target safely. Refresh capability details or choose another target.` | presentation path, canonical target if known, missing capability reason, available fallback tokens, watcher health, offered alternatives, no-write audit ref |

Rules:

1. The guarantee class must be visible before bytes are written when it
   is weaker than `atomic_replace_preferred`.
2. A degraded guarantee cannot be hidden behind a green save state. The
   compact state may be short, but a keyboard-reachable detail surface
   must show the full class, reason, and next action.
3. The UI copy in the schema is required copy, not suggested flavor
   text. Surfaces may add context, but they may not weaken the promise
   or omit the degraded-write statement.
4. A class may only move to an equal or weaker class during a save
   attempt unless the token is refreshed and compare-before-write is
   run again.
5. `policy_blocked`, `read_only_mount`, and `capability_uncertain` do
   not issue writes. They may offer save-as, export patch, request
   approval, open details, retry capability refresh, or cancel.

## 3. Compare-Before-Write Rules

Compare-before-write is the correctness floor. Watchers make conflicts
visible sooner; they are not trusted as the final proof.

### 3.1 Shared Flow

1. The editor or participant chain stages content against a
   `save_target_token_packet`.
2. Immediately before durable write, the VFS re-resolves the canonical
   target and reads the strongest generation token available now.
3. The VFS compares object identity, generation token, permission
   snapshot, policy epoch, and root/mount capability fields required by
   the packet.
4. The save either writes under the declared guarantee, re-compares,
   opens diff/review, blocks, or offers save-as/export. Silent
   overwrite is not a resolution.

### 3.2 Local Roots

Local POSIX-like and Windows-like roots prefer
`atomic_replace_preferred` when a same-volume temporary write and rename
are available.

- If canonical object identity differs from the token, block as
  wrong-target prevention and open alias/details.
- If the generation token differs, block durable write and open an
  external-change diff when current bytes can be read.
- If the watcher is degraded but the generation token is exact, run the
  compare and write only if the token matches.
- If the root can only provide `content_hash` or mtime/size, re-read
  immediately before write. If uncertainty remains, downgrade to
  `capability_uncertain` or `in_place_write_with_review`; do not claim
  atomic replace.

### 3.3 Remote Roots

Remote roots use `remote_conditional_write` when a provider object ID,
remote revision token, ETag, or equivalent precondition is available.

- A remote save must carry the provider target identity and revision
  token that will be used as the write precondition.
- A stale token blocks the write and reports a remote precondition
  conflict. It does not fall back to blind overwrite.
- Reconnect, route change, remote-agent skew, or mount graph change
  invalidates the token and requires re-compare before any write.
- If the remote can no longer provide a conditional token, the class
  becomes `capability_uncertain` unless a reviewed in-place write lane
  is explicitly available.

### 3.4 Cloud-Backed Sync Roots

Cloud-backed roots include local sync folders and provider-mounted
filesystems where another process can rewrite, hydrate, dehydrate, or
version files outside the editor.

- If the provider exposes a stable version ID or ETag, use it as the
  strongest generation token.
- If the provider exposes only local metadata, re-compare immediately
  before write and again after a reviewed in-place write to confirm the
  provider did not rewrite the target during the save.
- If hydration, conflict marker creation, offline state, or sync
  backpressure makes object identity uncertain, block or reopen review.
- A cloud-backed root that lacks atomic replace must use
  `in_place_write_with_review`; it may not show the atomic copy.

### 3.5 Degraded or Unknown Roots

Degraded roots include archive views, generated views, unknown mounts,
watcher-unavailable roots, roots with only content-hash identity, and
roots whose permission or policy state cannot be refreshed.

- Missing identity token plus writable-looking permissions is still
  `capability_uncertain`.
- `read_only_mount` and `policy_blocked` stop before compare if the
  packet already proves no write is legal.
- Watcher degradation alone does not block save when an exact
  generation token can be compared.
- Capability uncertainty blocks normal save. The surface may offer
  save-as, export patch, open details, retry capability refresh, or
  request approval when policy allows.

## 4. Action Selection

The compare result chooses one of the following actions. The packet
records the action and reason in `on_match`, `on_mismatch`, or
`on_uncertain`.

| Action | Required use |
|---|---|
| `write` | Only when identity, generation token, permission, policy, and guarantee class all match the token. |
| `recompare` | Only when the root reports recoverable transient uncertainty and no write has been issued. |
| `open_diff` | When current target bytes differ from the staged base and can be displayed safely. |
| `reopen_review` | When a prior review exists but the target token, policy epoch, participant output, or remote revision has changed. |
| `block` | When target identity differs, policy/read-only state blocks, generation token is missing, remote precondition fails, or capability is uncertain. |
| `save_as` | When writing the original target is unsafe but creating a user-selected alternate target is allowed. |
| `cancel` | When the user or policy declines the offered safe path. |

## 5. Checkpoint and Rollback Posture

The token must not promise stronger recovery than the system can
provide.

| Checkpoint class | Meaning |
|---|---|
| `not_required_no_write` | No mutation is issued. Audit/export refs are enough. |
| `recovery_journal` | Staged content can be recovered from the editor recovery journal. |
| `local_history_checkpoint` | Local history can restore the prior target content. |
| `workspace_checkpoint_group` | Multiple related mutations share one checkpoint group. |
| `provider_revision_checkpoint` | Provider revision history or remote checkpoint is the recovery route. |
| `metadata_only_checkpoint` | Body capture is blocked by policy; metadata records the operation and recovery route. |
| `unavailable_blocks` | No trustworthy checkpoint exists, so the write is blocked or must move to review/export. |

Rollback classes:

- `no_mutation`
- `exact_restore`
- `checkpoint_restore`
- `provider_revision_restore`
- `compensating_write`
- `metadata_only_review`
- `manual_recovery_required`
- `unavailable_blocks`

Rules:

1. `in_place_write_with_review` requires a checkpoint or an explicit
   `metadata_only_checkpoint` before write.
2. `remote_conditional_write` must record the provider revision or
   checkpoint route when the provider supports one.
3. `read_only_mount`, `policy_blocked`, and `capability_uncertain`
   record `not_required_no_write` or `unavailable_blocks`; they do not
   invent recovery for a write that did not happen.

## 6. Support and Export Fields

One packet must be enough to diagnose save conflicts and wrong-target
prevention. A support/export packet includes at minimum:

- `packet_id`
- `token_id`
- `presentation_path`
- `logical_workspace_identity`
- `canonical_object_identity`
- `strongest_generation_token`
- `permission_snapshot`
- `write_mode`
- `save_guarantee_class`
- `compare_before_write.current_result`
- `compare_before_write.observed_generation_token`
- selected action and reason
- `rollback_checkpoint_posture`
- `support_export.wrong_target_prevention`
- `support_export.conflict_diagnosis`
- provider revision, policy decision, mount graph, or capability gap
  refs when those are involved
- redaction policy and parity signature when exported

Raw file contents, raw provider payloads, secrets, credentials, and
unredacted absolute paths do not cross this boundary unless the user or
policy explicitly includes them through a separate support-bundle flow.

## 7. Fixture Coverage

| Fixture | Guarantee class | Primary behavior |
|---|---|---|
| [`local_atomic_write.yaml`](../../fixtures/io/save_target_cases/local_atomic_write.yaml) | `atomic_replace_preferred` | Local generation token matches; atomic replace is allowed. |
| [`cloud_backed_in_place_review.yaml`](../../fixtures/io/save_target_cases/cloud_backed_in_place_review.yaml) | `in_place_write_with_review` | Cloud-backed root lacks atomic replace; review opens before in-place write. |
| [`remote_revision_token_conflict.yaml`](../../fixtures/io/save_target_cases/remote_revision_token_conflict.yaml) | `remote_conditional_write` | Remote revision changed; conditional write is blocked and review reopens. |
| [`read_only_mount.yaml`](../../fixtures/io/save_target_cases/read_only_mount.yaml) | `read_only_mount` | Archive/read-only mount blocks save and offers alternate paths. |
| [`policy_blocked_write.yaml`](../../fixtures/io/save_target_cases/policy_blocked_write.yaml) | `policy_blocked` | Policy denies write even though the root is otherwise writable. |
| [`missing_capability_fallback.yaml`](../../fixtures/io/save_target_cases/missing_capability_fallback.yaml) | `capability_uncertain` | Missing identity/capability token blocks normal save and offers save-as/export. |
