# Macro recorder and replay for M5 automation

This is the reviewer-facing landing page for the **macro-recorder session and
replay object** and its first M5 automation consumers. The live object lives in
[`crates/aureline-runtime/src/macro_sessions/`](../../crates/aureline-runtime/src/macro_sessions/mod.rs);
the cross-tool boundary schema for the first-consumers packet is
[`schemas/automation/macro-recorder.schema.json`](../../schemas/automation/macro-recorder.schema.json);
the checked-in artifacts live under
[`artifacts/m5/automation/macro-recorder/`](../../artifacts/m5/automation/macro-recorder/);
the worked-example fixtures live under
[`fixtures/automation/m5/macro-recorder/`](../../fixtures/automation/m5/macro-recorder/);
and the fail-closed gate is
[`tools/ci/m5/macro_recorder_check.py`](../../tools/ci/m5/macro_recorder_check.py).

The frozen macro-session boundary contract this lane builds on is
[`schemas/automation/macro-session.schema.json`](../../schemas/automation/macro-session.schema.json),
frozen alongside the recipe-and-macro contract in
[`docs/m5/recipe-builder-and-macro-contract.md`](./recipe-builder-and-macro-contract.md).
A stopped session mints exactly one `macro_session_record` against that schema; a
discarded session mints none. The recorder is deliberately narrow: every captured
step is strictly UI or editor state, the projected safety labels are limited to
`macro_safe` and `ui_only`, and a macro is never admissible on the managed-only
channel.

## What the object is

A `MacroRecorderSession` records **one editing flow** captured as a reviewable,
profile-local macro. It carries:

- **Captured commands** — each strictly UI or editor state, named by a
  content-address digest, with its surface class, support class, replay posture,
  and a reviewable label. Raw buffer bytes, raw DOM, raw shell commands, and raw
  secrets never cross this boundary.
- **Active recording strip** — the live strip the recorder renders while it
  captures: whether it is capturing, the captured-command count, the
  supported/unsupported split, and the declared target scope.
- **Captured-command review** — the review a user inspects before saving or
  discarding, listing every captured command, surfacing every unsupported-command
  warning, and resolving whether the recording is safe to save.
- **Target-scope declaration** — the scope the macro replays against (active
  document, selection, editor group, single file, multiple files, or workspace).
- **Storage scope** — the profile-local resident storage (user or workspace), never
  the organization / managed-only channel.
- **Promotion affordance** — promotable to a declarative recipe, UI-only not
  promotable, or promotion blocked by policy.
- **Replay count** and **disposition** — how many times the macro has replayed, and
  whether the session was saved as a profile-local macro, saved and promoted to a
  recipe, discarded, or is still recording.

## Recording, review, save, and discard

The recorder captures UI or editor state into an ordered list of commands while the
**active recording strip** reports progress. When the user stops, the
**captured-command review** lists every command and surfaces an
**unsupported-command warning** for any command that runs a process, performs a
network call, mutates remote state, writes files, reads a secret, requires an
approval, or hands off to an external runner. Those commands are not recordable as
macro steps, and an unsupported command **blocks save** (`save_admissible` is
false) — that flow belongs in a declarative recipe, not a macro.

- **Save** stops the session and mints a `macro_session_record` plus a
  recorded-macro manifest reference against
  [`schemas/automation/recipe_manifest.schema.json`](../../schemas/automation/recipe_manifest.schema.json).
- **Discard** mints no manifest at all; the disposition is
  `discarded_no_macro_minted` and `resulting_macro_manifest_ref` is null.

## Replay is context, not authority

The single most important rule: **a recording is not authority**. The replay action
a session offers is *derived* from the session's repository-import state and the
`current_replay_blockers` the resolver observed *now* — never from the scope the
macro was captured under.

- `resolved_replay_class` resolves replay through one of twelve closed classes: two
  admissible (in the declared scope, or after an explicit scope reconciliation) and
  ten blocked (target-scope mismatch, active-context drift, supported-command-set
  changed, unsupported command captured, profile-scope mismatch, promotion required
  because the macro crosses scope, kill switch engaged, disabled by policy, revision
  retired, or imported from repository content).
- A recorded macro **fails closed**: any context drift the user has not explicitly
  reconciled, any change to the declared target scope or the supported-command set,
  and any repository-import refuses replay.
- `resolve_replay` mints an explicit `macro_replay_resolution` that **declares its
  target scope**, **refuses unsafe reuse on a context mismatch**, and **re-resolves
  the supported-command set** every time.

## Profile-local by default, promotion is explicit

Recorded macros are profile-local by default — they live in the user or workspace
scope and are never distributed on the managed-only channel. When a macro's declared
target scope **crosses files** (`multi_file_scope` or `workspace_scope`), it needs
broader review: it carries `macro_replay_blocked_promotion_required_crosses_scope`
so direct replay fails closed, and it must be **promoted to a declarative recipe** —
an explicit step the user takes, never a silent forward.

Repository content can never define an executable macro. A session marked
`imported_from_repository_content` always resolves to
`macro_replay_blocked_imported_from_repository_content`, so importing repository
content never silently installs a runnable macro into a user profile.

## First consumers and the fail-closed gate

`MacroRecorderFirstConsumersPacket` binds the first M5 automation families that
render a macro recorder — notebook, task/test/debug, request/API, package, incident,
and the AI assistant — each to a seeded panel of sessions, and pins nine freeze
invariants. `MacroRecorderFirstConsumersPacket::validate` recomputes the findings so
the typed Rust consumer and the Python gate agree. A dropped entrypoint, an empty
panel, a replay that implies stale context, an unsupported command that does not
block save, a repository-imported macro, a non-explicit cross-scope promotion, an
ambient or managed-only capture, an inconsistent replay-resolution projection, a
non-profile-local default, a raw secret, or a violated invariant **blocks stable**.

The packet projects a redacted **support export** (carrying the per-session scope,
state, replay action, and blocker classes plus the resolved replay resolutions) and
a compact **CLI / headless** view. The worked-example fixtures
([`macro_session_export_roundtrip.json`](../../fixtures/automation/m5/macro-recorder/macro_session_export_roundtrip.json),
[`cross_scope_macro_requires_promotion.json`](../../fixtures/automation/m5/macro-recorder/cross_scope_macro_requires_promotion.json),
[`unsupported_command_blocks_save.json`](../../fixtures/automation/m5/macro-recorder/unsupported_command_blocks_save.json),
and
[`replay_fails_closed_on_context_mismatch.json`](../../fixtures/automation/m5/macro-recorder/replay_fails_closed_on_context_mismatch.json))
prove the export round-trips, a cross-scope macro requires promotion, an unsupported
command blocks save, and replay fails closed on a context mismatch.

## Regenerating and verifying

The artifacts and fixtures are bit-for-bit derivable from the frozen seed:

```sh
cargo run -q -p aureline-runtime --example dump_m5_macro_recorder
cargo test -p aureline-runtime --test m5_macro_recorder
python3 tools/ci/m5/macro_recorder_check.py --repo-root .
```

Adding a new recorder state, recorded surface, replay-posture, or
promotion-affordance value is additive-minor and bumps the macro-session schema
version; repurposing an existing value is breaking and requires a new decision row.
