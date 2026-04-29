# Command invocation result and cross-surface parity contract

This document freezes the command invocation result contract every
launch-bearing surface uses after it asks the command router to do
work. It complements the command descriptor, shareability, palette,
menu, keybinding, CLI, recipe, AI, and companion contracts by pinning
one result packet, one rollback-handle shape, and one parity expectation
shape for desktop, CLI, automation, AI, voice, and companion clients.

Machine-readable companions:

- [`/schemas/commands/result_packet.schema.json`](../../schemas/commands/result_packet.schema.json)
  defines `command_result_packet_record`.
- [`/schemas/commands/rollback_handle.schema.json`](../../schemas/commands/rollback_handle.schema.json)
  defines `command_rollback_handle_record`.
- [`/schemas/commands/parity_expectation.schema.json`](../../schemas/commands/parity_expectation.schema.json)
  defines `command_parity_expectation_record`.
- [`/fixtures/commands/invocation_result_cases/`](../../fixtures/commands/invocation_result_cases/)
  contains worked result, rollback, and parity fixtures.

This contract projects from the product requirement that every
meaningful action has one command graph and that automation, AI, and
other non-desktop paths remain previewable, attributable, undoable, and
policy-governed. It also projects from the existing descriptor,
shareability, CLI, recipe, AI evidence, interaction-safety, companion,
notification, support, and interface-lifecycle contracts. If those
owning contracts disagree with this document, the owner wins and this
document updates in the same change.

## Scope

Frozen here:

- the minimum invocation-session fields every result packet quotes:
  invocation id, attempt id, issuing surface, authority class,
  canonical command id, command revision, canonical verb, alias used,
  argument provenance, context refs, enablement decision, preview
  posture, approval posture, and execution intent;
- the result packet body: outcome code, warning codes, error codes,
  created artifact refs, notification refs, activity refs, rollback
  handle ref, checkpoint refs, evidence refs, and export posture;
- rollback-handle records that describe recovery without granting
  ambient authority;
- cross-surface parity rows for menu / button / context menu,
  palette / keybinding / leader, CLI / recipe / AI / voice, and
  browser / mobile companion surfaces; and
- alias, deprecation, and migration rules that preserve automation and
  import compatibility without allowing stable command ids to be
  silently reassigned.

Out of scope:

- implementing the live command router or every surface;
- implementing full rollback mechanics, checkpoint storage, or
  provider-specific revert APIs;
- designing user-facing notification copy; and
- defining raw result bodies. This contract carries refs to artifacts,
  evidence, activity, notifications, and exports, not raw bytes.

## Invocation Session Minimums

Every `command_result_packet_record` contains an `invocation` block.
It is the minimum cross-surface invocation session, regardless of
whether the caller was a menu item, a keybinding, a CLI verb, a recipe
step, an AI tool, a voice phrase, or a companion handoff.

| Field | Rule |
| --- | --- |
| `invocation_session_id` | Stable across preview, approval, apply, retry, rollback, and follow-up background packets. |
| `invocation_attempt_id` | Unique per attempt inside the session. A retry gets a new attempt id, not a new session. |
| `issuing_surface` | Names the actual surface. Unknown surfaces deny instead of defaulting to palette. |
| `authority_class` | Names who initiated the action. AI, recipe, companion, extension, and remote authority may not collapse to local user authority. |
| `canonical_command_id` | The stable command id from the descriptor. It is never replaced by a label, alias, command row id, or recipe step id. |
| `command_revision_ref` | The descriptor revision the invocation saw. Revision drift reopens preview / approval. |
| `canonical_verb` | The dotted machine verb used by CLI, AI, and recipe surfaces. |
| `alias_used` | Records canonical use, legacy command ids, CLI aliases, AI tool handles, keybinding targets, voice phrases, companion tokens, and imported bridge aliases. |
| `argument_provenance_map` | One entry per typed argument slot, recording whether the value was typed, selected, inferred, supplied by CLI, proposed by AI, supplied by a recipe, captured by voice, handed off by companion, or pinned by policy. |
| `context_refs` | Opaque refs for focus, selection, workspace, trust state, execution context, scope filter, basis snapshot, and related context objects. |
| `enablement_decision` | The same enabled / disabled / hidden decision every surface would render, with typed reason and repair hook when non-enabled. |
| `preview_posture` | The descriptor preview class, whether preview was shown, and the preview record ref when present. |
| `approval_posture` | The descriptor approval class, the current approval state, and the approval ticket ref when present. |
| `execution_intent` | Query, preview-only, apply-after-preview, apply-with-approval, direct trusted path, rollback, dry run, background scheduling, or cancellation. |

