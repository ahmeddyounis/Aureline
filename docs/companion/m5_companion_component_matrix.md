# M5 companion component matrix

Status: frozen (M05-996, batch B118)

This contract freezes Aureline's reusable **companion-client components** into one
export-safe matrix so notification triage, bounded mobile review, CI awareness, session
follow, incident snapshots, and desktop handoff stop drifting across claimed M5 companion
surfaces. It is the shared companion-component contract layered on top of the already-claimed
companion triage, session-follow, incident-awareness, and desktop-handoff surfaces — it does
not re-architect them.

- Authoritative validator: `crates/aureline-companion` module
  `freeze_the_m5_companion_component_matrix` (`M5CompanionComponentMatrixPacket::validate`).
- Canonical export: `artifacts/release/m5-companion-component-proof/support_export.json`
  (regenerated only from the seed builder via the
  `dump_m5_companion_component_matrix` example).
- Combined boundary schema: `schemas/ui/m5-companion-component-matrix.schema.json`.
- Machine-readable matrix: `artifacts/release/m5-companion-component-proof/matrix.csv`.
- Design report: `artifacts/design/m5-companion-component-matrix.md`.

## Governed component families

| Component | Canonical schema |
| --- | --- |
| `notification_row` | `schemas/ui/m5-companion-notification-row.schema.json` |
| `mobile_review_card` | `schemas/ui/m5-mobile-review-card.schema.json` |
| `ci_status_card` | `schemas/ui/m5-ci-status-card.schema.json` |
| `session_follow_tile` | `schemas/ui/m5-session-follow-tile.schema.json` |
| `incident_snapshot_card` | `schemas/ui/m5-incident-snapshot-card.schema.json` |
| `desktop_handoff_sheet` | `schemas/ui/m5-desktop-handoff-sheet.schema.json` |

Downstream M5 rows point at a component's canonical per-component schema instead of restating
its companion UI truth by hand.

## Controlled disposition vocabulary

Consumers bind to **one** controlled disposition vocabulary — no companion surface invents a
parallel word for any of these:

`review_only`, `comment_capable`, `desktop_required`, `cached`, `stale`, `policy_blocked`,
`handoff_ready`.

Alongside dispositions, every component binds an **object kind** (what a tap opens), a
**client scope** (workspace/repo/org/device/account), a **freshness** class, and — where it
applies — a **severity**, and the desktop-handoff sheet always binds an **exact handoff
target**.

## Hard invariants

Every row asserts all four are `false`:

- `masks_scope_or_freshness` — never masks its client scope or freshness.
- `hides_capability_boundary` — never hides the companion-versus-desktop capability boundary.
- `invents_alternate_state_label` — never invents an alternate label for a governed state.
- `implies_desktop_action_is_companion_safe` — never implies a desktop-required action is
  companion-safe.

Raw file bodies, diff hunks, secret values, and private endpoints never cross this boundary;
the export is metadata-only.

## Object identity, scope, and handoff truth

Object identity, workspace/repo scope, freshness, the companion-versus-desktop capability
boundary, severity, and the exact handoff target remain explicit everywhere Aureline triages,
reviews, acknowledges, follows, escalates, or hands work back to desktop from a companion
client. Stale or cached state is always labeled — never shown as live — and a desktop-required
action is always honest before a tap or share action occurs.
