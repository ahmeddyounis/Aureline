# Problem-matcher / heuristic conformance corpus

These fixtures replay the problem-matcher / heuristic fallback slice of the
build/test interop conformance suite: the canonical packet runs the heuristic
corpus across the archetypes that depend on best-effort parsing (`rust_cargo`,
`node_workspace`, `python_pytest`, `polyglot_ci`), grading each case on the seven
conformance dimensions (capability negotiation, fallback reason, confidence
preservation, raw-payload retention, replay stability, degraded-state behavior,
and export parity). The blocking cases keep the fallback claim honest: a
heuristic case may not overclaim confidence, drop its fallback reason, or hide
its degraded state. The narrowing case shows that a corpus whose proof has aged
past its freshness window narrows below stable instead of staying green.

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
