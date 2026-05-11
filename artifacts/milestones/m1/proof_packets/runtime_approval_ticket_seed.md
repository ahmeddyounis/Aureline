# Proof packet: M1 runtime approval-ticket seed

Purpose: anchor proof captures for the unattended M1 lane that
validates the canonical runtime approval-ticket seed. The lane proves
the seed is consumable by the shell, the policy service, the
supervisor, the mutation-journal renderer, and the support-bundle
exporter — without re-encoding the authority-class, side-effect-class,
issuer-class, request-origin-class, use-posture, revocation-posture,
high-risk-flag, audit-event-id, denial-reason, or invalidation-reason
vocabularies on each surface.

Reviewer entry point:
[`/docs/runtime/m1_authority_model.md`](../../../docs/runtime/m1_authority_model.md).
Upstream governance authority-ticket contract:
[`/docs/governance/runtime_authority_contract.md`](../../../docs/governance/runtime_authority_contract.md).
Upstream provider-plane approval-ticket schema:
[`/schemas/integration/approval_ticket.schema.json`](../../../schemas/integration/approval_ticket.schema.json).

## Canonical sources

- `artifacts/runtime/m1_runtime_approval_ticket_seed.yaml` — seed
  rows the runner consumes. Carries:
  - the M1 envelope (`schema_version`, `matrix_id`, `owner_dri`,
    `overview_page`, `upstream_authority_contract_ref`,
    `upstream_integration_approval_ticket_schema_ref`,
    `row_schema_ref`, `build_identity_ref`, `validation_lane_ref`),
  - closed envelope vocabularies for authority class, side-effect
    class, issuer class, request-origin class, actor class, use
    posture, revocation posture class, redaction class, high-risk
    flag, audit-event id, denial reason, invalidation reason, and
    failure-drill id,
  - required coverage lists (authority classes, side-effect classes,
    request origins, revocation postures),
  - the named runtime consumers the seed asserts are live, and
  - one approval-ticket profile row per typed scenario with the
    uniform `(approval_ticket_profile_id, authority_class,
    side_effect_class, issuer_class, request_origin_class,
    use_posture, revocation_posture_class, example_payload_ref,
    owner_dri, failure_drill)` shape.

- `schemas/runtime/m1_runtime_approval_ticket_seed.schema.json` —
  envelope schema; freezes vocabularies, required coverage lists,
  named-consumer shape, matrix identity, and pins the canonical
  landing-page path.

- `schemas/runtime/approval_ticket.schema.json` — row schema;
  freezes the closed authority-class (`local_mutation`,
  `external_mutation`, `credential_projection`, `privileged_attach`),
  side-effect-class, issuer-class, request-origin-class, actor-class,
  use-posture, revocation-posture-class, redaction-class,
  high-risk-flag, audit-event-id, denial-reason, and
  invalidation-reason vocabularies, plus the conditional invariants
  (local_mutation ⇒ local-edit side-effect; external_mutation ⇒
  non-empty inner_provider_approval_ticket_refs;
  credential_projection ⇒ non-empty projection_mode_ref;
  privileged_attach ⇒ non-empty attach_target_ref and
  step_up_required_flag = true; local_destructive_edit ⇒ preview_ref
  and rollback_checkpoint_ref; external_irreversible_publish ⇒
  preview_ref; non-issuer request_origin_class ⇒ non-empty
  requesting_surface_ref; use_posture = bounded_reuse ⇔
  bounded_reuse_counter present; use_posture = session_scoped ⇒
  side-effect class is not in {external_irreversible_publish,
  local_destructive_edit}).

- `tests/governance/m1_runtime_approval_ticket_seed_lane/run_m1_runtime_approval_ticket_seed_lane.py`
  — unattended runner that replays the seed and emits the durable
  JSON capture.

## Upstream sources the seed projects against

