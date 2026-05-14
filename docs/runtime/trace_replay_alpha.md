# Trace and Replay Alpha

Aureline now has one bounded alpha contract for profile sessions, trace bundles,
replay capability, and comparison-class truth. The baseline is deliberately
import/view-only for replay: users can inspect imported evidence through
flamegraph, call-tree, timeline, comparison, and support/export rows, but the
surface does not claim live reverse execution.

## Contract

The canonical Rust implementation lives in
[`crates/aureline-runtime/src/trace_replay_alpha`](../../crates/aureline-runtime/src/trace_replay_alpha).

The boundary schemas are:

- [`schemas/runtime/profile_session_alpha.schema.json`](../../schemas/runtime/profile_session_alpha.schema.json)
- [`schemas/runtime/trace_bundle_alpha.schema.json`](../../schemas/runtime/trace_bundle_alpha.schema.json)
- [`schemas/runtime/replay_capability_alpha.schema.json`](../../schemas/runtime/replay_capability_alpha.schema.json)
- [`schemas/runtime/runtime_evidence_comparison_alpha.schema.json`](../../schemas/runtime/runtime_evidence_comparison_alpha.schema.json)

The checked-in descriptor artifacts are:

- [`artifacts/runtime/replay_capability_alpha.yaml`](../../artifacts/runtime/replay_capability_alpha.yaml)
- [`artifacts/runtime/comparison_class_alpha.yaml`](../../artifacts/runtime/comparison_class_alpha.yaml)

## Required Truth

Every profile-session descriptor binds:

- capture mode and source;
- `execution_context_id`;
- exact build, runtime, toolchain, symbol, and source-map refs;
- target process or run configuration;
- capture window and overhead class;
- mapping-quality state;
- data class, redaction mode, retention class, and support-pack item id.

Every trace-bundle manifest keeps raw and derived artifacts separate. Raw bundles
are immutable after capture, while flamegraph, call-tree, timeline, regression,
and advisory views remain derived artifacts with their own provenance and
digests.

Replay capability is declared as a descriptor, not inferred from UI controls. The
current checked-in descriptor uses `import_view_only`: timeline inspection is
available, frame inspection is limited to derived call-tree rows, and reverse
step/data inspection are disabled with explicit reasons.

Comparison packets disclose workload, corpus, source class, hardware/power
profile, runtime/toolchain, sample count, variance window, threshold state, and
comparison class. Imported unlike captures stay inspectable but are not treated
as equivalent regression baselines.

## First Consumers

The shell projection
[`crates/aureline-shell/src/profiling_alpha/mod.rs`](../../crates/aureline-shell/src/profiling_alpha/mod.rs)
renders the runtime packet into a profile/trace/replay surface. It keeps:

- profile identity, capture source, execution context, exact build, overhead,
  and mapping quality visible;
- trace immutability, derived views, redaction, retention, and digest count
  visible;
- replay lane state and disabled live-replay reason visible;
- comparison class and confounders visible before any regression claim.

The support-bundle seed consumes the same support/export projection through
`SupportSeedSurface::runtime_evidence_preview`. Raw trace/profile payloads remain
local-only by default through the existing `support.item.runtime_traces` rule;
the support preview preserves exact-build identity, mapping quality,
comparison class, replay lane state, redaction mode, and retention posture.

## Fixtures

Protected fixtures live under
[`fixtures/runtime/trace_bundle_alpha`](../../fixtures/runtime/trace_bundle_alpha):

- `runtime_evidence_import_view_only.json`
- `manifest.json`

The fixture proves one imported profile and trace bundle can be inspected without
claiming live replay or like-for-like comparison.

## Verify

```sh
cargo test -p aureline-runtime trace_replay_alpha
cargo test -p aureline-shell runtime_evidence_surface_and_support_preview_preserve_import_view_only_truth
```
