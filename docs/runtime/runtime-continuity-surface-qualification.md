# Runtime Continuity Surface Qualification

This document is the contract companion to
`artifacts/runtime/runtime-continuity-surface-qualification/support_export.json`.

- Packet id: `runtime-continuity-surface-qualification:stable:0001`
- Schema ref: `schemas/runtime/runtime-continuity-surface-qualification.schema.json`
- Upstream runtime packet:
  `artifacts/runtime/queue-session-terminal-governance.md`
- Help-facing summary:
  `docs/help/runtime-continuity-surface-qualification.md`

## Shared Vocabulary

- Profiles: `local_only`, `mirrored`, `managed`, `browser_handoff`
- Proof classes:
  `queue_fairness`, `restore_no_hidden_rerun`,
  `terminal_protocol_clipboard`, `transcript_and_shared_control`,
  `browser_handoff_continuity`
- Labels: `stable`, `beta`, `preview`
- Evidence currency: `current`, `stale`
- Narrow reasons:
  `proof_packet_stale`, `browser_handoff_continuity_unqualified`

## Stable Rules

- A profile may remain `stable` only when the checked queue/session/terminal
  governance packet stays current and satisfies every required proof class for
  that profile.
- `displayed_label` is the only label downstream docs/help, Help / About,
  support, and public-truth consumers may render. They must not widen back to
  `claim_label`.
- `browser_handoff` is intentionally narrower until a checked browser-handoff
  runtime packet proves live terminal authority, clipboard posture, and rerun
  semantics without inheriting desktop continuity by adjacency.
- Consumers must cite the shared index packet directly instead of paraphrasing
  queue fairness, restore fidelity, or terminal-boundary maturity in local
  prose.

## Proof Inputs

- Queue identities, fairness lanes, protected-path fitness, transcript export,
  and shared-control roles come from:
  `artifacts/runtime/queue-session-terminal-governance.md`
- Terminal protocol corpus and paste drills come from:
  `fixtures/terminal/protocol_corpus_alpha/manifest.json`
  and
  `fixtures/terminal/paste_boundary_alpha/high_risk_remote_multiline_review.json`
- Restore/no-hidden-rerun drills come from:
  `docs/runtime/context_cache_and_terminal_restore_contract.md`
  and
  `fixtures/terminal/restore_cases/failure_drill_lost_transport_becomes_transcript.json`

## Automatic Narrowing

- Any stale or structurally regressed upstream runtime packet narrows claimed
  rows to `beta` with `proof_packet_stale`.
- Any profile that still lacks a required proof class narrows to `preview`
  instead of inheriting a neighboring stable desktop claim.
- The checked packet intentionally exercises that rule on `browser_handoff`:
  handoff context is disclosed, but live terminal authority is not overclaimed.
