# M5 schema-migration and compatibility-window registries

This lane is the schema-migration + downgrade implement lane over the frozen
[M5 settings-governance matrix](./m5_settings_resolver_contract.md) (the `migrate_schema` family). It turns the
*schema-migration-record* grammar (how a version change declares the old key / alias, new key, transform, lossy
fidelity, compatibility window, and rollback note) and the *compatibility-window* grammar (how an upgrade,
import, restore, or downgrade flow discloses whether stored meaning is inside its window, deprecated but
supported, or outside the window) into registry resolvers that produce export-safe, honest projections, so the
upgrade, import, restore, downgrade, docs, CLI, and support surfaces migrate one canonical configuration truth
instead of a per-version, hand-copied path. The migration record and the compatibility window are separated in
runtime and serialized state: the fidelity label, old key / alias, new key, transform, compatibility window,
rollback note, compare-before-apply reference, and migration provenance reference live on the migration record,
while the window source, supported version range, deprecation review, validation status, review state, docs
pointer, and last review revision live on the compatibility window, and a migration never implies full fidelity
when it is lossy or requires manual review because a downstream flow found that path easier.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_setting_schema_migration_and_compatibility_window_registries` (the authoritative
  validator).
- **Combined schema:**
  `schemas/config/m5-setting-schema-migration-and-compatibility-window-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/config/m5-setting-definition.schema.json`](../../schemas/config/m5-setting-definition.schema.json)
  and
  [`schemas/governance/schema_migration_record.schema.json`](../../schemas/governance/schema_migration_record.schema.json)
  as its canonical domain contracts.
- **Checked proof:**
  `artifacts/release/m5-setting-schema-migration-and-compatibility-window-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`).
- **Narrowed fixtures:** `fixtures/config/m5-setting-schema-migration-and-compatibility-window-registries/`
  (`schema_migration_beta_narrowed.json`, `compatibility_window_preview_narrowed.json`).

## Two registries

1. **Schema-migration record** (`resolve_schema_migration_record_entry`) — publishes one migration record per
   version change: the fidelity label and canonical label mode, the old key / alias, the new key, the transform,
   the compatibility window, the rollback note, the compare-before-apply reference, and the migration provenance
   reference. A clean entry names a canonical registry token, a classified fidelity label, and a
   settings-governance role, covers the canonical / accessible / audit resolution forms, publishes a complete
   record, keeps its fidelity label honest, and materializes the compare-before-apply surface before a lossy or
   manual-review migration applies. Otherwise it degrades honestly — a fidelity label that overstates what the
   transform preserves (or a lossy migration that hides its compare surface) degrades to
   `migration_overstates_fidelity_or_hides_compare_surface`.
2. **Compatibility window** (`resolve_compatibility_window_entry`) — keeps a deprecated or unsupported migration
   honest. A clean entry names a classified window class and provides the complete window-source /
   supported-version-range / deprecation-review / validation-status / review-state / docs-pointer /
   last-review-revision compatibility-window object; a record that would mask a deprecated window without
   disclosing its window source or leave an outside-window migration without disclosing the downgrade guidance
   degrades to `compatibility_window_masks_window_source_or_hides_downgrade_guidance`.

## Per-entry schema-migration reference

The fidelity label carries its canonical label mode, and the resolver publishes the full migration record, so
the registry — never a hand-copied per-version assumption — is the single source of truth.
`schema_migration_record_is_complete` rejects a record missing any field,
`migration_does_not_overstate_fidelity` rejects an overstated label or hidden compare surface, and
`compatibility_window_stays_honest` rejects a record that has masked its window source or hidden its downgrade
guidance.

| fidelity label | label mode | old key | new key | transform | rollback note | compare reference |
| --- | --- | --- | --- | --- | --- | --- |
| exact | exact_migration_label | `old.editor.fontSize` | `new.editor.font-size` | `transform.rename-key-verbatim` | `rollback.restore-v1-key` | `compare.before-apply-0007` |
| compatible | compatible_migration_label | `old.workbench.themeMode` | `new.workbench.theme-mode` | `transform.coerce-enum-compatible` | `rollback.restore-v1-enum` | `compare.before-apply-0007` |
| lossy | lossy_migration_label | `old.telemetry.sampleRateBuckets` | `new.telemetry.sample-rate` | `transform.collapse-buckets-lossy` | `rollback.restore-v2-buckets` | `compare.before-apply-0007` |

An overstated fidelity label degrades to `migration_overstates_fidelity_or_hides_compare_surface`, an incomplete
record degrades to `schema_migration_record_incomplete`, and a masked window degrades to
`compatibility_window_masks_window_source_or_hides_downgrade_guidance`, so an overstated label, an incomplete
record, or a masked window can never turn release evidence green.

## Acceptance criteria (proven by resolved examples)

- **Configuration artifacts carry explicit migration provenance and compatibility-window truth across claimed
  upgrade and import paths.** Clean migration entries cover the canonical exact / compatible / lossy /
  manual-review fidelity labels and the first upgrade / import / restore / downgrade / support flows, a
  record-incomplete example (a missing compare / provenance reference) degrades, and no clean migration entry
  published an incomplete record.
- **No downgrade / import path implies full fidelity when the migration is lossy or requires manual review.** A
  fidelity-overstate example and an unbound example degrade, a clean fidelity-honest migration entry is present,
  and no clean entry overstated fidelity — so a lossy or manual-review migration can never silently imply full
  fidelity because a downstream flow found that path easier.
- **Regression suites fail when schema changes alter stored meaning without a checked-in migration record and
  compare surface.** Clean compatibility-window entries cover the within-window / deprecated / outside-window
  classes with full resolution-form coverage while providing the complete record object, and a record that would
  mask a deprecated window or leave an outside-window migration without disclosing its downgrade guidance
  degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_setting_schema_migration_and_compatibility_window_registries -- support-export
cargo run -p aureline-ui --example dump_m5_setting_schema_migration_and_compatibility_window_registries -- csv
cargo run -p aureline-ui --example dump_m5_setting_schema_migration_and_compatibility_window_registries -- report
cargo run -p aureline-ui --example dump_m5_setting_schema_migration_and_compatibility_window_registries -- migration-table
cargo run -p aureline-ui --example dump_m5_setting_schema_migration_and_compatibility_window_registries -- fixture-schema-migration-beta-narrowed
cargo run -p aureline-ui --example dump_m5_setting_schema_migration_and_compatibility_window_registries -- fixture-compatibility-window-preview-narrowed
```
