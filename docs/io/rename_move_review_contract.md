# Rename and move review contract

This contract freezes the review vocabulary for rename, move, copy,
cross-root transfer, alias-detach, and recompute plans before tree,
breadcrumb, refactor, import, migration, save-conflict, CLI, and
automation surfaces start mutating filesystem objects through separate
private rules.

Machine-readable companions:

- [`/schemas/io/rename_move_plan.schema.json`](../../schemas/io/rename_move_plan.schema.json)
  defines the `rename_move_plan_record` emitted before any risky path
  mutation is offered.
- [`/fixtures/io/rename_move_cases/`](../../fixtures/io/rename_move_cases/)
  contains YAML fixtures for local, remote, mirrored, generated
  companion, alias, and same-path ambiguity cases.

This contract composes with:

- [ADR 0006](../adr/0006-vfs-save-cache-identity.md) for VFS identity,
  root capability, watcher, and save pipeline semantics.
- [`/docs/filesystem/filesystem_identity_vocabulary.md`](../filesystem/filesystem_identity_vocabulary.md)
  for presentation path, logical workspace identity, canonical object,
  alias set, and readiness vocabulary.
- [`/docs/fs/path_truth_packet.md`](../fs/path_truth_packet.md) for
  path-truth chips and alias inspector projections.
- [`/docs/io/save_target_token_and_write_guarantee_contract.md`](./save_target_token_and_write_guarantee_contract.md)
  for compare-before-write and rollback posture vocabulary.
- [`/docs/editor/refactor_and_replace_transaction_contract.md`](../editor/refactor_and_replace_transaction_contract.md)
  and
  [`/docs/navigation/semantic_navigation_and_rename_contract.md`](../navigation/semantic_navigation_and_rename_contract.md)
  for semantic rename, refactor move, and preview packet linkage.
- [`/docs/migration/first_run_import_diff_and_rollback_contract.md`](../migration/first_run_import_diff_and_rollback_contract.md)
  for importer preview, checkpoint, and rollback gates.

If this document disagrees with the authoritative product design docs
or ADR 0006, those sources win and this contract plus its schema update
in the same change.

## Scope

Frozen here:

- the plan packet emitted before a rename, move, cross-root transfer,
  alias detach, or recompute action can mutate durable state;
- the review triggers for case folding, Unicode normalization,
  symlink and junction boundary escape, cross-root moves, generated
  companion relations, same-path/different-object ambiguity, and
  canonical-object drift;
- the cross-root transfer classes and capability checks that prevent
  platform folklore from becoming product behavior;
- the safe action vocabulary: `rename_in_place`, `copy_then_review`,
  `detach_alias`, `block_and_explain`, and
  `recompute_against_current_root_capabilities`; and
- parity requirements across file tree, breadcrumb, refactor rename,
  import or migration, save conflict, CLI, automation, and support
  export surfaces.

Out of scope:

- implementing provider-specific rename or move primitives;
- defining final language refactor semantics;
- defining final UI layout; and
- replacing the VFS identity, save-target token, or semantic rename
  packets.

## 1. Rename or move plan record

Every risky path mutation emits one `rename_move_plan_record` before a
normal apply affordance is shown. A plan is risky when the presentation
path change and the canonical object effect can diverge, when the source
and target roots have different capability envelopes, or when policy,
trust, generated lineage, alias, mirror, or remote freshness changes the
meaning of the operation.

Minimum fields:

| Field | Required purpose |
|---|---|
| `plan_id` | Stable diagnostic handle for one proposed path mutation. |
| `initiating_surface` | File tree, breadcrumb, semantic refactor, import or migration, save conflict, CLI, automation, or another governed surface. |
| `operation_intent` | The user or tool intent: rename, move, rename-and-move, copy, transfer, detach alias, or recompute. |
| `current_presentation_path` | The URI and label the user selected. This is never silently replaced by a canonical URI. |
| `proposed_presentation_path` | The requested presentation path after the operation, even when the plan will block. |
| `logical_workspace_identity` | Workspace object identity, root id, trust state, and policy scope. |
| `current_canonical_object` | The canonical object currently resolved from the selected path. |
| `proposed_canonical_effect` | Whether the plan preserves, creates, rebinds, detaches, relinks generated companions, becomes ambiguous, requires recompute, or blocks without mutation. |
| `capability_context` | Source root, target root, cross-root transfer class, capability checks, and whether recompute is required before apply. |
| `review_gate` | Review requirement, typed reason codes, preview or review refs, and whether stale plans block apply. |
| `policy_trust_boundary_notes` | Typed policy, trust, root authority, provider, mirror, generated-lineage, or symlink/junction boundary notes. |
| `rollback_posture` | Checkpoint class, rollback class, checkpoint ref, and honest rollback summary. |
| `next_safe_action` | The single primary safe action plus required confirmation and follow-up refs. |
| `parity_requirements` | Surfaces that must render the same vocabulary and the fields they must preserve. |
| `support_export` | Redaction-safe export summary and parity signature. |

