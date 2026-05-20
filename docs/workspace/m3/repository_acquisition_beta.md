# Repository-acquisition beta truth

Beta layer on top of the alpha source-locator, checkout-plan, and
bootstrap-queue vocabulary frozen in
[`source_acquisition_and_bootstrap_seed.md`](../source_acquisition_and_bootstrap_seed.md).
The seed froze the three boundary records (what source is being
acquired, what the checkout plan may do before trust admission, and
which typed bootstrap work the plan enqueues). The beta layer binds
those three records into one cross-surface projection so Start Center,
the command palette, deep-link resolvers, the CLI / headless
acquisition path, the policy-guided deployment lane, and support
exporters always agree, **before any hydrate / init / fetch path
runs**, on:

- **what source is being acquired** — the distinct acquisition verb
  (Open folder, Clone repository, Import bundle, Open archive, Resume
  snapshot) plus the locator class, transport, declared freshness, and
  signer-continuity posture quoted from the locator;
- **what checkout shape and cost band will be used** — full / partial /
  sparse / shallow / archive / live mode, submodule and LFS policy, the
  read-only partial roots already made safe to inspect, and a coarse
  expected cost band;
- **what credential posture applies** — the typed credential class
  derived from the locator auth mode and the plan trust stage, carrying
  only handle refs, never a raw secret;
- **which follow-up bootstrap actions remain manual** — the typed
  follow-up list derived from the queue items that have not yet
  succeeded, so setup never collapses into one opaque "setting up
  workspace" spinner; and
- **how to recover from interrupted acquisition** — the explicit
  resume / discard / open-read-only branches derived from the plan's
  resumable state, kept export-safe for support and enterprise rollout.

The exit-gate condition the surfaces guard together is the acquisition
anchor: **Aureline can explain what source is being acquired, what
checkout shape and cost band will be used, which follow-up bootstrap
actions remain manual, and how to recover from interrupted acquisition
without silently drifting into hidden setup or hidden trust
elevation.**

The machine-readable boundaries are:

- [`/schemas/workspace/source_locator.schema.json`](../../../schemas/workspace/source_locator.schema.json)
- [`/schemas/workspace/checkout_plan.schema.json`](../../../schemas/workspace/checkout_plan.schema.json)
- [`/schemas/workspace/bootstrap_queue_item.schema.json`](../../../schemas/workspace/bootstrap_queue_item.schema.json)

The projection record itself is:

- [`/schemas/workspace/repository_acquisition.schema.json`](../../../schemas/workspace/repository_acquisition.schema.json)

The worked fixtures live under:

- [`/fixtures/workspace/m3/repository_acquisition_and_bootstrap/`](../../../fixtures/workspace/m3/repository_acquisition_and_bootstrap/)

The Rust types are exported from `aureline_workspace::acquisition`. The
integration test
[`crates/aureline-workspace/tests/repository_acquisition_beta.rs`](../../../crates/aureline-workspace/tests/repository_acquisition_beta.rs)
replays every scenario fixture, proves the closed acceptance states,
and round-trips the frozen `bootstrap_cases` fixtures through the same
descriptors. If the seed vocabulary and this beta layer disagree, the
seed wins and the projection updates together.

## 1 Beta truth contract

Every acquisition surface reads exactly one
`RepositoryAcquisitionBetaProjection`. The projection is derived from
one `SourceLocatorRecord`, one `CheckoutPlanRecord`, and zero or more
`BootstrapQueueItemRecord`s. The projection refuses to assemble when the
plan does not reference the supplied locator, when a bootstrap item does
not reference the supplied plan and locator, or when a bootstrap item
carries no attributable evidence. This guarantees that downstream
surfaces never read a plan or item that bound to a sibling or stale
source.

The projection emits one closed answer per surface for:

1. `acquisition_verb` — `open_local`, `clone`, `import`, `open_archive`,
   `open_template_or_prebuild`, `resume`, or `open_deep_link`, resolved
   from the locator class. Open folder, Clone repository, Import bundle,
   Open archive, and Resume snapshot stay distinct verbs; the projection
   never collapses one into another.
2. `checkout_shape` — the `mode` (`full_checkout`, `partial_clone`,
   `sparse_checkout`, `shallow_history`, `archive_extract`,
   `live_attach`, `not_applicable`), the `partial_or_sparse` predicate,
   the `submodule_policy`, the `lfs_policy`, and the read-only partial
   roots the plan already made safe to inspect.
