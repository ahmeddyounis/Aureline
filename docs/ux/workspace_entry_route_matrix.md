# Workspace Entry Route Matrix and Safe-Open Contract

This contract freezes the route-level behavior for workspace entry
before shell implementation makes those routes surface-local. It sits
between the entry / restore object model and the concrete Start Center,
menu, palette, protocol-handler, and workspace-switcher surfaces.

The machine-readable schema lives at:

- [`/schemas/workspace/entry_route.schema.json`](../../schemas/workspace/entry_route.schema.json)

The worked fixtures live under:

- [`/fixtures/workspace/entry_route_cases/`](../../fixtures/workspace/entry_route_cases/)

This document composes with, and does not replace:

- [`/docs/workspace/entry_restore_object_model.md`](../workspace/entry_restore_object_model.md)
  for `entry_verb`, `target_kind`, `resulting_mode`,
  `admission_class`, restore levels, missing-target states, and
  next-step decision hooks.
- [`/docs/ux/start_center_contract.md`](./start_center_contract.md)
  for Start Center and workspace-switcher primary action posture.
- [`/docs/ux/archetype_detection_contract.md`](./archetype_detection_contract.md)
  for post-entry readiness, setup-later, and first-useful-work routing.
- [`/docs/adr/0018-workspace-trust-and-restricted-mode.md`](../adr/0018-workspace-trust-and-restricted-mode.md)
  for trust states, restricted-mode transitions, and permission
  propagation.
- [`/docs/support/recovery_ladder_packet.md`](../support/recovery_ladder_packet.md)
  for safe mode, open-without-restore, and restricted reopen recovery.
- [`/schemas/platform/deep_link_intent.schema.json`](../../schemas/platform/deep_link_intent.schema.json)
  for protocol-handler and deep-link intent review.

Where this contract disagrees with those sources, the upstream source
wins and this route matrix must update in the same change.

## Scope

This contract freezes one `workspace_entry_route_case_record` shape and
one route matrix covering:

- `workspace_entry.open_folder`
- `workspace_entry.open_workspace`
- `workspace_entry.clone_repository`
- `workspace_entry.import`
- `workspace_entry.resume_snapshot`
- `workspace_entry.restore_last_session`
- `workspace_entry.deep_link`
- `workspace_entry.open_in_safe_mode`
- `workspace_entry.continue_in_restricted_mode`

The route id is distinct from `entry_verb`, `source_surface`, and the
measurement `entry_route_id` (`er.*`). A route answers "which product
path is the user taking"; `entry_verb` answers "what entry object will
be emitted"; `source_surface` answers "where the intent originated";
`er.*` answers "how the onboarding and launch scoreboards aggregate
the event."

Out of scope: final UI layout, native file-picker implementation,
clone/import engines, recovery supervisor implementation, and concrete
command registration.

## Route Matrix

Every row in this matrix resolves to exactly one
`project_entry_action_record` or one trust/recovery transition record
on commit. A surface may be unavailable in a given deployment envelope,
but it may not redefine the route's meaning.