Rules:

1. A surface that cannot produce a `rename_move_plan_record` may not
   render a normal risky rename or move affordance. It may render
   inspect, preview, recompute, save-as, copy, or cancel.
2. The plan must carry both `current_presentation_path` and
   `current_canonical_object` whenever the root can provide canonical
   identity. If they differ, `review_gate.review_required` is true
   unless the plan is inspect-only.
3. A plan that depends on a stale capability snapshot, policy epoch,
   watcher claim, mirror digest, remote revision, generated lineage, or
   canonical object token must set `stale_plan_blocks_apply = true`.
4. No plan may encode "try the platform rename and see what happens" as
   a safe path. The `silent_best_effort_forbidden` field is always true.
5. Apply paths reuse the same plan from desktop, CLI, automation, import
   tooling, save conflict, and refactor rename flows. A surface may add
   local UI context, but it may not use a different review vocabulary.

## 2. Canonical object effects

The plan separates the visible path request from the canonical object
effect.

| Effect | Meaning | Review posture |
|---|---|---|
| `preserves_canonical_object` | Same object, new presentation path. | Allowed after capability checks; preview required for risky case, normalization, alias, or generated relations. |
| `creates_new_canonical_object` | Copy or transfer creates a new object identity. | Review required when root boundaries differ or rollback is weaker than exact restore. |
| `rebinds_alias_to_existing_object` | The visible path would point at an existing canonical object. | Review required; collision and same-path ambiguity must be shown. |
| `detaches_alias_to_new_authority` | The plan makes an alias or generated companion independently writable. | Review required; generated lineage and rollback posture must be explicit. |
| `changes_generated_companion_relation` | The operation relinks, detaches, or blocks a generated source/output relation. | Review required; generated output cannot be silently renamed away from its canonical source. |
| `ambiguous_same_path_different_object` | Same presentation path could resolve to a different canonical object. | Block until recompute or explicit review proves the target. |
| `unknown_until_recompute` | Root capabilities changed or were never known strongly enough. | Normal apply is blocked; recompute is the next safe action. |
| `blocked_no_mutation` | Policy, read-only, virtual, or unresolved identity blocks mutation. | No write or move occurs. |

## 3. Required review triggers

These trigger codes are contractual. They appear in review, support,
mutation journal, and CLI packets; they are not hidden adapter notes.

| Trigger | Required handling |
|---|---|
| `case_only_rename_on_insensitive_root` | Review before mutation on roots that fold case. The plan must disclose whether the root preserves case and whether a two-step temporary rename is required or forbidden. |
| `unicode_normalization_change` | Review before mutation when only Unicode normalization changes or when current and proposed names normalize to the same spelling. |
| `symlink_escape` | Review or block when the resolved canonical target escapes the selected root through a symlink. |
| `junction_escape` | Review or block when a Windows junction or mount-like boundary changes the target authority. |
| `cross_root_transfer` | Review before mutation when source and target roots differ or when copy-delete semantics replace same-root rename semantics. |
| `generated_companion_relation` | Review before mutation when source/generated, schema/output, notebook/result, or similar companion relations may detach or relink. |
| `same_path_different_object` | Block or recompute when the same presentation path may now resolve to a different canonical object. |
| `canonical_object_drift` | Recompute or reopen review when identity tokens, remote revisions, mirror digests, mount graph, or alias chains drift. |
| `policy_or_trust_boundary` | Block or request approval when the plan crosses trust, policy, permission, or provider authority boundaries. |
| `mirror_freshness_lag` | Recompute against the current root and mirror digest before any mutating transfer. |

Rules:

1. Any plan carrying one of the trigger codes above must set
   `review_gate.review_required = true` except when the next safe action
   is `block_and_explain` or
   `recompute_against_current_root_capabilities`; those actions are
   already non-mutating.
2. Case-only and normalization-only plans on uncertain roots do not
   degrade to blind copy/delete. They recompute or block.
3. Symlink and junction escape plans must show both the selected
   presentation boundary and the resolved canonical boundary.
4. Same-path/different-object ambiguity is a wrong-target prevention
   condition, not a rename conflict that can be auto-resolved.

## 4. Cross-root transfer classes

Cross-root plans name the transfer class before they name a primitive.
This prevents a file tree drag, refactor move, migration import, or save
conflict from assuming POSIX rename behavior across roots.

