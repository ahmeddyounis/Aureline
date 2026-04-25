# Review-workspace, comment-anchor, and merge-queue contract

This document freezes the cross-tool review-workspace object model that
every Aureline surface reads when it presents code review, comment
anchoring, or landing-action state. Local review (a checked-out branch
or worktree under the user's own clone), provider overlays (the hosted
review state mirrored from a remote code host), review bundles imported
from outside, and browser-handoff handoffs all resolve to one
`review_workspace_record`. Anchored comments resolve to one
`review_anchor_record`. Merge-queue / landing-action rows resolve to
one `merge_queue_action_record`. No surface mints a parallel "review
state" vocabulary.

The machine-readable boundaries are:

- [`/schemas/vcs/review_workspace.schema.json`](../../schemas/vcs/review_workspace.schema.json)
  — the `review_workspace_record` and `merge_queue_action_record`
  shapes.
- [`/schemas/vcs/review_anchor.schema.json`](../../schemas/vcs/review_anchor.schema.json)
  — the `review_anchor_record` shape and the closed
  comment-drift / freshness vocabularies the workspace and anchor
  records both cite.

Worked cases (local branch with no provider overlay; provider overlay
fresh; provider overlay stale during a degraded fetch; review bundle
imported offline; browser-handoff token source; durable comment anchor
across a refactor; comment anchor that drifts and refuses silent
retargeting; merge-queue action blocked because provider rule
freshness is unverified; merge-queue action allowed under fresh
provider rules; merge-queue action attributable through approval
ticket and command id) live under
[`/fixtures/vcs/review_workspace_cases/`](../../fixtures/vcs/review_workspace_cases/).

The eventual review-workspace crate's Rust types are the schema of
record. This document and the JSON Schema exports are the cross-tool
boundary every non-owning surface reads. The hosted-review inbox
contract and the merge-policy contract are forward dependencies: when
they land, this contract MUST be the upstream they cite for review-
workspace, anchor, and merge-queue identity. If this document and a
later hosted-review or merge-policy contract disagree, those contracts
win for hosted-review and merge-policy semantics and this document MUST
be updated in the same change.

Companion artifacts:

- [`/schemas/review/review_surface_record.schema.json`](../../schemas/review/review_surface_record.schema.json)
  and
  [`/docs/review/structured_artifact_review_seed.md`](../review/structured_artifact_review_seed.md)
  — the structured-artifact review-surface matrix every comment
  anchor and review-workspace row reads to know whether an artifact
  is line-oriented, cell-aware, structure-aware, perceptual,
  immutable, or sidecar-bound. Review workspaces never invent a
  per-row review-surface vocabulary; they cite this matrix.
- [`/schemas/docs/citation_anchor.schema.json`](../../schemas/docs/citation_anchor.schema.json)
  and
  [`/docs/docs_integrity/citation_and_reference_contract.md`](../docs_integrity/citation_and_reference_contract.md)
  — the docs citation model used when a review comment cites docs,
  runbooks, or release notes; comments do not mint a parallel docs
  citation shape.
- [`/schemas/navigation/navigation_artifacts.schema.json`](../../schemas/navigation/navigation_artifacts.schema.json)
  and
  [`/docs/navigation/navigation_and_saved_query_contract.md`](../navigation/navigation_and_saved_query_contract.md)
  — the navigation artifacts (breadcrumb, outline, bookmark, history,
  peek, search-collection snapshot) that are also consumed inside
  review surfaces. Review-workspace rows reuse the
  `review_diff_hunk_target` navigation-target kind; they do not
  re-mint it.
