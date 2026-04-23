# Target-discovery confidence, host-boundary, wrong-target reapproval, and managed-workspace lifecycle verification seed

This packet freezes one shared verification story for target and route
truth: how a launch-capable surface names which target it found, how
confidently, which host / isolation boundary a projected fact crossed,
whether a wrong-target detection requires renewed approval, and which
managed-workspace lifecycle state a surface is quoting. It exists so
future shell, notebook, AI, support-export, and release surfaces reuse
one reviewable object model instead of minting per-surface target
language, silent wrong-target swaps, or ambiguous suspend/resume copy.

If this packet, the
[`target_confidence_manifest.yaml`](../../fixtures/remote/target_confidence_manifest.yaml)
corpus, the
[`host_boundary_matrix.yaml`](../../artifacts/remote/host_boundary_matrix.yaml)
matrix, and the frozen runtime taxonomies disagree, the machine-readable
matrix and the canonical taxonomies win for tooling and this packet
must update in the same change.

Companion artifacts:

- [`/fixtures/remote/target_confidence_manifest.yaml`](../../fixtures/remote/target_confidence_manifest.yaml)
  — machine-readable case roster covering local exact targets,
  imported/cached target hints, remote helper attach, managed
  workspace reconnect, stale target metadata, and wrong-target
  detection requiring renewed approval.
- [`/artifacts/remote/host_boundary_matrix.yaml`](../../artifacts/remote/host_boundary_matrix.yaml)
  — machine-readable matrix binding route class, target class,
  host-boundary cue stack, managed-workspace lifecycle state,
  reachability, adapter-confidence placeholder, reapproval class,
  and export posture to what shell, support, export, and future
  AI / execution surfaces may claim.
- [`/fixtures/remote/reapproval_and_suspend_resume_cases/`](../../fixtures/remote/reapproval_and_suspend_resume_cases/)
  — reviewer-facing cases exercising suspend, resume, reconnect,
  rebuild, expired, and local-only continuation states plus
  wrong-target reapproval and adapter-confidence projections.
- [`/docs/runtime/target_discovery_and_install_review_taxonomy.md`](../runtime/target_discovery_and_install_review_taxonomy.md)
  — canonical target-discovery confidence, host-boundary cue, and
  managed-workspace lifecycle vocabularies this packet reuses.
- [`/docs/runtime/origin_target_route_taxonomy.md`](../runtime/origin_target_route_taxonomy.md)
  — canonical origin, target, route, exposure, route-change, and
  authority-linkage vocabularies this packet reuses.
- [`/docs/runtime/execution_context_vocabulary.md`](../runtime/execution_context_vocabulary.md)
  — canonical cross-surface execution-context fields every
  target-aware surface quotes.
- [`/artifacts/runtime/managed_workspace_lifecycle.yaml`](../../artifacts/runtime/managed_workspace_lifecycle.yaml)
  — canonical lifecycle-state matrix, transition reasons, and
  activation-budget slice vocabulary this packet cites without
  re-deriving.
- [`/artifacts/runtime/action_origin_target_labels.yaml`](../../artifacts/runtime/action_origin_target_labels.yaml)
  — canonical route-truth matrix this packet reuses for wrong-target
  detection, authority linkage, and exposure class.
