# Supportability handoff packets

When you are blocked and need help — from yourself, your team, or formal support — Aureline assembles one
typed **supportability handoff packet** instead of leaving you to scrape logs and paste screenshots. The
packet joins the objects that already describe your problem — a Project Doctor finding code, a guided
repair id, a crash artifact, the install / advisory state, a credential-state descriptor, an environment
summary, a precedence summary, and a restore-provenance record — into one escalation object. The contract
and fail-closed gate are owned by the `aureline-support` crate (`m5_supportability_handoff_packets`); the
canonical packet is checked in at `artifacts/support/m5/m5-supportability-handoff-packets.json` and
validated against `schemas/support/m5-supportability-handoff-packets.schema.json`.

## What every packet shows

- **One escalation object, not a pile of logs.** A packet joins finding codes, repair ids, crash
  artifacts, install / advisory state, credential-state descriptors, environment / precedence summaries,
  and restore-provenance records by reference. Each component carries its `source_ref` and `lineage_ref`,
  so the exact-build, finding-code, and repair-id lineage is preserved.
- **A visible, copyable exact-build id and incident ref.** Both are always shown and copyable, so you can
  quote them to support.
- **Visible data classes and redaction posture.** Every component shows its data class (`metadata`,
  `diagnostic_summary`, `environment_descriptor`, `credential_state`, `crash_artifact_reference`,
  `user_content_excerpt`) and its redaction posture. A handoff is never a monolithic export that hides
  these differences.
- **One handoff mode.** Each packet takes exactly one of **local self-diagnosis**, **team share**, or
  **formal support handoff**, each with its own allowed data classes and default redaction.

## Modes, allowed data classes, and default redaction

| Mode | Leaves machine | Allowed data classes | Default redaction |
| --- | --- | --- | --- |
| `local_self_diagnosis` | no | all six | `local_only_retained` |
| `team_share` | yes | metadata, diagnostic summary, environment descriptor, credential state, crash artifact reference | `redacted_summary` |
| `formal_support_handoff` | yes | metadata, diagnostic summary, environment descriptor, crash artifact reference | `metadata_safe_default` |

Local self-diagnosis stays on the machine, so it may retain every data class with no upload. Team share
may carry a credential-state descriptor as a redacted summary but never a user-content excerpt. Formal
support handoff withholds both credential state and user content from the vendor.

## The handoff / share gate

A fail-closed gate decides how each packet may be presented, from the disposition of its components for the
selected mode:

| Component disposition | Meaning |
| --- | --- |
| `carried` | included and send-safe for the mode, carried in full |
| `redacted` | included and send-safe, carried as a redacted summary |
| `withheld` | its data class cannot reach the mode, or it is policy-locked |
| `blocking` | included but cannot safely leave the machine for the mode |

- A packet with a **blocking** component is `send_blocked`: it warns and blocks before anything leaves.
- A packet that had to **redact**, **withhold**, or **downgrade** a component is `narrowed`.
- Otherwise the packet is `ready_to_share`.

Two invariants hold regardless of presentation: every component keeps its data class and redaction posture
visible, and the exact-build / finding-code / repair-id lineage is preserved on every component.

## The scenarios this corpus proves

| Packet | Mode | Status | Presentation |
| --- | --- | --- | --- |
| `local-self-diagnosis-no-upload` | local self-diagnosis | ready_to_share | **ready_to_share** |
| `team-share-redacted` | team share | redaction_narrowed | **narrowed** |
| `formal-support-handoff` | formal support handoff | redaction_narrowed | **narrowed** |
| `policy-locked-export` | formal support handoff | policy_locked | **narrowed** |
| `blocked-user-escalation` | team share | send_blocked | **send_blocked** |

- **No-upload local handoff** joins every source class into one object that never leaves the machine.
- **Team share** carries the finding and credential state as redacted summaries and withholds a user
  content excerpt.
- **Formal support handoff** withholds credential state entirely and labels a downgraded restore.
- **A policy-locked export** withholds and names the credential class locked by data-residency policy.
- **A blocked-user escalation** that stages unsafe content blocks the send before anything leaves.

## One vocabulary across surfaces

The Support Center, the CLI / headless support export (`aureline support handoff show <packet>`), the
issue-report flow, the shiproom / support drill packet, and the support export all bind to this one
registry. Each preserves the same handoff vocabulary, packet and component ids, and exact-build /
finding-code / repair-id lineage, keeps data classes visible, and narrows with the gate — so the packet
never forks between the desktop and CLI / headless paths.

## Regenerating this packet

This packet is checked in alongside the registry it documents. When the registry changes, update the
packet, schema, reviewer artifact, and fixtures together, then re-run the gate:

```sh
cargo test -p aureline-support m5_supportability_handoff
cargo run -p aureline-support --example dump_m5_supportability_handoff_packets
```