- [`/schemas/integration/browser_handoff_packet.schema.json`](../../schemas/integration/browser_handoff_packet.schema.json)
  and
  [`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md)
  — the browser-handoff packet model every browser_handoff_token
  source resolves to. Raw URLs never appear on a review-workspace row.
- [`/schemas/integration/approval_ticket.schema.json`](../../schemas/integration/approval_ticket.schema.json)
  — the approval-ticket model every mutation-class merge-queue action
  cites. A landing action never appears available without resolving
  to an approval ticket plus a command id.
- [`/schemas/governance/capability_lifecycle.schema.json`](../../schemas/governance/capability_lifecycle.schema.json)
  — re-exported `freshness_class`, `client_scope`, and
  `redaction_class` vocabularies (ADR-0011). The review-workspace
  contract never redefines them.
- [`/schemas/identity/policy_bundle.schema.json`](../../schemas/identity/policy_bundle.schema.json)
  — re-exported `workspace_trust_state_class` and policy / trust
  envelope (ADR-0001 / ADR-0018). A review action never appears
  available under an unset trust decision.
- [`/schemas/workspace/checkout_plan.schema.json`](../../schemas/workspace/checkout_plan.schema.json)
  and
  [`/schemas/workspace/source_locator.schema.json`](../../schemas/workspace/source_locator.schema.json)
  — opaque locators for the local branch / worktree the review
  workspace pins. Raw absolute paths never appear on a review-
  workspace row.

Normative source anchors:

- `.t2/docs/Aureline_Technical_Architecture_Document.md`
  Appendix CJ (review workspace, anchoring, and merge-queue matrix)
  and §16.7 (review-and-diff architecture).
- `.t2/docs/Aureline_PRD.md` — review-workspace MUST/SHOULD language
  for honest provider freshness, durable comment anchors, and
  attributable landing actions.

If this contract disagrees with those sources, those sources win and
this document plus the schemas and fixtures update in the same change.

## Why freeze this now

Until this contract lands, every surface that touches a review object
would be free to invent its own review-state vocabulary:

- Local review of a checked-out branch would mint per-surface
  "review state" enums, with no shared way to say "this is the local
  truth and the provider overlay is degraded".
- Provider-overlay rows would silently relabel cached review state
  as authoritative, so a reviewer working through a provider outage
  could not tell whether the approval cue or the merge-queue cue
  reflected current provider state or a stale cache.
- Review bundles imported from a colleague's export would land with
  no source class, so a downstream merge attempt could treat the
  bundle as if it had been re-fetched from the canonical provider.
- Browser-handoff token sources would carry raw URLs through review
  rows, violating ADR-0010's no-raw-URL boundary.
- Comment anchors would be free to silently retarget after a refactor
  ("the old line moved here") even when the surface could not prove
  the anchor's intent followed the move. The reviewer would then see
  comments rendered against text the original author never read.
- Merge-queue / landing actions would appear available on a row whose
  provider rule snapshot had not been freshly verified, without an
  attributable command id and without a paired approval ticket. The
  reviewer would press the button and discover the staleness only on
  the provider's failure path.

Freezing one record family (`review_workspace_record`,
`review_anchor_record`, `merge_queue_action_record`) and the closed
freshness, drift, and source-class vocabularies they read solves all
five problems in one shape.

## Scope

Frozen at this revision:

1. The `review_workspace_record` shape every surface reads to identify
   one review workspace, including:
   - one `review_workspace_source_class` from the closed five-value
     vocabulary (`local_branch_or_worktree`, `provider_overlay_fetched`,
     `review_bundle_imported`, `browser_handoff_token_source`,
     `composite_local_with_provider_overlay`);
   - the local locator (a `workspace_id_ref` plus a `branch_or_worktree_ref`
     into the workspace family) when the workspace has a local truth;
   - the optional `provider_overlay` block (provider-class, host-side
     object identity, last-fetched timestamp, and one of the four
     `provider_overlay_freshness_class` values:
     `provider_overlay_fresh`, `provider_overlay_stale_within_grace`,
     `provider_overlay_stale_beyond_grace_local_continues`,
     `provider_overlay_unavailable_local_continues`);
   - the `provider_authority_class` per row class
     (`provider_authoritative`, `local_parity_estimate`,
     `local_truth_only_no_provider_overlay`,
     `imported_bundle_snapshot`, `browser_handoff_token_only`);
   - the imported-bundle envelope and the browser-handoff envelope;
   - the `policy_context` (epoch + workspace trust state) and the
     `redaction_class` the workspace publishes under;
   - the `client_scopes` the workspace is admitted on;
   - the closed `review_workspace_lifecycle_state` vocabulary
     (`open_under_review`, `provider_overlay_degraded_local_continues`,
     `merged_landed`, `closed_unmerged`, `archived_tombstone`); and
   - the audit-event row shape on the `review_workspace` audit stream.

2. The `review_anchor_record` shape every anchored comment reads,
   including:
   - the anchor's `review_workspace_id_ref`;
   - the anchored `review_artifact_class` (cited from the structured-
     artifact review matrix, never re-minted) and the `review_surface_class`
     the anchor is rendered through;
   - the `anchor_target_kind` from the closed seven-value vocabulary
     (`text_line_range_anchor`, `cell_aware_notebook_anchor`,
     `structure_aware_config_node_anchor`, `image_region_anchor`,
     `evidence_section_anchor`, `whole_artifact_anchor`,
     `commit_or_revision_anchor`);
   - opaque `target_ref` and an optional `secondary_target_ref` for
     compound anchors (e.g. an image region tied to a paired diff
     hunk);
   - the `anchor_drift_state` from the closed six-value vocabulary
     (`anchor_bound_exact`, `anchor_remapped_with_recorded_mapping`,
     `anchor_drifted_user_must_resolve`,
     `anchor_target_deleted_re_anchor_or_resolve`,
     `anchor_scope_unavailable`, `anchor_archived_tombstone`);
   - the matching `anchor_drift_required_user_action`, paired through
     an allOf gate so a downstream surface that re-renders a drifted
     or deleted anchor as bound denies with
     `anchor_drift_state_required_action_mismatch`;
   - the `local_vs_provider_freshness_class` from the closed five-
     value vocabulary
     (`local_only_no_provider_overlay`, `local_and_provider_match_fresh`,
     `local_and_provider_match_stale_within_grace`,
     `local_and_provider_disagree_user_review_required`,
     `provider_overlay_unavailable_local_continues`);
   - the `comment_payload_redaction_class` and the
     `comment_payload_label_opaque_ref` (raw bodies never cross this
     boundary; the label points at the reviewable label registry);
   - the `posted_actor_ref`, `posted_at`, `superseded_by_anchor_id_ref`,
     and `archived_at`/`tombstoned_at` slots; and
   - the audit-event row shape on the `review_anchor` audit stream.

3. The `merge_queue_action_record` shape every landing-action row
   reads, including:
   - the `target_review_workspace_id_ref` and the bound
     `provider_rule_snapshot` block (provider-rule
     identity ref, snapshot taken-at timestamp, and one of the closed
     `provider_rule_snapshot_freshness_class` values:
     `provider_rule_snapshot_fresh`,
     `provider_rule_snapshot_stale_within_grace`,
     `provider_rule_snapshot_stale_beyond_grace_blocked`,
     `provider_rule_snapshot_unverifiable_blocked`);
   - the `merge_queue_action_class` from the closed eight-value
     vocabulary
     (`enqueue_for_merge`, `dequeue_from_merge`, `attempt_landing`,
     `mark_blocked_pending_freshness`,
     `mark_blocked_pending_required_check`,
     `mark_blocked_pending_required_review`,
     `mark_landed`, `mark_failed_to_land`);
   - the `actor_ref` (who initiated), the `command_id_ref` (the
     command-dispatch id under which it was invoked), and the
     `approval_ticket_ref` for every mutation-class action;
   - the `merge_queue_action_blocked_reason` from the closed seven-
     value vocabulary
     (`provider_rule_snapshot_stale`,
     `required_check_unverified_or_failing`,
     `required_review_missing_or_dismissed`,
     `target_review_workspace_drifted`,
     `policy_epoch_expired_re_evaluation_required`,
     `workspace_trust_unset_or_restricted`,
     `not_blocked_action_admissible`),
     paired through allOf gates so an `enqueue_for_merge` /
     `attempt_landing` / `mark_landed` action that names a non-
     `not_blocked_action_admissible` reason denies with
     `merge_queue_action_must_not_appear_available_under_blocked_reason`;
   - the `merge_queue_action_lifecycle_state` from the closed six-
     value vocabulary
     (`proposed`, `enqueued`, `landed`, `dequeued`, `failed`,
     `superseded`); and
   - the audit-event row shape on the `merge_queue_action` audit
     stream.

4. The acceptance invariants this contract enforces:

   - Local review remains usable when provider status is stale or
     unavailable. A `provider_overlay_stale_beyond_grace_local_continues`
     or `provider_overlay_unavailable_local_continues` overlay row
     MUST resolve `provider_authority_class = local_truth_only_no_provider_overlay`
     (or `local_parity_estimate` for read-only overlay reuse) and
     MUST NOT relabel cached approval / mergeability / review-required
     cues as authoritative.
   - Comment anchors are durable. A refactor that moves the line under
     an anchor MUST be expressed as either
     `anchor_remapped_with_recorded_mapping` (with a `remap_chain_target_id_refs`
     citation that records the mapping the resolver used) or as one
     of the drift / deletion / scope-unavailable states. Silent
     retargeting denies with `silent_anchor_relocation_forbidden`.
   - Landing actions are attributable. Every mutation-class
     `merge_queue_action_record` carries a non-null
     `actor_ref`, `command_id_ref`, and `approval_ticket_ref`. An
     action without those refs denies with
     `merge_queue_action_attribution_missing`.
   - Landing actions cannot appear available when freshness or
     provider state is missing. The allOf gate above forbids a
     `mark_blocked_pending_*` reason on an
     `enqueue_for_merge` / `attempt_landing` / `mark_landed` row.
   - Provider-authoritative rows are mechanically distinguishable
     from local parity estimates. The
     `provider_authority_class` field is required and closed; no row
     may omit it or invent a new value.
   - Forward links are reserved. Every record carries a
     `hosted_review_inbox_record_id_ref` slot (currently always
     `null` until the hosted-review inbox contract lands) and a
     `merge_policy_record_id_ref` slot (currently always `null`
     until the merge-policy contract lands). Surfaces never embed an
     inline hosted-review or merge-policy row; they cite the future
     refs.

Out of scope until a superseding decision row opens:

- Implementing any provider HTTP / OAuth / GitHub / GitLab /
  Bitbucket / Azure DevOps adapter. The contract reserves
  `provider_class` and `provider_overlay` shape; the wire integration
  is a later lane.
- Implementing a real merge queue, batch policy, kernel, or worker.
  The contract reserves the row shape; the engine is a later lane.
- Building the hosted-review inbox surface or the merge-policy
  resolver. They are forward dependencies (slots reserved).
- Building the full review UI (split-diff renderer, comment threads,
  hover cards, suggestion editor). The contract reserves the data
  model the UI binds to.
- Cross-repo review, multi-host review aggregation, or queue
  federation. Out of scope at this revision.

## 1. The review-workspace record

Every review surface in Aureline (the local diff explorer, the hosted-
review reader, the merge-queue panel, the AI review-aware overlays,
the support-bundle review section) MUST resolve the workspace it is
operating on to exactly one `review_workspace_record`. The record is
the answer to five questions a reviewer must be able to answer
without opening any other object:

1. *Where does the truth live?* — `review_workspace_source_class`
   plus the local locator and / or the provider overlay.
2. *Who is authoritative for which row class?* —
   `provider_authority_class` per workspace; comment-anchor and
   merge-queue records cite the workspace's posture.
3. *How fresh is the provider overlay right now?* —
   `provider_overlay_freshness_class` plus
   `provider_overlay.last_fetched_at`.
4. *Under what trust / policy was this row admitted?* —
   `policy_context` (epoch, trust state, workspace trust state) and
   `client_scopes`.
5. *What is the workspace's lifecycle posture?* —
   `review_workspace_lifecycle_state`.

### 1.1 Source-class vocabulary

`review_workspace_source_class` is closed and exhaustive:

- `local_branch_or_worktree` — the workspace is a checked-out branch
  or a Git worktree under the user's own clone. There is no provider
  overlay attached. `provider_authority_class` is
  `local_truth_only_no_provider_overlay`.
- `provider_overlay_fetched` — the workspace is a provider-side
  review object (a pull request / merge request / changelist) whose
  state was fetched from the canonical code host. `provider_overlay`
  is required and `provider_authority_class` is
  `provider_authoritative` for rows the provider owns
  (approval state, required-check state, mergeability hint, queue
  state) and `local_parity_estimate` for rows the local clone
  computes (line-level diff intent, draft local comments).
- `review_bundle_imported` — the workspace was hydrated from a review
  bundle exported from another instance. `provider_authority_class`
  is `imported_bundle_snapshot`. The bundle's source class travels
  on the bundle envelope; this row never re-fetches against the
  canonical provider unless the user explicitly opts the workspace
  back to `composite_local_with_provider_overlay`.
- `browser_handoff_token_source` — the workspace was opened through
  a browser-handoff packet (ADR-0010) with a return-anchor binding
  back into Aureline. `provider_authority_class` is
  `browser_handoff_token_only`. The packet ref is the only authority
  the row carries; raw URLs never appear.
- `composite_local_with_provider_overlay` — the workspace pairs a
  local branch / worktree with a fetched provider overlay so the
  reviewer can compare local truth against provider truth. Both the
  local locator and the `provider_overlay` block are required.
  `provider_authority_class` resolves per row class as in
  `provider_overlay_fetched`.

### 1.2 Provider-overlay freshness

`provider_overlay_freshness_class` is closed and named so the chip
the reviewer reads is mechanical:

- `provider_overlay_fresh` — the overlay was fetched within the
  workspace's grace window and the reviewer may rely on the
  provider-authoritative cues.
- `provider_overlay_stale_within_grace` — the overlay is past its
  ideal freshness but still inside the grace window; the chip
  surfaces "stale" but the workspace remains usable.
- `provider_overlay_stale_beyond_grace_local_continues` — the
  overlay is past the grace window. Provider-authoritative cues are
  no longer rendered as authoritative; local review continues.
- `provider_overlay_unavailable_local_continues` — the provider was
  unreachable on the last attempt. Local review continues; provider-
  authoritative cues fall back to the last-known cached label
  marked degraded.

The acceptance invariant: a workspace whose freshness resolves to
`provider_overlay_stale_beyond_grace_local_continues` or
`provider_overlay_unavailable_local_continues` MUST resolve
`provider_authority_class` to `local_truth_only_no_provider_overlay`
or `local_parity_estimate` for any row that names a mergeability,
approval, or queue-state cue. A surface that renders a stale or
unavailable overlay's approval / mergeability cue as authoritative
denies with `provider_overlay_authority_must_not_outlive_freshness`.

### 1.3 Provider-authority labels per row class

The workspace declares one `provider_authority_class` for the
workspace as a whole. Anchor and merge-queue rows cite this posture
when they render their own cues:

- `provider_authoritative` — the named row class is authoritative on
  the provider side. Approval state, required-check state,
  mergeability hint, and queue-state rows whose workspace is
  `provider_authoritative` MUST cite the provider's last-fetched
  snapshot, never an Aureline-local guess.
- `local_parity_estimate` — the named row class is computed locally
  from a fresh provider snapshot. The row is honest about the
  parity boundary: an approval-cue row with this label MUST surface
  the typed "estimated locally" disclosure.
- `local_truth_only_no_provider_overlay` — the named row class has
  no provider companion. The row is local truth.
- `imported_bundle_snapshot` — the named row class was carried by
  the imported bundle. Re-fetching against the canonical provider
  is forbidden until the user opts the workspace back to a composite
  posture.
- `browser_handoff_token_only` — the named row class only resolves
  through a browser-handoff packet round-trip. Local mutations are
  forbidden.

### 1.4 Lifecycle states

`review_workspace_lifecycle_state` is closed and ordered:

- `open_under_review` — active workspace, comments and merge-queue
  actions admissible per provider authority and policy.
- `provider_overlay_degraded_local_continues` — the workspace's
  provider overlay is stale-beyond-grace or unavailable; local
  review continues; provider-authoritative cues are degraded.
- `merged_landed` — the workspace's target review object was
  successfully landed. Audit refs cite the
  `merge_queue_action_record` whose `mark_landed` action closed it.
- `closed_unmerged` — the workspace's target review object was
  closed without landing.
- `archived_tombstone` — the workspace was archived for retention or
  cleanup; the row remains as a tombstone for audit / restore.

## 2. The review-anchor record

Every comment, suggestion, or evidence pointer attached inside a
review surface MUST resolve to exactly one `review_anchor_record`.
The record is the answer to four questions a reviewer must be able to
answer without opening the comment payload:

1. *What is anchored, and which review surface renders it?* —
   `review_artifact_class` (cited from the structured-artifact review
   matrix) plus `review_surface_class`.
2. *How is the anchor pinned?* — `anchor_target_kind` plus
   `target_ref` (and `secondary_target_ref` for compound anchors).
3. *Did the underlying object move, drift, or disappear?* —
   `anchor_drift_state` plus the matching
   `anchor_drift_required_user_action`.
4. *Is the anchor's local view in agreement with the provider
   overlay?* — `local_vs_provider_freshness_class`.

### 2.1 Anchor-target kinds

`anchor_target_kind` is closed and reads only against the structured-
artifact review matrix (`review_artifact_class`):

- `text_line_range_anchor` — line range in a line-oriented artifact.
  Pairs with `review_artifact_class` values whose review surface is
  line-aware.
- `cell_aware_notebook_anchor` — pairs with `jupyter_notebook` and
  carries a stable cell id ref. Notebook merge / review rules from
  the structured-artifact contract apply unchanged.
- `structure_aware_config_node_anchor` — pairs with the
  `structured_config_*` classes and carries a key path or section
  ref. Comments anchor to documented keys, not to byte offsets.
- `image_region_anchor` — pairs with `image_or_design_snapshot` and
  carries a region ref. Perceptual review rules apply.
- `evidence_section_anchor` — pairs with `evidence_packet`. Evidence
  is immutable; comments anchor read-only sections.
- `whole_artifact_anchor` — anchor binds to the artifact as a whole
  (used for general review notes, sign-off comments, or
  release-evidence summaries).
- `commit_or_revision_anchor` — anchor binds to a commit / revision
  identity rather than to an artifact. Used for "this commit
  introduced the regression" style comments.

### 2.2 Drift states and required actions

`anchor_drift_state` is closed and pairs through an allOf gate with
`anchor_drift_required_user_action`:

| `anchor_drift_state`                           | `anchor_drift_required_user_action`                            |
|-----------------------------------------------|----------------------------------------------------------------|
| `anchor_bound_exact`                          | `no_user_action_required_anchor_bound_or_remapped`             |
| `anchor_remapped_with_recorded_mapping`       | `no_user_action_required_anchor_bound_or_remapped`             |
| `anchor_drifted_user_must_resolve`            | `user_must_pick_successor_or_dismiss_drifted`                  |
| `anchor_target_deleted_re_anchor_or_resolve`  | `user_must_re_anchor_or_resolve_deleted_target`                |
| `anchor_scope_unavailable`                    | `user_must_widen_scope_or_load_pack_or_reach_remote`           |
| `anchor_archived_tombstone`                   | `user_must_restore_from_archive_or_acknowledge_tombstone`      |

A row whose pair is mismatched denies with
`anchor_drift_state_required_action_mismatch`. A row whose drift state
is `anchor_remapped_with_recorded_mapping` MUST cite a non-empty
`remap_chain_target_id_refs` recording the resolver's mapping. A row
whose drift state is `anchor_archived_tombstone` MUST cite
`archived_at`. A surface that re-renders any non-bound / non-remapped
anchor as bound (e.g. silently jumps the anchor onto a refactor
target) denies with `silent_anchor_relocation_forbidden` plus the
mismatched-pair denial above.

### 2.3 Local-vs-provider freshness on anchors

`local_vs_provider_freshness_class` is closed and named so the
reviewer can read the cue without opening the workspace:

- `local_only_no_provider_overlay` — the workspace has no provider
  overlay; the anchor reflects local truth.
- `local_and_provider_match_fresh` — the local anchor and the
  provider overlay agree and the overlay is fresh.
- `local_and_provider_match_stale_within_grace` — local and provider
  agree but the overlay is stale-within-grace; the chip surfaces
  "stale" without blocking.
- `local_and_provider_disagree_user_review_required` — local and
  provider overlay disagree on the anchor (e.g. provider has a
  newer comment thread the local view has not seen). The user MUST
  resolve before sending mutations through.
- `provider_overlay_unavailable_local_continues` — the provider
  overlay is unavailable; local review continues but the anchor
  cannot claim parity until the overlay returns.

## 3. The merge-queue action record

Every landing-action row MUST resolve to exactly one
`merge_queue_action_record`. The record is the answer to five
questions a reviewer must be able to answer without opening the
provider:

1. *Which workspace is this acting on?* —
   `target_review_workspace_id_ref`.
2. *Which provider rule snapshot is the action bound to, and how
   fresh is it?* — `provider_rule_snapshot.provider_rule_identity_ref`,
   `provider_rule_snapshot.taken_at`, and
   `provider_rule_snapshot_freshness_class`.
3. *Who is doing this and through which command?* — `actor_ref` and
   `command_id_ref`.
4. *Which approval ticket admitted this mutation?* —
   `approval_ticket_ref` (required for every mutation-class action).
5. *Is this admissible right now, and if not, why not?* —
   `merge_queue_action_class` paired with
   `merge_queue_action_blocked_reason`.

### 3.1 Action classes and blocked-reason gating

`merge_queue_action_class` is closed and exhaustive
(`enqueue_for_merge`, `dequeue_from_merge`, `attempt_landing`,
`mark_blocked_pending_freshness`,
`mark_blocked_pending_required_check`,
`mark_blocked_pending_required_review`,
`mark_landed`, `mark_failed_to_land`).

`merge_queue_action_blocked_reason` is closed and carries
`not_blocked_action_admissible` plus the six concrete blockers
(`provider_rule_snapshot_stale`,
`required_check_unverified_or_failing`,
`required_review_missing_or_dismissed`,
`target_review_workspace_drifted`,
`policy_epoch_expired_re_evaluation_required`,
`workspace_trust_unset_or_restricted`).

The allOf gate: rows whose `merge_queue_action_class` is
`enqueue_for_merge`, `attempt_landing`, or `mark_landed` MUST cite
`merge_queue_action_blocked_reason = not_blocked_action_admissible`.
A row that names any other reason on those classes denies with
`merge_queue_action_must_not_appear_available_under_blocked_reason`.
Rows whose class is `mark_blocked_pending_freshness`,
`mark_blocked_pending_required_check`, or
`mark_blocked_pending_required_review` MUST cite the matching
non-`not_blocked_action_admissible` reason.

### 3.2 Provider-rule-snapshot freshness

`provider_rule_snapshot_freshness_class` is closed:

- `provider_rule_snapshot_fresh` — the snapshot is inside the row's
  freshness window. Admissible for `enqueue_for_merge` /
  `attempt_landing` / `mark_landed`.
- `provider_rule_snapshot_stale_within_grace` — the snapshot is past
  ideal freshness but still inside the grace window; the chip
  surfaces "stale" without blocking. Surfaces SHOULD prompt a
  re-fetch.
- `provider_rule_snapshot_stale_beyond_grace_blocked` — the snapshot
  is past the grace window. Admissible only on
  `mark_blocked_pending_freshness` rows.
- `provider_rule_snapshot_unverifiable_blocked` — the snapshot
  cannot be verified (the provider is unreachable, or the rule
  identity could not be resolved). Admissible only on
  `mark_blocked_pending_freshness` rows.

A row whose snapshot freshness is
`stale_beyond_grace_blocked` or `unverifiable_blocked` and whose
class is not a `mark_blocked_pending_*` value denies with
`merge_queue_action_must_not_appear_available_under_blocked_reason`.

### 3.3 Attribution

Every mutation-class row (`enqueue_for_merge`, `dequeue_from_merge`,
`attempt_landing`, `mark_landed`, `mark_failed_to_land`) MUST cite a
non-null `actor_ref`, a non-null `command_id_ref`, and a non-null
`approval_ticket_ref`. The schema enforces this through allOf gates;
a row missing any of the three denies with
`merge_queue_action_attribution_missing`.

The `mark_blocked_pending_*` rows MAY omit `approval_ticket_ref`
(they are not mutation-class) but still MUST cite `actor_ref` and
`command_id_ref` so the row remains attributable.

## 4. Forward dependencies

Two refs are reserved on every record but currently always `null`:

- `hosted_review_inbox_record_id_ref` — the hosted-review inbox
  contract is a forward dependency. Once it lands, every
  `review_workspace_record` and `review_anchor_record` whose source
  class is `provider_overlay_fetched` or
  `composite_local_with_provider_overlay` MUST cite the inbox row
  the overlay was fetched through. Until then, the slot is reserved.
- `merge_policy_record_id_ref` — the merge-policy contract is a
  forward dependency. Once it lands, every
  `merge_queue_action_record` MUST cite the merge-policy row that
  resolved the action's admission. Until then, the slot is reserved.

The reserved slots survive the next contract's landing without a
breaking change: today they are nullable; later they become required
non-null when the upstream contract lands and bumps the
`review_workspace_schema_version` / `review_anchor_schema_version`
constants.

## 5. Audit streams

Two audit streams are reserved by this contract:

- `review_workspace_audit_event` — closed event-id vocabulary
  including `review_workspace_opened`, `review_workspace_provider_overlay_fetched`,
  `review_workspace_provider_overlay_freshness_changed`,
  `review_workspace_lifecycle_state_changed`,
  `review_workspace_imported_from_bundle`,
  `review_workspace_browser_handoff_minted`,
  `review_workspace_archived`,
  `review_workspace_audit_denial_emitted`. Denial events MUST cite
  one denial reason from the `review_workspace_denial_reason`
  vocabulary.
- `review_anchor_audit_event` — closed event-id vocabulary including
  `review_anchor_posted`, `review_anchor_drift_state_changed`,
  `review_anchor_remapped_with_chain`, `review_anchor_resolved`,
  `review_anchor_archived`, `review_anchor_audit_denial_emitted`.
  Denial events MUST cite one denial reason from the
  `review_anchor_denial_reason` vocabulary.
- `merge_queue_action_audit_event` — closed event-id vocabulary
  including `merge_queue_action_proposed`,
  `merge_queue_action_enqueued`, `merge_queue_action_blocked`,
  `merge_queue_action_landed`, `merge_queue_action_failed_to_land`,
  `merge_queue_action_dequeued`,
  `merge_queue_action_audit_denial_emitted`. Denial events MUST
  cite one denial reason from the
  `merge_queue_action_denial_reason` vocabulary.

The denial-reason vocabularies are listed in the schemas. Adding a
new denial reason or a new audit-event id is additive-minor and
bumps the per-record schema-version const; repurposing an existing
value is breaking and requires a new decision row.

## 6. Redaction posture

Every record published against this contract carries one
`redaction_class` from the re-exported capability-lifecycle
vocabulary. Defaults:

- Local-only review workspaces and their anchors default to
  `metadata_safe_default`.
- Workspaces whose source class is
  `provider_overlay_fetched`, `composite_local_with_provider_overlay`,
  or `browser_handoff_token_source` MAY raise to
  `internal_support_restricted` when the workspace touches
  organisationally restricted review state.
- Comments quoted by support exports MUST raise to
  `internal_support_restricted` and the
  `comment_payload_label_opaque_ref` MUST resolve through the
  redaction-aware label registry (the raw payload never crosses
  this boundary).
- Merge-queue action rows whose actor or approval ticket touches a
  credentialed flow MUST raise to `operator_only_restricted`.

Raw absolute paths, raw branch / commit URLs, raw author identity
strings, raw comment bodies, raw provider rule bodies, and raw
approval-ticket bodies never appear on any record published against
this contract. Every payload travels by opaque ref or through the
redaction-aware label registry.

## 7. Acceptance cross-walk

| Acceptance bullet from the plan | Where it lands |
|---|---|
| Local review remains usable when provider status is stale or unavailable, with cached overlays labeled honestly. | §1.2 freshness vocabulary + §1.3 authority labels + the `provider_overlay_authority_must_not_outlive_freshness` denial. Fixture `provider_overlay_stale_local_continues.yaml` and `provider_overlay_unavailable_local_continues.yaml`. |
| Comment-anchor fixtures show durable anchors, drift labelling, and no silent retargeting after refactors. | §2.2 drift-state allOf gate plus the `silent_anchor_relocation_forbidden` denial. Fixtures `anchor_durable_across_refactor_remapped.yaml` and `anchor_drifted_silent_retarget_denied.yaml`. |
| Queue or landing actions are attributable and cannot appear available when required freshness or provider state is missing. | §3.1 / §3.2 allOf gates plus the `merge_queue_action_attribution_missing` and `merge_queue_action_must_not_appear_available_under_blocked_reason` denials. Fixtures `merge_queue_action_attempt_landing_under_fresh_provider_rules.yaml`, `merge_queue_action_blocked_freshness_missing.yaml`, and `merge_queue_action_attribution_missing_denial.yaml`. |
| Fixture corpus includes one provider outage or stale overlay case where local review continuation remains possible without mislabelling mergeability or approval state. | Fixture `provider_overlay_unavailable_local_continues.yaml` exercises the outage path; `provider_overlay_stale_local_continues.yaml` exercises the stale path. Both pair with anchor fixtures that resolve to `provider_overlay_unavailable_local_continues` / `local_and_provider_match_stale_within_grace` to keep the chips honest. |

## 8. Versioning

Each schema in this family carries a document-level
`*_schema_version` const. Adding a new enum value, a new optional
property, or a new additive sub-record is additive-minor and bumps
the relevant `*_schema_version` const. Repurposing an existing value
is breaking and requires a new decision row. The schemas join the
`vcs` family row in
[`artifacts/governance/schema_families.yaml`](../../artifacts/governance/schema_families.yaml)
and each artifact joins
[`artifacts/governance/control_artifact_index.yaml`](../../artifacts/governance/control_artifact_index.yaml)
in the same change.
