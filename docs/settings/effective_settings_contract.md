# Effective settings contract

This page is the reviewer-facing landing point for the runtime
**effective-settings substrate**: the schema registry, the
precedence engine, and the locked-write flow. It tells a reviewer
how to ask the running shell what a setting resolves to, where the
value came from, what was shadowed, and what happens if a write is
attempted against a locked or shadowed scope — without reading
implementation logs.

The canonical truth lives in two crate modules:

- [`/crates/aureline-settings/src/schema/`](../../crates/aureline-settings/src/schema/)
  — `setting_id`, value type, default, allowed scopes, restart
  posture, lifecycle, and the catalog of setting definitions.
- [`/crates/aureline-settings/src/resolver/`](../../crates/aureline-settings/src/resolver/)
  — the precedence engine, the shadow-chain projection, and the
  typed `attempt_write` flow that returns a verdict plus typed
  denial reason instead of dropping the request silently.
- [`EffectiveSettingRecord`](../../crates/aureline-settings/src/resolver/effective.rs)
  — the serialized cross-surface record used by UI, CLI, sync,
  policy, docs/help, and support consumers. It carries the shadow
  chain, write posture, capability-dependency state, control-stack
  trace, and restart posture in one object.
- [`/crates/aureline-settings/src/inspector/`](../../crates/aureline-settings/src/inspector/)
  — the alpha inspector projection consumed by CLI inspection,
  settings UI rows, help deep links, write previews, and support
  export. See [`docs/settings/inspector_alpha.md`](./inspector_alpha.md).

Composed contracts (truth source for vocabulary and shape):

- [`docs/settings/precedence_lock_and_write_scope_contract.md`](./precedence_lock_and_write_scope_contract.md)
  — user-visible scope grammar, lock-state reasons, and write-scope
  review packets. The resolver implements the precedence ladder
  named there; the shadow chain renders the same relation
  vocabulary (`winner`, `shadowed`, `capped`, `policy_ceiling`).
- [`docs/settings/schema_registry_seed.md`](./schema_registry_seed.md)
  — publishing/consumption posture for the cross-tool boundary
  schemas. The runtime resolver is the schema **of record**; this
  page does not reinvent that vocabulary.
- [`docs/settings/effective_control_stack_contract.md`](./effective_control_stack_contract.md)
  — broader control stack (defaults, signed admin bundles,
  optional managed override, ceilings). The settings layer of that
  stack is what the resolver projects.

If this page disagrees with the source contracts, the source
contracts win and this page must be updated in the same change.

## Precedence ladder

Ordinary scopes (low to high). Every scope is layered; the highest
ordinary scope with a value wins, then the policy ceiling (if any)
narrows or pins the result.

| Rank | Scope token                       | Notes                                                |
|-----:|-----------------------------------|------------------------------------------------------|
|   10 | `built_in_default`                | Always present; default from the setting definition. |
|   15 | `channel_or_experiment_default`   | Release channel / experiment override.               |
|   40 | `imported_profile_default`        | Imported profile (Aureline, VS Code, JetBrains, …).  |
|   60 | `user_global`                     | User-global preference.                              |
|   70 | `machine_specific`                | Machine-local; never carried by sync.                |
|   80 | `workspace`                       | Workspace-owned setting.                             |
|   90 | `folder_or_module_override`       | Folder / module / workset override.                  |
|   95 | `language_override`               | Per-language override.                               |
|  120 | `session_override`                | Ephemeral session override.                          |
|  900 | `admin_policy_narrowing` *(cap)*  | Authority ceiling; never ordinary candidate.         |

Admin policy is **not** an ordinary preference. It applies after the
layered winner is chosen and may only narrow, lock, or constrain.
The shadow chain row for the policy ceiling stays visible whether
or not it actually capped the layered winner.

## Locked-write flow

Every write goes through `EffectiveSettingsResolver::attempt_write`,
which returns a typed [`WriteAttemptOutcome`](../../crates/aureline-settings/src/resolver/engine.rs)
with a schema-backed verdict:

- `allowed` — the overlay was recorded; `effective_after` shows the
  next reader's view.
- `allowed_with_restart` — the write is admitted and the definition
  names the required restart or reload posture.
- `allowed_with_preview` — the write is admitted only through a
  preview acknowledgement flow.
- `allowed_with_rollback_checkpoint` — the write is admitted after
  a rollback checkpoint exists.
- `allowed_with_rollback_checkpoint_and_approval` — the write is
  admitted after both checkpoint and approval handles exist.
- `allowed_requires_approval_ticket` — the write is admitted only
  through an approval-ticket path.
- `denied` — the overlay was **not** recorded; `denial_reason` names
  the typed reason, and `effective_after` quotes the still-current
  shadow chain so the surface can explain the denial without
  re-resolving.

