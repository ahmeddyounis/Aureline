# Bazel BEP/BES conformance corpus

These fixtures replay the Bazel Build Event Protocol / Build Event Service slice
of the build/test interop conformance suite: the canonical packet runs the Bazel
corpus across the `bazel_monorepo` archetype, grading the case on the seven
conformance dimensions (capability negotiation, fallback reason, confidence
preservation, raw-payload retention, replay stability, degraded-state behavior,
and export parity). The blocking cases here keep the Bazel claim honest: a case
that stops replaying deterministically blocks stable, and removing the corpus
entirely would silently shrink the interop claim for Bazel monorepos.

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
