# Trace Bundle Alpha Fixtures

These fixtures exercise the runtime profile/trace/replay alpha packet emitted by
`crates/aureline-runtime/src/trace_replay_alpha`.

- `runtime_evidence_import_view_only.json` preserves one imported profile,
  immutable trace bundle, replay-capability descriptor, comparison-class packet,
  and support/export projection. It keeps the replay lane
  `import_view_only`, maps the captured build as partial source-map evidence,
  and marks the raw trace payload as local-only retained.
- `manifest.json` lists the schema and artifact refs plus the acceptance states
  consumers must preserve.

Verify with:

```sh
cargo test -p aureline-runtime trace_replay_alpha
```
