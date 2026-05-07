# Permission prompt state machine, remembered decisions, and reapproval triggers

This document publishes the shared state machine for permission prompts:
how prompts are classified, how approvals and denials can be remembered
without silently widening scope, how expiry and revocation behave, and
how reapproval triggers are emitted so reviewers can reconstruct:

- why a prompt appeared,
- what it covered, and
- why it reappeared later.

This document exists to keep prompt semantics stable across desktop UI,
CLI/headless automation review, and export/support packets. Prompt copy
and layout may vary by surface, but the underlying packet vocabulary and
state transitions are shared.

## Companion artifacts

- [`/schemas/policy/permission_prompt_event.schema.json`](../../schemas/policy/permission_prompt_event.schema.json)
  — export-safe prompt event + reapproval trigger packet.
- [`/fixtures/policy/permission_prompt_cases/`](../../fixtures/policy/permission_prompt_cases/)
  — worked cases covering a destructive local workflow, a publish-capable
  workflow, and embedded-surface escalation when embedded approval is
  disallowed.

## Composes with (does not replace)

- [`/docs/ux/trust_prompt_contract.md`](../ux/trust_prompt_contract.md) and
  [`/schemas/trust/prompt_request.schema.json`](../../schemas/trust/prompt_request.schema.json)
  — prompt request anatomy (what/why/changes/works-if-denied), capability
  group vocabulary, and deny/degrade vocabulary.
- [`/docs/trust/capability_sheet_contract.md`](../trust/capability_sheet_contract.md) and
  [`/schemas/trust/capability_sheet.schema.json`](../../schemas/trust/capability_sheet.schema.json)
  — durable review language, transitive scope disclosure, remembered
  approvals list, and revocation-route vocabulary.
- [`/docs/ux/shell_interaction_safety_contract.md`](../ux/shell_interaction_safety_contract.md) and
  [`/schemas/ux/interaction_safety.schema.json`](../../schemas/ux/interaction_safety.schema.json)
  — consequence class, grant scope, authority-renewal triggers, and prompt
  open/decision audit event ids on protected shell surfaces.
- [`/docs/governance/runtime_authority_contract.md`](../governance/runtime_authority_contract.md) and
  [`/schemas/governance/authority_ticket.schema.json`](../../schemas/governance/authority_ticket.schema.json)
  — runtime authority tickets, drift invalidation, and the rememberable
  rule + renewable short-lived ticket model.
- [`/schemas/integration/approval_ticket.schema.json`](../../schemas/integration/approval_ticket.schema.json)
  and [`/schemas/integration/browser_handoff_packet.schema.json`](../../schemas/integration/browser_handoff_packet.schema.json)
  — provider-plane approval tickets and typed browser handoffs.
- [`/docs/network/transport_permission_matrix.md`](../network/transport_permission_matrix.md)
  — network permission classes and audit spine for egress-class decisions.
- [`/docs/ux/embedded_surface_boundary_cards.md`](../ux/embedded_surface_boundary_cards.md) and
  [`/schemas/ux/embedded_boundary_card.schema.json`](../../schemas/ux/embedded_boundary_card.schema.json)
  — embedded-surface action partitions; embedded bodies may request but
  must not claim final authority over native-reserved approvals.

If this document disagrees with the contracts above, the contract above
wins and this document + schema + fixtures update in the same change.

## Scope

Covered:

- prompt classification for local mutation, external mutation, credential
  projection, privileged debug/attach, network egress, provider publish,
  and browser handoff;
- remembered approvals and remembered denials (“denial memory”) with
  expiry and narrowing rules; and
- reapproval triggers (what changed) that force reprompt with delta.

Out of scope:

- enforcement implementation (policy engine, ticket minting service, UI);
- credential storage and secret material handling beyond the declared
  redaction boundaries in the composed contracts.

## Prompt classes (frozen)

Each permission prompt MUST be classifiable into exactly one
`prompt_class`. The prompt class is not UI; it is a typed authority
category used by audit, support export, and headless review.

