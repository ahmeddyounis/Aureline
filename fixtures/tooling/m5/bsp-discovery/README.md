# BSP discovery conformance corpus

These fixtures replay the BSP discovery slice of the build/test interop
conformance suite: the canonical packet runs the Build Server Protocol corpus
across every claimed M5 archetype that depends on BSP (`jvm_build_server` and
`bazel_monorepo`), grading each case on the seven conformance dimensions
(capability negotiation, fallback reason, confidence preservation, raw-payload
retention, replay stability, degraded-state behavior, and export parity). The
blocking cases here keep the BSP claim honest: a case that ran no capability
handshake blocks stable, and dropping the JVM build-server archetype leaves a
claimed profile unexercised.

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