| Route | Entry record | Required preview before commit | Required recovery if commit fails or narrows |
| --- | --- | --- | --- |
| `workspace_entry.open_folder` | `entry_verb = open`, `target_kind = local_folder` or `local_repo_root`, `resulting_mode = folder` or `repo_root` | Target identity, trust state, resulting mode, restore availability if recent state exists, optional archetype/readiness work, and whether execution remains gated. | Cancel leaves the current workspace untouched. If the open fails, offer `locate_missing_target`, `open_in_safe_mode`, or `reopen_previous_workspace`. If trust narrows unexpectedly, pause before switch and offer `continue_in_restricted_mode` or previous workspace return. |
| `workspace_entry.open_workspace` | `entry_verb = open`, `target_kind = workspace_manifest` or `workset_manifest`, `resulting_mode = workspace_with_roots` or `workset_slice` | Manifest identity, root list summary, per-root trust/policy boundary changes, missing roots, restore class, and whether adding roots changes the active workspace boundary. | Current workspace remains available until all roots pass admission. Failed root admission opens a boundary-choice sheet, not a half-switched shell. |
| `workspace_entry.clone_repository` | `entry_verb = clone`, `target_kind = remote_repository`, `resulting_mode = clone_then_review`, `clone_then_open`, `clone_then_add`, or `clone_only` | Host, auth posture, destination disposition, collision class, network/proxy/mirror posture, submodule/LFS/bootstrap side effects, and post-clone action. Clone never implies trust. | Failed materialization rolls back staging and returns to previous workspace or Start Center. Durable destination collisions must be reviewed before writes. Post-clone setup failures keep the clone inspectable and offer restricted open. |
| `workspace_entry.import` | `entry_verb = import`, `target_kind = portable_state_package`, `handoff_packet`, `competitor_config_root`, or `template_or_prebuild_snapshot` | Imported artifact class, producer/signature posture, affected profile/workspace scope, item classes, policy/trust blocks, comparison or dry-run result, rollback checkpoint class, and side effects. | Cancellation writes nothing outside labelled staging. Applied imports require a rollback or compensating recovery class. Failed import keeps the pre-import checkpoint and offers `review_migration_report`, `roll_back_import`, or previous workspace return. |
| `workspace_entry.resume_snapshot` | `entry_verb = resume`, target kind `managed_cloud_workspace`, `ssh_workspace`, `container_workspace`, `devcontainer_workspace`, or `template_or_prebuild_snapshot`, resulting mode `resume_live_session` or `open_prebuild_minimal` | Snapshot/session identity, live-vs-dehydrated status, authority re-evaluation, tenant or remote scope, missing prerequisites, and whether local work continues if remote attach fails. | Resume failure must preserve a read-only or local placeholder when available and expose reconnect, reauth, restricted continuation, or previous workspace return. Expired authority never resumes silently. |
| `workspace_entry.restore_last_session` | `entry_verb = restore`, `target_kind = recovery_checkpoint` or prior workspace target, `resulting_mode = restore_last_session` or `restore_from_checkpoint` | Restore level, missing-target states, dirty-buffer and remote-session counts, execution posture, checkpoint-linked recovery class, and open-without-restore bypass. | Failed restore keeps the restore prompt evidence and offers `open_without_restore`, `open_in_safe_mode`, `export_evidence`, or previous workspace return. It must never auto-rerun terminals, tasks, notebooks, debug sessions, or AI/provider actions. |
| `workspace_entry.deep_link` | `entry_verb = open`, `clone`, `import`, `resume`, or `restore` after intent review; `source_surface = deep_link` or companion/browser return | Origin, handler ownership, route class, target identity, replay posture, trust/policy/tenant scope, authority delta, and degraded fallback. Remote, managed, or privileged targets require review before authority can widen. | Denied or degraded links open an intent review sheet, read-only placeholder, browser fallback, or exportable explanation. A link can never strand the user in an untrusted empty shell; previous workspace remains reachable. |
| `workspace_entry.open_in_safe_mode` | Trust transition `safe_mode_workspace_restricted`; may pair with `entry_verb = restore` or `open` | Recovery-ladder reason, preserved state, disabled capabilities, restore suspension, extension quarantine posture, checkpoint/export path, and current workspace trust result. | Safe mode is reversible through the recovery ladder. If safe mode cannot enter, retain previous workspace or open a minimal local shell with evidence export. |
| `workspace_entry.continue_in_restricted_mode` | Trust transition `continue_in_restricted_mode`; may pair with any route whose admission narrowed | Prior and resulting trust state, capability floor, blocked execution/mutation surfaces, policy reason, escalation cues, and whether layout restore continues. | Continuing restricted must keep read/search/navigate/edit/save available. If the current task requires a blocked capability, offer trust review, policy review, safe mode, or previous workspace return instead of dead-ending. |

## Preview Contract

Every route preview carries these fields before execution:

- `target_identity_preview` — target kind, opaque identity ref,
  availability, freshness, and whether target identity is verified,
  inferred, missing, ambiguous, or policy-hidden.
- `trust_policy_preview` — current trust state, resulting trust state
  if known, authority delta, policy epoch, required review, and whether
  authority re-evaluation is mandatory.
- `destination_preview` — disposition and collision class for any route
  that may write bytes; clone/import routes must show staging vs durable
  destination before the first write.
- `side_effect_preview` — closed list of side-effect classes such as
  contacting remote, writing staging, materializing a clone, importing
  profile state, starting a container, changing layout, disabling
  extensions, or suspending restore.
- `import_preview` — required on import routes and any deep link that
  resolves to an import. It names artifact class, producer, signature
  state, affected item classes, blocked item classes, and rollback or
  compensating recovery class.
- `restore_preview` — required on restore, resume, safe-mode, and any
  route with restore availability. It names restore level, missing
  target states, session execution posture, dirty-buffer summary, and
  checkpoint-linked recovery class.
- `prerequisite_preview` — required when any setup, authority, runtime,
  mount, remote agent, credential store, policy source, or imported
  artifact is missing, expired, unavailable, or degraded.
- `fallback_preview` — at least one typed cancellation, rollback,
  restricted continuation, safe-open, inspect-only, or previous-
  workspace action.

A preview that cannot populate those fields denies with a typed
`review_requirement` instead of defaulting to a generic "open anyway"
path.

## Surface Parity

The same route must be reachable, with the same command id family and
the same review posture, from every applicable entry surface:

- Start Center advertises `open_folder`, `open_workspace`,
  `clone_repository`, `import`, and `restore_last_session` as distinct
  primary actions.
- Main menu exposes the same route ids under File / Open, File / Open
  Workspace, File / Clone, File / Import, File / Restore, and recovery
  entries when recovery state exists.
- Command palette exposes every route as a typed command row with the
  same disabled reason and preview sheet as the menu route.
- Deep-link and protocol-handler entry resolves through the same route
  record after intent review; it does not call a private opener.
