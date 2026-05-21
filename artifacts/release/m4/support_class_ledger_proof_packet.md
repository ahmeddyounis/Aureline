# v1.0 support-class ledger — proof packet

Reviewer-facing proof packet for the gated v1.0 support-class ledger,
certified-archetype manifest, and downgrade automation.

Canonical machine source (do not clone status text from this packet — ingest the
JSON):

- Ledger: [`/artifacts/release/support_class_ledger.json`](../support_class_ledger.json)
- Schema: [`/schemas/release/support_class_ledger.schema.json`](../../../schemas/release/support_class_ledger.schema.json)
- Companion doc: [`/docs/release/support_class_ledger.md`](../../../docs/release/support_class_ledger.md)
- Validator: `ci/check_support_class_ledger.py`
- Validation capture:
  [`/artifacts/release/captures/support_class_ledger_validation_capture.json`](../captures/support_class_ledger_validation_capture.json)
- Typed consumer: `aureline_release::support_class_ledger`

## What this packet proves

1. **The v1.0 support-class assignments are canonical, not prose.** Every
   subject the v1.0 release publishes a support class for has exactly one ledger
   entry binding it to the class it is put forward as (`claimed_class`), the
   class it effectively publishes after narrowing (`effective_class`), its
   backing stable claim, its required evidence path, and its owner sign-off.

2. **Certified is gated on the certified-archetype manifest.** A subject may
   publish as `certified` only when it references a manifest entry that is
   certified, fresh against the ledger `as_of` date, and owner-signed. The
   manifest is the single source for which archetype scope envelopes (client
   class, OS family, deployment mode, local-vs-remote mode) are certified.

3. **Downgrade automation narrows unqualified claims before publication.** The
   ledger ingests the stable claim matrix
   ([`/artifacts/release/stable_claim_matrix.json`](../stable_claim_matrix.json)):
   when a subject's backing stable claim is narrowed below the stable cutline,
   its support class narrows automatically and carries the
   `backing_stable_claim_narrowed` reason. Stale evidence, expired waivers,
   decertified archetypes, and missing owner sign-off narrow the published class
   the same way. Every downgrade reason is watched by a downgrade rule, and the
   firing rules drive the publication `proceed`/`hold` verdict.

## Proof-index registration

Each ledger entry registers under one row of the stable proof index
([`/artifacts/milestones/m3/public_proof_index.md`](../../milestones/m3/public_proof_index.md))
via its `evidence.proof_index_ref`, so this lane's proof is anchored to the
public-proof artifact index rather than to ad hoc notes. As the v1.0 proof index
is assembled it ingests these ledger entries by `entry_id`.

| Ledger entry | Claimed | Effective | Proof-index row |
|---|---|---|---|
| `provider_aware_language_intelligence` | certified | certified | `m3_public_proof:launch_wedge` |
| `repair_and_rollback_safety` | supported | supported | `m3_public_proof:exact_build_identity` |
| `export_and_offboarding_support` | certified | not_certified_in_this_mode | `m3_public_proof:boundary_truth` |
| `localization_readiness` | certified | experimental | `m3_public_proof:docs_freshness` |
| `regulated_environment_assurance` | supported | not_supported | `m3_public_proof:version_skew_truth` |

## Current posture

At this revision three subjects put forward as Certified or Supported are
narrowed below their claimed class — for a decertified archetype, stale support
evidence, an expired waiver, and a narrowed backing stable claim. Their reasons
fire four blocking downgrade rules, so the v1.0 support publication **holds**.
That is the honest posture: the repository is pre-implementation and most
subjects have not yet earned a published support class.

## How to refresh

1. Land qualification evidence, refreshed certified-archetype reports, and
   waivers first; point each entry's `evidence_refs`, `proof_index_ref`, and
   `certified_archetype_ref` at the canonical packets.
2. Set each entry's `ledger_state`, `active_downgrade_reasons`, and
   `effective_class` to the honest posture. An entry whose backing stable claim
   is narrowed, whose evidence is stale, or whose archetype is decertified
   narrows below its claimed class.
3. Recompute the `publication` block and `summary`, then run
   `python3 ci/check_support_class_ledger.py --repo-root .` and commit the
   regenerated capture in the same change set.
4. If delivery proves a narrower support claim than planned, narrow the claim and
   update the ledger — do not paper over the gap with prose.
