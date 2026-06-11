# Companion-Safe Redaction, Local-Core Continuity, and Offline Packet Flows Across Support and Incident Lanes

This document is the human-readable contract for the lane that ties the companion,
incident, and support surfaces together around three guarantees: every record that
crosses a companion, support, or incident boundary is **redaction-safe**, the **local
core stays authoritative and continues** when a provider degrades, and the support and
incident packets that flow out **assemble and replay offline** from the local core. It
has four sections: the **redaction policy** rows that record, per boundary and content
class, the redaction class applied before content crosses to a companion, support, or
incident surface; the **local-core continuity** rows that record, per capability,
whether the capability continues from the authoritative local core offline; the
**offline incident packet** rows that record the incident packets that assemble and
replay offline, each attributable and redacted; and the **offline support packet** rows
that record the support-export packets that assemble and replay offline, each redacted
and local-first. The machine-readable truth source is the checked-in support export;
later desktop companion panel, incident-workspace, CLI/headless, diagnostics, support
export, and Help/About surfaces ingest it instead of cloning status text.

- Record kind: `ship_companion_safe_redaction_local_core_continuity_and_offline_packet_flows_across_support_and_incident_lanes`
- Schema: `schemas/companion/ship-companion-safe-redaction-local-core-continuity-and-offline-packet-flows-across-support-and-incident-lanes.schema.json`
- Support export: `artifacts/companion/m5/ship_companion_safe_redaction_local_core_continuity_and_offline_packet_flows_across_support_and_incident_lanes/support_export.json`
- Markdown summary: `artifacts/companion/m5/ship_companion_safe_redaction_local_core_continuity_and_offline_packet_flows_across_support_and_incident_lanes.md`
- Fixtures: `fixtures/companion/m5/ship_companion_safe_redaction_local_core_continuity_and_offline_packet_flows_across_support_and_incident_lanes/`
- Producer crate: `aureline-companion`

## Sections and matrix inheritance

The packet has four sections. Every section inherits its qualification and staged
rollout stage from a frozen M5 companion-matrix lane (see
`docs/companion/m5/freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes.md`),
so a section never claims more than the matrix qualifies. This lane is cross-cutting: the
redaction-policy section binds to the `companion_notification` lane (keeping companions
narrow and redaction-safe), the incident-packet section to the `incident_workspace` lane
(keeping incident packets attributable), and the continuity and support-packet sections
to the `offboarding_continuity` lane (keeping the local core authoritative and never
stranding local work). The redaction-policy, continuity, and support-packet sections earn
their lane's Beta/staged-rollout qualification because companion-safe redaction is
enforced, the local core stays authoritative, and a local-first path is always available;
the offline incident-packet section inherits the Preview/early-access qualification
because its attribution and provider-assembled paths are less mature.

| Section | Matrix lane | Scope | Qualification | Rollout stage |
| --- | --- | --- | --- | --- |
| `redaction_policy` | `companion_notification` | `read_only` | beta | staged_rollout |
| `local_core_continuity` | `offboarding_continuity` | `read_only` | beta | staged_rollout |
| `offline_incident_packet` | `incident_workspace` | `read_only` | preview | early_access |
| `offline_support_packet` | `offboarding_continuity` | `read_only` | beta | staged_rollout |

## Read-only projection, with the local core authoritative

Every section is read-only. The surface **projects** redaction posture, continuity, and
offline-packet availability but never applies them: redaction is enforced and a packet is
assembled by the local core, never authored from this surface
(`action_applied_by_local_core_not_surface`). A local-first incident-packet path and a
local-first support-packet path are always offered as a fallback, so a degraded provider
never strands the support or incident workflow.

- **Redaction policy** rows record each `boundary` (`companion`, `support`, `incident`),
  its `content_class` (`notification_body`, `review_content`, `incident_evidence`,
  `support_diagnostics`, `session_transcript`, `usage_metrics`), the `redaction_class`
  applied before content crosses, and assert `no_payload_body = true`.
- **Local-core continuity** rows record each `capability` (`local_editing`,
  `local_search`, `incident_review`, `support_export_assembly`, `redaction_enforcement`,
  `offline_packet_replay`), its `continuity_posture`, whether it is `available_offline`,
  whether it `requires_provider_continuity` or `requires_admin_continuity`, and assert
  `local_work_preserved = true`.
- **Offline incident packets** record each `packet_class` (`evidence_timeline`,
  `runbook_execution`, `resource_slice`, `incident_export_bundle`), its `availability`,
  `completeness`, `redaction_class`, and whether `attribution_present`.
- **Offline support packets** record each `packet_class` (`diagnostics_bundle`,
  `config_snapshot`, `proof_packet_export`, `session_diagnostics`), its `availability`,
  `completeness`, and `redaction_class`.

