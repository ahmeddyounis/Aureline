# Settings UI certification — release evidence

Reviewer-facing evidence packet for the lane that finalizes the **settings UI,
effective-configuration inspector, setting-definition registry, lock reasons,
restart posture, and previewable writes** on claimed-stable desktop surfaces:
one canonical record per settings posture that binds one effective-setting record
per visible setting, a shadow contributor chain naming the active profile /
temporary profile / machine-local / synced / workspace / policy-owned
contributors, scope-explicit previewable writes, cross-surface parity, a
profile-switch review, a public claim ceiling, an automatic narrow-below-Stable
verdict, recovery and route parity across the settings inspector / command
palette / status bar / menus, accessibility across normal / high-contrast /
zoomed layouts, and rows that stay available without an account or managed
services.

Canonical machine sources (do not clone status text from this packet — ingest the JSON):

- Records / fixtures: [`/fixtures/ux/m4/finalize-the-settings-ui-with-effective-value-inspector/`](../../../fixtures/ux/m4/finalize-the-settings-ui-with-effective-value-inspector/)
- Schema: [`/schemas/ux/finalize-the-settings-ui-with-effective-value-inspector.schema.json`](../../../schemas/ux/finalize-the-settings-ui-with-effective-value-inspector.schema.json)
- Companion doc: [`/docs/ux/m4/finalize-the-settings-ui-with-effective-value-inspector.md`](../../../docs/ux/m4/finalize-the-settings-ui-with-effective-value-inspector.md)
- Typed source: `aureline_settings::settings_ui_stable` (`model`, `corpus`)
- Headless emitter: `aureline_settings_finalize_settings_ui_stable`
- Replay + invariant gate: `crates/aureline-settings/tests/settings_ui_stable_fixtures.rs`

## The claimed-stable matrix

| Record | Posture | Claim | Surface marker | Narrowing reason |
| --- | --- | --- | --- | --- |
| `nominal.json` | nominal | **stable** | stable | — |
| `temporary_profile_active.json` | temporary profile active | **stable** | stable | — |
| `surface_in_preview.json` | labs setting surface still in Preview | preview (narrowed) | preview | `surface_not_yet_stable` |
| `prose_clone_drill.json` | migration review clones prose | beta (narrowed) | beta | `surface_clones_prose` |

Coverage verdict: **2 Stable, 2 narrowed**. Each narrowed row names a reason and
drops below the launch cutline rather than inheriting an adjacent green row.

## Acceptance criteria → evidence

- **Users and admins can inspect the active value, scope winner, restart
  requirement, lock source, and escalation path for a blocked or policy-owned
  setting without leaving the settings flow.** Every `effective_settings[]` row
  carries `winning_value_summary`, `winning_scope`, `restart_posture`,
  `lock_state` / `lock_reason`, and an `escalation_path_ref` Diagnostics Center
  entry point; the policy-owned `security.ai.egress_policy` row resolves through
  the `admin_policy_narrowing` scope with a `policy_owned` winning contributor and
  a `capped` row for the shadowed user value.
- **Every visible setting resolves through one effective-setting record and can
  explain what value won, why it won, why it is locked or allowed, and whether
  restart or live-apply applies.** `pillars.every_setting_resolves_one_record` is
  derived from each row's `resolves_through_one_record`, which requires a winning
  scope, a lock state and reason, a restart posture, a non-empty shadow chain, and
  a canonical `effective_record_ref`.
- **Settings layering exposes active profile, temporary-profile, machine-local,
  synced, workspace, and policy-owned contributors in one inspectable shadow
  chain.** `contributor_coverage[]` rolls up the `contributor_class` of every
  shadow-chain row; the Stable postures expose `active_profile`,
  `temporary_profile`, `machine_local`, `synced_profile`, `workspace`, and
  `policy_owned`.
- **Previewable writes remain scope-explicit and show the target artifact,
  blocked-write reason when denied, restart impact, and lifecycle dependency.**
  `previewable_writes[]` exercises an allowed write, an
  `allowed_with_rollback_checkpoint_and_approval` high-risk write, a
  `policy_locked_value` denial, and a `scope_not_allowed_for_setting` denial. Each
  carries `scope_explicit`, a `target_artifact_ref`, a `diagnostics_entry_ref`,
  and — for the AI egress control — a `capability:identity_mode_required:...`
  lifecycle dependency.
- **Help, support, and migration surfaces consume the same setting-definition
  registry instead of cloning prose.** `surface_parity[]` covers the desktop UI,
  CLI inspect, Help/About, diagnostics / support export, and migration / import
  review; on the Stable postures each `consumes_shared_record` and does not
  `clones_prose`.
- **Profile-switch review summarizes immediate changes, restart-required deltas,
  excluded machine-specific state, narrowing effects, and rollback checkpoint
  posture before apply.** `profile_switch_review` is a real diff between two
  resolver states: `editor.tab_size` and `ui.theme` change immediately, the AI
  egress control is a restart-required delta and a narrowing effect under the
  active policy, `vfs.watcher.fallback_polling_ms` is excluded as machine-local,
  and a rollback checkpoint is created before apply.
- **Stable setting_ids and migration aliases stay canonical in exports.** Each
  row carries `setting_id_canonical` and a canonical id echoed in
  `upstream.certified_setting_ids[]`; `security.ai.egress_policy` keeps its
  `ai.network.egress_policy` migration alias.
- **Any surface still lacking stable qualification is automatically narrowed
  below Stable.** `surface_in_preview.json` narrows to `preview` via
  `surface_not_yet_stable`; `prose_clone_drill.json` narrows to `beta` via
  `surface_clones_prose`. Both carry `honesty_marker_present = true` and a bounded
  `waiver_ref` on the narrowed row or surface.

## Guardrails honored

- No surface can drift its settings explanation from the shared record without
  the lane detecting it: the prose-clone drill surfaces `surface_clones_prose` and
  forces the claim below Stable.
- A denied write always carries a typed `blocked_write_reason` and a
  `diagnostics_entry_ref` — never a toast-only truth.
- No setting relies on a flat value: `contributor_coverage[]` proves the shadow
  chain exposes the profile, sync, machine-local, and policy-owned contributors.
- The record stays available without an account or managed services
  (`available_without_account`, `available_without_managed_services`).

## Reproduce

```sh
cargo run -q -p aureline-settings \
  --bin aureline_settings_finalize_settings_ui_stable -- index

cargo run -q -p aureline-settings \
  --bin aureline_settings_finalize_settings_ui_stable -- emit-fixtures \
  fixtures/ux/m4/finalize-the-settings-ui-with-effective-value-inspector

cargo test -p aureline-settings --test settings_ui_stable_fixtures
```
