# Warm-startup / warm-restore / first-useful-work drill fixtures

Each `*.json` here is a pinned `warm_continuity_record` (schema:
`schemas/ux/harden_shell_startup_warm_restore_and_first_useful.schema.json`),
minted bit-for-bit from the in-code corpus in
`crates/aureline-shell/src/warm_continuity/corpus.rs`.

These are **generated, not hand-edited**. Regenerate with:

```sh
cargo run -q -p aureline-shell \
  --bin aureline_shell_warm_continuity_corpus -- emit-fixtures \
  fixtures/ux/m4/harden_shell_startup_warm_restore_and_first_useful
```

The replay + invariant gate is
`crates/aureline-shell/tests/warm_continuity_fixtures.rs`; it fails if any
fixture drifts from the corpus or violates the warm-continuity honesty
invariants. The contract narrative is
`docs/ux/m4/harden_shell_startup_warm_restore_and_first_useful.md` and the
release-evidence packet is
`artifacts/ux/m4/harden_shell_startup_warm_restore_and_first_useful.md`.

The seven drills cover the five required regression cases — sleep/resume,
display-topology change, missing extension, expired remote session, and revoked
authorization — plus the warm-relaunch and crash-recovery baselines.
