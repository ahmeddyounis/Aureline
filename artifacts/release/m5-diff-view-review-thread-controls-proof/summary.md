# M5 Diff-View and Review-Thread Controls

- Packet: `m5-diff-view-review-thread-controls:stable:0001`
- Label: `M5 diff-view and review-thread controls with change-kind, moved-versus-hidden context, source-versus-rendered truth, stable hunk identity, one thread-state vocabulary, comment-anchor durability, and provider-local-versus-provider-hosted parity aligned across editor, diff, review, notebook, support, and product surfaces`
- Consumer surfaces: 6
- Review thread states: draft, published, resolved, outdated, re_anchored, locked, pending_send, state_unknown
- Diff context visibilities: full_context, collapsed_context, elided_context, moved_context, re_anchored_context, visibility_unresolved
- Proof freshness SLO: 720 hours (last refresh: 2026-07-12T00:00:00Z)

## Consumer surfaces

- **editor_ui**: `stable`
  - Owner: Editor surface owner
  - Scope: The editor names diff change kinds and moved-versus-hidden context with no collapsed generic change, and shows draft-versus-published review threads with one controlled vocabulary; both degrade honestly when a change kind is collapsed or a thread state is encoded by color alone
  - Diff examples: 3 / thread examples: 3
- **diff_ui**: `stable`
  - Owner: Diff surface owner
  - Scope: The diff surface stays honest when context is moved, elided, collapsed, or re-anchored rather than pretending one immutable view, and degrades honestly when a moved region is hidden or collapsed context is not disclosed
  - Diff examples: 5 / thread examples: 2
- **review_ui**: `stable`
  - Owner: Review surface owner
  - Scope: The review surface exposes the same thread-state grammar and anchor durability across desktop, browser handoff, and exported packets, keeps outdated and resolved distinct without color, and degrades honestly when the two are blurred or an anchor silently drifts
  - Diff examples: 2 / thread examples: 4
- **notebook_ui**: `stable`
  - Owner: Notebook review owner
  - Scope: The notebook reuses the same diff and review-thread grammar in code cells, discloses a rebased hunk id rather than reading as stable, and degrades honestly when a hunk identity silently drifts or a draft reads as published
  - Diff examples: 2 / thread examples: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved diff and thread truth, so a rendered-versus-source blur, an opaque summary, an implicit provider locality, or an unstated identity is visible in evidence rather than hidden behind compact chrome
  - Diff examples: 4 / thread examples: 3
- **product_ui**: `stable`
  - Owner: In-product review owner
  - Scope: In-product surfaces reuse the same diff and thread grammar a user sees in the editor, always offering the command-backed detail path and degrading honestly when the trace path is missing, the rendering or hunk identity is unresolved, or the provider locality is unresolved
  - Diff examples: 4 / thread examples: 4
