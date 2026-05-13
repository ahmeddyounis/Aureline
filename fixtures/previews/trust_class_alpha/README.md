# Trust-Class Alpha Preview Fixtures

These fixtures drive the shell trust-class projection in
`crates/aureline-shell/src/previews/trust_classes.rs`.

The protected path proves that docs, preview, and package/install review
surfaces reuse the same `RawText`, `SanitizedRich`, `TrustedLocalActive`,
and `IsolatedRemoteActive` vocabulary, keep visible representation labels on
copy/export actions, and downgrade unsafe active previews to static or
metadata-only modes before active behavior can run.
