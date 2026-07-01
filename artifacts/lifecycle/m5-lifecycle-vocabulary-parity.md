# M5 lifecycle-vocabulary parity: controlled state terms kept semantically stable across UI, CLI, docs/help, support exports, telemetry, and claim publication

Generated from the seeded packet in
[`crate::m5_lifecycle_vocabulary_parity`](../../crates/aureline-shell/src/m5_lifecycle_vocabulary_parity/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_vocabulary_parity -- markdown > \
  artifacts/lifecycle/m5-lifecycle-vocabulary-parity.md
```

- Packet id: `m5-lifecycle-vocabulary-parity:stable:0001`
- Source schema ref: `schemas/lifecycle/m5-lifecycle-vocabulary-parity.schema.json`
- Certifies matrix packet: `m5-lifecycle-matrix:stable:0001`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Required parity dimensions: `cross_surface_term`, `semantic_distinction`, `export_code_parity`, `published_copy_narrowing`
- Consumer surfaces certified: `product_ui`, `cli`, `docs_help`, `diagnostics`, `support_export`, `telemetry`, `claim_tooling`, `release_notes`
- Controlled terms certified: 15
- Green (full parity): 11
- Yellow (auto-narrowed): 4
- Red (blocked): 0
- All rows publishable: `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-30T00:00:00Z`

## Parity rows

| Controlled term | Status | Cross-surface | Semantic distinction | Export code | Published copy | Headless | Waiver |
| --------------- | ------ | ------------- | -------------------- | ----------- | -------------- | -------- | ------ |
| Ready | `green` | `term_stable_across_all_surfaces` | `distinct_meaning_preserved` | `code_exports_identically_all_paths` | `copy_auto_narrows_on_state_change` | `true` | — |
| Warming | `green` | `term_stable_across_all_surfaces` | `distinct_meaning_preserved` | `code_exports_identically_all_paths` | `copy_auto_narrows_on_state_change` | `true` | — |
| Partial | `green` | `term_stable_across_all_surfaces` | `distinct_meaning_preserved` | `code_exports_identically_all_paths` | `copy_auto_narrows_on_state_change` | `true` | — |
| Stale | `green` | `term_stable_across_all_surfaces` | `distinct_meaning_preserved` | `code_exports_identically_all_paths` | `copy_auto_narrows_on_state_change` | `true` | — |
| Rebuilding | `green` | `term_stable_across_all_surfaces` | `distinct_meaning_preserved` | `code_exports_identically_all_paths` | `copy_auto_narrows_on_state_change` | `true` | — |
| Restricted | `green` | `term_stable_across_all_surfaces` | `distinct_meaning_preserved` | `code_exports_identically_all_paths` | `copy_auto_narrows_on_state_change` | `true` | — |
| Policy blocked | `yellow` | `term_stable_across_all_surfaces` | `distinct_meaning_preserved` | `disclosed_partial_export` | `copy_auto_narrows_on_state_change` | `true` | — |
| Reconnecting | `green` | `term_stable_across_all_surfaces` | `distinct_meaning_preserved` | `code_exports_identically_all_paths` | `copy_auto_narrows_on_state_change` | `true` | — |
| Degraded | `green` | `term_stable_across_all_surfaces` | `distinct_meaning_preserved` | `code_exports_identically_all_paths` | `copy_auto_narrows_on_state_change` | `true` | — |
| Read-only degraded | `yellow` | `term_stable_across_all_surfaces` | `disclosed_grouped_presentation` | `code_exports_identically_all_paths` | `copy_auto_narrows_on_state_change` | `true` | — |
| Unavailable | `green` | `term_stable_across_all_surfaces` | `distinct_meaning_preserved` | `code_exports_identically_all_paths` | `copy_auto_narrows_on_state_change` | `true` | — |
| Rollback available | `green` | `term_stable_across_all_surfaces` | `distinct_meaning_preserved` | `code_exports_identically_all_paths` | `copy_auto_narrows_on_state_change` | `true` | — |
| Deprecated | `yellow` | `term_stable_across_all_surfaces` | `distinct_meaning_preserved` | `code_exports_identically_all_paths` | `disclosed_manual_narrowing` | `true` | — |
| Experimental | `yellow` | `disclosed_surface_paraphrase` | `distinct_meaning_preserved` | `code_exports_identically_all_paths` | `copy_auto_narrows_on_state_change` | `true` | `waiver:experimental-surface-paraphrase:0001` |
| Retest pending | `green` | `term_stable_across_all_surfaces` | `distinct_meaning_preserved` | `code_exports_identically_all_paths` | `copy_auto_narrows_on_state_change` | `true` | — |

## Auto-narrowed rows

- `policy_blocked` (`yellow`) — Telemetry exports a disclosed coarse policy code for the controlled `policy_blocked` term until the specific block class is finalized, while still naming the same controlled state everywhere, so the export is narrowed and disclosed rather than losing the code.
- `read_only_degraded` (`yellow`) — A compact status surface groups the controlled `read_only_degraded` term under a disclosed "Degraded" family header while still naming it individually and keeping its distinct read-only meaning, so the term is narrowed and disclosed rather than collapsing into a generic degraded state.
- `deprecated` (`yellow`) — Published docs/help copy for the controlled `deprecated` term narrows through a disclosed manual publish step rather than automatically, so the copy is narrowed and disclosed rather than left overclaiming after the term is superseded.
- `experimental` (`yellow`) — Release notes present the controlled `experimental` term as a disclosed, waivered reader-facing "early access" label while still binding it to the same experimental status token everywhere else, so the term is narrowed and disclosed rather than paraphrased into a private synonym.

## Exact term causes

- `policy_blocked` — `upstream_dependency_narrowed` (disclosed: `true`) — The controlled term's status code exports in a disclosed reduced form on a subset of surfaces while still naming the same controlled state, so the export is narrowed and disclosed rather than lost.
- `read_only_degraded` — `upstream_dependency_narrowed` (disclosed: `true`) — A compact consumer surface groups this controlled term under a disclosed family header while still naming it individually, so the presentation is narrowed and disclosed rather than collapsing the term into a generic failure.
- `deprecated` — `upstream_dependency_narrowed` (disclosed: `true`) — The controlled term's published release/docs/help copy narrows only through a disclosed manual publish step rather than automatically, so the copy is narrowed and disclosed rather than overclaiming.
- `experimental` — `upstream_dependency_narrowed` (disclosed: `true`) — One consumer surface presents a disclosed, waivered friendlier label for this controlled term while still mapping it to the same status token, so the wording is narrowed and disclosed rather than drifting into a private synonym.

## Active waivers

- `waiver:experimental-surface-paraphrase:0001` (`experimental`, owner: Release notes owner, expires `2026-09-30T00:00:00Z`) — Release notes present the controlled `experimental` term as a disclosed reader-facing "early access" label while still binding it to the same experimental status token in every export, so the paraphrase is disclosed and waivered rather than drifting into a private synonym, and the controlled token is restored across surfaces when the term qualifies.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_vocabulary_parity -- validate
cargo test -p aureline-shell --test m5_lifecycle_vocabulary_parity_fixtures
```
