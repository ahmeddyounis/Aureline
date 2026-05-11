# Runtime authority model and approval-ticket seed

This document is the reviewer-facing landing page for the canonical
**runtime approval-ticket seed**: the typed authority objects that
later mutating, credential-projecting, and privileged-attach surfaces
consume so target identity, scope, expiry, actor lineage, policy
epoch, and revocation posture are not reinvented at every prompt.

This seed is not a live approval service. It freezes the
**vocabulary** and the **typed profile rows** so the shell, the
policy service, the supervisor, the mutation-journal renderer, the
support-bundle exporter, and any later admin or runtime-authority
service all read the same contract.

If this document and the row or envelope schemas disagree, the
schemas win and this document must be updated in the same change.
The upstream governance authority-ticket contract at
[`/docs/governance/runtime_authority_contract.md`](../governance/runtime_authority_contract.md)
freezes the full authority-ticket model, and the provider-plane
approval ticket at
[`/schemas/integration/approval_ticket.schema.json`](../../schemas/integration/approval_ticket.schema.json)
is admitted under runtime tickets of `authority_class =
external_mutation`. The runtime seed projects against those upstreams
without forking their closed vocabularies.

## Canonical sources

- Seed YAML: [`artifacts/runtime/m1_runtime_approval_ticket_seed.yaml`](../../artifacts/runtime/m1_runtime_approval_ticket_seed.yaml).
- Envelope schema: [`schemas/runtime/m1_runtime_approval_ticket_seed.schema.json`](../../schemas/runtime/m1_runtime_approval_ticket_seed.schema.json).
- Row schema: [`schemas/runtime/approval_ticket.schema.json`](../../schemas/runtime/approval_ticket.schema.json).
- Upstream governance authority-ticket contract:
  [`docs/governance/runtime_authority_contract.md`](../governance/runtime_authority_contract.md).
- Upstream provider-plane approval ticket:
  [`schemas/integration/approval_ticket.schema.json`](../../schemas/integration/approval_ticket.schema.json).
- Example fixtures: [`fixtures/runtime/approval_ticket_examples/`](../../fixtures/runtime/approval_ticket_examples/).
- Validation lane: [`tests/governance/m1_runtime_approval_ticket_seed_lane/run_m1_runtime_approval_ticket_seed_lane.py`](../../tests/governance/m1_runtime_approval_ticket_seed_lane/run_m1_runtime_approval_ticket_seed_lane.py).
- Proof packet:
  [`artifacts/milestones/m1/proof_packets/runtime_approval_ticket_seed.md`](../../artifacts/milestones/m1/proof_packets/runtime_approval_ticket_seed.md).

## Why a typed runtime approval-ticket seed now

Without a frozen runtime contract, every mutating surface invents its
own approval shape: the shell mints one prompt, the policy service
mints another, the AI overlay mints a third, and the support exporter
sees four incompatible approval payloads against one effect. This
seed closes that gap before any mutating M1 surface ships:

- Four authority classes are **closed**: `local_mutation`,
  `external_mutation`, `credential_projection`, `privileged_attach`.
  Adding a fifth requires a new decision row.
- Three issuer classes are **closed**: `shell`, `policy_service`,
  `supervisor`. No other lane MAY mint a ticket. AI conversations,
  extensions, recipes, CLI scripts, browser helpers, remote helpers,
  and automation schedulers MAY request approval but MAY NOT grant
  themselves authority; their requests land in one of the three
  issuer seats and carry a non-empty `requesting_surface_ref`.
- Revocation posture is **typed**. `live_unrevoked` is the only
  spendable state; every other posture is terminal and forces a
  re-prompt.
- Inner provider approval tickets ride on
  `inner_provider_approval_ticket_refs` so an `external_mutation`
  ticket cannot mint itself without a provider-plane approval
  underneath.

## Closed vocabularies

The seed envelope freezes these vocabularies. The row schema's
`$defs` is the canonical source; the envelope vocabulary MUST agree.

### Authority class

