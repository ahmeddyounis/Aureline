# M5 Emergency-Notice Banner Primitive: Reason Class, Affected Capability, Continuity, Deadline, and Dismissal Parity

- Packet: `m5-emergency-notice-banner-primitive:stable:0001`
- Label: `M5 emergency-notice banner primitive: reason class, affected capability, blast radius, local-work continuity, deadline / urgency, primary / recovery actions, and dismissal-rule parity across channels`
- Reason-class lanes: 5 (5 stable)
- Anatomy parts: reason_class, affected_capability, blast_radius, local_continuity_note, deadline_urgency, primary_action, recovery_action
- Severity classes: informational, low, moderate, high, critical, operational_emergency
- Channels: update_center, extension_host, native_notification, support_bundle
- Dismissal actions: acknowledge, snooze, dismiss
- Export fields: advisory_id, severity, action_state, affected_surface, mitigation_state, delivery_profile, freshness_state, continuity_note, disclosure_visibility, history_state
- Proof freshness SLO: 720 hours (last refresh: 2026-06-30T00:00:00Z)

## Reason-class lanes

- **Capability Kill Switch**: `stable`
  - Owner: Extension trust owner
  - Scope: The kill-switch lane renders the shared emergency banner so a compromised extension capability shows `capability_kill_switch`, the affected capability, the single-capability blast radius, `affected_capability_suspended_local_safe` continuity, and a must-acknowledge (blocked-until-remediated) dismissal rule — editing, review, and export stay safe, and there is no generic close button
  - Shell zone: `title_context_bar`
  - Worked emergencies: 1
    - `AURELINE-EMG-2026-0201` — operational_emergency (affected_capability_suspended_local_safe), dismissal `blocked_until_remediated`, local work stays safe
- **Trust-Root Rotation**: `stable`
  - Owner: Signing / trust-root owner
  - Scope: The trust-root-rotation lane renders the shared emergency banner so a rotated trust root shows `trust_root_rotation`, the new-and-previous signer continuity, the all-signed-updates blast radius, `blocked_pending_acknowledgement` continuity, and an acknowledge-required dismissal rule while local files stay safe
  - Shell zone: `title_context_bar`
  - Worked emergencies: 1
    - `AURELINE-EMG-2026-0202` — critical (blocked_pending_acknowledgement), dismissal `unacknowledged`, local work stays safe
- **Channel Freeze**: `stable`
  - Owner: Update / release channel owner
  - Scope: The channel-freeze lane renders the shared emergency banner so a frozen stable channel shows `channel_freeze`, the paused-updates blast radius, `local_work_continues_degraded` continuity, and an acknowledge-or-snooze dismissal rule — no update, but everything local still works
  - Shell zone: `title_context_bar`
  - Worked emergencies: 1
    - `AURELINE-EMG-2026-0203` — high (local_work_continues_degraded), dismissal `unacknowledged`, local work stays safe
- **Forced Disable**: `stable`
  - Owner: Extension governance owner
  - Scope: The forced-disable lane renders the shared emergency banner so a forcibly disabled deprecated extension shows `forced_disable`, the single-extension blast radius, `local_work_continues_safely` continuity, and an acknowledge-required dismissal rule — editing, review, and export continue safely and the banner never implies data loss
  - Shell zone: `title_context_bar`
  - Worked emergencies: 1
    - `AURELINE-EMG-2026-0204` — low (local_work_continues_safely), dismissal `unacknowledged`, local work stays safe
- **Signed Emergency Bundle**: `stable`
  - Owner: Managed emergency-distribution owner
  - Scope: The signed-emergency-bundle lane renders the shared emergency banner so an informational managed notice reads `continuity_assessment_pending` and is dismissible, while a signed bundle that confirms a localized cache-corruption event reads `data_loss_proven` — data loss is stated only when the event actually proves it
  - Shell zone: `title_context_bar`
  - Worked emergencies: 2
    - `AURELINE-EMG-2026-0205` — informational (continuity_assessment_pending), dismissal `not_acknowledgeable`
    - `AURELINE-EMG-2026-0206` — moderate (data_loss_proven), dismissal `unacknowledged`, data loss proven by the event
