# Replay-bundle fixture corpus

These fixtures replay the canonical replay bundle (the normalized history joined
to the typed raw-payload lineage, the replay / support / incident / AI evidence
joins, and the replay robustness drills) plus the blocking mutations that must
keep the raw-to-normalized chain intact, keep retention typed and bounded, keep
approval-gated payloads out of the export joins, and keep every join surface
present and provenance-preserving.

Each case records the mutation applied to the seed, the derived promotion state,
the validation finding count and kinds, the surface / retention-class /
source-kind / failure-mode token lists, and whether the support export stays
safe. They are generated from the seed by:

```sh
cargo run -p aureline-runtime --example dump_m5_replay_bundles
```

and replayed by `cargo test -p aureline-runtime --test m5_replay_bundles` and
`tools/ci/m5/replay_bundle_lineage_check.py`. The contract is documented in
`docs/m5/replay-and-raw-payload-lineage.md`.
