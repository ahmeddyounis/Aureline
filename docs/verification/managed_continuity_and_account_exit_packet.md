# Managed continuity, account-exit rehearsal, export-before-suspend matrix, and local-baseline verification seed

This packet freezes one shared verification story for managed
continuity and account-exit claims. It exists so later About,
settings, work-item, AI, workspace, support, and offboarding surfaces
share one inspectable rehearsal object instead of inventing per-vendor
"service unavailable" or "access revoked" copy when a managed
workspace is suspended, expires, an account exits, a seat is lost, or
a policy suspends managed actions. It also exists so the local-first
and graceful-degradation claims that the product depends on can be
substantiated in writing — what survives locally during each
scenario, what must be exported before the scenario commits, and what
can be reopened locally afterward.

If this packet, the
[`continuity_account_exit_cases/`](../../fixtures/managed/continuity_account_exit_cases/)
fixtures, the
[`local_baseline_proof.yaml`](../../artifacts/managed/local_baseline_proof.yaml)
artifact, and the frozen managed-workspace-lifecycle and account-
seat-plan-and-exit contracts disagree, the frozen contracts win for
tooling and this packet must update in the same change. Where a
downstream surface invents a conflicting rehearsal, this packet wins
and that surface is non-conforming.

Companion artifacts:

- [`/fixtures/managed/continuity_account_exit_cases/`](../../fixtures/managed/continuity_account_exit_cases/)
  — worked rehearsal cases covering suspended workspace with local
  docs / review still available, account exit with export path,
  grace-period downgrade, local-only fallback after a service outage,
  seat loss with local-baseline continuation, and policy suspension
  with local-baseline continuation.
- [`/artifacts/managed/local_baseline_proof.yaml`](../../artifacts/managed/local_baseline_proof.yaml)
  — machine-readable local-baseline proof artifact naming, for every
  rehearsal case, the admissible-surface set that survives locally
  without an account or managed service, the export-before-suspend
  matrix posture per artifact kind, and the inherited contract refs
  that govern each cell.

Inherited contracts (this packet is a narrowing record; it does not
restate them):

- [`/docs/managed/managed_workspace_lifecycle_contract.md`](../managed/managed_workspace_lifecycle_contract.md)
  and [`/schemas/managed/workspace_lifecycle_state.schema.json`](../../schemas/managed/workspace_lifecycle_state.schema.json)
  freeze the 12-phase managed-workspace lifecycle, the per-resource
  persistence posture, the continuation posture, the retry posture,
  and the local-only-continuation admissible-surface set every
  rehearsal cites. The packet quotes a workspace lifecycle record id
  rather than re-deriving workspace state.
- [`/docs/managed/account_seat_plan_and_exit_contract.md`](../managed/account_seat_plan_and_exit_contract.md)
  and [`/schemas/managed/account_exit_packet.schema.json`](../../schemas/managed/account_exit_packet.schema.json)
  freeze the account / org / seat / plan / grace / posture-origin
  vocabulary, the artifact-disposition set, the required-disclosure
  classes, the self-hosted alternative classes, and the provider-
  linked workflow consequences. The packet quotes an account-exit
  packet record id rather than re-deriving account posture.
- [`/docs/managed/metering_and_usage_export_contract.md`](../managed/metering_and_usage_export_contract.md)
  freezes managed metering, quota, and usage-export truth. The
  rehearsal MAY cite quota state refs but it does not restate amounts.
- [`/docs/integrations/provider_account_mapping_and_offline_capture_contract.md`](../integrations/provider_account_mapping_and_offline_capture_contract.md)
  freezes the offline-capture surface set every rehearsal that names
  queued provider drafts cites.
- [`/docs/admin/org_admin_seat_and_fleet_contract.md`](../admin/org_admin_seat_and_fleet_contract.md)
  and [`/schemas/admin/seat_lifecycle_row.schema.json`](../../schemas/admin/seat_lifecycle_row.schema.json)
  freeze the seat lifecycle vocabulary every seat-loss rehearsal
  cites.
- [`/docs/governance/data_portability_and_exit_matrix.md`](../governance/data_portability_and_exit_matrix.md)
  and [`/artifacts/governance/portability_artifact_matrix.yaml`](../../artifacts/governance/portability_artifact_matrix.yaml)
  freeze per-domain export and offboarding posture; rehearsal artifact
  rows MUST be consistent with the matching portability row.

Normative sources projected here:

- `.t2/docs/Aureline_PRD.md` — local-first operation, graceful
  degradation, managed control-plane boundary, managed-workspace
  suspend / expiry / rebuild, account exit, seat exhaustion, plan
  downgrade, grace window, policy suspension.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` — managed-
  service separation, local-core resilience, mirror and offline
  posture, retirement drain window, kill-switch quarantine.
- `.t2/docs/Aureline_Technical_Design_Document.md` — managed-account
  identity binding, seat-vs-plan-vs-org axes, offboarding export
  pathways, control-plane degrade, data-plane degrade.
- `.t2/docs/Aureline_Milestones_Document.md` — continuity and exit
  claims kept as inspectable packets during the foundations phase
  rather than live product surfaces.

## Shared header

```yaml
schema_version: 1
header_kind: evidence_packet_header
packet_family: verification_packet
packet_id: verification.managed_continuity_and_account_exit.seed
evidence_id: evidence.verification.managed_continuity_and_account_exit.packet
title: Managed continuity, account-exit rehearsal, export-before-suspend matrix, and local-baseline verification seed
ownership:
  owner_dri: "@ahmeddyounis"
  evidence_owner: "@ahmeddyounis"
  backup_owner: null
  backup_waiver: single-maintainer-backup
