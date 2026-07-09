# Rotation/revoke-event rows and export-safety banners (M05-992)

This lane implements two components frozen in the
[M5 credential component matrix](m5_credential_component_matrix.md) — the
`rotation_revoke_event_row` and the `export_safety_banner` — into one export-safe
packet with two co-equal control vectors. Together they keep credential lifecycle and
export behavior explicit after rotation, revoke, expiry, or a support / export handoff
event.

- Crate module:
  `crates/aureline-provider/src/implement_rotation_revoke_event_rows_and_export_safety_banners_with_impacted_workflow_remembered_decision_and_raw_secret_excluded_continuity_truth/`
- Boundary schema:
  [`schemas/ui/m5-rotation-revoke-export-safety-controls.schema.json`](../../schemas/ui/m5-rotation-revoke-export-safety-controls.schema.json)
- Release proof:
  `artifacts/release/m5-rotation-revoke-export-safety-proof/`
- Scenario fixtures:
  `fixtures/ui/m5-rotation-revoke-export-safety-controls/`
- Headless emitter:
  `cargo run -p aureline-provider --bin aureline_rotation_revoke_export_safety_primitive -- <subcommand>`

## Goal

Keep credential lifecycle and export behavior explicit after rotation, revoke, expiry,
or a support / export handoff event.

## Reused vocabulary

The credential classes, lifecycle states, export-safety classes, reveal postures,
degraded states, required labels, surface families, deployment lines, consumer surfaces,
accessibility routes, and downgrade triggers are reused verbatim from the frozen matrix,
so this lane never invents a parallel credential grammar. It mints new vocabulary only
for what that matrix left implicit about these two controls.

## Rotation/revoke-event row

A `RotationRevokeEventRow` always names its **credential class**, its **prior** and
**new lifecycle state**, its **impacted workflows** (running sessions, queued jobs, and
remembered decisions), its **recovery next step**, and its **audit event**. It always
offers keyboard-complete `follow_recovery_step` and `export_audit_evidence` actions.

Its **continuity class is derived**, never asserted, from the new lifecycle state, so a
revoked or expired credential can never read as still usable, and what rotation or revoke
will impact never has to be inferred:

| New lifecycle state | Derived continuity class | Still usable |
| --- | --- | --- |
| `active_current` | `still_active` | yes |
| `refresh_needed` / `rotation_due` | `action_required` | yes |
| `revoked` / `expired` | `no_longer_usable` | no |
| `superseded` | `superseded` | no |

Only a `still_active` or `action_required` row may claim the credential is still usable
(`claims_still_usable`). A still-active row must carry its still-active note, an
action-required row its action note, a no-longer-usable row its no-longer-usable note,
and a superseded row its superseded note. The rows cover all six lifecycle states, all
four continuity classes, and all six impacted-workflow classes (running session, queued
job, remembered decision, scheduled automation, delegated forward, and no active impact).

## Export-safety banner

An `ExportSafetyBanner` always states that **raw credentials are excluded by default**
from profiles, support bundles, handoff packets, recipes, and portable workspace exports,
names its **export surface**, its **export-safety class**, its **reveal posture**, and
the **handle-class / source labels preserved** where allowed. It always offers
keyboard-complete `view_redaction_policy` and `view_excluded_fields` actions.

Its **redaction posture is derived**, never asserted, from the export-safety class, so an
export never implies a raw secret is exportable and credential exclusion is never left to
implication:

| Export-safety class | Derived posture | Preserves handle labels |
| --- | --- | --- |
| `raw_secret_excluded` / `metadata_only` | `raw_excluded_labels_preserved` | yes |
| `handle_reference_only` | `handle_reference_only` | yes |
| `redacted_share` / `endpoints_masked` | `redacted_or_masked` | yes |
| `export_blocked` | `fully_blocked` | no |

Only a non-`fully_blocked` banner may claim it preserves handle-class / source labels
(`claims_preserves_handle_labels`). A raw-excluded or handle-reference banner must carry
its preserved-label note, a redacted / masked banner its redaction note, and a blocked
banner its blocked note. The banners cover all six export-safety classes, all four
postures, and all six export surfaces.

## Guardrails

Each control carries hard invariants, all of which must be `false`:

- Rotation/revoke-event row — `masks_impacted_workflows` (the running sessions, queued
  jobs, and remembered decisions a rotation / revoke impacts are never masked),
  `implies_raw_secret_exportable`, and `uses_friendly_connected_wording`.
- Export-safety banner — `implies_raw_secret_exportable`,
  `leaves_exclusion_to_implication` (credential exclusion is never left to implication),
  and `uses_friendly_connected_wording`.

Raw secret values, tokens, passphrases, and private endpoints never cross the export
boundary. The support export is metadata-only and export-safe.

## Acceptance criteria

- Users can see what running sessions, queued jobs, or remembered decisions are affected
  by rotation or revoke — the event row always names its impacted workflows and its
  recovery next step, and its derived continuity class never lets a revoked or expired
  credential read as still usable.
- Export surfaces no longer leave credential exclusion to implication — the export-safety
  banner always states that raw credentials are excluded by default across profiles,
  support bundles, handoff packets, recipes, and portable workspace exports, and its
  derived redaction posture keeps that default explicit and reusable while preserving
  handle-class / source labels where allowed.
