# Supportability qualification

Aureline's M5 supportability claims rest on **named qualification rows**, not on
a broad statement that recovery is "available". Every claimed supportability
surface — the Support Center, environment explainability, precedence inspection,
support-bundle consent, crash-intake/recovery, and the supportability handoff —
is qualified on every claimed M5 profile across the desktop and CLI/headless
deployment modes, and each row stands on its **own** proof.

The contract and downgrade automation are owned by the `aureline-support` crate
(`m5_supportability_qualification`). The canonical index is checked in at
`fixtures/support/m5/m5-supportability-qualification/packet.json` and validated
against `schemas/support/m5-supportability-qualification.schema.json`.

## What the index certifies

For every supportability surface on every claimed profile, one qualification row
records:

- **the surface and its deployment-mode coverage** — desktop and CLI/headless,
  so blocked-user recovery never fragments between the two;
- **its own backing proof** — the surface's own boundary schema, canonical
  artifact, fixture corpus, and record kind. A row may never borrow an adjacent
  crash or Doctor surface's evidence;
- **the supportability drills that back it** — diagnosis latency, consent-sheet
  accuracy, export-mode parity, crash-loop recovery, and exact-build intake;
- **its share posture** — whether the surface offers a team-share or
  formal-support send, and that local-save / self-diagnosis stays first-class
  beside it; and
- **the published state** plus the stale-proof tokens and downgrade-rule ids
  that explain any narrowing.

## The supportability surfaces

| Surface | Backing lane |
| --- | --- |
| `support_center` | `schemas/support/m5-support-center-matrix.schema.json` |
| `environment_explainability` | `schemas/runtime/m5-environment-status-strip.schema.json` |
| `precedence_inspection` | `schemas/support/m5-precedence-inspector.schema.json` |
| `support_bundle_consent` | `schemas/support/m5-support-bundle-consent.schema.json` |
| `crash_intake_recovery` | `schemas/support/m5-crash-intake.schema.json` |
| `supportability_handoff` | `schemas/support/m5-supportability-handoff-packets.schema.json` |

## Published states

| State | Meaning |
| --- | --- |
| `qualified` | The surface and all its bound drills are current on the profile. |
| `limited_profile_scoped` | The surface keeps a narrower, profile-scoped claim only. |
| `local_self_diagnosis_only` | Only the local-save / self-diagnosis path is claimable; any send path is unverified. |
| `blocked_unverified` | The broad surface claim is blocked pending fresh proof. |

## Auto-narrowing

A row narrows automatically — Help/About, support promises, and evaluation
materials narrow with it — when any of these triggers fire:

| Trigger | Effect |
| --- | --- |
| `surface_evidence_stale` | The surface's own lane evidence is stale or missing; the surface blocks and any integrated surface that binds it narrows. |
| `supportability_drill_stale` | A bound drill is stale; the rows it backs narrow, and a send-capable surface narrows to a local-save / self-diagnosis claim. |
| `deployment_mode_parity_lost` | A surface stops projecting identically across desktop and CLI/headless; the row narrows to the modes it still covers. |
| `policy_blocked_send` | A profile policy-blocks a send; the surface narrows to a local-save / self-diagnosis claim and local-save is never demoted below the blocked send. |
| `consumer_binding_missing` | A downstream surface stops ingesting the index by reference; the broad claim blocks until parity is restored. |

Two invariants hold regardless of state:

1. **Each surface stands on its own proof.** A passing crash or Doctor row may
   never keep a stale Support Center, export, or intake surface green.
2. **Local-save stays first-class.** A send-capable surface may never publish a
   row that demotes local-save or self-diagnosis below a team-share or
   formal-support send.

## One index, every consumer

Help/About, the desktop Support Center, CLI/headless support output, support
export, the shiproom claim packet
(`artifacts/shiproom/m5-supportability-claim-packet/m5_supportability_claim_packet.md`),
and the release manifest all ingest this one packet by reference and preserve
the row ids, surface and profile tokens, published state, deployment-mode
coverage, stale-proof tokens, and downgrade-rule ids verbatim. None of them
maintains a parallel supportability badge.

## Regenerating the evidence

```sh
cargo run -q -p aureline-support --example dump_m5_supportability_qualification_packet -- canonical \
  > fixtures/support/m5/m5-supportability-qualification/packet.json
cargo test -p aureline-support m5_supportability_qualification
```
