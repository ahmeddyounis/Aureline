# Repository-acquisition and bootstrap truth beta audit

Reviewer guidance for the M3 repository-acquisition and post-open
bootstrap conformance lane. This document is the human-readable companion
to the regression-gated corpus that proves Aureline's acquisition promise
across every claimed beta acquisition row.

The corpus sits on top of the beta acquisition projection introduced with
the source-locator / checkout-plan / bootstrap-queue records
([`repository_acquisition_beta.md`](repository_acquisition_beta.md) and the
frozen seed vocabulary in
[`source_acquisition_and_bootstrap_seed.md`](../source_acquisition_and_bootstrap_seed.md)).
Where that layer defines *what* the projection means, this lane proves the
projection keeps meaning it on every row — turning the acquisition UX
promise into a proof system that catches silent hydrate / init / fetch,
wrong-target clone state, mirror freshness drift, and ambiguous
interrupted-bootstrap recovery before beta claims harden.

The exit-gate condition the corpus guards is the M03-252 anchor:

> Aureline can explain what source is being acquired, what checkout shape
> and cost band will be used, which follow-up bootstrap actions remain
> manual, and how to recover from interrupted acquisition without silently
> drifting into hidden setup or hidden trust elevation.

## Where the corpus lives

- Fixtures + manifest:
  [`/fixtures/workspace/m3/bootstrap_truth_corpus/`](../../../fixtures/workspace/m3/bootstrap_truth_corpus/)
  (`manifest.json` is the single source of truth).
- Harness:
  [`/crates/aureline-qe/src/bootstrap_truth/`](../../../crates/aureline-qe/src/bootstrap_truth/).
- Replay: `cargo test -p aureline-qe --test bootstrap_truth_conformance`.
- Release evidence:
  [`/artifacts/workspace/m3/bootstrap_truth_report.md`](../../../artifacts/workspace/m3/bootstrap_truth_report.md).
- Mirror / air-gap evidence:
  [`/artifacts/workspace/m3/mirror_airgap_acquisition_audit.md`](../../../artifacts/workspace/m3/mirror_airgap_acquisition_audit.md).

## What a reviewer checks

Each positive drill binds one `SourceLocatorRecord`, one
`CheckoutPlanRecord`, and zero or more `BootstrapQueueItemRecord`s, then the
harness projects them and compares against the manifest's `expected_*`
fields. A reviewer auditing a claimed beta acquisition row confirms:

1. **Source identity is reconstructable.** `expected_acquisition_verb`,
   `expected_locator_class`, and `expected_transport_class` reproduce the
   source the row claims. Open folder, Clone, Import, Open archive, and
   Resume stay distinct verbs.
2. **Checkout shape and cost are disclosed before any fetch.**
   `expected_checkout_mode`, `expected_partial_or_sparse`,
   `expected_submodule_policy`, `expected_lfs_policy`, and
   `expected_cost_band` are pinned, so partial / sparse / shallow / archive
   / live states never read as full local truth.
3. **Credential posture is honest.** `expected_credential_posture` and
   `expected_credential_reauth_required` name the credential class without
   ever carrying a secret; reauth / reconnect surface their typed posture.
4. **Follow-up work stays typed and attributable.**
   `expected_manual_followup_count` is the count of remaining queue items;
   `expected_every_item_attributed` proves every enqueued item carries
   typed evidence. The four lineage negatives prove the projection refuses
   to assemble when a plan or item binds the wrong source / plan, or when
   an item carries no evidence.
5. **Interrupted recovery is distinguishable and export-safe.**
   `expected_interrupted`, `expected_interrupted_branches`,
   `expected_discard_posture`, and `expected_open_read_only_available` keep
   Resume, Discard, and Open-read-only distinct; the harness asserts the
   recovery card is `export_safe`.
6. **Honesty labels render verbatim.** `expected_honesty_labels` (mirror
   lag / stale, upstream delta, offline / signed-offline, signer first-seen
   / changed, read-only partial, shallow, partial-clone, sparse, submodule
   init pending, LFS pointer-only, reauth / reconnect, policy-narrowed)
   match the projection in order.
7. **Guardrails hold — and silent setup is caught.**
   `expected_guardrails_all_hold` is pinned. The
   `silent_setup.caught_as_failure` drill deliberately runs a side-effecting
   item before trust admission and pins
   `no_implicit_repo_code_execution = false` with
   `guardrails_all_hold = false`, so silent background setup is a **failing**
   condition, not a tolerated implementation detail.

## Acceptance criteria mapping

| Acceptance criterion | Where proven |
| --- | --- |
| The corpus can reconstruct and compare source locator, checkout plan, interruption state, and follow-up queue items for every claimed beta row. | Per-drill `expected_*` fields + the evidence-packet reconstruction check in the runner. |
| Silent background setup is caught as a failing condition. | `silent_setup.caught_as_failure` + `corpus_proves_silent_background_setup_is_caught`. |
| Mirror / offline drills prove provenance / freshness / signer continuity stay explicit with no public endpoints. | `clone.mirror_lagged_within_skew`, `mirror.proxy_stale_offline`, `import.airgap_signed_bundle`, `support.signer_review_export` + `corpus_covers_mirror_airgap_and_offline_rows`; see the mirror / air-gap audit. |
| Read-only partial-root recovery, Resume, and Discard stay distinguishable and export-safe. | `interrupted.mirror_clone_resume`, `interrupted.fetch_discard_required`, `interrupted.open_read_only_partial` + `corpus_distinguishes_resume_discard_and_read_only_recovery`. |
| Claimed acquisition semantics across desktop and headless paths. | Surfaces `start_center`, `command_palette`, `cli_headless`, `support` + `corpus_covers_desktop_and_headless_surfaces`. |

## Guardrails on the corpus itself

The corpus is tied to the canonical acquisition projection and evidence
packet, not to log-scraping or ad hoc screenshots. Fixtures carry only
opaque refs and typed labels; the runner scans every positive fixture for
forbidden raw-export tokens before projection, so the redaction contract is
enforced on the corpus, not only on individual surface read paths.

## Out of scope

This lane does not expand into broader CI / provider acquisition flows or
post-M3 cloud-workspace creation lanes, and it does not introduce a signed
template registry or full managed-workspace orchestration. It proves the
bounded M3 acquisition / bootstrap rows and nothing wider.