| Class | Meaning | Default next safe action |
|---|---|---|
| `same_root` | Source and target share one root authority. | `rename_in_place` after capability checks. |
| `same_authority_cross_root` | Roots are distinct product roots but one authority can prove identity and rollback. | `copy_then_review` or `rename_in_place` only when the authority explicitly supports it. |
| `local_to_remote` / `remote_to_local` | The operation crosses a local/remote boundary. | `copy_then_review`; delete or detach source only after review. |
| `remote_to_remote_same_provider` | One provider can prove both object ids and revisions. | Conditional transfer with review and provider rollback ref. |
| `remote_to_remote_different_provider` | Different remote authorities. | `copy_then_review`; no implicit source delete. |
| `container_boundary` | Container, bind mount, or workspace mount boundary changes. | Recompute capabilities and review. |
| `cloud_or_mirror_boundary` | Sync, mirror, or delayed replication boundary is involved. | Recompute freshness, then copy/review. |
| `archive_or_virtual_boundary` | Target or source is read-only, virtual, archive, or generated-only. | Block, save-as, or copy out under review. |
| `generated_companion_boundary` | The operation crosses source/generated relation authority. | Detach alias or block until generator relation is reviewed. |
| `unsupported_unknown` | The root cannot prove transfer semantics. | Recompute or block. |

## 5. Safe actions

The plan chooses one primary safe action. Silent platform fallback is not
a safe action.

| Action | When allowed |
|---|---|
| `rename_in_place` | Same canonical object can be preserved under current root capabilities, compare tokens are current, rollback is available, and any required preview has been reviewed. |
| `copy_then_review` | Cross-root, remote, mirror, cloud, or provider transfer where the source must remain intact until target identity, contents, permissions, generated relations, and rollback are reviewed. |
| `detach_alias` | Alias or generated companion should become independent, but only after the relation and future write authority are explicit. |
| `block_and_explain` | Policy, read-only, virtual, same-path/different-object, untrusted boundary, or missing identity makes mutation unsafe. |
| `recompute_against_current_root_capabilities` | Capability, watcher, policy, mirror, remote revision, mount graph, or canonical object drift invalidates the plan. |
| `open_review_diff` | Current and proposed targets can be compared safely before another action is chosen. |
| `request_approval` | Policy or trust can admit the operation after explicit approval; no mutation occurs while approval is pending. |
| `save_as` | Original target is unsafe but a user-selected alternate target can be created under review. |
| `cancel` | User or policy declines all offered safe paths. |

## 6. Surface parity

The following surfaces must render the same review vocabulary:

- file tree drag/drop, inline rename, and context menu actions;
- breadcrumb rename and move actions;
- semantic refactor rename and refactor move previews;
- import and migration tools that create or relocate files;
- save conflict, external move, and save-as recovery flows;
- quick-open, command palette, CLI, automation, and support export
  summaries.

Parity rules:

1. Every surface preserves `plan_id`, `operation_intent`,
   current/proposed presentation paths, current canonical object,
   proposed canonical effect, cross-root transfer class, review gate,
   next safe action, rollback posture, and support export fields.
2. A compact UI may hide secondary rows, but a keyboard-reachable detail
   surface must show the same typed fields the support export records.
3. A refactor rename preview may cite the rename/move plan; it may not
   reinterpret alias, root, generated, or rollback posture through a
   language-provider-only model.
4. Import or migration tools may stage copy plans, but they may not
   silently turn a blocked move into a copy/delete sequence.
5. Save conflict recovery may offer save-as or copy-out, but it must not
   reuse stale path assumptions after canonical object drift.

## 7. Fixture requirements

Fixtures under
[`/fixtures/io/rename_move_cases/`](../../fixtures/io/rename_move_cases/)
are reviewer-facing YAML records that validate the contract before
runtime implementations exist. Each fixture must:

- use `/schemas/io/rename_move_plan.schema.json`;
- include current and proposed presentation paths;
- include current canonical identity and proposed canonical effect;
- cite at least one review reason or explain why no mutation is allowed;
- declare the cross-root transfer class even for same-root local cases;
- carry policy/trust boundary notes, including an explicit `none` note
  when there is no boundary change;
- declare rollback posture without promising stronger recovery than the
  plan can provide;
- choose exactly one primary next safe action; and
- set `parity_requirements.one_vocabulary_for_all_surfaces = true` and
  `parity_requirements.no_surface_local_fallbacks = true`.

Acceptance is met when local, remote, mirrored, generated companion,
case-folding, normalization, symlink/junction escape, cross-root, and
same-path/different-object fixtures are reviewable without relying on
platform folklore or hidden best-effort behavior.
