# M5 trust-component release proof

Generated from the seeded packet in
[`crate::m5_trust_component_release_proof`](../../crates/aureline-shell/src/m5_trust_component_release_proof/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_trust_component_release_proof -- markdown > \
  artifacts/shell/m5-trust-component-release-proof.md
```

- Packet id: `m5-trust-component-release-proof:stable:0001`
- Source schema ref: `schemas/shell/m5-trust-component-release-proof.schema.json`
- Certifies matrix packet: `m5-trust-chronology-components:stable:0001`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Rows certified: 6
- Green: 3
- Yellow (auto-narrowed): 3
- Red (blocked): 0
- All rows publishable: `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-30T00:00:00Z`

## Certification dimensions

- `component_contract_truth`
- `cross_surface_parity`
- `support_export_proof`
- `proof_freshness`

## Truth pillars

- `effective_value_source_and_lock`
- `consequence_scope_and_reconsent`
- `chronology_verb_provenance_and_export`

## Certification rows

| Component family | Status | Qualification | Contract truth | Cross-surface parity | Support-export | Proof freshness | No-dropped-audit-truth | Waiver |
| ---------------- | ------ | ------------- | -------------- | -------------------- | -------------- | --------------- | ---------------------- | ------ |
| Settings row | `green` | `stable` | `contract_truth_certified_every_surface` | `parity_certified_across_surfaces` | `reconstructable_in_export_and_screenshot` | `exported_proof_fresh_and_current` | `true` | — |
| Capability sheet | `green` | `stable` | `contract_truth_certified_every_surface` | `parity_certified_across_surfaces` | `reconstructable_in_export_and_screenshot` | `exported_proof_fresh_and_current` | `true` | — |
| Event / history row | `green` | `stable` | `contract_truth_certified_every_surface` | `parity_certified_across_surfaces` | `reconstructable_in_export_and_screenshot` | `exported_proof_fresh_and_current` | `true` | — |
| Timeline group | `yellow` | `stable` | `contract_truth_certified_every_surface` | `disclosed_reduced_surface_projection` | `reconstructable_in_export_and_screenshot` | `exported_proof_fresh_and_current` | `true` | `waiver:reduced-surface-projection:0001` |
| Narrative summary card | `yellow` | `stable` | `contract_truth_certified_every_surface` | `parity_certified_across_surfaces` | `disclosed_partial_capture` | `exported_proof_fresh_and_current` | `true` | — |
| Chronology export preview | `yellow` | `stable` | `contract_truth_certified_every_surface` | `parity_certified_across_surfaces` | `reconstructable_in_export_and_screenshot` | `disclosed_partial_refresh` | `true` | — |

## Auto-narrowed rows

- `timeline_group` (`yellow`) — The timeline group serves a disclosedly reduced surface projection on the most compact secondary surface (a collapsed grouping heading in place of the full grouped detail) while the shared row grammar, stable verbs, provenance badges, and reopen path stay identical to the primary surface; the reduction is disclosed behind a waiver, so the row is narrowed below green while it is in force.
- `narrative_summary_card` (`yellow`) — Under the seeded release the narrative summary card reconstructs its verb / provenance / reopen truth from the support export and screenshot baselines but discloses a partial capture of some low-priority prose-summary phrasing while the export queue is throttled; the partial capture is disclosed and the row is narrowed below green.
- `chronology_export_preview` (`yellow`) — Under the seeded release the chronology export preview's exported proof is refreshed for every mandatory export field but discloses a partial refresh of a low-priority redaction-class annotation that awaits the next scheduled refresh while the current claim stays backed; the partial refresh is disclosed and the row is narrowed below green.

## Exact certification causes

- `timeline_group` — `audit_truth_lost_off_primary_surface` (disclosed: `true`) — A surface projection is disclosedly reduced (a compact secondary surface shows a summarized projection) while the shared row grammar is preserved; the reduction is disclosed behind a waiver and the row is narrowed below green.
- `narrative_summary_card` — `proof_stale` (disclosed: `true`) — The support export reconstructs the component truth and discloses a partial capture (some low-priority component detail is trimmed) while the reduction is disclosed and the row is narrowed below green.
- `chronology_export_preview` — `proof_stale` (disclosed: `true`) — The exported proof is refreshed and discloses a partial refresh (a low-priority slice awaits the next refresh) while the current claim stays backed; the reduction is disclosed and the row is narrowed below green.

## Active waivers

- `waiver:reduced-surface-projection:0001` (`timeline_group`, owner: Activity/evidence component owner, expires `2026-09-30T00:00:00Z`) — Under the seeded release the timeline group shows a disclosedly summarized projection on the most compact secondary surface (a collapsed grouping heading in place of the full grouped detail) while the shared row grammar, the stable verbs and provenance badges, and the reopen path stay identical to the primary surface. The narrowing is disclosed, never hides a grouped event, and keeps one row grammar.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_trust_component_release_proof -- validate
cargo test -p aureline-shell --test m5_trust_component_release_proof_fixtures
```
