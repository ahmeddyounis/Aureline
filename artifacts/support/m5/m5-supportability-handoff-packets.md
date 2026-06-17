# Supportability handoff packets — reviewer artifact

Human-readable companion to the governed packet at
`artifacts/support/m5/m5-supportability-handoff-packets.json`. The full contract and gate semantics live
in `docs/help/support/m5-supportability-handoff-packets.md`; the typed model lives in the
`aureline-support` crate (`m5_supportability_handoff_packets`).

This registry turns a blocked-user incident into one typed **supportability handoff packet** instead of an
ad hoc pile of logs and screenshots. Each packet joins the existing source objects by reference — a
finding code, a repair id, a crash artifact, the install / advisory state, a credential-state descriptor,
an environment summary, a precedence summary, and a restore-provenance record — and escalates them through
exactly one **handoff mode**: local self-diagnosis, team share, or formal support handoff. A fail-closed
handoff / share gate narrows or blocks any packet that would carry a data class further than its
destination allows, would have to redact, withhold, or downgrade a component, or would carry content that
cannot leave the machine — and it never collapses into a monolithic export that hides data-class
differences or redaction posture.

## Packet roll-up (as of 2026-06-17)

| Packet | Mode | Status | Presentation | Components |
| --- | --- | --- | --- | --- |
| `local-self-diagnosis-no-upload` | local_self_diagnosis | ready_to_share | **ready_to_share** | 8 |
| `team-share-redacted` | team_share | redaction_narrowed | **narrowed** | 6 |
| `formal-support-handoff` | formal_support_handoff | redaction_narrowed | **narrowed** | 7 |
| `policy-locked-export` | formal_support_handoff | policy_locked | **narrowed** | 4 |
| `blocked-user-escalation` | team_share | send_blocked | **send_blocked** | 4 |

One packet is fully ready to share (proving the gate is not a blanket flag), three narrow on a redacted,
withheld, policy-locked, or downgraded component, and one blocks an unsafe send. All three modes are
exercised, every component kind and data class appears, and every component keeps its data class and
redaction posture visible.

## The cases this corpus proves

### No-upload local handoff — `local-self-diagnosis-no-upload`

Local self-diagnosis joins all eight source classes — finding code, repair id, crash artifact, install /
advisory state, credential-state descriptor, environment summary, precedence summary, and
restore-provenance record — into one escalation object that never leaves the machine. Every component is
carried in full, so the packet is ready to share locally with no upload.

### Team-share side of the delta — `team-share-redacted`

The same incident escalated to the team. The finding and the credential-state descriptor are carried as
redacted summaries (`data_class_redacted_for_mode`), and a crash content excerpt is withheld because user
content never leaves the machine (`component_excluded_for_mode`). The packet narrows but still shares.

### Formal-support side of the delta — `formal-support-handoff`

The same incident escalated to formal support. The environment summary is redacted, the credential-state
descriptor is withheld entirely (formal support does not allow the credential-state data class), and the
restore provenance is downgraded and labeled (`lineage_downgraded`) rather than implying an exact restore.
Together with the team-share packet this proves the team-vs-formal delta: team share may carry credential
state redacted, formal support withholds it.

### Policy-locked export — `policy-locked-export`

A formal handoff where data-residency policy locks the credential-state class. The locked component is
withheld and named (`policy_locked_data_class`); the rest of the packet still escalates cleanly. The lock
is surfaced, never hidden.

### Blocked-user escalation with unsafe content — `blocked-user-escalation`

A blocked user stages a crash content excerpt that cannot be made safe for a team share. The gate blocks
the send (`send_blocked_unsafe_content`), warns before any packet leaves, and names the blocker (remove or
redact the excerpt, or save locally instead).

## Sign-off gate

Promotion of the supportability-handoff-packets registry holds unless all of the following are true on the
current packet (`M5SupportabilityHandoffPackets::validate()` returns no violations):

1. Every packet joins at least one source object, shows a visible copyable exact-build id and incident ref,
   and carries its one-step "Why is this escalating, and what does it carry?" explain entry plus the
   CLI / headless equivalent object.
2. Every component keeps its data class and redaction posture visible, carries its source_ref and
   lineage_ref, and any withheld component is withheld for a legitimate reason (policy lock or a
   data-class limit) — never a silent drop.
3. Every packet's `status`, `presentation`, `downgrade_reasons`, `lineage_complete` attestation, and
   `blocked_before_send` flag equal the recomputed fail-closed gate — so a packet can never present as
   ready while hiding a redacted, withheld, policy-locked, or downgraded component.
4. The per-mode policies match each mode's allowed data classes and default redaction posture.
5. No raw secret bodies, raw dumps, or raw payloads are carried (`raw_material_excluded`).
6. The five consumer bindings (Support Center, CLI / headless, issue-report flow, support drill packet,
   support export) are all present and reuse this packet's vocabulary, packet / component ids, and
   exact-build / finding-code / repair-id lineage, each keeping data classes visible.

## Regenerating this packet

This packet is checked in alongside the registry it reviews. When the registry changes, update the packet,
schema, reviewer artifact, and fixtures together, then re-run the gate:

```sh
cargo test -p aureline-support m5_supportability_handoff
cargo run -p aureline-support --example dump_m5_supportability_handoff_packets
```