The result packet does not replace the descriptor-owned
`invocation_session_packet_record`; it is the exported result envelope
that quotes the invocation minimums and adds outcome, evidence,
rollback, and parity linkage.

## Result Packet

Every invocation that reaches the command router emits exactly one
result packet for the attempt. Long-running commands may emit a packet
with `scheduled_pending_background` and later emit another packet for a
follow-up attempt under the same invocation session id.

Required result semantics:

| Field | Rule |
| --- | --- |
| `outcome_code` | Uses the shared outcome vocabulary: succeeded, succeeded with warnings, denied by enablement / preview / approval, cancelled, failed with typed error, rolled back, partially applied, or scheduled pending background. |
| `warning_codes` | Required when the outcome is `succeeded_with_warnings`; allowed for degraded-but-successful paths such as deprecated alias use, redacted export, limited rollback window, delayed activity projection, or background completion. |
| `error_codes` | Required for typed failures and denial outcomes. Error codes are machine-readable; surfaces may localize prose only after preserving the code. |
| `created_artifact_refs` | Ordered refs for created, modified, deleted, journaled, audited, preview, approval, browser-handoff, evidence, or rollback artifacts. |
| `notification_refs` | Refs to notification events and delivery posture. A command does not invent notification semantics; it links the notification contract. |
| `activity_refs` | Refs to history rows, status items, audit rows, support timelines, AI run history, or companion timelines. |
| `rollback_handle_ref` | Explicit rollback posture plus handle id when one exists. A null handle is only valid with a declared non-rollback posture. |
| `checkpoint_refs` | Checkpoints or snapshots created before or during the invocation. Mutating commands that promise rollback must cite at least one relevant checkpoint or rollback ticket. |
| `evidence_refs` | Evidence rows required by the descriptor result contract plus any preview, approval, AI, support, checkpoint, notification, activity, or rollback evidence emitted by this attempt. |
| `export_posture` | Export class, redaction class, export review ref, portable-profile eligibility, and support-bundle eligibility. |

Result packets are not stable raw output. Machine consumers read the
schema-bound packet and then resolve referenced artifacts through their
own owning contracts. Raw stdout, file paths, URLs, prompts,
credentials, provider payloads, and source bytes stay behind the
appropriate artifact, evidence, support, or redaction boundary.

## Rollback Handles

A rollback handle is evidence that recovery is possible, not a token
that can spend authority on its own. Spending a rollback handle re-runs
the same trust, policy, permission, preview, approval, credential,
execution-context, freshness, and basis-drift checks as the original
command.

`command_rollback_handle_record` freezes:

- identity: rollback handle id, result packet id, invocation session
  id, canonical command id, command revision, and canonical verb;
- rollback kind: inverse operation, checkpoint restore, compensating
  command, external provider revert, migration rollback, or
  support-guided recovery;
- state: available, pending, completed, expired, blocked by policy,
  unsafe after drift, external authority required, or unsupported for
  the outcome;
- lifetime: session, bounded time window, checkpoint GC, release-line
  support window, or support-retained metadata;
- checkpoint refs and affected artifact refs;
- spend requirements, including rollback command refs, preview and
  approval requirements, authority ticket refs, and no-bypass guards;
  and
- evidence, policy context, redaction class, and minted time.

Partially applied commands must link both applied and unapplied
artifacts in the result packet and must either publish an available
rollback handle or explicitly declare manual recovery / unsupported
rollback posture.

## Parity Expectation Rows

Parity is checked against result semantics, not just labels or shortcut
placement. A `command_parity_expectation_record` defines the expected
result behavior for a canonical command across four surface families:

| Surface family | Covered surfaces | Required parity |
| --- | --- | --- |
| `menu_button_context_menu` | Global menu, context menu, toolbar button | Emits the same result packet shape and preserves enablement, preview, approval, rollback, evidence, notification/activity, and export posture. |
| `palette_keybinding_leader` | Command palette, keybinding chord, leader sequence | Resolves the same command id and revision, records the alias or key target used, and cannot skip preview/approval because the user used a faster route. |
| `cli_recipe_ai_voice` | CLI, recipe, AI tool, voice surface | Uses the same command id, argument schema, provenance map, result packet, rollback/evidence refs, and typed deny/warn codes. Voice must confirm the resolved command before dispatch. |
| `browser_mobile_companion` | Browser companion, mobile companion | Read-mostly or handoff-first. Mutating attempts must route through the desktop / approval / handoff envelope and cannot bypass policy, trust, preview, approval, freshness, or redaction rules. |

Every parity row carries:

- `coverage_status`: claimed, explicitly narrowed, read-only mirror
  only, denied by client scope, or unsupported pending surface contract;
- `result_packet_required = true`;
- `invocation_session_minimum_required = true`;
- expected outcome, warning, and error codes;
- expected result contract classes and required evidence refs;
- rollback handle posture;
- export posture; and
- schema-pinned no-bypass guards.

Explicit narrowing is conforming only when it is visible in the parity
row. A companion surface that is read-only, an AI surface denied for a
high-blast mutation, or a CLI path excluded by client scope must publish
the same typed reason other surfaces can inspect.

## No-Bypass Rules

Every result packet and parity row pins these guards to `true`:

- trust revalidation required;
- policy revalidation required;
- permission prompt revalidation required;
- preview path preserved;
- approval path preserved;
- credential broker revalidation required;
- execution context revalidation required;
- freshness floor revalidation required;
- capability class may not widen; and
- result schema may not be replaced.

Parity expectation rows add two future-surface guards:

- companion mutation must hand off to the desktop / approval envelope;
- voice invocation must confirm the resolved command before dispatch.

Consequences:

- A toolbar button and a CLI verb for the same command cannot return
  different success semantics.
- An AI tool handle cannot apply a preview-required command without
  emitting preview and approval evidence.
- A recipe cannot reuse an old approval ticket after policy, trust,
  target, provider, or descriptor revision drift.
- A companion can request approval, capture offline triage, or hand off
  a scoped action, but it cannot become a private mutation plane.
- A voice phrase can resolve to a command alias, but the result packet
  records the alias and the confirmed canonical command id.

## Alias, Deprecation, And Migration Rules

Stable command ids are never silently reassigned. Rename, replacement,
or semantic split requires a new canonical command id or descriptor
revision plus typed alias / migration metadata.

Rules:

- New keybindings, recipes, AI tool registrations, CLI docs, and
  automation examples target canonical command ids unless a stable
  alias is explicitly admitted.
- Every invocation records `alias_used`, even when the caller used the
  canonical id. Deprecated aliases therefore remain visible in support,
  compatibility, and migration telemetry.
- Deprecated aliases require a support-window ref and a migration trace
  ref before they may resolve.
- Retired aliases may remain discoverable in migration bridge cards,
  support export, compatibility reports, or explicit lookup, but they
  may not silently dispatch as if current.
- Imported keymaps, settings, recipes, and command aliases preserve the
  source-system ref and the canonical resolution ref.
- Release notes are not sufficient deprecation metadata. Machine-
  readable alias rows, compatibility notes, and migration traceability
  must ship with the change.
- A deprecated alias may either deny with a typed migration reason or
  succeed with `deprecated_alias_used`; in both cases the result packet
  records the alias, successor, support-window, and migration refs.

## Fixture Coverage

The invocation-result fixture corpus exercises:

- a palette invocation that succeeds, creates artifacts, emits
  notification/activity refs, and publishes a rollback handle;
- an AI-initiated publish path that opens approval and schedules
  background work without bypassing preview or evidence capture;
- a CLI invocation through a deprecated alias that succeeds with a
  typed warning and migration trace;
- a companion mutation attempt that denies with the same policy/trust
  result semantics as desktop; and
- a parity expectation row covering all four required surface families.

