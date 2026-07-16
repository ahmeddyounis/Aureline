# M5 retirement-manifest and manifest-change-diff registries

This lane makes terminal-lifecycle retirement operable over the frozen
[M5 retired-state matrix](./m5-retired-state-ops.md). It emits one machine-readable retirement manifest per retiring
supported line or stable-facing capability so CLI, docs / help, partner packets, and support bundles inherit one
canonical retirement object rather than hand-authored parallel prose, and emits a typed manifest-change diff whenever
a manifest changes — instead of letting a changed cutoff date or replacement path mutate silently. It records the
*retirement-manifest* grammar (the manifest emitted per retiring class — one typed field per manifest section: the
last-supported version / channel pinned to an exact build, the retirement trigger, the cutoff date, the successor
reference, the disable path, and the export / rollback route — each bound to one object-class identity with its
exact-build joins) and the *manifest-change-diff* grammar (the typed diff event naming whether a manifest changed its
cutoff date, its successor / replacement path, or its disable path / export / rollback route, naming the active diff
reason) into registry resolvers that produce export-safe, honest projections, so CLI, docs / help, partner-packet,
and support-bundle surfaces resolve one canonical retirement object instead of re-synthesizing retirement truth by
hand. The manifest and the diff are separated in runtime and serialized state: the manifest fields, exact-build
joins, disable path, and export / rollback route live on the retirement-manifest entry, while the resolved object
identity, affected manifest field, previous-versus-current manifest-state reference, diff-scope state, and active
diff reason live on the manifest-change-diff entry, and a retiring class's rollback / export route stays preserved so
support language never runs ahead of its closed support note.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_retirement_manifest_and_change_diff_registries` (the
  authoritative validator).
- **Combined schema:**
  `schemas/program/m5-retirement-manifest-and-change-diff-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/program/m5-retirement-manifest.schema.json`](../../schemas/program/m5-retirement-manifest.schema.json)
  (reused from the frozen matrix — the retirement manifest each retiring supported line or stable-facing capability is recorded against)
  and
  [`schemas/program/m5-retirement-manifest-change-diff.schema.json`](../../schemas/program/m5-retirement-manifest-change-diff.schema.json)
  (minted by this lane) as its canonical domain contracts.
- **Checked proof:**
  `artifacts/release/m5-retirement-manifest-and-change-diff-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`). This checked-in proof is the first machine-readable
  retirement manifest — it demonstrates one terminal-retirement loop end to end for the first retirement-bearing
  classes (a supported line and a stable-facing capability).
- **Narrowed fixtures:**
  `fixtures/release/m5-retirement-manifest-and-change-diff-registries/`
  (`retirement_manifest_beta_narrowed.json`, `manifest_change_diff_preview_narrowed.json`).

## Two registries

1. **Retirement manifest** (`resolve_retirement_manifest_entry`) — emits one typed manifest field per manifest
   section, per retiring class: the field and its canonical mode, the exact-build joins (repo rows, bundle IDs,
   install topology, toolchain envelope), the last-supported / known-limits state, the disable path / rollback target,
   and the owning roster. A clean entry names a canonical registry token, a classified manifest field, and a
   retirement role, covers the canonical / accessible / audit resolution forms, publishes a complete object,
   preserves its rollback / export route before a claim widens, and keeps a public-facing successor / disable field
   matched to the closed support note. Otherwise it degrades honestly — a class widening its claim without a preserved
   rollback / export route, or a public-facing field running its language ahead of the closed support note, degrades
   to `retirement_manifest_lets_line_widen_without_rollback_or_runs_support_ahead_of_proof`, the structured blocker
   reason a widen-without-route attempt must surface.
2. **Manifest-change diff** (`resolve_manifest_change_diff_entry`) — turns a manifest change into a visible, typed diff
   event rather than a silent mutation. A clean entry names a classified diff scope (cutoff-date-change,
   replacement-path-change, or disable-or-export-route-change) and provides the complete object-identity /
   affected-manifest-field / previous-versus-current-manifest-state / diff-scope / active-reason diff object; a diff
   that would keep support language ahead of the closed support note, hide the diff, or let a gap masquerade as covered
   degrades to `manifest_change_diff_runs_support_ahead_of_proof_or_drops_manifest_change_diff`.

## Per-entry manifest reference

Each manifest field carries its canonical mode, and the resolver publishes the full manifest object, so the
registry — never a class assumed to have retired cleanly — is the single source of truth.
`retirement_manifest_object_is_complete` rejects an object missing any manifest field,
`line_preserves_rollback_and_diagnostics_before_widening` rejects a claim widening without a preserved rollback /
export route or language running ahead of the closed support note, and `manifest_change_diff_stays_honest` rejects a
diff that has kept support language ahead of the closed support note.

## Acceptance criteria (proven by resolved examples)

- **At least one supported line and one stable-facing capability emit a retirement manifest with stable IDs and
  exact-build joins.** Clean retirement-manifest entries cover the canonical last-supported-version-channel /
  retirement-trigger / cutoff-date / successor-reference / disable-path / export-rollback-route manifest fields and
  the first release-center / help-docs / support / marketplace-registry / install-update surfaces, an
  object-incomplete example degrades, and no clean manifest entry published an incomplete object.
- **Retirement manifests expose successor and rollback / export truth without requiring hand-authored parallel prose
  to stay consistent.** A widen-without-route example and an unbound example degrade, a clean manifest entry is
  present, and no clean entry is unbounded or unbound.
- **A changed cutoff date or replacement path produces a visible diff in the manifest instead of a silent mutation.**
  Clean manifest-change-diff entries cover the cutoff-date-change / replacement-path-change /
  disable-or-export-route-change diff scopes with full resolution-form coverage while providing the complete diff
  object — the resolved object identity and the active diff reason — and a diff that would keep support language ahead
  of the closed support note or drop the diff degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_retirement_manifest_and_change_diff_registries -- support-export
cargo run -p aureline-ui --example dump_m5_retirement_manifest_and_change_diff_registries -- csv
cargo run -p aureline-ui --example dump_m5_retirement_manifest_and_change_diff_registries -- report
cargo run -p aureline-ui --example dump_m5_retirement_manifest_and_change_diff_registries -- retirement-manifest-table
cargo run -p aureline-ui --example dump_m5_retirement_manifest_and_change_diff_registries -- fixture-retirement-manifest-beta-narrowed
cargo run -p aureline-ui --example dump_m5_retirement_manifest_and_change_diff_registries -- fixture-manifest-change-diff-preview-narrowed
```
