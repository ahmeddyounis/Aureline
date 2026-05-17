# Evidence Timeline Packet

Packet ref: `evidence_timeline_packet:delete_hold_chronology.beta`

Schema: [`schemas/support/evidence_timeline.schema.json`](../../../schemas/support/evidence_timeline.schema.json)  
Doc: [`docs/support/m3/chronology_and_delete_honesty_beta.md`](../../../docs/support/m3/chronology_and_delete_honesty_beta.md)  
Fixture: [`fixtures/support/evidence_timeline/delete_hold_chronology_packet.json`](../../../fixtures/support/evidence_timeline/delete_hold_chronology_packet.json)

## Controlled state coverage

| State token | Operator label | Count |
| --- | --- | ---: |
| `requested_deletion` | `Delete requested` | 1 |
| `queued_deletion` | `Delete queued` | 1 |
| `held_data` | `Legal hold` | 1 |
| `retained_evidence` | `Policy retention` | 1 |
| `completed_deletion` | `Delete completed` | 1 |

## Chronology export

| Chronology order | Source display order | Source time | Actor order | State |
| ---: | ---: | --- | ---: | --- |
| 0 | 4 | `2026-05-16T09:00:00-04:00` | 10 | `requested_deletion` |
| 1 | 2 | `2026-05-16T13:00:05Z` | 20 | `queued_deletion` |
| 2 | 1 | `2026-05-16T16:00:05+03:00` | 30 | `held_data` |
| 3 | 0 | `2026-05-16T13:00:15Z` | 40 | `retained_evidence` |
| 4 | 3 | `2026-05-17T08:15:00Z` | 50 | `completed_deletion` |

The second and third rows share the same instant but keep different
source timezone context. Export order is chronology plus actor order,
not display order.

## Support/export posture

- Redaction class: `metadata_safe_default`
- Raw content exported: `false`
- Support item id: `support.item.evidence_timeline_packet`
- Preview section: `governance_and_export_controls`
