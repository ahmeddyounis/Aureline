# Alpha crash symbolication binding fixtures

These fixtures exercise the
[`aureline_crash::envelope`](../../../crates/aureline-crash/src/envelope/mod.rs)
alpha contract that joins a runtime crash envelope to a release-side
exact-build symbol manifest. They are consumed by
`crates/aureline-crash/tests/crash_symbolication_alpha.rs`.

| File | Purpose |
|---|---|
| `symbol_manifest_preview_alpha.json` | Frozen preview-channel manifest used by the alpha crash-envelope linked drill after the checked-in release manifest advanced to the beta candidate. |
| `crash_envelope_partial_renderer.json` | Crash envelope that names only the renderer source-map module, exercising the `partial` binding state. |
| `symbol_manifest_partial.json` | Symbol manifest covering only the shell binary, used together with the partial envelope to prove that missing/extra rows surface as `partial`. |

The `linked` scenario reuses the existing exact crash envelope at
[`fixtures/support/incident_trail_alpha/crash_envelope.json`](../incident_trail_alpha/crash_envelope.json)
and the preview symbol manifest above.

The `missing_manifest` scenario is exercised by passing `None` for
`symbol_manifest` to `bind_crash_envelope`; no separate fixture is
needed.

The `build_mismatch` scenario is exercised by reusing the linked
envelope and substituting a different `primary_exact_build_identity_ref`
on the manifest at test time; no separate fixture is needed.
