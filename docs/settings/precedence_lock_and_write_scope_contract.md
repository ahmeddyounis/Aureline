# Settings precedence, lock-state, and write-scope contract

This contract publishes the user-visible grammar Aureline uses when a
settings surface explains resolution or previews mutation. The
setting-definition registry remains the schema truth for setting ids,
value types, allowed scopes, aliases, migrations, redaction, preview
classes, and restart posture. This document defines the separate
inspection and mutation grammar so settings UI, CLI, migration, sync,
support, and export flows do not invent precedence folklore or silently
broaden write scope.

Companion artifacts:

- [`/schemas/settings/precedence_resolution.schema.json`](../../schemas/settings/precedence_resolution.schema.json)
  defines `precedence_resolution_packet` and
  `write_scope_review_packet`.
- [`/schemas/settings/lock_state_reason.schema.json`](../../schemas/settings/lock_state_reason.schema.json)
  defines the shared lock-state reason packet and reason rows used by
  both resolution and write-scope packets.
- [`/fixtures/settings/precedence_cases/`](../../fixtures/settings/precedence_cases/)
  contains worked cases for shadow chains, policy and emergency
  ceilings, fan-out review, mixed-version downgrade, alias-only import,
  stale reads, wrong target, secret requirements, and missing
  dependencies.

This contract composes with:

- [`docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`](../adr/0008-settings-definition-and-effective-configuration-resolver.md)
  for the setting-definition and effective-setting vocabulary.
- [`docs/settings/schema_registry_seed.md`](./schema_registry_seed.md)
  for schema publishing, alias retention, and registry evolution.
- [`docs/settings/sync_and_device_registry_seed.md`](./sync_and_device_registry_seed.md)
  for optional sync bundles, sessions, and conflict packets.
- [`docs/policy/admin_policy_and_bundle_cache_contract.md`](../policy/admin_policy_and_bundle_cache_contract.md)
  for policy ceilings, safe defaults, and emergency disable behavior.

If this document and the settings ADR disagree, the ADR wins and this
contract plus schemas must be updated in the same change.

## Scope

Frozen here:

- the user-visible precedence lattice across product defaults,
  packages, templates, imports, profile, user, device, workspace,
  folder, environment, remote target, session override, policy, and
  emergency override scopes;
- deterministic tie-break and shadow-chain rules;
- lock-state reason codes and repair-action vocabulary for inherited,
  policy-locked, unsupported, wrong-target, secret-required,
  missing-dependency, degraded, stale, and alias-only states;
- write-scope review packets for edits that fan out across user,
  workspace, folder, profile, device, tenant, imported settings,
  remote target, environment, or policy authority;
- downgrade rules for mixed-version settings, absent capabilities,
  imported aliases, and stale effective-setting reads.

Out of scope:

- implementing a settings UI, CLI, resolver crate, sync backend, or
  policy evaluator;
- changing the canonical setting-definition schema;
- changing the low-level effective-setting record already frozen by
  the settings ADR.

## Core invariants

1. **Schema truth stays separate from explanation truth.** The
   setting-definition row says what a setting is and where it may be
   written. A precedence-resolution packet says how the current value
   was selected for a target.
2. **Every inspector shows the same chain.** UI, CLI, support export,
   migration, sync, and docs use the same `shadow_chain`,
   `blocked_scopes`, `lock_reasons`, and `downgrade` fields.
3. **Ceilings are explicit.** Policy and emergency overrides are not
   ordinary user preferences. They cap, lock, disable, or freeze the
   layered winner and remain visible as authority rows.
4. **Blocked scopes remain visible.** A wrong target, unsupported
   scope, missing secret, absent dependency, stale read, or alias-only
   source is not dropped from the explanation.
5. **Write previews name exact targets.** A write-scope review packet
   lists every logical file or authority that would change and every
   target that was refused.
6. **No silent local-to-shared broadening.** A request that begins as
   user, device, or local scope cannot fan out to workspace, folder,
   tenant, policy, imported, or remote authority without an explicit
   review requirement or denial.
7. **Downgrades are honest.** Mixed versions, absent capabilities,
   imported aliases, and stale effective-setting reads degrade to
   inspect-only or read-only until repaired.

## Precedence lattice

The lattice is ordered by rank. Higher rank ordinary value candidates
win over lower rank ordinary value candidates. Policy and emergency
override are authority ceilings: they are evaluated after the layered
candidate winner and may only narrow, lock, disable, freeze, or cap.

