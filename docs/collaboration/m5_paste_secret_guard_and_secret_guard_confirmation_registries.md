# M5 paste-secret-guard and secret-guard-confirmation registries

Implement lane over the frozen [M5 collaboration-control component matrix][matrix]
(`m5_shared_terminal_debug_control_grant_presenter_consent_retention_and_session_restore_view_matrix`).
It makes the matrix's explicit paste / secret guard operable — as durable,
resolved records — and adds the secret-guard confirmation, by carrying honest projections of two
registries so the claimed M5 shared terminal / debug view, companion-follow flow, control-grant prompt,
paste / secret guard, support / export packets, and help / docs surfaces inherit one canonical paste-secret
guard and one secret-guard confirmation rather than a hand-authored parallel prose that has to be kept
consistent. It closes the gap between the already-landed shared-terminal / debug view stream,
control-channel badge, control-grant, and presenter-handoff lanes and the explicit guardrail the source
set now expects: the riskiest collaboration actions — high-risk paste, terminal broadcast, clipboard
bridge, debug-evaluate, environment-variable reveal, and variable-body reveal — are bounded so a shared
shell / debug moment cannot accidentally leak secrets or perform hidden high-impact actions.

## Registry-A — paste-secret guard

One durable, canonical paste-secret guard per high-risk action, carrying:

- the risky-action class — high-risk paste, terminal broadcast, clipboard bridge, debug-evaluate,
  environment-variable reveal, or variable-body reveal — so each guarded action names what it gates;
- the disclosed scope, target, and reason shown before the action can commit, so a risky paste or
  reveal cannot proceed silently;
- the step-up / confirm posture required, and the visible guardrail badge kept mechanically distinct from
  ordinary presence and follow state;
- the single-guard binding, so the guard on one action never reads as an approval on another;
- the resolution-form coverage (canonical object, accessible summary, audit record).

A guard that cannot bind its scope / target / reason to its session / target scope, that is a
hand-copied per-entry assumption instead of tracing to the shared registry, or that publishes an
incomplete object degrades honestly instead of letting a risky reveal read as an implicit approval. The
registry reuses the matrix `m5-paste-secret-guard.schema.json` domain schema.

## Registry-B — secret-guard confirmation

The typed secret-guard confirmation a participant reads before any risky paste or reveal commits — the
risky-action class it decided, the disclosed scope, target, and reason, the guard outcome (allowed with
confirm, step-up required, denied, or blocked), and the audit-safe attribution of who decided — plus the
fresh, visible guard event any risky action must raise: a disclose, an allowed-with-confirm, a step-up, a
deny, or a blocked outcome. The confirmation stays mechanically distinct from any raw secret body rather
than being flattened into one generic confirm dialog, and never copies a raw secret, command text,
variable body, or clipboard content into logs or exports. A declined or blocked event stays attributable
in-session and on export. The registry mints the `m5-secret-guard-confirmation.schema.json` domain schema.

## Acceptance criteria proven by the resolved examples

1. Risky paste or reveal actions cannot proceed silently and always disclose scope, target, and reason
   before commit: a guard that would let a high-risk paste, terminal broadcast, clipboard bridge,
   debug-evaluate, environment-variable reveal, or variable-body reveal proceed without disclosing its
   scope, target, and reason, or without an explicit policy / consent posture and visible guardrail,
   degrades instead of reading as a clean, guarded action.
2. Declined or blocked secret-guard events remain attributable without copying raw secret bodies into
   support / export paths: the risky-action class, disclosed scope / target / reason, guard outcome, and
   attribution stay visible in the UI projection, the CSV / export, and the support packet instead of
   collapsing into a generic status pill — and never carry raw secret, command, variable-body, or
   clipboard material.
3. Collaboration presence never implies permission to reveal, no step-up / confirm posture is skipped
   where required, and prior terminal / debug input never replays on join or restore; the registries keep
   each paste-secret-guard and secret-guard-confirmation dimension distinct.

Raw secrets, raw command text, variable bodies, clipboard contents, and private endpoints never cross
this boundary. The Rust validator in `crates/aureline-ui` is the authoritative gate; the checked-in
combined registries schema
(`schemas/collaboration/m5-paste-secret-guard-and-secret-guard-confirmation-registries.schema.json`)
documents the shape.

[matrix]: ../../crates/aureline-ui/src/m5_shared_terminal_debug_control_grant_presenter_consent_retention_and_session_restore_view_matrix/mod.rs