| Token | Meaning |
| --- | --- |
| `local_mutation` | Destructive or irreversible local edits (workspace write, rename, project-wide replace, apply extension/AI edit, import profile). |
| `external_mutation` | Provider-plane effects (publish, comment, merge, deferred publish drain). Carries the inner provider approval ticket on `inner_provider_approval_ticket_refs`. |
| `credential_projection` | Projecting a credential handle into a build, terminal, tool invocation, or clipboard. Requires `projection_mode_ref`. |
| `privileged_attach` | Debugger attach, privileged inspection, deep-capture, privileged-snapshot access. Requires `attach_target_ref` and `step_up_required_flag = true`. |

### Side-effect class

| Token | Used by |
| --- | --- |
| `local_reversible_edit` | `local_mutation` |
| `local_destructive_edit` | `local_mutation` (forces `preview_ref` + `rollback_checkpoint_ref`) |
| `external_reversible_comment` | `external_mutation` |
| `external_irreversible_publish` | `external_mutation` (forces `preview_ref`) |
| `credential_handle_projection` | `credential_projection` (forces `projection_mode_ref`) |
| `privileged_inspection_attach` | `privileged_attach` (forces `attach_target_ref` and `step_up_required_flag = true`) |

### Issuer class

`shell`, `policy_service`, `supervisor`. No other lane MAY mint.

### Request origin class

Three issuer seats (`user_shell_prompt`, `policy_decision`,
`supervisor_control_path`) plus seven requester classes
(`ai_conversation_request`, `extension_request`, `recipe_request`,
`cli_script_request`, `browser_helper_request`, `remote_helper_request`,
`automation_scheduler_request`). Requester origins MUST carry a
non-empty `requesting_surface_ref`; the lane that mints the ticket
MUST still be one of the three admissible issuers.

### Use posture

`single_use` (default), `bounded_reuse` (requires a counter + window
expiry), and `session_scoped` (forbidden for
`external_irreversible_publish` and `local_destructive_edit`).

### Revocation posture class

`pending_issue`, `live_unrevoked`, `expired`, `revoked_by_user`,
`revoked_by_admin`, `revoked_by_drift`, `revoked_by_rotation`,
`revoked_by_emergency_action`, `lineage_broken`. Only `live_unrevoked`
is spendable.

### Invalidation reason (drift dimensions)

`target_identity_drift`, `workspace_trust_drift`, `policy_epoch_drift`,
`provider_scope_drift`, and `sandbox_profile_drift` are the five
drift dimensions that MUST invalidate an in-flight ticket before
spend. `rotation`, `admin_revoke`, `user_revoke`,
`emergency_action_override`, and `ticket_lineage_broken` are the
other reasons.

### Denial reason

A frozen 30-token set that names which dimension failed. Denials
fail closed; they MUST NOT silently retry, and they MUST NOT
downgrade the authority class or the declared side-effect class.

## Seeded approval-ticket profiles

| `approval_ticket_profile_id` | Authority class | Side effect | Issuer | Request origin | Use posture |
| --- | --- | --- | --- | --- | --- |
| `local_mutation.project_wide_destructive_edit` | `local_mutation` | `local_destructive_edit` | `shell` | `user_shell_prompt` | `single_use` |
| `external_mutation.provider_review_comment_publish` | `external_mutation` | `external_irreversible_publish` | `shell` | `user_shell_prompt` | `single_use` |
| `credential_projection.build_sandbox_registry_read` | `credential_projection` | `credential_handle_projection` | `policy_service` | `policy_decision` | `bounded_reuse` |
| `privileged_attach.local_language_server_debugger` | `privileged_attach` | `privileged_inspection_attach` | `shell` | `user_shell_prompt` | `session_scoped` |
| `local_mutation.extension_requested_apply` | `local_mutation` | `local_reversible_edit` | `shell` | `extension_request` | `single_use` |