- `docs/governance/runtime_authority_contract.md` — upstream
  governance authority-ticket contract (full authority-ticket model,
  the six authority classes including local-workspace-mutation,
  external-provider-mutation, credential-projection,
  debug-or-privileged-inspection, policy-or-admin-change, and
  automation-lineage-admission). The M1 runtime seed projects the
  four protected classes (`local_mutation`, `external_mutation`,
  `credential_projection`, `privileged_attach`) into the runtime
  boundary without forking the upstream's vocabularies.
- `schemas/integration/approval_ticket.schema.json` — upstream
  provider-plane approval-ticket schema. The seed admits provider
  approval tickets under runtime tickets of `authority_class =
  external_mutation` via `inner_provider_approval_ticket_refs`.

## Named runtime consumers

- `docs/runtime/m1_authority_model.md` — reviewer-facing landing
  page that quotes the seeded rows verbatim so the shell, the policy
  service, the supervisor, the mutation-journal renderer, and the
  support-bundle exporter all read the same authority vocabulary.
- `docs/governance/runtime_authority_contract.md` — upstream
  authority-ticket contract; the runner asserts the doc resolves on
  disk so the M1 seed cannot quietly outlive its upstream.
- `tests/governance/m1_runtime_approval_ticket_seed_lane/run_m1_runtime_approval_ticket_seed_lane.py`
  — live CI/review consumer (this lane) that replays the seed,
  asserts closed-vocabulary agreement with the row schema, the
  conditional invariants, required coverage, named-consumer
  resolution, and reproduces every named failure drill loudly.

## Live runtime consumers (read-only)

- `artifacts/build/build_identity.json` — exact-build identity that
  the capture embeds for cross-artifact traceability.

## Validation captures

- `artifacts/milestones/m1/captures/runtime_approval_ticket_seed_validation_capture.json`

## Approval-ticket profile coverage

The seed asserts the following approval-ticket profiles are present
as typed rows:

| `approval_ticket_profile_id` | Authority class | Side effect | Issuer | Origin |
| --- | --- | --- | --- | --- |
| `local_mutation.project_wide_destructive_edit` | `local_mutation` | `local_destructive_edit` | `shell` | `user_shell_prompt` |
| `external_mutation.provider_review_comment_publish` | `external_mutation` | `external_irreversible_publish` | `shell` | `user_shell_prompt` |
| `credential_projection.build_sandbox_registry_read` | `credential_projection` | `credential_handle_projection` | `policy_service` | `policy_decision` |
| `privileged_attach.local_language_server_debugger` | `privileged_attach` | `privileged_inspection_attach` | `shell` | `user_shell_prompt` |
| `local_mutation.extension_requested_apply` | `local_mutation` | `local_reversible_edit` | `shell` | `extension_request` |

The union of every row covers all four M1 protected authority
classes, the four protected side-effect classes
(`local_destructive_edit`, `external_irreversible_publish`,
`credential_handle_projection`, `privileged_inspection_attach`),
both issuer seats (`user_shell_prompt`, `policy_decision`), and the
`live_unrevoked` revocation posture.

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

## Refresh

Re-run the validation lane after a change to:

- the seed YAML,
- either schema (envelope or row),
- the reviewer-facing landing page,
- the upstream governance authority-ticket contract or the upstream
  provider-plane approval-ticket schema the seed cross-references, or
- the build-identity record the capture embeds.

## Closure rule

The lane stays open until the latest capture lands under the governed
proof root and every row reports PASS for closed-vocabulary
membership (authority_class, side_effect_class, issuer_class,
request_origin_class, actor_class, use_posture,
revocation_posture_class, redaction_class, high_risk_flag,
audit_event_id, denial_reason, invalidation_reason), the conditional
invariants (authority-class / side-effect agreement, inner provider
tickets for external_mutation, projection_mode_ref for
credential_projection, attach_target_ref + step_up_required_flag for
privileged_attach, preview_ref + rollback_checkpoint_ref for
destructive edits, preview_ref for irreversible publishes,
requesting_surface_ref for non-issuer origins, bounded_reuse_counter
agreement, session_scoped forbiddens), the required coverage rules
(authority classes, side-effect classes, request origins, revocation
postures), named-runtime-consumer existence, and its five named
failure drills.
