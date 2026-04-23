# Fixture: contextual degraded — extension quarantined (Degraded)

## Scenario

A third-party extension has crashed three times in the current
grouped-burst window. The shell promotes the extension to
`extension.quarantined` and surfaces a contextual degraded row
on the extension's activity-centre card. The workspace remains
operational; the extension's capability is narrowed.

## Row bindings

- **Failure-tier row.** `tier.contextual_degraded`.
- **Lifecycle row.** `lifecycle:extension`, state
  `extension.quarantined`.
- **Startup-state intersection.** None (the quarantine happened
  mid-session).
- **Controlled label.** `Degraded`.
- **Truth class.** `runtime_observed_truth` on the extension
  host status.
- **Degraded-state token.** `Limited` on the extension's
  capability rows; composes to `RetestPending` after a repair
  attempt.

## Required axes rendered

- **Cause token.** `quarantined_extension` (named verbatim
  with extension id ref and crash-class ref).
- **Preserved.** Other extensions; workspace edits; save
  pipeline; recovery journal; keyboard-reachable chrome.
- **Narrowed.** The quarantined extension's contributed
  commands, views, and providers. The activity-centre card
  names exactly what is blocked (e.g., "Commands from
  *[extension-ref]* disabled").
- **Next-safe action hooks.** `review_archetype_match`
  (open extension details), `continue_in_restricted_mode`
  (continue without the extension),
  `review_trust_and_open` (reveal the last-failure reason).

## Placement

- **Failure tier.** `tier.contextual_degraded`.
- **Delivery surfaces.** `activity_center_digest_card` for the
  extension; `status_item` pinned to the activity rail with
  the last-failure reveal; `durable_job_row` for any
  in-progress repair job.
- **Interruptibility tier.** `tier_durable`.

## Controlled label

- **Label text rendered.** "Degraded" (reserved per taxonomy
  §8.6).
- **Supporting subtitle.** Names the quarantined extension id
  by ref and the crash-loop threshold that triggered
  quarantine.

## Recovery affordances (keyboard-reachable)

- **`Reload extension`** — re-attempts activation with a
  typed retry.
- **`Reveal last failure`** — opens the quarantine evidence
  inspector (crash class, crash count, crash timestamps).
- **`Open extension details`** — navigates to the extension
  detail surface.
- **`Continue without extension`** — confirms the
  `rung.restricted_mode_fallback` posture for this session.
- **`Quarantine extension`** — already applied; visible as
  the active rung id.

## Last-failure reason (mandatory visibility)

- **Keyboard-reachable reveal.** Activity-centre card drawer;
  reveal verb announced verbatim.
- **Support-export field.** Included on
  `recovery_ladder_packet` and `object_issue_handoff`;
  preserves extension id, crash class, grouped-burst id, and
  the recovery-rung history.
- **Export-safe description.** Redaction pass runs; raw
  extension logs are not included on the lock-screen summary
  payload class (the OS notification, if emitted at the
  critical tier, uses `lock_screen_safe_generic`).

## Promotion triggers

- **To `tier.workflow_block`.** User invokes a command
  contributed by the quarantined extension; a window-
  attached sheet explains the block and offers the
  reload / continue-without / reveal-evidence actions.
- **To `tier.session_recovery`.** Quarantine + crash-loop
  across session restart → `rung.safe_mode` entry.

## Recovery / support / measurement

- **Recovery-ladder rung.** `rung.extension_quarantine`;
  `rung.restricted_mode_fallback` on continue-without.
- **Support-packet family.** `recovery_ladder_packet`,
  `object_issue_handoff`.
- **Journey-trace class.** `shell_open`.

## Accessibility

- "Degraded" announced on the activity-centre card; the
  narrowed-capability subtitle is separately addressable.
- The reveal affordance is reachable from the activity-rail
  focus cycle.

## Forbidden copy and patterns on this path

- "All extensions loaded"
- "Extension is fine"
- "Error loading extension"
- Toast-only rendering of the quarantine event (violates
  `toast_only_forbidden_for_durable_work`)
- Silent disappearance of the extension's contributed surfaces
  (the missing-extension-placeholder rule applies)

## Expected observable outcomes

- The controlled label `Degraded` renders on the extension's
  activity-centre card.
- Narrowed capabilities are separately named.
- Promotion to workflow block / session recovery only fires on
  a trigger.
- `overclaims_readiness = false` is asserted.

## Fixture fields (seed)

```yaml
__fixture__:
  name: contextual_degraded_extension_quarantined
  taxonomy_rows:
    - tier.contextual_degraded
    - lifecycle:extension
  controlled_label: "Degraded"
  doc_section: docs/ux/state_and_recovery_taxonomy.md#7.2
running_build_identity_ref: build-identity-seed-state-ctx-extension-quarantined
overclaims_readiness: false
```
