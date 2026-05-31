# Artifact: Publish stable migration guides, compatibility tables, and switching known-limits for launch cohorts

**Task:** Publish the canonical, checked-in switching truth a launch cohort reads before adopting Aureline — the migration guide, the machine-readable compatibility table, and the switching known-limits — and the stability qualification that truth may claim, with automatic narrowing below Stable when the evidence does not back it.
**Status:** Implemented
**Verification class:** Automated functional + Conformance / interoperability suite + Design QA / UX validation + Release evidence review

## Summary

This lane binds a migration guide identity, a machine-readable compatibility table whose rows map each source capability to an `exact` / `translated` / `partial` / `shimmed` / `unsupported` outcome generated from real imported artifacts, the disclosed switching known-limits, the stability qualification claim, and an explicit provider/browser-handoff disclosure into one validated packet. A stable switch claim is only allowed when it is evidence-backed and resolves to a *current* compatibility table with no unsupported core capability and no unresolved blocking known-limit, describes a reversible switch, and is fully attributed about its provider/browser-handoff source. Stale evidence, an unsupported core capability, an unresolved blocking limit, a prose-only basis, a missing table, an irreversible switch, or incomplete attribution automatically narrow the visible tier below Stable (to `beta` or `preview`) and record machine-readable reasons. The checked-in packet is canonical: docs/help, release packets, and the switch planner ingest it instead of cloning status prose.

## What changed

- New Rust module: `crates/aureline-workspace/src/publish_stable_migration_guides_compatibility_tables_and_switching/mod.rs`
- Re-exported from `crates/aureline-workspace/src/lib.rs`
- New schema: `schemas/review/publish-stable-migration-guides-compatibility-tables-and-switching.schema.json`
- New fixtures: `fixtures/review/m4/publish-stable-migration-guides-compatibility-tables-and-switching/`
  - `stable_vs_code_solo_switch_current.json` — VS Code solo-switcher guide resolves to a current table and holds Stable.
  - `jetbrains_stale_evidence_auto_narrow.json` — stale compatibility evidence narrows a Stable claim to `beta`.
  - `vim_unsupported_core_narrows_to_preview.json` — an unsupported core capability plus an unresolved blocking limit narrow to `preview`.
  - `imported_user_prose_only_narrowed.json` — a prose-only Stable claim that binds no table narrows to `preview`.
  - `team_pilot_beta_browser_handoff_holds.json` — an honest Beta guide with an explicit, attributed browser handoff passes through unchanged.
- New tests: `crates/aureline-workspace/tests/publish_stable_migration_guides_compatibility_tables_and_switching_alpha.rs`
- New docs: `docs/m4/publish-stable-migration-guides-compatibility-tables-and-switching.md`

## Acceptance criteria

- [x] The checked-in implementation, fixtures, and proof packet are current and self-describing (the schema, fixtures, and docs reference one another) rather than ad hoc notes. (`publish-stable-...schema.json`, fixtures dir, this packet)
- [x] Any surface still lacking stable qualification is automatically narrowed below Stable, with machine-readable reasons, instead of inheriting an adjacent green row. (`jetbrains_stale_evidence_auto_narrow.json`, `vim_unsupported_core_narrows_to_preview.json`, `imported_user_prose_only_narrowed.json`, `stale_evidence_narrows_to_beta`, `unsupported_core_capability_narrows_to_preview`, `prose_only_stable_claim_is_narrowed`)
- [x] Migration workflows stay previewable, attributable, and reversible: the guide records previewability and reversibility, and an irreversible switch cannot hold a stable claim. (`irreversible_switch_narrows_below_stable`)
- [x] Provider-linked / browser-handoff behavior is explicit about source, actor, freshness, target, and return path, and incomplete attribution narrows below Stable. (`team_pilot_beta_browser_handoff_holds.json`, `incomplete_attribution_narrows_below_stable`)
- [x] Outcome labels are generated as `exact`, `translated`, `partial`, `shimmed`, or `unsupported` from real imported artifact references. (`migration_compatibility_table_row.derived_from_artifact_ref`, `every_fixture_builds_validates_and_matches_expectations`)
- [x] Switching known-limits stay disclosed with severity and workaround class rather than collapsing into one banner; a blocking limit with no workaround blocks a stable switch. (`switching_known_limit`, `blocking_known_limit_narrows_to_preview`)

## Guardrails honored

- Migration approximation never masquerades as exact parity: an `unsupported` core row or a `blocking` / `no_workaround` known-limit narrows a Stable claim, and the table keeps every outcome label visible.
- Hosted/provider mutations are not hidden behind local chrome: `allows_hidden_provider_mutation`, `allows_unattributed_handoff`, and `allows_irreversible_switch_without_disclosure` are forced false and validated; incomplete handoff attribution narrows the claim.
- Public scope is not widened: the module adds one bounded packet family above the existing migration-wizard import-fidelity lane without altering it.
- A narrower stable claim is published rather than papered over: the effective tier, downgrade flag, and reasons are re-derived from the evidence at validation time, so the packet cannot drift from its table.

## How to verify

```bash
cargo test -p aureline-workspace --test publish_stable_migration_guides_compatibility_tables_and_switching_alpha
cargo test -p aureline-workspace --lib publish_stable_migration
```

Materialized packets for every fixture validate against `schemas/review/publish-stable-migration-guides-compatibility-tables-and-switching.schema.json` (checked with a Draft 2020-12 validator).

## Risks / follow-ups

- The compatibility table is consumed as input; a later revision should source it directly from the import-fidelity pipeline that produces per-item outcomes instead of accepting a producer-supplied table.
- Evidence freshness is declarative (`stale_past_window` supplied by the producer). When a wall-clock evaluation lane lands, the narrowing can be derived from `freshness_expires_at` versus evaluation time.
- Source-tool, cohort, and handoff classes are closed string vocabularies; when the provider/migration crates stabilize typed enums, these should be narrowed to share them.
