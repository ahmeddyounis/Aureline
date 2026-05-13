# Terminal Protocol Corpus Alpha

This packet is the first checked-in terminal protocol corpus for alpha terminal
claims. It keeps protocol coverage, paste safety, clipboard-write policy, and
restore posture in one fixture set so downstream shell, support, and CI paths
consume the same terminal-owned truth.

## Artifacts

- `fixtures/terminal/protocol_corpus_alpha/manifest.json` lists required
  coverage and every case file.
- `fixtures/terminal/protocol_corpus_alpha/cases/*.json` contains the protected
  corpus cases.
- `tests/terminal/restore_conformance.rs` is the fixture-driven conformance
  suite. It is cargo-runnable through
  `crates/aureline-terminal/tests/restore_conformance.rs`.
- `crates/aureline-terminal/src/protocol_corpus/mod.rs` owns the typed
  projections used by the suite.

## Coverage Contract

The escape/control baseline covers UTF-8 streams, wide glyphs, combining marks,
alternate screen, mouse reporting, bracketed paste control, hyperlinks,
truecolor, OSC 7 cwd hints, and OSC 133-style command boundaries. Fixtures name
capability tokens instead of storing raw PTY escape bytes.

The paste pack includes multiline paste, bracketed paste disclosure, remote
clipboard bridge paste, production-labeled target paste, policy result before
commit, line count before commit, and no-auto-submit behavior. A protected case
must keep target boundary, policy result, and line count visible before commit.

The clipboard-write pack includes OSC 52 writes, remote clipboard bridge writes,
policy-blocked writes, admin policy gating, workspace trust gating, and
metadata-only audit labels. A protected case must prove admin/trust gating and
audit-safe labeling without embedding raw clipboard payload bytes.

The restore suite distinguishes:

- `ended`: prior session ended; execution requires the fresh-session command.
- `reconnect_required`: target or transport must be explicitly reconnected.
- `restored_transcript`: transcript is evidence-only and cannot become live
  execution.

All restore states keep `auto_rerun_forbidden=true`.

## Verification

Run:

```sh
cargo test -p aureline-terminal --test restore_conformance
```

For broader terminal coverage, run:

```sh
cargo test -p aureline-terminal
```
