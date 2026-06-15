# Offboarding cards — human-readable rendering

Human-readable rendering of the canonical offboarding-card set. This row is a
depth-lane proof governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).
The machine-readable truth is at `artifacts/service/m5-offboarding-cards.json`.

## Per-event card

| Lifecycle event | Card | Entitlement state | Posture origin | Effective claim | Maps to managed state |
| --- | --- | --- | --- | --- | --- |
| grace_period | offboarding_card.grace_period | entitlement_in_grace | account | managed_narrowed | grace_period |
| seat_loss | offboarding_card.seat_loss | entitlement_suspended_admin | seat | local_safe_only | seat_removed |
| cancellation | offboarding_card.cancellation | entitlement_expired | plan | local_safe_only | (none) |
| org_switch | offboarding_card.org_switch | entitlement_pending_recheck | org | managed_narrowed | org_switched |

The effective claim is the lifecycle event's cap; a grace window and an org switch
keep managed work in a narrowed form, while a seat loss and a cancellation drop
managed work to the local-safe baseline. Every card declares `managed_full` before
narrowing. Cards that map to a frozen managed state are cross-checked against the
control-plane row.

## What each card states

| Card | Export rights | Final usage disclosure | Deletion timeline | Owner handoff |
| --- | --- | --- | --- | --- |
| grace_period | managed_admissible_per_grace, export_before_suspend, offboarding_export_admissible | bound_to_unit_as_of_scope | export until grace close; managed copies suspended at close, deleted after retention; local never deleted | billing_contact via billing_portal |
| seat_loss | export_before_suspend, offboarding_export_admissible | suppressed_no_managed_number | export before the seat's access ends; managed deletion per org retention; local never deleted | seat_administrator via admin_console |
| cancellation | export_before_suspend, offboarding_export_admissible | bound_to_unit_as_of_scope | export until deadline; managed copies suspended at deadline, deleted after retention; local never deleted | billing_contact via billing_portal |
| org_switch | export_before_suspend, offboarding_export_admissible | suppressed_no_managed_number | export the prior org's artifacts before the switch; nothing deleted by the switch; local never deleted | organization_owner via admin_console |

## Local versus tenant-scoped state

Every card separates local artifacts (on device, user-owned) from tenant-scoped
managed state (rebinds, is reclaimed, or is tenant-retained). The seat-loss and
org-switch cards make this separation the headline, and the support/admin packet
binds those two cards.

| Card | Local artifacts (stay on device) | Tenant-scoped managed state |
| --- | --- | --- |
| seat_loss | local files, settings, Git history; local support bundles | seat quota, usage, synced settings reclaimed by the organization |
| org_switch | local files, settings, Git history; cached snapshots labeled with as-of | prior org's quota, usage, synced copies stay with that tenant; new org rebinds on recheck |

## Action priority — export is never buried

| Action | Rank | Present on |
| --- | --- | --- |
| export_now | 1 | all four cards |
| continue_local | 2 | all four cards |
| review_deletion_timeline | 3 | all four cards |
| contact_owner | 4 | all four cards |
| upgrade_or_renew | 5 | grace_period, cancellation |

No `upgrade_or_renew` action ranks above `export_now` or `continue_local`. The
seat-loss and org-switch cards carry no upgrade prompt at all.

## The four events stay distinct

Every card lists the other three events in `must_not_collapse_with` and sets
`distinct_from_sign_in_failure` and `not_a_generic_account_error`, so a grace
window, a seat loss, a cancellation, and an org switch never collapse into one
account error, and none is a sign-in/reauth failure.

## Surface bindings

| Surface | Binds cards |
| --- | --- |
| account_surface | all four cards |
| diagnostics | all four cards |
| help_about | all four cards |
| support_admin_packet | the seat_loss and org_switch cards (local-vs-tenant separation) |
| claim_public_truth_automation | all four cards |

## Summary

- 4 offboarding cards, one per lifecycle event; an exhaustive set.
- Every card states the event type, effective date, impacted managed features,
  export rights, local-safe continuation, deletion timeline, and owner handoff.
- Every card keeps a non-empty local-safe continuation and never deletes local
  data.
- Org-switch and seat-loss cards separate local artifacts from tenant-scoped
  managed state.
- Export and local continuation always outrank any upgrade or renewal prompt.
- 2 cards narrow to managed_narrowed (grace, org switch) and 2 to local_safe_only
  (seat loss, cancellation).
- 5 surfaces, each projecting the effective claim, never a stronger one.
