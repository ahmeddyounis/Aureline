# Stable migration guides, compatibility tables, and switching known-limits for launch cohorts

**Status:** Stable workspace lane — implemented in `crates/aureline-workspace`.

## Goal

Every published migration guide a launch cohort reads before adopting Aureline carries one canonical, checked-in truth: a machine-readable **compatibility table** mapping each source capability to an `exact` / `translated` / `partial` / `shimmed` / `unsupported` outcome, an explicit list of **switching known-limits**, and a **stability qualification** that may claim a stable switch only when the evidence backs it. When the evidence does not, the visible tier is automatically narrowed below Stable instead of inheriting an adjacent green claim. Docs/help, release packets, and the switch planner ingest this packet rather than cloning status prose.

## Design principles

1. **Outcome labels from real artifacts** — Every compatibility row carries one of `exact`, `translated`, `partial`, `shimmed`, `unsupported`, derived from an imported artifact reference. No silent drop or heuristic parity.
2. **No prose-only stable claim** — A `stable` switch tier may never be implied from guide prose. It must be `evidence_backed` and point at the published compatibility table.
3. **Automatic narrowing below Stable** — Stale evidence, an unsupported core capability, an unresolved blocking known-limit, a prose-only basis, a missing table, an irreversible switch, or incomplete handoff attribution each narrow the visible tier to `beta` (aging evidence only) or `preview` (any structural shortfall), with machine-readable reasons.
4. **Known-limits stay disclosed** — Switching limitations keep their severity and workaround class rather than collapsing into a single banner; a `blocking` limit with `no_workaround` blocks a stable switch.
5. **Previewable and reversible** — The guide records whether the switch it describes is previewable and reversible; an irreversible switch cannot hold a stable claim.
6. **Exact provider/browser-handoff disclosure** — Any provider-linked or browser-handoff step is explicit about source, actor, freshness, target, and return path; ownership is disclosed. The stable line never hides hosted authority behind local chrome.

## Record kinds

| Record kind | Purpose |
|---|---|
| `migration_switching_publication_packet` | Top-level packet consumed by migration center, docs/help, release packets, the switch planner, support export, and About. |
| `migration_guide_identity` | Guide id, source tool, launch cohort, doc reference, evidence freshness, reversibility, and previewability. |
| `migration_compatibility_table` / `migration_compatibility_table_row` | Machine-readable table mapping each source capability to an outcome label, marking core capabilities. |
| `switching_known_limit` | A disclosed switching limit with a severity class and workaround class. |
| `stable_qualification_claim` | Claimed tier, effective tier after the table is applied, support claim, and narrowing reasons. |
| `provider_handoff_disclosure` | Explicit source, actor, freshness, target, and return path for provider/browser handoff. |
| `migration_switching_publication_inspection` | Compact boolean projection for CLI and inspector surfaces. |

## Closed vocabularies

### Source tools
- `vs_code_code_oss`, `jetbrains_family`, `vim_neovim`, `emacs`

### Launch cohorts
- `solo_switcher`, `team_pilot`, `org_rollout`, `design_partner`, `imported_user`

### Compatibility outcome labels
- `exact`, `translated`, `partial`, `shimmed`, `unsupported`

### Stability tiers
- `stable`, `beta`, `preview`, `withdrawn` (only `stable` is a stable switch claim)

### Evidence freshness classes
- `fresh_current`, `aging_within_window`, `stale_past_window`, `evidence_unknown`

### Known-limit severity / workaround classes
- Severity: `blocking`, `major`, `minor`, `informational`
- Workaround: `native_alternative`, `bridge_available`, `manual_step`, `no_workaround`

### Handoff source / actor / freshness classes
- Source: `local_only`, `hosted_provider`, `browser_handoff`, `mirror_offline`
- Actor: `local_user`, `aureline_runtime`, `hosted_provider_service`, `browser_session`
- Freshness: `live_current`, `cached_disclosed`, `snapshot_dated`, `freshness_unknown`

### Narrowing reasons
- `evidence_freshness_expired`, `compatibility_table_unsupported_present`, `known_limit_blocking_unresolved`, `prose_only_claim`, `missing_compatibility_table`, `not_reversible`, `attribution_incomplete`

## Key invariants

- A `stable` effective tier requires `claim_basis_class == evidence_backed`, a `compatibility_table_ref` that resolves to the published table, current evidence, no unsupported core capability, no unresolved blocking known-limit, `reversible == true`, and complete handoff attribution.
- A downgraded claim carries at least one reason and never keeps a `stable` tier.
- The stored effective tier, downgrade flag, and reasons are re-derived from the table/known-limits at validation time, so a packet cannot drift from its evidence.
- `allows_hidden_provider_mutation`, `allows_unattributed_handoff`, and `allows_irreversible_switch_without_disclosure` are forced `false` and validated.

## File locations

| Artifact | Path |
|---|---|
| Implementation | `crates/aureline-workspace/src/publish_stable_migration_guides_compatibility_tables_and_switching/mod.rs` |
| Schema | `schemas/review/publish-stable-migration-guides-compatibility-tables-and-switching.schema.json` |
| Fixtures | `fixtures/review/m4/publish-stable-migration-guides-compatibility-tables-and-switching/` |
| Tests | `crates/aureline-workspace/tests/publish_stable_migration_guides_compatibility_tables_and_switching_alpha.rs` |
| Proof packet | `artifacts/review/m4/publish-stable-migration-guides-compatibility-tables-and-switching.md` |

## Integration with existing lanes

- Sits above the `stabilize_migration_wizard_import_fidelity_for_editor_launch_paths` module: that module owns per-item import outcomes; this module owns the published, cohort-facing switching truth and its stability qualification.
- Reuses the same outcome vocabulary (`exact` / `translated` / `partial` / `shimmed` / `unsupported`) so a compatibility table can be assembled from real import-fidelity results.

## Verification

```bash
cargo test -p aureline-workspace --test publish_stable_migration_guides_compatibility_tables_and_switching_alpha
cargo test -p aureline-workspace --lib publish_stable_migration
```

Materialized packets for every fixture validate against `schemas/review/publish-stable-migration-guides-compatibility-tables-and-switching.schema.json` (checked with a Draft 2020-12 validator).
