# Migration center: diff, rollback, and unsupported-gap taxonomy — contract

This is the reviewer-facing companion for the stable lane that makes the
migration center replacement-grade for switching users: one governed
disclosure record per imported source ecosystem that binds the **before/after
diff**, the **rollback**, and the **Exact / Translated / Partial / Shimmed /
Unsupported taxonomy** to a public claim ceiling and an automatic
narrow-below-Stable verdict.

Do not clone status text from this doc — ingest the canonical machine sources:

- Records / fixtures:
  [`/fixtures/ux/m4/finish-the-migration-center-diff-rollback-and-unsupported/`](../../../fixtures/ux/m4/finish-the-migration-center-diff-rollback-and-unsupported/)
- Schema:
  [`/schemas/ux/finish-the-migration-center-diff-rollback-and-unsupported.schema.json`](../../../schemas/ux/finish-the-migration-center-diff-rollback-and-unsupported.schema.json)
- Release-evidence packet:
  [`/artifacts/ux/m4/finish-the-migration-center-diff-rollback-and-unsupported.md`](../../../artifacts/ux/m4/finish-the-migration-center-diff-rollback-and-unsupported.md)
- Typed source: `aureline_shell::migration_center_stable` (`model`, `corpus`)
- Headless emitter: `aureline_shell_migration_center_stable`
- Replay + invariant gate:
  `crates/aureline-shell/tests/migration_center_stable_fixtures.rs`

## Why one disclosure record per imported-user flow

A switching user who imports a profile asks three questions before they trust
the result: *what exactly changed (the diff), can I undo it (the rollback), and
what did not come across (the unsupported-gap taxonomy)?* When the migration
center, settings import history, command palette, support export, Help/About,
and docs each answer those questions with their own bespoke status text, they
drift — a row implies the import was lossless while the taxonomy already shows
unsupported gaps, a flow implies rollback is available when no pre-apply
checkpoint was verified for it, or an unsupported gap is hidden until after
apply.

`aureline_shell::migration_center_stable` mints one
`migration_flow_disclosure_record` per imported source ecosystem. Each seeded
record is a genuine projection of the **live preview** code: the diff and
checkpoint requirement come from
`aureline_shell::migration_wizard::seeded_migration_wizard_page`, and the
taxonomy comes from
`aureline_shell::migration_corpus::seeded_migration_scoreboard`. The
taxonomy, domains, and source-ecosystem vocabulary are reused verbatim from
`aureline_shell::import::diff_review` and `aureline_shell::migration_corpus`,
so there is no parallel model — the replay gate cross-checks that each record's
upstream identities are the same wizard review, mapping report, rollback
requirement, and scoreboard the migration center page already pivots on. A
preview correlation is not promoted to an apply session, and a
`rollback-requirement:*` value is never exposed as a checkpoint handle.

## The three pillars the record binds

1. **The diff (`diff`).** `reviewed_before_apply` is true, `row_count` is the
   classified-row count, `every_row_has_before_after` and
   `every_row_uses_one_requirement` are true. A flow whose diff is not a
   reviewable before/after surface before apply is narrowed.

2. **The rollback (`rollback`).** `rollback_requirement_ref` is the dry-run
   requirement the execution layer must satisfy before apply. It is not a
   checkpoint. `checkpoint_ref`, `restore_record_ref`, and the Undo/Compare refs
   remain absent until a real apply publishes matching evidence. Without those
   refs, every lifecycle and availability boolean is false and the flow narrows
   with `rollback_evidence_incomplete`.

3. **The unsupported-gap taxonomy (`taxonomy`).** The Exact / Translated /
   Partial / Shimmed / Unsupported counts, `classifications_present`, and the
   union of Unsupported and Shimmed `gaps` — each with
   `visible_before_apply: true` so a gap is discovered during preview rather than
   as missing behaviour after apply.

All support-visible strings and refs are bounded before projection. Raw absolute
paths, URLs, traversal-shaped text, duplicate evidence refs, untyped gap ids,
and impossible UTC timestamps fail closed. Arrays used by taxonomy, routes,
accessibility, evidence, and narrative exports carry schema and builder limits,
so a deserialized record cannot turn support rendering into unbounded work.

## The public claim ceiling

`claim_ceiling` is bound to the real evidence; the builder rejects each
over-claim as a `BuildError`:

- `asserts_diff_reviewed_before_apply` — only when the diff is reviewable before
  apply.
- `asserts_rollback_available` — only when the rollback is live for this flow
  (verified pre-apply checkpoint with undo and compare).
- `asserts_no_unsupported_gaps` — only when the taxonomy carries no Unsupported
  or Shimmed gaps.
- `asserts_full_fidelity_import` — only when there are no Partial, Shimmed, or
  Unsupported rows.

## Automatic narrowing below Stable

`stable_qualification.claim_class` is **derived** from the evidence, never
supplied — a flow can never publish a claim wider than its proof. The cutline is
`stable | beta > preview > not_claimed`; a flow that is missing any pillar of
evidence (diff reviewed before apply, a live verified rollback, gaps visible
before apply, a complete taxonomy) drops to `beta` and names at least one
`narrowing_reasons` entry from the closed set:

- `diff_not_reviewed_before_apply`
- `rollback_evidence_incomplete`
- `unsupported_gaps_hidden_before_apply`
- `taxonomy_incomplete`

The seeded matrix is preview-only. All four flows are therefore `beta`, name
`rollback_evidence_incomplete`, expose no checkpoint/restore/Undo/Compare refs,
and report `qualifies_stable: false`. A future execution-backed projection may
qualify Stable only after binding a real checkpoint and restore record for that
specific flow; preview data alone can never do so.

## Recovery, route, and accessibility parity

- **Recovery routes** always include Reopen-report and Export-support; a flow
  with a live rollback adds Compare and Undo; a flow with gaps adds Review-gaps.
  Every route is keyboard reachable, and `surfaces.recovery_action_ids` is the
  same list in the same order.
- **Route parity** reaches the same flow from the migration center, settings
  import history, command palette, and a menu command — each keyboard reachable,
  each activating the same flow, each a canonical `aureline://` ref.
- **Reopen surfaces** stay `settings` / `help` / `support_export`.
- **Accessibility** carries the tab order, a row narration that discloses the
  source ecosystem, action labels matching the recovery routes, and per-mode
  reachability for normal, high-contrast, and zoomed layouts.
- **Availability** — `available_without_account` and
  `available_without_managed_services` are fixed true; absent identity or
  services never hide a flow.

## Regenerating the fixtures

```sh
cargo run -q -p aureline-shell \
  --bin aureline_shell_migration_center_stable -- emit-fixtures \
  fixtures/ux/m4/finish-the-migration-center-diff-rollback-and-unsupported
```

The replay gate (`cargo test -p aureline-shell --test
migration_center_stable_fixtures`) fails when the on-disk JSON drifts from the
in-code projection, when a claim ceiling over-claims, when a narrowed flow does
not drop below the cutline or name a reason, when a gap is hidden before apply,
when recovery/route/accessibility parity breaks, or when a top-level ref is not
a canonical durable object.
