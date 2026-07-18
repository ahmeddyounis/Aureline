# M5 Collaboration-Control Shared Consumers: One Vocabulary Across Surfaces

This lane is the B155 consumer-adoption capstone. It binds the six governed collaboration-control objects
frozen by the
[`m5_shared_terminal_debug_control_grant_presenter_consent_retention_and_session_restore_view_matrix`](./m5-collaboration-control-ops.md)
— the **shared terminal / debug view**, the **control grant**, the **presenter token**, the **consent
envelope**, the **retention review**, and the **session-restore view** — to the concrete consumers that render
them, and proves, by fixtures rather than screenshots, that the same seeded collaboration session carries one
identical vocabulary wherever Aureline joins a session, follows from a browser / mobile companion, requests or
grants control, hands off the presenter token, reviews recording / retention, exports a support packet, or
restores a disconnected session.

- Rust module: `aureline_ui::m5_collaboration_control_shared_consumers_one_vocabulary_across_surfaces`
- Boundary schema: [`schemas/collaboration/m5-collaboration-control-shared-consumers.schema.json`](../../schemas/collaboration/m5-collaboration-control-shared-consumers.schema.json)
- Proof bundle: `artifacts/release/m5-collaboration-control-shared-consumers-proof/` (`support_export.json`, `matrix.csv`, `summary.md`)
- Fixtures: `fixtures/collaboration/m5-collaboration-control-shared-consumers/` (`compact_remote_narrowed.json`, `exported_redaction_narrowed.json`)
- Emitter: `cargo run -p aureline-ui --example dump_m5_collaboration_control_shared_consumers -- <support-export|report|csv|fixture-compact-remote-narrowed|fixture-exported-redaction-narrowed|validate>`

## Consumers

Nine shared consumer surfaces adopt the collaboration-control vocabulary: the shared terminal / debug view, the
collaboration join-review sheet, the control-grant prompt, the presenter-handoff sheet, the paste / secret
guard, the collaboration retention sheet, the session-restore view, the support / export packet, and the help /
docs surface. Each of the six objects is adopted by at least two distinct consumers, so an object is proven to
be shared collaboration-control infrastructure rather than a one-surface fork that invents its own grant,
recording, or restore labels.

## One vocabulary, no drift

For a given seeded collaboration session, every consumer surface must present identical
`CollaborationControlSharedStateFacetValues`: the same collaboration-control-role word, object word,
registry-reference word, session-state word, surface-context word, and authority-source word. The
collaboration-control-role word must be a token from the frozen `M5CollaborationControlRole` vocabulary
(`control_authority_disclosure`, `active_driver_disclosure`, `view_first_default_disclosure`,
`consent_scope_disclosure`, `recording_retention_state_disclosure`, `paste_secret_guard_disclosure`,
`replay_free_restore_disclosure`), so no surface invents an alternate label for the control authority, the
single active driver, the view-first default, or the join-time consent scope.

A role that carries control-authority, active-driver, view-first-default, or consent-scope meaning is a **gate
role**: it must pair its surface presentation with a real
`authority_source_disclosed_and_control_grant_bound` continuity and never collapse to a masquerade sentinel
(`control_acquired_from_presence_alone`, `presence_shown_as_control_authority`,
`second_active_driver_shown_on_a_sensitive_surface`, `prior_input_replayed_as_live_control`).

## Narrowing is disclosed

A surface may narrow *how much* it renders across the desktop-full, compact, remote-projected, and
exported-redacted representations, but never reword the vocabulary. Every narrowed representation carries an
explicit `CollaborationControlSharedNarrowNote` naming the reason, the preserved vocabulary, and the next
action; remote and exported forms additionally name their remote-source and export-safe-detail boundaries.

## Map back to one object

Support / export consumers point at the canonical per-domain schema and the frozen matrix by id, so an exported
packet — and every copy / export / open-in-provider action — maps back to one shared contract object rather
than diverging into a surface-local payload or collapsing stable authority / session labels to generic prose.
Raw secret values, command text, variable bodies, and clipboard contents stay outside the support boundary.

## No silent queued grants

Deferred-intent and outbox systems can never queue a control grant, a presenter handoff, terminal input, or any
other sensitive collaboration-control action across a reconnect or offline boundary. A refused control action
explains why it was refused and demands a fresh live review rather than replaying later as if it were an
idempotent background write, so no queue / retry system can smuggle authority across a reconnect or offline
boundary. This is asserted by the
`deferred_intent_never_queues_control_grants_presenter_handoffs_or_terminal_input` and
`refused_control_actions_explain_instead_of_replaying_as_idempotent_background_writes` trust invariants, the
`deferred_intent_and_outbox_systems_blocked_from_queueing_sensitive_control_actions` projection invariant, and
the `deferred_intent_queued_a_sensitive_control_action_without_a_fresh_live_review` downgrade trigger.

## Guardrails

Each binding re-asserts the batch's five hard invariants (all MUST be `false`): it never acquires terminal /
debug control from presence or follow without an explicit grant, never allows more than one active driver on a
sensitive surface, never starts recording / retention / guest-scope widening silently, never replays prior
terminal / debug input on join or restore, and never reveals raw secrets, command text, variable bodies, or
clipboard contents without a guard.

## Acceptance criteria mapping

1. **Consumers no longer restate collaboration-control truth with surface-local fields or silent fallbacks** —
   enforced by the per-subject facet identity and the `collaboration_control_vocabulary_drift_across_surfaces`
   violation over nine shared consumer surfaces, with the frozen-role-token gate
   (`collaboration_control_role_word_outside_vocabulary`) keeping the role vocabulary controlled and the
   gate-role `authority_source_missing_for_gate_role` check keeping presence from masquerading as control
   authority.
2. **Queue / retry systems can explain why a control action was refused instead of replaying it later as if it
   were an idempotent background write** — enforced by the deferred-intent trust and projection invariants and
   the `deferred_intent_queued_a_sensitive_control_action_without_a_fresh_live_review` downgrade trigger,
   backed by the five guardrail row-invariants (led by
   `acquires_terminal_or_debug_control_from_presence_without_an_explicit_grant`) and
   `points_at_canonical_contracts` / `support_export_reference_missing` so exported packets map back to one
   contract object.