Frozen denial reason codes:

| Code                        | Triggered by                                                      |
|-----------------------------|-------------------------------------------------------------------|
| `unknown_setting`           | `setting_id` is not registered.                                   |
| `scope_not_allowed`         | Target scope is not in the definition's `allowed_scopes`.         |
| `policy_locked`             | Admin policy pins the value; proposed value is not the pin.       |
| `policy_constrained_value`  | Admin policy admits a narrowed set; proposed value is outside it. |
| `capability_dependency_unmet` | A declared feature, identity, trust, policy, workspace, extension, or credential dependency is unavailable. |
| `validation_failed`         | Type / range / enum check failed (carries `detail`).              |
| `retired_setting`           | Setting is retired and refuses writes.                            |

A denial is never silent. Surfaces MUST quote the typed reason; a
"could not save" message without a code is a bug.

## Protected walk

Reviewers can exercise the substrate end to end through the
`aureline-shell` consumer card. Each step is automated by a
unit test in
[`crates/aureline-shell/src/state_cards/effective_settings_card.rs`](../../crates/aureline-shell/src/state_cards/effective_settings_card.rs);
manual reproduction is below.

1. **Inspect a setting across scopes.** Build a resolver from the
   seed catalog
   ([`SchemaRegistry::with_seed_catalog`](../../crates/aureline-settings/src/schema/registry.rs)),
   register a `user_global` overlay for `editor.tab_size`, then a
   `workspace` overlay. The shell card returned by
   `materialize_effective_settings_card` quotes the winning scope
   (`workspace`), the source label, and the full shadow chain
   (`built_in_default → user_global → workspace`).

2. **Attempt a locked write.** Register an
   `admin_policy_narrowing` overlay with a `SingleValue` constraint
   on `security.ai.egress_policy`. Calling
   `preview_locked_write` (or `attempt_write` for real) with a
   value outside the pinned set returns `verdict=denied`,
   `denial_reason_code=policy_locked`, and the same shadow chain
   the inspector card would show — including the active policy
   ceiling row.

3. **Verify precedence, shadow chain, and lock reason.** Both the
   inspector card and the locked-write review record carry
   `lock_state`, `lock_reason`, `restart_posture`, and the typed
   shadow rows. Surfaces render those verbatim; nothing in the
   shell adds a private label.

## Failure drill

The required failure drill is "attempt a locked or shadowed
settings write and confirm the resolver surfaces scope and lock
source instead of accepting it silently". The dedicated unit tests
that pin this behavior are:

- [`write_under_policy_lock_is_denied_with_typed_reason`](../../crates/aureline-settings/src/resolver/engine.rs)
  — proves a policy-locked write returns `policy_locked` and the
  shadow chain still surfaces the policy row.
- [`write_violating_validation_is_denied_without_recording`](../../crates/aureline-settings/src/resolver/engine.rs)
  — proves a value outside the declared range is rejected and the
  overlay is **not** mutated.
- [`write_to_unsupported_scope_is_denied`](../../crates/aureline-settings/src/resolver/engine.rs)
  — proves a write to a scope not in the definition is rejected.
- [`preview_locked_write_quotes_typed_denial_reason`](../../crates/aureline-shell/src/state_cards/effective_settings_card.rs)
  — proves the shell preview surface materialises the typed reason.

Together they show that the resolver never silently accepts a
locked or shadowed write and never invents a generic "could not
save" string.

## File-based and exportable

`EffectiveSettingsResolver::export_state` returns a deterministic
JSON value that round-trips through `import_state`. The export is
file-based; no sync transport, managed account, or remote service is
required to capture or replay settings state. Support packets and
fixtures should embed the exported overlay set rather than recording
ad-hoc snapshots.

## Cross-Surface Fixtures

`fixtures/config/effective_settings_shadow_chain/` contains a
policy-locked effective record and a scope-explicit write preview.
They are generated from the headless inspector path and prove that
support, CLI, sync, and UI projections can quote the same source
chain and destination preview.

## Out of scope for this seed

- Sync transport, conflict packets, or device-binding lifecycle —
  see [`docs/settings/sync_and_device_registry_seed.md`](./sync_and_device_registry_seed.md).
- Settings UI rendering — surfaces consume the card record verbatim.
- Full profile-import UI — alias retention and runtime alias
  redirection are present in the schema registry and inspector path.
- Remote/container settings authority and target-aware resolution
  — the resolver intentionally does not model `remote_target`
  scope yet; later work adds that lane.

## Where to register evidence

Validation-lane captures, build identity, owner, and artifact links
register in the release evidence index. Use this page as the
reviewer-facing landing point when a human-readable link is needed.
