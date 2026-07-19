# M5 collaboration-state surface certification (M05-1323)

This contract is the **closing B156 surface-certification capstone** over the frozen M5 collaboration-state
shared-object authority, anchor-drift, convergence, and session-archive matrix
(`m5_collaboration_replica_shared_object_authority_anchor_drift_convergence_and_session_archive_matrix`). Where the
freeze matrix defines the six governed collaboration-state shared-object classes — **CRDT-backed shared text,
sampled presence / cursors / selections, server-ordered comments / annotations / review pins, presenter / follow
state, higher-risk control plane, and sealed session archive** — the M05-1315–1322 implementation lanes resolve
their replica descriptor / shared-object record, unsent-local-edit preservation, anchor-history / rebind review,
compaction-manifest / archive-finalization, degradation-ladder, provenance / freshness, share-eligibility, and
headless-inspect / support-bundle parity registry truth; this capstone **certifies** that the shared-object
authority, convergence, anchor-drift, compaction, and degradation truth holds on every claimed M5 **desktop,
browser-companion, review, incident / support, and audit / export surface** — the authority model, local-truth
preservation, merge / drift semantics, downgrade behavior, anchor-drift history, export posture, and session
provenance / freshness — and auto-narrows any profile that cannot sustain it.

- **Module:** `crates/aureline-ui/src/m5_collaboration_state_surface_certification/`
- **Schema:** `schemas/collaboration/m5-collaboration-state-surface-certification.schema.json`
- **Release proof:** `artifacts/release/m5-collaboration-state-surface-certification/`
  (`support_export.json`, `matrix.csv`) and `…-surface-certification.md`
- **Fixtures:** `fixtures/collaboration/m5-collaboration-state-surface-certification/`

## What the packet certifies

The packet is keyed on the claimed **consumer profile** a shared-session owner, a review / follow consumer, or a
support / export consumer reads a collaboration-state object through, not on the underlying object class it renders:

1. **Fully-certified collaboration-state lane** — a shared session whose authority model, local-truth preservation,
   merge / drift semantics, downgrade behavior, anchor-drift history, export posture, and session provenance /
   freshness all converge on one export-safe, provider-authoritative, internally consistent record identical across
   every consumer. The **only** profile that may certify a `certified_collaboration_state_truth` claim.
2. **Reviewable collaboration-state record structure** — a self-sufficient, inspectable session-bound
   sealed-session-archive / compaction-manifest record; certifies at most `reviewable_collaboration_state_record`.
3. **Unproven-authority-model profile** — a CRDT-backed shared-text replica whose authority model can no longer be
   confirmed disclosed; auto-narrows to `authority_model_unverified_projection`.
4. **Unconfirmed-convergence-state profile** — a presenter / follow convergence state that cannot be confirmed (a
   convergence-degraded state risks reading as awareness-degraded); auto-narrows to
   `convergence_state_unverified_projection`.
5. **Unpreserved-local-truth profile** — a higher-risk control-plane downgrade whose local-unsent preservation
   cannot be confirmed; auto-narrows to `local_truth_preservation_unverified_projection`.
6. **Unresolved-anchor-drift profile** — a server-ordered comment / annotation / review-pin anchor-drift history
   that cannot be confirmed append-only; auto-narrows to `anchor_drift_unverified_projection`.
7. **Undisclosed-export-posture profile** — a sealed session-archive export posture that cannot be proven
   policy-labeled; auto-narrows to `export_posture_unverified_projection`.
8. **Unproven-provenance-freshness profile** — a sampled presence / cursors / selections session whose provenance /
   freshness is unproven; auto-narrows to `provenance_freshness_unverified_projection`.

Each row certifies its profile across **nine truth axes** — visual, keyboard, screen-reader, high-zoom-reflow,
high-contrast, localization, CLI/export, degraded-state, and collaboration-state-truth behavior — and resolves to a
derived verdict:

- **green** — every axis certified, every invariant held, the claimed collaboration-state tier delivered;
- **yellow** — a truth axis is not current, so the collaboration-state claim auto-narrows to the weakest supported
  ceiling with a bound reason and a frozen downgrade trigger;
- **red** (blocked) — a degraded axis is hidden behind a fresh certified claim, a hard invariant breaks, CLI/export
  parity drops, a non-lane profile claims a certified collaboration-state record, or the narrowing is inconsistent.

The eight seeded rows cover all six frozen object classes (CRDT-backed shared text and sealed session archive each
appear on more than one row), so the certification runs across the full matrix rather than a single class.

## Invariants

1. **A degraded axis must produce a visible claim narrowing.** A profile that keeps a
   `certified_collaboration_state_truth` / `reviewable_collaboration_state_record` claim while one of its truth axes
   is not current over-claims and blocks.
2. **Only a fully-certified collaboration-state lane may certify a certified collaboration-state record.** Every
   other profile is at most a reviewable collaboration-state record structure or a narrowed projection.
3. **CLI/export parity is always-on.** Support and automation must always be able to reconstruct the authority
   model, convergence state, local-truth disposition, anchor-drift history, export posture, and provenance /
   freshness as text / JSON / Markdown.
4. **Every B156 hard invariant holds per row.** No profile may let a collaboration replica overwrite the canonical
   local buffer, VFS, or Git truth; discard unsent local edits on a permission downgrade, relay failure, or
   leave-session flow; silently rebind a comment, annotation, or review pin without append-only drift history;
   collapse a convergence-degraded, awareness-degraded, or anchor-unresolved state into a generic stale badge; or
   export an op-log, snapshot, or archive without policy-labeled redaction and actor lineage.
5. **One canonical proof bundle.** Every row cites exactly one canonical collaboration-state convergence matrix
   proof bundle (`artifacts/release/m5-collaboration-convergence-proof/support_export.json`) — the frozen
   collaboration-state convergence matrix proof — so support, docs / help, release, and public-proof surfaces
   consume a single collaboration-state certification source rather than hand-authored prose.

## Boundary

The packet is metadata-only. Raw credentials, plaintext secrets, bearer tokens, endpoint URLs, and private-key
material never cross this boundary; the export carries typed tokens, opaque evidence refs, and repo-relative
paths only.

## Regenerating the artifacts

The checked-in release artifacts and fixtures are byte-locked to the seed builder. To regenerate them after an
intentional change, run:

```
GEN_COLLABORATION_STATE_CERT_ARTIFACTS=1 cargo test -p aureline-ui \
  m5_collaboration_state_surface_certification::tests::regenerate_checked_artifacts_when_requested
```

then rebuild so the `include_str!` byte-lock tests pick up the new bytes.
