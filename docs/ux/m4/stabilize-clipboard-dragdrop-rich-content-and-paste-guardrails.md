# Clipboard, Drag/Drop, Rich Content, Paste Guardrails, and Undo Lineage

This is the reviewer-facing companion for the stable transfer-safety lane. It
freezes one governed packet for copy, paste, drag/drop, rich-content trust,
large transfer feedback, and named undo-group lineage across editor, terminal,
notebook, docs, shell, and support surfaces.

Do not clone status text from this doc. Ingest the canonical machine sources:

- Records / fixtures:
  [`/fixtures/ux/m4/stabilize-clipboard-dragdrop-rich-content-and-paste-guardrails/`](../../../fixtures/ux/m4/stabilize-clipboard-dragdrop-rich-content-and-paste-guardrails/)
- Schema:
  [`/schemas/ux/transfer-safety.schema.json`](../../../schemas/ux/transfer-safety.schema.json)
- Release-evidence packet:
  [`/artifacts/ux/m4/stabilize-clipboard-dragdrop-rich-content-and-paste-guardrails.md`](../../../artifacts/ux/m4/stabilize-clipboard-dragdrop-rich-content-and-paste-guardrails.md)
- Typed source:
  `aureline_editor::stabilize_clipboard_dragdrop_rich_content_and_paste_guardrails`
- Headless emitter:
  `aureline_transfer_safety`
- Replay + invariant gate:
  `crates/aureline-editor/tests/transfer_safety_replay.rs`

## Why one governed packet

Transfer behavior is infrastructure. A generic Copy, Paste, or Drop action can
cross trust and authority boundaries faster than the user can inspect them:
rendered rich content can lose raw source truth, remote terminals can write the
local clipboard, multiline shell input can auto-submit, and broad paste/drop
mutations can disappear into one opaque undo row.

The `transfer_safety_packet` binds those concerns into one record:

- **Representation truth.** The packet names whether the default is raw,
  rendered, escaped, sanitized, sandboxed, generated, or metadata-only. The
  default preserves useful plain text, and rich/rendered copy is additive.
- **Sensitive review.** Token-like values, certificate fingerprints, private
  paths, support links, multiline shell input, remote clipboard bridges, and
  broad replace/import operations carry a visible pre-commit label and policy
  gate.
- **Boundary context.** Local, SSH, container, managed-workspace, browser, and
  support-export boundaries are shown before risky transfer commits.
- **Paste guardrails.** Multiline terminal paste proves bracketed paste,
  disabled automatic submit, and explicit confirmation.
- **Drop preview.** Move, copy, attach, open, import, and split drops show the
  resulting verb, insertion indicator, modifier-key meaning, and keyboard route.
- **Named undo truth.** Mutating transfer actions carry one named undo group,
  source attribution, mutation journal reference, recovery class, and history /
  reopen surfaces.
- **Large transfer feedback.** Large paste/import/attach/replace paths show
  progress, cancellation, and a post-action summary.
- **Rich-content trust.** Sanitized or active rich surfaces expose the trust
  class, raw-source inspection, and Copy plain text.

## Claimed-Stable Matrix

| Record | Surface | Action | Demonstrates |
| --- | --- | --- | --- |
| `editor_rich_copy_preserves_raw.json` | editor | copy | default raw copy with explicit rendered and escaped alternatives |
| `support_sensitive_copy_preview.json` | support | copy | support link, private path, and token-like copy review before clipboard mutation |
| `terminal_remote_clipboard_policy.json` | terminal | copy | OSC 52 remote clipboard write is policy-aware and boundary-labeled |
| `terminal_multiline_paste_guard.json` | terminal | paste | production-labeled multiline paste uses bracketed paste and no automatic submit |
| `shell_drag_drop_import_preview.json` | shell | drag/drop | import verb, insertion indicator, modifier cues, progress, and undo group |
| `shell_cross_window_split_preview.json` | shell | split | cross-window tab detach preserves explicit split semantics and keyboard parity |
| `notebook_large_output_attach.json` | notebook | attach | large sanitized output attach is cancellable and source-attributed |
| `docs_sanitized_rich_copy.json` | docs | copy | rendered docs copy is labeled and raw Markdown remains reachable |
| `editor_multi_file_replace_undo.json` | editor | multi-file replace | broad replace creates one named undo group with checkpoint and reopen lineage |

## How the Invariants Are Derived

The builder computes the packet validity instead of trusting fixture prose:

- A packet fails if default copy does not preserve useful plain text or expose a
  raw/escaped alternative.
- A rendered copy packet fails if raw or escaped source alternatives are hidden.
- Sensitive or boundary-crossing packets fail without a visible pre-commit review
  and policy gate.
- Boundary-crossing packets fail when the boundary label is absent before
  commit.
- Paste packets fail without bracketed paste, disabled auto-submit, and explicit
  confirmation.
- Drag/drop packets fail when the target verb, insertion indicator, modifier
  cues, or keyboard command route is missing.
- Mutating packets fail without a named undo group, source attribution, mutation
  journal reference, recovery class, and history/reopen surfaces.
- Large transfers fail without progress, cancellation, and a completion summary.
- Rich-content packets fail without trust posture, raw inspection, and plain-text
  copy.

## Regenerating Fixtures

```sh
cargo run -p aureline-editor --bin aureline_transfer_safety
```

The replay gate fails if the checked-in JSON drifts from the live in-code
projection:

```sh
cargo test -p aureline-editor --test transfer_safety_replay
```