- [`/schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json)
  — canonical machine-readable execution-context fields every
  target-discovery packet quotes.

Normative sources projected here:

- `.t2/docs/Aureline_PRD.md`
  — requirement register and evidence-governance posture; explicit
  live-environment targeting and non-replayable runtime approvals.
- `.t2/docs/Aureline_Technical_Architecture_Document.md`
  — `TOOL-CTX-002`, `TOOL-ENV-006`, `TOOL-INFRA-002`, `TOOL-EXEC-007`,
  `SEC-AUTHZ-011`, `SEC-CRED-009`, `SEC-TRUST-001`, `SEC-NET-005`,
  `ARCH-COMP-005`, `ARCH-STATE-012`. Appendix CM (wrong-target drill
  corpus, context-badge parity suite) and Appendix CF (approval
  ticket replay/expiry tests) are directly projected here.
- `.t2/docs/Aureline_Technical_Design_Document.md`
  — execution-context provenance, managed-workspace lifecycle,
  remote-agent attach, compatibility skew rules.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md`
  — target and boundary chip rules, wrong-target correction
  disclosure, suspend/resume surface copy discipline.
- `.t2/docs/Aureline_Milestones_Document.md`
  — target truth, wrong-target reapproval, and managed-workspace
  lifecycle as release-blocking posture during the foundations phase.

## Shared header

```yaml
schema_version: 1
header_kind: evidence_packet_header
packet_family: verification_packet
packet_id: verification.target_and_host_boundary.seed
evidence_id: evidence.verification.target_and_host_boundary.packet
title: Target-discovery confidence, host-boundary, wrong-target reapproval, and managed-workspace lifecycle verification seed
ownership:
  owner_dri: "@ahmeddyounis"
  evidence_owner: "@ahmeddyounis"
  backup_owner: null
  backup_waiver: single-maintainer-backup
coverage:
  requirement_ids:
    - TOOL-CTX-002
    - TOOL-ENV-006
    - TOOL-INFRA-002
    - TOOL-EXEC-007
    - SEC-AUTHZ-011
    - SEC-CRED-009
    - SEC-TRUST-001
    - SEC-NET-005
    - ARCH-COMP-005
    - ARCH-STATE-012
  claim_row_refs:
    - packet_row:target_and_host_boundary.target_confidence_contract
    - packet_row:target_and_host_boundary.host_boundary_cue_stack
    - packet_row:target_and_host_boundary.route_class_projection
    - packet_row:target_and_host_boundary.wrong_target_reapproval
    - packet_row:target_and_host_boundary.managed_workspace_lifecycle
    - packet_row:target_and_host_boundary.adapter_confidence_placeholder
    - packet_row:target_and_host_boundary.reviewer_label_projection
    - packet_row:target_and_host_boundary.seed_corpus
  covered_lanes:
    - release_evidence
    - support_export
    - governance_packets
    - docs_public_truth
result_status: seed_only
visibility_class: internal
freshness:
  captured_at: 2026-04-23T00:00:00Z
  stale_after: P30D
  freshness_class: warm_cached
  source_revision: target_and_host_boundary_seed@1
  trigger_revision: target_and_host_boundary_packet@1
environment:
  channel_context: not_applicable
  deployment_context:
    - not_applicable
  environment_summary: >
    Seed packet over the frozen target-discovery, host-boundary,
    managed-workspace lifecycle, and origin/target/route vocabularies.
    No live resolver, remote-agent broker, or managed control plane is
    wired to this packet yet. Claims are structural: every case in the
    manifest, matrix row, and suspend/resume case reuses the existing
    frozen tokens rather than minting new per-surface language.
artifact_links:
  supporting_evidence_ids:
    - evidence.verification.target_confidence_manifest
    - evidence.verification.host_boundary_matrix
    - evidence.verification.reapproval_and_suspend_resume_cases
    - evidence.runtime.target_discovery_taxonomy
    - evidence.runtime.managed_workspace_lifecycle_matrix
    - evidence.runtime.action_origin_target_labels
  exact_build_identity_refs: []
  fixture_refs:
    - fixtures/remote/target_confidence_manifest.yaml
    - fixtures/remote/reapproval_and_suspend_resume_cases/
    - fixtures/runtime/route_taxonomy_examples/wrong_target_detected_managed_workspace_drift.yaml
    - fixtures/runtime/execution_context_examples/
    - fixtures/workspace/entry_restore_examples/resume_managed_workspace.json
    - fixtures/support/object_handoff_examples/trust_warning_remote_boundary.json
    - fixtures/support/object_handoff_examples/command_detail_wrong_target.json
  archetype_refs: []
  source_anchor_refs:
    - docs/runtime/target_discovery_and_install_review_taxonomy.md
    - docs/runtime/origin_target_route_taxonomy.md
    - docs/runtime/execution_context_vocabulary.md
    - artifacts/runtime/managed_workspace_lifecycle.yaml
    - artifacts/runtime/action_origin_target_labels.yaml
    - artifacts/runtime/execution_scope_matrix.yaml
    - schemas/runtime/execution_context.schema.json
    - docs/adr/0009-execution-context-and-scope.md
    - docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md
    - docs/adr/0011-capability-lifecycle-and-dependency-markers.md
  waiver_refs: []
  known_limit_refs: []
  migration_packet_refs: []
```

## Summary

This seed packet freezes:

- one reviewer-facing `target_truth_record` object that names the
  discovered target, its confidence class, its host-boundary cue
  stack, its route class, its reachability, and its
  wrong-target-correction posture;
- one explicit mapping from reviewer-facing lifecycle labels
  (`warming`, `ready`, `degraded`, `paused`, `suspended`, `expired`,
  `local_only_continuation`) onto the frozen
  `managed_workspace_lifecycle_state` vocabulary;
- one closed `wrong_target_correction_class` and
  `reapproval_requirement_class` so wrong-target detection cannot
  render as a silent swap or a generic "changed target" chip;
- one `adapter_confidence_placeholder` so remote-agent, helper, and
  bridged-host adapters can project comparable confidence evidence
  alongside the canonical `target_discovery_confidence_class`;
- one corpus of machine-readable cases covering local exact, imported
  or cached hints, remote helper attach, managed workspace reconnect,
  stale target metadata, wrong-target reapproval, suspend/resume,
  rebuild, expired session, and local-only continuation.

It does not claim a live execution resolver, a remote-agent broker,
or a managed control plane is wired up. It claims only that packet
data exists in one reviewable form and reuses the frozen runtime
vocabularies already landed in this repository.

## Claim coverage

| Packet row | Requirement id(s) | Status | Visibility | Supporting evidence ids | Notes |
|---|---|---|---|---|---|
| `packet_row:target_and_host_boundary.target_confidence_contract` | `TOOL-CTX-002`, `TOOL-ENV-006` | `seed_only` | `internal` | `evidence.verification.target_confidence_manifest` | Freezes one machine-readable target-truth object per discovered target. |
| `packet_row:target_and_host_boundary.host_boundary_cue_stack` | `TOOL-EXEC-007`, `SEC-TRUST-001`, `SEC-NET-005` | `seed_only` | `internal` | `evidence.verification.host_boundary_matrix` | Host-boundary cue stack is ordered outermost-to-innermost and pairs with an authority envelope tag. |
| `packet_row:target_and_host_boundary.route_class_projection` | `TOOL-INFRA-002`, `ARCH-STATE-012` | `seed_only` | `internal` | `evidence.runtime.action_origin_target_labels` | Route class, exposure class, and authority linkage stay typed rather than rendered as ambient copy. |
| `packet_row:target_and_host_boundary.wrong_target_reapproval` | `TOOL-INFRA-002`, `SEC-AUTHZ-011`, `SEC-CRED-009` | `seed_only` | `internal` | `evidence.verification.reapproval_and_suspend_resume_cases` | Wrong-target detection requires one typed correction class and one reapproval requirement class; silent target swaps are non-conforming. |
| `packet_row:target_and_host_boundary.managed_workspace_lifecycle` | `TOOL-CTX-002`, `TOOL-ENV-006`, `ARCH-COMP-005` | `seed_only` | `internal` | `evidence.runtime.managed_workspace_lifecycle_matrix` | Reviewer-facing warming, ready, degraded, paused, suspended, expired, and local-only continuation labels project onto the frozen lifecycle state vocabulary. |
| `packet_row:target_and_host_boundary.adapter_confidence_placeholder` | `TOOL-CTX-002`, `ARCH-COMP-005` | `seed_only` | `internal` | `evidence.verification.target_confidence_manifest` | Adapter-confidence placeholder lets remote-agent, helper, and bridged-host adapters project comparable confidence evidence without minting parallel confidence vocabularies. |
| `packet_row:target_and_host_boundary.reviewer_label_projection` | `ARCH-STATE-012`, `TOOL-CTX-002` | `seed_only` | `internal` | `evidence.verification.host_boundary_matrix` | One reviewer-label projection keeps shell, notebook, AI, and support surface copy comparable. |
| `packet_row:target_and_host_boundary.seed_corpus` | `TOOL-CTX-002`, `SEC-AUTHZ-011` | `seed_only` | `internal` | `evidence.verification.target_confidence_manifest`, `evidence.verification.reapproval_and_suspend_resume_cases` | One stable case-id set now covers the required confidence, wrong-target, and lifecycle scenarios. |

## What this seed freezes

- One `target_truth_record` shape every launch-capable surface
  (task, test, debug, terminal, notebook-kernel, scaffolding, AI
  tool-call, doctor-repair, import-probe, replay-probe) reuses for
  projected target and route language.
- One reviewer-facing lifecycle-label set that maps onto the frozen
  `managed_workspace_lifecycle_state` vocabulary without minting
  parallel per-surface labels.
- One closed wrong-target correction and reapproval story so
  wrong-target detection names *why* the target changed, *what* the
  prior target was, and *what authority* admitted the corrected
  target.
- One adapter-confidence placeholder so remote helpers and bridged
  adapters project comparable confidence alongside the resolver's
  canonical target-discovery confidence.
- One seed corpus of cases the support-export, release, and docs
  surfaces can cite by id rather than re-deriving prose.

## Target truth record

Every case in the machine-readable manifest resolves to one
`target_truth_record` with these required fields:

- `case_id`
- `invocation_session_id`
- `execution_context_id`
- `target_class` — the ADR-0009 execution target class.
- `action_target_class` — the origin/target/route vocabulary token.
- `action_route_class`
- `action_exposure_class`
- `action_origin_class`
- `host_boundary_cue_stack` — ordered outermost-to-innermost list
  of `host_boundary_cue_class` tokens.
- `target_discovery_confidence_class` — from the frozen
  `target_discovery_and_install_review_taxonomy.md` vocabulary.
- `divergence_or_inference_reasons` — typed list when confidence is
  `probed_divergent`, `inferred_from_ambient`, or
  `unresolved_requires_user`; `null` on
  `canonical_declared` / `canonical_materialised`.
- `adapter_confidence_placeholder` — optional object described
  below; carries adapter-side confidence evidence for remote helpers
  and bridged adapters. `null` when the target is local.
- `reachability_state` — `reachable`, `warming`, `degraded`,
  `unreachable`, or `disabled_by_policy`.
- `managed_workspace_lifecycle_state` — from the frozen
  `managed_workspace_lifecycle_state` vocabulary when the target is
  a managed workspace; `null` otherwise.
- `managed_workspace_reviewer_label` — reviewer-facing projection
  of the lifecycle state (see table below); `null` when the target
  is not a managed workspace.
- `activation_budget_summary_ref` — required when the target is a
  managed workspace in any state other than `undeclared`.
- `wrong_target_correction_class` — one of
  `no_correction_needed`,
  `corrected_before_commit`,
  `corrected_after_partial_effect`,
  `requires_user_confirmation`,
  `blocked_pending_reapproval`.
- `reapproval_requirement_class` — one of `no_reapproval_required`,
  `session_ticket_refresh_required`,
  `approval_ticket_reissue_required`,
  `admin_confirmation_required`,
  `policy_narrowing_required`,
  `trust_reevaluation_required`.
- `prior_target_ref` — present when
  `wrong_target_correction_class` is any value other than
  `no_correction_needed`.
- `prior_route_class` — same trigger.
- `route_change_reason_code` — from the frozen
  `route_change_reason_code_vocabulary`.
- `authority_linkage_class` — from the frozen
  `authority_linkage_class_vocabulary`.
- `target_evidence_refs` — ordered list of backing evidence
  (execution-context record, route-truth packet, approval ticket,
  lifecycle audit events, provenance records).
- `freshness_class` — ADR-0011 token.
- `redaction_class` — ADR-0007 token.
- `export_inclusion_posture` — `metadata_safe_default`,
  `operator_only_restricted`, or `broadened_capture_opt_in`.

Rule: a `target_truth_record` with `target_class = managed_workspace`
MUST carry a `managed_workspace_lifecycle_state`,
`managed_workspace_reviewer_label`, and
`activation_budget_summary_ref`. A record that is missing any of the
three is non-conforming.

Rule: a record whose `target_discovery_confidence_class` is
`resolver_unavailable` MUST set
`wrong_target_correction_class = blocked_pending_reapproval` and
`reapproval_requirement_class = trust_reevaluation_required` unless
the record is a read-only inspection probe.

## Host-boundary cue stack

The cue stack reuses the frozen `host_boundary_cue_class` tokens
(see [`target_discovery_and_install_review_taxonomy.md`](../runtime/target_discovery_and_install_review_taxonomy.md),
Vocabulary 2) and layers them outermost-to-innermost. The stack is
not a free list: every entry MUST come from the frozen vocabulary.

| Position | Meaning | Example |
|---|---|---|
| outer | boundary nearest the rendering surface | `local_host_boundary` when the editor renders locally |
| intermediate (optional, repeatable) | boundary the fact crossed on the way in | `managed_workspace_boundary`, `remote_ssh_boundary` |
| inner | boundary that owns the projected fact | `ai_sandbox_boundary`, `notebook_kernel_boundary`, `bridged_host_boundary` |

Rules:

1. Every projected target-truth record MUST carry at least one cue.
   A local fact uses `local_host_boundary` alone.
2. A cue other than `local_host_boundary` MUST pair with the ADR-0005
   authority envelope tag `projected_from_execution` or
   `projected_from_provider_overlay` and an ADR-0011
   `freshness_class`.
3. A record whose cue stack contains `remote_agent_boundary`,
   `managed_workspace_boundary`, `browser_handoff_return_boundary`,
   or `bridged_host_boundary` MUST NOT render as
   `freshness_class = authoritative_live` unless the canonical
   owner of that boundary has been contacted since the stale-after
   window on the referenced evidence.
4. The stack is ordered. Rendering a cue chip that collapses two
   distinct boundaries into one token is non-conforming.

## Route class and exposure projection

Every target-truth record quotes one
`action_route_class` and one `action_exposure_class` from the frozen
matrix in
[`action_origin_target_labels.yaml`](../../artifacts/runtime/action_origin_target_labels.yaml).
The packet reuses — it does not extend — the existing route-truth
vocabulary.

Rules:

1. A record whose `action_target_class = managed_workspace_target`
   admits `managed_control_plane_route`,
   `remote_agent_attach_route`, `approval_gated_route`, and the
   local/remote-rpc routes when the managed workspace projects
   through another boundary. It MUST NOT claim `in_process_route`.
2. A record whose cue stack contains
   `browser_handoff_return_boundary` MUST also carry
   `authority_linkage_class = browser_handoff_packet_linked` or
   `authority_linkage_class = approval_ticket_linked`.
3. A record whose `action_exposure_class` is
   `tunnel_exposed_public` or `cross_tenant_visible` MUST set
   `export_inclusion_posture = operator_only_restricted` unless the
   user has opted into broadened capture.

## Wrong-target correction and reapproval

Wrong-target detection MUST render through one `target_truth_record`
with a typed correction class and a typed reapproval requirement
class. The packet forbids silent swaps.

### `wrong_target_correction_class` (frozen)

| Token | Meaning | Required rendering |
|---|---|---|
| `no_correction_needed` | Resolver stayed on the originally-resolved target; no correction event. | No correction chip required. |
| `corrected_before_commit` | Resolver detected the wrong target before any externally-visible effect; route packet minted against the corrected target. | Correction disclosure MUST render before commit; prior target MUST be preserved as `prior_target_ref`. |
| `corrected_after_partial_effect` | Resolver detected the wrong target after a partial effect landed; journal entry and correction disclosure are mandatory. | Correction disclosure MUST render inline and cite the mutation-journal entry; reviewer may not proceed without acknowledgement. |
| `requires_user_confirmation` | Resolver discovered more than one plausible target and is asking the user to pick; no commit proceeds until confirmation. | Prompt MUST name every candidate target by id; no silent default. |
| `blocked_pending_reapproval` | Resolver detected the wrong target and further action is blocked until a typed reapproval completes. | Surface MUST render the reapproval requirement chip; continue action is denied until the reapproval lands. |

### `reapproval_requirement_class` (frozen)

| Token | Meaning | Typical trigger |
|---|---|---|
| `no_reapproval_required` | The correction did not cross an authority boundary; no ticket minting. | Wrong target corrected within the same managed binding and same policy epoch. |
| `session_ticket_refresh_required` | Managed-session ticket or SSH ticket expired or rotated; refresh suffices. | Resume on a managed workspace whose session ticket is expired. |
| `approval_ticket_reissue_required` | ADR-0010 approval ticket is no longer valid against the corrected target (policy epoch rolled, actor changed, target changed). | Wrong-target detection on a connected-provider publish flow. |
| `admin_confirmation_required` | Admin-confirmation policy narrows the corrected target's permitted effects. | Managed workspace entering a policy-narrowed state after a provider-side migration. |
| `policy_narrowing_required` | Policy pack narrows allowed targets; user must pick inside the narrowed set. | Policy-pack install narrows remote helper targets mid-session. |
| `trust_reevaluation_required` | Workspace trust state must be re-evaluated before any further action. | Imported target from an untrusted workspace becomes the active target. |

Rules:

1. A record with `wrong_target_correction_class` other than
   `no_correction_needed` MUST carry `prior_target_ref`,
   `prior_route_class`, and a `route_change_reason_code` that is not
   `canonical_no_route_change`.
2. `blocked_pending_reapproval` MUST pair with a non-null
   `reapproval_requirement_class` and a non-null repair-hook ref.
3. A wrong-target correction MUST preserve, not drop, the prior
   approval ticket, browser-handoff packet, or managed-control-plane
   token in the `target_evidence_refs` list. Dropping the prior
   authority evidence is non-conforming.
4. A record whose correction class is `corrected_after_partial_effect`
   MUST cite a mutation-journal entry and MUST NOT carry
   `export_inclusion_posture = metadata_safe_default`; operator-only
   restricted export is the minimum.

## Managed-workspace lifecycle projection

The packet does not mint new lifecycle states. It maps reviewer-facing
lifecycle labels that shell, notebook, AI, and support surfaces reuse
to the frozen `managed_workspace_lifecycle_state` vocabulary in
[`managed_workspace_lifecycle.yaml`](../../artifacts/runtime/managed_workspace_lifecycle.yaml).

### Reviewer-label projection (frozen)

| `managed_workspace_reviewer_label` | Projects onto `managed_workspace_lifecycle_state` | Reachability | Reviewer meaning |
|---|---|---|---|
| `warming` | `provisioning`, `warming` | `warming` | Capsule / prebuild warmers are running; launches deny or queue. |
| `ready` | `ready` | `reachable` | Instance is reachable, capsule in-sync, activators applied. Normal launch target. |
| `degraded` | `recovering` (optionally overlaid on `ready`) | `degraded` | Control plane detected a drift; surface may project `degraded` while recovery is in flight. |
| `paused` | `snapshot_paused` | `unreachable` | User or admin paused the instance with a snapshot; resume restores from the snapshot. |
| `suspended` | `idle_suspended` | `unreachable` | Instance auto-paused after an idle window; filesystem preserved. |
| `expired` | `hibernated`, `retiring`, or `retired` (or a live session whose managed-session ticket has expired and requires reauth) | `unreachable` or `reachable_pending_reauth` | Long-idle snapshot in cold storage; or session ticket has expired and reauth is required; or instance retired. Distinguished by the `expiry_reason_class` below. |
| `local_only_continuation` | `idle_suspended`, `recovering`, `quarantined`, or lifecycle `null` (no managed binding) | `unreachable` | Remote or managed target is unreachable; the editor continues in a reduced-scope local-only mode with the narrowed authority envelope. |

### `expiry_reason_class` (frozen)

Used only when `managed_workspace_reviewer_label = expired`:

- `session_ticket_expired`
- `hibernation_window_elapsed`
- `retirement_drain_window_completed`
- `policy_epoch_rolled`
- `kill_switch_tripped`
- `successor_image_available`

### `local_only_continuation_reason_class` (frozen)

Used only when
`managed_workspace_reviewer_label = local_only_continuation`:

- `route_dependency_unreachable`
- `managed_control_plane_unreachable`
- `remote_agent_attach_unreachable`
- `browser_handoff_return_unavailable`
- `user_requested_local_fallback`
- `admin_requested_local_fallback`

Rules:

1. A reviewer label MAY NOT claim `ready` unless the underlying
   lifecycle state is `ready` and the reachability is `reachable`.
   Projection of `ready` over `recovering`, `warming`, or
   `idle_suspended` is non-conforming.
2. A reviewer label of `expired` MUST carry an `expiry_reason_class`;
   `null` is non-conforming.
3. A reviewer label of `local_only_continuation` MUST carry a
   `local_only_continuation_reason_class` and MUST NOT render the
   managed-workspace cue as the primary boundary; the primary cue is
   `local_host_boundary` with the managed-workspace cue preserved
   only as an outer / stacked boundary (record that the editor still
   remembers the managed target).
4. A reviewer label of `degraded` MAY overlay `ready` only when the
   underlying lifecycle state is `recovering`; overlaying `degraded`
   on `paused`, `suspended`, or `expired` is non-conforming.
5. Surfaces MAY NOT collapse two distinct reviewer labels into one
   chip (e.g., render `suspended` and `expired` as "inactive").

## Adapter-confidence placeholder

Remote-agent, bridged-helper, and notebook-kernel adapters may
project confidence evidence that rides alongside the canonical
`target_discovery_confidence_class`. The placeholder exists so later
adapter lanes reuse one reviewable object model instead of minting
parallel confidence vocabularies.

### `adapter_confidence_placeholder` fields

- `adapter_kind` — one of `remote_agent_attach`,
  `remote_ssh_adapter`, `managed_workspace_adapter`,
  `notebook_kernel_adapter`, `bridged_helper_adapter`,
  `ai_sandbox_adapter`.
- `adapter_confidence_class` — one of
  `adapter_authoritative_match`,
  `adapter_probed_consistent`,
  `adapter_probed_divergent`,
  `adapter_inferred_from_session`,
  `adapter_unreachable`.
- `adapter_divergence_or_inference_reasons` — typed list when the
  class is `adapter_probed_divergent` or
  `adapter_inferred_from_session`.
- `adapter_authority_envelope_tag` — ADR-0005 tag.
- `adapter_freshness_class` — ADR-0011 token.
- `adapter_evidence_refs` — ordered list of backing evidence.

Rules:

1. A record whose `target_class` is `remote_agent`, `remote_ssh`,
   `managed_workspace`, or `notebook_kernel_remote` and whose
   confidence is `probed_consistent` or `canonical_materialised`
   SHOULD carry an `adapter_confidence_placeholder` so later lanes
   keep adapter-side confidence comparable.
2. `adapter_confidence_class = adapter_unreachable` MUST force the
   surrounding record to `reachability_state = unreachable` and
   MUST set `wrong_target_correction_class` to
   `blocked_pending_reapproval` or `requires_user_confirmation`.
3. Adapter confidence NEVER widens the target-discovery confidence
   class. The weaker signal wins.

## Host-boundary matrix cross-walk

The [`host_boundary_matrix.yaml`](../../artifacts/remote/host_boundary_matrix.yaml)
binds every admissible combination of route class, target class,
host-boundary cue stack, and managed-workspace lifecycle state to
what the shell, support packet, export flow, and future AI /
execution surfaces MAY claim. The matrix is authoritative for
admissibility; this packet only states the contract:

- Each row names an admissible combination and the minimum-field set
  any surface MUST populate when projecting that combination.
- Each row names which surfaces MAY claim the row: one or more of
  `shell_command_router`, `support_export`, `object_handoff_packet`,
  `release_evidence_packet`, `ai_tool_call_plane`, `notebook_kernel`,
  and `doctor_repair`.
- Each row names the required redaction class and default export
  posture.
- Each row names at least one conformance test.

Rule: a surface projecting target / route language that is not in
the matrix is non-conforming until the matrix adds the combination.
Extending the matrix is additive-minor.

## Seed corpus

The machine-readable manifest seeds the following case ids. Every
case carries one `target_truth_record` and at least one
conformance-test ref.

| Case id | Target class | Confidence | Reviewer label | Correction class | Reapproval class | Notes |
|---|---|---|---|---|---|---|
| `target.local.canonical_declared.exact_match` | `local_host_target` | `canonical_declared` | n/a | `no_correction_needed` | `no_reapproval_required` | Local exact target pinned by capsule + lockfile. |
| `target.imported.cached_hint.stale_metadata_detected` | `local_host_target` | `inferred_from_ambient` | n/a | `requires_user_confirmation` | `trust_reevaluation_required` | Imported/cached target metadata is stale; user must confirm. |
| `target.remote_helper.attach.probed_consistent` | `remote_agent_target` | `probed_consistent` | n/a | `no_correction_needed` | `no_reapproval_required` | Remote helper attach; adapter reports consistent probe. |
| `target.managed_workspace.reconnect.session_ticket_expired` | `managed_workspace_target` | `canonical_materialised` | `expired` | `blocked_pending_reapproval` | `session_ticket_refresh_required` | Managed reconnect blocked until session ticket is refreshed. |
| `target.stale.metadata_rejected.resolver_reports_divergence` | `local_host_target` | `probed_divergent` | n/a | `requires_user_confirmation` | `trust_reevaluation_required` | Stale target metadata; resolver rejects as divergent. |
| `target.wrong_target.managed_workspace_drift.reapproval_required` | `managed_workspace_target` | `canonical_materialised` | `degraded` | `corrected_before_commit` | `approval_ticket_reissue_required` | Wrong-target detection on managed workspace drift; approval ticket reissue required. |

The [reapproval and suspend/resume cases](../../fixtures/remote/reapproval_and_suspend_resume_cases/)
seed the following case ids:

| Case id | Reviewer label | Lifecycle state | Notes |
|---|---|---|---|
| `reapproval.managed_workspace.suspend_idle_window` | `suspended` | `idle_suspended` | Idle-window auto-suspend with preserved filesystem. |
| `reapproval.managed_workspace.resume_from_idle_suspended` | `ready` | `ready` (from `idle_suspended`) | Resume from idle; ticket refresh required. |
| `reapproval.managed_workspace.reconnect_after_control_plane_failover` | `degraded` | `recovering` | Control-plane failover; reachability `degraded` while recovery runs. |
| `reapproval.managed_workspace.rebuild_after_successor_image` | `warming` | `provisioning` (from `retiring`) | Successor image available; rebuild required. |
| `reapproval.managed_workspace.expired_hibernation_window_elapsed` | `expired` | `hibernated` | Long-idle snapshot; expiry reason `hibernation_window_elapsed`. |
| `reapproval.local_only_continuation.remote_agent_unreachable` | `local_only_continuation` | `null` (no managed binding active) | Editor continues locally with reduced scope while remote agent is unreachable. |

## Surface admissibility

| Surface | May claim `target_truth_record` | May claim wrong-target correction | May claim managed-workspace reviewer label | Projection rule |
|---|---|---|---|---|
| `shell_command_router` | yes | yes | yes | MUST quote the record by packet id; no re-derivation. |
| `object_handoff_packet` (support) | yes | yes | yes | MUST preserve prior target refs and reapproval requirement class when packaging. |
| `support_export` | yes | yes | yes | MUST carry `operator_only_restricted` posture when cue stack contains a non-local boundary. |
| `release_evidence_packet` | yes | yes | yes | MUST quote freshness class; stale records may not render as `authoritative_live`. |
| `ai_tool_call_plane` | yes | yes | yes | MUST render cue stack inline and never widen confidence. |
| `notebook_kernel` | yes (local + remote kernels) | yes | yes (when kernel runs inside a managed workspace) | MUST preserve notebook-kernel cue in the stack; MAY NOT collapse notebook boundary into the managed-workspace cue. |
| `doctor_repair` | yes (read-only probes) | yes (proposes correction) | yes | Read-only probes MAY carry `target_discovery_confidence_class = resolver_unavailable` without blocking if the probe is an inspection, not a commit. |

Rule: any surface not named here MUST NOT claim a target-truth
record; it quotes one minted by one of the surfaces above.

## Evidence joins

| `evidence_id` | Family / source kind | Why it is linked here | Freshness note | Artifact ref |
|---|---|---|---|---|
| `evidence.verification.target_confidence_manifest` | `verification_corpus` | Defines the case roster every target-truth record cites. | current | `fixtures/remote/target_confidence_manifest.yaml` |
| `evidence.verification.host_boundary_matrix` | `verification_matrix` | Defines admissible route / target / boundary / lifecycle combinations and surface-admissibility rules. | current | `artifacts/remote/host_boundary_matrix.yaml` |
| `evidence.verification.reapproval_and_suspend_resume_cases` | `verification_corpus` | Supplies reviewer-facing suspend / resume / reconnect / rebuild / expired / local-only-continuation cases. | current | `fixtures/remote/reapproval_and_suspend_resume_cases/` |
| `evidence.runtime.target_discovery_taxonomy` | `source_anchor` | Canonical target-discovery confidence, host-boundary cue, and managed-workspace lifecycle vocabularies. | current | `docs/runtime/target_discovery_and_install_review_taxonomy.md` |
| `evidence.runtime.managed_workspace_lifecycle_matrix` | `source_anchor` | Canonical lifecycle-state matrix and activation-budget slice vocabulary. | current | `artifacts/runtime/managed_workspace_lifecycle.yaml` |
| `evidence.runtime.action_origin_target_labels` | `source_anchor` | Canonical origin / target / route / exposure matrix. | current | `artifacts/runtime/action_origin_target_labels.yaml` |

## Verification method

- **Verification classes used:** design review, vocabulary-reuse
  review, fixture review, schema-alignment review.
- **Procedure summary:** verified that the packet and its companion
  manifest, matrix, and case corpus reuse the frozen
  target-discovery-confidence, host-boundary-cue,
  managed-workspace-lifecycle, origin / target / route / exposure,
  route-change-reason-code, and authority-linkage vocabularies
  without minting parallel tokens. Verified that reviewer-facing
  labels (`warming`, `ready`, `degraded`, `paused`, `suspended`,
  `expired`, `local_only_continuation`) project onto the frozen
  lifecycle states explicitly and that wrong-target correction
  classes and reapproval requirement classes cover the scenarios
  named in the spec.
- **Automation refs:** `not_yet_seeded` for a dedicated target /
  boundary matrix validator; structural parsing is currently the
  available automation.

## Known gaps and waivers

- **Waiver refs:** `none`.
- **Known-limit refs:** `none`.
- **Migration-packet refs:** `none`.
- **Explicit gaps:** no live execution resolver, remote-agent
  broker, managed control plane, or adapter registry is wired to
  this packet yet.
- **Explicit gaps:** no dedicated JSON Schema exists yet for the
  `target_truth_record`, the `host_boundary_matrix` row shape, or
  the reapproval / suspend / resume case shape. The reserved shape
  is documented here for later schema landing.
- **Explicit gaps:** the adapter-confidence placeholder is a
  reserved slot; the adapter-side emitters are not yet defined.

## Reviewer signoff

- **Reviewer / forum:** `@ahmeddyounis`
- **Decision:** `needs_follow_up`
- **Date:** `2026-04-23`
- **Reviewed claim rows:**
  `packet_row:target_and_host_boundary.target_confidence_contract`,
  `packet_row:target_and_host_boundary.host_boundary_cue_stack`,
  `packet_row:target_and_host_boundary.route_class_projection`,
  `packet_row:target_and_host_boundary.wrong_target_reapproval`,
  `packet_row:target_and_host_boundary.managed_workspace_lifecycle`,
  `packet_row:target_and_host_boundary.adapter_confidence_placeholder`,
  `packet_row:target_and_host_boundary.reviewer_label_projection`,
  `packet_row:target_and_host_boundary.seed_corpus`
- **Blocking refs:** `none`

## Refresh trigger

- **Named rerun trigger:** `corpus_or_matrix_revision_changed`.
- **Expected freshness window:** `P30D`.
- **Next packet family to update with the same evidence ids:**
  support-export packet, release-evidence packet, or AI / notebook
  surface packet that starts quoting target-truth or managed-workspace
  reviewer labels.
