# Support Center information architecture, capability cards, and route-to-evidence contract

This document freezes the Support Center's top-level navigation, its
per-module capability-card field set, and the symptom-to-module route
table Aureline uses across desktop and CLI/headless surfaces. It is
the navigation surface that sits above the support-intake picker
contract, the recovery ladder, the repair transaction, the support
bundle, the object-handoff packet, the runbook execution contract,
and the Project Doctor scenario matrix.

If this document, the
[`schemas/support/support_center_capability_card.schema.json`](../../schemas/support/support_center_capability_card.schema.json)
schema, the
[`artifacts/support/support_center_routes.yaml`](../../artifacts/support/support_center_routes.yaml)
artifact, and the
[`fixtures/support/support_center_cases/`](../../fixtures/support/support_center_cases/)
corpus disagree, the frozen support-bundle contract, the support-
intake contract, the object-handoff contract, the recovery-ladder
contract, the repair-transaction contract, the project-doctor scenario
matrix, and the record-class registry win for tooling and this packet
plus its companion artifacts update in the same change.

## Companion artifacts

- [`/schemas/support/support_center_capability_card.schema.json`](../../schemas/support/support_center_capability_card.schema.json)
  — boundary schema for `support_center_capability_card_record`,
  `support_center_route_record`, and
  `support_center_information_architecture_record`. Defines the
  closed nine-module set, the closed six symptom-surface classes,
  the closed five deployment-context classes, the closed card-
  posture vocabulary, the closed evidence-source vocabulary, the
  closed local-action and export-action vocabularies, the headless-
  availability vocabulary, the closed policy-constraint vocabulary,
  the closed open-related-packet action vocabulary, the no-upload-
  first invariant block, and the per-deployment-context parity-row
  shape.
- [`/artifacts/support/support_center_routes.yaml`](../../artifacts/support/support_center_routes.yaml)
  — machine-readable card and route table. Carries one capability
  card per module, one route per symptom surface, and one
  information-architecture index that pins the closed module,
  symptom-surface, and deployment-context sets.
- [`/fixtures/support/support_center_cases/`](../../fixtures/support/support_center_cases/)
  — one worked case per deployment context (local-only, managed,
  self-hosted, mirrored, offline). Cases project onto
  `support_center_route_record` so per-deployment parity gaps remain
  visible rather than implicit.
- [`/docs/support/support_center_concept.md`](./support_center_concept.md)
  — product-facing Support Center concept that names the design
  principles and the recovery-ladder rungs the IA modules align
  with.
- [`/docs/support/support_intake_and_escalation_contract.md`](./support_intake_and_escalation_contract.md)
  — intake-bound capability-card contract with the seven-card set
  the IA's nine-module set extends. The IA picks up
  `advisory_or_incident_history` and `field_diagnostics` as new
  modules that have no intake-bound capability and resolve to
  read-only review and read-only diagnosis surfaces.
- [`/fixtures/support/scenario_matrix.yaml`](../../fixtures/support/scenario_matrix.yaml)
  — Project Doctor scenario matrix every IA route preserves an
  evidence-id binding into.
- [`/schemas/support/scenario_picker.schema.json`](../../schemas/support/scenario_picker.schema.json),
  [`/schemas/support/doctor_finding.schema.json`](../../schemas/support/doctor_finding.schema.json),
  [`/schemas/support/support_bundle.schema.json`](../../schemas/support/support_bundle.schema.json),
  [`/schemas/support/object_handoff_packet.schema.json`](../../schemas/support/object_handoff_packet.schema.json),
  [`/schemas/support/repair_transaction.schema.json`](../../schemas/support/repair_transaction.schema.json),
  [`/schemas/support/recovery_action.schema.json`](../../schemas/support/recovery_action.schema.json),
  [`/schemas/support/runbook_packet.schema.json`](../../schemas/support/runbook_packet.schema.json),
  [`/schemas/support/escalation_packet.schema.json`](../../schemas/support/escalation_packet.schema.json),
  [`/schemas/support/support_pack_item.schema.json`](../../schemas/support/support_pack_item.schema.json),
  and
  [`/schemas/support/trace_capture_request.schema.json`](../../schemas/support/trace_capture_request.schema.json)
  — closed vocabularies the IA card and route records re-export
  rather than re-mint.