The union of the seeded rows covers all four M1 authority classes,
both issuer seats (`shell` and `policy_service`), the four protected
side-effect classes (`local_destructive_edit`,
`external_irreversible_publish`, `credential_handle_projection`,
`privileged_inspection_attach`), and a requester-origin row that
forces a non-empty `requesting_surface_ref`.

## Failure-drill coverage

Five named drills, all reproducible under
`--force-drill <approval_ticket_profile_id>:<drill_id>`:

| Row | Drill | Expected check id |
| --- | --- | --- |
| `local_mutation.project_wide_destructive_edit` | `runtime_approval_ticket_drill.local_destructive_preview_ref_dropped` | `runtime_approval_ticket.preview_ref_required_for_destructive_or_irreversible_side_effect` |
| `external_mutation.provider_review_comment_publish` | `runtime_approval_ticket_drill.external_mutation_inner_provider_ticket_dropped` | `runtime_approval_ticket.inner_provider_approval_ticket_required_for_external_mutation` |
| `credential_projection.build_sandbox_registry_read` | `runtime_approval_ticket_drill.credential_projection_projection_mode_ref_dropped` | `runtime_approval_ticket.projection_mode_ref_required_for_credential_projection` |
| `privileged_attach.local_language_server_debugger` | `runtime_approval_ticket_drill.privileged_attach_step_up_required_flag_dropped` | `runtime_approval_ticket.step_up_required_flag_must_be_true_for_privileged_attach` |
| `local_mutation.extension_requested_apply` | `runtime_approval_ticket_drill.extension_request_missing_requesting_surface_ref` | `runtime_approval_ticket.requesting_surface_ref_required_for_non_issuer_origin` |

## Named runtime consumers

The validation lane resolves every `named_runtime_consumers[].consumer_ref`
on disk and asserts that `consumed_fields` is non-empty. The
following consumers are live as of this seed:

- **This document** (`docs/runtime/m1_authority_model.md`) — quotes
  the seeded rows verbatim so future runtime, support, and review
  surfaces all read the same authority vocabulary.
- **Upstream governance authority-ticket contract**
  (`docs/governance/runtime_authority_contract.md`) — the upstream
  contract the runtime seed projects against.
- **Validation lane**
  (`tests/governance/m1_runtime_approval_ticket_seed_lane/run_m1_runtime_approval_ticket_seed_lane.py`)
  — replays the seed against the envelope schema, the row schema,
  the upstream contract, and the example fixtures; reproduces every
  named failure drill loudly under `--force-drill`.

## Refresh

Re-run the validation lane after a change to:

- the seed YAML,
- the envelope schema or the row schema,
- this reviewer-facing landing page,
- the upstream governance authority-ticket contract or the upstream
  provider-plane approval-ticket schema the seed cross-references, or
- the build-identity record the capture embeds.

## Closure rule

The lane stays open until the latest capture lands under the governed
proof root and every row reports PASS for closed-vocabulary
membership (authority class, side-effect class, issuer class, request
origin class, use posture, revocation posture class, redaction class,
high-risk flag, audit-event id, denial reason, invalidation reason),
the conditional invariants
(`local_mutation` ⇒ local-edit side-effect;
`external_mutation` ⇒ non-empty `inner_provider_approval_ticket_refs`;
`credential_projection` ⇒ non-empty `projection_mode_ref`;
`privileged_attach` ⇒ non-empty `attach_target_ref` and
`step_up_required_flag = true`;
`local_destructive_edit` ⇒ `preview_ref` and `rollback_checkpoint_ref`;
`external_irreversible_publish` ⇒ `preview_ref`;
non-issuer `request_origin_class` ⇒ non-empty `requesting_surface_ref`;
`use_posture = bounded_reuse` ⇔ `bounded_reuse_counter` present;
`use_posture = session_scoped` ⇒ side-effect class is not in
{`external_irreversible_publish`, `local_destructive_edit`}),
the required coverage rules (authority classes, side-effect classes,
request origins, revocation postures), named-runtime-consumer
existence with non-empty `consumed_fields`, and its five named
failure drills.
