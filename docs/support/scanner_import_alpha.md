# Scanner Import Alpha

This support note defines the first imported scanner evidence lane. It
keeps SARIF-shaped scanner findings attributable, read-only by default,
and visibly distinct from live local diagnostics.

## Contract

Machine-readable companions:

- [`/schemas/diagnostics/scanner_import_session_alpha.schema.json`](../../schemas/diagnostics/scanner_import_session_alpha.schema.json)
  defines scanner import sessions, run descriptors, suppression/baseline
  registers, review packets, Problems projections, and support exports.
- [`/schemas/diagnostics/diagnostic_delta_alpha.schema.json`](../../schemas/diagnostics/diagnostic_delta_alpha.schema.json)
  defines the compact alpha delta packet with `new`, `resolved`,
  `persisting`, `suppressed`, `waived`, and `unmapped` states.
- [`/fixtures/quality/sarif_alpha/`](../../fixtures/quality/sarif_alpha/)
  contains the protected SARIF-shaped fixture and import binding used by
  the Rust consumer.
- [`/artifacts/quality/diagnostic_delta_alpha_sample.json`](../../artifacts/quality/diagnostic_delta_alpha_sample.json),
  [`/artifacts/quality/scanner_review_packet_alpha_sample.json`](../../artifacts/quality/scanner_review_packet_alpha_sample.json),
  and
  [`/artifacts/quality/suppression_baseline_alpha_sample.json`](../../artifacts/quality/suppression_baseline_alpha_sample.json)
  are redaction-safe sample packets for review and support handoff.

## Runtime Consumer

The shell consumer lives in
[`crates/aureline-shell/src/diagnostics/imported/mod.rs`](../../crates/aureline-shell/src/diagnostics/imported/mod.rs).
It parses the supported SARIF-shaped payload, preserves raw scanner
bodies only by opaque `raw_payload_ref`, publishes imported findings to
the existing diagnostic bus as `scanner_import` / `imported_snapshot`
rows, and emits:

- a diagnostic delta packet;
- a suppression/baseline register;
- a diagnostic review packet;
- a Problems projection; and
- a support export bound to `support.item.imported_diagnostics`.

The support export includes import lineage, delta state, local
confirmation refs, release-visible debt counts, and raw-payload backlinks
without embedding raw scanner bodies, source paths, source snippets, URLs,
logs, or secrets.

## Truth Rules

- Imported scanner rows are read-only even when a compatible local
  analyzer later confirms the same rule family.
- Local confirmation is a separate ref and review action; it does not
  rewrite imported evidence into live local truth.
- Suppressions, waivers, and baselines remain versioned debt records with
  owner, actor, expiry or reopen posture, evidence refs, and
  release-visible accounting.
- Unmapped scanner findings stay visible in Problems/review/support
  exports but do not receive inline source anchors or delta claims that
  require mapping.
