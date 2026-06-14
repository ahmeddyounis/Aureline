# M5 mode strips, leader-sequence guides, register/clipboard pickers, and capability-gap banners

This contract is the first consumer that binds the frozen M5 keyboard-mode
taxonomy to the **live keyboard posture** a user actually sees on a claimed M5
surface. Where the
[keyboard-mode / clipboard-route / drag-drop-verb / grouped-history continuity
matrix](./freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist.md)
freezes the canonical vocabulary, this contract renders that vocabulary as
on-surface state: a mode strip, a leader-sequence guide, a register/clipboard
picker, and a capability-gap banner per surface.

Aureline's switching wedge depends on keyboard-first, recoverable interaction
across every new M5 surface — notebook, data/API, preview, docs, and
companion-adjacent panes — not just the editor core. Imported Vim, Neovim, Emacs,
and Helix workflows only stay migration-friendly when the current mode, the
pending sequence, the register/clipboard route, and any capability gap are
**explicit, reachable product state** rather than ad hoc per-surface behavior.

## What a strip records

Each [`SurfaceModeStrip`](../../../crates/aureline-shell/src/mode_strip_leader_sequence_register_picker_and_capability_gap_banner_surfaces/mod.rs)
binds a durable surface subject (reusing the frozen `KeyboardSurfaceSubject`,
keyed by surface kind, origin class, and a non-display fingerprint) to:

- a **keymap source preset** (`vim_preset`, `neovim_preset`, `emacs_preset`,
  `helix_preset`, `non_modal_default`, `imported_custom_preset`) — the imported
  workflow the surface advertises, named explicitly so keys never silently change
  meaning;
- a **visible current mode** (`normal_mode`, `insert_mode`, `operator_pending_mode`,
  `read_only_navigation_mode`, …, or `mode_unknown_downgraded`) plus the frozen
  mode-strip class it realizes;
- a **pending-sequence state**: the operator and sequence guide tokens in flight
  (reviewable labels, never raw key buffers), the numeric count prefix, the
  timeout posture (`waits_for_explicit_completion`, `timeout_cancels_pending`,
  `timeout_commits_longest_prefix`, `ambiguity_held_for_explicit_choice`), and the
  resolution (`unambiguous`, `awaiting_disambiguation`, `resolved_longest_match`,
  `unsupported_downgraded`);
- a **register/clipboard picker**: the frozen clipboard-route class, the active and
  selectable register tokens, the plain-text and sensitive-copy posture, and
  whether the picker is keyboard-reachable;
- an **accessibility block**: whether mode changes and the pending sequence are
  announced to assistive technology and reachable by keyboard, and that the strip
  is not hover-only;
- zero or more **capability-gap banners**, each naming a gap kind
  (`modal_sequence_unsupported`, `named_register_unsupported`, …), a disposition
  (`narrowed_to_supported_subset` or `rejected_outright`), a precise explanation, an
  export-safe reason token, and its reachability posture;
- a **reopenable verification proof** (reused `AxisVerification`) and a claimed and
  effective `ContinuityParityGrade`.

## Auto-downgrade

A strip auto-downgrades — its effective grade ranks strictly below its claim and
it records a downgrade trigger and a precise label — whenever:

- the visible current mode is unidentified (`mode_unknown_downgraded` or a
  downgraded mode-strip class);
- a claimed leader sequence is unsupported (`unsupported_downgraded`);
- the clipboard route drops the plain-text representation (`rich_only_denied`);
- the surface stops being keyboard-complete or its macro replay stops being
  explicit; or
- its verification proof is stale, missing, review-pending, or imported proof
  standing in for a local claim.

Any **narrowed or rejected affordance** (mode, sequence, register/clipboard route)
must carry a capability-gap banner explaining it — a surface never narrows
silently. Verification-freshness downgrades are governed separately and do not
require an affordance banner, because no affordance was narrowed.

## Guardrails

`ModeStripSurfacePacket::validate` refuses a packet that:

- lets an unsupported modal sequence read as silently approximated
  (`sequence_silently_approximated`);
- lets a rich-only copy become the only clipboard representation
  (`clipboard_plain_text_lost`);
- narrows or rejects an affordance with no capability-gap banner
  (`capability_gap_missing_for_narrowing`);
- hides a capability gap or mode change behind hover-only UI
  (`capability_gap_not_reachable`, `mode_changes_not_reachable`); or
- lets a provider-linked / imported surface read as a locally verified one
  (`imported_reads_as_local`).

Mode changes and sequence ambiguity are always keyboard- and screen-reader-
reachable. Raw clipboard bodies, raw key buffers, raw provider payloads, file
contents, private paths, and credentials never cross this boundary; the packet
carries only typed class tokens, booleans, opaque ids, fingerprint digests, and
reviewable labels.

## Consumers

Product, help/migration, accessibility, diagnostics, and support-export surfaces
ingest these strips directly rather than re-narrating keyboard-mode posture by
hand. Because every strip is export-safe and verification-bound, migration, help,
and support tooling can reconstruct the exact keyboard-mode and capability-gap
posture a user saw on a claimed M5 surface — including which affordance narrowed
or was rejected and why.

## Artifacts

- Schema: `schemas/interaction/implement-mode-strips-leader-sequence-guides-register-pickers-and-capability-gap-banners.schema.json`
- Support export: `artifacts/interaction/m5/implement-mode-strips-leader-sequence-guides-register-pickers-and-capability-gap-banners/support_export.json`
- Markdown summary: `artifacts/interaction/m5/implement-mode-strips-leader-sequence-guides-register-pickers-and-capability-gap-banners.md`
- Fixtures: `fixtures/interaction/m5/implement-mode-strips-leader-sequence-guides-register-pickers-and-capability-gap-banners/`
- Conformance dump: `cargo run -p aureline-shell --example dump_mode_strip_surfaces`
