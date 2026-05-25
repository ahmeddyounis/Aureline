# Migration-flow disclosure stable matrix fixtures

Each `*.json` here is a pinned `migration_flow_disclosure_record` (schema:
`schemas/ux/finish-the-migration-center-diff-rollback-and-unsupported.schema.json`),
minted bit-for-bit from the in-code corpus in
`crates/aureline-shell/src/migration_center_stable/corpus.rs`. The corpus
projects one record per imported source ecosystem through the **live** migration
builders (`seeded_migration_wizard_page` for the diff/rollback/compare/undo
evidence and `seeded_migration_scoreboard` for the taxonomy), so these records
are a genuine projection of the shell's migration code rather than a parallel
model.

These are **generated, not hand-edited**. Regenerate with:

```sh
cargo run -q -p aureline-shell \
  --bin aureline_shell_migration_center_stable -- emit-fixtures \
  fixtures/ux/m4/finish-the-migration-center-diff-rollback-and-unsupported
```

The replay + invariant gate is
`crates/aureline-shell/tests/migration_center_stable_fixtures.rs`; it fails if
any fixture drifts from the corpus or violates a disclosure invariant (claim
ceiling, automatic narrowing below Stable, gaps-visible-before-apply, recovery /
route / surface parity, or accessibility). The contract narrative is
`docs/ux/m4/finish-the-migration-center-diff-rollback-and-unsupported.md` and the
release-evidence packet is
`artifacts/ux/m4/finish-the-migration-center-diff-rollback-and-unsupported.md`.

The four records cover the claimed stable matrix across both sides of the
cutline:

| Fixture | Ecosystem | Claim | Taxonomy (E/T/P/S/U) | Rollback live |
| --- | --- | --- | --- | --- |
| `vs_code_code_oss.json` | VS Code / Code-OSS | stable | 1/1/1/1/1 | yes |
| `jetbrains_family.json` | JetBrains IDEs | beta (narrowed) | 1/1/1/1/1 | no |
| `vim_neovim.json` | Vim / Neovim | beta (narrowed) | 1/1/1/1/1 | no |
| `emacs.json` | Emacs | beta (narrowed) | 1/1/1/1/1 | no |
