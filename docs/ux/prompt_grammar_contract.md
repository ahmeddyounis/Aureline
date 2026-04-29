# Consequence-Bearing Prompt Grammar Contract

This document freezes the copy grammar for prompts that ask a user to
commit to a destructive, trust-changing, approval-bearing, consent,
publish, promote, rollback, or revoke outcome. It complements the prompt
anatomy contract by defining how titles, body lines, buttons, and
follow-up actions name the object, scope, consequence, and recovery path.

Companion artifacts:

- [`/artifacts/ux/prompt_family_matrix.yaml`](../../artifacts/ux/prompt_family_matrix.yaml)
  - machine-readable prompt-family grammar, required slots, button rules,
    material-change triggers, and fixture coverage.
- [`/fixtures/trust/prompt_grammar_cases/`](../../fixtures/trust/prompt_grammar_cases/)
  - worked grammar cases for destructive delete, widened-retention consent
    renewal, approval-expiry renewal, and promote-vs-rollback review.

This contract composes with:

- [`/docs/ux/trust_prompt_contract.md`](./trust_prompt_contract.md)
  for request anatomy, authority owner, policy lock, side-effect envelope,
  denial/degrade, and copy/export posture.
- [`/docs/ux/shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md)
  for consequence class, preview/apply/revert, recovery class, authority
  renewal, and required visible fields.
- [`/docs/ux/dialog_sheet_contract.md`](./dialog_sheet_contract.md)
  for surface selection, consequence visibility, safe default focus, and
  nested-overlay policy.
- [`/docs/ux/disabled_reason_grammar.md`](./disabled_reason_grammar.md)
  for blocked, degraded, stale, and preview-required explanation grammar.

Out of scope: dialog layout, visual styling, animation, and component
chrome. A modal, sheet, review surface, CLI prompt, or headless trace may
render differently, but it must preserve the same grammar slots.

## Scope

This grammar applies whenever object, scope, consequence, authority, or
recovery is material to user trust or product state:

| Prompt family | Examples |
| --- | --- |
| Destructive action | delete, remove, purge, detach, discard, clear, overwrite |
| Trust or policy change | trust workspace, widen extension capability, change policy source |
| Approval request | request approval, renew expired approval, approve changed scope |
| Consent renewal | renew recording, retention, export, telemetry, collaboration, or provider consent |
| Publish review | publish package, review, comment, release, docs pack, support packet |
| Promote review | promote candidate to a wider ring, channel, mirror, or support state |
| Rollback review | roll back channel pointer, repin prior version, restore from checkpoint |
| Revoke review | revoke grant, yank artifact, disable tunnel, remove active access |

These verbs are not interchangeable. `Publish` makes an object available
at a target. `Promote` widens an already-published or staged object to a
new channel or audience. `Rollback` changes state back to a prior known
target. `Revoke` removes or disables active access or availability. A
surface that collapses these into `Confirm` or `Continue` is
non-conforming.

## Required Grammar Slots

Every consequence-bearing prompt must render these slots before the user
can choose an outcome:

| Slot | Required content |
| --- | --- |
| `action_verb` | The exact verb: delete, trust, request, renew, publish, promote, rollback, revoke, or a narrower domain verb. |
| `object_label` | The named object or count and kind of objects. Avoid pronouns when the target can drift. |
| `scope_label` | Workspace, workset, account, provider, channel, route, data class, approver set, retention window, or audience affected. |
| `consequence_label` | What changes if the primary action succeeds, including shared, external, durable, or irreversible effects. |
| `recovery_label` | Exact undo, checkpoint, export-before-change, compensating action, or no exact recovery. |
| `authority_label` | Who owns the decision when trust, policy, approval, consent, provider, or admin scope matters. |
| `deny_or_keep_label` | What remains available when the user denies, cancels, keeps local, or chooses the safer route. |
| `details_label` | The open-details, preview, evidence, policy, retention, approver, or impact route. |

If any required slot is unknown, the prompt must render as blocked,
details-only, or preview-required. It must not fall back to an `Are you
sure?` question.

## Title Grammar

Use one of these shapes:

```text
{Action verb} {object_label}?
{Action verb} {object_label} for {scope_label}?
{Renew|Request} {authority_label} for {object_label}?
{Promote|Rollback|Revoke} {object_label} {target_or_scope_label}?
```

Examples:

- `Delete 3 saved worksets?`
- `Renew recording consent for 90-day retention?`
- `Request release-manager approval for staging deploy?`
- `Promote 1.13.0-rc.1 to Stable?`
- `Roll back Stable to 1.12.4?`
- `Revoke public tunnel for port 3000?`

Forbidden title shapes on product-owned consequential prompts:

- `Are you sure?`
- `Continue?`
- `OK?`
- `Confirm?`
- `Proceed?`
- `This action cannot be undone` as the whole title

## Body Grammar

Body copy must use this order unless a narrower surface contract requires
more detail:

1. **Object and scope:** name the object, count, target, audience, or
   owner set.
2. **Consequence:** name the material state change in concrete terms.
3. **Recovery:** name the undo, checkpoint, export, rollback, revoke, or
   no-recovery truth.
4. **Authority or consent:** when relevant, name policy owner, approver
   set, prior grant, expiry, retention, exportability, or changed target.
5. **Deny path:** name what remains available if the user does not
   choose the primary action.

Body copy must not rely on vague warnings such as `This may affect your
data` when the affected data class, retention window, audience, or
recovery path is known.

## Button Grammar

Primary actions must describe the resulting state. They must not be
generic acknowledgements.

| Situation | Primary label shape | Non-conforming |
| --- | --- | --- |
| Delete | `Delete {count_or_object}` | `OK`, `Continue`, `Yes`, `Confirm` |
| Trust/policy grant | `Trust {scope}`, `Grant {capability} for {duration}` | `Allow`, `Accept` |
| Approval request | `Request {approver_scope} approval`, `Renew {scope} approval` | `Submit`, `Continue` |
| Consent renewal | `Renew consent for {retention_or_recording_scope}` | `Agree`, `Continue` |
| Publish | `Publish {object} to {target}` | `Publish`, `Confirm` when target is material |
| Promote | `Promote {object} to {channel_or_audience}` | `Promote` when target is omitted |
| Rollback | `Roll back {target} to {prior_state}` | `Confirm rollback` |
| Revoke | `Revoke {grant_or_route}` | `Disable`, `Confirm` when active access is material |

Secondary actions must also name outcome where the outcome is not a pure
cancel:

- `Keep local draft`
- `Stay in restricted mode`
- `Export before delete`
- `Open retention policy`
- `Open approver matrix`
- `Keep in Preview`
- `Export impact summary`

`Cancel` is allowed only when it means no product state changes. If
dismissing preserves a draft, blocks a workflow, or keeps a grant active,
use a specific label such as `Keep draft blocked`, `Leave grant active`,
or `Continue read-only`.

## Same-Surface Action Obligations

Preview, export, rollback, and open-details actions must appear on the
same prompt surface when they materially affect the decision:

| Trigger | Required same-surface action |
| --- | --- |
| Destructive action with exportable data | Export or copy-before-change action. |
| Multi-object, generated, protected, or external mutation | Preview or open diff/review action. |
| Recoverable durable action | Open checkpoint, rollback, restore, or recovery details. |
| External shared publish, promote, rollback, or revoke | Export impact summary and open evidence/attestation route. |
| Changed trust, policy, approver, retention, recording, export, or publish target | Open details for the changed axis. |
| No exact recovery | Open impact details before the destructive primary action is enabled. |

Details do not replace required visible slots. They deepen the evidence
or policy trail after the prompt has already named object, scope,
consequence, and recovery.

## Material-Change Language

Some changes require stronger wording and cannot reuse a prior generic
prompt.

### Recording, Retention, and Exportability

Consent renewal must say what widened:

- recording changed from off to on, or from local to shared;
- retention duration increased;
- retention owner changed from local user to managed policy;
- exportability changed from metadata-only to raw, sanitized, support,
  admin, provider, or public export;
- audience changed from local user to workspace, organization, provider,
  public link, or support staff.

Use concrete language:

```text
Recording will be retained for {new_window} by {authority_label}.
This is wider than the prior {old_window} consent.
{export_scope} can be exported as {representation_class}.
```

### Approver Scope

Approval renewal must say whether approval expired, was revoked, or no
longer covers the current target. It must also name changed approver
scope:

```text
The prior approval expired at {relative_or_absolute_time}.
The current action now requires {new_approver_scope}; the prior
{old_approver_scope} approval no longer admits this change.
```

### Publish Target

Publish prompts must name destination, visibility, mutability, auth
source class, and dry-run state when applicable. If target changes from
private to public, mirror-only to production, internal to external, or
preview to stable, the title or first body line must name the new target.

### Promote, Rollback, and Revoke

Promotion prompts must name the wider channel or audience and the
evidence freshness that justifies widening. Rollback prompts must name
the current target and prior state. Revoke prompts must name the active
grant, route, artifact, or access being disabled and whether historical
records remain inspectable.

## Non-Conformance

A prompt is non-conforming when it:

- uses `Are you sure?`, `Continue`, `OK`, `Yes`, `No`, `Accept`,
  `Submit`, or `Confirm` as the only meaningful title or primary action;
- omits object, scope, consequence, or recovery when those values are
  material to trust or recovery;
- treats publish, promote, rollback, and revoke as synonyms;
- renews consent without naming widened recording, retention,
  exportability, audience, or policy owner;
- renews approval without naming expiry, changed approver scope, or
  changed target scope;
- changes publish target, visibility, mutability, or auth source without
  saying so before the primary action is available;
- hides preview, export, rollback, impact, or details actions that are
  necessary to understand the decision; or
- uses different copy semantics across UI, CLI, support export, docs, or
  audit/evidence projections for the same prompt record.
