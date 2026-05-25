# Settings UI with the effective-configuration inspector — contract

This is the reviewer-facing companion for the stable lane that finalizes the
**settings UI** as a governed launch property: one governed record per settings
posture that binds **one effective-setting record per visible setting** (value,
winning scope, lock reason, restart posture, and the full setting-definition
registry row), a **shadow contributor chain** naming the active profile, the
temporary profile, machine-local, synced, workspace, and policy-owned
contributors, **scope-explicit previewable writes**, **cross-surface parity**
across the desktop UI / CLI inspect / Help/About / diagnostics export /
migration review, and a **profile-switch review** — all to a public claim
ceiling and an automatic narrow-below-Stable verdict.

This lane is a settings-side certification that *projects* the live settings
runtime. It does not invent a private "configuration" read; it proves the
effective-settings object model resolves and stays attributable to one source of
truth across surfaces. It builds on the schema registry
(`aureline_settings::schema::SchemaRegistry`), the precedence engine and
locked-write flow (`aureline_settings::resolver`), and the shared inspector
projection (`aureline_settings::inspector`).

Do not clone status text from this doc — ingest the canonical machine sources:

- Records / fixtures:
  [`/fixtures/ux/m4/finalize-the-settings-ui-with-effective-value-inspector/`](../../../fixtures/ux/m4/finalize-the-settings-ui-with-effective-value-inspector/)
- Schema:
  [`/schemas/ux/finalize-the-settings-ui-with-effective-value-inspector.schema.json`](../../../schemas/ux/finalize-the-settings-ui-with-effective-value-inspector.schema.json)
- Release-evidence packet:
  [`/artifacts/ux/m4/finalize-the-settings-ui-with-effective-value-inspector.md`](../../../artifacts/ux/m4/finalize-the-settings-ui-with-effective-value-inspector.md)
- Typed source: `aureline_settings::settings_ui_stable` (`model`, `corpus`)
- Headless emitter: `aureline_settings_finalize_settings_ui_stable`
- Replay + invariant gate:
  `crates/aureline-settings/tests/settings_ui_stable_fixtures.rs`

## Why one governed certification record

Settings are read and written from many surfaces — the desktop settings UI, a
CLI / headless inspect, Help/About, a diagnostics or support export, and
migration / import review. If each surface re-derives "what value is active,
which scope won, why it is locked, and whether it needs a restart" from its own
private read, the surfaces drift: the UI shows one winner, the export shows
another, and a policy-locked write fails with a toast-only reason that support
cannot reconstruct. The result is a green "settings are truthful" claim that is
really an average over surfaces that each explain a setting a little differently,
with a flat effective value that hides the active profile, the temporary
profile, the synced artifact, and the policy ceiling behind it.

A `settings_ui_certification_record` closes that gap. For one settings posture
it binds:

- **One effective-setting record per visible setting.** Every
  `effective_settings[]` row resolves through a single record that explains the
  winning value, the `winning_scope`, the `lock_state` / `lock_reason`, the
  `restart_posture`, and the setting-definition registry fields — `declared_type`,
  `allowed_scopes`, `default_value_preview`, `migration_aliases`,
  `sensitivity_class`, `redaction_class`, `preview_class`,
  `capability_dependencies`, and `help_doc_ref`. The row is projected from the
  live resolver and inspector, never re-derived.
- **A shadow contributor chain, never a flat value.** Each row's `shadow_chain[]`
  classifies every contributor with a `contributor_class` — built-in default,
  channel default, **active profile**, **temporary profile**, **machine-local**,
  **synced**, **workspace**, folder / language overrides, and the **policy-owned**
  ceiling. The certification's `contributor_coverage[]` rollup proves the chain
  can expose the active profile, temporary profile, machine-local, synced,
  workspace, and policy-owned contributors in one place.
- **Scope-explicit previewable writes.** Every `previewable_writes[]` row names
  the `target_scope`, the `target_artifact_ref` that would receive the write, the
  `blocked_write_reason` and a `diagnostics_entry_ref` Diagnostics Center entry
  point when denied, the `restart_impact`, and any `lifecycle_dependency` — before
  commit. A denied write that hides its reason or its escalation path narrows the
  posture.
- **Cross-surface parity.** One `surface_parity[]` row per desktop UI, CLI
  inspect, Help/About, diagnostics / support export, and migration / import
  review, each proving it `consumes_shared_record` and does not `clones_prose`.
- **A profile-switch review.** `profile_switch_review` summarizes the
  `immediate_changes[]`, the `restart_required_changes[]`, the
  `excluded_machine_specific[]` state, the `narrowing_effects[]`, and whether a
  rollback checkpoint is created before apply.

## The public claim ceiling and automatic narrowing

`pillars` are *derived* from the rows, never asserted. `claim_ceiling` records
what the posture is allowed to claim; the builder refuses to mint a record whose
ceiling exceeds its proof. `stable_qualification` then narrows the posture below
Stable with a named `narrowing_reasons[]` entry whenever a pillar fails or the
lowest surface marker is below Stable — so a posture never inherits Stable by
adjacency.

| Narrowing reason | Meaning |
| --- | --- |
| `setting_resolution_incomplete` | A visible setting does not resolve through one explaining record. |
| `shadow_chain_flattened` | The shadow chain does not expose the required contributor classes. |
| `write_not_scope_explicit` | A previewable write is not scope-explicit or hides a blocked reason. |
| `surface_clones_prose` | A surface clones prose instead of consuming the shared record. |
| `profile_switch_review_incomplete` | The profile-switch review does not summarize the switch coherently. |
| `setting_id_not_canonical` | A setting_id is not canonical in exports. |
| `surface_not_yet_stable` | The lowest surface marker is below Stable. |

## Reachability, accessibility, and availability

The same record is reachable from the settings inspector, the command palette,
the status bar, and a menu command (`routes[]`), keyboard-first, and the recovery
routes (`recovery_routes[]`) hold across normal, high-contrast, and zoomed
layouts (`accessibility.layout_modes[]`). Every record stays available without an
account or managed services.

## Regenerating the records

```sh
cargo run -q -p aureline-settings \
  --bin aureline_settings_finalize_settings_ui_stable -- emit-fixtures \
  fixtures/ux/m4/finalize-the-settings-ui-with-effective-value-inspector
```

The replay gate
`crates/aureline-settings/tests/settings_ui_stable_fixtures.rs` fails if the
on-disk JSON drifts from the in-code projection.
