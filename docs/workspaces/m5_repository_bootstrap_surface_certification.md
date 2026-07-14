# M5 repository-bootstrap surface certification (M05-1195)

This contract is the **closing B142 surface-certification capstone** over the frozen M5 repository-bootstrap
matrix (`m5_repository_bootstrap_matrix`). Where the freeze matrix defines the five governed project-entry
acquisition families — **open-local, clone-remote, open-archive, import-bundle, and resume-snapshot** — the
1189–1192 implementation lanes resolve their per-surface source-locator, checkout-plan, credential-posture,
evidence-packet, staged-trust, resumable-acquisition, and post-open-queue truth, the 1193 shared-consumer lane
aligns their grammar across surfaces, and the 1194 accessibility lane proves keyboard / screen-reader /
high-zoom / high-contrast / localization / CLI-export parity, this capstone **certifies** that the shared
repository-bootstrap truth holds on every claimed M5 **project-entry profile** and auto-narrows any profile that
cannot sustain it.

- **Module:** `crates/aureline-ui/src/m5_repository_bootstrap_surface_certification/`
- **Schema:** `schemas/workspaces/m5-repository-bootstrap-surface-certification.schema.json`
- **Release proof:** `artifacts/release/m5-repository-bootstrap-surface-certification/`
  (`support_export.json`, `matrix.csv`) and `…-surface-certification.md`
- **Fixtures:** `fixtures/workspaces/m5-repository-bootstrap-surface-certification/`

## What the packet certifies

The packet is keyed on the claimed **profile** a user, reviewer, admin, or support engineer reads a
source-locator, checkout-plan, credential-posture, staged-trust, bootstrap-evidence, or post-open-queue surface
through, not on the reusable acquisition family it renders:

1. **Live trusted acquisition surface** — a live, first-party, fully-current repository acquisition. The **only**
   profile that may certify a `trusted_acquisition_surface` claim.
2. **Reviewable acquisition structure** — a self-sufficient, inspectable source-locator / checkout-plan /
   registry reference; certifies at most `reviewable_acquisition_surface`.
3. **Disclosed checkout-plan profile** — an open-archive surface whose checkout-plan proof can only be partially
   disclosed; auto-narrows to `checkout_plan_disclosed_projection`.
4. **Unverified trust-stage profile** — an import-bundle surface whose staged-trust fence cannot be confirmed;
   auto-narrows to `trust_stage_unverified_projection`.
5. **Unverified bootstrap-evidence profile** — a resume-snapshot surface whose signer / mirror provenance or
   bootstrap-evidence has aged out or is policy-blocked; auto-narrows to
   `bootstrap_evidence_unverified_projection`.

Each row certifies its profile across **nine truth axes** — visual, keyboard, screen-reader, high-zoom-reflow,
high-contrast, localization, CLI/export, degraded-state, and repository-bootstrap-component-truth behavior — and
resolves to a derived verdict:

- **green** — every axis certified, every invariant held, the claimed acquisition tier delivered;
- **yellow** — a truth axis is not current, so the acquisition claim auto-narrows to the weakest supported
  ceiling with a bound reason and a frozen downgrade trigger;
- **red** (blocked) — a degraded axis is hidden behind a fresh trusted claim, a hard invariant breaks, CLI/export
  parity drops, a non-live profile claims a trusted acquisition surface, or the narrowing is inconsistent.

## Invariants

1. **A degraded axis must produce a visible claim narrowing.** A profile that keeps a
   `trusted_acquisition_surface` / `reviewable_acquisition_surface` claim while one of its truth axes is not
   current over-claims and blocks.
2. **Only a live first-party profile may certify a trusted acquisition surface.** Every other profile is at most
   a reviewable acquisition structure or a narrowed projection.
3. **CLI/export parity is always-on.** Support and automation must always be able to reconstruct the canonical
   source locator, checkout plan, credential posture, evidence packet, staged-trust rule, post-open queue, and
   registry reference as text / JSON / Markdown.
4. **Every B142 hard invariant holds per row.** No profile may rewrite clone into open when a local checkout
   already exists, run repo-owned actions (hooks, tasks, extensions, package restores, submodule or LFS
   hydration, generator installs) implicitly during acquisition, lose signer or mirror provenance across an
   offline or mirrored fetch, strand partial acquisition without Resume / Discard / read-only choices, or hide
   the bootstrap credential posture behind generic connected-state copy.
5. **One canonical proof bundle.** Every row cites exactly one canonical repository-bootstrap proof bundle
   (`artifacts/release/m5-repository-bootstrap-proof/support_export.json`) — the frozen repository-bootstrap
   matrix proof — so release, docs, and support consume a single repository-bootstrap certification source rather
   than hand-authored prose.

## Boundary

The packet is metadata-only. Raw credentials, plaintext secrets, bearer tokens, endpoint URLs, and private-key
material never cross this boundary; the export carries typed tokens, opaque evidence refs, and repo-relative
paths only.

## Regenerating the artifacts

The checked-in release artifacts and fixtures are byte-locked to the seed builder. To regenerate them after an
intentional change, run:

```
GEN_REPOSITORY_BOOTSTRAP_CERT_ARTIFACTS=1 cargo test -p aureline-ui \
  m5_repository_bootstrap_surface_certification::tests::regenerate_checked_artifacts_when_requested
```

then rebuild so the `include_str!` byte-lock tests pick up the new bytes.