## Why this exists

The Support Center concept already names the recovery ladder, the
repair-transaction grammar, the bundle preview, and the object-
handoff packet. The support-intake contract already pins the seven
intake-bound capability cards and the four required intake surfaces.
What stayed implicit was the navigation surface a user actually meets
when they hit a failure state: which top-level modules exist, what
each module is allowed to do, how a symptom on the error, blocked-
action, crash-loop, update-failure, policy-denial, or transport-
failure surface routes into the narrowest safe diagnostic or repair
card, and how parity gaps between local-only, managed, self-hosted,
mirrored, and offline deployments stay visible rather than implicit.

Without this contract:

- the Support Center would invent surface-local module names that
  drift from the intake-bound capability registry;
- crash-loop, update-failure, and transport-failure routes would
  drift toward upload-first first actions instead of opening Doctor,
  crash triage, or field diagnostics locally;
- advisory and incident history would either be hidden behind a
  network fetch or invented as free-text review prose;
- field diagnostics would either auto-capture without consent or get
  buried under a generic "support" label; and
- managed, self-hosted, mirrored, and offline deployments would
  inherit the desktop-only happy path with no named alternate path.

This contract closes those gaps in one IA packet: nine top-level
modules, seven capability cards (Doctor, safe mode, bisect/
quarantine, support bundle, crash triage, guided repair, issue or
escalation handoff) re-aligned with two new IA-only modules
(advisory or incident history, field diagnostics), six symptom-
surface routes, and one five-row deployment-context parity block on
every card and every route.

## Top-level modules (closed at nine)

Every conforming Support Center exposes the same nine top-level
modules. Hiding any module — for example, jumping straight from
crash-loop detection to issue-handoff without exposing crash triage —
is non-conforming.

| Module (`support_center_module_class`) | Primary job | Aligned intake capability |
|---|---|---|
| `project_doctor` | Run versioned probes; render stable findings, severity, and confidence read-only. | `doctor` |
| `safe_mode` | Enter the published safe-mode runtime profile. | `safe_mode` |
| `bisect_or_quarantine` | Isolate one extension or runtime host at a time; quarantine known-bad components. | `extension_bisect` |
| `support_bundle` | Preview, redact, and locally export a support bundle. | `support_bundle` |
| `crash_triage` | Open the symbolicated crash envelope, exact-build, and symbol fidelity state. | `crash_triage` |
| `guided_repair` | Run preview → checkpoint → apply → verify with a named reversal class. | `guided_repair` |
| `issue_or_escalation_handoff` | Compose an object-issue handoff or escalation packet from the failed object. | `issue_escalation` |
| `advisory_or_incident_history` | Browse the local advisory and incident history; resume drafted handoffs. | (none — IA-only module) |
| `field_diagnostics` | Capture read-only transport, policy, trust, watcher, and reachability diagnostics. | (none — IA-only module) |

Adding a module is breaking and requires a new decision row in
`artifacts/governance/decision_index.yaml`. The two IA-only modules
(`advisory_or_incident_history`, `field_diagnostics`) declare
`aligned_intake_capability_class: null` so the picker capability
registry remains the source of truth for intake capabilities.

## Capability-card field contract

Every Support Center capability card MUST declare:

- **`purpose_summary`** — one reviewable sentence stating the
  module's purpose so the user does not have to read the contract
  docs to know what the module does.
- **`card_posture_class`** — one of `read_only_diagnosis`,
  `read_only_evidence_review`, `mutating_with_review_and_preview`,
  `mutating_local_disposable_only`, `export_only_local_first`, or
  `handoff_only_no_repair`. Diagnosis is read-only by default; only
  guided repair is `mutating_with_review_and_preview`; only safe
  mode and bisect/quarantine are `mutating_local_disposable_only`.
- **`confidence_class`** — one of `high`, `medium`, `low`,
  `unsupported`, re-exported from
  `schemas/support/doctor_finding.schema.json`. The card MUST NOT
  claim higher confidence than the underlying probe or evidence
  supports.
