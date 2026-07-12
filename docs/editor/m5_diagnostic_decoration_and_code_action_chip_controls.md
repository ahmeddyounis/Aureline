# M5 diagnostic-decoration and code-action-chip controls

This is the second **implement lane** over the frozen
[M5 editor-inline component matrix](../../schemas/ui/m5-editor-inline-component-matrix.schema.json)
(see the [component contract](m5_editor_inline_components_contract.md)), after the
[editor-tab / gutter lane](m5_editor_tab_and_gutter_controls.md). It turns the two inline
*problem-and-action* components — the **diagnostic decoration** and the **code-action chip** — into
resolvers that produce export-safe, honest projections across the claimed M5 editor, diagnostics,
notebook, AI, support, and product surfaces.

- Rust source: `crates/aureline-editor/src/m5_diagnostic_decoration_and_code_action_chip_state_and_fix_posture/`
- Combined schema: [`schemas/ui/m5-diagnostic-decoration-code-action-chip-controls.schema.json`](../../schemas/ui/m5-diagnostic-decoration-code-action-chip-controls.schema.json)
- Per-component schemas: [`m5-diagnostic-decoration.schema.json`](../../schemas/ui/m5-diagnostic-decoration.schema.json),
  [`m5-code-action-chip.schema.json`](../../schemas/ui/m5-code-action-chip.schema.json)
- Proof packet: `artifacts/release/m5-diagnostic-decoration-code-action-chip-controls-proof/`
- Narrowed fixtures: `fixtures/ui/m5-diagnostic-decoration-code-action-chip-controls/`

The Rust validator in `crates/aureline-editor` is the authoritative gate; this doc and the schema
document the shape.

## What the resolvers guarantee

### `resolve_diagnostic_decoration`

A diagnostic decoration reads as a clean, legible state only when it names:

- the **problem identity / message**, never unstated;
- the **severity** (error, warning, info, hint, stale, or unknown) with **no-color-only** semantics
  (a name or icon-with-label, never a bare colored underline);
- the **source / provider class** (language server, compiler, linter, test runner, or imported /
  external), never unresolved, so a diagnostic's evidence class is legible;
- the **freshness** (current, stale, recomputing, superseded, or never-computed), never showing a
  stale / superseded diagnostic as current;
- the **anchor durability** (exact, re-anchored, drifted, outdated, or orphaned), never silently
  drifting;
- the **stable linkage** to Problems / output / support, never a bare floating underline.

It degrades — never silently passes — when the identity is unstated, the severity is unresolved or
color-only, the source is unstated, the freshness is unresolved, a stale diagnostic is shown as
current, the anchor is unresolved or silently drifted, the linkage target is unresolved or broken, an
**imported** diagnostic overstates its certainty relative to a native run, or no command-backed
detail path is reachable.

### `resolve_code_action_chip`

A code-action chip reads as a clean, invocable state only when it names:

- the **exact-versus-inferred fix posture** (exact, inferred, heuristic, multiple candidates, or not
  applicable) with **no-color-only** semantics, never presenting an inferred / heuristic fix as exact;
- the **preview-required apply scope** (preview-required, review-required, direct-apply, blocked, or
  not-applicable), never bypassing the preview / apply truth established elsewhere in the sheet;
- the **blocked-action reason** (policy-denied, precondition-unmet, conflicting-change, or
  insufficient-capability) whenever the action is blocked;
- the **side-effect class** (single-file, multi-file, workspace-wide, external-state, or
  irreversible), disclosed whenever a fix touches multiple files or external state.

It degrades when the identity is unstated, the posture is unresolved or color-only, an inferred fix is
presented as exact, the apply scope is unresolved, a preview-required action bypasses its preview, a
blocked action hides its reason, the side-effect class is unresolved or hidden for a multi-file /
external-state fix, or no command-backed detail path is reachable.

## Hard invariants

Every controls row carries four hard invariants that must stay `false`:

- `diagnostic_severity_or_source_encoded_by_color_alone`
- `diagnostic_anchor_or_freshness_silently_drifts`
- `inferred_or_blocked_fix_presented_as_exact_or_ready`
- `code_action_bypasses_preview_or_apply_truth`

## Acceptance criteria, proven by examples

The packet's `validate()` proves each acceptance criterion against the resolved examples rather than
merely asserting a governance bool:

1. **One severity / source / freshness vocabulary correlates underlines, markers, chips, and panel
   entries.** Clean decorations cover at least two distinct severities and two distinct sources, a
   color-only-severity example degrades, a stale-shown-as-current example degrades, and no clean
   decoration is color-only.
2. **Users can tell whether a fix is exact, inferred, blocked, or review-required before invoking
   it.** Clean chips cover at least two distinct fix postures and two distinct apply scopes, an
   inferred-shown-as-exact example degrades, and no clean chip shows an inferred fix as exact.
3. **No claimed inline action path bypasses the broader preview / apply truth.** At least one clean
   chip requires and offers a preview, a preview-bypass example degrades, no clean chip bypasses a
   required preview, and a clean decoration and clean chip both expose a command-backed detail
   entrypoint.

## Regenerating the artifacts

The headless emitter is the only mint-from-truth path:

```text
cargo run -p aureline-editor --example dump_m5_diagnostic_chip_controls -- support-export
cargo run -p aureline-editor --example dump_m5_diagnostic_chip_controls -- report
cargo run -p aureline-editor --example dump_m5_diagnostic_chip_controls -- csv
cargo run -p aureline-editor --example dump_m5_diagnostic_chip_controls -- fixture-diagnostics-ui-beta-narrowed
cargo run -p aureline-editor --example dump_m5_diagnostic_chip_controls -- fixture-ai-ui-preview-narrowed
```
