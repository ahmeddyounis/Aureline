# Repository-acquisition and bootstrap truth conformance evidence

This artifact is the release-consumable conformance evidence for the M3
repository-acquisition and post-open bootstrap beta lane. Every claimed
beta acquisition row reads exactly one `RepositoryAcquisitionBetaProjection`
assembled from a `SourceLocatorRecord`, a `CheckoutPlanRecord`, and a typed
bootstrap queue. Every projection is exercised by at least one drill in
[`fixtures/workspace/m3/bootstrap_truth_corpus/`](../../../fixtures/workspace/m3/bootstrap_truth_corpus/);
the drills are executed by
[`crates/aureline-qe/src/bootstrap_truth/`](../../../crates/aureline-qe/src/bootstrap_truth/)
and replayed by
`cargo test -p aureline-qe --test bootstrap_truth_conformance`.

The corpus is owned by the QE crate so the same fixture matrix can gate
Start Center / command-palette / deep-link projections, CLI / headless
mirrors, support-export parity reviews, and release evidence reviews from
one shared truth.

The exit-gate condition the corpus guards is the M03-252 anchor:

> Aureline can explain what source is being acquired, what checkout shape
> and cost band will be used, which follow-up bootstrap actions remain
> manual, and how to recover from interrupted acquisition without silently
> drifting into hidden setup or hidden trust elevation.

## Coverage matrix

| Axis | Drill ids | Outcome anchored |
| --- | --- | --- |
| Local open — no fetch, no setup | `local.open_folder` | `open_local`, `local_no_fetch`, no queue, `surface_must_disclose = false`, guardrails hold. |
| Clone remote — submodules + LFS | `clone.remote_submodule_lfs` | `clone`, `submodule_policy = init_pending`, `lfs_policy = pointer_only`, `large_fetch_or_hydrate`, 3 manual followups. |
| Mirror parity — lagged within skew | `clone.mirror_lagged_within_skew` | Honesty `mirror_lagged`; mirror-freshness evidence present; `mirror_not_masquerading_as_live` holds. |
| Mirror / air-gap parity — stale, upstream unreachable | `mirror.proxy_stale_offline` | Honesty `mirror_stale` + `signer_first_seen`; provenance / freshness / signer stay explicit with no live upstream. |
| Air-gapped import — signed offline bundle | `import.airgap_signed_bundle` | `import`, `archive_extract`, `signed_offline_bundle`, `local_no_fetch`; never masquerades as live fetch (headless surface). |
| Import handoff — first-seen signer | `import.handoff_first_signer` | `import`, `offline_snapshot` + `signer_first_seen` + `read_only_partial`; deferred docs import stays manual. |
| Shallow checkout | `checkout.shallow_history` | `shallow_history` mode, `partial_or_sparse = true`, honesty `shallow_history`, deepen deferred. |
| Partial clone / promisor | `checkout.partial_clone_promisor` | `partial_clone` mode, `large_fetch_or_hydrate`, honesty `partial_clone`. |
| Sparse workset | `checkout.sparse_workset` | `sparse_checkout` mode, honesty `sparse_workset`, index warm-up deferred. |
| Nested submodule tree — init pending | `submodule.nested_tree_init_pending` | `submodule_policy = init_pending`, two manual init items, recursive init blocked. |
| LFS pointer-only — read-only browse | `lfs.pointer_only_read_only` | Honesty `read_only_partial` + `shallow_history` + `lfs_pointer_only`; missing typed as not-yet-hydrated. |
| Interrupted — resumable (all branches) | `interrupted.mirror_clone_resume` | `interrupted = true`; branches resume / open-read-only / discard / switch-to-live / refresh-mirror; export-safe. |
| Interrupted — discard required | `interrupted.fetch_discard_required` | `discard_with_compensation`, `open_read_only_available = false`, sole branch `discard_and_restart`. |
| Interrupted — open read-only partial | `interrupted.open_read_only_partial` | `open_read_only_available = true`; read-only-partial recovery distinguishable and export-safe. |
| Resume — live attach, reauth required | `resume.managed_workspace_reauth` | `resume`, `live_attach`, `credential_posture = reauth_required`; authority never silently widened (headless surface). |
| Policy-narrowed clone | `policy.generators_blocked` | Honesty `policy_narrowed`; generator install skipped (`policy_excludes`); package restore stays pending. |
| Silent background setup — caught | `silent_setup.caught_as_failure` | `guardrails_all_hold = false`, `no_implicit_repo_code_execution = false`. Silent setup is a failing condition. |
| Support / export audit — signer review | `support.signer_review_export` | Support surface; honesty `mirror_stale` + `upstream_delta_outside_skew` + `signer_changed_review_required`; export-safe evidence packet. |
| Deep-link open | `deeplink.review_open` | `open_deep_link`, `not_applicable` mode, deep-link side effect blocked. |
| Open archive — read-only snapshot | `open_archive.snapshot_read_only` | `open_archive`, `offline_snapshot` + `read_only_partial`, `local_no_fetch`. |
| Negative — plan binds a sibling locator | `negative.plan_binds_sibling_locator` | Projection rejects with `checkout plan references locator …`. |
| Negative — item binds a sibling plan | `negative.item_binds_sibling_plan` | Projection rejects with `… references plan …`. |
| Negative — item binds a sibling locator | `negative.item_binds_sibling_locator` | Projection rejects with `bootstrap item … references locator …`. |
| Negative — item carries no attributable evidence | `negative.item_missing_evidence` | Projection rejects with `… carries no attributable evidence`. |

