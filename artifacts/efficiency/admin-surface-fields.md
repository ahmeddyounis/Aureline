# Admin and support efficiency-surface fields

The policy-or-admin surface and support exports must be able to explain the
active low-power and thermal posture **without improvising their own wording**.
They render the export-safe projection of an efficiency-state claim entry through
the [publication-ingestion register](publication-ingestion-register.json); they
never restate a low-power posture in hand-written prose and never carry raw
energy, power, thermal, battery, log, provider, or content material.

This document is the canonical reference for which fields the admin and support
surfaces may carry and which fields may never cross the publication boundary. It
is the human-readable companion to the `export_safe_fields` and
`forbidden_fields` tables in
[`publication-ingestion-register.json`](publication-ingestion-register.json), and
the validator `ci/check_efficiency_publication_ingestion.py` enforces that every
binding's `disclosed_fields` is a subset of the export-safe set and disjoint from
the forbidden set.

The canonical entries are the rows of the
[M5 efficiency-state governance matrix](m5-efficiency-governance.json); the admin
and support surfaces point at those rows rather than cloning their wording.

## Export-safe fields

Each field below is projected verbatim from the claim entry. The admin operator
or support agent reads these values; they are stable governance tokens and
reviewable sentences only.

| Field | What it carries |
| --- | --- |
| `entry_id` | The stable governance-matrix row id the surface points at. |
| `title` | The entry's reviewable one-line title. |
| `m5_surface` | The M5 surface the entry governs (notebooks, previews, traces, …). |
| `efficiency_state` | The active efficiency-state token. |
| `source_of_change` | The source-of-change tokens that drove the state. |
| `posture` | The entry's claimed low-power posture. |
| `published_claim_ceiling` | The strongest posture the entry may publish before narrowing. |
| `effective_posture` | The entry's narrowed effective posture after governance. |
| `certification_state` | The entry's certification outcome (`certified`, `narrowed`, `quarantined`). |
| `claim_support` | Whether the claim is `supported`, `narrowed`, or `unsupported`. |
| `override_posture` | Whether and how the adaptation may be overridden. |
| `recovery_state` | The staged-recovery state of the adaptation. |
| `fired_narrowing_reasons` | The reasons that narrowed the claim, for diagnosis. |

The policy-or-admin surface typically discloses the entry id, the M5 surface, the
posture and narrowed effective posture, the certification and claim-support
state, the **override posture and recovery state**, and the fired narrowing
reasons. That is enough for an admin to explain *what posture is active on which
surface, whether it can be overridden, how it recovers, and why it was narrowed*
— entirely from the canonical entry, and entirely from the override rules the
entry already carries.

A support export discloses the same posture, claim-support, override, and
recovery vocabulary so support and admin read an identical posture.

## Forbidden fields

These never cross into any product, admin, or export surface. A binding that
discloses one of them fails the gate.

| Field | Why it is withheld |
| --- | --- |
| `raw_energy_trace` | Raw energy traces stay in retained lab telemetry. |
| `raw_power_samples` | Raw power samples are telemetry, not export-safe claim vocabulary. |
| `raw_thermal_samples` | Raw thermal samples are telemetry, not export-safe claim vocabulary. |
| `raw_battery_telemetry` | Raw battery telemetry is withheld; only the efficiency state and source-of-change tokens are published. |
| `raw_log` | Raw logs may carry content and never appear in a surface. |
| `provider_payload` | Raw provider request or response bodies never appear in an efficiency surface. |
| `secret_material` | Credentials, tokens, and other secret material never appear in any surface. |
| `user_content` | Document bodies and other user content never cross the boundary. |
| `file_path` | File paths can leak user content and are never published. |
| `machine_label` | Raw machine labels are not part of an export-safe efficiency projection. |

## Narrowed and unsupported claims

Because the admin and support surfaces render the entry's `effective_posture`,
`certification_state`, and `claim_support`, a narrowed or unsupported claim shows
the same way on the admin surface and in a support export as it does in docs,
help, the About surface, and service-health. A quarantined entry renders as
`unsupported` with its fired narrowing reasons; it never renders its old ceiling.
An admin never has to decide how to describe a narrowed posture — the entry
already carries the export-safe vocabulary and the override rules that govern it.
