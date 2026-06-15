# M5 evidence pointer — local extension workspace strips, unsigned/local-only truth, runtime-class badges, and hot-reload state

Reviewer contract for the canonical M5 local extension workspace strips that give an
author the always-on authoring chrome for each marketed M5 ecosystem artifact family:
package identity, source path, workspace origin, runtime class, host/ABI, signing state,
trust badge, build freshness (last-built), and load state (last-loaded and
hot-reload/relaunch posture). This row is a depth-lane proof governed by the canonical M5
evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).

## Canonical artifacts

- Truth packet: `artifacts/ecosystem/m5/m5-local-workspace-strip.json`
- Boundary schema: `schemas/ecosystem/m5-local-workspace-strip.schema.json`
- Reviewer contract: `docs/m5/implement-local-extension-workspace-strips-unsigned-or-local-only-truth-runtime-class-badges-and-hot-reload-state.md`
- Human-readable rendering: `artifacts/m5/implement-local-extension-workspace-strips-unsigned-or-local-only-truth-runtime-class-badges-and-hot-reload-state.md`
- Overview companion: `docs/ecosystem/m5/m5-local-workspace-strip.md`
- Fixture corpus: `fixtures/ecosystem/m5/m5-local-workspace-strip/`
- Owning crate module: `crates/aureline-ecosystem/src/m5_workspace_strip/`

## Reuses the frozen publish-preview gate

The local workspace strip is the always-on authoring chrome that complements the
publish-preview gate
(`artifacts/ecosystem/m5/m5-author-and-publish-preview.json`). The packet reuses the
closed artifact-family, runtime-class, host/ABI, signing-state, trust-posture, and
hot-reload vocabulary frozen by that gate — one strip per marketed family — rather than
minting a parallel set, so the strip and the publish preview describe the same artifact.

## What the strip proves

- **Local builds never inherit a trusted badge.** The strip caps the rendered trust
  posture by both the signing state and the workspace origin. A `local_dev_workspace` or
  `sideloaded_workspace` origin renders `unsigned_local_only` even when the artifact is
  `signed_verified` on a trusted machine; an `unsigned_local_dev`, `unsigned_sideload`,
  or `revoked_signature` artifact renders `unsigned_local_only` regardless of origin. The
  fixture exercises this with a signed-verified recipe pack in a local-dev workspace, an
  unsigned local-model pack, and a revoked mirror-backed variant — all capped to
  local-only.
- **Local-only is distinguished from published and mirror-backed.** Every strip names
  its origin, so authoring surfaces can tell a local-only artifact from a published or
  mirror-backed one at a glance.
- **Hot reload cannot widen authority silently.** A hot reload that would widen the
  runtime class, add an external executable, or expand permissions holds the running
  instance in `reload_held_for_review` until a fresh review clears it.
- **Runtime class and host/ABI are never hidden.** Both are required fields on every
  strip, so a change that affects compatibility or publish readiness is always visible.
- **The strip never outruns the gate.** `cross_check_matrix()` proves every strip renders
  no stronger a badge than the publish-preview gate would grant the same family.

## Executable proof

`crates/aureline-ecosystem/src/m5_workspace_strip/tests.rs` loads the embedded packet,
asserts it validates with zero violations, proves every closed vocabulary is
exhaustively exercised, asserts the non-inheritance, hot-reload-held, and build/load
coherence guardrails, and cross-checks the strip board against the publish-preview gate.
`M5LocalWorkspaceStripBoard::validate()` is the CI-facing gate that flags any overstated
rendered badge, inherited trust, silently-widened hot reload, incoherent build/load
state, or summary drift.

## Freshness

The packet is current as of the `as_of` date embedded in the JSON artifact. CI gates
recompute the strip decision against the embedded strips and fail if the board is stale
or underqualified.
