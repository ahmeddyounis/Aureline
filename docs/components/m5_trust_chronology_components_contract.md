# M5 Settings-Row, Capability-Sheet, Evidence-Chronology, and Chronology-Export Component Contract

> Task: M05-756 · Batch B88 · Delivery class: high-trust component contract +
> reusable primitive implementation + support/export parity.

This contract freezes the checked-in matrix for Aureline's highest-trust reusable
UI components — the ones that still drift too easily by feature lane: settings
rows, permission/capability sheets, event/history rows, timeline groups, narrative
summary cards, and chronology export previews. It names the controlled anatomy,
state vocabulary, export fields, and supportability hooks M5 will honor for each
component family, so later M5 rows can no longer invent private
settings/capability/history row semantics without changing the matrix.

- **Boundary schema:** [`schemas/ui/m5-trust-chronology-components.schema.json`](../../schemas/ui/m5-trust-chronology-components.schema.json)
- **Rust source of truth:** `crates/aureline-shell/src/freeze_the_m5_settings_row_capability_sheet_evidence_chronology_and_chronology_export_component_matrix/`
- **Headless emitter:** `aureline_shell_m5_trust_chronology_components`
- **Checked support export:** [`artifacts/release/m5-trust-chronology-proof/support_export.json`](../../artifacts/release/m5-trust-chronology-proof/support_export.json)
- **Matrix CSV:** [`artifacts/release/m5-trust-chronology-proof/matrix.csv`](../../artifacts/release/m5-trust-chronology-proof/matrix.csv)
- **Report:** [`artifacts/components/m5-trust-chronology-components.md`](../../artifacts/components/m5-trust-chronology-components.md)
- **Narrowed fixtures:** [`fixtures/ui/m5-trust-chronology-components/`](../../fixtures/ui/m5-trust-chronology-components/)

The shell topology this matrix binds against — the eight canonical shell zones,
the compact/standard/expanded responsive classes, the window classes, the
consumer surfaces, and the ten claimed M5 surface families — is reused verbatim
from the frozen
[M5 shell-zone matrix](../../schemas/shell/m5-shell-zone.schema.json). This lane
mints no parallel slot, layout, window, surface-family, or consumer vocabulary; it
adds only the vocabulary for the high-trust components themselves.

## Track invariant

One settings-row model carries effective-versus-configured truth, source pills,
and lock-state explainability; one capability-sheet model groups requests by
consequence, shows transitive scope, and preserves reduced-mode and re-consent
behavior; one evidence/chronology model carries stable verbs, provenance badges,
and portable detail/export semantics; and no M5 lane invents a second row grammar
or drops audit/support truth.

## Component families (rows)

Each row binds one component family to its canonical shell zone, the responsive
classes it must survive, the window classes it keeps continuity across, the
claimed M5 surface families that render it, its mandatory labels, its
family-specific controlled vocabulary, its non-visual accessibility routes, its
consumer surfaces, and the downgrade triggers that narrow it below its claim.

| Component family | Zone | Family-specific vocabulary |
| --- | --- | --- |
| `settings_row` | `main_workspace` | settings-row states + source pills |
| `capability_sheet` | `transient_overlay` | consequence classes + scope states |
| `event_history_row` | `bottom_panel` | chronology verbs + provenance badges |
| `timeline_group` | `bottom_panel` | chronology verbs + provenance badges + detail states |
| `narrative_summary_card` | `right_inspector` | chronology verbs + provenance badges + detail states |
| `chronology_export_preview` | `transient_overlay` | chronology export fields |

The `component_family` predicates drive per-family lints: settings families must
declare settings-row states and source pills; capability families must declare
consequence classes and scope states; chronology-row families (event/history,
timeline group, narrative summary) must declare stable verbs and provenance
badges; grouping families (timeline group, narrative summary) must also declare
chronology detail states; the export family must declare chronology export
fields. Vocabulary a family does not carry stays empty.

## Controlled vocabularies