## Transverse invariants pinned by the harness

- `corpus_covers_every_claimed_beta_verb` — `open_local`, `clone`,
  `import`, `open_archive`, and `resume` each keep a drill, so the five
  distinct beta verbs stay distinct.
- `corpus_covers_every_interrupted_recovery_branch` — every recovery
  branch (`resume_acquisition`, `discard_and_restart`,
  `open_read_only_partial`, `refresh_mirror`, `switch_to_live_origin`) is
  exercised.
- `corpus_distinguishes_resume_discard_and_read_only_recovery` — a
  staging-only resumable row, a discard-with-compensation row, and a
  read-only-partial row are all present, keeping the three recovery paths
  distinguishable and export-safe.
- `corpus_proves_silent_background_setup_is_caught` — at least one drill
  pins `no_implicit_repo_code_execution = false` and
  `guardrails_all_hold = false`.
- `corpus_covers_mirror_airgap_and_offline_rows` — `mirror_lagged`,
  `mirror_stale`, `signed_offline_bundle`, and `offline_snapshot` honesty
  labels are all present even when public endpoints are absent.
- `corpus_covers_desktop_and_headless_surfaces` — `start_center`,
  `command_palette`, `cli_headless`, and `support` surfaces are all
  represented, proving acquisition semantics across desktop and headless.
- `corpus_covers_checkout_shape_and_topology_axes` — full / partial /
  sparse / shallow / archive / live checkout modes plus the
  submodule / LFS / read-only topology honesty labels are covered.
- `negative_drills_protect_source_plan_and_queue_lineage` — the four
  lineage negatives keep source-locator, checkout-plan, and
  bootstrap-queue binding and attribution enforced.

## Cross-surface evidence-packet check

For every positive drill the harness asserts the projection's
`BootstrapEvidencePacket`:

- reconstructs the source-locator ref and checkout-plan ref from the
  fixture records,
- joins one ref per bootstrap-queue item,
- reports `every_item_attributed` as expected, and
- stays `export_safe`.

Interrupted drills additionally assert the recovery card is `export_safe`,
so Resume / Discard / Open-read-only state never leaves ambiguous
half-authoritative workspace state and never leaks raw paths, credentials,
or bytes into a support packet.

## Replay

```
cargo test -p aureline-qe --test bootstrap_truth_conformance
```

The corpus manifest at
`fixtures/workspace/m3/bootstrap_truth_corpus/manifest.json` is the
canonical pass / fail input; CI consumers SHOULD treat any `failures()`
returned by `run_corpus_from_repo_root` as a beta release blocker for the
workspace acquisition / bootstrap lane.

## Redaction guarantees

Every fixture is metadata-safe: only opaque refs and typed labels cross the
boundary. The runner scans each positive fixture for forbidden raw-export
tokens before projection, so the redaction contract lives on the corpus
itself, not only on individual surface read paths.