- **`primary_target_record_kind`** plus **`primary_target_schema_ref`**
  — the record kind the card hands the user to. Project Doctor
  hands a `doctor_finding_record`; safe mode and bisect/quarantine
  hand a `recovery_action_record`; the support bundle hands a
  `support_bundle_preview_item`; crash triage hands a
  `crash_envelope`; guided repair hands a `repair_preview_record`;
  issue or escalation handoff hands a
  `support_escalation_packet_record`; advisory or incident history
  hands an `advisory_record`; field diagnostics hands a
  `trace_capture_request`.
- **`evidence_sources`** — at least one
  `evidence_source_class` row. Every card MUST cite the source it
  reads. Cards never invent evidence; they quote one of
  `doctor_probe_output`, `support_pack_item`,
  `support_bundle_preview_item`, `recovery_action_record`,
  `repair_preview_record`, `repair_transaction_record`,
  `runbook_step_result`, `trace_capture_request`, `crash_envelope`,
  `object_handoff_packet`, `incident_workspace_packet`,
  `advisory_record`, `exact_build_identity_manifest`,
  `policy_decision_log`, or `transport_diagnostic_log`.
- **`local_actions`** — at least one entry. Every card MUST be
  reachable without a network or upload step. Tokens are drawn from
  the closed local-action vocabulary
  (`open_inspector_only`, `open_preview_with_review`,
  `save_packet_local_only`, `render_evidence_local_only`,
  `narrow_runtime_locally`, `rebuild_disposable_state_locally`,
  `capture_local_checkpoint`, `open_handoff_draft_local_only`).
- **`export_actions`** — at least one entry, drawn from the closed
  export-action vocabulary. Cards that produce no export MUST list
  `no_export_action` as the sole entry rather than omit the field.
- **`headless_availability_class`** plus
  **`headless_availability_summary`** — names how the card renders
  in CLI/headless and what alternate path is available when it is
  narrowed. Cards that declare `unavailable_with_named_alternate`
  MUST name an alternate path on the `cli_headless` parity row of
  the support-intake contract.
- **`policy_constraints`** — at least one entry. Cards with no
  policy constraint declare `no_constraint` explicitly so an absent
  value never silently implies an unconstrained surface. Tokens
  cover `managed_admin_required_for_apply`,
  `managed_admin_required_for_export`,
  `managed_admin_required_for_handoff`, `narrowed_by_trust_state`,
  `narrowed_by_offline_state`, `narrowed_by_mirror_only_state`,
  `narrowed_by_self_hosted_state`, and
  `no_silent_credential_access`.
- **`open_related_packet_actions`** — at least one entry. Every
  card names the next packet a reviewer can open instead of free-
  text follow-up copy. Tokens cover the canonical open targets
  (Doctor finding, bundle preview, recovery action, repair preview,
  runbook packet, object handoff, escalation packet, advisory
  record, incident workspace packet, trace capture, crash envelope,
  field-diagnostics report).
- **`deployment_context_parity_rows`** — exactly five rows, one per
  `deployment_context_class` (`local_only`, `managed`,
  `self_hosted`, `mirrored`, `offline`). Each row carries an
  `availability_class`, a `narrowed_when_summary`, and an
  `alternate_path_summary`. Rows that declare `unavailable` MUST
  name an alternate path so a desktop-only or managed-only happy
  path cannot be implied.
- **`aligned_intake_capability_class`** — names the intake-bound
  `support_capability_class` the card aligns with so the picker
  capability registry remains source of truth for intake. The two
  IA-only modules declare `null`.

## Symptom-to-module route contract

Every Support Center route record MUST declare:

- **`symptom_surface_class`** — one of `error_surface`,
  `blocked_action_surface`, `crash_loop_surface`,
  `update_failure_surface`, `policy_denial_surface`, or
  `transport_failure_surface`.
- **`primary_module_class`** — the module the route resolves to
  first. Routes MUST resolve to the narrowest safe diagnostic or
  repair surface (Project Doctor, safe mode, bisect/quarantine,
  crash triage, or field diagnostics) before any export-only or
  handoff-only module.
- **`default_first_action_class`** — the first action the route
  recommends, drawn from the closed default-first-action
  vocabulary. The vocabulary includes
  `refuse_route_no_upload_first` for routes whose only network-
  bound option would otherwise be an upload-first path; such
  routes defer to a local-only diagnosis or evidence card.