| Prompt class | Typical authority | Minimum required fields (in addition to shared prompt anatomy) |
|---|---|---|
| `local_mutation` | local workspace edits (including destructive) | `side_effect_class`, `consequence_class`, recovery posture, target scope |
| `external_mutation` | provider-visible mutation that is not an irreversible publish | provider actor + approval ticket linkage + target scope |
| `credential_projection` | projecting a credential handle into a tool/run/build/clipboard | credential projection posture + revocation/expiry disclosure |
| `privileged_debug_or_attach` | debugger attach, remote attach, privileged inspection | host boundary cues + route class + step-up posture when required |
| `network_egress` | egress beyond baseline posture (extension/AI/tool/docs/telemetry) | `permission_class`-aligned disclosure + egress/route context + deny/degrade option |
| `provider_publish` | irreversible publish (release publish, registry publish, irreversible provider action) | irreversible consequence disclosure + “no silent reuse” posture |
| `browser_handoff` | leaving the product via a typed browser handoff | browser-handoff packet ref + origin/return disclosure + replay/expiry posture |

### Shared required anatomy

Regardless of class, every prompt MUST have enough typed fields to answer:

- what is requested,
- who is asking and who owns the authority,
- why it is needed,
- what changes if allowed,
- what still works if denied,
- which scope is requested and how long it lasts,
- where the grant/denial can be inspected or revoked later, and
- what (if anything) can be copied/exported from the prompt.

Those slots are carried by the prompt request and capability sheet
records; the prompt event packet links them and adds state-machine truth.

## State machine (prompt lifecycle)

The lifecycle is modeled as events over a stable `permission_prompt_id`.

### Events

1. `permission_prompt_opened`
   - A prompt becomes visible or is made available for headless review.
   - If this is a renewal or widening, the event MUST carry non-empty
     reapproval triggers and a non-null prior-grant reference.
2. One of:
   - `permission_prompt_granted`
   - `permission_prompt_denied`
   - `permission_prompt_cancelled`
   - `permission_prompt_expired`
   - `permission_prompt_revoked_before_commit`
3. `permission_prompt_reprompt_required` (optional, but required when a
   remembered decision is invalidated or expires and a reprompt is about
   to be shown)
   - Emitted before the UI reprompts so audit consumers can prove the
     reprompt happened and why.

### Remembered decisions (approvals and denials)

Remembering a decision never creates an unlimited bearer credential.
The durable memory is a narrow rule; each use still mints a short-lived
ticket bound to the current context.

Rules:

- Remembered approvals/denials MUST be bound to:
  - actor identity (who is asking / on whose behalf),
  - target scope,
  - route/host boundary posture when applicable,
  - side-effect class and consequence class, and
  - policy context (policy source + policy epoch) and trust posture.
- A remembered decision MAY be narrowed (smaller scope, fewer side
  effects). It MUST NOT be silently widened.
- Remembered decisions MUST have an expiry posture. “Forever” is
  forbidden for destructive, networked, provider-backed, secret-bearing,
  or publish-class prompts.
- Denial memory is permitted only when it keeps the system productive
  (e.g., “continue local-only”, “draft saved locally”) and only when the
  reprompt/expiry behavior is explicit.

### Reapproval triggers (no silent widening)

If any reapproval trigger fires, the next prompt MUST reopen with the
delta (not a restated full envelope), and MUST NOT spend an old ticket.

Triggers are computed as a set; common examples:

- target changed,
- host boundary changed,
- route changed,
- actor changed,
- policy source or policy epoch changed,
- context/basis snapshot drifted or became stale,
- grant expired or was revoked, and
- side effects widened (side-effect class or consequence class widened).

Each prompt-open event carries the trigger set so support exports and
headless reviewers can reconstruct why the prompt reappeared.

## Embedded approval escalation (embedded approval disallowed)

Embedded surfaces (extension-hosted views, embedded account/marketplace,
embedded dashboards) may request an action, but they must not present an
embedded “native-reserved” approval UI that claims final authority.

When a request originates from an embedded surface:

- the embedded surface action partition is `embedded_request_only`, and
- the actual prompt is escalated to a host-native prompt surface (or to
  a typed browser handoff / platform-mandated host flow when required).

Cases exercising this escalation are published under the fixture corpus.

