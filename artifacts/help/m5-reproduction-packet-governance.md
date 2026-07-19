# M5 reproduction-packet review

Packet set: `m5_reproduction_packet_set:default`

| Packet | Surface | Flow | Posture | Data exit | Preview confirmed? | Offline reusable? |
| --- | --- | --- | --- | --- | --- | --- |
| `reproduction_packet:docs_pane` | Docs pane | Copy summary | Metadata refs only | `metadata_safe_object_refs` | true | false |
| `reproduction_packet:trust_warning` | Trust warning | Submit later | Security channel only | `security_payloads_only` | true | true |
| `reproduction_packet:update_screen` | Update screen | Save local | Metadata refs only | `no_payload_leaves_product` | true | true |
| `reproduction_packet:workflow_bundle` | Workflow bundle | Submit later | Redacted, support-scoped | `redacted_support_packet` | true | true |
| `reproduction_packet:other_surface` | Other surface | Copy summary | Fully redacted, public-safe | `metadata_safe_object_refs` | true | false |

Every packet previews each sensitive field before share, tokens are always removed, and packet creation never auto-submits — building a packet is separate from sending it.
