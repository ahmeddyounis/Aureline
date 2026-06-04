# Stable Runbook Source, Step Envelope, and Handoff Truth

This contract makes runbooks executable guidance instead of rich-text
instructions. Support, incident, admin, CLI, companion, and export surfaces
must consume the checked-in packet shape instead of inventing local wording for
source authority, step state, approval, deviation, or browser handoff truth.

## Source Descriptors

Every runbook source declares:

- `source_class`: repo/workspace-local, mirrored docs pack, managed catalog, or
  browser-only vendor documentation.
- `authoritative_posture`: authoritative, managed-delegated, reference-only, or
  downgraded because authority cannot be verified.
- `signer_or_source_ref`, `freshness_state`, `approver_policy_ref`, and
  `export_right`.

Browser-only vendor documentation is always `reference_only`. It can provide
context and handoff anchors, but it cannot become managed-catalog authority.

## Executable Step Envelopes

Each step envelope carries stable identity and bounded authority:

- `step_id`
- `step_class`: observe, verify, mitigate, rollback, or communicate
- `target_selector_scope`
- `approval_requirement`
- `destination_class`
- `expected_evidence_outputs`
- optional `shared_action_envelope`
- optional `external_handoff`

Mutating in-product steps must reuse the shared action envelope with
`action_envelope_ref`, `preview_hash_ref`, `approval_ref`, and `audit_ref`.
Runbook execution does not get a separate approval or audit bypass.

## Step Results

Step records keep executable results distinct from advisory prose:

- `preview_only`
- `approved`
- `executed`
- `handoff_required`
- `deviated`

Every execution record carries an `incident_timeline_id`, actor ref, approval
refs, deviation refs, external handoff refs, and evidence/export links so
support, incident, and compliance packets share one chronology.

## Deviation Notes

When an operator departs from the prescribed step sequence, a durable
`runbook_deviation_note` is emitted. The note links the departed step, actor,
timeline id, reason, and evidence refs. Deviation notes remain exportable
metadata and join to incidents, reviews, rollout actions, and support bundles by
stable id.

## Browser and Vendor-Console Handoffs

External pivots use `external_handoff` and `runbook_external_handoff_bundle`
records. They preserve destination class, reason, return anchor or follow-up
note, and stable handoff ids. Raw provider URLs, console sessions, approval
bodies, and secret payloads are excluded by default.

Browser/mobile companions and embedded docs panes may surface context and
approval requests within the declared scope. They may not create a hidden
privileged mutate channel or mark a provider-owned object changed unless a
separately reviewed command executes.

## Local Checklist Completion

Local follow-up records preserve `provider_object_ownership` and
`local_completion_state`. Checking off a local follow-up item can close local
work, but it must not imply that a linked provider-owned incident, ticket, or
alert changed unless `provider_mutation_claimed` is true and a
`reviewed_command_ref` is present.

## References

- Boundary schema:
  `schemas/support/runbook-step-envelope.schema.json`
- Fixture corpus:
  `fixtures/support/m4/stabilize-runbook-source-step-envelope-and-handoff-truth/`
- Crate consumer:
  `crates/aureline-support/src/stabilize_runbook_source_step_envelope_and_handoff_truth/mod.rs`
