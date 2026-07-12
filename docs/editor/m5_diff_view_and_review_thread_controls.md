# M5 diff-view and review-thread controls

This is the third **implement lane** over the frozen
[M5 editor-inline component matrix](../../schemas/ui/m5-editor-inline-component-matrix.schema.json)
(see the [component contract](m5_editor_inline_components_contract.md)), after the
[editor-tab / gutter lane](m5_editor_tab_and_gutter_controls.md) and the
[diagnostic-decoration / code-action-chip lane](m5_diagnostic_decoration_and_code_action_chip_controls.md).
It turns the two inline *review-flow* components — the **diff view** and the **review thread** — into
resolvers that produce export-safe, honest projections across the claimed M5 editor, diff, review,
notebook, support, and product surfaces.

- Rust source: `crates/aureline-editor/src/m5_diff_view_and_review_thread_anchor_durability_and_review_state/`
- Combined schema: [`schemas/ui/m5-diff-view-review-thread-controls.schema.json`](../../schemas/ui/m5-diff-view-review-thread-controls.schema.json)
- Per-component schemas: [`m5-diff-view.schema.json`](../../schemas/ui/m5-diff-view.schema.json),
  [`m5-review-thread.schema.json`](../../schemas/ui/m5-review-thread.schema.json)
- Proof packet: `artifacts/release/m5-diff-view-review-thread-controls-proof/`
- Narrowed fixtures: `fixtures/ui/m5-diff-view-review-thread-controls/`

The Rust validator in `crates/aureline-editor` is the authoritative gate; this doc and the schema
document the shape.

## What the resolvers guarantee

### `resolve_diff_view`

A diff view reads as a clean, legible state only when it names:

- the **hunk identity / label**, never unstated;
- the **change kind** (added, removed, modified, moved, conflicted, or unchanged context), never
  collapsed into an ambiguous generic change;
- the **context visibility** (full, collapsed, elided, moved, or re-anchored), never hiding a moved
  region or pretending one immutable view over collapsed / elided context;
- the **source-versus-rendered relationship** (source-exact, rendered-faithful, rendered-approximate,
  rendered-transformed, or binary / opaque), never blurring a rendered / transformed diff with the
  exact source bytes;
- the **stable hunk identity** (stable, rebased, synthesized, merged, or unstable), never silently
  drifting across rebases or re-renders;
- an **inspectable, export-safe structural summary**, never an opaque blob.

It degrades — never silently passes — when the identity is unstated, the change kind is collapsed, the
context visibility is unresolved, a moved region is hidden, collapsed / elided context is not
disclosed, the source rendering is unresolved or blurred with the source, the hunk identity is
unresolved or silently drifted, the structural summary is opaque, or no command-backed detail path is
reachable.

### `resolve_review_thread`

A review thread reads as a clean, legible state only when it names:

- the **controlled thread state** (draft, published, resolved, outdated, re-anchored, locked, or
  pending-send) using **one shared vocabulary**, never encoded by color or provider-specific jargon;
- the **outdated-versus-resolved distinction**, never blurring the two;
- the **comment-anchor durability** (exact, re-anchored, drifted, outdated, or orphaned), never
  silently drifting;
- the **provider-local-versus-provider-hosted locality** (provider-local, provider-hosted, mirrored,
  handoff-pending, or detached-export), kept **explicit** so desktop, browser handoff, and exported
  review packets do not drift on comment truth;
- a draft / pending-send comment as **unsent**, never reading as published.

It degrades when the identity is unstated, the thread state is unresolved or color / jargon-only,
outdated and resolved are blurred, the anchor is unresolved or silently drifted, the provider locality
is unresolved or implicit, a draft / pending-send thread reads as published, or no command-backed
detail path is reachable.

## Hard invariants

Every controls row carries four hard invariants that must stay `false`:

- `diff_moved_or_hidden_context_pretends_immutable_view`
- `diff_hunk_identity_or_source_rendering_silently_drifts`
- `review_outdated_and_resolved_state_blurred`
- `review_anchor_or_provider_locality_silently_drifts`

## Acceptance criteria, proven by examples

The packet's `validate()` proves each acceptance criterion against the resolved examples rather than
merely asserting a governance bool:

1. **Claimed M5 review flows expose the same thread-state grammar and anchor-durability behavior across
   desktop, browser handoff, and exported packets.** Clean threads cover at least two distinct thread
   states and two distinct anchor durabilities, span provider-local and provider-hosted localities, a
   color-only thread-state example degrades, and no clean thread is color-only.
2. **Diff consumers remain honest when context is moved, elided, collapsed, or re-anchored rather than
   pretending one immutable view.** Clean diffs cover at least two distinct context visibilities, a
   moved-context-hidden example degrades, a hidden-context example degrades, and no clean diff hides a
   moved or elided region.
3. **Users can distinguish outdated from resolved state without relying on color or provider-specific
   jargon.** At least one clean outdated thread and one clean resolved thread exist, an
   outdated-resolved-blurred example degrades, no clean thread blurs the two, and a clean diff and clean
   thread both expose a command-backed detail entrypoint.

## Regenerating the artifacts

The headless emitter is the only mint-from-truth path:

```text
cargo run -p aureline-editor --example dump_m5_diff_review_controls -- support-export
cargo run -p aureline-editor --example dump_m5_diff_review_controls -- report
cargo run -p aureline-editor --example dump_m5_diff_review_controls -- csv
cargo run -p aureline-editor --example dump_m5_diff_review_controls -- fixture-diff-ui-beta-narrowed
cargo run -p aureline-editor --example dump_m5_diff_review_controls -- fixture-review-ui-preview-narrowed
```
