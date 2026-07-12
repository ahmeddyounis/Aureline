# M5 AI-Message-Card and Evidence-Timeline Controls

- Packet: `m5-ai-message-card-evidence-timeline-controls:stable:0001`
- Label: `M5 AI-message-card and evidence-timeline controls with source context, confidence / uncertainty class, route / provider locality, spend / cost posture, safe actions, and timestamp / evidence-kind / lineage / redaction truth aligned across editor, review, notebook, AI, support, and product surfaces`
- Consumer surfaces: 6
- AI message states: draft, streaming, review_required, blocked_by_policy, applied, reverted, failed, stale_evidence, state_unknown
- Evidence kinds: tool_invocation, validation_run, retrieval, user_edit, external_reference, kind_unresolved
- Proof freshness SLO: 720 hours (last refresh: 2026-07-12T00:00:00Z)

## Consumer surfaces

- **ai_ui**: `stable`
  - Owner: AI surface owner
  - Scope: The AI surface names draft, streaming, review-required, blocked, applied, reverted, failed, and stale-evidence message states with one controlled vocabulary and renders evidence as an inspectable timeline; both degrade honestly when a state is encoded generically or the evidence is hidden in an opaque log
  - Card examples: 3 / evidence examples: 3
- **editor_ui**: `stable`
  - Owner: Editor AI owner
  - Scope: The editor renders AI cards with the same state / source / confidence grammar and evidence entries with stated lineage, degrading honestly when a card leaves its confidence unstated or an evidence entry omits its tool / validation lineage
  - Card examples: 2 / evidence examples: 2
- **review_ui**: `stable`
  - Owner: Review AI owner
  - Scope: The review surface keeps approval state and external-source context inspectable before an AI output is trusted or applied, and preserves redaction truth on evidence, degrading honestly when approval is hidden, a source reads as workspace-grounded, or a redacted trail reads as complete
  - Card examples: 3 / evidence examples: 2
- **notebook_ui**: `stable`
  - Owner: Notebook AI owner
  - Scope: The notebook reuses the same message and evidence grammar in code cells, discloses stale evidence rather than reading as fresh, and degrades honestly when a message state or an evidence kind / lineage cannot be resolved
  - Card examples: 2 / evidence examples: 3
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved card and evidence truth, so an undisclosed source, an over-budget spend read as free, an unstated identity, a missing related resource, or an unresolved disclosure state is visible in evidence rather than hidden behind compact chrome, and redacted / partial trails stay disclosed
  - Card examples: 4 / evidence examples: 5
- **product_ui**: `stable`
  - Owner: In-product AI owner
  - Scope: In-product surfaces reuse the same card and evidence grammar a user sees in the AI panel, always offering the command-backed detail path and safe actions, and degrading honestly when the trace path is missing, the route or spend posture is unresolved or implicit, or an evidence entry lacks a timestamp or replay / export action
  - Card examples: 6 / evidence examples: 4
