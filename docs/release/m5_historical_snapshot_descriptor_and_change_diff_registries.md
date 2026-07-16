# M5 historical-snapshot-descriptor and descriptor-change-diff registries

This lane makes non-live historical evidence operable over the frozen
[M5 historical-reference matrix](../support/m5-historical-evidence-ops.md). It emits one machine-readable snapshot
descriptor per preserved object — a retirement snapshot and a captured support / export evidence packet — so CLI,
docs / help, support, and review / incident surfaces inherit one canonical descriptor rather than hand-authored
parallel prose, and emits a typed descriptor-change diff whenever a descriptor changes — instead of letting a changed
producer build, target link, or retention state mutate silently. It records the *historical-snapshot-descriptor*
grammar (the descriptor emitted per preserved object — one typed field per descriptor section: the canonical object
ID and source class, the capture time, the producer / build identity, the provenance lineage and trust class, the
retention / removal state, and whether the object is analysis-only, reopenable, or metadata-only — each bound to one
object-class identity with its capture-context joins) and the *descriptor-change-diff* grammar (the typed diff event
naming whether a descriptor changed its producer build, its current live-target link, or its retention / removal
state, naming the active diff reason) into registry resolvers that produce export-safe, honest projections, so CLI,
docs / help, support, and review / incident surfaces resolve one canonical descriptor instead of re-synthesizing
non-live-evidence truth by hand. The descriptor and the diff are separated in runtime and serialized state: the
descriptor fields, capture-context joins, live-target reference, and retention state live on the
historical-snapshot-descriptor entry, while the resolved object identity, affected descriptor field,
previous-versus-current descriptor-state reference, diff-scope state, and active diff reason live on the
descriptor-change-diff entry, and every preserved object's non-live evidence stays attributable to its capture
context so a snapshot never reads as a current live object.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_historical_snapshot_descriptor_and_change_diff_registries` (the
  authoritative validator).
- **Combined schema:**
  `schemas/program/m5-historical-snapshot-descriptor-and-change-diff-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/program/m5-historical-snapshot-descriptor.schema.json`](../../schemas/program/m5-historical-snapshot-descriptor.schema.json)
  (reused from the frozen matrix — the historical-snapshot descriptor each retirement snapshot or captured support / export evidence packet is recorded against)
  and
  [`schemas/program/m5-descriptor-change-diff.schema.json`](../../schemas/program/m5-descriptor-change-diff.schema.json)
  (minted by this lane) as its canonical domain contracts.
- **Checked proof:**
  `artifacts/release/m5-historical-snapshot-descriptor-and-change-diff-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`). This checked-in proof is the first machine-readable
  retirement manifest — it demonstrates one terminal-retirement loop end to end for the first retirement-bearing
  classes (a supported line and a stable-facing capability).
- **Narrowed fixtures:**
  `fixtures/release/m5-historical-snapshot-descriptor-and-change-diff-registries/`
  (`historical_snapshot_descriptor_beta_narrowed.json`, `descriptor_change_diff_preview_narrowed.json`).

## Two registries

1. **Retirement manifest** (`resolve_historical_snapshot_descriptor_entry`) — emits one typed manifest field per manifest
   section, per retiring class: the field and its canonical mode, the exact-build joins (repo rows, bundle IDs,
   install topology, toolchain envelope), the last-supported / known-limits state, the disable path / rollback target,
   and the owning roster. A clean entry names a canonical registry token, a classified manifest field, and a
   retirement role, covers the canonical / accessible / audit resolution forms, publishes a complete object,
   preserves its rollback / export route before a claim widens, and keeps a public-facing successor / disable field
   matched to the closed support note. Otherwise it degrades honestly — a class widening its claim without a preserved
   rollback / export route, or a public-facing field running its language ahead of the closed support note, degrades
   to `historical_snapshot_descriptor_lets_line_widen_without_rollback_or_runs_support_ahead_of_proof`, the structured blocker
   reason a widen-without-route attempt must surface.
2. **Descriptor-change diff** (`resolve_descriptor_change_diff_entry`) — turns a descriptor change into a visible, typed
   diff event rather than a silent mutation. A clean entry names a classified diff scope (producer-build-change,
   target-link-change, or retention-state-change) and provides the complete object-identity /
   affected-descriptor-field / previous-versus-current-descriptor-state / diff-scope / active-reason diff object; a diff
   that would present stale capture context as current, hide the diff, or let a gap masquerade as covered
   degrades to `descriptor_change_diff_runs_support_ahead_of_proof_or_drops_descriptor_change_diff`.

## Per-entry manifest reference

Each manifest field carries its canonical mode, and the resolver publishes the full manifest object, so the
registry — never a class assumed to have retired cleanly — is the single source of truth.
`historical_snapshot_descriptor_object_is_complete` rejects an object missing any manifest field,
`line_preserves_rollback_and_diagnostics_before_widening` rejects a claim widening without a preserved rollback /
export route or language running ahead of the closed support note, and `descriptor_change_diff_stays_honest` rejects a
diff that has kept support language ahead of the closed support note.

## Acceptance criteria (proven by resolved examples)

- **At least one supported line and one stable-facing capability emit a retirement manifest with stable IDs and
  exact-build joins.** Clean historical-snapshot-descriptor entries cover the canonical last-supported-version-channel /
  retirement-trigger / cutoff-date / successor-reference / disable-path / export-rollback-route manifest fields and
  the first release-center / help-docs / support / marketplace-registry / install-update surfaces, an
  object-incomplete example degrades, and no clean manifest entry published an incomplete object.
- **Retirement manifests expose successor and rollback / export truth without requiring hand-authored parallel prose
  to stay consistent.** A widen-without-route example and an unbound example degrade, a clean manifest entry is
  present, and no clean entry is unbounded or unbound.
- **A changed cutoff date or replacement path produces a visible diff in the manifest instead of a silent mutation.**
  Clean descriptor-change-diff entries cover the cutoff-date-change / replacement-path-change /
  disable-or-export-route-change diff scopes with full resolution-form coverage while providing the complete diff
  object — the resolved object identity and the active diff reason — and a diff that would keep support language ahead
  of the closed support note or drop the diff degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_historical_snapshot_descriptor_and_change_diff_registries -- support-export
cargo run -p aureline-ui --example dump_m5_historical_snapshot_descriptor_and_change_diff_registries -- csv
cargo run -p aureline-ui --example dump_m5_historical_snapshot_descriptor_and_change_diff_registries -- report
cargo run -p aureline-ui --example dump_m5_historical_snapshot_descriptor_and_change_diff_registries -- historical-snapshot-descriptor-table
cargo run -p aureline-ui --example dump_m5_historical_snapshot_descriptor_and_change_diff_registries -- fixture-historical-snapshot-descriptor-beta-narrowed
cargo run -p aureline-ui --example dump_m5_historical_snapshot_descriptor_and_change_diff_registries -- fixture-descriptor-change-diff-preview-narrowed
```