- **`secondary_module_classes`** — closed list of follow-on modules
  the route may suggest after the primary module. The list MUST
  NOT lead with a `mutating_with_review_and_preview` module unless
  the symptom surface is `crash_loop_surface` or
  `update_failure_surface`.
- **`no_upload_first_invariant`** — a block carrying
  `local_first_path_named: true` and
  `upload_required_for_first_action: false`. Both rules are
  enforced by the schema. A route whose first action requires an
  upload is non-conforming.
- **`deployment_context_parity_rows`** — exactly five rows, one
  per `deployment_context_class`.
- **`evidence_preserved_for_escalation`** — names the optional
  scenario family and the closed list of evidence-id classes the
  route preserves on the way to a card. The user does not restate
  their case if a later escalation packet picks up the route's
  evidence.

### Route → primary module table

Routes pinned in
[`artifacts/support/support_center_routes.yaml`](../../artifacts/support/support_center_routes.yaml):

| Symptom surface | Primary module | Default first action | Notes |
|---|---|---|---|
| `error_surface` | `project_doctor` | `open_project_doctor` | Doctor finding ids and exact-build identity travel with the route. |
| `blocked_action_surface` | `project_doctor` | `open_project_doctor` | Policy decision id is preserved alongside the Doctor finding. |
| `crash_loop_surface` | `crash_triage` | `open_crash_triage_card` | Crash envelope, exact-build identity, and extension inventory are preserved. |
| `update_failure_surface` | `advisory_or_incident_history` | `open_advisory_or_incident_history` | Advisory id and update session id are preserved. |
| `policy_denial_surface` | `project_doctor` | `open_project_doctor` | Policy decision id, finding id, and handoff packet id are preserved. |
| `transport_failure_surface` | `field_diagnostics` | `open_field_diagnostics` | Transport diagnostic log id is preserved; trace capture is opt-in. |

Adding a symptom surface or repurposing an existing surface is
breaking. Adding an extra route under an existing surface (for
example, a second `error_surface` route that resolves to a domain-
specific Doctor probe family) is additive-minor provided it follows
the no-upload-first invariant and re-uses the closed module set.

## Deployment-context parity (closed at five contexts)

Every capability card and every route MUST carry exactly one parity
row per `deployment_context_class`:

| Context | Default expectation |
|---|---|
| `local_only` | Reference surface for a user-installed individual baseline. |
| `managed` | Fleet under a managed admin or control plane; managed-admin sign-off MAY gate apply, export, or handoff but never silently auto-retargets. |
| `self_hosted` | User-administered control plane or runtime; self-hosted helpers and feeds render their freshness verbatim. |
| `mirrored` | Internal mirror or proxy of registries, docs, or update artifacts; mirror unreachability surfaces a typed state instead of failing silently. |
| `offline` | Air-gapped or temporarily disconnected device; network-bound paths narrow gracefully and local-only review remains available. |

Each row carries an `availability_class` token re-exported from
`schemas/support/doctor_finding.schema.json` (`available`,
`available_read_only`, `available_local_only`, `limited_available`,
`read_only`, `unavailable`), a `narrowed_when_summary`, and an
`alternate_path_summary`. A row that declares `unavailable` MUST name
an alternate path.

Rule: a card or route that implies a desktop-only or managed-only
happy path — for example, by setting `offline.availability_class =
unavailable` without naming an alternate path — is non-conforming.

## No upload-first invariant

Every Support Center route MUST resolve to a local-first first
action. The schema enforces:

- `no_upload_first_invariant.local_first_path_named: true` on every
  route;
- `no_upload_first_invariant.upload_required_for_first_action:
  false` on every route;
- `default_first_action_class` is one of `open_project_doctor`,
  `open_support_bundle_preview` (which is itself a local-first
  preview surface, not an upload),
  `open_safe_mode_card`, `open_bisect_or_quarantine_card`,
  `open_crash_triage_card`, `open_guided_repair_card`,
  `open_issue_or_escalation_handoff_draft` (a draft, not an
  upload), `open_advisory_or_incident_history`,
  `open_field_diagnostics`, or `refuse_route_no_upload_first`.

