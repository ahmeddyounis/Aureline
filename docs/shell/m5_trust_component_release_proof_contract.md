# M5 trust-component release-proof contract

This lane is the **release-evidence certification capstone** that closes the B88 component-contract
lane on top of the frozen
[M5 settings-row, capability-sheet, evidence-chronology, and chronology-export component
matrix](../components/m5_trust_chronology_components_contract.md). Where the matrix *freezes* the six
governed high-trust reusable components — the settings row, the permission/capability sheet, the
event/history row, the timeline group, the narrative summary card, and the chronology export preview
— this lane *certifies*, per component family and against the family's own frozen matrix row, that
its component truth holds on every claimed M5 trust/config/activity/support surface. It is the
release proof the acceptance criteria require: claimed M5 surfaces either pass the shared component
proof packet or narrow their claim state, and release / help / support packets point to one
certification bundle for settings/capability/history component truth.

The lane exists so that M5 can honestly claim mature trust/admin/AI/remote/update/support surfaces:
the reusable components carrying effective-value truth, permission scope, and chronology/export
semantics behave the same way everywhere Aureline uses them, and a family that drifts off the shared
contract auto-narrows rather than keeping a stale claim.

## Governed component families

The certification proof covers exactly the six governed component families the matrix freezes, and
refuses to ship if any is missing:

- `settings_row` — Settings row
- `capability_sheet` — Capability sheet
- `event_history_row` — Event / history row
- `timeline_group` — Timeline group
- `narrative_summary_card` — Narrative summary card
- `chronology_export_preview` — Chronology export preview

## Truth pillars

Every family declares the component-truth pillar it carries, and the union across the six families
must cover the whole track invariant:

- `effective_value_source_and_lock` — the settings pillar (effective-versus-configured value with its
  source pill and lock state).
- `consequence_scope_and_reconsent` — the capability pillar (consequence-grouped permission scope with
  transitive scope and re-consent).
- `chronology_verb_provenance_and_export` — the chronology pillar (stable verbs, provenance badges,
  and portable detail / export), carried by the event row, timeline group, narrative card, and export
  preview.

## Per-family certification row

Each row pulls its component bindings straight from the family's own frozen matrix row — the
settings-row states, source pills, consequence classes, capability scope states, chronology verbs,
provenance badges, chronology detail states, chronology export fields, accessibility routes, required
labels, shell zone, responsive classes, window classes, surface families, consumer surfaces,
downgrade triggers, owner role, scope summary, and qualification. Each row certifies all ten claimed
M5 surface families (`certified_surface_families`), so a family that leaves any claimed M5 surface
uncertified auto-narrows. It is certified across four posture axes:

- **component contract truth** — `contract_truth_certified_every_surface` (green),
  `disclosed_reduced_contract_truth` (yellow: a rarely-seen source pill or low-frequency verb is
  summarized on a secondary surface while the core truth stays certified), or
  `contract_truth_collapsed_or_drifted` (red: the contract truth collapses into a generic value or
  drifts from the frozen vocabulary on a claimed surface).
- **cross-surface parity** — `parity_certified_across_surfaces` (green),
  `disclosed_reduced_surface_projection` (yellow, waiver-backed: a compact secondary surface shows a
  summarized projection while the shared row grammar is preserved), or
  `row_grammar_diverged_off_primary_surface` (red: the component reinvents a second row grammar off the
  primary surface).
- **support-export proof** — `reconstructable_in_export_and_screenshot` (green),
  `disclosed_partial_capture` (yellow: a low-priority component detail is trimmed while the reduction
  is disclosed), or `component_truth_absent_from_capture` (red: the component truth is absent from the
  support-export capture).
- **proof freshness** — `exported_proof_fresh_and_current` (green), `disclosed_partial_refresh`
  (yellow: a low-priority slice awaits the next refresh while the current claim stays backed), or
  `exported_proof_stale_or_divergent` (red: the exported proof is stale or divergent from the current
  component contract).

A per-row hard invariant, `never_drops_audit_or_support_truth`, must hold: a setting, capability, or
chronology truth may never be dropped off the primary surface. `false` is a blocker regardless of the
four axes.

## Derived status and auto-narrowing

Each row's green/yellow/red status is **derived**, never asserted. Any hard blocker (a blocked axis, a
broken invariant, an uncertified claimed M5 surface family, or an undeclared truth pillar) forces
`red`; any disclosed narrowing forces `yellow`; otherwise `green`. A disclosed reduced cross-surface
projection must be backed by an active waiver to stay yellow rather than red. Contract-truth causes
reach for the family's own frozen primary contract trigger (`effective_configured_conflated`,
`consequence_grouping_dropped`, `verb_vocabulary_drift`, or `export_field_dropped`); cross-surface and
audit-truth causes use `audit_truth_lost_off_primary_surface`; support-export and proof-freshness
causes use `proof_stale`. This is the stale-proof detection that narrows a claim when a consumer
drifts off the shared component contract.

## Boundary and artifacts

The records carry no raw URLs, raw local paths, raw usernames, raw hostnames, tokens, or credentials —
only stable ids, closed vocabulary, counts, refs, and short labels. The Rust validator in
`crates/aureline-shell` is the authoritative gate; the boundary schema is
[`schemas/shell/m5-trust-component-release-proof.schema.json`](../../schemas/shell/m5-trust-component-release-proof.schema.json)
and documents the shape.

The headless emitter `aureline_shell_m5_trust_component_release_proof` is the only mint-from-truth
path for the published artifacts:

- Packet: [`artifacts/release/m5-trust-component-release-proof/packet.json`](../../artifacts/release/m5-trust-component-release-proof/packet.json)
- Dashboard: [`artifacts/release/m5-trust-component-release-proof/dashboard.json`](../../artifacts/release/m5-trust-component-release-proof/dashboard.json)
- Support export: [`artifacts/release/m5-trust-component-release-proof/support_export.json`](../../artifacts/release/m5-trust-component-release-proof/support_export.json)
- CSV: [`artifacts/release/m5-trust-component-release-proof/matrix.csv`](../../artifacts/release/m5-trust-component-release-proof/matrix.csv)
- Markdown report: [`artifacts/shell/m5-trust-component-release-proof.md`](../../artifacts/shell/m5-trust-component-release-proof.md)

The protected fixtures under
[`fixtures/ui/m5-trust-component-release-proof/packet.json`](../../fixtures/ui/m5-trust-component-release-proof/packet.json)
(plus `dashboard.json`, `support_export.json`, and `compact.txt`) are asserted bit-for-bit equal to
the seed by the integration test
`crates/aureline-shell/tests/m5_trust_component_release_proof_fixtures.rs`.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_trust_component_release_proof -- validate
cargo test -p aureline-shell --lib m5_trust_component_release_proof::
cargo test -p aureline-shell --test m5_trust_component_release_proof_fixtures
```
