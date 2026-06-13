# Prompt-Composer Session-Artifact Fixtures

## session_artifacts.json

A session-artifact catalogue captured across the M5 AI surfaces, holding one row
per prompt-composer draft or session so the lane coverage stays complete on the
desktop composer, review workspace, and browser companion surfaces.

The inline assist edit (`composer-inline-edit`) is an active session artifact at
Stable on the desktop composer: workspace-scoped and workspace-local, durable
until deleted with a workspace-scoped delete/export posture. Its provenance
records a primary workspace-file attachment and a docs-pack reference, a resolved
symbol mention with a target alongside a scope-excluded file mention that stays
out of scope, and two replay-visible context receipts — one user addition and one
budget omission with a precise reason. Its evidence is fully replay-safe and
needs no raw prompt text.

The patch review session (`review-patch-session`) is an active session artifact
at Beta on the review workspace, retained until the user revokes consent with a
user-scoped delete posture and a redacted replay-safe evidence lineage.

The docs companion draft (`browser-docs-draft`) is the scoped case: a
draft-in-progress at Preview on the browser companion, thread-scoped and
session-only so it stays scoped rather than drifting into durable memory. Its
evidence is redacted replay-safe and raw-prompt-free.

The deleted composer session (`composer-deleted-session`) is a deleted tombstone
at Held: its body is gone but its scope-revocation context receipt stays
replay-visible so support and audit can still explain what was filtered out, and
its evidence preserves a replay lineage without reverting.

This demonstrates that every artifact preserves attachment origin, mention
resolution, and context add-or-remove receipts; that a removal or omission names
a precise reason rather than an unspecified catch-all; that a draft stays scoped
and locality-safe; and that the evidence lineage stays replay-safe and
raw-prompt-free even after the artifact is deleted.
