# M5 evidence pointer — sideload review sheets

Reviewer contract for the canonical M5 sideload review sheets that give an author or
operator the reviewed-install surface for an unpacked or archive-backed side-load of
each marketed M5 ecosystem artifact family: source identity, signing state, requested
permissions, disclosed external executables, registry-binding decision, runtime class,
host/ABI, rendered trust tier, and review disposition. This row is a depth-lane proof
governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).

## Canonical artifacts

- Truth packet: `artifacts/ecosystem/m5/m5-sideload-review.json`
- Boundary schema: `schemas/ecosystem/m5-sideload-review.schema.json`
- Reviewer contract: `docs/m5/ship-sideload-review-sheets-with-source-identity-signing-state-permissions-executables-update-binding-and-limited-trust-continuity.md`
- Human-readable rendering: `artifacts/m5/ship-sideload-review-sheets-with-source-identity-signing-state-permissions-executables-update-binding-and-limited-trust-continuity.md`
- Overview companion: `docs/ecosystem/m5/m5-sideload-review.md`
- Fixture corpus: `fixtures/ecosystem/m5/m5-sideload-review/`
- Owning crate module: `crates/aureline-ecosystem/src/m5_sideload_review/`

## Reuses the shared M5 vocabulary

The sideload review sheet is the reviewed-install counterpart for a package that lives
on local disk rather than in the registry. It reuses the closed artifact-family,
source-class, runtime-class, host/ABI, signing-state, trust-posture, and anti-abuse
vocabulary already frozen by the install-governance matrix and the publish-preview gate
(`artifacts/ecosystem/m5/m5-author-and-publish-preview.json`) rather than minting a
parallel set, so a side-load, a registry install, and a publish preview describe the
same artifact with the same words.

## What the sheet proves

- **Local builds never inherit a trusted badge.** The rendered trust tier is capped by
  *both* the signing state and the registry-binding decision. A `signed_verified`
  framework pack or recipe pack built locally renders `unsigned_local_only` because it
  stays local; only a `bound_to_registry_identity` binding lifts the cap, and only as
  far as `registry_bound` — never `verified_publisher` or `enterprise_approved`.
- **Stay-local versus bind-to-registry-later is a first-class decision.** Every sheet
  records its `update_binding` and offers the matching action, and the binding ceiling
  keeps a still-local or bind-later package at `unsigned_local_only`.
- **Widening cannot apply through a silent hot reload.** A permission widening,
  runtime-class change, host/ABI rebind, newly introduced external executable, changed
  binding, or changed release channel on an installed side-load recomputes to
  `fresh_review_required` and disables the accept action until a fresh review clears it.
- **Revoked signatures and anti-abuse quarantines block the install.** A revoked
  signature or an anti-abuse quarantine forces `blocked` regardless of source.
- **Limited-trust continuity is preserved on installed rows.** A reload that does not
  rebind to the registry never raises the installed row's rendered badge.
- **Install-style review is never bypassed.** The review sheet is the install-style
  review; a side-load on local disk is reviewed, not waved through.
- **Records are export-safe.** Every field is a typed state, a redacted display hint,
  or an opaque ref — no absolute paths, archive bytes, signing secrets, or executable
  payloads.

## Executable proof

`crates/aureline-ecosystem/src/m5_sideload_review/tests.rs` loads the embedded packet,
asserts it validates with zero violations, proves every closed vocabulary, disposition,
and review trigger is exercised, asserts the non-inheritance, fresh-review-on-widening,
revoked/quarantine-block, and limited-trust-continuity guardrails, and checks the
export projection. `M5SideloadReview::validate()` is the CI-facing gate that flags any
overstated rendered badge, inherited trust, hidden widening trigger, inconsistent
source identity or signature, silently elevated continuity, or summary drift.