| Rank | Scope | Role | Writable by ordinary settings edit | Notes |
|---:|---|---|---|---|
| 10 | `product_default` | Built-in, release-channel, or experiment default. | No | Maps to built-in and channel or experiment default layers. |
| 20 | `package_default` | Package or extension-contributed default admitted by the package registry. | No | Package provenance must be visible; package defaults cannot redefine trust or policy meaning. |
| 30 | `template_default` | Project template, scaffold, prebuild, or starter-kit default. | No | Template lineage stays visible; templates may not silently widen trust or egress. |
| 40 | `imported_setting` | Imported profile, migration import, or compatibility translation. | Import or migration only | Imported aliases may become alias-only downgrade rows instead of durable writes. |
| 50 | `profile_setting` | Active profile preference that is portable in profile scope. | Yes, through profile apply/edit | Machine bindings and workspace trust are excluded. |
| 60 | `user_setting` | User-global preference. | Yes | Sync may target this scope when allowed and non-widening. |
| 70 | `device_setting` | Machine/device-local value. | Yes, local device only | Never ordinary sync; exports need a machine-binding addendum. |
| 80 | `workspace_setting` | Workspace-owned or workspace-local setting. | Yes, when workspace authority allows | Shared with workspace users; high-risk writes need preview/checkpoint. |
| 90 | `folder_setting` | Folder, module, or workset-scoped override. | Yes, when the setting allows folder scope | Deeper folder wins inside this rank. |
| 100 | `environment_setting` | Environment, devcontainer, process, or shell-derived override. | Usually no; explicit save may promote elsewhere | Wrong-target or missing-capability states must remain visible. |
| 110 | `remote_target_setting` | Remote, SSH, container, managed workspace, or target-side authority. | Through target authority only | Target mismatch degrades to `wrong_target`; disconnected targets degrade read-only. |
| 120 | `session_override` | Command/session/local action override. | Yes, ephemeral only | Never auto-promotes to durable scope. |
| 900 | `policy` | Signed admin, tenant, or managed policy ceiling. | No user write | May lock, constrain, or narrow; never merges as user preference. |
| 1000 | `emergency_override` | Signed emergency disable, freeze, or safe-default ratchet. | No user write | Highest ceiling; may only narrow, disable, or freeze until signed evidence clears it. |

The packet uses these user-visible scopes even when a lower-level
effective-setting record uses the ADR scope ids. For example,
`product_default` may map to `built_in_default` or
`channel_or_experiment_default`, and `device_setting` maps to
`machine_specific`. The mapping is one-way for explanation. It does
not change setting-definition allowed scopes.

## Tie-break rules

Candidates are first filtered by setting id, alias migration, target
context, allowed scope, and read freshness. Blocked candidates are
kept in the packet with `relation: blocked`.

Ordinary value candidates then resolve in this order:

1. Highest `precedence_rank` wins.
2. Inside the same rank, a non-blocked candidate beats a blocked or
   downgraded candidate.
3. An explicit target match beats a target-class match, which beats a
   global candidate.
4. More specific scope beats less specific scope. A deeper folder wins
   over a parent folder; an exact remote target wins over a target
   family; an exact environment wins over a generic environment.
5. Fresh data beats stale or last-known-good data.
6. Exact schema compatibility beats compatible downgrade, which beats
   alias-only or mixed-version downgrade.
7. Higher source revision wins when revisions share an authority and
   the schema declares monotonic revision semantics.
8. `stable_source_ref` is the final deterministic tie-break. It is a
   last resort only; the packet must still expose
   `conflict_requires_review` when equal-rank rows are not semantically
   equivalent.

Policy and emergency rows do not use ordinary value tie-breaks.
`policy` applies as a narrowing ceiling after the ordinary winner.
`emergency_override` then applies as the highest ceiling. An emergency
row may supersede policy only when it is signed, in scope, and stricter
than the current value.

## Shadow chain

Every `precedence_resolution_packet` carries three related views:

- `candidates`: full rows for every source considered, including
  package, template, import, user, workspace, folder, environment,
  remote target, policy, and emergency sources.
- `shadow_chain`: compact ordered rows for inspectors. Every row has
  `winner`, `shadowed`, `blocked`, `capped`, `ignored`, or
  `downgraded` relation semantics through `relation`.
- `blocked_scopes`: explicit list of sources that could not
  participate as ordinary candidates because of unsupported scope,
  wrong target, secret requirement, missing dependency, stale read,
  policy lock, emergency override, or alias-only migration.

Shadowed means a candidate was valid but lost to a higher rank or a
tie-break. Blocked means the candidate could not be considered as an
ordinary value. Capped means the candidate was the layered winner but
policy or emergency authority narrowed it. Downgraded means the value
can be inspected but writes or certainty are reduced.

## Lock-state reasons

`lock_state_reason.schema.json` freezes reason rows so surfaces can
render disabled controls and repair paths without scraping logs.