## Companion-safe redaction

Every redaction-policy row, incident packet, and support packet asserts that no raw
payload body crosses the boundary (`no_payload_body` / `no_payload_body_crosses_boundary`):
the four redaction classes (`redacted_summary`, `metadata_only`, `reference_only`,
`withheld`) are body-free by construction and differ only in how much redacted metadata
accompanies the record. A redaction is shown as proven (`redaction_verified = true`) only
when backed by evidence. An unverifiable redaction clears `redaction_verified`, sets
`redaction_label_shown = true`, and narrows its class toward the conservative end
(`redacted_summary` → `reference_only`), so it is never shown as proven and never leaves a
summary that was not verified.

## Offline packet flows

A local-first packet path (`local_ready` or `local_staging`) is always present for both
the incident lane (`incident_packet_local_path_always_available`) and the support lane
(`support_packet_local_path_always_available`), so a degraded provider never strands the
support or incident workflow. A packet claims completeness (`complete_verified`) only when
`claim_verified = true`; an unverifiable claim narrows to `complete_unverified` and is
labeled. An incident packet stays attributable (`attribution_present = true`) or, when its
attribution can no longer be established, clears the flag and sets
`attribution_label_shown` so it is honestly labeled rather than shown as attributable.

## Local-core continuity

Every local-core-authoritative capability stays available offline
(`continuity_posture.continues_offline ⇒ available_offline`), `local_work_preserved` never
goes false, and a capability that requires provider or admin continuity discloses it
through `requires_provider_continuity` / `requires_admin_continuity`. When a provider
degrades, the local path keeps working and the degraded capability is labeled, not hidden.

## Stale-state honesty

Every item carries a `freshness` state (`live`, `cached`, `stale`, `unknown`). When
freshness `requires_label` (stale or unknown), `stale_label_shown` is set, and a degraded
item is never shown as live.

## Degraded behavior

`apply_redaction_degradation` narrows sections, narrows redaction to its conservative
class, narrows packet availability to its local path, downgrades completeness claims, and
downgrades freshness from a per-observation signal, recording the reasons in
`degraded_labels`:

| Observation | Effect |
| --- | --- |
| `managed_service_available = false` | every section narrows one step, every live/cached item goes stale; labels `managed_service_degraded`, `freshness_downgraded_to_stale` |
| `redaction_proof_available = false` | every verified redaction downgrades to claimed-unverified and is labeled, a `redacted_summary` class narrows to `reference_only`, the redaction-policy section narrows; labels `redaction_proof_unavailable`, `redaction_narrowed_to_reference` |
| `packet_assembler_available = false` | every provider-assembled packet narrows to `unavailable` while the local path remains; the incident-packet and support-packet sections narrow; labels `packet_assembler_unavailable`, `packet_narrowed_to_local_path` |
| `completeness_verified = false` | every verified completeness claim downgrades to `complete_unverified` and is labeled; the packet sections narrow; labels `completeness_unverified`, `completeness_claim_downgraded` |
| `incident_attribution_available = false` | every attributed incident packet narrows its attribution and is labeled; the incident-packet section narrows; labels `incident_attribution_unavailable`, `incident_attribution_narrowed` |
| `proof_fresh = false` / `upstream_matrix_narrowed = true` | every section narrows; labels `proof_stale` / `upstream_matrix_narrowed` |
| `host_session_active = false` | every host-dependent exact handoff narrows to `unresolved`; labels `host_session_inactive`, `handoff_target_unresolved` |

The local path always remains, local work is always preserved, no raw payload body ever
crosses, and degraded state is labeled, never hidden.

## Boundary and redaction

Credential bodies, raw provider payloads, raw incident evidence bodies, and raw
support-bundle contents never cross this boundary; the packet carries only redacted
summaries and opaque refs (`record_ref`, `deep_link_ref`).

## Regenerating the artifacts

The checked-in support export, the Markdown summary, and the degraded fixtures are
generated deterministically from the first-consumer surface builder:

```text
cargo run -p aureline-companion --example dump_redaction_continuity_offline_packet_surface -- canonical
cargo run -p aureline-companion --example dump_redaction_continuity_offline_packet_surface -- markdown
cargo run -p aureline-companion --example dump_redaction_continuity_offline_packet_surface -- managed_service_degraded
cargo run -p aureline-companion --example dump_redaction_continuity_offline_packet_surface -- redaction_proof_lost
cargo run -p aureline-companion --example dump_redaction_continuity_offline_packet_surface -- packet_assembler_down
cargo run -p aureline-companion --example dump_redaction_continuity_offline_packet_surface -- completeness_unverified
cargo run -p aureline-companion --example dump_redaction_continuity_offline_packet_surface -- incident_attribution_lost
```
