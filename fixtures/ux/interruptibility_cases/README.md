# Interruptibility-arbitration case fixtures

Worked YAML fixtures for the interruptibility-arbitration contract,
the escalation matrix, and the focus-steal prevention rules:

- [`/docs/ux/interruptibility_arbitration_contract.md`](../../../docs/ux/interruptibility_arbitration_contract.md)
- [`/artifacts/ux/escalation_matrix.yaml`](../../../artifacts/ux/escalation_matrix.yaml)

The directory contains one fixture per worked scenario the contract
acceptance calls out — trust, auth, update, and repair flows
requesting attention without bypassing the escalation model — plus
companion fixtures for typing, presentation, screen share, voice
capture, assistive-tech, and OS-notification reopen flows.

Each fixture pins:

- one `arbitration_row_id` from the matrix;
- one `active_flow_class`;
- the incoming `interruptibility_tier`;
- the active `quiet_hours_mode` set;
- the chosen `arbitration_outcome_class`;
- the `delivery_surface_class` used (or
  `delivery_surface_class = not_delivered_held` when held);
- the `escalation_required_triggers` value cited;
- the `focus_steal_attempt_class` set the row denies;
- the upstream `interaction_safety_packet_id_ref` quoted (for
  review-sheet routing);
- the `reopen_target_record` and revalidation posture;
- the audit / lineage / suppression / badge proofs every reviewer
  joins by ref.

Fixture index:

- `trust_downgrade_during_typing.yaml` — workspace trust downgrade
  arrives while the user is typing; routes via the review sheet, no
  modal, no toast.
- `auth_callback_during_presentation.yaml` — provider-handoff
  approval expiry arrives during full-screen presentation; durable
  attention item + scoped lock-screen summary; reopen denies with
  revalidation.
- `update_install_during_screen_share.yaml` — update reaches
  ready-to-restart while screen share is active; durable row +
  status item; reboot prompt held until share ends.
- `repair_flow_during_voice_capture.yaml` — extension crash repair
  card requests attention while voice capture is active; routes to
  attention item; voice-mode unaffected.
- `critical_safety_during_typing.yaml` — secret-broker credential
  compromise during typing; tier_critical_safety routes via review
  sheet; never silenced.
- `os_reopen_after_quiet_hours_exit.yaml` — OS notification reopen
  after quiet-hours exit lands on the canonical durable row, not
  on a generic home screen.
- `assistive_tech_transient_held.yaml` — under reduced-attention
  posture a transient toast routes to status_item with a screen-
  reader announcement on the keyboard parity lane.
- `modal_focus_steal_denied_on_review.yaml` — a product-owned modal
  attempt while review-canvas is active denies with
  product_owned_nested_overlay_forbidden /
  focus_steal_on_protected_path; the parent sheet updates in place.
