# M5 Diagnostic-Decoration and Code-Action-Chip Controls

- Packet: `m5-diagnostic-decoration-code-action-chip-controls:stable:0001`
- Label: `M5 diagnostic-decoration and code-action-chip controls with severity/source/freshness, exact-versus-inferred fix posture, preview-required apply scope, blocked-action reasons, and side-effect class aligned across editor, diagnostics, notebook, AI, support, and product surfaces`
- Consumer surfaces: 6
- Diagnostic source classes: language_server, compiler, linter, test_runner, imported_external, source_unknown
- Code-action apply scopes: preview_required, review_required, direct_apply, blocked, not_applicable, scope_unresolved
- Proof freshness SLO: 720 hours (last refresh: 2026-07-12T00:00:00Z)

## Consumer surfaces

- **editor_ui**: `stable`
  - Owner: Editor surface owner
  - Scope: The editor names problem severity and source on diagnostic decorations with no-color-only semantics and offers exact-versus-inferred code-action chips; both degrade honestly when severity is encoded by color alone or a fix posture is
  - Diagnostic examples: 3 / chip examples: 3
- **diagnostics_ui**: `stable`
  - Owner: Diagnostics surface owner
  - Scope: The diagnostics surface correlates underlines, markers, and panel entries through one severity/source/freshness vocabulary and degrades honestly when severity, freshness, or a fix's apply scope cannot be resolved or a stale diagnostic is shown as current
  - Diagnostic examples: 5 / chip examples: 2
- **notebook_ui**: `stable`
  - Owner: Notebook code-pane owner
  - Scope: The notebook reuses the same diagnostic decoration and code-action chip grammar in code cells, discloses staleness rather than reading as current, and degrades honestly when an anchor silently drifts or a multi-file fix hides its side effect
  - Diagnostic examples: 2 / chip examples: 2
- **ai_ui**: `stable`
  - Owner: AI surface owner
  - Scope: AI surfaces never imply native certainty for an imported diagnostic and never present an inferred fix as exact; blocked and external-state actions name their reason and side effect, degrading honestly when an imported diagnostic overstates certainty or an inferred fix reads as exact
  - Diagnostic examples: 2 / chip examples: 3
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved diagnostic and fix truth, so a color-only severity, a broken Problems linkage, a hidden block reason, or a bypassed preview is visible in evidence rather than hidden behind compact chrome
  - Diagnostic examples: 3 / chip examples: 5
- **product_ui**: `stable`
  - Owner: In-product editor owner
  - Scope: In-product surfaces reuse the same diagnostic and fix grammar a user sees in the editor, always offering the command-backed detail/preview path and degrading honestly when the trace path is missing, the source or anchor is unresolved, or a fix's side-effect class is unresolved
  - Diagnostic examples: 5 / chip examples: 3