3. `expected_cost_band` — `local_no_fetch`, `metered_fetch`,
   `large_fetch_or_hydrate`, `live_attach`, or `unknown`, derived
   deterministically from transport, posture, topology, and the
   bootstrap queue. The seed records carry no byte estimates.
4. `credential_posture` — the typed `BootstrapCredentialPosture`:
   `no_credentials_needed`, `local_identity_inherited`,
   `handle_present_not_yet_used`, `browser_or_device_handoff_pending`,
   `provider_ticket_required`, `reauth_required`, or `unknown`. Carries
   handle / ticket refs only; raw secrets never appear.
5. `interrupted_recovery` — present only when the plan's resume state is
   one of the explicit interrupted branches. Names the resume state, the
   discard posture, whether a read-only partial root is inspectable now,
   and the explicit `branches` (`resume_acquisition`,
   `discard_and_restart`, `open_read_only_partial`, `refresh_mirror`,
   `switch_to_live_origin`). Never a binary retry / cancel.
6. `manual_followups` — the typed list of bootstrap items that have not
   yet succeeded, each with its item class, state, execution class, and
   absence class, so the surface renders typed work rather than one
   opaque spinner.
7. `evidence_packet` — the export-safe `BootstrapEvidencePacket` joining
   source identity, checkout plan, and bootstrap-queue refs with the
   union of typed evidence classes.
8. `honesty_labels` — the closed honesty vocabulary the surface renders
   verbatim (`mirror_lagged`, `mirror_stale`,
   `upstream_delta_outside_skew`, `offline_snapshot`,
   `signed_offline_bundle`, `signer_changed_review_required`,
   `signer_first_seen`, `read_only_partial`, `shallow_history`,
   `partial_clone`, `sparse_workset`, `submodule_init_pending`,
   `lfs_pointer_only`, `reauth_required`, `reconnect_required`,
   `policy_narrowed`).
9. `guardrails` — the typed `AcquisitionGuardrails` (see §6).

## 2 Checkout shape

| Field | Derived from | Meaning |
|---|---|---|
| `mode` | locator class + topology markers | `live_attach` for a live-resume target; `archive_extract` for snapshot / handoff / portable-state; `partial_clone` when a partial-clone-filter or promisor marker is present; `sparse_checkout` when a sparse-workset marker is present; `shallow_history` when a shallow marker is present; otherwise `full_checkout`. |
| `partial_or_sparse` | markers + posture + read-only roots | True when the checkout cannot claim full local truth (partial / sparse / shallow, a partially-acquired / filtered posture, or any read-only partial root). |
| `submodule_policy` | topology markers | `not_present` / `init_pending` / `init_partial` / `init_complete` / `init_failed`. |
| `lfs_policy` | topology markers | `not_present` / `pointer_only` / `hydrate_pending` / `hydrate_partial` / `hydrate_complete` / `hydrate_failed`. |
| `read_only_partial_roots` | plan | The typed roots the plan already made safe to inspect; rendered as `Read-only partial` with the class-specific chip. |

## 3 Acquisition verbs

The verb is resolved from the locator class and never guessed from the
destination. Clone is never confused with open-local-copy: the `clone`
verb only resolves for a `repo_url` or `mirror_or_proxy_repo` locator,
and `open_local` only resolves for a local folder / file / workspace /
workset manifest. Import (`handoff_packet` / `portable_state_package`),
Open archive (`snapshot_archive`), Open template or prebuild
(`template` / `prebuild_snapshot`), Resume (`live_resume_target` /
`recovery_checkpoint`), and Open deep link
(`review_or_work_item_deep_link`) each carry their own review, trust,
and recovery semantics.

## 4 Manual followups

A bootstrap item is a remaining manual follow-up when its state is
`pending`, `failed_recoverable`, `failed_blocking`, or any `awaiting_*`
state. Items that have `succeeded`, been `skipped`, are `running`, or
were `cancelled` are not remaining. Each follow-up carries the item
class, state, execution class, and typed absence class so the surface
can say "this item is blocked on network" or "this content is not yet
hydrated" rather than "setup failed".

## 5 Bootstrap queue

The post-open bootstrap queue is modelled item by item — submodule
init, LFS hydrate, partial-clone hydrate, package restore, toolchain
install, devcontainer attach, prebuild attach, extension restore, index
warm-up, docs import, credential provisioning, settings materialize,
and the rest of the closed item-class set — so the queue renders as a
typed progress list. Every item carries at least one typed
attributable-evidence entry; an item with no evidence is rejected at
projection time. Every failed, awaiting, or blocked item carries a
typed blocker and at least one typed repair hook.

