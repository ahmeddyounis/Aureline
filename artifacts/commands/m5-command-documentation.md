# M5 command documentation: canonical command-record docs, parity cards, fresh examples, and copy-safe alias/deprecation export across every claimed M5 command surface

Generated from the seeded packet in
[`crate::m5_command_documentation`](../../crates/aureline-shell/src/m5_command_documentation/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_command_documentation -- markdown > \
  artifacts/commands/m5-command-documentation.md
```

- Packet id: `m5-command-documentation:stable:0001`
- Source schema ref: `schemas/commands/m5-command-documentation.schema.json`
- Certifies matrix packet: `m5-discoverability-affordances:stable:0001`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Required documentation dimensions: `documentation_record`, `cross_surface_naming`, `example_freshness`, `doc_export_parity`
- Documentation-record fields published: `command_id`, `primary_label`, `aliases`, `lifecycle_state`, `supported_surfaces`, `invocation_schema_summary`, `side_effect_risk_class`, `result_rollback_semantics`
- Parity cards rendered: `menu`, `button`, `palette`, `cli_headless`, `ai_tool`, `recipe`, `voice_companion_hint`
- Derivation anchors: `docs_help_anchor`, `shortcut_notation`, `accessibility_narration_hint`
- Surface families certified: 10
- Green (full conformance): 6
- Yellow (auto-narrowed): 4
- Red (blocked): 0
- All rows publishable (stable-claim gate): `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-30T00:00:00Z`

## Documentation rows

| Surface family | Status | Documentation record | Cross-surface naming | Example freshness | Doc export | Lifecycle | Headless | Waiver |
| -------------- | ------ | -------------------- | -------------------- | ----------------- | ---------- | --------- | -------- | ------ |
| Menu-bar item | `green` | `command_record_examples_and_lifecycle_certified` | `canonical_naming_and_replacement_stable` | `canonical_examples_fresh_and_not_alias_only` | `command_id_and_replacement_reconstructable` | `stable` | `true` | — |
| Menu group / submenu | `green` | `command_record_examples_and_lifecycle_certified` | `canonical_naming_and_replacement_stable` | `canonical_examples_fresh_and_not_alias_only` | `command_id_and_replacement_reconstructable` | `stable` | `true` | — |
| Context menu | `yellow` | `command_record_examples_and_lifecycle_certified` | `disclosed_surface_paraphrase` | `canonical_examples_fresh_and_not_alias_only` | `command_id_and_replacement_reconstructable` | `stable` | `true` | `waiver:command-doc-surface-paraphrase:0001` |
| Command / action bar | `yellow` | `disclosed_reduced_doc_detail` | `canonical_naming_and_replacement_stable` | `canonical_examples_fresh_and_not_alias_only` | `command_id_and_replacement_reconstructable` | `stable` | `true` | — |
| Keybinding resolver layer | `green` | `command_record_examples_and_lifecycle_certified` | `canonical_naming_and_replacement_stable` | `canonical_examples_fresh_and_not_alias_only` | `command_id_and_replacement_reconstructable` | `stable` | `true` | — |
| Conflict review sheet | `green` | `command_record_examples_and_lifecycle_certified` | `canonical_naming_and_replacement_stable` | `canonical_examples_fresh_and_not_alias_only` | `command_id_and_replacement_reconstructable` | `stable` | `true` | — |
| Import-bridge row | `yellow` | `command_record_examples_and_lifecycle_certified` | `canonical_naming_and_replacement_stable` | `disclosed_partial_example_refresh` | `command_id_and_replacement_reconstructable` | `stable` | `true` | — |
| Disabled-command explainer | `green` | `command_record_examples_and_lifecycle_certified` | `canonical_naming_and_replacement_stable` | `canonical_examples_fresh_and_not_alias_only` | `command_id_and_replacement_reconstructable` | `stable` | `true` | — |
| Leader / sequence help overlay | `green` | `command_record_examples_and_lifecycle_certified` | `canonical_naming_and_replacement_stable` | `canonical_examples_fresh_and_not_alias_only` | `command_id_and_replacement_reconstructable` | `beta` | `true` | — |
| Command-documentation surface | `yellow` | `command_record_examples_and_lifecycle_certified` | `canonical_naming_and_replacement_stable` | `canonical_examples_fresh_and_not_alias_only` | `disclosed_partial_capture` | `stable` | `true` | — |

## Auto-narrowed rows

- `context_menu` (`yellow`) — On the space-constrained context menu one command renders a disclosed, waivered short paraphrase of its canonical primary label while still pointing at the canonical command id, the same lifecycle / deprecation truth, and the same replacement guidance — so the naming is narrowed and disclosed rather than an invented alternate label.
- `command_bar` (`yellow`) — On the constrained command / action bar the documentation record takes a disclosed reduced detail — the invocation-schema summary and side-effect / risk detail are folded into an expandable section while the command id, primary label, aliases, and lifecycle / deprecation truth stay visible — so the record is narrowed and disclosed rather than missing or mismatched.
- `import_bridge_row` (`yellow`) — One imported-binding example slice takes a disclosed partial refresh — the stale migration example is flagged and scheduled for refresh rather than presented as current — so the example freshness is narrowed and disclosed rather than shipping a stale or alias-only example unnoticed.
- `command_documentation_surface` (`yellow`) — On the legacy documentation export the copy-safe export surface takes a disclosed partial capture — the export captures the command id and replacement guidance but not the full alias list, while still disclosing the gap — so the copy-safe export parity is narrowed and disclosed rather than absent.

## Exact conformance causes

- `context_menu` — `alternate_label_invented` (disclosed: `true`) — One constrained surface renders a disclosed, waivered short paraphrase of the canonical label while still pointing at the canonical command id and the same replacement guidance — so the naming is narrowed and disclosed rather than an invented alternate label.
- `command_bar` — `lifecycle_or_deprecation_hidden` (disclosed: `true`) — On a constrained surface the documentation record takes a disclosed reduced detail — the invocation-schema summary and side-effect / risk detail are folded into an expandable section while the command id, primary label, aliases, and lifecycle / deprecation truth stay visible — so the record is narrowed and disclosed rather than missing or mismatched.
- `import_bridge_row` — `proof_stale` (disclosed: `true`) — One canonical-example slice takes a disclosed partial refresh — the stale slice is flagged and scheduled for refresh rather than presented as current — so the example freshness is narrowed and disclosed rather than shipping a stale or alias-only example unnoticed.
- `command_documentation_surface` — `proof_stale` (disclosed: `true`) — One legacy documentation export takes a disclosed partial capture — the export captures the command id and replacement guidance but not the full alias list, while still disclosing the gap — so the copy-safe export parity is narrowed and disclosed rather than absent.

## Active waivers

- `waiver:command-doc-surface-paraphrase:0001` (`context_menu`, owner: Shell/command-docs owner, expires `2026-09-30T00:00:00Z`) — On the space-constrained context menu one command renders a disclosed, waivered short paraphrase of its canonical primary label — the compact affordance shortens the label while the surface still points at the canonical command id, the same lifecycle / deprecation truth, and the same replacement guidance, and the command-documentation surface and help pages keep the full canonical label — so the naming is narrowed and disclosed rather than an invented alternate label. The exception retires when the context menu renders the full canonical label on every claimed family.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_command_documentation -- validate
cargo test -p aureline-shell --test m5_command_documentation_fixtures
```
