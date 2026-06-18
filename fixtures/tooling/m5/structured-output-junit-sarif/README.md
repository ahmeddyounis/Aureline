# Structured-output JUnit/SARIF conformance corpus

These fixtures replay the structured-output importer slice of the build/test
interop conformance suite (the corpus every claimed M5 archetype depends on): the
canonical packet runs the JUnit/SARIF/JSON corpus across all six archetypes,
grading each case on the seven conformance dimensions (capability negotiation,
fallback reason, confidence preservation, raw-payload retention, replay
stability, degraded-state behavior, and export parity). The baseline case is the
canonical stable packet; the blocking cases keep the importer claim honest: a
case that loses its retained raw payload, or breaks support/release/AI export
parity, blocks stable.

Each case records the mutation applied to the seed, the derived promotion state,
the validation finding count and kinds, the corpus-family / archetype /
dimension / source-kind token lists, and whether the support export stays safe.
They are generated from the seed by:

```sh
cargo run -p aureline-runtime --example dump_m5_interop_conformance
```

and replayed by `cargo test -p aureline-runtime --test m5_interop_conformance`
and `tools/ci/m5/interop_conformance_check.py`. The contract is documented in
`docs/m5/build-test-interop-corpora.md`.
