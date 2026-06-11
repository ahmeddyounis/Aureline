# Framework generators and codemods with preview, diff, rollback, and execution-context reuse

This contract describes the export-safe packet that carries the **framework
generator and codemod run** truth for the generator gallery, preview pane,
diff-review, run, rollback/recovery, diagnostics, and support surfaces: the
pinned generator version each run carries, whether a preview was produced,
whether the change diff was reviewed, whether the run can be rolled back, whether
a warm execution context was reused, how run-fresh it is, its support class, and
its downgrade banner. The packet is the canonical truth those surfaces ingest
instead of letting starter convenience outrun provenance, preview, or rollback,
or presenting heuristic or bridge behavior as exact first-party truth.

- Boundary schema:
  `schemas/templates/implement-framework-generators-or-codemods-with-preview-diff-rollback-and-execution-context-reuse.schema.json`
- Implementation:
  `crates/aureline-templates/src/implement_framework_generators_or_codemods_with_preview_diff_rollback_and_execution_context_reuse/`
- Checked support export:
  `artifacts/templates/m5/implement_framework_generators_or_codemods_with_preview_diff_rollback_and_execution_context_reuse/support_export.json`
- Fixtures:
  `fixtures/templates/m5/implement_framework_generators_or_codemods_with_preview_diff_rollback_and_execution_context_reuse/`

This packet **references** the upstream template-manifest, framework-pack, and
generation diff-review/recovery records — the `template_manifest_alpha` contract,
the framework-pack header packet, and the generation diff-review and recovery
packet frozen in `docs/templates/template_registry_and_scaffold_contract.md` and
the framework-pack lane — by opaque ref (`app_id`, `framework_pack_ref`,
`generator_id`, `rollback_handle_refs`, `execution_context_refs`) rather than
embedding them, and reuses the prior support-class and downgrade vocabulary
instead of inventing parallel terms.

## Boundary discipline

The packet is metadata only. Raw source bodies, raw diffs, generated file
contents, repository URLs, hostnames, secrets, and user-authored content never
cross this boundary. Rows carry opaque refs, closed-vocabulary class tokens,
short reviewable summaries, structural locators (`generator_locator`,
`rollback_handle_refs`, `execution_context_refs`), and export-safe chip labels
(`diff_stat_label`, `freshness_chip_label`). `validate` rejects any export that
leaks obviously forbidden material.

## Row truth

Each `generator_run_row` binds one generator or codemod run to:

- **Kind, provenance, and target** — `generator_kind` (`scaffold_generator`,
  `codemod`, `migration_codemod`, `refactor_generator`, or `config_generator`),
  `generator_version` (the pinned version, always disclosed), `generator_locator`,
  and `target_summary`.
- **Preview** — `preview_class` (`preview_available`, `preview_partial`, or
  `preview_unavailable`) and `preview_summary`. A `preview_unavailable` run must
  raise the `preview_unavailable_banner` and is blocked from confident apply — a
  run is never applied blind.
- **Diff review** — `diff_review_class` (`diff_reviewed`, `diff_pending`, or
  `diff_unavailable`), `diff_summary`, and `diff_stat_label`. A `diff_unavailable`
  run must show a downgrade banner and is blocked from confident apply.
- **Rollback** — `rollback_class` (`rollback_available`, `rollback_partial`,
  `rollback_unavailable`, or `rolled_back`), `rollback_summary`, and
  `rollback_handle_refs`. A captured, partial, or rolled-back rollback carries at
  least one rollback-handle ref. A `rollback_unavailable` run must raise the
  `rollback_unavailable_banner` and is blocked — a run is never applied without a
  way back.
- **Execution-context reuse** — `context_reuse_class` (`context_reused`,
  `context_fresh`, `context_reuse_unavailable`, or `context_reuse_unknown`),
  `context_reuse_summary`, and `execution_context_refs`. A
  `context_reuse_unavailable` or `context_reuse_unknown` state must show a banner.
  Falling back to a fresh context is honest, not a block — a reuse fallback is
  labeled, never silently hidden, and the run stays offered.
- **Freshness chip** — `freshness_class` (`fresh`, `rescan_available`, `aging`,
  `stale`, or `freshness_unknown`), `freshness_chip_label`, and `last_run`. A
  `stale` or `freshness_unknown` chip must show a downgrade banner.
- **Support honesty** — `support_class` keeps bridge/heuristic behavior labeled.
  A `bridge_behavior` or `heuristic_mapping` run must disclose a known issue, carry
  the matching `bridge_behavior_disclosed` / `heuristic_mapping_disclosed`
  downgrade trigger, and show a banner, so an inferred or bridged generator is
  never presented as exact first-party truth.
- **Downgrade banner** — `downgrade_banner_class` (`no_banner`, `freshness_banner`,
  `preview_unavailable_banner`, `diff_unavailable_banner`,
  `rollback_unavailable_banner`, `support_class_banner`, `context_reuse_banner`, or
  `policy_block_banner`) makes the narrowing cue explicit.
- **Downgrade and projection** — `downgrade_triggers`, `consumer_surfaces`, and
  `admitted_for_display`. A blocked run (stale or unknown freshness, unavailable
  preview, unavailable diff, unavailable rollback, or a hard-block banner) can
  never be admitted for confident display.

## Downgrade automation

`apply_downgrade_automation` narrows runs from observed runtime signals so a stale
or unrunnable generator narrows before it is offered, instead of being hidden or
presented as exact truth:

- **An unavailable preview** marks the preview and diff unavailable, raises the
  preview-unavailable banner, and withdraws display.
- **A lost rollback handle** marks the rollback unavailable, raises the
  rollback-unavailable banner, and withdraws display.
- **An unavailable diff** marks the diff unavailable, raises the diff-unavailable
  banner, and withdraws display.
- **A failed context reuse** narrows the reuse state to unavailable, raises a
  context-reuse banner, and keeps the run offered — falling back to a fresh
  context is honest, not a block.
- **A stale run record** narrows freshness to `stale`, raises a freshness banner,
  and withdraws display.
- **Stale proof or a narrowed upstream** withdraws display.

A raised banner is never lowered to a softer cue, and a narrowed run stays a
valid, export-safe packet, so the run and support surfaces show a current, labeled
state rather than an optimistic placeholder.

## Consumers

`current_generator_run_export()` reads and validates the checked support export.
It is the first real consumer: a generator gallery, preview-pane, diff-review,
run, rollback/recovery, diagnostics, or support-export surface ingests the
canonical packet through it. The two checked fixtures
(`rollback_unavailable_blocked.json`, `context_reuse_unavailable_labeled.json`)
are valid, narrowed packets that exercise the downgrade behavior the canonical
export keeps green.

The artifact and fixtures are regenerated deterministically from the canonical
builder:

```text
cargo run -p aureline-templates --example dump_framework_generators -- canonical
cargo run -p aureline-templates --example dump_framework_generators -- markdown
cargo run -p aureline-templates --example dump_framework_generators -- rollback_unavailable
cargo run -p aureline-templates --example dump_framework_generators -- context_reuse_unavailable
```
