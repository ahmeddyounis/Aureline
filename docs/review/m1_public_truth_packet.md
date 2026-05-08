# Public-truth review packet (M1 seed)

This document is the lightweight review workflow for hardening any
public-truth-facing change (docs, Help/About, service-health, command help, and
support/provenance summaries).

Canonical checkpoint source: `artifacts/milestones/m1/late_freeze_pack.yaml`.

## When to use this packet

Use this workflow when a change:

- widens a user-facing claim (or removes a caveat);
- changes provenance/source/version/freshness badges or copy;
- changes Help/About/service-health routing or embedded-origin chrome; or
- changes command naming, disabled-reason explanations, or diagnostics copy.

## Review steps (fail closed)

1. **Locate the governing checkpoint row.** Find the matching `output_id` in
   `artifacts/milestones/m1/late_freeze_pack.yaml`. If no row exists, add one
   before proceeding.
2. **Verify canonical refs.** Follow the row’s `proof_artifact_refs` and confirm
   the canonical sources (contracts/schemas/seeds) exist and are current.
3. **Verify parity.** Ensure Help/About and docs surfaces cite the same source
   and version/freshness vocabulary (no surface-local synonyms).
4. **Verify exact-build identity linkage.** Any provenance-bearing surface must
   join back to `artifacts/build/build_identity.json` (or explicitly deny and
   route to a repair hook).
5. **Run the pack validator.** `python3 ci/check_m1_late_freeze_pack.py --repo-root .`

## Narrowing rule

If proof is missing, narrow or deny the claim rather than widening with
untracked prose. Record the missing artifact or mismatch in the checkpoint pack
so later work converges on one source of truth.

