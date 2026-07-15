# M5 Settings-Governance Shared Consumers: One Registry Across Surfaces

**Status:** Stable · B143 consumer-adoption capstone
**Module:** `aureline_ui::m5_settings_governance_shared_consumers_one_registry_across_surfaces`
**Schema:** [`schemas/config/m5-settings-governance-shared-consumers.schema.json`](../../schemas/config/m5-settings-governance-shared-consumers.schema.json)
**Proof:** [`artifacts/release/m5-settings-governance-shared-consumers-proof/`](../../artifacts/release/m5-settings-governance-shared-consumers-proof/)
**Fixtures:** [`fixtures/config/m5-settings-governance-shared-consumers/`](../../fixtures/config/m5-settings-governance-shared-consumers/)

This lane is the consumer-adoption capstone for the five governed configuration-runtime families frozen in
the [settings-governance matrix](m5_settings_resolver_contract.md) and implemented by the setting-definition
/ effective-setting, write-intent / policy-constraint, sync-conflict / device-action, schema-migration /
compatibility-window, and capability-lifecycle / kill-switch lanes. It binds each shared settings-governance
family to the concrete settings-resolver, shell, sync-service, policy-service, capability-service,
diagnostics, docs / help, CLI / export, and support-export consumers that render it — the GUI settings, CLI
/ headless inspect, Project Doctor, support export, import / export, and policy-explainer surfaces — and
proves, by fixtures rather than screenshots, that the same configuration profile presents the **same
registry** everywhere it appears.

## Why this exists

The sheet already hardens install / configuration portability and sync-device truth, structured config and
signed policy-bundle lanes, command-lifecycle and experiment disclosure, settings / capability / evidence
components, and portable-state migration / export foundations, but it left Aureline's actual settings
resolver, sync conflict engine, and capability-lifecycle runtime objects too implicit for each claimed
configuration-bearing surface. This lane wires those rules into the daily-driver configuration surfaces so
effective-setting state, write intent, migration posture, sync conflict class, device lineage, and
capability lifecycle cannot drift between the GUI settings, CLI / headless inspect, Project Doctor, support
export, import / export, and policy explainer: every surface consumes the shared registry rather than
private wording or hand-copied settings-row copy. When two consumers describe the same configuration state
differently, the regression suite fails.

## The three honesty axes

1. **Reuse.** Each of the five settings-governance families is adopted by **at least two distinct
   consumers**, so a family is proven shared settings-resolver infrastructure rather than a one-surface fork
   of setting-definition, write-intent, sync-conflict, or capability-lifecycle copy.
2. **One registry / no drift.** For a given configuration profile every consumer surface presents the
   identical six-word grammar — `settings_governance_role_word`, `family_word`, `registry_reference_word`,
   `resolution_context_word`, `surface_context_word`, and `evidence_continuity_word`. The role word must be a
   token from the frozen `M5SettingsGovernanceRole` vocabulary (`setting_definition`, `effective_resolution`,
   `write_intent`, `policy_constraint`, `sync_conflict`, `schema_migration`, `capability_lifecycle`), so no
   surface rewrites a role in its own words. A surface may narrow *how much* it shows across desktop,
   compact, remote, and exported representations, but never reword the grammar per surface — and a role that
   carries write-intent, policy-constraint, sync-conflict, or capability-lifecycle meaning may never let a
   surface recycle a retired setting ID, rewrite a scoped write into a broader scope, silently overwrite
   locked or machine-only state during sync, hide a lifecycle or experiment dependency behind unpublished
   markers, or hide a kill-switch or policy-disable cause behind generic unavailable copy.
3. **Map back to one family.** Support and CLI/export consumers point at the canonical per-domain schema and
   the frozen matrix by id, so an exported packet always maps a configuration surface back to one shared
   contract family.

## Guardrails (each MUST be false on every binding)

- `recycles_a_retired_setting_id`
- `rewrites_a_scoped_write_into_a_broader_scope`
- `silently_overwrites_locked_or_machine_only_state_during_sync`
- `hides_lifecycle_or_experiment_dependency_behind_unpublished_markers`
- `hides_kill_switch_or_policy_disable_cause_behind_generic_unavailable_copy`

## Narrowing is disclosed, never hidden

A compact, remote, or exported representation carries an explicit `narrow_note` naming the reason, the
preserved grammar, and the next action; a remote representation names its remote source, and an exported
representation names its export-safe detail boundary rather than collapsing the profile out of view. When a
route only supports a compact projection, remote-backed inspect, or an export-safe redaction rather than a
full desktop disclosure, the narrowing is surfaced consistently. Stale proof or a missing canonical
reference **narrows** the claim via a `SettingsGovernanceSharedConsumersDowngradeTrigger` rather than hiding
the family.

## Seeded coverage

Five configuration profiles — one per family — fan out to fifteen consumer bindings covering all nine
consumers and all four representations:

| Family | Role | Consumers |
| --- | --- | --- |
| `resolve_setting` | `effective_resolution` | settings resolver, shell, CLI export |
| `write_setting` | `write_intent` | policy service, shell, support export |
| `sync_scope` | `sync_conflict` | sync service, diagnostics, docs/help |
| `migrate_schema` | `schema_migration` | capability service, diagnostics, sync service |
| `rollout_capability` | `capability_lifecycle` | docs/help, capability service, support export |

Two checked narrowed fixtures prove the grammar survives compact / remote and exported / redacted forms
without rewording.

## Regenerating the proof

```text
cargo run -p aureline-ui --example dump_m5_settings_governance_shared_consumers -- support-export
cargo run -p aureline-ui --example dump_m5_settings_governance_shared_consumers -- csv
cargo run -p aureline-ui --example dump_m5_settings_governance_shared_consumers -- report
cargo run -p aureline-ui --example dump_m5_settings_governance_shared_consumers -- fixture-compact-remote-narrowed
cargo run -p aureline-ui --example dump_m5_settings_governance_shared_consumers -- fixture-exported-redaction-narrowed
```

The example is the only mint-from-truth path for the checked support export, matrix CSV, Markdown summary,
and narrowed fixtures; the module tests fail if any drifts from the seed builder.