coverage:
  requirement_ids:
    - GOV-EVID-901
    - GOV-TRUTH-901
    - GOV-CORPUS-901
    - ARCH-PACK-901
  claim_row_refs:
    - packet_row:managed_continuity_and_account_exit.rehearsal_record_contract
    - packet_row:managed_continuity_and_account_exit.export_before_suspend_matrix
    - packet_row:managed_continuity_and_account_exit.local_baseline_proof_artifact
    - packet_row:managed_continuity_and_account_exit.required_rehearsal_set
    - packet_row:managed_continuity_and_account_exit.seed_corpus
  covered_lanes:
    - release_evidence
    - support_export
    - governance_packets
    - docs_public_truth
result_status: seed_only
visibility_class: internal
freshness:
  captured_at: 2026-05-04T00:00:00Z
  stale_after: P30D
  freshness_class: warm_cached
  source_revision: managed_continuity_and_account_exit_seed@1
  trigger_revision: managed_continuity_and_account_exit_packet@1
environment:
  channel_context: not_applicable
  deployment_context:
    - not_applicable
  environment_summary: >
    Seed packet over the frozen managed-workspace-lifecycle and
    account-seat-plan-and-exit contracts already landed in the
    repository. No managed-workspace control plane, account-identity
    broker, offboarding executor, or rehearsal-automation harness is
    wired to this packet yet. Claims are structural: every case in
    the continuity-and-account-exit fixture set, every artifact-row
    posture in the export-before-suspend matrix, and every admissible
    surface in the local-baseline proof reuses existing frozen tokens
    rather than minting new per-surface language.
artifact_links:
  supporting_evidence_ids:
    - evidence.verification.managed_continuity_and_account_exit.continuity_account_exit_cases
    - evidence.verification.managed_continuity_and_account_exit.local_baseline_proof
    - evidence.managed.workspace_lifecycle_cases
    - evidence.managed.account_exit_cases
    - evidence.managed.managed_workspace_lifecycle_contract
    - evidence.managed.account_seat_plan_and_exit_contract
  exact_build_identity_refs: []
  fixture_refs:
    - fixtures/managed/continuity_account_exit_cases/
    - fixtures/managed/workspace_lifecycle_cases/
    - fixtures/managed/account_exit_cases/
  archetype_refs: []
  source_anchor_refs:
    - docs/managed/managed_workspace_lifecycle_contract.md
    - docs/managed/account_seat_plan_and_exit_contract.md
    - docs/managed/metering_and_usage_export_contract.md
    - docs/integrations/provider_account_mapping_and_offline_capture_contract.md
    - docs/admin/org_admin_seat_and_fleet_contract.md
    - docs/governance/data_portability_and_exit_matrix.md
    - schemas/managed/workspace_lifecycle_state.schema.json
    - schemas/managed/account_exit_packet.schema.json
    - artifacts/governance/portability_artifact_matrix.yaml
    - artifacts/managed/local_baseline_proof.yaml
  waiver_refs: []
  known_limit_refs: []
  migration_packet_refs:
    - artifacts/governance/portability_artifact_matrix.yaml
