# Accessibility surface signoff — proof packet

Reviewer-facing proof packet for the accessibility surface signoff register that
binds shell, tree, palette, diff, terminal, debugger, settings, auth, and
recovery to per-dimension qualification across keyboard, screen-reader,
IME/grapheme/bidi, zoom, high-contrast, and reduced-motion for the M4 stable
line.

Canonical machine source (do not clone status text from this packet — ingest the
JSON):

- Register: [`/artifacts/release/stabilize_accessibility_signoff_across_shell_tree_palette_diff_terminal_debugger_settings_auth_and_recovery.json`](../../stabilize_accessibility_signoff_across_shell_tree_palette_diff_terminal_debugger_settings_auth_and_recovery.json)
- Schema: [`/schemas/release/stabilize_accessibility_signoff_across_shell_tree_palette_diff_terminal_debugger_settings_auth_and_recovery.schema.json`](../../../schemas/release/stabilize_accessibility_signoff_across_shell_tree_palette_diff_terminal_debugger_settings_auth_and_recovery.schema.json)
- Companion doc: [`/docs/m4/stabilize-accessibility-signoff-across-shell-tree-palette-diff-terminal-debugger-settings-auth-and-recovery.md`](../../../docs/m4/stabilize-accessibility-signoff-across-shell-tree-palette-diff-terminal-debugger-settings-auth-and-recovery.md)
- Typed consumer: `aureline_release::stabilize_accessibility_signoff_across_shell_tree_palette_diff_terminal_debugger_settings_auth_and_recovery`

## What this packet proves

1. **Every touched surface has exactly one typed signoff row.** Each of the nine
   surfaces the milestone enumerates — shell, tree, palette, diff, terminal,
   debugger, settings, auth, recovery — has an [`AccessibilitySurfaceSignoffRow`]
   binding it to the level it is put forward as (`claim_label`), the level it
   effectively holds after narrowing (`published_label`), its per-dimension
   checks, its proof refs and freshness window, and its owner sign-off.

2. **Every surface reports against all six dimensions.** Each row carries a
   [`DimensionCheck`] for keyboard, screen-reader, IME/grapheme/bidi, zoom,
   high-contrast, and reduced-motion. A dimension in `passed` or `degraded`
   state supports a Stable claim; `partial`, `blocked`, or `pending_evidence`
   forces the surface below the cutline unless an active waiver covers the gap.

3. **A surface with blocked or pending dimensions narrows automatically.** The
   diff, terminal, and recovery surfaces carry blocked or pending-evidence
   dimensions (screen-reader and reduced-motion), so they are narrowed to `beta`
   and carry the `dimension_blocked` gap reason. The debugger surface's proof
   packet is stale, so it narrows to `beta` with the `evidence_stale` reason.
   Four blocking rules fire, so the stable train **holds**.

4. **Downgrade automation narrows unqualified surfaces before publication.** A
   surface that is not qualified, has stale evidence past its freshness window,
   relied on an expired waiver, lost its backing stable claim, or has blocked
   dimensions narrows below the cutline automatically. Every downgrade reason is
   watched by a signoff rule, and the firing rules drive the promotion
   `proceed`/`hold` verdict.

## Proof-index registration

Each row registers under one row of the stable proof index
([`/artifacts/milestones/m3/public_proof_index.md`](../../milestones/m3/public_proof_index.md))
via its `proof_packet.proof_index_ref`, so this lane's proof is anchored to the
public-proof artifact index rather than to ad hoc notes.

## Current posture

At this revision five surfaces hold a Stable claim (shell, tree, palette,
settings, auth) and four are narrowed below the cutline: diff, terminal, and
recovery have blocked or pending-evidence dimensions, and debugger's proof packet
is stale. Their reasons fire four blocking signoff rules, so the stable train
**holds**. That is the honest posture: the repository is pre-implementation and
several complex surfaces do not yet have complete accessibility evidence.

## Accessibility of this lane

The register and its companion doc are text/JSON artifacts: the doc renders as
headed sections and plain Markdown tables (no color-only encoding), and the
machine source carries the same truth so Help/About, the release center, support
exports, docs, and shiproom dashboards ingest one record rather than restating
status text.

## How to refresh

1. Land per-dimension evidence for each surface first; point each dimension
   check's `evidence_ref` at the canonical fixtures.
2. Set each row's `signoff_state`, `active_gap_reasons`, `published_label`, and
   dimension states to the honest posture. A surface that has not captured
   evidence for all six dimensions stays narrowed below the cutline.
3. Recompute the `promotion` block and `summary`, then run
   `cargo test -p aureline-release` and commit the regenerated capture in the
   same change set.
4. If delivery proves a narrower stable claim than planned, narrow the claim and
   update the register — do not paper over the gap with prose.
