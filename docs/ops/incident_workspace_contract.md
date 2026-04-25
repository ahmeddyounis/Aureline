# Operational incident workspace, runbook packet, and evidence-handoff bundle contract

This document freezes the object model Aureline uses when an
operational incident is opened, diagnosed, mitigated, and handed off to
postmortem, review, browser, console, or last-synced offline-view
readers. The goal is to keep operational diagnosis, code context,
approvals, and export bundles attributable without depending on a live
vendor console or a hidden alternate control plane.

If this document, the companion schemas, and the worked fixtures
disagree, the normative sources in `.t2/docs/` win and this document
plus its companions update in the same change.

## Companion artifacts

- [`/schemas/ops/incident_workspace.schema.json`](../../schemas/ops/incident_workspace.schema.json)
  — boundary schema for `incident_workspace_record`. Carries the
  incident header, ownership, pinned code context, alert / page
  snapshot, log / trace / metric signal slices with provenance, the
  bounded resource snapshot, and the closed lists of runbook-packet,
  action-ledger, browser / console handoff, and evidence-handoff bundle
  refs the workspace exports.
- [`/schemas/ops/runbook_packet.schema.json`](../../schemas/ops/runbook_packet.schema.json)
  — boundary schema for `runbook_packet_record` and
  `action_ledger_entry_record`. Freezes the typed step grammar, the
  sandbox-and-approval posture every mutating step inherits from
  ordinary runtime actions, the preview-hash pin every mutating step
  computes before apply, and the append-only outcome vocabulary every
  action-ledger entry records after the step ran.
- [`/schemas/ops/evidence_handoff_bundle.schema.json`](../../schemas/ops/evidence_handoff_bundle.schema.json)
  — boundary schema for `evidence_handoff_bundle_record`. Freezes the
  immutable-snapshot rules for postmortem / review bundles, browser /
  console handoff exits, and last-synced offline views: bundles are
  frozen-no-post-hoc-rewrite by default, corrections supersede via a
  new bundle, and the prior bundle is never deleted, backdated, or
  rewritten in place.
- [`/fixtures/ops/incident_cases/`](../../fixtures/ops/incident_cases/)
  — worked fixtures exercising the contract.

This contract composes with (and does not replace):

- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
  — secret-broker handle and raw-secret-forbidden boundary for every
  log / trace / metric / alert / runbook payload that crosses the
  workspace boundary.
- [`/docs/adr/0009-execution-context-and-scope.md`](../adr/0009-execution-context-and-scope.md)
  and
  [`/schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json)
  — target identity, sandbox posture, and policy epoch every mutating
  runbook step resolves against.
- [`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md)
  and
  [`/schemas/integration/browser_handoff_packet.schema.json`](../../schemas/integration/browser_handoff_packet.schema.json),
  [`/schemas/integration/approval_ticket.schema.json`](../../schemas/integration/approval_ticket.schema.json)
  — browser-handoff packet and approval-ticket envelope every
  mutating runbook step that exits the local control plane travels
  through.
- [`/docs/adr/0018-workspace-trust-and-restricted-mode.md`](../adr/0018-workspace-trust-and-restricted-mode.md)
  — workspace-trust state every mutating action inherits.
- [`/docs/security/emergency_action_model.md`](../security/emergency_action_model.md)
  and
  [`/schemas/security/emergency_action_record.schema.json`](../../schemas/security/emergency_action_record.schema.json)
  — emergency-action / revocation linkage the action-ledger pins by
  reference when a runbook step invokes an emergency control.
- [`/docs/security/high_risk_control_quorum.md`](../security/high_risk_control_quorum.md)
  and
  [`/schemas/security/break_glass_event.schema.json`](../../schemas/security/break_glass_event.schema.json)
  — break-glass event the action-ledger pins by reference when a
  runbook step invokes a break-glass profile.
- [`/docs/security/severity_matrix.md`](../security/severity_matrix.md)
  and
  [`/schemas/security/incident_workspace_packet.schema.json`](../../schemas/security/incident_workspace_packet.schema.json)
  — security-side private-triage packet the operational workspace
  composes with by reference when the same incident also opened a
  security triage. The two packet families are intentionally distinct:
  the operational workspace is read-mostly diagnosis and mitigation
  state for live operational issues; the security packet is the
  private-triage workspace for trust-affecting findings.