- Workspace switcher re-advertises open, clone, import, restore,
  restricted continuation, and safe-mode recovery when they are valid
  from an active workspace.

Parity rules:

1. A route visible on more than one surface must carry one
   `route_contract_ref` and one `primary_command_id_ref`. A surface
   may add placement-specific chrome, but not placement-specific
   semantics.
2. A route hidden or disabled on a surface must cite a typed disabled
   reason and a reachable alternate surface. Hiding all routes that
   preserve local work is non-conforming.
3. Review requirements are sticky. If Start Center requires import
   comparison, a palette or deep-link invocation of the same import
   also requires comparison.
4. Measurement aggregation (`er.*`) does not change product semantics.
   `er.clone_or_import` may aggregate clone and import for reporting,
   but the product route and `entry_verb` stay distinct.
5. Keyboard and accessibility parity follows the Start Center and
   shell contracts: every primary route, cancellation action, safe-open
   action, and previous-workspace action is keyboard reachable.

## Deep-Link Authority

Deep links are external requests and must be treated as reviewable
intents until admitted.

Deep-link rules:

1. Validate origin, handler ownership, expected action class, replay
   posture, target identity, trust state, tenant/workspace scope, and
   policy epoch before executing anything.
2. `authority_delta = none` is the only path that may open without an
   authority review. Any trust boundary crossing, policy boundary
   crossing, auth scope widening, remote authority rebind, privileged
   command request, or mutating action requires review first.
3. Remote, managed, or privileged targets may open a read-only
   placeholder or intent review sheet while authority is pending, but
   they may not attach, clone, import, restore, run, trust, or mutate
   silently.
4. A deep link that points at a stale, missing, moved, expired,
   ambiguous, or policy-blocked target routes to a typed fallback and
   keeps the previous workspace reachable.
5. A deep link may carry a proposed route, but the resolver computes
   the final route from current target identity, trust, policy, and
   replay state. The link's own claim of authority is never trusted.

## Safe-Open, Restricted-Open, and Restore Transitions

`workspace_entry.open_in_safe_mode` is a recovery route. It enters
`safe_mode_workspace_restricted`, cites a recovery-ladder action, names
which capabilities are disabled, and suspends layout restore when the
recovery-ladder row requires it. It is not a generic "try again" path.

`workspace_entry.continue_in_restricted_mode` is an explicit
continuation after a trust denial, expiry, policy narrowing, identity
gate, or recovery fallback. It preserves the restricted-posture floor:
read, search, navigate, edit, save, workspace open, support export, and
admin-policy read remain admitted. Mutating or execution surfaces stay
gated until trust/policy review admits them.

`workspace_entry.restore_last_session` is distinct from safe mode and
restricted continuation. Restore names what will rehydrate, what will
remain evidence-only, what is missing, and what will not rerun. Restore
failure never implies state deletion; evidence and checkpoints remain
available until the user chooses a typed cleanup or rollback path.

## Cancellation, Rollback, and Previous Workspace

Entry routes are two-phase:

1. **Preview phase** resolves route, target, authority, destination,
   side effects, prerequisites, restore/import classes, and fallback
   actions.
2. **Commit phase** emits the entry, trust, recovery, restore, or
   import record and performs the minimum side effects required by the
   route.

Rules:

1. Cancellation during preview leaves the previous workspace, current
   buffers, trust state, restore prompt, recent-work rows, and recovery
   evidence unchanged.
2. New workspace switch commits are not destructive until admission
   succeeds. The previous workspace snapshot stays reopenable until the
   new workspace reaches a first useful surface or the user explicitly
   closes it.
3. Labelled staging is the default for clone/import writes before
   review. Staging rollback removes only the staged route artifacts and
   never deletes existing user files.
4. Durable import or restore mutations require a checkpoint or a
   compensating recovery class before commit. If no recovery class
   exists, the preview must say so and offer inspect-only/export instead.
5. Unexpected capability narrowing pauses commit. If the resulting mode
   is more restricted than previewed, the user sees the narrowed
   capability set and chooses `continue_in_restricted_mode`,
   `open_in_safe_mode`, `open_without_restore`, retry/reconnect/reauth,
   or `reopen_previous_workspace`.
6. Failed open, clone, import, restore, or resume paths preserve at
   least one escape: previous workspace, Start Center, inspect-only
   placeholder, safe mode, restricted continuation, or evidence export.
   A dead-end empty shell is non-conforming.

## Fixture Coverage

The fixture corpus contains one case per required route plus a
privileged deep-link case:

- `open_folder_plain.json`
- `open_workspace_multi_root_boundary.json`
- `clone_repository_review_first.json`
- `import_portable_state_compare.json`
- `resume_snapshot_reauth_required.json`
- `restore_last_session_missing_remote.json`
- `deep_link_managed_workspace_requires_review.json`
- `open_in_safe_mode_crash_loop.json`
- `continue_in_restricted_mode_after_policy_narrow.json`

Each fixture is a `workspace_entry_route_case_record` validated by the
schema in `/schemas/workspace/entry_route.schema.json`.
