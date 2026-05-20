# Repository-acquisition and bootstrap truth conformance corpus

This corpus is the failure / recovery drill harness for the M3
repository-acquisition and post-open bootstrap beta projection
(`RepositoryAcquisitionBetaProjection` over the `SourceLocatorRecord`,
`CheckoutPlanRecord`, and `BootstrapQueueItemRecord` boundary records owned
by `aureline-workspace::acquisition`).

It converts the acquisition/bootstrap UX promise into a regression-gated
proof system: each drill binds one source locator, one checkout plan, and
zero or more bootstrap-queue items, then pins the projected acquisition
truth a claimed beta row must reproduce — verb, source/transport identity,
checkout shape and cost band, credential posture, interrupted-recovery
branches, the manual follow-up queue, honesty labels, guardrails, and the
export-safe evidence packet.

Every drill is loaded by the conformance harness at
[`crates/aureline-qe/src/bootstrap_truth/`](../../../../crates/aureline-qe/src/bootstrap_truth/)
and replayed by
`cargo test -p aureline-qe --test bootstrap_truth_conformance`.

## Single source of truth

`manifest.json` is authoritative. Positive drills MUST parse, project, and
match **every** `expected_*` field in the manifest. Negative drills MUST
FAIL projection with an error whose message contains
`expected_failure_substring`. The fixtures carry only the scenario records
and a `__fixture__` prelude — they do **not** restate the expectations, so
there is exactly one place to read and audit the pinned truth.

Boundary schemas:

- [`/schemas/workspace/source_locator.schema.json`](../../../../schemas/workspace/source_locator.schema.json)
- [`/schemas/workspace/checkout_plan.schema.json`](../../../../schemas/workspace/checkout_plan.schema.json)
- [`/schemas/workspace/bootstrap_queue_item.schema.json`](../../../../schemas/workspace/bootstrap_queue_item.schema.json)
- [`/schemas/workspace/repository_acquisition.schema.json`](../../../../schemas/workspace/repository_acquisition.schema.json)

Reviewer guidance: [`docs/workspace/m3/bootstrap_truth_beta_audit.md`](../../../../docs/workspace/m3/bootstrap_truth_beta_audit.md).
Conformance artifact: [`artifacts/workspace/m3/bootstrap_truth_report.md`](../../../../artifacts/workspace/m3/bootstrap_truth_report.md).
Mirror / air-gap audit: [`artifacts/workspace/m3/mirror_airgap_acquisition_audit.md`](../../../../artifacts/workspace/m3/mirror_airgap_acquisition_audit.md).

## Coverage axes

| Axis | Drill ids |
| --- | --- |
| Local open — no fetch, no setup | `local.open_folder` |
| Clone remote — submodules + LFS | `clone.remote_submodule_lfs` |
| Mirror parity — lagged within skew | `clone.mirror_lagged_within_skew` |
| Mirror / air-gap parity — stale, upstream unreachable | `mirror.proxy_stale_offline` |
| Air-gapped import — signed offline bundle | `import.airgap_signed_bundle` |
| Import handoff — first-seen signer | `import.handoff_first_signer` |
| Shallow checkout | `checkout.shallow_history` |
| Partial clone / promisor | `checkout.partial_clone_promisor` |
| Sparse workset | `checkout.sparse_workset` |
| Nested submodule tree — init pending | `submodule.nested_tree_init_pending` |
| LFS pointer-only — read-only browse | `lfs.pointer_only_read_only` |
| Interrupted — resumable (all branches) | `interrupted.mirror_clone_resume` |
| Interrupted — discard required | `interrupted.fetch_discard_required` |
| Interrupted — open read-only partial | `interrupted.open_read_only_partial` |
| Resume — live attach, reauth required | `resume.managed_workspace_reauth` |
| Policy-narrowed clone | `policy.generators_blocked` |
| Silent background setup — caught as failure | `silent_setup.caught_as_failure` |
| Support / export audit — signer review lineage | `support.signer_review_export` |
| Deep-link open | `deeplink.review_open` |
| Open archive — read-only snapshot | `open_archive.snapshot_read_only` |
| Negative — plan binds a sibling locator | `negative.plan_binds_sibling_locator` |
| Negative — item binds a sibling plan | `negative.item_binds_sibling_plan` |
| Negative — item binds a sibling locator | `negative.item_binds_sibling_locator` |
| Negative — item carries no attributable evidence | `negative.item_missing_evidence` |

## Transverse invariants

The conformance suite also pins, across the whole positive set:

- every claimed beta verb (`open_local`, `clone`, `import`,
  `open_archive`, `resume`) keeps a drill;
- every interrupted-recovery branch (`resume_acquisition`,
  `discard_and_restart`, `open_read_only_partial`, `refresh_mirror`,
  `switch_to_live_origin`) is exercised, and Resume / Discard /
  Open-read-only stay distinguishable by discard posture and
  read-only availability;
- silent background setup is caught (`no_implicit_repo_code_execution =
  false`) as a failing guardrail, not a tolerated detail;
- mirror / air-gap / offline rows keep their freshness and signer honesty
  labels even when public upstream endpoints are absent;
- desktop (`start_center`, `command_palette`) and headless / support
  (`cli_headless`, `support`) surfaces are all represented;
- checkout-mode and topology axes (full / partial / sparse / shallow /
  archive / live, plus submodule / LFS / read-only honesty) are covered;
- the four lineage negatives keep source-locator, checkout-plan, and
  bootstrap-queue binding and attribution enforced.

## Running the corpus

```
cargo test -p aureline-qe --test bootstrap_truth_conformance
```

The crate also exposes the corpus loader + projection assertions as a
library (`aureline_qe::bootstrap_truth::{load_corpus, run_corpus,
run_corpus_from_repo_root}`), so other harnesses (Start Center / palette UI
checks, CLI/headless mirrors, support-export parity reviews, release
evidence reviews) can quote the same drill matrix without re-parsing the
fixtures.

## Redaction guarantees

Every fixture is metadata-safe: only opaque refs and typed labels cross the
boundary. Raw absolute paths, raw credentials, raw remote URLs with
embedded credentials, raw archive bytes, and raw policy-bundle bytes never
appear. The runner additionally scans each positive fixture for forbidden
raw-export tokens (`raw_path_export_allowed`,
`raw_remote_url_export_allowed`, `raw_credential_export_allowed`,
`raw_secret_export_allowed`, `raw_token_export_allowed`,
`raw_archive_bytes_export_allowed`, `raw_policy_bundle_bytes_export_allowed`);
any occurrence fails the drill before projection. Removing any positive or
negative drill without a replacement is a breaking contract change for the
`workspace.bootstrap_truth_corpus.beta` corpus.