A route whose only network-bound option would otherwise be an
upload-first path MUST resolve to `refuse_route_no_upload_first` and
defer to a local-only diagnosis or evidence card. The escalation
packet's `delivery_path_prominence_class` pin is preserved across the
IA: `local_only_review` remains at primary equal prominence.

## Evidence preservation rule

Every Support Center route preserves at least one stable evidence-id
class on the way to a card so a later escalation packet can
reconstruct the case without re-asking the user. The
`evidence_preserved_for_escalation` block names:

- an optional `scenario_family_class` reference re-exported from the
  intake-bound vocabulary in
  `schemas/support/scenario_picker.schema.json` (or `null` when the
  symptom is family-agnostic);
- a non-empty list of preserved evidence-id classes (for example
  `doctor_finding_id`, `support_bundle_id`,
  `recovery_action_id`, `object_handoff_packet_id`,
  `crash_envelope_id`, `extension_inventory_id`,
  `advisory_record_id`, `update_session_id`, `policy_decision_id`,
  `exact_build_identity_id`, `transport_diagnostic_log_id`); and
- a one-sentence summary that names the preservation contract.

Rule: a route whose `preserved_evidence_id_classes` is empty is non-
conforming. The user MUST be able to navigate from a failure state to
a card and then later open an escalation packet without the IA
losing the route's evidence ids.

## Information-architecture index

The
[`artifacts/support/support_center_routes.yaml`](../../artifacts/support/support_center_routes.yaml)
file carries one
`support_center_information_architecture_record` at the bottom. The
index pins:

- the closed nine-module set;
- the closed six symptom-surface set;
- the closed five deployment-context set;
- one capability card ref per module (nine refs);
- at least one route ref per symptom surface (six refs);
- the no-upload-first invariant summary.

Tooling MUST read the index, not just the cards or routes, when
auditing IA conformance.

## Acceptance check

The three acceptance rules from the milestone spec resolve to:

| Acceptance rule | Where enforced |
|---|---|
| A user can navigate from a failure state to the narrowest safe diagnostic or repair surface without needing support docs first. | `default_first_action_class` plus the `secondary_module_classes` rule that forbids leading with a `mutating_with_review_and_preview` module unless the symptom is `crash_loop_surface` or `update_failure_surface`; `no_upload_first_invariant` enforces local-first; the route table covers all six symptom surfaces. |
| Capability cards make clear what can be inspected locally, what needs user consent, and what is unavailable on the current profile. | `card_posture_class`, `local_actions`, `export_actions`, `policy_constraints`, `headless_availability_class`, and `deployment_context_parity_rows` (closed at five rows) on every capability card; `aligned_intake_capability_class` aligns the IA card with the intake-bound capability registry. |
| Support Center routes preserve stable scenario family and evidence IDs for later escalation packets. | `evidence_preserved_for_escalation.preserved_evidence_id_classes` is non-empty on every route; `evidence_preserved_for_escalation.scenario_family_ref_optional` re-exports the intake-bound `scenario_family_class` vocabulary so the picker stays source of truth. |

## What this seed does not promise

- No live Support Center renderer, navigation engine, route engine,
  card host, advisory feed reader, or field-diagnostic capture
  runtime is wired up. The IA modules, capability cards, and routes
  are reviewable objects only.
- No new schema changes to
  `schemas/support/scenario_picker.schema.json`,
  `schemas/support/escalation_packet.schema.json`,
  `schemas/support/object_handoff_packet.schema.json`,
  `schemas/support/support_bundle.schema.json`,
  `schemas/support/recovery_action.schema.json`, or
  `schemas/support/repair_transaction.schema.json` are required.
  This packet projects onto those vocabularies and adds
  `support_center_capability_card_record`,
  `support_center_route_record`, and
  `support_center_information_architecture_record` in one new
  schema.
- Adding a capability card or route under an existing module or
  symptom surface is additive-minor provided the new row reuses the
  closed vocabularies. Repurposing a `support_center_module_class`,
  `symptom_surface_class`, `deployment_context_class`,
  `card_posture_class`, `local_action_class`, `export_action_class`,
  `headless_availability_class`, `policy_constraint_class`,
  `evidence_source_class`, `open_related_packet_action_class`, or
  `default_first_action_class` token is breaking and requires a new
  decision row in `artifacts/governance/decision_index.yaml`.
