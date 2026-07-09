# M5 incident-snapshot cards and desktop-handoff sheets

Status: implemented. This contract implements the last two components frozen in the
[M5 companion component matrix](m5_companion_component_matrix.md) — the `incident_snapshot_card`
and the `desktop_handoff_sheet` — as one export-safe controls packet with two co-equal control
vectors. It preserves exact incident and escalation context when the task exceeds companion
scope: a user never has to infer which service or run an incident belongs to, whether the
companion can actually remediate, or what exactly a handoff will open on desktop.

- Crate module:
  `crates/aureline-companion/src/implement_incident_snapshot_cards_and_desktop_handoff_sheets_with_service_run_identity_severity_status_target_identity_auth_tenant_reminder_and_open_on_desktop_truth`
- Boundary schema:
  [`schemas/ui/m5-incident-snapshot-card-desktop-handoff-sheet-controls.schema.json`](../../schemas/ui/m5-incident-snapshot-card-desktop-handoff-sheet-controls.schema.json)
- Checked support export:
  `artifacts/release/m5-incident-snapshot-card-desktop-handoff-sheet-proof/support_export.json`
  (plus `matrix.csv` and `summary.md`)
- Scenario fixtures:
  `fixtures/ui/m5-incident-snapshot-card-desktop-handoff-sheet-controls/`
- Headless emitter:
  `cargo run -p aureline-companion --example dump_incident_snapshot_card_desktop_handoff_sheet_controls -- <support-export|report|csv|fixture-*|validate>`

## Reused frozen vocabulary

This lane never invents a parallel companion grammar. It reuses the frozen matrix enums
verbatim: object kind, client scope, freshness, severity, handoff target, degraded reason,
required labels, surface family, deployment line, consumer surface, accessibility route, and
downgrade triggers. It mints new vocabulary only for what the matrix left implicit about these
two controls (the frozen matrix freezes only severity for the incident family and only the
handoff target for the handoff family).

## Incident-snapshot card

Each incident-snapshot card names its **service/source class** (`service_class`,
`service_label`), its **stable service and run identity** (`service_ref`, `run_ref`), the
object it references (`object_kind`, `object_label`), its **exact object landing reference**
(`object_landing_ref` — the one stable object `Open` lands on, never a generic activity page),
its client scope, its **severity** (`severity`, `severity_label`), its **latest status**
(`incident_status`), and its freshness.

Its **awareness class** is *derived* from the incident status by `resolve_incident_awareness`,
never asserted:

| incident status | awareness class | live incident? | open? |
| --- | --- | --- | --- |
| `firing` | `active_unacknowledged` | yes | yes (awareness note required) |
| `acknowledged` / `investigating` | `active_acknowledged` | yes | yes (awareness note required) |
| `mitigating` | `mitigating` | yes | yes (awareness note required) |
| `resolved` | `resolved` | yes | no |
| `stale` | `stale_unknown` | no (stale note required) | no |

A stale incident status can therefore never read as a live incident. While an incident is open
the card carries an **awareness-only note** so the companion never overpromises remediation
depth — remediation happens on desktop, not the companion (`implies_companion_remediation` must
be `false`). Each card offers a keyboard-complete `Open` verb and a bounded `Acknowledge` verb;
a card whose `handoff_target` is `no_handoff` offers no `handoff_to_desktop` verb, so no verb
points at a target it cannot resolve.

## Desktop-handoff sheet

Each desktop-handoff sheet names its **target object** (`handoff_target`,
`target_object_label`), its **stable target identity** (`target_ref`), the object it references,
its exact object landing reference, its client scope, exactly **what opens on desktop**
(`opens_on_desktop_note`), and — where relevant — an **auth or tenant reminder**
(`auth_context`, `auth_tenant_reminder_note`).

Its **open class** is *derived* from the frozen handoff target by `resolve_handoff_open`, never
asserted:

| handoff target | open class | openable? |
| --- | --- | --- |
| `file_location` | `opens_exact_location` | yes |
| `review_panel` / `ci_pipeline_run` | `opens_exact_panel` | yes |
| `incident_workspace` / `agent_session` | `opens_exact_workspace` | yes |
| `no_handoff` | `not_openable` | no (not-openable note required) |

A sheet with no resolvable target therefore degrades to an explicit not-openable state instead
of implying a desktop client will open the intended object, and a not-openable sheet never
offers the `open_on_desktop` verb — so no user is offered an ambiguous open into a target the
desktop cannot resolve. When the auth context is anything other than `same_auth_no_reminder`
(re-auth, tenant switch, account mismatch, or scope elevation), the sheet carries an explicit
reminder so the desktop client opens the intended object without user archaeology.

## Hard invariants (per control, all `false`)

- `masks_scope_or_freshness`
- `hides_capability_boundary`
- `invents_alternate_state_label`
- `implies_desktop_action_is_companion_safe`
- `routes_to_generic_activity_page`
- `implies_companion_remediation` (incident-snapshot card only)

## Coverage

The canonical packet carries six incident-snapshot cards covering all five awareness classes
and all six incident statuses, and six desktop-handoff sheets covering all four open classes and
all six handoff targets. Raw incident payloads, log bodies, secret values, and private endpoints
never cross this boundary.
