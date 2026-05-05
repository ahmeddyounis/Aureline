# UI copy contract: action labels, error messages, and AI copy guardrails

This contract freezes user-visible control labels and error language so UI,
CLI-adjacent projections, exports, support bundles, and AI surfaces do not invent
inconsistent or overclaiming copy.

It is normative. If this document disagrees with a source product, architecture,
UI/UX, or design-system specification, the source wins and this contract updates
in the same change. If a downstream surface invents looser copy, this contract
wins and the surface is non-conforming.

Companion artifacts:

- [`/artifacts/copy/ui_copy_lint_rules.yaml`](../../artifacts/copy/ui_copy_lint_rules.yaml)
  - machine-readable lint rules and fail gates for action labels, error messages,
    and AI copy.
- [`/schemas/copy/error_message.schema.json`](../../schemas/copy/error_message.schema.json)
  - boundary schema for the four-part error-message structure.
- [`/fixtures/copy/ui_copy_cases/`](../../fixtures/copy/ui_copy_cases/)
  - worked fixtures spanning trust prompts, settings rows, banners, toasts, AI
    cards, and help/error surfaces.

This contract composes with, and does not replace:

- [`/docs/copy/naming_and_state_label_contract.md`](./naming_and_state_label_contract.md)
  and [`/artifacts/copy/controlled_glossary.yaml`](../../artifacts/copy/controlled_glossary.yaml)
  for verb-first, outcome-specific action labels and controlled state vocabulary.
- [`/docs/ai/ai_copy_guardrails_contract.md`](../ai/ai_copy_guardrails_contract.md),
  [`/artifacts/ai/approved_ai_terms.yaml`](../../artifacts/ai/approved_ai_terms.yaml),
  and [`/artifacts/ai/forbidden_ai_terms.yaml`](../../artifacts/ai/forbidden_ai_terms.yaml)
  for evidence-first certainty language and overclaiming bans on AI surfaces.
- [`/docs/copy/translation_safe_content_ops_contract.md`](./translation_safe_content_ops_contract.md)
  and [`/schemas/ux/message_placeholder.schema.json`](../../schemas/ux/message_placeholder.schema.json)
  for translation-safe placeholders, truncation/pseudoloc review gates, and
  downstream copy parity.
- [`/docs/ux/degraded_mode_pattern.md`](../ux/degraded_mode_pattern.md) and
  [`/docs/ux/banner_notice_contract.md`](../ux/banner_notice_contract.md) for
  “what still works” + “next safe action” disclosure in degraded states.

## Scope

This contract applies to any user-visible copy that can change what a user does
next, including:

- action labels on buttons, menus, toasts, sheets, and command-surface projections;
- error and warning messages in UI, CLI summaries, exports, and support packets; and
- AI-adjacent copy that could influence trust, adoption of changes, or perceived
  validation, freshness, or safety.

Out of scope: marketing copy and prose that does not affect user decisions.

## Action-label contract

Action labels are controls. They must be precise enough that users do not need
surrounding context to understand what will happen.

Rules:

1. **Verb-first and outcome-specific.** Prefer `Verb + object/scope`:
   - `Open output`
   - `Rebuild workspace index`
   - `Export support bundle`
   - `Continue read-only`
2. **No standalone vague labels on consequential paths.** Standalone labels such
   as `OK`, `Yes`, `No`, `Continue`, `Apply`, `Confirm`, `Submit`, and `Accept`
   are non-conforming when they are the entire label for an action that changes
   state, scope, trust, policy, execution, publishing, export, or deletion.
3. **Controlled exceptions are explicit.**
   - Navigation-only labels (`Back`, `Close`) are acceptable only when they do
     not perform a mutation.
   - Cancellation labels (`Cancel`) are acceptable only when they commit no
     state change.
   - “Continue” is acceptable only when it is qualified by the outcome or
     posture (`Continue read-only`, `Continue without connecting`, `Continue to
     review`) and never as a proxy for an unnamed consequential action.
4. **The same action uses the same label everywhere.** Buttons, menus, palettes,
   CLI summaries, exports, accessibility strings, and support packets reuse the
   same canonical label for the same command or event projection.

Mechanical fail gates:

- See the `not`-lists on action-label fields in:
  - [`/schemas/ux/button_action.schema.json`](../../schemas/ux/button_action.schema.json)
  - [`/schemas/ux/toast_event.schema.json`](../../schemas/ux/toast_event.schema.json)

## Error-message contract

Error messages are recovery tools. They must help users answer four questions:

1. **What failed?**
2. **Why it likely failed?** (or what is unknown)
3. **What still works?**
4. **What is the next safe action?**

Rules:

- **All four parts are required** for user-facing errors on trust-sensitive,
  degraded, blocked, recovery, and operational surfaces.
- **Scope must be explicit** when a concrete object/scope exists (workspace,
  host, provider, job, file, run, setting row, export). Do not hide scope behind
  “something” or “an error”.
- **Residual capability is mandatory.** If a failure does not block local work,
  say so. If it blocks only one lane, say what remains safe locally.
- **Next action is mandatory.** Provide exactly one primary next safe action
  where possible; do not end at “Try again later” unless no safe action exists.

Boundary schema:

- [`/schemas/copy/error_message.schema.json`](../../schemas/copy/error_message.schema.json)
  defines the `error_message_record` used by fixtures and future lint tooling.

## AI-copy contract (summary)

AI surfaces must not smuggle certainty, freshness, validation, or safety claims
into UI copy.

Rules (see the full AI contract for details):

- Use `Suggested`, `Proposed`, or `Draft` for AI-generated changes not yet
  accepted and applied.
- Use validation language (`Validation passed`, `Tests passing`, `Safe`) only
  when backed by a validation record and declared scope.
- If context is partial, stale, omitted, or policy-blocked, copy must name the
  covered and excluded scope rather than implying “the whole project”.
- Avoid overclaiming phrasing (for example, “fixed”, “guaranteed”, “no risk”)
  without bounded evidence.

Normative source:

- [`/docs/ai/ai_copy_guardrails_contract.md`](../ai/ai_copy_guardrails_contract.md)

## Placeholders and truncation safety

Translation-safe and truncation-safe copy is part of correctness.

Rules:

- Use structured placeholders (and translator notes) for variable insertion
  rather than ad-hoc string concatenation.
- Do not embed raw paths, raw URLs, raw command lines, raw secrets, or raw
  provider payloads in user-visible strings unless a governing redaction policy
  explicitly allows them.
- Action labels must remain meaningful when truncated; do not rely on a suffix
  that is likely to be cut off. Prefer shorter object nouns over vague verbs.
- If truncation is unavoidable, the accessible name and detail view must preserve
  the full meaning (including scope and next safe action).

Normative source:

- [`/docs/copy/translation_safe_content_ops_contract.md`](./translation_safe_content_ops_contract.md)