- [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
  and
  [`/schemas/support/support_bundle.schema.json`](../../schemas/support/support_bundle.schema.json)
  — support / export bundle the workspace pins by reference rather
  than re-embedding raw payloads.
- [`/docs/runtime/fault_domains_and_restart_policy.md`](../runtime/fault_domains_and_restart_policy.md)
  and
  [`/schemas/runtime/forensic_packet.schema.json`](../../schemas/runtime/forensic_packet.schema.json)
  — resource-counter / strike-window / heartbeat / fault-domain /
  host-class lineage the resource snapshot composes with by reference.

Normative sources this contract projects from:

- `.t2/docs/Aureline_PRD.md` operational-supportability and
  diagnostic-evidence passages.
- `.t2/docs/Aureline_Technical_Architecture_Document.md`
  fault-domain, supervisor, runbook, and recovery-ladder passages.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` operational-handoff,
  postmortem, and admin / policy review passages.

If this document disagrees with those sources, those sources win and
this document plus the companion schemas update in the same change.

## Why this exists

Every operational surface that talks about incidents, runbooks,
mitigations, or postmortems otherwise reaches for one of three failure
modes:

1. **Free-text incident pages.** Each surface invents a slightly
   different shape for the incident header, the alert snapshot, and
   the action chronology. Postmortem readers cannot resolve the same
   target context the responder saw because nothing pinned the
   exact-build identity, branch, or commit at incident open.
2. **Mutating "just run this script" runbooks.** The runbook step
   carries a free-text description and the action runs as ambient
   shell rather than through the same sandbox / approval / preview
   posture that ordinary runtime actions inherit. Reviewers cannot
   tell whether the mutation went through workspace-trust, an
   approval-ticket envelope, a break-glass profile, or none of the
   above.
3. **Hidden alternate control planes.** A responder hands off to a
   browser, a vendor console, or an admin SaaS surface, mutates the
   target there, and returns to the workspace. The transition is
   invisible to the chronology, the evidence is unattributable, and
   the postmortem reader has no way to resolve who did what under
   which approval envelope.

This contract closes those gaps by freezing one operational object
family:

- **`incident_workspace_record`** — the read-mostly diagnosis and
  mitigation surface every responder works inside. Read-mostly is the
  default: `mutating_action_admission_class` resolves to
  `read_mostly_default_no_mutation_admitted` until a typed sandbox /
  approval posture is admitted.
- **`runbook_packet_record`** — the typed step grammar with
  sandbox-and-approval posture inherited from ordinary runtime
  actions.
- **`action_ledger_entry_record`** — the append-only chronology of
  what ran, under which posture, with which preview-hash, and with
  which outcome. Prior entries never mutate and are never backdated;
  corrections supersede via stable id.
- **`evidence_handoff_bundle_record`** — the immutable snapshot every
  postmortem / review / browser-handoff / console-handoff /
  last-synced-offline-view reader resolves against. Frozen-no-post-hoc-
  rewrite by default; corrections supersede via a new bundle.

## Scope

Frozen at this revision:

- one `incident_workspace_record` carrying incident header,
  operational severity, lifecycle state, ownership, deployment-profile
  scope, policy context, pinned code context, alert / page snapshot,
  signal slices with provenance, resource snapshot, runbook-packet
  refs, action-ledger entry refs, browser / console handoff records,
  evidence-handoff bundle refs, security-side private-triage / advisory
  / emergency-action / break-glass / support-bundle linkage, mutating-
  action admission class, and redaction class;
- one `runbook_packet_record` carrying runbook id, applicable
  operational-severity and deployment-profile scopes, default
  target-context, ordered step records (read-only or mutating), and
  policy / redaction envelope;
- one `action_ledger_entry_record` per executed step carrying actor
  ref, executed-at timestamp, step-intent class, target-context,
  sandbox-admission, approval-admission, preview-hash and applied
  preview-hash, approval-ticket / break-glass / browser-handoff /
  console-handoff / publish-later-queue-item / provider-callback /
  rollback-checkpoint refs, raw-ref class and id, outcome class and
  summary, supersedes ref, and policy / redaction envelope;
- one `evidence_handoff_bundle_record` carrying bundle kind, source
  incident workspace id, immutable-snapshot timestamp, integrity hash,
  integrity / rewrite / handoff-destination / retention classes,
  preserved target context, closed lists of evidence-item / runbook /
  action-ledger / signal-slice refs, support / security / advisory /
  emergency-action / break-glass / browser-handoff / console-handoff /
  postmortem-review linkage, and policy / redaction envelope.

Out of scope at this revision (named explicitly so reviewers know what
is *not* being decided here):

- implementing incident UIs, paging-provider integrations, alert
  routers, observability collectors, runbook editors, action runners,
  postmortem editors, browser-handoff exits, console-handoff exits, or
  admin / policy review surfaces — every contract above is a boundary
  schema and narrative companion, not a user-facing surface or an
  implementation crate;
- live wiring against any specific paging provider (PagerDuty,
  Opsgenie, VictorOps, internal paging routes), observability backend
  (OpenTelemetry collector, Prometheus, Datadog, Honeycomb,
  CloudWatch, Stackdriver), or admin console (cloud provider
  console, on-prem admin UI);
- final user-facing copy, status-page integration, or notification
  routing — those compose against the `notification_side_effect_class`
  vocabulary in
  [`/schemas/work_items/status_transition_packet.schema.json`](../../schemas/work_items/status_transition_packet.schema.json)
  but are not frozen here.

## Read-mostly default

The workspace is read-mostly by default. `incident_workspace_record`
resolves `mutating_action_admission_class` to
`read_mostly_default_no_mutation_admitted` until a typed sandbox /
approval posture is admitted. Concretely:

- read-only step intents (`diagnostic_inspection_read_only`,
  `evidence_capture_read_only`, `communication_read_only`,
  `approval_gate_no_mutation`) pin sandbox to
  `inspect_only_no_mutation`, approval to
  `no_approval_required_read_only`, preview-hash to null, and
  outcome class to a read-only-admissible value;
- mutating step intents (`mitigation_mutating_under_runtime_approval`,
  `mitigation_mutating_under_break_glass`,
  `mitigation_mutating_browser_handoff`,
  `mitigation_mutating_console_handoff`,
  `automation_invoke_recorded_recipe`) MUST cite a non-null
  preview-hash, MUST resolve sandbox-admission to a typed
  admissible class, and MUST resolve approval-admission to a typed
  admissible class — the same posture ordinary runtime actions
  inherit;
- `step_intent_unknown_requires_review` is the safe-default that
  strips every admissible_under_* posture;
- `mutating_admission_unknown_pending_review` is the workspace-level
  safe-default that fails closed; it forbids any mutating
  action-ledger entry until a typed admission class is resolved;
- `lifecycle_state_class = imported_replay_no_live_lifecycle` and
  `alert_source_class = imported_alert_no_live_source` force the
  workspace into a read-only replay posture: no live mutation is
  admissible against an imported snapshot.

The action-ledger inherits the same posture: the entry's
`step_intent_class`, `sandbox_admission_class`, and
`approval_admission_class` MUST match the runbook step the entry
records. A mutation labelled `mitigation_mutating_browser_handoff`
that omits the matching `browser_handoff_packet_ref` is denied with
the closed reason
`browser_handoff_intent_must_cite_browser_handoff_packet_ref`.

## Preview-hash and rollback rules

Every mutating action computes a preview-hash before apply (the same
posture ordinary runtime actions follow). The hash is recorded on the
runbook step record and on the matching action-ledger entry under
`preview_hash`; the observed hash on apply is recorded under
`applied_preview_hash`. Reviewers compare the two:

- equal → clean apply, outcome `success_observed` /
  `success_under_partial_evidence` admissible;
- distinct → drift between previewed and applied mutation; outcome
  `rolled_back_after_partial_apply` admissible only when
  `rollback_checkpoint_ref` is non-null;
- captured offline → outcome `captured_offline_pending_drain` MUST
  cite a non-null `publish_later_queue_item_ref` so the deferred
  intent is visible in the publish-later queue rather than dropped
  silently;
- external handoff pending callback → outcome
  `external_handoff_pending_provider_callback` MUST cite a non-null
  `provider_callback_envelope_ref` so the callback envelope arrival
  flips the entry to confirmed rather than the responder marking it
  succeeded by hand.

Raw bodies (request / response / payload bytes) never cross the
workspace boundary. The action-ledger entry pins a typed `raw_ref_class`
and a non-null `raw_ref_id` when a raw artefact is required for
later reconstruction; the resolver enforces redaction defaults under
ADR-0007.

## Browser and console handoff are attributable transitions

Every transition out of the local control plane travels through a
browser-handoff packet ref or a console-handoff session ref:

- `mitigation_mutating_browser_handoff` runbook steps cite a non-null
  `browser_handoff_packet_ref` (resolved through
  [`/schemas/integration/browser_handoff_packet.schema.json`](../../schemas/integration/browser_handoff_packet.schema.json))
  and pin `target_context_class = browser_handoff_external_target`;
- `mitigation_mutating_console_handoff` runbook steps cite a non-null
  `console_handoff_session_ref` and pin
  `target_context_class = console_handoff_external_target`;
- the workspace records each transition in
  `browser_or_console_handoff_records` with its handoff kind class and
  the recorded-at timestamp so a postmortem reader can resolve the
  exit chronology;
- the matching evidence-handoff bundle MUST resolve
  `bundle_kind_class` to
  `browser_console_handoff_attributable_exit` /
  `console_handoff_attributable_exit` and cite the same handoff ref
  so the exit is visible in the chronology rather than a hidden
  alternate control plane.

`incident_workspace_silent_browser_handoff_forbidden_denial` and
`incident_workspace_silent_console_handoff_forbidden_denial` close
the loop: an action-ledger entry that mutates against an external
target without citing a typed handoff ref is denied by audit.

## Immutable evidence-handoff bundles

Postmortem review, internal review, browser-handoff exit,
console-handoff exit, support-export companion, and last-synced offline
view all read the same `evidence_handoff_bundle_record`. The bundle is
frozen-no-post-hoc-rewrite by default:

- `integrity_class = frozen_no_post_hoc_rewrite` is the default; the
  `superseded_by_bundle_id_ref` field MUST be null;
- corrections land as a new bundle with
  `integrity_class = superseded_by_new_bundle`, a non-null
  `superseded_by_bundle_id_ref` on the prior bundle, and a non-null
  `supersedes_bundle_id_ref` on the new bundle. The prior bundle is
  preserved in the chronology and is never deleted, backdated, or
  rewritten in place;
- `rewrite_admissibility_class = no_post_hoc_rewrite_admitted` is the
  default; `correction_must_supersede_via_new_bundle` is admissible
  for typed corrections; `ai_tool_proposed_correction_pending_review`
  forbids leaving pending review state and forces
  `integrity_class` into `withdrawn_invalid_invocation` or
  `integrity_unverifiable_user_review_required`;
- `bundle_kind_class = ai_tool_proposed_bundle_pending_review`
  forbids leaving pending review and forbids handoff to live
  destinations;
- `bundle_kind_class = imported_evidence_replay_bundle_no_live_target`
  forbids any live destination so an imported replay bundle cannot
  silently route a mutation against a live target;
- `preserved_target_context` pins the execution-context /
  deployment-profile / exact-build identity at
  `immutable_snapshot_at` so postmortem / review readers resolve the
  same target context the responder saw;
- the closed lists of `evidence_item_refs`,
  `included_runbook_packet_id_refs`,
  `included_action_ledger_entry_id_refs`, and
  `included_signal_slice_id_refs` are pinned at snapshot time and
  never rewritten in place. A correction that needs to add or remove
  a ref lands as a new bundle.

`incident_workspace_silent_post_hoc_evidence_rewrite_forbidden_denial`
closes the loop: an export bundle that tries to rewrite a prior
bundle's content in place is denied by audit and forced through the
new-bundle / supersede path.

## Browser, console, support, and security composition

The workspace composes with neighbouring contracts by reference rather
than by re-embedding their bodies:

- **support / export**: `linked_support_bundle_refs` pins
  `support_bundle_record` ids; the bundle resolver under ADR-0007
  enforces the redaction defaults.
- **security private-triage**:
  `linked_security_incident_packet_refs` pins
  `incident_workspace_packet_record` ids when the operational
  incident also opened a security triage. The two packet families
  remain distinct: the operational workspace is read-mostly diagnosis
  and mitigation state, the security packet is the private-triage
  workspace.
- **advisory / emergency / break-glass**: `linked_advisory_refs`,
  `linked_emergency_action_refs`, `linked_break_glass_event_refs` pin
  the matching ids when the action-ledger invokes an emergency
  control.
- **forensic / fault-domain**: the resource snapshot pins
  `linked_forensic_packet_refs` so the detailed counters (CPU, memory,
  queue depths, handle counts, strike windows, heartbeat,
  fault-domain, host-class) live on the forensic packet rather than
  duplicated on the workspace.

## Out of scope

- live wiring against any specific paging provider, observability
  backend, or admin console (named above);
- runbook editors, action runners, postmortem editors, browser /
  console handoff UIs, last-synced offline-view readers — every
  surface composes against the contract above but is not frozen here;
- final user-facing copy, status-page integration, or notification
  routing.

## Worked fixtures

See [`/fixtures/ops/incident_cases/`](../../fixtures/ops/incident_cases/)
for worked fixtures exercising the contract.