```

## Summary

This seed packet freezes:

- one reviewer-facing `managed_continuity_rehearsal_record` shape every
  rehearsal that quotes managed continuity or account-exit posture
  reuses, naming the rehearsal class, the bound workspace lifecycle
  record id, the bound account-exit packet id, the export-before-
  suspend artifact rows, the local-baseline admissible-surface set,
  the reopen-locally-afterward set, and the inherited-contract refs;
- one closed `rehearsal_class` vocabulary covering managed-workspace
  suspension, managed-workspace expiry, account exit, seat loss,
  plan or grace downgrade, policy suspension, and local-only
  fallback after a managed-service outage;
- one closed `export_artifact_kind_class` vocabulary covering the
  seven artifact kinds the spec names: workspace files, review
  packets, evidence packets, notebooks, workflow bundles, queued
  provider drafts, and support captures;
- one closed `export_before_suspend_disposition_class` vocabulary
  with explicit projections onto the existing artifact-disposition
  vocabulary so rehearsal rows do not silently mint a parallel "best
  effort" disposition;
- one closed `local_baseline_admissible_surface_class` vocabulary
  re-exported verbatim from the managed-workspace-lifecycle contract
  so a rehearsal that narrows any of these surfaces is non-conforming;
- one closed `reopen_locally_afterward_class` vocabulary covering the
  reopen-after-close paths admissible without a managed account or
  managed service; and
- one seed corpus covering every required rehearsal named in the
  spec — suspended workspace with local docs / review still
  available, account exit with export path, grace-period downgrade,
  local-only fallback after service outage — plus seat loss and
  policy suspension so all five scenario classes named in the
  spec are exercised.

It does not claim a managed-workspace control plane, an account-
identity broker, an offboarding executor, or a live rehearsal-
automation harness is wired up. It claims only that the packet, the
continuity-and-account-exit fixture set, and the local-baseline proof
artifact now exist in one reviewable form and reuse the frozen
managed-continuity vocabulary already landed elsewhere.

## Claim coverage

| Packet row | Requirement id(s) | Status | Visibility | Supporting evidence ids | Notes |
|---|---|---|---|---|---|
| `packet_row:managed_continuity_and_account_exit.rehearsal_record_contract` | `GOV-EVID-901`, `GOV-TRUTH-901` | `seed_only` | `internal` | `evidence.verification.managed_continuity_and_account_exit.continuity_account_exit_cases` | Freezes one machine-readable `managed_continuity_rehearsal_record` shape every rehearsal surface reuses. |
| `packet_row:managed_continuity_and_account_exit.export_before_suspend_matrix` | `GOV-TRUTH-901`, `ARCH-PACK-901` | `seed_only` | `internal` | `evidence.verification.managed_continuity_and_account_exit.local_baseline_proof` | Closed export-before-suspend matrix over the seven required artifact kinds. |
| `packet_row:managed_continuity_and_account_exit.local_baseline_proof_artifact` | `GOV-EVID-901`, `GOV-TRUTH-901` | `seed_only` | `internal` | `evidence.verification.managed_continuity_and_account_exit.local_baseline_proof` | Machine-readable local-baseline proof per rehearsal class. |
| `packet_row:managed_continuity_and_account_exit.required_rehearsal_set` | `GOV-CORPUS-901`, `GOV-TRUTH-901` | `seed_only` | `internal` | `evidence.verification.managed_continuity_and_account_exit.continuity_account_exit_cases` | Required rehearsal set covers the four spec-named scenarios plus seat loss and policy suspension. |
| `packet_row:managed_continuity_and_account_exit.seed_corpus` | `GOV-CORPUS-901`, `GOV-EVID-901` | `seed_only` | `internal` | `evidence.verification.managed_continuity_and_account_exit.continuity_account_exit_cases` | Stable case-id set covers every rehearsal class named in the spec. |

## Core rule

A managed continuity claim is not a slogan. Every assertion that
"local edit / save / search / Git / tasks / BYOK AI keep working" or
"the user can export and reopen locally" MUST point at a
`managed_continuity_rehearsal_record` whose bound workspace lifecycle
record id, account-exit packet id, export-before-suspend matrix rows,
and local-baseline admissible-surface set survive review. A surface
that asserts continuity without quoting a rehearsal record id is
non-conforming.

A rehearsal that cannot show what stays usable locally during the
scenario is non-conforming. A rehearsal that omits what must be
exported before the scenario commits is non-conforming. A rehearsal
that fails to name what can be reopened locally afterward is
non-conforming.

The packet narrows downstream language; it does not weaken the
inherited contracts. Where the rehearsal disagrees with the
managed-workspace-lifecycle or account-seat-plan-and-exit contract,
the inherited contract wins.

## Rehearsal classes

Every `managed_continuity_rehearsal_record` resolves to one closed
`rehearsal_class` token below. Surfaces MAY NOT mint additional
classes or collapse two distinct classes into one chip.

### `rehearsal_class` (frozen)

| Token | Meaning | Required inherited bindings |
|---|---|---|
| `managed_workspace_suspended` | Managed workspace is auto- or user-paused; filesystem preserved, compute released. | Workspace lifecycle record with `lifecycle_phase_class = suspended`. |
| `managed_workspace_expired` | Managed workspace expired (long-idle hibernation, retirement drain, session ticket, policy epoch, kill switch, successor image, or access-end window). | Workspace lifecycle record with `lifecycle_phase_class = expired` and a typed `expiry_reason_class`. |
| `account_exit` | User-initiated leave or admin-initiated transfer; account is in offboarding or transferred. | Account-exit packet with `account_state_class` in `{offboarding_in_progress, offboarded, transferred}`. |
| `seat_loss` | Seat suspended, reclaimed, downgraded, transferred, or deprovisioned independent of account-level state. | Account-exit packet whose `seat_state_class` is in `{suspended, reclaimed, downgraded, transferred, deprovisioned}` and whose `posture_origin_class` is `seat`. |
| `plan_grace_downgrade` | Account is in a typed grace window with a scheduled or committed plan downgrade. | Account-exit packet with `grace_state_class` in `{grace_active, grace_warning, grace_final_warning, grace_expired}` and `plan_state_class` outside `active_within_plan` and `not_applicable_individual_account`. |
| `policy_suspension` | Posture origin is `policy` or account is `locked_for_review`; managed actions are policy-suppressed. | Account-exit packet whose `posture_origin_class = policy` or `account_state_class = locked_for_review`. |
| `local_only_fallback_after_service_outage` | Managed control plane is unreachable; editor continues locally with a narrowed scope. | Workspace lifecycle record with `lifecycle_phase_class = degraded`, `degraded_reason_class = control_plane_failure`, and `posture_class = workspace_unavailable_local_only_continuation`. |

Rules (frozen):

1. A rehearsal whose class is `managed_workspace_suspended`,
   `managed_workspace_expired`, or
   `local_only_fallback_after_service_outage` MUST quote a
   `workspace_lifecycle_record_ref`. Silent omission is
   non-conforming.
2. A rehearsal whose class is `account_exit`, `seat_loss`,
   `plan_grace_downgrade`, or `policy_suspension` MUST quote an
   `account_exit_packet_ref`. Silent omission is non-conforming.
3. A rehearsal MAY quote both a `workspace_lifecycle_record_ref`
   and an `account_exit_packet_ref` when both axes are in scope; it
   MAY NOT collapse them into a single chip.
4. A `local_only_fallback_after_service_outage` rehearsal MUST set
   the local-baseline admissible-surface set to the full closed
   surface set named in this packet. Narrowing the set during a
   service outage is non-conforming.

## `managed_continuity_rehearsal_record`

Every rehearsal in the fixture corpus resolves to one record with the
following required fields. The field set projects the existing
`managed_workspace_lifecycle_state_record` and
`account_exit_packet_record` shapes; it does not redefine them.

- `case_id` — opaque, stable id, safe to log.
- `rehearsal_class` — closed token above.
- `rehearsal_summary` — one short sentence; never a raw URL or
  redaction-sensitive value.
- `workspace_lifecycle_record_ref` — required when
  `rehearsal_class` is in
  `{managed_workspace_suspended, managed_workspace_expired,
    local_only_fallback_after_service_outage}`; optional otherwise.
- `account_exit_packet_ref` — required when `rehearsal_class` is in
  `{account_exit, seat_loss, plan_grace_downgrade,
    policy_suspension}`; optional otherwise.
- `seat_lifecycle_row_refs` — required when `rehearsal_class` is
  `seat_loss`; optional otherwise.
- `metering_quota_state_refs` — optional.
- `posture_origin_class` — re-exported verbatim from the
  account-seat-plan-and-exit contract; required on every rehearsal
  that binds an account-exit packet.
- `export_before_suspend_rows` — non-empty list; one entry per
  applicable `export_artifact_kind_class`.
- `local_baseline_admissible_surfaces` — non-empty list of closed
  `local_baseline_admissible_surface_class` tokens.
- `reopen_locally_afterward` — non-empty list of closed
  `reopen_locally_afterward_class` tokens.
- `self_hosted_alternative_class_refs` — list of closed
  `self_hosted_alternative_class` tokens re-exported verbatim from
  the account-seat-plan-and-exit contract; required when
  `rehearsal_class` is in
  `{account_exit, seat_loss, plan_grace_downgrade,
    policy_suspension}`.
- `provider_linked_workflow_consequence_refs` — list of consequence
  rows quoted from the bound account-exit packet; required when an
  account-exit packet ref is present.
- `local_baseline_proof_ref` — opaque ref to the
  `local_baseline_proof.yaml` row that governs this rehearsal.
- `caveats` — list of reviewer-facing caveats; never a raw URL.
- `policy_context` — typed `policy_epoch`, `trust_state`,
  `deployment_profile_class`, and `redaction_class` block.
- `minted_at` — monotonic timestamp.
- `expires_at` — monotonic timestamp.
- `links` — block with `contract_ref`, `local_baseline_proof_ref`,
  and inherited contract refs.

Rule: a rehearsal that cannot fill `rehearsal_class`,
`export_before_suspend_rows`, `local_baseline_admissible_surfaces`,
or `reopen_locally_afterward` MUST deny render and route to a
`rehearsal_disclosure_incomplete` repair hook rather than fall back
to a generic "service unavailable" chip.

Rule: a rehearsal whose `rehearsal_class` requires a workspace
lifecycle ref or an account-exit packet ref but omits the bound ref
is non-conforming.

Rule: the rehearsal's `local_baseline_admissible_surfaces` set
MUST be a subset of the surface set declared in
`local_baseline_proof.yaml` for the matching `rehearsal_class`. A
narrower per-rehearsal set MUST cite a typed
`narrowing_reason_class`; otherwise the rehearsal MUST claim the
full set.

## Export-before-suspend matrix

The matrix is the contract for "what must be exported first" before
a suspend, exit, or downgrade commits. It exists so users do not
discover, post-suspend, that an artifact they depended on is now
read-only or discarded.

### `export_artifact_kind_class` (frozen)

| Token | Meaning |
|---|---|
| `workspace_files` | The user-edited workspace filesystem on the managed workspace volume or on the local host. |
| `review_packets` | Review packets (review evaluation results, change-stack review workspaces, traceability links) bound to managed surfaces. |
| `evidence_packets` | Mutation journal, route-truth packets, AI evidence, support evidence already on the managed workspace volume or local host. |
| `notebooks` | Notebook files and per-notebook scratch artifacts; kernel handles are not part of this kind (kernels never persist across phases). |
| `workflow_bundles` | Workflow bundle files (CI / CD / release / automation definitions) bound to managed surfaces. |
| `queued_provider_drafts` | Publish-later, browser-handoff, or offline-capture queue items the user has not yet drained to the provider. |
| `support_captures` | Support bundles already prepared or capture-able from local evidence. |

### `export_before_suspend_disposition_class` (frozen)

| Token | Meaning | Projects onto |
|---|---|---|
| `already_exported_local_artifact` | Already on the local host and survives the rehearsal close. | `exported_before_close`, `preserved_locally_after_close` |
| `must_export_before_suspend` | Read-write today, becomes read-only or discarded at the rehearsal close; export is required first. | `becomes_read_only_during_grace`, `becomes_read_only_during_suspension` |
| `exportable_during_access_end_window` | Export remains available without a support ticket while the access-end window is open. | `exportable_during_access_end_window` |
| `exportable_post_close_via_legal_hold` | Export available post-close only through a typed legal-hold or retention path. | `exportable_post_close_via_legal_hold`, `policy_held_post_close` |
| `discarded_at_close` | Discarded at the close transition by design (credentials and tokens are the canonical example). | `discarded_at_close` |
| `policy_held_post_close` | Held by policy after close; subject to retention rules. | `policy_held_post_close` |
| `redacted_by_policy` | Disposition exists but its content is redacted by policy. | `redacted_by_policy` |
| `preserved_locally_after_close` | Lives on the local host; survives the close. | `preserved_locally_after_close` |
| `not_applicable` | Not applicable for this rehearsal. | `not_applicable` |

### Required matrix per rehearsal class (minimum required cells)

| Artifact kind | `managed_workspace_suspended` | `managed_workspace_expired` | `account_exit` | `seat_loss` | `plan_grace_downgrade` | `policy_suspension` | `local_only_fallback_after_service_outage` |
|---|---|---|---|---|---|---|---|
| `workspace_files` | `must_export_before_suspend` or `preserved_locally_after_close` | `exportable_during_access_end_window` or `preserved_locally_after_close` | `exportable_during_access_end_window` | `exportable_during_access_end_window` | `must_export_before_suspend` or `preserved_locally_after_close` | `preserved_locally_after_close` | `preserved_locally_after_close` |
| `review_packets` | `must_export_before_suspend` | `exportable_during_access_end_window` | `exportable_during_access_end_window` or `policy_held_post_close` | `exportable_during_access_end_window` | `must_export_before_suspend` | `redacted_by_policy` or `must_export_before_suspend` | `preserved_locally_after_close` |
| `evidence_packets` | `must_export_before_suspend` | `exportable_during_access_end_window` | `exportable_during_access_end_window` | `exportable_during_access_end_window` | `must_export_before_suspend` | `redacted_by_policy` or `must_export_before_suspend` | `preserved_locally_after_close` |
| `notebooks` | `must_export_before_suspend` | `exportable_during_access_end_window` | `exportable_during_access_end_window` | `exportable_during_access_end_window` | `must_export_before_suspend` | `redacted_by_policy` or `must_export_before_suspend` | `preserved_locally_after_close` |
| `workflow_bundles` | `preserved_locally_after_close` or `must_export_before_suspend` | `preserved_locally_after_close` | `preserved_locally_after_close` | `preserved_locally_after_close` | `preserved_locally_after_close` | `preserved_locally_after_close` | `preserved_locally_after_close` |
| `queued_provider_drafts` | `must_export_before_suspend` | `must_export_before_suspend` | `must_export_before_suspend` | `must_export_before_suspend` | `must_export_before_suspend` | `must_export_before_suspend` | `preserved_locally_after_close` |
| `support_captures` | `already_exported_local_artifact` or `must_export_before_suspend` | `exportable_during_access_end_window` | `exportable_during_access_end_window` | `exportable_during_access_end_window` | `must_export_before_suspend` | `must_export_before_suspend` | `support_bundle_export_local_only` (cited via the local-baseline admissible-surface set) |

Rule: a row MAY claim a weaker disposition (closer to
`must_export_before_suspend`) than the matrix admits when the
underlying provider does not back the stronger guarantee. A row MAY
NOT claim a stronger disposition (closer to
`already_exported_local_artifact` or `preserved_locally_after_close`)
than the matrix admits.

Rule: every row whose disposition is `must_export_before_suspend`
MUST carry a `deadline_at` timestamp drawn from the bound workspace
lifecycle record's `expiry.expired_at` /
`access_end_window_closes_at` or the bound account-exit packet's
`grace.grace_window_closes_at` / `exit_intent.scheduled_close_at`.

Rule: every row whose disposition is `already_exported_local_artifact`
MUST carry an opaque `export_ref`. Surfaces that claim the artifact
is exported without an opaque ref are non-conforming.

Rule: every row whose artifact kind is `queued_provider_drafts` MUST
cite either an `offline_capture_continuation` admissible surface or
a typed `provider_linked_workflow_consequence_ref` from the bound
account-exit packet so the queued draft state cannot silently
disappear at suspend.

## Local-baseline admissible surfaces

A managed continuity claim is hollow without a typed admissible-
surface set. The packet re-exports the closed surface set frozen by
the managed-workspace-lifecycle contract verbatim so a rehearsal that
narrows any of these surfaces is structurally non-conforming.

### `local_baseline_admissible_surface_class` (frozen, re-exported)

| Token | Meaning |
|---|---|
| `file_open_and_edit` | Open and edit local files. |
| `file_save_to_local_disk` | Save edits to the local disk. |
| `local_search` | Search the local working tree. |
| `local_git` | Local Git read / write, commits, branches, diffs. |
| `local_tasks` | Already-authorized local tasks (build, test, run) that do not require the managed workspace. |
| `local_byok_ai` | Direct local or BYOK AI routes whose budget policy admits the current quota state. |
| `local_authorized_automation` | Already-authorized automation that does not cross the managed boundary. |
| `support_bundle_export_local_only` | Export a support bundle from local evidence only. |
| `offboarding_export_local_only` | Export already-prepared offboarding artifacts that live on the local host. |
| `already_exported_local_artifacts` | Read and re-export artifacts that have already crossed onto the local host. |

Rule: a `local_only_fallback_after_service_outage` rehearsal MUST
claim every surface in this set. Narrowing the set during a service
outage is non-conforming.

Rule: any other rehearsal class MAY narrow the set only by citing a
typed `narrowing_reason_class` (closed: `policy_suppressed_managed_action`,
`seat_revoked_local_history_only`, `byok_route_blocked_by_budget`,
`offline_capture_drained_already`). Narrowing without a typed reason
is non-conforming.

## Reopen-locally-afterward set

Every rehearsal MUST name what the user can reopen on the local host
after the rehearsal close. The list is closed.

### `reopen_locally_afterward_class` (frozen)

| Token | Meaning |
|---|---|
| `reopen_local_workspace_files` | Reopen the workspace files cloned or exported to the local host. |
| `reopen_local_review_packets` | Reopen review packets exported to the local host. |
| `reopen_local_evidence_packets` | Reopen evidence packets exported to the local host. |
| `reopen_local_notebooks` | Reopen notebooks saved or exported to the local host. |
| `reopen_local_workflow_bundles` | Reopen workflow bundles saved or exported to the local host. |
| `reopen_local_queued_drafts_offline_capture` | Reopen queued provider drafts via the offline-capture continuation. |
| `reopen_local_support_bundles` | Reopen already-exported support bundles. |
| `reopen_self_hosted_org_deployment` | Reopen managed-style administration via the self-hosted org deployment alternative. |
| `reopen_account_free_local_deployment` | Reopen the editor under the account-free local deployment profile. |
| `reopen_byok_local_ai_route` | Reopen AI routes via BYOK local inference. |
| `reopen_local_only_collaboration` | Reopen review and comments via the local-only collaboration alternative. |
| `reopen_signed_file_based_policy` | Reopen managed-style policy distribution via a signed file or bundle path. |
| `reopen_offline_capture_continuation` | Reopen provider-side capture via the offline-capture continuation. |

Rule: every rehearsal MUST name at least one
`reopen_locally_afterward_class` token. A rehearsal that asserts
"continuation" without a typed reopen path is non-conforming.

Rule: every `reopen_*` token whose effective continuation is a
self-hosted or local alternative MUST cite the matching
`self_hosted_alternative_class` token from the account-seat-plan-and-
exit contract.

## Local-baseline proof artifact

`local_baseline_proof.yaml` is the machine-readable companion every
rehearsal cites. Its rows are the durable answer to "what stays
usable locally without an account or managed service?" — one row per
`rehearsal_class`. A surface that claims local continuity for a
rehearsal class without resolving the row against this artifact is
non-conforming.

The artifact's row shape is:

- `rehearsal_class` — closed token above.
- `local_baseline_admissible_surfaces` — closed surface set per
  rehearsal class (full set for
  `local_only_fallback_after_service_outage`; narrowed-with-reason
  permitted for other classes).
- `export_before_suspend_matrix` — per-artifact-kind disposition
  drawn from the matrix above.
- `reopen_locally_afterward` — closed token list per rehearsal
  class.
- `inherited_contract_refs` — refs to the managed-workspace-lifecycle
  and account-seat-plan-and-exit contracts that govern the cell.
- `caveats` — reviewer-facing caveats; never a raw URL.

The artifact is reusable across shiproom and public-proof work
without rewording managed-continuity claims. It is the single source
of truth a public-proof or release-evidence packet quotes when
asserting "local-first" or "graceful degradation" for the rehearsal
class.

## Required rehearsal set

The packet requires that the following rehearsals exist in the
corpus. A rehearsal-set that omits any of these is non-conforming.

| Rehearsal id | Rehearsal class | Spec requirement |
|---|---|---|
| `managed_continuity.suspended_workspace_local_docs_and_review` | `managed_workspace_suspended` | "suspended workspace with local docs / review still available" |
| `managed_continuity.account_exit_with_export_path` | `account_exit` | "account exit with export path" |
| `managed_continuity.grace_period_downgrade` | `plan_grace_downgrade` | "grace-period downgrade" |
| `managed_continuity.local_only_fallback_after_service_outage` | `local_only_fallback_after_service_outage` | "local-only fallback after service outage" |
| `managed_continuity.seat_loss_local_baseline` | `seat_loss` | spec scenario class "seat loss" |
| `managed_continuity.policy_suspension_local_baseline` | `policy_suspension` | spec scenario class "policy suspension" |

## Seed corpus

The fixture corpus seeds the following case ids. Every case carries
one `managed_continuity_rehearsal_record` plus at least one
inherited-record ref.

| Case id | Rehearsal class | Notes |
|---|---|---|
| `managed_continuity.suspended_workspace_local_docs_and_review` | `managed_workspace_suspended` | Workspace lifecycle is `suspended`; local docs and review packets remain reachable on the local host while the managed instance is paused. |
| `managed_continuity.account_exit_with_export_path` | `account_exit` | Account is `offboarding_in_progress`; access-end window open; every artifact kind has a typed export disposition; self-hosted alternatives named. |
| `managed_continuity.grace_period_downgrade` | `plan_grace_downgrade` | Account is in `grace_warning` with a scheduled plan downgrade; `must_export_before_suspend` rows carry a deadline_at drawn from the grace close. |
| `managed_continuity.local_only_fallback_after_service_outage` | `local_only_fallback_after_service_outage` | Workspace lifecycle is `degraded` under `control_plane_failure`; full local-baseline admissible-surface set; managed actions blocked with a typed reason. |
| `managed_continuity.seat_loss_local_baseline` | `seat_loss` | Seat lifecycle is `deprovisioned`; `posture_origin_class = seat`; local-baseline admissible-surface set; reopen via `reopen_account_free_local_deployment` and `reopen_byok_local_ai_route`. |
| `managed_continuity.policy_suspension_local_baseline` | `policy_suspension` | Account `account_state_class = locked_for_review` or `posture_origin_class = policy`; managed actions are policy-suppressed; local-baseline admissible-surface set; reopen via `reopen_signed_file_based_policy` and `reopen_account_free_local_deployment`. |

## Surface admissibility

| Surface | May mint `managed_continuity_rehearsal_record` | May claim `export_before_suspend_disposition_class` | May claim `local_baseline_admissible_surface_class` | Projection rule |
|---|---|---|---|---|
| `about_panel_continuity_chip` | no | no | yes (quoted) | Quotes the rehearsal record; MUST NOT mint a parallel local-baseline claim. |
| `settings_managed_account_row` | no | no | yes (quoted) | Quotes the rehearsal record alongside the account-exit packet; MUST disable a managed toggle whose matching consequence is `revoked` or `paused_pending_admin_action`. |
| `workspace_lifecycle_chip` | no | no | yes (quoted) | Quotes the workspace lifecycle record id and the rehearsal record id; MAY NOT collapse them into one chip. |
| `support_export_row` | no | yes (quoted) | yes (quoted) | Preserves the rehearsal record under the support-export redaction envelope; MUST preserve export refs and deadline_at. |
| `release_evidence_packet` | no | yes (quoted) | yes (quoted) | MUST quote freshness class; a stale rehearsal record MAY NOT render as `authoritative_live`. |
| `shiproom_public_proof_packet` | no | yes (quoted) | yes (quoted) | MUST quote `local_baseline_proof.yaml` rows verbatim; MAY NOT reword the local-baseline claim per audience. |
| `offboarding_export_path` | no | yes (quoted) | yes (quoted) | Quotes the rehearsal record alongside the account-exit packet; MUST render the access-end window. |
| `rehearsal_corpus_authoring` | yes | yes | yes | Authoritative surface for minting rehearsals; MUST cite this packet, the schema-projected fields, and at least one inherited contract. |

Rule: any surface not named here MAY NOT mint a rehearsal record or
a local-baseline claim; it quotes one minted by the authoritative
surface above.

## UI and support rules

### Editor and About surfaces

- The About panel managed-continuity chip MUST quote the rehearsal
  record by id alongside the workspace lifecycle and account-exit
  records. It MAY collapse posture into one short sentence but MAY
  NOT drop the `rehearsal_class`, the `posture_origin_class`, or the
  local-baseline admissible-surface set.
- The settings managed-account row MUST cite the rehearsal record
  for any continuity assertion it offers. A toggle whose matching
  workflow consequence is `revoked`, `paused_pending_reauth`, or
  `blocked_policy_suppressed` MUST be disabled with a typed reason.

### Workspace and notebook surfaces

- A workspace surface running under a typed rehearsal MUST preserve
  the workspace lifecycle chip alongside the rehearsal chip; a
  notebook surface MUST preserve the kernel-boundary cue (kernels
  do not survive any rehearsal except `local_only_fallback_after_service_outage`
  via `regenerated_on_resume`).
- A workspace surface MAY NOT render local edit / save / search /
  Git / tasks / BYOK AI as blocked under any rehearsal. Blocking
  any local-baseline admissible surface during a rehearsal is
  non-conforming.

### AI surfaces

- AI route disclosure cards MUST quote the rehearsal record alongside
  the quota state and the workspace lifecycle record. Local / BYOK
  AI routes MUST stay admissible whenever the rehearsal admits
  `local_byok_ai`.
- A managed AI route MUST honor the bound account-exit packet's
  `provider_linked_workflow_consequence` for `managed_ai_route`.

### Review and support surfaces

- Support packets quote `managed_continuity_rehearsal_record_id`,
  not free-text "service unavailable" copy.
- Support copy MUST disclose the rehearsal class, the export-before-
  suspend matrix rows the user is being offered, and the local-
  baseline admissible-surface set.
- Support MUST NOT claim a managed surface is "available later"
  without a typed reopen path drawn from
  `reopen_locally_afterward`.

### Offboarding and shiproom surfaces

- Offboarding packets MUST quote the rehearsal record alongside the
  workspace lifecycle record, the account-exit packet, and the
  metering record.
- Shiproom and public-proof packets MUST cite
  `local_baseline_proof.yaml` directly. Rewording the local-baseline
  claim per audience is non-conforming.

## Forbidden patterns

The following are non-conforming:

- asserting "local-first" or "graceful degradation" for a managed
  scenario without quoting a `managed_continuity_rehearsal_record_id`;
- collapsing two `rehearsal_class` tokens into one chip;
- omitting `export_before_suspend_rows` on a rehearsal;
- omitting `local_baseline_admissible_surfaces` on a rehearsal;
- omitting `reopen_locally_afterward` on a rehearsal;
- rendering an `already_exported_local_artifact` claim without an
  opaque export ref;
- rendering a `must_export_before_suspend` row without a deadline
  drawn from the bound workspace lifecycle or account-exit record;
- narrowing the local-baseline admissible-surface set during a
  `local_only_fallback_after_service_outage` rehearsal;
- narrowing the surface set on any other rehearsal without a typed
  `narrowing_reason_class`;
- silently dropping queued provider drafts at suspend without a
  cited offline-capture continuation surface or a quoted provider-
  linked workflow consequence;
- rewording the local-baseline proof per audience instead of citing
  `local_baseline_proof.yaml` verbatim;
- using raw user emails, raw tenant names, raw provider account
  ids, raw billing ids, raw URLs, or raw display names in any
  rehearsal record.

## Evolution rules

- Adding a new `rehearsal_class`, `export_artifact_kind_class`,
  `export_before_suspend_disposition_class`,
  `local_baseline_admissible_surface_class`,
  `reopen_locally_afterward_class`, or `narrowing_reason_class` is
  additive-minor and requires a `schema_version` bump on this
  packet, on the local-baseline proof artifact, and at least one
  fixture.
- Repurposing an existing class is breaking and requires a new
  decision row in `artifacts/governance/decision_index.yaml` plus a
  migration note for support / export consumers.
- Any new managed-continuity surface MUST cite this packet, the
  local-baseline proof artifact, and at least one rehearsal fixture
  before claiming local-first or graceful-degradation behavior.
- The rehearsal record is independent of the workspace lifecycle
  record, the account-exit packet, the seat lifecycle row, and the
  quota state record. Surfaces that quote multiple records MUST
  preserve each record id and not collapse them into a single chip.

## Evidence joins

| `evidence_id` | Family / source kind | Why it is linked here | Freshness note | Artifact ref |
|---|---|---|---|---|
| `evidence.verification.managed_continuity_and_account_exit.continuity_account_exit_cases` | `verification_corpus` | Defines the rehearsal-case roster every continuity record cites. | current | `fixtures/managed/continuity_account_exit_cases/` |
| `evidence.verification.managed_continuity_and_account_exit.local_baseline_proof` | `verification_corpus` | Defines the per-rehearsal-class local-baseline proof artifact every continuity surface quotes. | current | `artifacts/managed/local_baseline_proof.yaml` |
| `evidence.managed.workspace_lifecycle_cases` | `source_anchor` | Inherited workspace-lifecycle fixtures every continuity record cites. | current | `fixtures/managed/workspace_lifecycle_cases/` |
| `evidence.managed.account_exit_cases` | `source_anchor` | Inherited account-exit fixtures every continuity record cites. | current | `fixtures/managed/account_exit_cases/` |
| `evidence.managed.managed_workspace_lifecycle_contract` | `source_anchor` | Canonical managed-workspace-lifecycle vocabulary this packet projects. | current | `docs/managed/managed_workspace_lifecycle_contract.md` |
| `evidence.managed.account_seat_plan_and_exit_contract` | `source_anchor` | Canonical account / seat / plan / grace / exit vocabulary this packet projects. | current | `docs/managed/account_seat_plan_and_exit_contract.md` |

## Verification method

- **Verification classes used:** design review, vocabulary-reuse
  review, fixture review, schema-alignment review.
- **Procedure summary:** verified that the packet, the rehearsal
  fixture corpus, and the local-baseline proof artifact reuse the
  managed-workspace-lifecycle `lifecycle_phase_class`,
  `persistence_class`, `continuation_posture`,
  `local_only_admissible_surface_class`, and `retry_outcome_class`
  vocabularies; the account-seat-plan-and-exit `account_state_class`,
  `org_state_class`, `seat_state_class`, `plan_state_class`,
  `grace_state_class`, `posture_origin_class`,
  `exit_artifact_class`, `exit_artifact_disposition_class`,
  `self_hosted_alternative_class`, and
  `provider_linked_workflow_consequence_class` vocabularies; and the
  governance portability-row vocabulary without minting parallel
  tokens. Verified that every required rehearsal named in the spec
  is present in the corpus, every rehearsal carries an export-before-
  suspend matrix row per applicable artifact kind, every rehearsal
  carries a local-baseline admissible-surface set, every rehearsal
  carries a reopen-locally-afterward set, and the local-baseline
  proof artifact carries one row per `rehearsal_class`.
- **Automation refs:** `not_yet_seeded` for a dedicated continuity-
  corpus validator; structural parsing is the available automation.
  The fixture corpus is structurally parseable as YAML.

## Known gaps and waivers

- **Waiver refs:** `none`.
- **Known-limit refs:** `none`.
- **Migration-packet refs:** `none`.
- **Explicit gaps:** no managed-workspace control plane, account-
  identity broker, offboarding executor, or live rehearsal-automation
  harness is wired to this packet yet.
- **Explicit gaps:** no dedicated JSON Schema exists yet for the
  `managed_continuity_rehearsal_record`, the `rehearsal_class`
  enum, the `export_artifact_kind_class` enum, the
  `export_before_suspend_disposition_class` enum, the
  `local_baseline_admissible_surface_class` enum, the
  `reopen_locally_afterward_class` enum, or the
  `narrowing_reason_class` enum. Reserved shapes are documented here
  for later schema landing alongside the existing
  `workspace_lifecycle_state.schema.json` and
  `account_exit_packet.schema.json` schemas.
- **Explicit gaps:** the rehearsal-automation harness, the public-
  proof packet generator, and the shiproom citation pipeline are
  out of scope here; they will land with later packets that quote
  this seed.

## Reviewer signoff

- **Reviewer / forum:** `@ahmeddyounis`
- **Decision:** `needs_follow_up`
- **Date:** `2026-05-04`
- **Reviewed claim rows:**
  `packet_row:managed_continuity_and_account_exit.rehearsal_record_contract`,
  `packet_row:managed_continuity_and_account_exit.export_before_suspend_matrix`,
  `packet_row:managed_continuity_and_account_exit.local_baseline_proof_artifact`,
  `packet_row:managed_continuity_and_account_exit.required_rehearsal_set`,
  `packet_row:managed_continuity_and_account_exit.seed_corpus`
- **Blocking refs:** `none`

## Refresh trigger

- **Named rerun trigger:** `managed_continuity_or_account_exit_vocabulary_revision_changed`.
- **Expected freshness window:** `P30D`.
- **Next packet family to update with the same evidence ids:**
  shiproom / public-proof packet, support-export packet, release-
  evidence packet, or admin-reconciliation packet that starts
  quoting rehearsal records, export-before-suspend matrix rows, or
  local-baseline admissible-surface tokens.