| Reason code | Meaning | Typical repair action |
|---|---|---|
| `none` | No lock reason. | `none` |
| `inherited` | Value is inherited from a lower or parent scope. | `inspect_source` |
| `policy_locked` | Policy pins the value or blocks mutation. | `request_policy_exception` |
| `policy_constrained` | Policy admits only a narrower value set. | `request_policy_exception` |
| `emergency_override` | Emergency disable, freeze, or safe default is active. | `inspect_source` |
| `unsupported_scope` | The setting cannot be read or written at that scope. | `choose_supported_scope` |
| `wrong_target` | Candidate belongs to another folder, environment, device, or remote target. | `retarget_setting` |
| `secret_required` | Applying or inspecting the effective value requires a brokered secret alias not currently bound. | `bind_secret` |
| `missing_dependency` | Required package, extension, runtime, feature, or target capability is absent. | `install_dependency` |
| `degraded_read_only` | Resolver can show a last-known-good or partial value but cannot safely mutate. | `use_local_only` |
| `migration_alias_only` | Legacy id resolved only as an alias for migration/inspection, not durable write. | `open_migration_review` |
| `mixed_version_downgrade` | Producer and consumer schema versions differ enough to reduce certainty. | `upgrade_schema` |
| `imported_alias_only` | Imported source used a legacy alias that must be reviewed before durable write. | `open_migration_review` |
| `stale_effective_read` | The effective-setting read is older than the freshness budget. | `refresh_effective_read` |
| `absent_capability` | Capability marker is unavailable or unsatisfied. | `install_dependency` or the owning surface's repair action |
| `read_only_authority` | Current authority can inspect but not mutate the source. | `inspect_source` |
| `insufficient_permission` | Caller lacks authority for this scope. | `request_approval` |
| `unsupported_write_fanout` | Requested fan-out crosses an unsupported target class. | `choose_supported_scope` |
| `checkpoint_required` | A rollback checkpoint is required before apply. | `create_checkpoint` |
| `approval_required` | Approval ticket is required before apply. | `request_approval` |
| `validation_failed` | Proposed value failed type, enum, range, regex, or shape validation. | `inspect_source` |

A packet may contain multiple reasons. The decisive reason appears
first, but supporting reasons remain visible so support and migration
flows can explain what else would block the write after the first
problem is repaired.

## Write-scope review

A `write_scope_review_packet` is required whenever a settings edit
might touch more than the initiating target, might touch a shared
target, might touch policy-controlled authority, or might be blocked
by downgrade state.

The packet contains:

- `requested_change`: initiating setting id, requested scope, actor,
  and redacted new value preview.
- `fanout_summary`: requested scope, effective write scope, exact
  write count, blocked count, shared-scope flag, policy-controlled
  flag, and local-to-shared broadening flag.
- `scope_writes`: one row per exact logical file or authority. Each
  row names `target_scope`, `write_ref_kind`, `write_ref`,
  `authority_ref`, `will_write`, `blocked`, lock reasons, review
  requirements, checkpoint ref, approval ticket ref, and exact change
  summary.
- `review_requirements`: flattened requirements the apply surface must
  satisfy before committing.
- `downgrade`: whether the preview was computed from exact, stale,
  mixed-version, alias-only, absent-capability, or read-only state.

Allowed write target classes are `user`, `workspace`, `folder`,
`profile`, `device`, `tenant`, `imported_settings`, `remote_target`,
`environment`, and `policy`. `policy` rows can appear only as blocked
or admin-owned targets in ordinary user previews.

Apply is non-conforming when:

- `fanout_summary.broadens_from_local_to_shared` is true and no
  `show_preview` or stronger review requirement is present;
- any `scope_writes[*].blocked` row is omitted from the preview;
- a `checkpoint_required` or `approval_required` row reaches apply
  without its ref;
- a local request silently writes workspace, folder, tenant, imported,
  remote-target, or policy authority;
- a policy-controlled row is rewritten as user, profile, or workspace
  preference.

## Downgrade rules

Downgrade rules keep inspection honest under degraded conditions.

| Downgrade | Trigger | Required posture |
|---|---|---|
| `mixed_version_settings` | Producer and consumer setting schema versions differ and the consumer cannot prove exact value shape. | Show value as compatible, mixed, or unsupported; block writes unless a migration preview exists. |
| `absent_capability` | Capability marker, package, extension, target runtime, broker, or policy epoch is absent. | Keep the last-known-valid value visible; mark writes blocked or repair-required. |
| `imported_alias` | Import or migration source uses a legacy id that can redirect for inspection but should not be persisted without review. | Show canonical id, alias source, and migration review action; do not silently write the alias. |
| `stale_effective_read` | Resolver snapshot is older than the freshness budget or remote target is disconnected. | Serve inspect-only or last-known-good state; require refresh before mutation. |
| `degraded_read_only` | Resolver can read enough metadata to explain but cannot prove safe mutation. | Mark packet read-only; write preview may show proposed targets but apply is blocked. |

Downgrade never deletes shadow-chain rows. It changes confidence and
write posture while preserving the evidence needed for support,
migration, sync, and export.

## Reuse across flows

Migration uses this grammar to show aliases, mixed-version rows, lossy
transforms, checkpoint requirements, and imported sources.

Sync uses this grammar to explain why local, profile, user, or
device values won, and why workspace, tenant, policy, or imported
values were omitted or refused.

Support and export use this grammar to emit source chains, blocked
scopes, downgrade state, and write target refs without raw secrets,
raw paths, hostnames, tenant names, or source content.

Settings inspectors use this grammar to answer four questions without
implementation logs:

1. Which scope won?
2. Which scopes were shadowed?
3. Which scopes were blocked or capped?
4. What exact files or authorities would a write change?
