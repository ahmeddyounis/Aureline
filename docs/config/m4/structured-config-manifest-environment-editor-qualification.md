# Structured Config, Manifest, And Environment Editor Qualification

This qualification record is the shared contract for stable structured config,
manifest, lockfile, environment-file, and live-state surfaces.

The source of truth is the typed record emitted by
`aureline_config_structured_editor_qualification`. UI, CLI/headless inspect,
Help/docs, routed actions, and support exports use the same vocabulary:
`source`, `effective_value`, `planned_preview`, `live_observed`,
`round_trip_risk`, target context, apply timing, parameter source, and
copy/export mode.

Stable surfaces must:

- show whether the current view is canonical source, rendered/effective
  projection, planned preview, or observed live state;
- preserve comments, unknown fields, authored ordering, and extension
  namespaces, or downgrade before mutation to raw/source-only, compare-only, or
  high-risk review;
- expose the environment/policy/source chain and the winning effective layer;
- make reset/remove-override operations layer-specific;
- keep secrets as handles, redacted placeholders, or key paths during
  copy/export by default;
- share local syntax, schema, environment probe, remote/auth, policy, and
  dry-run validation with stale-result invalidation reasons;
- block wrong-target, unresolved, deferred, stale-validation, and unsafe
  round-trip drills before save or apply.

Live-state projections are inspect-only until a separate apply model qualifies
them. A save to canonical source never implies live convergence when execution,
deployment, provider runtime, or cluster observation is deferred.

The M5 family-specific deepening of this contract lives in
[`/docs/config/structured_config_parameter_source_and_round_trip_review.md`](../structured_config_parameter_source_and_round_trip_review.md).
That packet reuses this generic source/effective/live and round-trip guardrail
vocabulary, then freezes per-parameter rows, value chips, compare-before-save
review sheets, and export/support disclosure for the newer structured-config
families.

Regenerate fixtures with:

```sh
cargo run -q -p aureline-config --bin aureline_config_structured_editor_qualification -- emit-fixtures fixtures/config/m4/structured-config-manifest-environment-editor-qualification
```

Replay invariants with:

```sh
cargo test -p aureline-config --test structured_editor_qualification_fixtures
```
