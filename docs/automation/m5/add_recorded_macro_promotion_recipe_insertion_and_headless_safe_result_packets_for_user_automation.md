# Recorded-macro promotion, recipe insertion, and headless-safe result packets for user automation

This contract carries user-authored automation through its full lifecycle in one
export-safe truth packet whose unit of truth is a user-automation row. Shell,
docs, support export, and release tooling consume the packet directly instead of
re-describing a macro's promotion, insertion, or headless posture by hand.

- Packet type: `aureline_ai::add_recorded_macro_promotion_recipe_insertion_and_headless_safe_result_packets_for_user_automation::UserAutomationPacket`
- Schema: [`schemas/ai/add-recorded-macro-promotion-recipe-insertion-and-headless-safe-result-packets-for-user-automation.schema.json`](../../../schemas/ai/add-recorded-macro-promotion-recipe-insertion-and-headless-safe-result-packets-for-user-automation.schema.json)
- Support export: [`artifacts/ai/m5/add_recorded_macro_promotion_recipe_insertion_and_headless_safe_result_packets_for_user_automation/support_export.json`](../../../artifacts/ai/m5/add_recorded_macro_promotion_recipe_insertion_and_headless_safe_result_packets_for_user_automation/support_export.json)
- Fixtures: [`fixtures/ai/m5/add_recorded_macro_promotion_recipe_insertion_and_headless_safe_result_packets_for_user_automation/`](../../../fixtures/ai/m5/add_recorded_macro_promotion_recipe_insertion_and_headless_safe_result_packets_for_user_automation/)

This lane builds directly on the signed/shared recipe-pack lane and projects from
the recorded-macro / declarative-recipe contract in
[`docs/automation/recipe_and_macro_contract.md`](../recipe_and_macro_contract.md):
recorded macros are constrained to UI or editor state, a macro becomes shareable
only by being promoted to a declarative recipe, and both reuse the frozen command
preview, approval, and audit rules rather than inventing a parallel scripting
surface. It reuses the tool-gateway side-effect and approval vocabularies, the
routing-policy provider/locality mode vocabulary, the signed/shared recipe-pack
replay-preview, audit, and reversibility vocabularies, and the frozen M5
qualification, downgrade, and rollback-posture vocabularies — it does not fork a
parallel set of terms.

## The user-automation row

Each `UserAutomationRow` binds, for one recorded macro flowing through its
lifecycle:

| Field | Meaning |
| --- | --- |
| `macro_id`, `macro_label`, `macro_family_label`, `macro_version` | Identity, label, family, and version. |
| `capture_content_address` | Content address of the captured macro bytes, proving the exact capture a replay rode. |
| `capture_provenance` | Recorded from a user session, recorded from a replay session, imported from a shared recipe pack, or synthesized from a template. |
| `recorded_step_count` | Count of recorded steps in the capture (count only; no raw material). |
| `publisher_source_class`, `publisher_identity_ref` | Who published the automation and the signed identity record. |
| `resolved_mode` | Local, BYOK, managed, or enterprise-gateway mode the automation resolves to. |
| `promotion` | How the macro graduates into a reusable recipe. |
| `insertion` | How the promoted recipe inserts into a target surface. |
| `headless_result` | The headless-safe result the automation produces when run with no operator present. |
| `steps` | One disclosure per effect: side-effect class, headless safety, interactive preview, approval gate, audit, reversibility, and a review-safe label. |
| `claimed_qualification` | Stable, Beta, Preview, Experimental, Held, or Unavailable. |
| `downgrade_rules` | Closed set of triggers that narrow the claim. |
| `rollback_posture`, `rollback_verified` | Reversal posture for an automation-policy change and whether it was drilled. |
| `evidence_packet_refs` | Evidence backing a claimed automation. |

## Recorded-macro promotion

The `promotion` block is the macro-to-recipe graduation axis. A recorded macro
becomes a reusable, shareable recipe only through an explicit, audited promotion
gate — there is no silent forward:

- `recorded_pending_review` / `reviewed_held_not_promoted` — captured but not yet
  promoted; carries no recipe ref and may not claim Stable while pending review.
