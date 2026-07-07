# M5 prompt-composer component matrix (design QA)

Shared design / schema / QA / release matrix for the reusable M5 pre-send
prompt-composition components (row **M05-884**, batch B104). Design, schema, QA, and
release owners consume this one matrix instead of rewording composer mode / scope /
route / attachment / trust / taint / draft / send truth per surface.

**Canonical truth (do not re-key):**

- Contract doc:
  `docs/ai/m5/freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix.md`
- Schema:
  `schemas/ai/freeze-the-m5-prompt-composer-header-context-attachment-pill-mention-resolver-slash-command-row-budget-strip-tainted-context-warning-and-draft-state-component-matrix.schema.json`
- Support export + CSV + report:
  `artifacts/ai/m5/freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix/`
- Emitter: `cargo run -p aureline-ai --bin aureline_ai_prompt_composer_component_matrix -- report`

## Component families and the truth each must always show

| Family | Must always show | Never allowed to |
| --- | --- | --- |
| `prompt_composer_header` | mode + scope + route/provider/model | leave what will be sent, how wide, or where it routes implicit |
| `context_attachment_pill` | attached object identity + trust state | show a stale / unverified / tainted attachment as trusted-fresh |
| `mention_resolver` | mention-resolution state | send an unresolved / ambiguous mention as if it bound cleanly |
| `slash_command_row` | availability + approval gate | show a disabled / gated / policy-hidden command as ready |
| `budget_size_strip` | budget posture + omitted-context reason | present an over-budget send or silently dropped context as clean |
| `tainted_context_warning` | taint source + severity | show injection-suspected / quarantine-required context as trusted |
| `draft_state_row` | draft locality + retention | show a local-only draft as synced or a retained draft as purged |
| `attachment_stale_banner` | staleness reason | leave a moved / deleted / revoked attachment silently attached |
| `send_review_control` | send posture + review requirement | let a review-needed or blocked request send as a plain action |

## Design acceptance gates

1. **One vocabulary.** Composer mode, scope, route class, attachment kind, trust
   state, mention resolution, slash-command state, budget posture, omitted-context
   reason, taint source, taint severity, draft locality, staleness reason, send
   posture, and review requirement use only the frozen tokens in the
   schema/contract. No surface invents parallel labels.
2. **Mandatory labels.** Every component exposes `identity`, `state`, and
   `keyboard_route`, plus the truth labels relevant to it (`composer_mode`,
   `route_provider_model`, `trust_or_taint`).
3. **Non-visual parity.** Every component is keyboard-focusable, screen-reader
   announced, non-hover reachable, pointer-optional, high-contrast safe, and
   support-exportable. Nothing is panel-only or chat-only.
4. **Deployment parity.** The same truth survives local-OSS, self-hosted, managed,
   air-gapped, and mirror/offline lines.
5. **Distinct blocked states.** `local_only`, `managed_route`, `policy_blocked`,
   `over_budget`, `unresolved_missing`, and the tainted-context states each keep
   their own token; none collapses into a generic send.
6. **Auto-narrowing.** When a downgrade trigger fires the component drops below
   Stable while staying visible (fixtures: `tainted_context_warning` → Beta,
   `send_review_control` → Preview).

See `matrix.csv` in the canonical artifact directory for the per-family
surface-family / deployment-line / required-label / consumer-surface /
downgrade-trigger grid.
