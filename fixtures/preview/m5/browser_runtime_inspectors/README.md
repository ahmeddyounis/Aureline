# M5 Browser-Runtime Inspectors Fixtures

## inspectors_label_target_attach_mapping_and_redaction_with_snapshot_continuity.json

A target-kind, attach-depth, mapping-quality, redaction-safety, and
session-continuity drill fixture for the browser-runtime inspector packet. The
five inspector lanes — DOM, CSS, console, network, and storage — each carry at
least one row, and the six target kinds — embedded preview, external browser,
simulator/emulator, device browser, remote preview session, and captured
snapshot — are all named through one vocabulary.

The packet demonstrates all four mapping-quality labels: an `exact` DOM inspector
on an embedded preview whose mutation previews the real source diff before commit;
an `approximate` DOM inspector on an external browser; a `generated_only` CSS
inspector on a simulator showing a generated stylesheet; and `runtime_only`
console, network, and storage inspectors.

It also demonstrates the redaction and continuity rules: the sensitive console,
network, and storage lanes carry redaction-safe postures (`redacted_by_default`,
`metadata_only`, and `hashed_reference`) so tokens, request bodies, and storage
entries never leak; the network inspector is a reconnect that re-pins its prior
session ref so history stays attributable without forcing a downgrade; and the
storage inspector is an imported captured snapshot that records a
`snapshot_imported` downgrade trigger with a precise non-generic degraded label,
so continuity is preserved without the snapshot masquerading as a live runtime.
The single mutation-capable row carries an explicit `dom_mutation` side-effect
class, a target identity, and a `review_required` posture, and targets a live
runtime rather than the captured snapshot.

The fixture validates against
`schemas/preview/browser_runtime_inspectors.schema.json` and is byte-aligned with
the in-crate builder via
`cargo run -p aureline-preview --example dump_m5_browser_runtime_inspectors`.
