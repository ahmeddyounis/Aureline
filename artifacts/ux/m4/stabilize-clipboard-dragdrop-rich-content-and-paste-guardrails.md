# Clipboard, Drag/Drop, Rich Content, Paste Guardrails, and Undo Lineage — Release Evidence

Reviewer-facing evidence packet for the stable transfer-safety lane: one
canonical `transfer_safety_packet` covers representation truth, sensitive copy,
remote clipboard policy, high-risk paste, drag/drop verb cues, cross-window
split semantics, large-transfer feedback, rich-content trust, and named undo
lineage across editor, terminal, notebook, docs, shell, and support flows.

Canonical machine sources:

- Records / fixtures: [`/fixtures/ux/m4/stabilize-clipboard-dragdrop-rich-content-and-paste-guardrails/`](../../../fixtures/ux/m4/stabilize-clipboard-dragdrop-rich-content-and-paste-guardrails/)
- Schema: [`/schemas/ux/transfer-safety.schema.json`](../../../schemas/ux/transfer-safety.schema.json)
- Companion doc: [`/docs/ux/m4/stabilize-clipboard-dragdrop-rich-content-and-paste-guardrails.md`](../../../docs/ux/m4/stabilize-clipboard-dragdrop-rich-content-and-paste-guardrails.md)
- Typed source: `aureline_editor::stabilize_clipboard_dragdrop_rich_content_and_paste_guardrails`
- Headless emitter: `aureline_transfer_safety`
- Replay + invariant gate: `crates/aureline-editor/tests/transfer_safety_replay.rs`

## Claimed-Stable Matrix

| Record | Surface | Action | Sensitive review | Drop preview | Paste guard | Large transfer | Named undo |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `editor_rich_copy_preserves_raw.json` | editor | copy | no | no | no | no | no |
| `support_sensitive_copy_preview.json` | support | copy | yes | no | no | no | no |
| `terminal_remote_clipboard_policy.json` | terminal | copy | yes | no | no | no | no |
| `terminal_multiline_paste_guard.json` | terminal | paste | yes | no | yes | no | yes |
| `shell_drag_drop_import_preview.json` | shell | drag/drop | yes | yes | no | yes | yes |
| `shell_cross_window_split_preview.json` | shell | split | no | yes | no | no | yes |
| `notebook_large_output_attach.json` | notebook | attach | yes | yes | no | yes | yes |
| `docs_sanitized_rich_copy.json` | docs | copy | yes | no | no | no | no |
| `editor_multi_file_replace_undo.json` | editor | multi-file replace | yes | no | no | yes | yes |

## Acceptance Criteria to Evidence

- **Rich-content copy versus raw copy is explicit.**
  `editor_rich_copy_preserves_raw.json` and
  `docs_sanitized_rich_copy.json` prove that rendered copy is an additive action
  and that raw or escaped source copy remains reachable.
- **Sensitive-copy preview is visible before commit.**
  `support_sensitive_copy_preview.json` carries support-link, private-path, and
  token-like content classes with a policy gate before the clipboard changes.
- **Remote clipboard policy is enforced.**
  `terminal_remote_clipboard_policy.json` covers an OSC 52 clipboard write from
  an SSH session; the SSH-to-local boundary and policy outcome are visible
  before commit.
- **Multiline paste guardrails are present.**
  `terminal_multiline_paste_guard.json` proves bracketed paste, disabled
  automatic submit, explicit confirmation, and terminal input-buffer lineage.
- **Drag/drop verb cues are explicit and keyboard reachable.**
  `shell_drag_drop_import_preview.json`,
  `shell_cross_window_split_preview.json`, and
  `notebook_large_output_attach.json` carry a resulting verb, insertion
  indicator, modifier cue, and command fallback id.
- **Named undo groups and history/reopen truth are present for mutations.**
  Paste, dropped import, cross-window split, notebook output attach, and
  multi-file replace rows each carry a named undo group, source attribution,
  mutation journal ref, recovery class, and history surfaces.
- **Large transfers stay interruptible.**
  Dropped import, notebook output attach, and multi-file replace rows all carry
  progress, cancellation, and completion summary fields.
- **Cross-surface consumers share one packet.**
  Every fixture includes `surface_projections[]` whose rows declare that editor,
  terminal, notebook, docs, shell, and support consumers read the shared record
  instead of minting local transfer semantics.

## Verification

```sh
# Regenerate fixtures from the in-code corpus
cargo run -p aureline-editor --bin aureline_transfer_safety

# Fixture replay + acceptance-criteria invariants
cargo test -p aureline-editor --test transfer_safety_replay
```

The fixtures are a literal projection of the Rust corpus. The replay gate fails
when checked-in JSON drifts from the live packet serialization or when coverage
for raw/rich copy, sensitive copy, remote clipboard policy, multiline paste,
drag/drop verb cues, large-transfer feedback, or named undo lineage disappears.