## 6 Guardrails

The projection guarantees six typed guardrails, each mapping to a
guardrail or acceptance criterion in the acquisition spec:

| Guardrail | Holds when |
|---|---|
| `clone_not_confused_with_open` | The verb matches the locator: `clone` only for a remote / mirror locator, `open_local` only for a local one. |
| `no_implicit_repo_code_execution` | The plan blocks at least one repository-owned code path (hook, filter, generator, package script, auto-task) and no side-effecting queue item has already started or completed before trust admission. |
| `bootstrap_items_attributed` | Every enqueued bootstrap item carries at least one typed evidence entry. |
| `browse_safe_inspection_available` | The plan advertises at least one browse-safe action so the user can inspect before execution / setup approval. |
| `mirror_not_masquerading_as_live` | A mirror / proxy acquisition declares a non-live freshness class and carries mirror-freshness evidence; a lagged / stale source is never rewritten to fresh / live. |
| `no_hidden_trust_elevation` | A signer change that requires review never sits on an admitted stage without a review hook, and `reauth_required` / `reconnect_required` / `signer_review_required` stages surface their typed next-step hook. |

`AcquisitionGuardrails::all_hold()` is true only when every guardrail
holds.

## 7 Honesty labels

Honesty labels are the alpha-level vocabulary a surface renders verbatim
alongside the acquisition row. A mirror-served acquisition renders
`mirror_lagged` / `mirror_stale` and `upstream_delta_outside_skew`
distinctly; an offline or signed-offline bundle renders
`offline_snapshot` / `signed_offline_bundle`; a first-seen or
review-required signer renders `signer_first_seen` /
`signer_changed_review_required`; partial / sparse / shallow / pointer /
read-only state renders the matching topology label; and a
policy-narrowed plan renders `policy_narrowed`.

## 8 Credential posture

`credential_posture` names the credential class without ever carrying a
raw secret. It is derived from the locator auth mode and the plan trust
stage: an `ssh_agent` / `pat_handle` source reads
`handle_present_not_yet_used`; an `oauth_handle` / `device_code_handle`
reads `browser_or_device_handoff_pending`; a `managed_session_ticket` /
`connected_provider_ticket` reads `provider_ticket_required`; an
`anonymous` / `none` / local source reads `no_credentials_needed`; and a
`reauth_required` trust stage, an expired / revoked live attach
authority, or an `auth_expired` / `authority_revoked` item blocker reads
`reauth_required`.

## 9 Interrupted recovery

Interrupted acquisition has explicit branches, never a binary retry /
cancel. When the plan's resume state is `interrupted_resumable`,
`interrupted_discard_required`, or
`interrupted_open_read_only_available`, the projection emits an
`interrupted_recovery` card naming the resume state, the discard
posture, whether a read-only partial root is inspectable now, and the
explicit branches drawn from the plan's next-step decision hooks. The
card guarantees at least one explicit branch even if the plan
under-specified its hooks, and it is always export-safe so support and
enterprise rollout can replay it.

## 10 Fixture coverage

The scenario fixture suite covers, at minimum:

- `open_local_folder` — Open folder, local, no fetch, no setup.
- `clone_remote_submodules_lfs` — Clone, submodule init pending, LFS
  pointer-only, three deferred bootstrap items.
- `open_snapshot_archive` — Open archive, offline snapshot, read-only
  extraction.
- `import_handoff_packet` — Import bundle, first-seen signer, read-only
  extracted bundle.
- `open_template_with_bootstrap_queue` — Open prebuild, typed queue of
  toolchain / package / index / docs items.
- `resume_managed_workspace_reauth` — Resume, live attach, reauth
  required.
- `interrupted_mirror_clone_resume` — interrupted mirror clone with
  explicit resume / discard / open-read-only branches and a lagged
  mirror delta.
- `airgap_signed_bundle` — air-gap import preserving signed-offline
  freshness and preauthorized signer rotation.
- `policy_guided_generators_blocked` — fleet policy narrowing with a
  policy-excluded generator-install item.
- `lfs_pointer_only_read_only` — shallow + LFS pointer-only read-only
  browse.

Removing any of these scenarios without a replacement fixture is a
breaking contract change.
