# M5 migration-scoreboard and scoreboard-delta registries

This lane keeps post-launch migration and switching promises tied to real field outcomes rather than frozen
launch-time confidence over the frozen
[M5 supported-line-transparency matrix](./m5-supported-line-transparency-ops.md). It publishes one versioned, scored
*migration scoreboard* per active stable or LTS-candidate line so replacement-grade and daily-driver claims,
docs/help/migration owners, support escalations, and partner / procurement reviews inherit current field truth
rather than anecdotal support threads, and emits a typed *scoreboard delta* whenever the published scoreboard changes
— instead of letting an accumulating field-pain cluster, unsupported-item category, docs/help gap, or rollback
failure drift into a forgotten support thread. It records the *migration-scoreboard* grammar (the versioned, scored
migration path published per active supported line — one typed row per importer / bridge outcome class: cleanly
imported, translated to an equivalent, partially imported, shimmed through a compatibility shim, unsupported item
category, and rollback-cleanliness result, tracked by source tool / version / archetype — each bound to one
supported-line identity with rollback cleanliness, docs/help parity, and linked compatibility evidence, and
public-safe outcome classes separated from internal-only migration detail) and the *scoreboard-delta* grammar (the
typed periodic delta event naming whether post-launch field pain is clustering in an outcome class or source
archetype, the unsupported-item categories are accumulating, or a docs/help gap or rollback failure is accumulating
versus the last published scoreboard, naming the active delta reason) into registry resolvers that produce
export-safe, honest projections, so release / help, docs, support, and migration surfaces resolve one canonical,
freshness-checked truth instead of re-synthesizing migration truth by hand. The scoreboard and the delta are
separated in runtime and serialized state: the scored outcome class, affected migration rows, linked
compatibility-report / known-limits / docs-help / migration-pack refs, and rollback posture live on the
migration-scoreboard entry, while the resolved line identity, affected scoreboard-section reference,
previous-versus-current scoreboard reference, delta-scope state, and active delta reason live on the scoreboard-delta
entry, and a line's rollback posture stays preserved so replacement-grade language never runs ahead of current
migration truth.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_supported_line_migration_scoreboard_and_scoreboard_delta_registries` (the
  authoritative validator).
- **Combined schema:**
  `schemas/program/m5-supported-line-migration-scoreboard-and-scoreboard-delta-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/program/m5-migration-scoreboard.schema.json`](../../schemas/program/m5-migration-scoreboard.schema.json)
  (reused from the frozen matrix — the versioned migration scoreboard each supported-line migration record is
  recorded against) and
  [`schemas/program/m5-scoreboard-delta.schema.json`](../../schemas/program/m5-scoreboard-delta.schema.json)
  (minted by this lane) as its canonical domain contracts.
- **Checked proof:**
  `artifacts/release/m5-supported-line-migration-scoreboard-and-scoreboard-delta-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`). This checked-in proof is the first versioned migration
  scoreboard — it demonstrates one durable post-launch migration-scoreboard loop end to end for at least one active
  supported line.
- **Narrowed fixtures:**
  `fixtures/release/m5-supported-line-migration-scoreboard-and-scoreboard-delta-registries/`
  (`migration_scoreboard_beta_narrowed.json`, `scoreboard_delta_preview_narrowed.json`).

## Two registries

1. **Migration scoreboard** (`resolve_migration_scoreboard_entry`) — builds one typed row per importer / bridge
   outcome class, per active supported line: the outcome class and its canonical mode, the affected migration rows,
   the linked compatibility-report / known-limits / docs-help / migration-pack refs, the outcome state, the
   rollback-cleanliness target, and the owning roster, with public-safe outcome classes separated from internal-only
   migration detail. A clean entry names a canonical registry token, a classified outcome class, and a transparency
   role, covers the canonical / accessible / audit resolution forms, publishes a complete object, preserves its
   rollback posture before a claim widens, and keeps a public-facing outcome class's replacement-grade claim matched
   to current migration truth. Otherwise it degrades honestly — a line widening its replacement-grade claim on stale
   migration truth, or a public-facing outcome class running its switching language ahead of current evidence,
   degrades to `migration_scoreboard_lets_line_widen_without_rollback_or_runs_support_ahead_of_proof`, the structured
   blocker reason a widen-on-drifted-migration-truth attempt must surface.
2. **Scoreboard delta** (`resolve_scoreboard_delta_entry`) — turns a scoreboard change into a typed periodic delta
   event against the last published scoreboard rather than a forgotten support thread. A clean entry names a
   classified delta scope (field-pain-cluster, unsupported-category-growth, or docs-help-or-rollback-gap) and
   provides the complete line-identity / affected-scoreboard-section / previous-versus-current-scoreboard /
   delta-scope / active-reason delta object; a delta that would keep switching language ahead of current migration
   truth, hide the delta, or let a gap masquerade as covered degrades to
   `scoreboard_delta_runs_support_ahead_of_proof_or_drops_scoreboard_delta`.

## Per-entry scoreboard reference

The scored outcome class carries its canonical mode, and the resolver publishes the full scoreboard object, so the
registry — never a scoreboard merely assumed to still be current — is the single source of truth.
`migration_scoreboard_object_is_complete` rejects an object missing any scoreboard field,
`line_preserves_rollback_and_diagnostics_before_widening` rejects a claim widening on stale migration truth or
switching language running ahead of current evidence, and `scoreboard_delta_stays_honest` rejects a delta that has
kept switching language ahead of current migration truth.

## Acceptance criteria (proven by resolved examples)

- **A versioned migration scoreboard exists for at least one active supported line and shows importer/bridge outcome
  classes, rollback cleanliness, docs/help parity, and linked compatibility evidence.** Clean migration-scoreboard
  entries cover the canonical imported / translated / partial / shimmed / unsupported / rollback-cleanliness outcome
  classes and the first release-center / shiproom / executive-steering / program-governance / support surfaces, an
  object-incomplete example degrades, and no clean scoreboard entry published an incomplete object.
- **Claim-state or support-language changes can cite scoreboard data instead of relying on anecdotal support
  threads.** A widen-on-drifted-migration-truth example and an unbound example degrade, a clean scoreboard entry is
  present, and no clean entry is unbounded or unbound.
- **Migration docs/help owners can identify concrete deltas between the last published scoreboard and the current one
  without reconstructing data manually.** Clean scoreboard-delta entries cover the field-pain-cluster /
  unsupported-category-growth / docs-help-or-rollback-gap delta scopes with full resolution-form coverage while
  providing the complete delta object — the resolved line identity and the active delta reason — and a delta that
  would keep switching language ahead of current migration truth or drop the delta degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_supported_line_migration_scoreboard_and_scoreboard_delta_registries -- support-export
cargo run -p aureline-ui --example dump_m5_supported_line_migration_scoreboard_and_scoreboard_delta_registries -- csv
cargo run -p aureline-ui --example dump_m5_supported_line_migration_scoreboard_and_scoreboard_delta_registries -- report
cargo run -p aureline-ui --example dump_m5_supported_line_migration_scoreboard_and_scoreboard_delta_registries -- migration-scoreboard-table
cargo run -p aureline-ui --example dump_m5_supported_line_migration_scoreboard_and_scoreboard_delta_registries -- fixture-migration-scoreboard-beta-narrowed
cargo run -p aureline-ui --example dump_m5_supported_line_migration_scoreboard_and_scoreboard_delta_registries -- fixture-scoreboard-delta-preview-narrowed
```
