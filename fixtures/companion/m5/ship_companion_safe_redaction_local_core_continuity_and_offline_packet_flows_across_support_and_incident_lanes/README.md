# Companion-Safe Redaction, Local-Core Continuity, and Offline Packet Flow Fixtures

These fixtures are generated deterministically from the first-consumer surface builder in
`aureline-companion` and validate against
`schemas/companion/ship-companion-safe-redaction-local-core-continuity-and-offline-packet-flows-across-support-and-incident-lanes.schema.json`.

## managed_service_degraded_surface.json

A surface where the managed service is degraded, so every section narrows one qualification
step and one rollout step, and every live/cached item is forced to `stale` with
`stale_label_shown` set. `degraded_labels` records `managed_service_degraded` and
`freshness_downgraded_to_stale`. Demonstrates that a degraded managed service narrows the
claim and downgrades freshness honestly while the local-first packet paths remain, redaction
stays enforced, and local-core continuity is preserved.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_redaction_continuity_offline_packet_surface -- managed_service_degraded
```

## redaction_proof_lost_surface.json

A surface where redaction proof is unavailable, so every verified redaction (across the
redaction-policy, incident-packet, and support-packet sections) downgrades to
claimed-but-unverified, sets `redaction_label_shown`, clears `redaction_verified`, a
`redacted_summary` class narrows to `reference_only`, and the redaction-policy section
narrows one step. `degraded_labels` records `redaction_proof_unavailable` and
`redaction_narrowed_to_reference`. Demonstrates that a redaction is shown as proven only when
verifiable, and otherwise narrows to a more conservative class and is labeled — no raw payload
body ever crosses regardless.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_redaction_continuity_offline_packet_surface -- redaction_proof_lost
```

## packet_assembler_down_surface.json

A surface where the offline packet assembler is unavailable, so every provider-assembled
packet (`requires_provider_assembly`) narrows to `unavailable` while every
`local_ready`/`local_staging` packet keeps working, and the incident-packet and
support-packet sections narrow one step. `degraded_labels` records
`packet_assembler_unavailable` and `packet_narrowed_to_local_path`. Demonstrates that losing
the assembler narrows the managed packet while the local-first path keeps the support and
incident workflow able to assemble and replay offline.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_redaction_continuity_offline_packet_surface -- packet_assembler_down
```

## completeness_unverified_surface.json

A surface where packet completeness is unverified, so every verified completeness claim (in
incident and support packets) downgrades to `complete_unverified`, sets `proof_label_shown`,
clears `claim_verified`, and the packet sections narrow one step. `degraded_labels` records
`completeness_unverified` and `completeness_claim_downgraded`. Demonstrates that a completeness
claim is shown as proven only when verifiable, and otherwise labeled.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_redaction_continuity_offline_packet_surface -- completeness_unverified
```

## incident_attribution_lost_surface.json

A surface where incident attribution is unavailable, so every attributed incident packet
narrows its attribution (`attribution_present` cleared), sets `attribution_label_shown`, and
the incident-packet section narrows one step. `degraded_labels` records
`incident_attribution_unavailable` and `incident_attribution_narrowed`. Demonstrates that an
incident packet stays attributable or is honestly labeled — it is never shown as attributable
when its attribution can no longer be established.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_redaction_continuity_offline_packet_surface -- incident_attribution_lost
```
