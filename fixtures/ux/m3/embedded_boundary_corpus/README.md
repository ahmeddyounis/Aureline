# Embedded boundary audit corpus fixtures

Deterministic, mint-from-truth fixtures for the embedded-surface
boundary audit corpus. The corpus promotes the beta embedded boundary
audit (a single conformant page) into a regression-gated proof system:
every claimed embedded beta surface (docs/help panes, marketplace/account
pages, provider-owned webviews, service dashboards, auth-handoff sheets)
gets a worked **boundary drill** that must pass, plus an adversarial
**denial drill** that the gate must reject.

These files are projections of the seeded packet built by
[`crates/aureline-shell/src/embedded_boundary_corpus/mod.rs`](../../../../crates/aureline-shell/src/embedded_boundary_corpus/mod.rs)
and emitted by the headless inspector
[`crates/aureline-shell/src/bin/aureline_shell_embedded_boundary_corpus.rs`](../../../../crates/aureline-shell/src/bin/aureline_shell_embedded_boundary_corpus.rs).
The inspector is the only mint path, so the checked-in JSON cannot drift
from the Rust types. The boundary vocabulary schema of record is
[`schemas/ux/embedded_surface_boundary.schema.json`](../../../../schemas/ux/embedded_surface_boundary.schema.json);
the per-row audit validator is reused from the beta audit lane.

## Layout

```
packet.json           -- full corpus packet (base audit page + cases + matrix + support export)
corpus_cases.json     -- worked drill cases (boundary + denial)
matrix.json           -- coverage matrix projection
support_export.json   -- redacted support-export projection (no raw payload)
```

## Case shape

Each case carries:

- the audited row and its 1:1 support row (reused beta audit types);
- the authority pair (`native_path_authority_token` vs
  `fallback_path_authority_token`) so the fallback path can be proven not
  to widen authority past the product-owned command;
- `browser_fallback_truth` (return target + reason + object identity) for
  open-in-browser drills;
- `lifecycle_persistence` (initial render / after restart / after
  re-entry) for the high-risk approval-fence drills;
- `expectation` (`conformant` or `denied`), the
  `expected_denial_reason_tokens` it must produce, the
  `actual_denial_reason_tokens` the gate produced, and `verdict_holds`.

A conformant case must produce zero denial reasons. A denial case must
produce at least the reasons it names; otherwise `verdict_holds` is
false and the validator rejects the packet.

## Regenerate

```sh
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_corpus -- packet         > fixtures/ux/m3/embedded_boundary_corpus/packet.json
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_corpus -- cases          > fixtures/ux/m3/embedded_boundary_corpus/corpus_cases.json
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_corpus -- matrix-json     > fixtures/ux/m3/embedded_boundary_corpus/matrix.json
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_corpus -- support-export  > fixtures/ux/m3/embedded_boundary_corpus/support_export.json
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_corpus -- report-md       > artifacts/ux/m3/embedded_boundary_audit_report.md
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_corpus -- doc-md          > docs/ux/m3/embedded_boundary_audit_beta.md
```

## Verify

```sh
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_corpus -- validate
cargo test -p aureline-shell --test embedded_boundary_corpus_fixtures
```