- `promoted_to_recipe` — graduated into a recipe; names the resulting recipe in
  `promoted_recipe_ref`, and a promotion that graduates a mutating macro carries a
  real promotion approval gate.
- `promotion_blocked_policy` / `promotion_blocked_tainted_capture` /
  `promotion_withdrawn` — blocked; narrows its claim out of every public lane.

An imported capture (`imported_from_shared_recipe_pack`) names the signed recipe
pack it rode in `recipe_pack_ref`, so its signing posture traces back to the
signed/shared recipe-pack lane.

## Recipe insertion

The `insertion` block records how a promoted recipe inserts into a target — the
composer prompt, a workspace document, the command palette, the automation queue,
or a headless job. Insertion is preview-first: a mutating automation previews
before it commits into a target surface. A headless target (the automation queue
or a headless job) cannot rely on a recurring per-session or per-invocation
interactive prompt that can never appear with no operator present.

## Headless-safe result packets

Headless safety is the load-bearing guarantee. Every step's `headless_safety`
discloses how it behaves with no interactive operator present:

- `headless_safe_inspect_only` — inspect-only; always safe to run headless.
- `headless_safe_preauthorized_policy` — mutating, but pre-authorized by an
  explicit policy grant that is itself gated and audited.
- `headless_deferred_to_interactive` — deferred for later interactive review; not
  executed headless.
- `headless_blocked_fail_closed` / `headless_denied_by_policy` — held back so the
  step never silently executes.

An irreversible external publish can never run unattended headless. The
`headless_result` block is content-addressed, reconciles its `steps_total`,
`steps_completed`, `steps_deferred`, and `steps_blocked` counts against the
disclosed steps, and its `state` must agree with whether any step deferred or
blocked. A headless run that executed a mutating step is durably, exportably
audited.

## Invariants enforced by validation

`UserAutomationPacket::validate` returns a closed set of typed violations.
Recorded-macro promotion, recipe insertion, and headless replay follow the same
preview, policy, and audit rules as first-party commands:

- Every automation carries a content-addressed capture, advertises at least one
  step disclosure, gives each disclosure a label, and discloses no side-effect
  class twice.
- A publisher class that requires identity carries a publisher identity ref; an
  imported capture names the signed recipe pack it rode.
- A mutating step previews before it applies interactively, carries a real
  approval gate, and is audited. A declared reversibility agrees with the
  side-effect class.
- An inspect-only step is headless-safe inspect-only; a mutating step never claims
  the inspect-only headless class; an irreversible external publish never runs
  unattended headless; a step pre-authorized to run headless still carries a gate
  and is audited.
- A promoted macro names its recipe and an unpromoted one names none; promoting a
  mutating macro carries a real promotion gate.
- Insertion is preview-first for a mutating automation, and a headless-target
  insertion does not rely on a recurring interactive prompt.
- The headless result block is content-addressed, reconciles its step counts, has
  a state that agrees with its deferred/blocked counts, and is externally audited
  when a mutating step ran headless.
- A blocked promotion may not keep a Stable, Beta, or Preview claim; a macro
  pending promotion review may not claim Stable.
- A claimed automation carries evidence refs, has a verified rollback path when
  its posture can be reversed, and carries a closed downgrade rule set that
  includes the proof-stale and provider-unavailable triggers and only narrows
  below the claimed qualification.
- The packet carries a proof-freshness block so stale proof automatically narrows
  claimed automations.

## Boundary

The packet carries content addresses, classes, and review-safe labels only. Raw
shell fragments, raw filesystem paths, raw endpoint URLs, credential bodies, raw
API keys, OAuth tokens, and raw captured UI or editor buffer bytes never cross
this boundary; `validate` rejects export-safe JSON that embeds raw automation or
credential material.

## Regenerating the artifacts

The checked-in support export and fixtures are produced by the in-crate builder
and can be regenerated deterministically:

```bash
cargo run -p aureline-ai --example dump_user_automation_packet -- support
cargo run -p aureline-ai --example dump_user_automation_packet -- fixture
```
