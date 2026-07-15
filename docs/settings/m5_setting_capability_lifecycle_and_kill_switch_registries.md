# M5 capability-lifecycle and kill-switch registries

This lane is the capability-lifecycle / kill-switch implement lane over the frozen
[M5 settings-governance matrix](./m5_settings_resolver_contract.md) (the `rollout_capability` family). It turns
the *capability-record* grammar (how a capability record, Labs enrollment, rollout plan, and dependency marker
declare the lifecycle state, the accountable owner, the scope, the review / expiry window, the enabled posture,
the artifact dependency marker, the fallback, and the rollback note a capability carries) and the
*kill-switch-record* grammar (how a kill-switch or policy-disable record names the disabling source, the disabled
timestamp, the preserved user-authored data, the self-explanation, the capability dependency, the fallback, and
the last revision for a kill-switch / policy-disable / dependency-unavailable / review-expired / manual-opt-out
disable) into registry resolvers that produce export-safe, honest projections, so the settings, docs / help,
bundle, import-apply, docs, CLI, support, and claim-publication surfaces resolve one canonical lifecycle truth
instead of a scattered Labs / Preview / DisabledByPolicy label path. The capability record and the kill-switch
ledger are separated in runtime and serialized state: the lifecycle class, owner, scope, review / expiry, enabled
posture, dependency marker, fallback, and rollback note live on the capability record, while the disabling source,
disabled timestamp, preserved-data reference, explanation reference, capability dependency, fallback reference,
and last ledger revision live on the kill-switch record, and a kill switch or policy disable never loses
user-authored data or hides its cause because a downstream surface found that path easier.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_setting_capability_lifecycle_and_kill_switch_registries` (the authoritative
  validator).
- **Combined schema:**
  `schemas/config/m5-setting-capability-lifecycle-and-kill-switch-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/config/m5-capability-lifecycle.schema.json`](../../schemas/config/m5-capability-lifecycle.schema.json)
  and
  [`schemas/governance/capability_lifecycle.schema.json`](../../schemas/governance/capability_lifecycle.schema.json)
  as its canonical domain contracts.
- **Checked proof:**
  `artifacts/release/m5-setting-capability-lifecycle-and-kill-switch-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`).
- **Narrowed fixtures:** `fixtures/config/m5-setting-capability-lifecycle-and-kill-switch-registries/`
  (`capability_lifecycle_beta_narrowed.json`, `kill_switch_preview_narrowed.json`).

## Two registries

1. **Capability record** (`resolve_capability_record_entry`) — publishes one capability record per capability: the
   lifecycle class and canonical state mode, the owner, the scope, the review / expiry window, the enabled
   posture, the dependency marker, the fallback, and the rollback note. A clean entry names a canonical registry
   token, a classified lifecycle class, and a settings-governance role, covers the canonical / accessible / audit
   resolution forms, publishes a complete record, keeps its dependency marker published, and publishes a fallback
   before a protected (Labs / Preview / Beta) capability is claimed. Otherwise it degrades honestly — a hidden
   dependency marker (or a protected capability that published no fallback) degrades to
   `capability_hides_dependency_or_lacks_fallback`.
2. **Kill-switch record** (`resolve_kill_switch_record_entry`) — keeps a kill-switch / policy-disable /
   dependency-unavailable / review-expired / manual-opt-out disable reconstructable. A clean entry names a
   classified kill-switch class and provides the complete disabling-source / disabled-timestamp /
   preserved-data-reference / explanation-reference / capability-dependency / fallback-reference /
   last-ledger-revision kill-switch-record object; a record that would hide a kill-switch / policy-disable cause
   without disclosing its reason or leave preserved user-authored data without disclosing that it stays preserved
   degrades to `kill_switch_hides_cause_or_drops_data_preservation`.

## Per-entry capability reference

The lifecycle class carries its canonical state mode, and the resolver publishes the full capability record, so the
registry — never a hand-copied Labs / Preview label — is the single source of truth. `capability_record_is_complete`
rejects a record missing any field, `capability_does_not_hide_dependency` rejects a hidden dependency marker or a
protected capability without a fallback, and `kill_switch_record_preserves_data_and_explains` rejects a record that
has hidden its cause or dropped its user-data preservation.

| lifecycle class | state mode | owner | enabled posture | dependency marker | fallback | rollback note |
| --- | --- | --- | --- | --- | --- | --- |
| labs | labs_capability | `owner.ai-platform-team` | `posture.opt-in-off-by-default` | `dependency.marker-ai-runtime-v3` | `fallback.classic-completion` | `rollback.disable-restores-classic` |
| preview | preview_capability | `owner.collab-team` | `posture.opt-in-preview` | `dependency.marker-collab-relay-v2` | `fallback.single-user-editing` | `rollback.disable-keeps-local-edits` |
| beta | beta_capability | `owner.search-team` | `posture.opt-in-beta` | `dependency.marker-index-service-v4` | `fallback.lexical-search` | `rollback.disable-restores-lexical` |

A hidden dependency marker degrades to `capability_hides_dependency_or_lacks_fallback`, an incomplete record
degrades to `capability_record_incomplete`, and a hidden kill-switch ledger degrades to
`kill_switch_hides_cause_or_drops_data_preservation`, so a hidden dependency, an incomplete record, or a hidden
kill-switch cause can never turn release evidence green.

## Acceptance criteria (proven by resolved examples)

- **Lifecycle states, dependency markers, and kill-switch posture remain canonical across claimed M5 settings,
  docs, bundle, import, and export surfaces.** Clean capability entries cover the canonical labs / preview / beta /
  generally-available / graduated / deprecated classes and the first settings / docs-help / bundle / import-apply /
  support flows, a record-incomplete example degrades, and no clean capability entry published an incomplete
  record.
- **No stable-facing surface can depend on a hidden Labs / Preview capability without an explicit dependency
  marker and fallback.** A dependency-hide example and an unbound example degrade, a clean dependency-published
  capability entry is present, and no clean entry hid its dependency marker — so a stable surface can never depend
  on a hidden capability because a downstream flow found that path easier.
- **Regression suites fail when two consumers describe the same capability state, dependency, or kill-switch
  outcome differently.** Clean kill-switch entries cover the kill-switch / policy-disabled / dependency-unavailable
  / review-expired / manual-opt-out classes with full resolution-form coverage while providing the complete record
  object, and a record that would hide a kill-switch / policy-disable cause or leave preserved user-authored data
  without disclosing its preservation degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_setting_capability_lifecycle_and_kill_switch_registries -- support-export
cargo run -p aureline-ui --example dump_m5_setting_capability_lifecycle_and_kill_switch_registries -- csv
cargo run -p aureline-ui --example dump_m5_setting_capability_lifecycle_and_kill_switch_registries -- report
cargo run -p aureline-ui --example dump_m5_setting_capability_lifecycle_and_kill_switch_registries -- capability-table
cargo run -p aureline-ui --example dump_m5_setting_capability_lifecycle_and_kill_switch_registries -- fixture-capability-lifecycle-beta-narrowed
cargo run -p aureline-ui --example dump_m5_setting_capability_lifecycle_and_kill_switch_registries -- fixture-kill-switch-preview-narrowed
```