- **Settings-row states:** `effective_matches_configured`,
  `overridden_by_higher_source`, `inherited_from_default`, `locked_by_policy`,
  `pending_reload_to_apply`, `invalid_value_held`, `redacted_managed_value`.
- **Source pills:** `default_value`, `user_configured`, `workspace_configured`,
  `policy_managed`, `remote_profile`, `environment_override`.
- **Capability consequence classes:** `read_local_context`, `modify_workspace`,
  `execute_code`, `network_access`, `credential_access`, `system_control`.
- **Capability scope states:** `requested_not_granted`, `granted_full_scope`,
  `granted_reduced_scope`, `transitive_scope_disclosed`, `re_consent_required`,
  `revoked_with_history`.
- **Chronology verbs (stable, closed):** `created`, `updated`, `ran`, `approved`,
  `rejected`, `failed`, `recovered`, `exported`.
- **Provenance badges:** `human_initiated`, `ai_initiated`,
  `automation_initiated`, `remote_actor`, `system_initiated`,
  `replayed_from_history`.
- **Chronology detail states:** `collapsed`, `expanded`, `grouped_by_object`,
  `grouped_by_time`, `filtered`, `reopenable_detail`.
- **Chronology export fields:** `event_verb`, `provenance`, `timestamp`,
  `object_ref`, `actor_role`, `outcome_code`, `redaction_class`.
- **Accessibility routes:** `keyboard_focusable`, `screen_reader_announced`,
  `non_hover_reachable`, `pointer_optional`, `high_contrast_safe`,
  `support_exportable`.
- **Required labels:** `identity`, `state`, `keyboard_route` (mandatory on every
  component) plus `provenance`, `effective_value`, `audit_reopen_path`.

## Hard invariants

Every row asserts four booleans that MUST be `false`; any `true` value is a
`component_invariant_violated` blocker:

- `conflates_effective_and_configured` — a settings row never conflates the
  effective value with the configured value.
- `hides_permission_scope` — a capability sheet never hides transitive/downstream
  scope.
- `invents_private_row_grammar` — no component invents a second row grammar.
- `drops_audit_or_support_truth` — audit/support truth is never lost off the
  primary surface.

## Downgrade triggers

`effective_configured_conflated`, `source_pill_missing`, `lock_state_unexplained`,
`consequence_grouping_dropped`, `transitive_scope_hidden`, `re_consent_skipped`,
`verb_vocabulary_drift`, `provenance_badge_missing`,
`chronology_detail_not_reopenable`, `export_field_dropped`,
`audit_truth_lost_off_primary_surface`, `proof_stale`.

## First consumers bound to the matrix

- Settings surfaces consume the settings-row / source-pill vocabulary.
- Capability sheets consume the consequence / scope vocabulary.
- Activity and evidence surfaces consume the chronology verb / provenance /
  detail vocabulary.
- Chronology export previews read a single canonical export-field source.
- Support/export and the accessibility bridge each read one canonical
  trust-component source.

## Regenerating the artifacts

```sh
BIN=aureline_shell_m5_trust_chronology_components
cargo run -q -p aureline-shell --bin $BIN -- support-export > artifacts/release/m5-trust-chronology-proof/support_export.json
cargo run -q -p aureline-shell --bin $BIN -- csv            > artifacts/release/m5-trust-chronology-proof/matrix.csv
cargo run -q -p aureline-shell --bin $BIN -- report         > artifacts/components/m5-trust-chronology-components.md
cargo run -q -p aureline-shell --bin $BIN -- fixture-narrative-summary-card-beta-narrowed        > fixtures/ui/m5-trust-chronology-components/narrative_summary_card_beta_narrowed.json
cargo run -q -p aureline-shell --bin $BIN -- fixture-chronology-export-preview-preview-narrowed  > fixtures/ui/m5-trust-chronology-components/chronology_export_preview_preview_narrowed.json
cargo run -q -p aureline-shell --bin $BIN -- validate
```

The inline test `checked_support_export_matches_seed` and the fixture round-trip
test assert the checked-in JSON is bit-for-bit identical to the seed builder, so
the artifacts can never silently drift from the in-code matrix.
