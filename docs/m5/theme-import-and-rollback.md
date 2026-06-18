# Imported themes: mapping reports, unresolved slots, and rollback

This page is the contract narrative for how the migration center imports a theme
from another tool. Imported themes stay honest only if the product can show,
before a user trusts the result, what translated cleanly, what stayed
approximate, and what did not map at all. Aureline never implies parity when
slots are unresolved, when syntax coverage is partial, or when a fallback path
changed semantic meaning.

It is **not** a separate object model. The canonical machine-readable truth for
this lane is the imported-theme report produced by the shell projection in
[`crate::theme_import_reports`](../../crates/aureline-shell/src/theme_import_reports/mod.rs),
frozen by the boundary schema
[`schemas/ux/m5-theme-import-report.schema.json`](../../schemas/ux/m5-theme-import-report.schema.json),
pinned by the fixtures under
[`fixtures/ux/m5/theme-import-corpus/report.json`](../../fixtures/ux/m5/theme-import-corpus/report.json),
published as the report under
[`artifacts/ux/m5/theme-import-reports/m5_theme_import_report.md`](../../artifacts/ux/m5/theme-import-reports/m5_theme_import_report.md),
and gated by
[`tools/ci/m5/theme_import_report_check.py`](../../tools/ci/m5/theme_import_report_check.py).
The migration center, support/export, compatibility packets, release and
public-truth packs, and sync/import flows **ingest** that one report object; they
do not rephrase its status text.

The closed appearance vocabulary it carries —
[`source_ecosystem_class`](../../schemas/ux/theme_import_report.schema.json),
`mapping_state`, `parity_claim_state`, `import_outcome_state`, and
`rollback_path_class` — is re-exported by reference from
[`schemas/ux/theme_import_report.schema.json`](../../schemas/ux/theme_import_report.schema.json);
this lane mints no parallel appearance vocabulary, and it sits beside the
appearance-object families frozen in
[`docs/m5/theme-package-and-appearance-objects.md`](theme-package-and-appearance-objects.md).

## What every imported-theme row carries

Each imported theme is projected as one row that pins:

- **Source provenance** — the source ecosystem (`vscode`, `jetbrains`, `vim`,
  `emacs`, `zed`, `sublime`, `textmate`, `unknown`), the tool name and version,
  and an opaque source-theme identifier. No raw theme file or token value crosses
  the boundary.
- **A translated-token count and an explicit unresolved-slot count.** A partial
  mapping can never read as full: the row shows `translated / total` and the
  unresolved count side by side, and any non-zero unresolved count is disclosed
  with the listed slots.
- **Syntax-token coverage** — translated, substituted, unresolved, and blocked
  scope counts plus a coverage percent — so partial syntax coverage is visible
  rather than implied.
- **A parity note and a controlled `parity_claim_state`** — `not_claimed`,
  `claimed_with_report`, `partial_claim_with_gaps`, or
  `denied_unresolved_or_blocked`. A row claims full parity (`claimed_with_report`)
  only when zero slots are unresolved, zero honesty checks are blocked, and
  syntax coverage is complete. A visually plausible fallback is never reported as
  full support.
- **A rollback ref** — every imported visual customization carries a reversible
  rollback path (`restore_appearance_checkpoint`, `discard_preview`,
  `reopen_import_review`, or `manual_repair_required`), so an import that proves
  incompatible or semantically misleading can always be reverted.
- **A controlled `import_outcome_state`** the migration center routes on —
  `preview_ready`, `applied`, `applied_with_warnings`, `blocked`, `rolled_back`,
  `cancelled`, `policy_denied`, or `review_required`.

## The honesty spectrum

The seeded report exercises the full spectrum the contract is built to keep
legible. The same report object is what support/export, compatibility packets,
release/public-truth packs, and sync/import flows consume.

### VS Code clean translate {#vscode-clean-translate}

A VS Code dark theme maps one-to-one onto Aureline's semantic and syntax tokens.
Every slot translated, so the row claims full parity (`claimed_with_report`),
backed by the report. The apply is checkpointed and reversible with
`restore_appearance_checkpoint`.

### JetBrains partial mapping {#jetbrains-partial}

A JetBrains Darcula scheme translates its editor colors cleanly, but a handful of
IDE-specific slots have no native target. Those are substituted with disclosed
fallbacks or left unresolved, the outcome is `applied_with_warnings`, and parity
is `partial_claim_with_gaps` with the unresolved slots listed.

### Zed rolled back {#zed-rolled-back}

An applied Zed theme proved semantically misleading: it recolored a protected
trust cue with color alone. The honesty check blocked the mapping
(`blocked_honesty`) and the import was rolled back to its checkpoint. Parity is
`denied_unresolved_or_blocked`. This is the proof that imported visual
customizations stay reversible when they prove incompatible.

### Vim review required {#vim-review-required}

A Vim colorscheme maps its syntax scopes but leaves the IDE chrome unresolved.
The import is held at `review_required` with the unresolved chrome slots listed,
and the rollback path is `reopen_import_review` so the user re-decides them before
anything applies.

### TextMate blocked {#textmate-blocked}

A legacy TextMate `tmTheme` exposes only editor foreground and background. Rather
than render a plausible-looking but mostly unmapped theme and imply parity, the
migration center `blocked` the import and the preview is discarded.

## Rollback and provenance in support, export, and sync

The support-export wrapper quotes the report id and every row id, source-theme
identifier (provenance), checkpoint ref, and rollback ref, so support, offline
review, and sync/import preserve source provenance and unresolved-slot counts
rather than reconstructing them. The report-level invariant flags
(`every_import_reversible`, `no_overclaimed_parity`,
`unresolved_counts_disclosed`, `no_raw_theme_content`) are what release tooling
reads to narrow a row instead of shipping it as implicitly stable.

## Regenerate

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_theme_import_reports -- markdown > \
  artifacts/ux/m5/theme-import-reports/m5_theme_import_report.md
```

`cargo test -p aureline-shell` mints the same report, so the fixtures under
`fixtures/ux/m5/theme-import-corpus/` stay bit-for-bit equal to the seed and the
structural invariants are enforced in Rust as well as by the CI gate.
